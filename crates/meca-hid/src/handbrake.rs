//! MECA EVO HB - Load Cell Handbrake
//!
//! 2-byte input report containing a single 16-bit axis data (see `docs/handbrake.md`).
//! Shares the same curve, deadzone and feature report data as the pedals.

use crate::{DeviceKind, Error, InputDevice, Result, codec::read_u16_le, transport::Transport};

/// Input report layout (1x 2-byte)
const INPUT_REPORT_LEN: usize = 2;

/// Open connection to handbrake.
pub struct Handbrake {
    transport: Transport,
}

impl Handbrake {
    /// Opens the connection to pedals.
    pub fn open() -> Result<Self> {
        Ok(Self {
            transport: Transport::open(DeviceKind::Handbrake)?,
        })
    }
}

impl InputDevice for Handbrake {
    type Input = u16;

    fn read_input(&mut self) -> Result<Self::Input> {
        let mut buf = [0u8; INPUT_REPORT_LEN];
        let bytes_read = self.transport.read(&mut buf)?;

        if bytes_read < INPUT_REPORT_LEN {
            return Err(Error::ShortReport {
                expected: INPUT_REPORT_LEN,
                got: bytes_read,
            });
        }

        read_u16_le(&buf).ok_or(Error::ShortReport {
            expected: INPUT_REPORT_LEN,
            got: bytes_read,
        })
    }
}
