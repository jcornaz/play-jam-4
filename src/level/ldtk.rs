use alloc::vec::Vec;

use anyhow::anyhow;
use serde::Deserialize;

use grid::Grid;
use math2d::Vector;

use crate::TILE_SIZE;

use super::Cell;

const RAW_DATA: &[&str] = &[include_str!(
    "../../assets/levels/simplified/level_0/data.json"
)];

const RAW_INT_GRIDS: &[&str] = &[include_str!(
    "../../assets/levels/simplified/level_0/foreground.csv"
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

impl From<Lift> for (Vector, Vector, f32) {
    fn from(value: Lift) -> Self {
        let base = value.position / TILE_SIZE;
        let key = value.custom_fields.key.into();
        let height = base.y - value.custom_fields.arrival.cy - 1.;
        (base, key, height)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LiftCustomFields {
    pub arrival: Point,
    pub key: Point,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Point {
    pub cx: f32,
    pub cy: f32,
}

impl From<Point> for Vector {
    fn from(Point { cx, cy }: Point) -> Self {
        Vector::new(cx, cy)
    }
}

pub fn load_grid(level_num: usize, width: usize, height: usize) -> anyhow::Result<Grid<Cell>> {
    let raw = &RAW_INT_GRIDS
        .get(level_num)
        .ok_or_else(|| anyhow!("failed to load environment int-grid for level {level_num}"))?;
    let cells = raw
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|v| match v {
            "1" => Cell::Terrain,
            "2" => Cell::Hazard,
            _ => Cell::Empty,
        });
    Ok(Grid::from_iter(width, height, cells))
}
