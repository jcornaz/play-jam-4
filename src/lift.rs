use collision::Aabb;
use crankit_graphics::image::Image;

use crate::{IVector, Vector, TILE_SIZE};

#[derive(Debug)]
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
    pub fn new(base: Vector, height: f32) -> Self {
        Self {
            base,
            height,
            current: 0.0,
        }
    }

    pub fn _lift(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.height);
    }

    pub fn _collision_box(&self) -> Aabb {
        let pos = self.position();
        Aabb::from_min_max(
            pos + COLLISION_BOX_TOP_LEFT,
            pos + COLLISION_BOX_BOTTOM_RIGHT,
        )
    }

    pub fn draw(&self, image: &Image) {
        let pos = (self.position() * TILE_SIZE).as_vector_i32() + IMAGE_TOP_LEFT;
        image.draw(pos);
    }

    fn position(&self) -> Vector {
        let mut pos = self.base;
        pos.y -= self.current;
        pos
    }
}
