# MECA EVO HB (Handbrake) - HID Protocol

Reverse-engineering notes for the MECA EVO Handbrake.  
The handbrake is basically the same as the brake pedal.

## Device identity

Tools used: `tools/dump.py`, USBTreeView

| Field        | Value                      |
| ------------ | -------------------------- |
| VID:PID      | `04d8:ea34`                |
| Manufacturer | MECA-SIM-HARDWARE s.r.o    |
| Product      | MECA EVO1 HB               |
| Usage page   | `0x0001` (Generic Desktop) |
| Usage        | `0x0004` (Joystick)        |

Vendor ID `04d8` belongs to Microchip Technology, Inc.

## Input report structure

Tool used: `tools/dump.py`

2-byte input report, no report ID. Single 16-bit LE axis, range 0-4096 (0x0000-0x1000).
Same format as the pedals but only one axis instead of three.

**Observation**: pulled handbrake from resting position to calibrated max, values went from
`00 00` to `00 10`.

## Config protocol (Feature report, 64 bytes)

Identical structure to the EVO1 pedals: config written via SET_REPORT. All pedal rules and structures apply, see [pedal documentation](./pedals.md#config-protocol-feature-report-64-bytes)!

- dataId = 0xF101 (same as clutch but since handbrake has its own microcontroller this doesn't matter).
- Calibrated to force instead of travel (same as brake).
