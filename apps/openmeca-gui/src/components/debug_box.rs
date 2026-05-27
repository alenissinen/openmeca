use iced::{
    Background, Border, Color, Element, Theme,
    widget::{column, container, text},
};

use crate::{
    Message,
    constants::{COLOR_BORDER, COLOR_CARD_BG, COLOR_SUBTLE, COLOR_TEXT, FONT_MONO},
};

pub fn debug_box<'a>(
    report: [u16; 32],
    accent: Color,
    dz_bottom: u8,
    dz_top: u8,
) -> Element<'a, Message> {
    // First byte of the report is the data_id
    let data_id = report[0];

    let hex: String = report
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            if i < 31 {
                format!("{:04x} ", v)
            } else {
                format!("{:04x}", v)
            }
        })
        .collect();

    container(
        column![
            text("FEATURE REPORT (32 x uint16)")
                .size(12)
                .font(FONT_MONO)
                .color(accent),
            text(hex).size(10).font(FONT_MONO).color(COLOR_TEXT),
            text(format!(
                "dataId: 0x{:04x} · bottomDZ: {}% · topDZ: {}%",
                data_id, dz_bottom, dz_top
            ))
            .size(10)
            .font(FONT_MONO)
            .color(COLOR_SUBTLE),
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
