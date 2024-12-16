use agb::{
    display::object::Object,
    fixnum::{FixedNum, Vector2D},
};

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

#[derive(Copy, Clone)]
pub struct Player<'a> {
    direction: Direction,
    position: Vector2D<FixedNum<8>>,

    object_sprite: &'a Object<'a>,
}
