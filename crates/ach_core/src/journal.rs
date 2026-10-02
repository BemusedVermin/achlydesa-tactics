//! Versioned, append-only binary journal and the common file header.
//! Implements Technical Design §22.2 (save contents) and §22.3 (schema versions), and
//! Simulation §28.6 (save, loading, recovery).
//!
//! Layout: header, then frames of `u32 LE length` + postcard bytes. The running
//! fingerprint is `fp = fnv1a64(fp.to_le_bytes() ++ payload)` per frame, seeded with
//! `fnv1a64(header_bytes)`; the length prefix is not hashed.

use crate::error::CoreError;
use crate::hash::fnv1a64;
use crate::rng::Seed;
use crate::time::SimTime;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{Read, Write};
use std::marker::PhantomData;

/// Magic for journal files.
pub const MAGIC_JOURNAL: [u8; 4] = *b"ACHJ";
/// Magic for snapshot files.
pub const MAGIC_SNAPSHOT: [u8; 4] = *b"ACHS";
/// Magic for terrain chunk files (used by P1-06).
pub const MAGIC_TERRAIN: [u8; 4] = *b"ACHT";
/// The only supported `format_version`.
pub const FORMAT_VERSION: u16 = 1;
/// Largest permitted frame payload, 16 MiB.
pub const MAX_FRAME_BYTES: u32 = 16 * 1024 * 1024;

/// Common header of every Achlydesa binary file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileHeader {
    /// File kind: one of the `MAGIC_*` constants.
    pub magic: [u8; 4],
    /// Container format version; currently [`FORMAT_VERSION`].
    pub format_version: u16,
    /// Simulation schema version, supplied by the caller (reserved for migration).
    pub schema_version: u32,
    /// Campaign seed.
    pub seed: Seed,
    /// Scenario or content id.
    pub label: String,
}

