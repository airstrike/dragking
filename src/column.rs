//! Distribute draggable content vertically.
// This widget is a modification of the original `Column` widget from [`iced`]
//
// [`iced`]: https://github.com/iced-rs/iced
//
// Copyright 2019 Héctor Ramón, Iced contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{tree, Operation, Tree, Widget};
use iced::advanced::{overlay, renderer, Clipboard, Shell};
use iced::alignment::{self, Alignment};
use iced::event::{self, Event};
use iced::{mouse, Transformation};
use iced::{
    Background, Border, Color, Element, Length, Padding, Pixels, Point,
    Rectangle, Size, Theme, Vector,
};

use crate::{Action, DragEvent, DropPosition};

pub fn column<'a, Message, Theme, Renderer>(
    children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Column<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    Column::with_children(children)
}

const DRAG_DEADBAND_DISTANCE: f32 = 5.0;

/// A container that distributes its contents vertically.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::{button, column};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     column![
///         "I am on top!",
///         button("I am in the center!"),
///         "I am below.",
///     ].into()
/// }
/// ```
#[allow(missing_debug_implementations)]
pub struct Column<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
{
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    align: Alignment,
    clip: bool,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    on_drag: Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Column<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// Creates an empty [`Column`].
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Creates a [`Column`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`Column`] with the given elements.
    pub fn with_children(
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Creates a [`Column`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`Column`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Column::width`] or [`Column::height`] accordingly.
    pub fn from_vec(
        children: Vec<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::INFINITY,
            align: Alignment::Start,
            clip: false,
            children,
            class: Theme::default(),
            on_drag: None,
        }
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = amount.into().0;
        self
    }

    /// Sets the [`Padding`] of the [`Column`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Column`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Column`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`Column`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`] .
    pub fn align_x(mut self, align: impl Into<alignment::Horizontal>) -> Self {
        self.align = Alignment::from(align.into());
        self
    }

    /// Sets whether the contents of the [`Column`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Adds an element to the [`Column`].
    pub fn push(
        mut self,
        child: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let child = child.into();
        let child_size = child.as_widget().size_hint();

        self.width = self.width.enclose(child_size.width);
        self.height = self.height.enclose(child_size.height);

        self.children.push(child);
        self
    }

    /// Adds an element to the [`Column`], if `Some`.
    pub fn push_maybe(
        self,
        child: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
    ) -> Self {
        if let Some(child) = child {
            self.push(child)
        } else {
            self
        }
    }

    /// Extends the [`Column`] with the given children.
    pub fn extend(
        self,
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// The message produced by the [`Column`] when a child is dragged.
    pub fn on_drag(
        mut self,
        on_reorder: impl Fn(DragEvent) -> Message + 'a,
    ) -> Self {
        self.on_drag = Some(Box::new(on_reorder));
        self
    }

    // Computes the index and position where a dragged item should be dropped.
    fn compute_target_index(
        &self,
        cursor_position: Point,
        layout: Layout<'_>,
        dragged_index: usize,
    ) -> (usize, DropPosition) {
        for (i, child_layout) in layout.children().enumerate() {
            let bounds = child_layout.bounds();

            if bounds.contains(cursor_position) {
                if i == dragged_index {
                    // Cursor is over the dragged item itself
                    return (i, DropPosition::Swap);
                }

                let thickness = bounds.height.min(bounds.width) / 4.0;
                let top_threshold = bounds.y + thickness;
                let bottom_threshold = bounds.y + bounds.height - thickness;

                if cursor_position.y < top_threshold {
                    // Near the top edge - insert before
                    return (i, DropPosition::Before);
                } else if cursor_position.y > bottom_threshold {
                    // Near the bottom edge - insert after
                    return (i + 1, DropPosition::After);
                } else {
                    // Middle area - swap
                    return (i, DropPosition::Swap);
                }
            } else if cursor_position.y < bounds.y {
                // Cursor is before this child
                return (i, DropPosition::Before);
            }
        }

        // Cursor is after all children
        (self.children.len(), DropPosition::After)
    }
}

impl<'a, Message, Renderer> Default for Column<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message, Theme, Renderer: renderer::Renderer>
    FromIterator<Element<'a, Message, Theme, Renderer>>
    for Column<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
{
    fn from_iter<
        T: IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    >(
        iter: T,
    ) -> Self {
        Self::with_children(iter)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Column<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Action>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Action::Idle)
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.max_width(self.max_width);

        layout::flex::resolve(
            layout::flex::Axis::Vertical,
            renderer,
            &limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            self.align,
            &self.children,
            &mut tree.children,
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                });
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut event_status = event::Status::Ignored;

        let action = tree.state.downcast_mut::<Action>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_position) =
                    cursor.position_over(layout.bounds())
                {
                    for (index, child_layout) in layout.children().enumerate() {
                        if child_layout.bounds().contains(cursor_position) {
                            *action = Action::Picking {
                                index,
                                origin: cursor_position,
                            };
                            event_status = event::Status::Captured;
                            break;
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                match *action {
                    Action::Picking { index, origin } => {
                        if let Some(cursor_position) = cursor.position() {
                            if cursor_position.distance(origin)
                                > DRAG_DEADBAND_DISTANCE
                            {
                                // Start dragging
                                *action = Action::Dragging {
                                    index,
                                    origin,
                                    last_cursor: cursor_position,
                                };
                                if let Some(on_reorder) = &self.on_drag {
                                    shell.publish(on_reorder(
                                        DragEvent::Picked { index },
                                    ));
                                }
                                event_status = event::Status::Captured;
                            }
                        }
                    }
                    Action::Dragging { origin, index, .. } => {
                        if let Some(cursor_position) = cursor.position() {
                            *action = Action::Dragging {
                                last_cursor: cursor_position,
                                origin,
                                index,
                            };
                            event_status = event::Status::Captured;
                        }
                    }
                    _ => {}
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                match *action {
                    Action::Dragging { index, .. } => {
                        if let Some(cursor_position) = cursor.position() {
                            let bounds = layout.bounds();
                            if bounds.contains(cursor_position) {
                                let (target_index, drop_position) = self
                                    .compute_target_index(
                                        cursor_position,
                                        layout,
                                        index,
                                    );

                                if let Some(on_reorder) = &self.on_drag {
                                    shell.publish(on_reorder(
                                        DragEvent::Dropped {
                                            index,
                                            target_index,
                                            drop_position,
                                        },
                                    ));
                                    event_status = event::Status::Captured;
                                }
                            } else if let Some(on_reorder) = &self.on_drag {
                                shell.publish(on_reorder(
                                    DragEvent::Canceled { index },
                                ));
                                event_status = event::Status::Captured;
                            }
                        }
                        *action = Action::Idle;
                    }
                    Action::Picking { .. } => {
                        // Did not move enough to start dragging
                        *action = Action::Idle;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let child_status = self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge);

        event::Status::merge(event_status, child_status)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let action = tree.state.downcast_ref::<Action>();

        if let Action::Dragging { .. } = *action {
            return mouse::Interaction::Grabbing;
        }

        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state, layout, cursor, viewport, renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let action = tree.state.downcast_ref::<Action>();
        let style = theme.style(&self.class);

        match action {
            Action::Dragging {
                index,
                last_cursor,
                origin,
                ..
            } => {
                let child_count = self.children.len();

                // Determine the target index based on cursor position
                let target_index = if cursor.position().is_some() {
                    let (target_index, _) =
                        self.compute_target_index(*last_cursor, layout, *index);
                    target_index.min(child_count - 1)
                } else {
                    *index
                };

                // Store the width of the dragged item
                let drag_bounds =
                    layout.children().nth(*index).unwrap().bounds();
                let drag_height = drag_bounds.height + self.spacing;

                // Draw all children except the one being dragged
                let mut translations = 0.0;
                for i in 0..child_count {
                    let child = &self.children[i];
                    let state = &tree.children[i];
                    let child_layout = layout.children().nth(i).unwrap();

                    // Draw the dragged item separately
                    // TODO: Draw a shadow below the picked item to enhance the
                    // floating effect
                    if i == *index {
                        let scaling = Transformation::scale(style.scale);
                        let translation = *last_cursor - *origin * scaling;
                        renderer.with_transformation(scaling, |renderer| {
                            renderer.with_translation(
                                translation,
                                |renderer| {
                                    renderer.with_layer(
                                        child_layout.bounds(),
                                        |renderer| {
                                            child.as_widget().draw(
                                                state,
                                                renderer,
                                                theme,
                                                defaults,
                                                child_layout,
                                                cursor,
                                                viewport,
                                            );
                                        },
                                    );
                                },
                            );
                        });
                    } else {
                        let offset: i32 = match target_index.cmp(index) {
                            std::cmp::Ordering::Less
                                if i >= target_index && i < *index =>
                            {
                                1
                            }
                            std::cmp::Ordering::Greater
                                if i > *index && i <= target_index =>
                            {
                                -1
                            }
                            _ => 0,
                        };

                        let translation =
                            Vector::new(0.0, offset as f32 * drag_height);
                        renderer.with_translation(translation, |renderer| {
                            child.as_widget().draw(
                                state,
                                renderer,
                                theme,
                                defaults,
                                child_layout,
                                cursor,
                                viewport,
                            );
                            // Draw an overlay if this item is being moved
                            // TODO: instead of drawing an overlay, it would be nicer to
                            // draw the item with a reduced opacity, but that's not possible today
                            if offset != 0 {
                                renderer.fill_quad(
                                    renderer::Quad {
                                        bounds: child_layout.bounds(),
                                        ..renderer::Quad::default()
                                    },
                                    style.moved_item_overlay,
                                );

                                // Keep track of the total translation so we can
                                // draw the "ghost" of the dragged item later
                                translations -= (child_layout.bounds().height
                                    + self.spacing)
                                    * offset.signum() as f32;
                            }
                        });
                    }
                }
                // Draw a ghost of the dragged item in its would-be position
                let ghost_translation = Vector::new(0.0, translations);
                renderer.with_translation(ghost_translation, |renderer| {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: drag_bounds,
                            border: style.ghost_border,
                            ..renderer::Quad::default()
                        },
                        style.ghost_background,
                    );
                });
            }
            _ => {
                // Draw all children normally when not dragging
                for ((child, state), layout) in self
                    .children
                    .iter()
                    .zip(&tree.children)
                    .zip(layout.children())
                {
                    child.as_widget().draw(
                        state, renderer, theme, defaults, layout, cursor,
                        viewport,
                    );
                }
            }
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Column<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(column: Column<'a, Message, Theme, Renderer>) -> Self {
        Self::new(column)
    }
}

/// The theme catalog of a [`Column`].
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// The appearance of a [`Column`].
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The scaling to apply to a picked element while it's being dragged.
    pub scale: f32,
    /// The color of the overlay on items that are moved around
    pub moved_item_overlay: Color,
    /// The outline border of the dragged item's ghost
    pub ghost_border: Border,
    /// The background of the dragged item's ghost
    pub ghost_background: Background,
}

/// A styling function for a [`Column`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    Style {
        scale: 1.05,
        moved_item_overlay: theme
            .extended_palette()
            .primary
            .base
            .color
            .scale_alpha(0.2),
        ghost_border: Border {
            width: 1.0,
            color: theme.extended_palette().secondary.base.color,
            radius: 0.0.into(),
        },
        ghost_background: theme
            .extended_palette()
            .secondary
            .base
            .color
            .scale_alpha(0.2)
            .into(),
    }
}
