mod constants;

use constants::*;
use iced::{
    Alignment, Background, Border, Color, Element, Length, Subscription, Task, Theme,
    time::{self, Duration},
    widget::{Space, button, column, container, mouse_area, row, text},
    window,
};
use meca_hid::DeviceStatus;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Openmeca")
        .theme(App::theme)
        .centered()
        .decorations(false)
        .default_font(FONT_UI)
        .font(include_bytes!("../fonts/Outfit-Regular.ttf"))
        .font(include_bytes!("../fonts/JetBrainsMono-Regular.ttf"))
        .window_size((1152, 720))
        .antialiasing(true)
        .subscription(App::subscription)
        .run()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NavItem {
    Throttle,
    Brake,
    Clutch,
    Handbrake,
    Shifter,
}

impl NavItem {
    fn label(&self) -> &'static str {
        match self {
            NavItem::Throttle => "Throttle",
            NavItem::Brake => "Brake",
            NavItem::Clutch => "Clutch",
            NavItem::Handbrake => "Handbrake",
            NavItem::Shifter => "Shifter",
        }
    }

    fn is_connected(&self, status: &DeviceStatus) -> bool {
        match self {
            NavItem::Throttle | NavItem::Brake | NavItem::Clutch => status.pedals,
            NavItem::Handbrake => status.handbrake,
            NavItem::Shifter => status.shifter,
        }
    }
}

struct App {
    selected: NavItem,
    devices: DeviceStatus,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    DragWindow,
    Minimize,
    ToggleMaximize,
    Close,
    Navigate(NavItem),
    RefreshDevices,
    DevicesUpdated(DeviceStatus),
}

impl App {
    fn new() -> Self {
        Self {
            selected: NavItem::Throttle,
            devices: DeviceStatus::default(),
            theme: Theme::Oxocarbon,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragWindow => window::oldest().and_then(move |id| window::drag(id)),
            Message::Minimize => window::oldest().and_then(move |id| window::minimize(id, true)),
            Message::ToggleMaximize => {
                window::oldest().and_then(move |id| window::toggle_maximize(id))
            }
            Message::Close => window::oldest().and_then(move |id| window::close(id)),
            Message::Navigate(item) => {
                self.selected = item;
                Task::none()
            }
            Message::RefreshDevices => Task::perform(async { meca_hid::discover() }, |result| {
                Message::DevicesUpdated(result.unwrap_or_default())
            }),
            Message::DevicesUpdated(status) => {
                self.devices = status;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(2)).map(|_| Message::RefreshDevices)
    }

    fn view(&self) -> Element<'_, Message> {
        let title_bar = self.title_bar();
        let sidebar = self.sidebar();

        let content = container(text(self.selected.label()).size(14))
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill);

        let body = row![sidebar, content].height(Length::Fill);

        column![title_bar, body].into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn title_bar(&self) -> Element<'_, Message> {
        let title = row![text("Openmeca").size(16)].padding([0, 8]);
        let controls = row![
            title_button("—", Message::Minimize, false),
            title_button("X", Message::Close, true),
        ]
        .spacing(0);

        let bar = row![title, Space::new().width(Length::Fill), controls]
            .align_y(iced::Alignment::Center);

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

    fn sidebar(&self) -> Element<'_, Message> {
        let nav_items = column(
            [
                NavItem::Throttle,
                NavItem::Brake,
                NavItem::Clutch,
                NavItem::Handbrake,
                NavItem::Shifter,
            ]
            .into_iter()
            .map(|item| self.nav_item(item))
            .collect::<Vec<_>>(),
        )
        .spacing(0);

        let count = self.devices.connected_count();
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
        .padding([0, 0])
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(COLOR_SIDEBAR_BG)),
            ..Default::default()
        })
        .into()
    }

    fn nav_item(&self, item: NavItem) -> Element<'_, Message> {
        let connected = item.is_connected(&self.devices);
        let selected = self.selected == item;
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
            self.theme.palette().text
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
}

fn title_button(label: &str, msg: Message, is_close: bool) -> Element<'_, Message> {
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
