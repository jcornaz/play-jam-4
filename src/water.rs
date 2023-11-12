use core::time::Duration;

use anyhow::anyhow;

use crankit_graphics::image::Image;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

pub struct Images {
    width: usize,
    height: usize,
    body: Image,
    surface: Image,
}

impl Images {
    pub fn load() -> anyhow::Result<Self> {
        let body = Image::load("img/water/body")
            .map_err(|err| anyhow!("cannot load water body image: {err}"))?;
        let surface = Image::load("img/water/surface")
            .map_err(|err| anyhow!("cannot load water surface image: {err}"))?;
        let [w, h] = body.size();
        Ok(Self {
            width: w as usize,
            height: h as usize,
            body,
            surface,
        })
    }
}

pub struct Water {
    level: f32,
}

const RAISE_SPEED: f32 = 0.3;

/// Offset of the image relative to the level
const IMAGE_OFFSET: i32 = 7;
impl Water {
    pub fn new() -> Self {
        Self { level: 0.5 }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.level += RAISE_SPEED * delta_time.as_secs_f32();
    }

    pub fn draw(&self, images: &Images) {
        let y = SCREEN_HEIGHT - (self.level * TILE_SIZE) as i32 - IMAGE_OFFSET;
        for x in (0..SCREEN_WIDTH).step_by(images.width) {
            images.surface.draw([x, y]);
            for y in ((y + images.height as i32)..SCREEN_HEIGHT).step_by(images.height) {
                images.body.draw([x, y]);
            }
        }
    }
}
