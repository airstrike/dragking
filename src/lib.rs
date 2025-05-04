use iced::Point;

pub use self::column::column;
pub use self::row::row;
pub mod column;
pub mod row;

use iced::animation::Animation;
use iced::time::Instant;

#[derive(Debug, Clone)]
pub(crate) enum Action {
    Idle {
        now: Option<Instant>,
        animations: ItemAnimations,
    },
    Picking {
        index: usize,
        origin: Point,
        now: Instant,
        animations: ItemAnimations,
    },
    Dragging {
        index: usize,
        origin: Point,
        last_cursor: Point,
        now: Instant,
        animations: ItemAnimations,
    },
}

impl Default for Action {
    fn default() -> Self {
        Self::Idle {
            now: None,
            animations: ItemAnimations::default(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct ItemAnimations {
    /// Offset animations for each item
    pub offsets: Vec<Animation<f32>>,
}

impl ItemAnimations {
    pub fn zero(&mut self) {
        for animation in &mut self.offsets {
            *animation = Animation::new(0.0);
        }
    }

    pub fn is_animating(&self, now: Instant) -> bool {
        self.offsets.iter().any(|anim| anim.is_animating(now))
    }

    pub fn with_capacity(&mut self, count: usize) {
        if self.offsets.len() < count {
            self.offsets.resize_with(count, || Animation::new(0.0));
        }
    }
}

#[derive(Debug, Clone)]
pub enum DragEvent {
    Picked { index: usize },
    Dropped { index: usize, target_index: usize },
    Canceled { index: usize },
}
