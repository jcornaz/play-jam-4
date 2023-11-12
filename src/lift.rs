#![allow(unused)]

use collision::Aabb;
use crankit_graphics::image::Image;

use crate::{IVector, Vector};

pub struct Lift {
    base: Vector,
    height: f32,
    current: f32,
}

/// Position of the top left-corner of the image relative to the lift position
const IMAGE_TOP_LEFT: IVector = IVector::new(-24, -240);

/// Top-left of the collision box relative to the lift position
const COLLISION_BOX_TOP_LEFT: Vector = Vector::new(-0.5, -16. / 240.);

/// Bottom-right of the collision box relative to the lift position
const COLLISION_BOX_BOTTOM_RIGHT: Vector = Vector::new(0.5, 0.);

impl Lift {
    pub fn new(position: Vector, height: f32) -> Self {
        Self {
            base: position,
            height,
            current: 0.0,
        }
    }

    pub fn lift(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.height);
    }

    pub fn collision_box(&self) -> Aabb {
        let pos = self.position();
        Aabb::from_min_max(
            pos + COLLISION_BOX_TOP_LEFT,
            pos + COLLISION_BOX_BOTTOM_RIGHT,
        )
    }

    pub fn draw(&self, image: &Image) {
        image.draw(self.position().as_vector_i32() + IMAGE_TOP_LEFT);
    }

    fn position(&self) -> Vector {
        let mut pos = self.base;
        pos.y -= self.current;
        pos
    }
}
