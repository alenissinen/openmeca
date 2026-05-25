//! Opens connection to the EVO1 pedals and reads live pedal values.
//!
//! Usage: `cargo run --example pedals`
//! Requires a connected MECA EVO1 pedal set.

use std::process::ExitCode;

use meca_hid::{InputDevice, PedalInput, Pedals};

fn main() -> ExitCode {
    let mut pedals = match Pedals::open() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not open connection to pedals: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Pedals connected. Press pedals (Ctrl+C to quit).");
    println!("{:>8} {:>8} {:>8}", "THROTTLE", "BRAKE", "CLUTCH");

    let mut prev = PedalInput::default();
    loop {
        match pedals.read_input() {
            Ok(input) => {
                if input != prev {
                    println!(
                        "{:>8} {:>8} {:>8}",
                        input.throttle, input.brake, input.clutch,
                    );
                    prev = input;
                }
            }
            Err(e) => {
                eprintln!("Read error: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
}
