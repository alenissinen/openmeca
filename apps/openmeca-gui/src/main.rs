mod components;
mod constants;
mod utils;

use constants::*;
use iced::{
    Color, Element, Length, Subscription, Task, Theme,
    time::{self, Duration},
    widget::{button, column, container, row, scrollable, text},
    window,
};
use meca_hid::{DeviceStatus, PedalChannel, PedalInput, curve::Curve};

use crate::components::{
    curve_editor::curve_editor, curve_points::curve_points, deadzones::deadzones,
    debug_box::debug_box, live_box::live_box, sidebar::sidebar, title_bar::title_bar,
};
use crate::utils::pedal_input::pedal_subscription;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Openmeca")
        .theme(App::theme)
        .centered()
        .decorations(false)
        .default_font(FONT_UI)
        .font(include_bytes!("../fonts/Outfit-Regular.ttf"))
        .font(include_bytes!("../fonts/JetBrainsMono-Regular.ttf"))
        .window_size((1052, 620))
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

    fn pedal_color(&self, theme: &Theme) -> Color {
        match self {
            NavItem::Throttle | NavItem::Brake | NavItem::Clutch => theme.palette().primary,
            _ => COLOR_DIM,
        }
    }

    fn raw_value(&self, input: &PedalInput) -> u16 {
        match self {
            NavItem::Throttle => input.throttle,
            NavItem::Brake => input.brake,
            NavItem::Clutch => input.clutch,
            _ => 0,
        }
    }

    fn channel(&self) -> PedalChannel {
        match self {
            NavItem::Throttle => PedalChannel::Throttle,
            NavItem::Brake => PedalChannel::Brake,
            NavItem::Clutch => PedalChannel::Clutch,
            _ => PedalChannel::Throttle,
        }
    }

    fn curve<'a>(&self, curves: &'a PedalCurves) -> &'a Curve {
        match self {
            NavItem::Throttle => &curves.throttle,
            NavItem::Brake => &curves.brake,
            NavItem::Clutch => &curves.clutch,
            _ => &curves.throttle,
        }
    }

    fn curve_mut<'a>(&self, curves: &'a mut PedalCurves) -> &'a mut Curve {
        match self {
            NavItem::Throttle => &mut curves.throttle,
            NavItem::Brake => &mut curves.brake,
            NavItem::Clutch => &mut curves.clutch,
            _ => &mut curves.throttle,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct PedalCurves {
    throttle: Curve,
    brake: Curve,
    clutch: Curve,
}

#[derive(Debug, Clone)]
pub enum DzEnd {
    Bottom,
    Top,
}

struct App {
    selected: NavItem,
    devices: DeviceStatus,
    pedal_input: PedalInput,
    curves: PedalCurves,
    debug: bool,
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
    PedalInputUpdated(PedalInput),
    SetDeadzone(DzEnd, u8),
    SetCurvePoint(usize, u16),
    ToggleDebug,
}

impl App {
    fn new() -> Self {
        Self {
            selected: NavItem::Throttle,
            devices: DeviceStatus::default(),
            pedal_input: PedalInput::default(),
            curves: PedalCurves::default(),
            debug: false,
            theme: Theme::Oxocarbon,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragWindow => window::oldest().and_then(window::drag),
            Message::Minimize => window::oldest().and_then(|id| window::minimize(id, true)),
            Message::ToggleMaximize => window::oldest().and_then(window::toggle_maximize),
            Message::Close => window::oldest().and_then(window::close),
            Message::Navigate(item) => {
                self.selected = item;
                Task::none()
            }
            Message::RefreshDevices => Task::perform(async { meca_hid::discover() }, |r| {
                Message::DevicesUpdated(r.unwrap_or_default())
            }),
            Message::DevicesUpdated(status) => {
                self.devices = status;
                Task::none()
            }
            Message::PedalInputUpdated(input) => {
                self.pedal_input = input;
                Task::none()
            }
            Message::SetDeadzone(end, value) => {
                let curve = self.selected.curve_mut(&mut self.curves);
                match end {
                    DzEnd::Bottom => curve.set_bottom_dz(value),
                    DzEnd::Top => curve.set_top_dz(100u8 - value),
                }
                Task::none()
            }
            Message::SetCurvePoint(i, y) => {
                let curve = self.selected.curve_mut(&mut self.curves);
                let points = curve.points();
                let mut new_y: [u16; 5] = points[2..=6]
                    .iter()
                    .map(|p| p.y)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                new_y[i] = y;
                curve.set_middle_y(&new_y);
                Task::none()
            }
            Message::ToggleDebug => {
                self.debug = !self.debug;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            time::every(Duration::from_secs(2)).map(|_| Message::RefreshDevices),
            pedal_subscription(),
        ])
    }

    fn view(&self) -> Element<'_, Message> {
        let curve = self.selected.curve(&self.curves);
        let raw = self.selected.raw_value(&self.pedal_input);
        let accent = self.selected.pedal_color(&self.theme);

        let dz_bottom = curve.bottom_dz_percentage();
        let dz_top = curve.top_dz_percentage();

        let y_values = [
            curve.points()[2].y,
            curve.points()[3].y,
            curve.points()[4].y,
            curve.points()[5].y,
            curve.points()[6].y,
        ];

        let report = curve.to_report(self.selected.channel().data_id());

        let debug_toggle = button(
            text(if self.debug { "DEBUG ▲" } else { "DEBUG ▼" })
                .size(10)
                .font(FONT_MONO),
        )
        .on_press(Message::ToggleDebug)
        .style(move |_: &Theme, _| button::Style {
            background: Some(if self.debug {
                iced::Background::Color(COLOR_BORDER)
            } else {
                iced::Background::Color(COLOR_CARD_BG)
            }),
            border: iced::Border {
                color: COLOR_BORDER,
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: if self.debug { accent } else { COLOR_DIM },
            ..Default::default()
        });

        let mut panels = column![
            live_box(raw, accent),
            deadzones(dz_bottom, dz_top, accent),
            curve_points(y_values, accent),
        ]
        .spacing(16);

        if self.debug {
            panels = panels.push(debug_box(report, accent, dz_bottom, dz_top));
        }

        let content = match self.selected.is_connected(&self.devices) {
            true => {
                // Estimated height of the right column
                let column_height: f32 = (120.0 + 16.0) * 2.0 + 105.0 + 130.0 + 16.0;

                container(
                    column![
                        row![iced::widget::Space::new().width(Length::Fill), debug_toggle],
                        row![
                            curve_editor(curve, raw, accent, column_height),
                            scrollable(panels).height(Length::Fill).width(330)
                        ]
                        .spacing(16)
                        .height(Length::Fill),
                    ]
                    .spacing(12),
                )
                .padding(12)
                .width(Length::Fill)
                .height(Length::Fill)
            }
            false => container(
                text(
                    "
                This device isn't connected!\n
                If the device is connected but doesn't show up, \
                try restarting the application.\n
                If the issue persists, please open an issue in github!
                ",
                )
                .size(18)
                .line_height(1.0)
                .width(Length::Fill)
                .height(Length::Fill)
                .center(),
            ),
        };

        let body =
            row![sidebar(&self.selected, &self.devices, &self.theme), content].height(Length::Fill);

        column![title_bar(), body].into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
