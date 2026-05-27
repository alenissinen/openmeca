use iced::{
    Background, Border, Color, Element, Theme,
    widget::{column, container, progress_bar, row, text},
};

use crate::{
    Message,
    constants::{COLOR_BORDER, COLOR_CARD_BG, COLOR_SUBTLE, FONT_MONO},
};

pub fn live_box<'a>(raw: u16, accent: Color) -> Element<'a, Message> {
    let pct = (raw as f32 / 4096.0 * 100.0).round() as u32;

    container(
        column![
            text("LIVE").size(12).font(FONT_MONO),
            row![
                column![
                    text(format!("{}%", pct))
                        .size(28)
                        .font(FONT_MONO)
                        .color(accent),
                    text("CALIBRATED")
                        .size(10)
                        .font(FONT_MONO)
                        .color(COLOR_SUBTLE),
                ]
                .spacing(2),
                column![
                    text(format!("{}", raw)).size(28).font(FONT_MONO),
                    text("RAW OUT").size(10).font(FONT_MONO).color(COLOR_SUBTLE),
                ]
                .spacing(2),
            ]
            .spacing(16),
            progress_bar(0.0..=4096.0, raw as f32)
                .girth(4)
                .style(move |_: &Theme| progress_bar::Style {
                    background: Background::Color(COLOR_BORDER),
                    bar: Background::Color(accent),
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                }),
        ]
        .spacing(8),
    )
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
