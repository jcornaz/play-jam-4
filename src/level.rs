use alloc::format;
use alloc::vec::Vec;

use anyhow::anyhow;

use crankit_graphics::image::Image;
use grid::Grid;

use crate::{Vector, TILE_SIZE};

pub struct Level {
    pub walls_image: Image,
    pub player_start: Vector,
    pub grid: Grid<Cell>,
    pub lifts: Vec<(Vector, f32)>,
}

impl Level {
    pub fn load(num: usize) -> anyhow::Result<Self> {
        let walls_image = Image::load(&format!("img/levels/level_{num}"))
            .map_err(|err| anyhow!("Cannot load level image {num}: {err}"))?;
        let data = ldtk::Data::load(num)?;
        let player_start = data.entities.player[0] / TILE_SIZE;
        let grid = ldtk::load_grid(num, data.width / 16, data.height / 16)?;
        let lifts = data.entities.lifts.into_iter().map(Into::into).collect();
        Ok(Self {
            walls_image,
            player_start,
            grid,
            lifts,
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
    use alloc::vec::Vec;

    use anyhow::anyhow;
    use serde::Deserialize;

    use grid::Grid;
    use math2d::Vector;

    use crate::TILE_SIZE;

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
        pub player: [Vector; 1],
        #[serde(rename = "lift")]
        pub lifts: Vec<Lift>,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Lift {
        #[serde(flatten)]
        pub position: Vector,
        pub custom_fields: LiftCustomFields,
    }

    impl From<Lift> for (Vector, f32) {
        fn from(value: Lift) -> Self {
            let base = value.position / TILE_SIZE;
            let height = base.y - value.custom_fields.arrival.cy - 1.;
            (base, height)
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct LiftCustomFields {
        pub arrival: ArrivalPoint,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct ArrivalPoint {
        pub cy: f32,
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
