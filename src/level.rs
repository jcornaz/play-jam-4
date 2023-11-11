use alloc::format;
use anyhow::anyhow;
use crankit_graphics::image::Image;
use grid::Grid;

use crate::Vec2;

pub struct Level {
    pub walls_image: Image,
    pub player_start: Vec2,
    pub grid: Grid<Cell>,
}

impl Level {
    pub fn load(num: usize) -> anyhow::Result<Self> {
        let walls_image = Image::load(&format!("img/levels/level_{num}"))
            .map_err(|err| anyhow!("Cannot load level image {num}: {err}"))?;
        let data = ldtk::Data::load(num)?;
        let player_start = data.entities.player[0];
        let grid = ldtk::load_grid(num, data.width, data.height)?;
        Ok(Self {
            walls_image,
            player_start,
            grid,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub enum Cell {
    #[default]
    Empty,
    Terrain,
}

mod ldtk {
    use grid::Grid;

    use crate::Vec2;
    use anyhow::anyhow;
    use serde::Deserialize;

    use super::Cell;

    const RAW_DATA: &[&str] = &[include_str!(
        "../assets/levels/simplified/level_0/data.json"
    )];

    const RAW_INT_GRIDS: &[&str] = &[include_str!(
        "../assets/levels/simplified/level_0/environment.csv"
    )];

    #[derive(Debug, Clone, Deserialize)]
    pub struct Data {
        pub width: usize,
        pub height: usize,
        pub entities: Entities,
    }

    impl Data {
        pub fn load(level_num: usize) -> Result<Self, anyhow::Error> {
            let raw = RAW_DATA
                .get(level_num)
                .ok_or_else(|| anyhow!("No data for level {level_num}"))?;
            let data: Self = serde_json::from_str(raw)
                .map_err(|err| anyhow!("failed to deserialize level data: {err}"))?;
            Ok(data)
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Entities {
        pub player: [Vec2; 1],
    }

    pub fn load_grid(level_num: usize, width: usize, height: usize) -> anyhow::Result<Grid<Cell>> {
        let raw = &RAW_INT_GRIDS
            .get(level_num)
            .ok_or_else(|| anyhow!("failed to load environment int-grid for level {level_num}"))?;
        let cells = raw
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|v| if v == "0" { Cell::Empty } else { Cell::Terrain });
        Ok(Grid::from_iter(width, height, cells))
    }
}
