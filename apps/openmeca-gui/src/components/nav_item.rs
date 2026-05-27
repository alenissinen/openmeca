use iced::{
    Alignment, Background, Border, Color, Element, Length, Theme,
    widget::{Space, button, container, row, text},
};
use meca_hid::DeviceStatus;

use crate::{
    Message, NavItem,
    constants::{ACCENT_GREEN, ACCENT_RED, COLOR_INACTIVE, FONT_UI},
};

pub fn nav_item<'a, 'b>(
    item: NavItem,
    devices: &'a DeviceStatus,
    selected: NavItem,
    theme: &'b Theme,
) -> Element<'a, Message> {
    let connected = item.is_connected(devices);
    let selected = selected == item;
    let label = item.label();

    let dot_color = if connected { ACCENT_GREEN } else { ACCENT_RED };
    let dot = container(Space::new().height(0).width(0))
        .width(6)
        .height(6)
        .style(move |_: &Theme| container::Style {
            background: Some(Background::Color(dot_color)),
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    let text_color = if selected {
        theme.palette().text
    } else {
        COLOR_INACTIVE
    };

    let label_text = text(label).size(14).font(FONT_UI).color(text_color);

    let content = row![dot, label_text]
        .spacing(10)
        .align_y(Alignment::Center)
        .padding([8, 14]);

    let selected_bg = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.05,
    };

    let hovered_bg = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.03,
    };

    button(content)
        .width(Length::Fill)
        .style(move |_: &Theme, status| {
            let bg = match (selected, status) {
                (true, _) => Some(Background::Color(selected_bg)),
                (false, button::Status::Hovered) => Some(Background::Color(hovered_bg)),
                _ => None,
            };

            button::Style {
                background: bg,
                text_color,
                border: Border::default(),
                snap: true,
                shadow: Default::default(),
            }
        })
        .on_press(Message::Navigate(item))
        .into()
}
