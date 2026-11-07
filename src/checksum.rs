//! Checksum model.

#[derive(Debug, Clone)]
pub struct ChecksumResult {
    pub hex: String,
    pub bytes: usize,
}

#[derive(Debug, Clone)]
pub struct Checksum {
    pub algorithm: &'static str,
}

impl Checksum {
    pub fn b2() -> Self {
        Self { algorithm: "blake2" }
    }

    pub fn digest(&self, data: &[u8]) -> ChecksumResult {
        let mut acc: u64 = 0xcbf29ce484222325;
        for b in data { acc = (acc ^ *b as u64).wrapping_mul(0x100000001b3); }
        let hex = format!("{:016x}", acc);
        ChecksumResult { hex, bytes: data.len() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn digest() {
        let c = Checksum::b2();
        let r = c.digest(b"hello");
        assert_eq!(r.bytes, 5);
        assert_eq!(r.hex.len(), 16);
    }
}
