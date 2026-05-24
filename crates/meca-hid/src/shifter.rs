//! MECA EVO SQ - Sequential Shifter
//!
//! A digital device: 1-byte input report with two button bits and no configuration
//! (see `docs/shifter.md`). Bit 0 = upshift (pull), bit 1 = downshift (push away).

use crate::{DeviceKind, Error, InputDevice, Result, transport::Transport};

const UPSHIFT_BIT: u8 = 1 << 0; // 0x01
const DOWNSHIFT_BIT: u8 = 1 << 1; // 0x02

/// The shifters current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Shift {
    /// Lever pulled towards (upshift).
    pub up: bool,
    /// Lever pushed away (downshift).
    pub down: bool,
}

impl Shift {
    /// Decodes the state from the input report.
    const fn from_byte(byte: u8) -> Self {
        Self {
            up: byte & UPSHIFT_BIT != 0,
            down: byte & DOWNSHIFT_BIT != 0,
        }
    }
}

/// Open connection to the shifter.
pub struct Shifter {
    transport: Transport,
}

impl Shifter {
    /// Opens the connection to shifter.
    pub fn open() -> Result<Self> {
        Ok(Self {
            transport: Transport::open(DeviceKind::Shifter)?,
        })
    }
}

impl InputDevice for Shifter {
    type Input = Shift;

    fn read_input(&mut self) -> Result<Self::Input> {
        let mut buf = [0u8; 1];
        let bytes_read = self.transport.read(&mut buf)?;

        if bytes_read < 1 {
            return Err(Error::ShortReport {
                expected: 1,
                got: bytes_read,
            });
        }

        Ok(Shift::from_byte(buf[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_idle_and_directions() {
        assert_eq!(
            Shift::from_byte(0x00),
            Shift {
                up: false,
                down: false
            }
        );

        assert_eq!(
            Shift::from_byte(0x01),
            Shift {
                up: true,
                down: false
            }
        );

        assert_eq!(
            Shift::from_byte(0x02),
            Shift {
                up: false,
                down: true
            }
        );

        // 0x03 is not physically producible but it should still be handled properly.
        assert_eq!(
            Shift::from_byte(0x03),
            Shift {
                up: true,
                down: true
            }
        );
    }
}
