use iced::Length::Fill;
use iced::widget::{checkbox, column, container, pick_list, row, space, text};
use iced::{Center, Element, Task, Theme};

use dragking::DragEvent;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("iced — Draggable Widgets")
        .window(iced::window::Settings {
            size: iced::Size::new(400.0, 400.0),
            ..Default::default()
        })
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {
    elements: Vec<String>,
    mode: Mode,
    allow_dragging: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
enum Mode {
    Row,
    #[default]
    Column,
}

#[derive(Debug, Clone)]
enum Message {
    Reorder(DragEvent),
    SwitchMode(Mode),
    ToggleDragging(bool),
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
                    } => {
                        // Update self.elements based on index and target_index
                        let item = self.elements.remove(index);
                        self.elements.insert(target_index, item);
                    }
                    DragEvent::Canceled { .. } => {
                        // Optionally handle cancel event
                    }
                }
            }
            Message::ToggleDragging(boolean) => {
                self.allow_dragging = boolean;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
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
                .style(|_| dragking::row::Style {
                    scale: 1.5,
                    moved_item_overlay: iced::Color::BLACK.scale_alpha(0.75),
                    ghost_background: iced::color![170, 0, 0]
                        .scale_alpha(0.25)
                        .into(),
                    ghost_border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: 5.0.into(),
                    },
                })
                .align_y(Center)
                .on_drag_maybe(self.allow_dragging.then_some(Message::Reorder))
                // Alternatively use `on_drag` to always receive drag events
                // .on_drag(Message::Reorder).
                .into(),
        };

        let toggle = checkbox("Enable dragging", self.allow_dragging)
            .text_line_height(1.0)
            .on_toggle(Message::ToggleDragging);

        container(
            column![
                row![
                    toggle,
                    space::horizontal(),
                    text("Drag items around!"),
                    space::horizontal(),
                    pick_list(
                        [Mode::Row, Mode::Column],
                        Some(&self.mode),
                        Message::SwitchMode,
                    )
                ]
                .align_y(Center),
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
