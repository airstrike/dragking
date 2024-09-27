use iced::widget::{column, container, pick_list, row, text};
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
                .on_drag(Message::Reorder)
                .into(),
            Mode::Row => dragking::row(items.collect::<Vec<_>>())
                .spacing(5)
                .on_drag(Message::Reorder)
                .into(),
        };

        container(
            column![
                row![
                    text("Drag items around!").width(Fill),
                    pick_list(
                        [Mode::Row, Mode::Column],
                        Some(&self.mode),
                        Message::SwitchMode,
                    )
                ],
                container(drag).padding(20).width(Fill).height(Fill).style(
                    |_| {
                        container::Style {
                            border: iced::Border {
                                color: iced::Color::BLACK.scale_alpha(0.2),
                                width: 1.0,
                                radius: 5.0.into(),
                            },
                            ..Default::default()
                        }
                    }
                )
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
    container(text(label))
        .style(container::rounded_box)
        .padding(5)
        .into()
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Column => write!(f, "Column"),
            Mode::Row => write!(f, "Row"),
        }
    }
}
