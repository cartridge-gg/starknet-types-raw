use std::error::Error;

/// Error returned by [Felt::from_be_bytes] indicating the maximum field value
/// was exceeded.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OverflowError;

impl Error for OverflowError {}

// TryFrom<Felt> for primitive
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PrimitiveFromFeltError;

impl Error for PrimitiveFromFeltError {}

#[derive(Debug, PartialEq, Eq)]
pub enum FromStrError {
    InvalidNibble(u8),
    InvalidDigit(u8),
    InvalidLength { max: usize, actual: usize },
    Overflow,
    EmptyString,
}

impl Error for FromStrError {}

impl From<OverflowError> for FromStrError {
    fn from(_: OverflowError) -> Self {
        Self::Overflow
    }
}

impl std::fmt::Display for FromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNibble(n) => write!(f, "invalid nibble: 0x{:x}", *n),
            Self::InvalidDigit(d) => write!(f, "invalid digit: '{}'", *d as char),
            Self::InvalidLength { max, actual } => {
                write!(f, "more than {} digits found: {}", *max, *actual)
            }
            Self::Overflow => f.write_str(OVERFLOW_MSG),
            Self::EmptyString => f.write_str("empty string"),
        }
    }
}

impl core::fmt::Display for PrimitiveFromFeltError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed to convert `Felt` into primitive type")
    }
}

const OVERFLOW_MSG: &str = "The maximum field value was exceeded.";

impl std::fmt::Display for OverflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(OVERFLOW_MSG)
    }
}
