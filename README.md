# Openmeca

An open-source control panel and HID protocol documentation for the MECA EVO sim racing peripherals: load cell pedals, handbrake and shifter.

> [!IMPORTANT]  
> This is an independent, unofficial project and is **not** affiliated with Meca Sim Hardware.
> "MECA", "Meca" and product names are used only to describe hardware compatibility.
> If you are affiliated with Meca Sim Hardware, please send email to me! Email address can be found from my github profile.

## Why this exists

I own the MECA EVO1 pedals, EVO handbrake and EVO sequential shifter. After resetting my windows installation, I faced a problem: downloading the official control panel is really hard since Meca Sim Hardware seems to be no longer operating. Without it calibration, deadzones and curve settings cannot be configured.

This project has two goals:

1. Reverse engineer and document the USB HID protocol of the EVO series to make it easier for others to build software around the hardware.
2. Build an open-source control panel so the hardware stays usable whether you have the original control panel or not. For example if you buy used EVO1 pedals it is nice to be able to use them.

This is also a learning exercise combined to a real problem because low-level (device) programming is something I would like to make my living with in the future.

## Repository structure

- `docs/`: notes and protocol documentation, each device has its own file (`pedals.md` etc).
- `tools/`: scripts etc. used during reverse engineering.

## License

MIT
See [LICENSE](LICENSE) for details.
