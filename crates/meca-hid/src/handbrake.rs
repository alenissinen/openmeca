//! MECA EVO HB - Load Cell Handbrake
//!
//! 2-byte input report containing a single 16-bit axis data (see `docs/handbrake.md`).
//! Shares the same curve, deadzone and feature report data as the pedals.

use crate::{
    DeviceKind, Error, InputDevice, Result,
    codec::{decode_u16_le_buffer, encode_u16_le_buffer, read_u16_le},
    curve::Curve,
    transport::Transport,
};

/// Input report layout (1x 2-byte)
const INPUT_REPORT_LEN: usize = 2;

/// Open connection to handbrake.
pub struct Handbrake {
    transport: Transport,
}

impl Handbrake {
    /// Handbrake dataId
    const DATA_ID: u16 = 0xF101;

    /// Opens the connection to pedals.
    pub fn open() -> Result<Self> {
        Ok(Self {
            transport: Transport::open(DeviceKind::Handbrake)?,
        })
    }

    /// Read handbrake feature report and convert it to a curve.
    pub fn read_curve(&self) -> Result<Curve> {
        let bytes = self.transport.get_feature()?;
        let data = decode_u16_le_buffer(&bytes);

        Curve::from_report(&data).ok_or(Error::ShortReport {
            expected: 19,
            got: data.len(),
        })
    }

    /// Write the current curve as a feature report into the device.
    pub fn write_curve(&self, curve: &Curve) -> Result<()> {
        let report = curve.to_report(Self::DATA_ID);
        let bytes = encode_u16_le_buffer(&report);

        self.transport.set_feature(&bytes)
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
