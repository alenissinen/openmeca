use std::time::Duration;

use iced::{
    Subscription,
    futures::{SinkExt, channel::mpsc},
};
use meca_hid::{InputDevice, Shifter};

use crate::Message;

fn shifter_stream() -> impl iced::futures::Stream<Item = Message> {
    let (mut tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        loop {
            let mut shifter = loop {
                let result = tokio::task::spawn_blocking(Shifter::open).await;

                match result {
                    Ok(Ok(s)) => break s,
                    _ => tokio::time::sleep(Duration::from_millis(500)).await,
                }
            };

            loop {
                let (s, result) = tokio::task::spawn_blocking(move || {
                    let input = shifter.read_input();
                    (shifter, input)
                })
                .await
                .expect("spawn_blocking panicked");

                shifter = s;

                match result {
                    Ok(input) => {
                        if tx.send(Message::ShifterInputUpdated(input)).await.is_err() {
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

pub fn shifter_subscription() -> Subscription<Message> {
    Subscription::run(shifter_stream)
}
