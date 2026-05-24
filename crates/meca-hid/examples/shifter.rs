//! Opens connection to the EVO shifter and prints every shift event.
//!
//! Usage: `cargo run --example shifter`
//! Requires a connected MECA EVO SQ.

use meca_hid::{InputDevice, Shift, Shifter};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut shifter = match Shifter::open() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not open connection to shifter: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Shifter connected. Push or pull the lever (Ctrl+C to quit)");

    // Print only on change to prevent spam
    let mut prev = Shift::default();
    loop {
        match shifter.read_input() {
            Ok(shift) => {
                if shift != prev {
                    report(shift);
                    prev = shift;
                }
            }
            Err(e) => {
                eprintln!("Read error: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
}

/// Prints a human-readable line for the current shift state.
fn report(shift: Shift) {
    match (shift.up, shift.down) {
        (false, false) => println!("idle"),
        (true, false) => println!("upshift"),
        (false, true) => println!("downshift"),
        (true, true) => println!("both"),
    }
}
