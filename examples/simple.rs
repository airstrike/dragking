use iced::widget::{
    button, checkbox, column, container, horizontal_space, pick_list, row, text,
};
use iced::Length::Fill;
use iced::{Center, Element, Task, Theme};

use dragking::{DragEvent, DropPosition};

pub fn main() -> iced::Result {
    iced::application("iced — Draggable Widgets", App::update, App::view)
        .window(iced::window::Settings {
            size: iced::Size::new(400.0, 400.0),
            ..Default::default()
        })
        .theme(App::theme)
        .run_with(App::new)
}

#[derive(Default)]
struct App {
    elements: Vec<String>,
    mode: Mode,
    allow_dragging: bool,
    last_clicked: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
enum Mode {
    #[default]
    Row,
    Column,
}

#[derive(Debug, Clone)]
enum Message {
    Reorder(DragEvent),
    SwitchMode(Mode),
    ToggleDragging(bool),
    Clicked(String),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                elements: vec![
                    "Apple".to_string(),
                    "Banana".to_string(),
                    "Cherry".to_string(),
                    "Date".to_string(),
                    "Elderberry".to_string(),
                ],
                allow_dragging: true,
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Clicked(str) => {
                self.last_clicked = str;
            }
            Message::SwitchMode(mode) => {
                self.mode = mode;
            }
            Message::ToggleDragging(boolean) => {
                self.allow_dragging = boolean;
            }
            Message::Reorder(event) => {
                match event {
                    DragEvent::Picked { .. } => {
                        // Optionally handle pick event
                    }
                    DragEvent::Dropped {
                        index,
                        target_index,
                        drop_position,
                    } => {
                        // Update self.elements based on index, target_index, drop_position
                        match drop_position {
                            DropPosition::Before | DropPosition::After => {
                                if target_index != index
                                    && target_index != index + 1
                                {
                                    let item = self.elements.remove(index);
                                    let insert_index = if index < target_index {
                                        target_index - 1
                                    } else {
                                        target_index
                                    };
                                    self.elements.insert(insert_index, item);
                                }
                            }
                            DropPosition::Swap => {
                                if target_index != index {
                                    self.elements.swap(index, target_index);
                                }
                            }
                        }
                    }
                    DragEvent::Canceled { .. } => {
                        // Optionally handle cancel event
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let items = self.elements.iter().map(|label| pickme(label));
        let drag: Element<'_, Message> = match self.mode {
            Mode::Column => dragking::column(items.collect::<Vec<_>>())
                .spacing(5)
                // For the column example only, set the deadband_zone to zero
                .deadband_zone(0.0)
                .on_drag_maybe(self.allow_dragging.then_some(Message::Reorder))
                // Alternatively use `on_drag` to always receive drag events
                // .on_drag(Message::Reorder)
                .align_x(Center)
                .into(),
            Mode::Row => dragking::row(items.collect::<Vec<_>>())
                .spacing(5)
                // For the row example only, show a totally custom Style
                .style(|theme| dragking::row::Style {
                    scale: 1.2,
                    moved_item_overlay: iced::Color::BLACK
                        .scale_alpha(0.25)
                        .into(),
                    ghost_background: iced::color![170.0, 0.0, 0.0]
                        .scale_alpha(0.25)
                        .into(),
                    ghost_border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: 5.0.into(),
                    },
                    ..dragking::row::default(theme)
                })
                .align_y(Center)
                .on_drag_maybe(self.allow_dragging.then_some(Message::Reorder))
                // Alternatively use `on_drag` to always receive drag events
                // .on_drag(Message::Reorder)
                .into(),
        };

        let toggle = checkbox("Enable dragging", self.allow_dragging)
            .text_line_height(1.0)
            .on_toggle(Message::ToggleDragging);

        container(
            column![
                row![
                    toggle,
                    horizontal_space(),
                    pick_list(
                        [Mode::Row, Mode::Column],
                        Some(&self.mode),
                        Message::SwitchMode,
                    )
                    .text_line_height(1.0)
                ]
                .spacing(5),
                container(drag)
                    .padding(20)
                    .width(Fill)
                    .height(Fill)
                    .align_x(Center)
                    .align_y(Center)
                    .style(|_| {
                        container::Style {
                            border: iced::Border {
                                color: iced::Color::BLACK.scale_alpha(0.2),
                                width: 1.0,
                                radius: 5.0.into(),
                            },
                            ..Default::default()
                        }
                    }),
                i_like(&self.last_clicked),
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
        Theme::TokyoNightLight
    }
}

fn pickme(label: &str) -> Element<'_, Message> {
    button(label)
        .style(button::secondary)
        .on_press(Message::Clicked(label.to_string()))
        .padding(5)
        .into()
}

fn i_like(label: &str) -> Element<'_, Message> {
    match label {
        "" => text("Click on a fruit to select it.")
            .size(10)
            .align_x(Center)
            .into(),
        _ => text(format!("I like {}", pluralize_fruit(label)))
            .size(10)
            .align_x(Center)
            .into(),
    }
}

fn pluralize_fruit(label: &str) -> String {
    match label {
        "Apple" => "Apples",
        "Banana" => "Bananas",
        "Cherry" => "Cherries",
        "Date" => "Dates",
        "Elderberry" => "Elderberries",
        _ => unreachable!(),
    }
    .to_lowercase()
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Column => write!(f, "Column"),
            Mode::Row => write!(f, "Row"),
        }
    }
}
