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

## Report descriptor

Tools used: Wireshark, USBPcap

37-byte HID report descriptor.

Raw bytes:
05 01 09 04 a1 01 09 33 09 34 09 35 15 00 26 00 10 75 10 95 03 81 02
09 00 15 00 26 00 10 75 10 95 20 b1 02 c0

Decoded:

| Bytes      | Item                         | Meaning                 |
| ---------- | ---------------------------- | ----------------------- |
| `05 01`    | Usage Page (Generic Desktop) |                         |
| `09 04`    | Usage (Joystick)             |                         |
| `a1 01`    | Collection (Application)     |                         |
| `09 33`    | Usage (Rx)                   | axis 1                  |
| `09 34`    | Usage (Ry)                   | axis 2                  |
| `09 35`    | Usage (Rz)                   | axis 3                  |
| `15 00`    | Logical Minimum (0)          |                         |
| `26 00 10` | Logical Maximum (4096)       | confirms range          |
| `75 10`    | Report Size (16)             | 16-bit axes             |
| `95 03`    | Report Count (3)             | 3 axes                  |
| `81 02`    | Input (Data,Var,Abs)         | 6-byte input report     |
| `09 00`    | Usage (Undefined)            | vendor specific blob?   |
| `15 00`    | Logical Minimum (0)          |                         |
| `26 00 10` | Logical Maximum (4096)       |                         |
| `75 10`    | Report Size (16)             |                         |
| `95 20`    | Report Count (32)            | 32 x 16 bits = 64 bytes |
| `b1 02`    | Feature (Data,Var,Abs)       | 64-byte feature report  |
| `c0`       | End Collection               |                         |

### Findings

- Input report confirmed: 3x uint16 (LE) axes, logical max 4096. Axes are reported as Rx/Ry/Rz.
- 64-byte feature report, most likely carrier of configuration data (calibration, deadzones, curve). Explains interrupt OUT endpoint.
