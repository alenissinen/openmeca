use std::time::Duration;

use iced::{
    Subscription,
    futures::{SinkExt, channel::mpsc},
};
use meca_hid::{InputDevice, Pedals};

use crate::Message;

// Builds a stream that reads every HID report and yields
// `Message::PedalInputUpdated`. Reconnects on disconnect.
fn pedal_stream() -> impl iced::futures::Stream<Item = Message> {
    let (mut tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        loop {
            // Open the device (retry interval 500ms)
            let mut pedals = loop {
                let result = tokio::task::spawn_blocking(Pedals::open).await;

                match result {
                    Ok(Ok(p)) => break p,
                    _ => tokio::time::sleep(Duration::from_millis(500)).await,
                }
            };

            // Read loop, keep device open and yield reports
            loop {
                let (r, result) = tokio::task::spawn_blocking(move || {
                    let input = pedals.read_input();
                    (pedals, input)
                })
                .await
                .expect("spawn_blocking panicked");

                pedals = r;

                match result {
                    Ok(input) => {
                        if tx.send(Message::PedalInputUpdated(input)).await.is_err() {
                            return; // subscription was dropped
                        }
                    }
                    Err(_) => break, // device lost, fallback to reconnect loop
                }
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });

    rx
}

// Helper function to create the subscription
pub fn pedal_subscription() -> Subscription<Message> {
    Subscription::run(pedal_stream)
}
