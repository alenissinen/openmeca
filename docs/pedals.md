# MECA EVO1 Pedals (load cell) - HID Protocol

Reverse-engineering notes for the MECA EVO1 load cell pedals.

## Device identity

Tools used: `tools/dump.py`, USBTreeView

| Field        | Value                      |
| ------------ | -------------------------- |
| VID:PID      | `04d8:ea35`                |
| Manufacturer | MECA-SIM-HARDWARE s.r.o    |
| Product      | MECA EVO1                  |
| Usage page   | `0x0001` (Generic Desktop) |
| Usage        | `0x0004` (Joystick)        |

Vendor ID `04d8` belongs to Microchip Technology, Inc.

## Input report structure

Tool used: `tools/dump.py`

6-byte input report, no report ID.

| Offset | Bits | Field    | Type   | Endianness | Range         |
| ------ | ---- | -------- | ------ | ---------- | ------------- |
| 0      | 16   | Clutch   | uint16 | LE         | 0x0000-0x1000 |
| 2      | 16   | Brake    | uint16 | LE         | 0x0000-0x1000 |
| 4      | 16   | Throttle | uint16 | LE         | 0x0000-0x1000 |

Max value = 4096 (0x1000), not raw 12-bit ADC range (max would be 4095).

**Observation**: pressed pedals in throttle -> brake -> clutch order.

- Throttle press changed bytes 4 and 5 from `00 00 00 00 00 00` (pedal resting) to `00 00 00 00 00 10` (pedal fully pressed).
- Brake press changed bytes 2 and 3, same range.
- Clutch press changed bytes 0 and 1, same range.
- LE confirmed by the fact that max value is 4096 in the original control panel and when pedal is fully pressed, the bits sent from the device are `00 10`.
- No report id since the report consists of 6 8-bit values and bit 0 is clutch data.

## USB Endpoints

Tool used: USBTreeView

| Endpoint | Direction | Type      | Max packet size | Purpose               |
| -------- | --------- | --------- | --------------- | --------------------- |
| `0x01`   | OUT       | Interrupt | 64 bytes        | TBD                   |
| `0x81`   | IN        | Interrupt | 64 bytes        | Continuous pedal data |
