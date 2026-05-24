//! USB identifiers for the MECA EVO device family.
//! All devices share Microchip vendor id, product id selects the device.

/// USB vendor id on all EVO devices (Microchip Technology Inc.)
pub const VENDOR_ID: u16 = 0x04D8;

/// One of the three EVO devices, identified by USB product id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    Pedals,
    Handbrake,
    Shifter,
}

impl DeviceKind {
    /// The USB product id for current device.
    pub const fn product_id(self) -> u16 {
        match self {
            DeviceKind::Pedals => 0xEA35,
            DeviceKind::Handbrake => 0xEA34,
            DeviceKind::Shifter => 0xEA33,
        }
    }

    /// Maps a USB product id back to a device if it is a known one.
    pub const fn from_product_id(pid: u16) -> Option<Self> {
        match pid {
            0xEA35 => Some(DeviceKind::Pedals),
            0xEA34 => Some(DeviceKind::Handbrake),
            0xEA33 => Some(DeviceKind::Shifter),
            _ => None,
        }
    }
}
