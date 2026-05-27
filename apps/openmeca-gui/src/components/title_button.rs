use iced::{
    Element, Length, Theme,
    widget::{button, text},
};

use crate::{Message, constants::TITLE_BAR_HEIGHT};

pub fn title_button(label: &str, msg: Message, is_close: bool) -> Element<'_, Message> {
    button(
        text(label)
            .size(14)
            .center()
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(40.0)
    .height(TITLE_BAR_HEIGHT)
    .style(move |_: &Theme, status| {
        let bg = match (status, is_close) {
            (button::Status::Hovered, true) => iced::Color::from_rgb(0.8, 0.15, 0.15),
            (button::Status::Pressed, true) => iced::Color::from_rgb(0.6, 0.1, 0.1),
            (button::Status::Hovered, false) => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            (button::Status::Pressed, false) => iced::Color::from_rgba(1.0, 1.0, 1.0, 0.15),
            _ => iced::Color::TRANSPARENT,
        };

        let text_color = match (status, is_close) {
            (button::Status::Hovered | button::Status::Pressed, true) => iced::Color::WHITE,
            _ => iced::Color::from_rgb(0.7, 0.7, 0.7),
        };

        button::Style {
            background: Some(bg.into()),
            text_color,
            snap: true,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    })
    .on_press(msg)
    .into()
}
