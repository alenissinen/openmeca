# MECA EVO SQ (Sequential Shifter) - HID Protocol

Reverse-engineering notes for the MECA EVO SQ sequential shifter.
The shifter is a digital (button) device, not an analog axis.

It should also be noted that the shifter can be mounted to a sim rig the other way around which means that `01` is push and `02` is pull, but this doesn't matter since you map these in game to shift up and shift down yourself.

## Device identity

Tools used: `tools/dump.py`, USBTreeView

| Field        | Value                      |
| ------------ | -------------------------- |
| VID:PID      | `04d8:ea33`                |
| Manufacturer | MECA-SIM-HARDWARE s.r.o    |
| Product      | MECA EVO1 SQ               |
| Usage page   | `0x0001` (Generic Desktop) |
| Usage        | `0x0004` (Joystick)        |

Vendor ID `04d8` belongs to Microchip Technology, Inc.

## Input report structure

Tool used: `tools/dump.py`

| Value  | Control bit   | Meaning                      |
| ------ | ------------- | ---------------------------- |
| `0x00` | -             | idle                         |
| `0x01` | 0 (0000 0001) | pull toward driver (upshift) |
| `0x02` | 1 (0000 0010) | push away (downshift)        |

`0x03` (both bits) is logically possible but can't be reproduced since pushing and pulling at the same time is somewhat impossible.

**Observation**: `python dump.py 04d8 ea33` -> idle = `00`, pull = `01`, push = `02`. Returns to `00` (idle) every time.

## Config

The shifter doesn't have a feature report. `read_feature` fails with an I/O error (device rejects the GET_REPORT). The shifter is a pure input device which means that it doesn't have any configurable features etc.
