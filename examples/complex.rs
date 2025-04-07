use iced::widget::{
    button, column, container, horizontal_rule, pick_list, responsive, row,
    rule, slider, text, toggler, vertical_rule,
};
use iced::{Center, Element, Fill, Task, Theme};

use dragking::DragEvent;

pub fn main() -> iced::Result {
    iced::application("iced — Draggable Widgets", App::update, App::view)
        .window(iced::window::Settings {
            size: iced::Size::new(600.0, 400.0),
            ..Default::default()
        })
        .theme(App::theme)
        .run_with(App::new)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    Other,
}

impl Language {
    const ALL: [Language; 6] = [
        Language::C,
        Language::Elm,
        Language::Ruby,
        Language::Haskell,
        Language::Rust,
        Language::Other,
    ];
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Rust => "Rust",
                Language::Elm => "Elm",
                Language::Ruby => "Ruby",
                Language::Haskell => "Haskell",
                Language::C => "C",
                Language::Other => "Other",
            }
        )
    }
}

#[derive(Default)]
struct App {
    widgets: Vec<WidgetType>,
    mode: Mode,
    // Widget states
    slider_value: f32,
    selected_language: Option<Language>,
    toggle_value: bool,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum Mode {
    Row,
    #[default]
    Column,
}

#[derive(Debug, Clone)]
enum WidgetType {
    Slider,
    Button,
    PickList,
    Toggler,
    Text,
}

#[derive(Debug, Clone)]
enum Message {
    Reorder(DragEvent),
    SwitchMode(Mode),
    SliderChanged(f32),
    ButtonPressed,
    LanguageSelected(Language),
    TogglerChanged(bool),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                widgets: vec![
                    WidgetType::Slider,
                    WidgetType::Button,
                    WidgetType::PickList,
                    WidgetType::Toggler,
                    WidgetType::Text,
                ],
                slider_value: 50.0,
                selected_language: None,
                toggle_value: false,
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SwitchMode(mode) => {
                self.mode = mode;
            }
            Message::Reorder(event) => {
                if let DragEvent::Dropped {
                    index,
                    target_index,
                } = event
                {
                    let item = self.widgets.remove(index);
                    self.widgets.insert(target_index, item);
                }
            }
            Message::SliderChanged(value) => {
                self.slider_value = value;
            }
            Message::ButtonPressed => {
                // Handle button press
            }
            Message::LanguageSelected(language) => {
                self.selected_language = Some(language);
            }
            Message::TogglerChanged(value) => {
                self.toggle_value = value;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let items = || {
            self.widgets.iter().map(|widget| match widget {
                WidgetType::Slider => Element::from(
                    slider(
                        0.0..=100.0,
                        self.slider_value,
                        Message::SliderChanged,
                    )
                    .width(150),
                ),
                WidgetType::Button => button(text("Click me!").size(16))
                    .on_press(Message::ButtonPressed)
                    .padding(8)
                    .into(),
                WidgetType::PickList => pick_list(
                    &Language::ALL[..],
                    self.selected_language,
                    Message::LanguageSelected,
                )
                .placeholder("Choose language...")
                .padding(8)
                .width(150)
                .into(),
                WidgetType::Toggler => toggler(self.toggle_value)
                    .on_toggle(Message::TogglerChanged)
                    .width(100)
                    .into(),
                WidgetType::Text => text("Drag me around!").size(16).into(),
            })
        };

        let drag: Element<'_, Message> = match self.mode {
            Mode::Column => responsive(move |size| {
                dragking::column(items().map(|item| {
                    row![vertical_rule(5).style(handle), item]
                        .align_y(Center)
                        .spacing(5)
                        .width(size.width)
                        .into()
                }))
                .spacing(5)
                .deadband_zone(0.0)
                .on_drag(Message::Reorder)
                .align_x(Center)
                .into()
            })
            .into(),
            Mode::Row => responsive(move |size| {
                dragking::row(items().map(|item| {
                    column![horizontal_rule(5).style(handle), item]
                        .height(size.height)
                        .align_x(Center)
                        .spacing(5)
                        .into()
                }))
                .spacing(5)
                .on_drag(Message::Reorder)
                .align_y(Center)
                .into()
            })
            .into(),
        };

        container(
            column![
                row![
                    text("Drag widgets around!").width(Fill),
                    pick_list(
                        [Mode::Row, Mode::Column],
                        Some(&self.mode),
                        Message::SwitchMode,
                    )
                ],
                container(drag)
                    .padding(20)
                    .width(Fill)
                    .height(Fill)
                    .align_x(Center)
                    .align_y(Center)
                    .style(|_| container::Style {
                        border: iced::Border {
                            color: iced::Color::BLACK.scale_alpha(0.2),
                            width: 1.0,
                            radius: 5.0.into(),
                        },
                        ..Default::default()
                    })
            ]
            .align_x(Center)
            .spacing(5),
        )
        .padding(20)
        .height(Fill)
        .width(Fill)
        .align_y(Center)
        .align_x(Center)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

fn handle(_theme: &Theme) -> rule::Style {
    rule::Style {
        width: 5,
        radius: 0.into(),
        color: iced::Color::BLACK.scale_alpha(0.2),
        fill_mode: rule::FillMode::Full,
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Column => write!(f, "Column"),
            Mode::Row => write!(f, "Row"),
        }
    }
}
