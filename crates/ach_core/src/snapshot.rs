//! Snapshot files: common header plus one frame of caller state.
//! Implements Technical Design §22.2 (save contents) and Simulation §28.6 (save, loading, recovery).

use crate::error::CoreError;
use crate::journal::{
    FileHeader, MAGIC_SNAPSHOT, read_frame, read_header, write_frame, write_header,
};
use serde::{Serialize, de::DeserializeOwned};
use std::io::{Read, Write};

/// Writes `header` then one frame holding the postcard bytes of `state`.
///
/// Output is byte-deterministic for equal inputs. `header.magic` should be
/// [`MAGIC_SNAPSHOT`].
pub fn write_snapshot<W: Write, T: Serialize>(
    w: &mut W,
    header: &FileHeader,
    state: &T,
) -> Result<(), CoreError> {
    write_header(w, header)?;
    let bytes = postcard::to_stdvec(state).map_err(|e| CoreError::Codec(e.to_string()))?;
    write_frame(w, &bytes)
}

/// Reads a snapshot written by [`write_snapshot`].
///
/// A missing state frame is reported as [`CoreError::TruncatedFrame`].
pub fn read_snapshot<R: Read, T: DeserializeOwned>(
    r: &mut R,
) -> Result<(FileHeader, T), CoreError> {
    let header = read_header(r, MAGIC_SNAPSHOT)?;
    let bytes = read_frame(r)?.ok_or(CoreError::TruncatedFrame)?;
    let state = postcard::from_bytes(&bytes).map_err(|e| CoreError::Codec(e.to_string()))?;
    Ok((header, state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::MAGIC_JOURNAL;
    use crate::rng::Seed;

    fn header() -> FileHeader {
        FileHeader::new(MAGIC_SNAPSHOT, 3, Seed(9), "scn")
    }

    #[test]
    fn round_trip_and_determinism() {
        let state = (vec![1u32, 2, 3], "hello".to_string());
        let (mut a, mut b) = (Vec::new(), Vec::new());
        write_snapshot(&mut a, &header(), &state).unwrap();
        write_snapshot(&mut b, &header(), &state).unwrap();
        assert_eq!(a, b);
        let (h, back): (_, (Vec<u32>, String)) = read_snapshot(&mut a.as_slice()).unwrap();
        assert_eq!(h, header());
        assert_eq!(back, state);
    }

    #[test]
    fn missing_state_frame_is_truncated() {
        let mut buf = Vec::new();
        write_header(&mut buf, &header()).unwrap();
        let r: Result<(FileHeader, u8), _> = read_snapshot(&mut buf.as_slice());
        assert_eq!(r.unwrap_err(), CoreError::TruncatedFrame);
    }

    #[test]
    fn journal_magic_is_rejected() {
        let mut buf = Vec::new();
        let h = FileHeader::new(MAGIC_JOURNAL, 1, Seed(0), "");
        write_snapshot(&mut buf, &h, &0u8).unwrap();
        let r: Result<(FileHeader, u8), _> = read_snapshot(&mut buf.as_slice());
        assert!(matches!(r, Err(CoreError::BadMagic { .. })));
    }
}
