//! Wrapper over `hidapi` shared by all EVO devices.
//!
//! Owns the `hidapi` context and an open `HidDevice` and exposes the operations the protocol
//! needs: reading an input report, getting/setting the 64-byte feature report for configuration
//! (pedals and handbrake).

use hidapi::{HidApi, HidDevice};

use crate::{
    DeviceKind, VENDOR_ID,
    error::{Error, Result},
};

/// An open connection to one EVO device
pub(crate) struct Transport {
    device: HidDevice,
}

impl Transport {
    /// Opens the connected device of given `kind`.
    ///
    /// Returns [`Error::DeviceNotFound`] if no matching device is available.
    pub(crate) fn open(kind: DeviceKind) -> Result<Self> {
        let api = HidApi::new()?;
        let device = api
            .open(VENDOR_ID, kind.product_id())
            .map_err(|_| Error::DeviceNotFound(kind))?;

        Ok(Self { device })
    }

    /// Reads one input report into `buffer`, returns the bytes read.
    /// Blocks until a report arrives (bInterval is 1ms)
    pub(crate) fn read(&self, buffer: &mut [u8]) -> Result<usize> {
        Ok(self.device.read(buffer)?)
    }

    /// Reads the 64-byte feature report (id 0) into a fixed buffer.
    ///
    /// Hidapi expects the report id in byte 0 -> we request 65 bytes and return the
    /// 64-byte payload (1..65)
    pub(crate) fn get_feature(&self) -> Result<[u8; 64]> {
        // Byte 0 is the report id (0)
        let mut buf = [0u8; 65];
        let bytes_read = self.device.get_feature_report(&mut buf)?;

        if bytes_read < 65 {
            return Err(Error::ShortReport {
                expected: 65,
                got: bytes_read,
            });
        }

        let mut payload = [0u8; 64];
        payload.copy_from_slice(&buf[1..]);

        Ok(payload)
    }

    /// Writes the 64-byte feature report (id 0) via SET_REPORT.
    pub(crate) fn set_feature(&self, payload: &[u8; 64]) -> Result<()> {
        // Byte 0 is the report id (0)
        let mut buf = [0u8; 65];

        buf[1..].copy_from_slice(payload);
        self.device.send_feature_report(&buf)?;

        Ok(())
    }
}
