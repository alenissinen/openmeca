use iced::{
    Background, Border, Element, Length, Theme,
    widget::{Space, column, container, row, text},
};
use meca_hid::DeviceStatus;

use crate::{
    Message, NavItem,
    constants::{ACCENT_GREEN, COLOR_DIM, COLOR_SIDEBAR_BG, FONT_MONO, SIDEBAR_WIDTH},
};

use super::nav_item::nav_item;

pub fn sidebar<'a>(
    selected: &'a NavItem,
    devices: &'a DeviceStatus,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let nav_items = column(
        [
            NavItem::Throttle,
            NavItem::Brake,
            NavItem::Clutch,
            NavItem::Handbrake,
            NavItem::Shifter,
        ]
        .into_iter()
        .map(|item| nav_item(item, devices, selected.clone(), theme))
        .collect::<Vec<_>>(),
    )
    .spacing(0);

    let count = devices.connected_count();
    let status_color = if count > 0 { ACCENT_GREEN } else { COLOR_DIM };
    let status_label = if count > 0 {
        format!(
            "{} device{} connected",
            count,
            if count == 1 { "" } else { "s" }
        )
    } else {
        "No devices connected".to_string()
    };

    let status = row![
        container(Space::new())
            .width(8)
            .height(8)
            .style(move |_: &Theme| container::Style {
                background: Some(Background::Color(status_color)),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }),
        text(status_label).size(11).font(FONT_MONO).color(COLOR_DIM),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center)
    .padding([0, 12]);

    container(column![
        nav_items,
        Space::new().height(Length::Fill),
        container(status).width(Length::Fill).padding([12, 0]),
    ])
    .width(SIDEBAR_WIDTH)
    .height(Length::Fill)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(COLOR_SIDEBAR_BG)),
        ..Default::default()
    })
    .into()
}
