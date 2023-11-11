use anyhow::anyhow;
use crankit_graphics::image::Image;

use crate::Vec2;

pub struct Images {
    idle: Image,
    _running: [Image; 4],
    _falling: Image,
    _dying: Image,
}

impl Images {
    pub fn load() -> anyhow::Result<Self> {
        let sheet = &Image::load("img/player-sheet")
            .map_err(|err| anyhow!("cannot load player images: {err}"))?;
        let mut images = sheet.split_columns(7);
        let idle = images.next().unwrap();
        let _running = [
            images.next().unwrap(),
            images.next().unwrap(),
            images.next().unwrap(),
            images.next().unwrap(),
        ];
        let _falling = images.next().unwrap();
        let _dying = images.next().unwrap();
        Ok(Self {
            idle,
            _running,
            _falling,
            _dying,
        })
    }
}

pub struct Player {
    position: Vec2,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }

    pub fn draw(&self, images: &Images) {
        images.idle.draw_from_center(self.position.as_vector_i32());
    }
}
