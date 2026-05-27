use iced::{
    Alignment, Background, Border, Color, Element, Length, Theme,
    widget::{column, container, row, text},
};

use crate::{
    Message,
    constants::{COLOR_BORDER, COLOR_CARD_BG, COLOR_SUBTLE, FONT_MONO},
};

pub fn curve_points<'a>(y_values: [u16; 5], accent: Color) -> Element<'a, Message> {
    let points: Vec<Element<Message>> = y_values
        .iter()
        .enumerate()
        .map(|(i, &y)| {
            container(
                column![
                    text(format!("{}", y))
                        .size(12)
                        .font(FONT_MONO)
                        .color(accent),
                    text(format!("pt{}", i + 1))
                        .size(8)
                        .font(FONT_MONO)
                        .color(COLOR_SUBTLE)
                ]
                .spacing(2)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
        })
        .collect();

    container(column![text("CURVE POINTS").size(14).font(FONT_MONO), row(points),].spacing(8))
        .padding(16)
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(COLOR_CARD_BG)),
            border: Border {
                color: COLOR_BORDER,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
}
