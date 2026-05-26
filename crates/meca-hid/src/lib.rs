#![forbid(unsafe_code)]
//! `meca-hid` is a driver for MECA EVO sim racing peripherals
//! (pedals, handbrake, sequential shifter), built from the reverse-engineered protocol
//! documented in `docs/`.
//!
//! Each device is its own type because they differ from each other
//! (axis count, whether configuration exists at all). The only thing all of the three devices
//! share at the API level is reading input: [`InputDevice`].
//!

mod codec;
pub mod curve;
mod error;
mod handbrake;
mod ids;
mod pedals;
mod shifter;
mod transport;

pub use error::{Error, Result};
pub use handbrake::Handbrake;
pub use ids::{DeviceKind, VENDOR_ID};
pub use pedals::{PedalChannel, PedalInput, Pedals};
pub use shifter::{Shift, Shifter};

/// A device that produces input reports. The report differs per device so each implementor
/// provides its own [`InputDevice::Input`] type.
///
/// Covers only reading input. Configuration (curves, deadzones, calibration) is on the concrete
/// types.
pub trait InputDevice {
    /// The decoded input this device produces (e.g. only 1 axis, shifter state).
    type Input;

    /// Reads and decodes the next input report.
    fn read_input(&mut self) -> Result<Self::Input>;
}

/// Connection status of all three devices.
#[derive(Debug, Clone, Default)]
pub struct DeviceStatus {
    pub pedals: bool,
    pub handbrake: bool,
    pub shifter: bool,
}

impl DeviceStatus {
    /// Returns the amount of connected devices
    pub fn connected_count(&self) -> u8 {
        [self.pedals, self.handbrake, self.shifter]
            .iter()
            .filter(|&b| *b)
            .count() as u8
    }
}

/// Enumerates connected HID devices and returns connection status of each EVO device.
pub fn discover() -> Result<DeviceStatus> {
    let api = hidapi::HidApi::new()?;
    let list: Vec<&hidapi::DeviceInfo> = api
        .device_list()
        .filter(|d| d.vendor_id() == VENDOR_ID)
        .collect();

    Ok(DeviceStatus {
        pedals: list
            .iter()
            .any(|d| d.product_id() == DeviceKind::Pedals.product_id()),
        handbrake: list
            .iter()
            .any(|d| d.product_id() == DeviceKind::Handbrake.product_id()),
        shifter: list
            .iter()
            .any(|d| d.product_id() == DeviceKind::Shifter.product_id()),
    })
}
