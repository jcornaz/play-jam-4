use alloc::format;
use alloc::vec::Vec;

use anyhow::anyhow;

use crankit_graphics::image::Image;
use crankit_graphics::LoadError;
use grid::Grid;

use crate::{Vector, TILE_SIZE};

pub struct Level {}

impl From<LevelDef> for Level {
    fn from(value: LevelDef) -> Self {
        todo!()
    }
}

pub struct LevelDef {
    pub background: [Image; 2],
    pub foreground: [Image; 2],
    pub player_start: Vector,
    pub grid: Grid<Cell>,
    pub lifts: Vec<(Vector, f32)>,
}

impl LevelDef {
    pub fn load(num: usize) -> anyhow::Result<Self> {
        let (background, foreground) =
            Self::load_images(num).map_err(|err| anyhow!("failed to load level images: {err}"))?;
        let data = ldtk::Data::load(num)?;
        let player_start = data.entities.player[0] / TILE_SIZE;
        let grid = ldtk::load_grid(num, data.width / 16, data.height / 16)?;
        let lifts = data.entities.lifts.into_iter().map(Into::into).collect();
        Ok(Self {
            background,
            foreground,
            player_start,
            grid,
            lifts,
        })
    }

    fn load_images(num: usize) -> Result<([Image; 2], [Image; 2]), LoadError> {
        let background = [
            Image::load(&format!("img/levels/level_{num}/background"))?,
            Image::load(&format!("img/levels/level_{num}/background_deco"))?,
        ];
        let foreground = [
            Image::load(&format!("img/levels/level_{num}/foreground"))?,
            Image::load(&format!("img/levels/level_{num}/foreground_deco"))?,
        ];
        Ok((background, foreground))
    }
}

#[derive(Debug, Clone, Default)]
pub enum Cell {
    #[default]
    Empty,
    Terrain,
    Hazard,
}

mod ldtk;
