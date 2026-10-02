//! FNV-1a 64-bit hashing for journal fingerprints.
//! Implements Technical Design §22.1 (deterministic fingerprints).

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// FNV-1a 64-bit over `bytes`. Frozen: offset `0xcbf29ce484222325`, prime `0x100000001b3`.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(FNV_OFFSET, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(FNV_PRIME)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // frozen P1-03; changing these breaks every save
    #[test]
    fn known_answers() {
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
    }
}