impl FileHeader {
    /// A header at the current [`FORMAT_VERSION`].
    pub fn new(magic: [u8; 4], schema_version: u32, seed: Seed, label: impl Into<String>) -> Self {
        Self {
            magic,
            format_version: FORMAT_VERSION,
            schema_version,
            seed,
            label: label.into(),
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>, CoreError> {
        let label_len = u16::try_from(self.label.len())
            .map_err(|_| CoreError::LabelTooLong(self.label.len()))?;
        let mut out = Vec::with_capacity(20 + self.label.len());
        out.extend_from_slice(&self.magic);
        out.extend_from_slice(&self.format_version.to_le_bytes());
        out.extend_from_slice(&self.schema_version.to_le_bytes());
        out.extend_from_slice(&self.seed.0.to_le_bytes());
        out.extend_from_slice(&label_len.to_le_bytes());
        out.extend_from_slice(self.label.as_bytes());
        Ok(out)
    }
}

fn io_err(e: std::io::Error) -> CoreError {
    CoreError::Io(e.to_string())
}

/// Writes the header in the little-endian layout of Technical Design §22.2.
pub fn write_header<W: Write>(w: &mut W, h: &FileHeader) -> Result<(), CoreError> {
    w.write_all(&h.to_bytes()?).map_err(io_err)
}

fn read_array<R: Read, const N: usize>(r: &mut R) -> Result<[u8; N], CoreError> {
    let mut buf = [0u8; N];
    r.read_exact(&mut buf).map_err(io_err)?;
    Ok(buf)
}

/// Reads and validates a header.
///
/// Fails with [`CoreError::BadMagic`] if the magic differs from `expected_magic`, and
/// [`CoreError::UnsupportedFormat`] if `format_version` is not [`FORMAT_VERSION`].
pub fn read_header<R: Read>(r: &mut R, expected_magic: [u8; 4]) -> Result<FileHeader, CoreError> {
    let magic = read_array::<_, 4>(r)?;
    if magic != expected_magic {
        return Err(CoreError::BadMagic {
            expected: expected_magic,
            found: magic,
        });
    }
    let format_version = u16::from_le_bytes(read_array(r)?);
    if format_version != FORMAT_VERSION {
        return Err(CoreError::UnsupportedFormat(format_version));
    }
    let schema_version = u32::from_le_bytes(read_array(r)?);
    let seed = Seed(u64::from_le_bytes(read_array(r)?));
    let label_len = usize::from(u16::from_le_bytes(read_array(r)?));
    let mut label = vec![0u8; label_len];
    r.read_exact(&mut label).map_err(io_err)?;
    let label = String::from_utf8(label).map_err(|_| CoreError::InvalidLabel)?;
    Ok(FileHeader {
        magic,
        format_version,
        schema_version,
        seed,
        label,
    })
}

/// Writes one frame: `u32 LE` length then `payload`.
pub(crate) fn write_frame<W: Write>(w: &mut W, payload: &[u8]) -> Result<(), CoreError> {
    let len = u32::try_from(payload.len())
        .ok()
        .filter(|&n| n <= MAX_FRAME_BYTES)
        .ok_or(CoreError::FrameTooLarge(payload.len() as u64))?;
    w.write_all(&len.to_le_bytes()).map_err(io_err)?;
    w.write_all(payload).map_err(io_err)
}

/// Reads up to `buf.len()` bytes, returning how many were read (short only at EOF).
fn read_full<R: Read>(r: &mut R, buf: &mut [u8]) -> Result<usize, CoreError> {
    let mut n = 0;
    while n < buf.len() {
        match r.read(&mut buf[n..]) {
            Ok(0) => break,
            Ok(k) => n += k,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(io_err(e)),
        }
    }
    Ok(n)
}

/// Reads one frame payload. `Ok(None)` is a clean end of file at a frame boundary.
pub(crate) fn read_frame<R: Read>(r: &mut R) -> Result<Option<Vec<u8>>, CoreError> {
    let mut len_bytes = [0u8; 4];
    match read_full(r, &mut len_bytes)? {
        0 => return Ok(None),
        4 => {}
        _ => return Err(CoreError::TruncatedFrame),
    }
    let len = u32::from_le_bytes(len_bytes);
    if len > MAX_FRAME_BYTES {
        return Err(CoreError::FrameTooLarge(u64::from(len)));
    }
    let mut payload = vec![0u8; len as usize];
    if read_full(r, &mut payload)? < payload.len() {
        return Err(CoreError::TruncatedFrame);
    }
    Ok(Some(payload))
}

fn chain(fp: u64, payload: &[u8]) -> u64 {
    let mut bytes = Vec::with_capacity(8 + payload.len());
    bytes.extend_from_slice(&fp.to_le_bytes());
    bytes.extend_from_slice(payload);
    fnv1a64(&bytes)
}

/// One journal record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalEntry<E> {
    /// Zero-based sequence number.
    pub seq: u64,
    /// Simulation time of the event.
    pub time: SimTime,
    /// The caller's event.
    pub event: E,
}

/// Appends entries to a journal and maintains its fingerprint.
pub struct JournalWriter<W: Write> {
    w: W,
    next_seq: u64,
    last_time: SimTime,
    fingerprint: u64,
}

impl<W: Write> JournalWriter<W> {
    /// Writes `header` and starts an empty journal.
    pub fn create(mut w: W, header: FileHeader) -> Result<Self, CoreError> {
        let bytes = header.to_bytes()?;
        w.write_all(&bytes).map_err(io_err)?;
        Ok(Self {
            w,
            next_seq: 0,
            last_time: SimTime(i64::MIN),
            fingerprint: fnv1a64(&bytes),
        })
    }

    /// Continues an existing journal after a load; `w` must be positioned at its end.
    pub fn resume(w: W, next_seq: u64, last_time: SimTime, fingerprint: u64) -> Self {
        Self {
            w,
            next_seq,
            last_time,
            fingerprint,
        }
    }

