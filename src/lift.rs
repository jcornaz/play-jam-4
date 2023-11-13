use libm::fabsf;

use collision::Aabb;
use crankit_graphics::image::Image;

use crate::player::Player;
use crate::{IVector, Vector, TILE_SIZE};

#[derive(Debug)]
pub struct Lift {
    base: Vector,
    key: Option<Vector>,
    height: f32,
    current: f32,
}

/// Position of the top left-corner of the image relative to the lift position
const IMAGE_TOP_LEFT: IVector = IVector::new(-24, -240);

/// Top-left of the collision box relative to the lift position
const COLLISION_BOX_TOP_LEFT: Vector = Vector::new(-1.5, -1.);

/// Bottom-right of the collision box relative to the lift position
const COLLISION_BOX_BOTTOM_RIGHT: Vector = Vector::new(1.5, 2.0);

/// Top-left of the interaction box relative to the lift position
const INTERACTION_BOX_TOP_LEFT: Vector = Vector::new(-1.5, -2.);

/// Bottom-right of the interaction box relative to the lift position
const INTERACTION_BOX_BOTTOM_RIGHT: Vector = Vector::new(1.5, -1.);

const SPEED_FACTOR: f32 = 0.01;

impl Lift {
    pub fn new(base: Vector, key: Vector, height: f32) -> Self {
        Self {
            base,
            key: Some(key),
            height,
            current: 0.0,
        }
    }

    pub fn update(&mut self, crank_speed: f32, player: &mut Player) {
        let previous = self.current;
        self.current = (self.current + fabsf(crank_speed) * SPEED_FACTOR).clamp(0.0, self.height);
        player.move_by(Vector::new(0.0, previous - self.current))
    }

    pub fn interaction_box(&self) -> Aabb {
        let pos = self.position();
        Aabb::from_min_max(
            pos + INTERACTION_BOX_TOP_LEFT,
            pos + INTERACTION_BOX_BOTTOM_RIGHT,
        )
    }

    pub fn collision_box(&self) -> Aabb {
        let pos = self.position();
        Aabb::from_min_max(
            pos + COLLISION_BOX_TOP_LEFT,
            pos + COLLISION_BOX_BOTTOM_RIGHT,
        )
    }

    pub fn draw(&self, lift_image: &Image, key_image: &Image) {
        let pos = (self.position() * TILE_SIZE).as_vector_i32() + IMAGE_TOP_LEFT;
        lift_image.draw(pos);
        if let Some(key) = self.key.map(|p| (p * TILE_SIZE).as_vector_i32()) {
            key_image.draw(key);
        }
    }

    fn position(&self) -> Vector {
        let mut pos = self.base;
        pos.y -= self.current;
        pos
    }
}
