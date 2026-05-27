use iced::{
    Color, Element, Length,
    widget::{column, container, row, text},
};
use meca_hid::Shift;

use crate::{
    Message,
    constants::{COLOR_BORDER, COLOR_CARD_BG, COLOR_INACTIVE, COLOR_SUBTLE, FONT_MONO, FONT_UI},
};

fn shift_indicator<'a>(label: &'static str, active: bool, accent: Color) -> Element<'a, Message> {
    container(text(label).size(28).font(FONT_MONO).color(if active {
        accent
    } else {
        COLOR_INACTIVE
    }))
    .style(move |_: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(if active {
            Color { a: 0.08, ..accent }
        } else {
            COLOR_CARD_BG
        })),
        border: iced::Border {
            color: if active { accent } else { COLOR_BORDER },
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .padding(40)
    .width(Length::Fill)
    .height(Length::Fill)
    .center(Length::Fill)
    .into()
}

pub fn shifter_view<'a>(shift: Shift, accent: Color) -> Element<'a, Message> {
    container(
        column![
            text("SHIFTER TEST").size(18).font(FONT_UI),
            row![
                shift_indicator("▲  UP", shift.up, accent),
                shift_indicator("▼  DOWN", shift.down, accent),
            ]
            .spacing(16)
        ]
        .spacing(20),
    )
    .style(|_: &iced::Theme| container::Style {
        ..Default::default()
    })
    .padding(20)
    .width(Length::Fill)
    .into()
}