    /// Appends `event` at `time` and returns its sequence number.
    ///
    /// Fails with [`CoreError::TimeWentBackwards`] if `time` precedes the previous entry
    /// (equal times are allowed). A rejected time leaves the writer unchanged.
    pub fn append<E: Serialize>(&mut self, time: SimTime, event: &E) -> Result<u64, CoreError> {
        if time < self.last_time {
            return Err(CoreError::TimeWentBackwards {
                last_ms: self.last_time.0,
                attempted_ms: time.0,
            });
        }
        let seq = self.next_seq;
        let next_seq = seq.checked_add(1).ok_or(CoreError::IdExhausted)?;
        let entry = JournalEntry { seq, time, event };
        let bytes = postcard::to_stdvec(&entry).map_err(|e| CoreError::Codec(e.to_string()))?;
        write_frame(&mut self.w, &bytes)?;
        self.fingerprint = chain(self.fingerprint, &bytes);
        self.next_seq = next_seq;
        self.last_time = time;
        Ok(seq)
    }

    /// Fingerprint chained over the header and every frame so far.
    pub fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// Sequence number the next append will receive.
    pub fn next_seq(&self) -> u64 {
        self.next_seq
    }

    /// Returns the underlying writer.
    pub fn into_inner(self) -> W {
        self.w
    }
}

/// Iterates the entries of a journal.
pub struct JournalReader<R: Read, E> {
    r: R,
    done: bool,
    _event: PhantomData<fn() -> E>,
}

impl<R: Read, E: DeserializeOwned> JournalReader<R, E> {
    /// Reads and validates the header, returning it with an entry iterator.
    pub fn open(mut r: R) -> Result<(FileHeader, Self), CoreError> {
        let header = read_header(&mut r, MAGIC_JOURNAL)?;
        Ok((
            header,
            Self {
                r,
                done: false,
                _event: PhantomData,
            },
        ))
    }
}

impl<R: Read, E: DeserializeOwned> Iterator for JournalReader<R, E> {
    type Item = Result<JournalEntry<E>, CoreError>;

    /// Yields entries until clean EOF; after any error, iteration ends.
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let item = match read_frame(&mut self.r) {
            Ok(None) => None,
            Ok(Some(bytes)) => {
                Some(postcard::from_bytes(&bytes).map_err(|e| CoreError::Codec(e.to_string())))
            }
            Err(e) => Some(Err(e)),
        };
        if !matches!(item, Some(Ok(_))) {
            self.done = true;
        }
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pos::WorldPos;
    use proptest::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    enum Ev {
        Said { who: String, text: String },
        Moved(WorldPos),
        Marked(SimTime),
        Idle,
    }

    fn header() -> FileHeader {
        FileHeader::new(MAGIC_JOURNAL, 7, Seed(42), "test-scenario")
    }

    fn event(i: u64) -> Ev {
        let n = i as i64;
        match i % 4 {
            0 => Ev::Said {
                who: format!("p{i}"),
                text: "x".repeat((i % 17) as usize),
            },
            1 => Ev::Moved(WorldPos::surface_m(n, -n)),
            2 => Ev::Marked(SimTime(n * 1000)),
            _ => Ev::Idle,
        }
    }

    fn write_all(n: u64) -> (Vec<u8>, u64) {
        let mut w = JournalWriter::create(Vec::new(), header()).unwrap();
        for i in 0..n {
            w.append(SimTime(i as i64 * 10), &event(i)).unwrap();
        }
        let fp = w.fingerprint();
        (w.into_inner(), fp)
    }

    #[test]
    fn round_trip_10k() {
        let (bytes, _) = write_all(10_000);
        let (h, reader) = JournalReader::<_, Ev>::open(bytes.as_slice()).unwrap();
        assert_eq!(h, header());
        let entries: Vec<_> = reader.collect::<Result<_, _>>().unwrap();
        assert_eq!(entries.len(), 10_000);
        for (i, e) in entries.iter().enumerate() {
            let i = i as u64;
            assert_eq!(
                *e,
                JournalEntry {
                    seq: i,
                    time: SimTime(i as i64 * 10),
                    event: event(i)
                }
            );
        }
    }

    #[test]
    fn byte_determinism() {
        assert_eq!(write_all(500), write_all(500));
    }

    #[test]
    fn fingerprint_starts_at_header_hash() {
        let w = JournalWriter::create(Vec::new(), header()).unwrap();
        assert_eq!(w.fingerprint(), fnv1a64(&header().to_bytes().unwrap()));
    }

