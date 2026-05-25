//! MECA EVO1 - Load Cell Pedals
//!
//! 6-byte input report that has three 16-bit (LE) axes (see `docs/pedals.md`). Configuartion
//! is a 64-byte feature report.

use crate::{DeviceKind, Error, InputDevice, Result, codec::iter_u16_le, transport::Transport};

/// Input report layout (3x 2-byte)
const INPUT_REPORT_LEN: usize = 6;

/// Decoded pedal state. Each axis has a range from 0-4096 (0x0000-0x1000). Logical max
/// defined by observation and HID report descriptor.
///
/// Field order matches the report: clutch (offset 0), brake (2), throttle (4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PedalInput {
    pub clutch: u16,
    pub brake: u16,
    pub throttle: u16,
}

impl PedalInput {
    /// Decodes three u16 values from the 6-byte report.
    /// Returns `None` if buffer has less than 6 bytes.
    fn from_report(buf: &[u8]) -> Option<Self> {
        let mut axes = iter_u16_le(buf);

        Some(Self {
            clutch: axes.next()?,
            brake: axes.next()?,
            throttle: axes.next()?,
        })
    }
}

/// Open connection to the pedals.
pub struct Pedals {
    transport: Transport,
}

impl Pedals {
    /// Opens the connection to pedals.
    pub fn open() -> Result<Self> {
        Ok(Self {
            transport: Transport::open(DeviceKind::Pedals)?,
        })
    }
}

impl InputDevice for Pedals {
    type Input = PedalInput;

    fn read_input(&mut self) -> Result<Self::Input> {
        let mut buf = [0u8; INPUT_REPORT_LEN];
        let bytes_read = self.transport.read(&mut buf)?;

        if bytes_read < INPUT_REPORT_LEN {
            return Err(Error::ShortReport {
                expected: INPUT_REPORT_LEN,
                got: bytes_read,
            });
        }

        PedalInput::from_report(&buf).ok_or(Error::ShortReport {
            expected: INPUT_REPORT_LEN,
            got: bytes_read,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_zeroed_report() {
        let buf = [0u8; 6];
        let input = PedalInput::from_report(&buf).unwrap();

        assert_eq!(input, PedalInput::default());
    }

    #[test]
    fn decodes_known_values() {
        // clutch = 0x1000 (4096), brake = 0x0000, throttle = 0x008C (140)
        let buf = [0x00, 0x10, 0x00, 0x00, 0x8C, 0x00];
        let input = PedalInput::from_report(&buf).unwrap();

        assert_eq!(input.clutch, 4096);
        assert_eq!(input.brake, 0);
        assert_eq!(input.throttle, 140);
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(PedalInput::from_report(&[0x00, 0x10]).is_none());
    }
}
