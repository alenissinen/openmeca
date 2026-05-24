//! The only error type this crate uses.

use crate::ids::DeviceKind;
use std::fmt;

/// Covers everything that can go wrong when talking to an EVO device
#[derive(Debug)]
pub enum Error {
    /// `hidapi` layer failed
    Hid(hidapi::HidError),
    /// No connected device matched the requested kind.
    DeviceNotFound(DeviceKind),
    /// Input report was shorter than expected.
    ShortReport { expected: usize, got: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Hid(e) => write!(f, "HID error: {e}"),
            Error::DeviceNotFound(kind) => write!(f, "Device not found: {kind:?}"),
            Error::ShortReport { expected, got } => {
                write!(
                    f,
                    "Short input report: expected {expected} bytes, got {got}"
                )
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Hid(e) => Some(e),
            _ => None,
        }
    }
}

// Enables `?` on hidapi calls.
impl From<hidapi::HidError> for Error {
    fn from(e: hidapi::HidError) -> Self {
        Error::Hid(e)
    }
}

/// Alias used throughout the crate for convenience
pub type Result<T> = std::result::Result<T, Error>;
