use iced::{
    Element, Font, Length, Task, Theme,
    widget::{Space, button, container, mouse_area, row, text},
    window,
};

// Font used for UI
const FONT_UI: Font = Font::with_name("Outfit");

// Font used for pedal values and other numbers
const FONT_MONO: Font = Font::with_name("JetBrains Mono");

// Height of the custom title bar
const TITLE_BAR_HEIGHT: f32 = 36.0;

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
        .run()
}

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {
    DragWindow,
    Minimize,
    ToggleMaximize,
    Close,
}

impl App {
    fn new() -> Self {
        Self
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragWindow => window::oldest().and_then(move |id| window::drag(id)),
            Message::Minimize => window::oldest().and_then(move |id| window::minimize(id, true)),
            Message::ToggleMaximize => {
                window::oldest().and_then(move |id| window::toggle_maximize(id))
            }
            Message::Close => window::oldest().and_then(move |id| window::close(id)),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title_bar = self.title_bar();

        iced::widget::column![title_bar].into()
    }

    fn theme(&self) -> Theme {
        Theme::Oxocarbon
    }

    fn title_bar(&self) -> Element<'_, Message> {
        let title = row![text("Openmeca").size(16)].padding([0, 8]);
        let controls = row![
            title_button("—", Message::Minimize, false),
            title_button("X", Message::Close, true),
        ]
        .spacing(0);

        let bar = row![title, Space::new().width(Length::Fill), controls]
            .align_y(iced::Alignment::Center)
            .padding([0, 0]);

        mouse_area(
            container(bar)
                .width(Length::Fill)
                .height(TITLE_BAR_HEIGHT)
                .style(|_theme: &Theme| container::Style {
                    background: Some(iced::Color::from_rgb(0.08, 0.08, 0.08).into()),
                    ..Default::default()
                }),
        )
        .on_press(Message::DragWindow)
        .on_double_click(Message::ToggleMaximize)
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
    .style(move |_theme: &Theme, status| {
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
