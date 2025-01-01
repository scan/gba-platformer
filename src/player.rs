use core::cell::RefCell;
use agb::display::object::Object;
use alloc::rc::Rc;
use crate::num::{FixedNum, Vector2D, Rect, num};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    RIGHT,
    LEFT,
}

impl Into<bool> for Direction {
    fn into(self) -> bool {
        match self {
            Direction::RIGHT => false,
            Direction::LEFT => true,
        }
    }
}

#[derive(Clone)]
pub struct Player<'a> {
    direction: Direction,
    position: Vector2D<FixedNum>,

    object_sprite: Rc<RefCell<Object<'a>>>,
}

impl<'a> Player<'a> {
    pub fn new(object_sprite: Object<'a>) -> Self {
        Self {
            direction: Direction::RIGHT,
            position: Vector2D::default(),
            object_sprite: Rc::new(RefCell::new(object_sprite)),
        }
    }

    pub fn translate(&mut self, diff: FixedNum) {
        self.direction = if diff >= num!(0.0) {
            Direction::RIGHT
        } else {
            Direction::LEFT
        };
        self.position.x += diff;
    }

    pub fn update(&mut self) {
        let mut obj = self.object_sprite.borrow_mut();

        obj.set_position((self.position.x.floor(), self.position.y.floor()));
        obj.set_hflip(self.direction.into());
        obj.show();
    }

    pub fn hitbox(&self) -> Rect {
        Rect {
            position: self.position,
            size: Vector2D {
                x: num!(8.0),
                y: num!(8.0),
            },
        }
    }
}

impl<'a> Drop for Player<'a> {
    fn drop(&mut self) {
        self.object_sprite.borrow_mut().hide();
    }
}
