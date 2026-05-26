//! Opens connection to the EVO handbrake and reads live values.
//!
//! Usage: `cargo run --example handbrake`
//! Requires a connected MECA EVO HB.

use std::process::ExitCode;

use meca_hid::{Handbrake, InputDevice};

fn main() -> ExitCode {
    let mut hb = match Handbrake::open() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Could not open connection to handbrake: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Handbrake connected. Pull the lever (Ctrl+C to quit).");

    let mut last = 0u16;
    loop {
        match hb.read_input() {
            Ok(value) => {
                if value != last {
                    println!("{value:>5} / 4096  ({:.1}%)", value as f32 / 4096.0 * 100.0);
                    last = value;
                }
            }
            Err(e) => {
                eprintln!("Read error: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
}
