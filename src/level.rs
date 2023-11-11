use alloc::format;
use anyhow::anyhow;
use crankit_graphics::image::Image;

use crate::Vec2;

pub struct Level {
    pub walls_image: Image,
    pub player_start: Vec2,
}

impl Level {
    const RAW_DATA: &'static [&'static str] = &[include_str!(
        "../assets/levels/simplified/level_0/data.json"
    )];

    pub fn load(num: usize) -> anyhow::Result<Self> {
        let walls_image = Image::load(&format!("img/levels/level_{num}"))
            .map_err(|err| anyhow!("Cannot load level {num}: {err}"))?;
        let data: ldtk::Data = serde_json::from_str(
            Self::RAW_DATA
                .get(num)
                .ok_or_else(|| anyhow!("No data for level {num}"))?,
        )
        .map_err(|err| anyhow!("failed to deserialize level data: {err}"))?;
        let player_start = data.entities.player[0];
        Ok(Self {
            walls_image,
            player_start,
        })
    }
}

mod ldtk {
    use crate::Vec2;
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize)]
    pub struct Data {
        pub entities: Entities,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Entities {
        pub player: [Vec2; 1],
    }
}
