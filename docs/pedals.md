# MECA EVO1 Pedals (load cell) - HID Protocol

Reverse-engineering notes for the MECA EVO1 load cell pedals.

## Device identity

Tool used: `tools/dump.py`

| Field        | Value                      |
| ------------ | -------------------------- |
| VID:PID      | `04d8:ea35`                |
| Manufacturer | MECA-SIM-HARDWARE s.r.o    |
| Product      | MECA EVO1                  |
| Usage page   | `0x0001` (Generic Desktop) |
| Usage        | `0x0004` (Joystick)        |

Vendor ID `04d8` belongs to Microchip Technology, Inc.
