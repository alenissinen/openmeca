mod components;
mod constants;

use constants::*;
use iced::{
    Element, Length, Subscription, Task, Theme,
    time::{self, Duration},
    widget::{column, container, row, text},
    window,
};
use meca_hid::DeviceStatus;

use crate::components::{sidebar::sidebar, title_bar::title_bar};

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
        let content = container(text(self.selected.label()).size(14))
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill);

        let body =
            row![sidebar(&self.selected, &self.devices, &self.theme), content].height(Length::Fill);

        column![title_bar(), body].into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
