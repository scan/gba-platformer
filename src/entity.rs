use agb::{
    display::{object::Object, HEIGHT, WIDTH},
    fixnum,
};

use crate::{
    level::Level,
    num::{FixedNum, Rect, Vector2D},
};

pub struct Entity<'a> {
    sprite: Object<'a>,
    position: Vector2D,
    velocity: Vector2D,
    size: (u16, u16),
    visible: bool,
}

impl<'a> Entity<'a> {
    pub fn new(sprite: Object<'a>, position: Vector2D, size: (u16, u16)) -> Self {
        Self {
            sprite,
            position,
            velocity: Vector2D::default(),
            visible: false,
            size,
        }
    }

    pub fn collider(&self) -> Rect {
        Rect::new(
            self.position,
            (self.size.0 as i32, self.size.1 as i32).into(),
        )
    }

    pub fn set_position(&mut self, position: impl Into<Vector2D>) {
        self.position = position.into();
    }

    pub fn set_velocity(&mut self, velocity: impl Into<Vector2D>) {
        self.velocity = velocity.into();
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn update_position(&mut self, level: &Level) -> Vector2D {
        let initial_position = self.position;

        let y = self.velocity.y.to_raw().signum();
        if y != 0 {
            let (delta, collided) =
                self.collision_in_direction((0, y).into(), self.velocity.y.abs(), |v| {
                    level.collides(v)
                });
            self.position += delta;
            if collided {
                self.velocity.y = 0.into();
            }
        }
        let x = self.velocity.x.to_raw().signum();
        if x != 0 {
            let (delta, collided) =
                self.collision_in_direction((x, 0).into(), self.velocity.x.abs(), |v| {
                    level.collides(v)
                });
            self.position += delta;
            if collided {
                self.velocity.x = 0.into();
            }
        }

        self.position - initial_position
    }

    pub fn commit(&mut self, offset: Vector2D) {
        if !self.visible {
            self.sprite.hide();
        } else {
            let position = (self.position - offset).floor();
            let size: fixnum::Vector2D<i32> = self.size.into();
            self.sprite.set_position(position - size / 2);
            if position.x < -8
                || position.x > WIDTH + 8
                || position.y < -8
                || position.y > HEIGHT + 8
            {
                self.sprite.hide();
            } else {
                self.sprite.show();
            }
        }
    }

    fn collision_in_direction(
        &self,
        direction: Vector2D,
        distance: FixedNum,
        collision: impl Fn(Vector2D) -> Option<Rect>,
    ) -> (Vector2D, bool) {
        let mut current_position = self.position;
        let mut remaining_distance = distance;
        let mut collided = false;

        while remaining_distance > 0.into() {
            let step = if remaining_distance > 1.into() {
                1.into()
            } else {
                remaining_distance
            };

            let next_position = current_position + direction * step;
            let next_hitbox = Rect::new(
                next_position,
                (self.size.0 as i32, self.size.1 as i32).into(),
            );

            if let Some(obstacle) = collision(next_position) {
                if next_hitbox.touches(obstacle) {
                    collided = true;
                    break;
                }
            }

            current_position = next_position;
            remaining_distance -= step;
        }

        ((current_position - self.position), collided)
    }
}
