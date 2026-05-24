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
mod error;
mod ids;

pub use error::{Error, Result};
pub use ids::{DeviceKind, VENDOR_ID};

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