    #[test]
    fn time_backwards_rejected_equal_allowed() {
        let mut w = JournalWriter::create(Vec::new(), header()).unwrap();
        w.append(SimTime(5), &Ev::Idle).unwrap();
        w.append(SimTime(5), &Ev::Idle).unwrap();
        let fp = w.fingerprint();
        let err = w.append(SimTime(4), &Ev::Idle).unwrap_err();
        assert_eq!(
            err,
            CoreError::TimeWentBackwards {
                last_ms: 5,
                attempted_ms: 4
            }
        );
        assert_eq!((w.next_seq(), w.fingerprint()), (2, fp));
    }

    #[test]
    fn truncation_yields_one_error() {
        let (bytes, _) = write_all(10);
        for cut in [1, 3, 5] {
            let short = &bytes[..bytes.len() - cut];
            let (_, reader) = JournalReader::<_, Ev>::open(short).unwrap();
            let items: Vec<_> = reader.collect();
            assert_eq!(items.len(), 10, "cut {cut}");
            assert!(items[..9].iter().all(Result::is_ok));
            assert_eq!(items[9].as_ref().unwrap_err(), &CoreError::TruncatedFrame);
        }
    }

    #[test]
    fn oversized_frame_rejected() {
        let mut bytes = Vec::new();
        write_header(&mut bytes, &header()).unwrap();
        bytes.extend_from_slice(&(MAX_FRAME_BYTES + 1).to_le_bytes());
        let (_, mut reader) = JournalReader::<_, Ev>::open(bytes.as_slice()).unwrap();
        let err = reader.next().unwrap().unwrap_err();
        assert_eq!(
            err,
            CoreError::FrameTooLarge(u64::from(MAX_FRAME_BYTES) + 1)
        );
        assert!(reader.next().is_none());
    }

    #[test]
    fn bad_magic_and_version() {
        let mut bytes = Vec::new();
        write_header(&mut bytes, &header()).unwrap();
        let err = read_header(&mut bytes.as_slice(), MAGIC_SNAPSHOT).unwrap_err();
        assert_eq!(
            err,
            CoreError::BadMagic {
                expected: MAGIC_SNAPSHOT,
                found: MAGIC_JOURNAL
            }
        );
        bytes[4] = 2;
        let err = read_header(&mut bytes.as_slice(), MAGIC_JOURNAL).unwrap_err();
        assert_eq!(err, CoreError::UnsupportedFormat(2));
    }

    #[test]
    fn header_layout_is_little_endian() {
        let mut bytes = Vec::new();
        let h = FileHeader::new(MAGIC_TERRAIN, 0x0102_0304, Seed(1), "ab");
        write_header(&mut bytes, &h).unwrap();
        let expected: Vec<u8> = [
            &b"ACHT"[..],
            &[1, 0],
            &[4, 3, 2, 1],
            &[1, 0, 0, 0, 0, 0, 0, 0],
            &[2, 0],
            b"ab",
        ]
        .concat();
        assert_eq!(bytes, expected);
    }

    proptest! {
        #[test]
        fn resume_matches_uninterrupted(
            steps in proptest::collection::vec((0i64..50, 0u64..1000), 0..40),
            split in 0usize..40,
        ) {
            let split = split.min(steps.len());
            let mut t = 0;
            let timed: Vec<(SimTime, Ev)> = steps.iter().map(|&(dt, i)| {
                t += dt;
                (SimTime(t), event(i))
            }).collect();

            let mut whole = JournalWriter::create(Vec::new(), header()).unwrap();
            for (t, e) in &timed { whole.append(*t, e).unwrap(); }

            let mut first = JournalWriter::create(Vec::new(), header()).unwrap();
            for (t, e) in &timed[..split] { first.append(*t, e).unwrap(); }
            let (next_seq, fp) = (first.next_seq(), first.fingerprint());
            let last = if split == 0 { SimTime(i64::MIN) } else { timed[split - 1].0 };
            let mut bytes = first.into_inner();
            let mut second = JournalWriter::resume(&mut bytes, next_seq, last, fp);
            for (t, e) in &timed[split..] { second.append(*t, e).unwrap(); }
            prop_assert_eq!(second.fingerprint(), whole.fingerprint());
            prop_assert_eq!(bytes, whole.into_inner());
        }
    }
}
