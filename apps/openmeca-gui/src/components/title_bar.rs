use iced::{
    Background, Element, Length, Theme,
    widget::{Space, container, mouse_area, row, text},
};

use crate::{
    Message,
    constants::{COLOR_TITLE_BG, TITLE_BAR_HEIGHT},
};

use super::title_button::title_button;

pub fn title_bar<'a>() -> Element<'a, Message> {
    let title = row![text("Openmeca").size(16)].padding([0, 8]);
    let controls = row![
        title_button("—", Message::Minimize, false),
        title_button("X", Message::Close, true),
    ]
    .spacing(0);

    let bar =
        row![title, Space::new().width(Length::Fill), controls].align_y(iced::Alignment::Center);

    mouse_area(
        container(bar)
            .width(Length::Fill)
            .height(TITLE_BAR_HEIGHT)
            .style(|_: &Theme| container::Style {
                background: Some(Background::Color(COLOR_TITLE_BG)),
                ..Default::default()
            }),
    )
    .on_press(Message::DragWindow)
    .on_double_click(Message::ToggleMaximize)
    .into()
}
