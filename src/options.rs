//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Default,
    Binary,
    Text,
    Tag,
    Quiet,
}

#[derive(Debug, Clone)]
pub struct ChecksumOptions {
    pub mode: OutputMode,
    pub check: bool,
}

impl Default for ChecksumOptions {
    fn default() -> Self {
        Self { mode: OutputMode::Default, check: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn options() {
        assert_eq!(ChecksumOptions::default().mode, OutputMode::Default);
    }
}
