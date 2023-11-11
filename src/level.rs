use alloc::format;
use crankit_graphics::{image::Image, LoadError};

pub struct Level {
    image: Image,
}

impl Level {
    pub fn load(num: u8) -> Result<Self, LoadError> {
        let image = Image::load(&format!("img/levels/level_{num}"))?;
        Ok(Self { image })
    }

    pub fn draw(&self) {
        self.image.draw([0, 0]);
    }
}
