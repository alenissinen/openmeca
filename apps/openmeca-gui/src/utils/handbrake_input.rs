use std::time::Duration;

use iced::{
    Subscription,
    futures::{SinkExt, channel::mpsc},
};
use meca_hid::{Handbrake, InputDevice};

use crate::Message;

fn handbrake_stream() -> impl iced::futures::Stream<Item = Message> {
    let (mut tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        loop {
            let mut handbrake = loop {
                let result = tokio::task::spawn_blocking(Handbrake::open).await;

                match result {
                    Ok(Ok(h)) => break h,
                    _ => tokio::time::sleep(Duration::from_millis(500)).await,
                }
            };

            loop {
                let (h, result) = tokio::task::spawn_blocking(move || {
                    let input = handbrake.read_input();
                    (handbrake, input)
                })
                .await
                .expect("spawn_blocking panicked");

                handbrake = h;

                match result {
                    Ok(input) => {
                        if tx
                            .send(Message::HandbrakeInputUpdated(input))
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                    Err(_) => break,
                }
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });

    rx
}

pub fn handbrake_subscription() -> Subscription<Message> {
    Subscription::run(handbrake_stream)
}
