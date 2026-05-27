use iced::{
    Background, Border, Color, Element, Theme,
    widget::{column, container, row, slider, text},
};

use crate::{
    DzEnd, Message,
    constants::{COLOR_BORDER, COLOR_CARD_BG, COLOR_SUBTLE, FONT_MONO},
};

pub fn deadzones<'a>(bottom: u8, top: u8, accent: Color) -> Element<'a, Message> {
    container(
        column![
            text("DEADZONES").size(14).font(FONT_MONO),
            row![
                text("Bottom DZ")
                    .size(12)
                    .font(FONT_MONO)
                    .color(COLOR_SUBTLE)
                    .width(70),
                slider(0u8..=100, bottom, |v| Message::SetDeadzone(
                    DzEnd::Bottom,
                    v
                ))
                .step(1u8)
                .style(move |_: &Theme, _| slider::Style {
                    rail: slider::Rail {
                        backgrounds: (Background::Color(accent), Background::Color(COLOR_BORDER)),
                        width: 4.0,
                        border: Border {
                            radius: 2.0.into(),
                            ..Default::default()
                        }
                    },
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle { radius: 7.0 },
                        background: Background::Color(accent),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    }
                }),
                text(format!("{}%", bottom))
                    .size(14)
                    .font(FONT_MONO)
                    .width(36)
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
            row![
                text("Top DZ")
                    .size(12)
                    .font(FONT_MONO)
                    .color(COLOR_SUBTLE)
                    .width(70),
                slider(0u8..=100, top, |v| Message::SetDeadzone(DzEnd::Top, v))
                    .step(1u8)
                    .style(move |_: &Theme, _| slider::Style {
                        rail: slider::Rail {
                            backgrounds: (
                                Background::Color(accent),
                                Background::Color(COLOR_BORDER),
                            ),
                            width: 4.0,
                            border: Border {
                                radius: 2.0.into(),
                                ..Default::default()
                            },
                        },
                        handle: slider::Handle {
                            shape: slider::HandleShape::Circle { radius: 7.0 },
                            background: Background::Color(accent),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                        },
                    }),
                text(format!("{}%", top)).size(14).font(FONT_MONO).width(36),
            ]
            .spacing(8)
            .align_y(iced::Alignment::Center),
        ]
        .spacing(10),
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
