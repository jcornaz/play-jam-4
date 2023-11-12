use alloc::format;
use alloc::vec::Vec;
use core::time::Duration;

use anyhow::anyhow;
use playdate_sys::println;

use collision::Aabb;
use crankit_graphics::{image::Image, LoadError};
use crankit_input::ButtonState;
use grid::Grid;

use crate::{lift::Lift, player::Player, water::Water, Images, Vector, TILE_SIZE};

const PENETRATION_RESOLUTION_MAX_ITER: u32 = 10;

pub struct Level {
    definition: Definition,
    player: Player,
    water: Water,
    lifts: Vec<Lift>,
    active_lift: Option<usize>,
}

impl Level {
    pub fn update(&mut self, delta_time: Duration, buttons: ButtonState, crank_change: f32) {
        self.player.handle_input(buttons);
        self.player.update(delta_time);
        let player_collision_box = self.player.collision_box();
        self.resolve_collisions();
        if let Some(lift) = self.active_lift.map(|i| &mut self.lifts[i]) {
            if player_collision_box.collides(lift.interaction_box()) {
                lift.update(crank_change, &mut self.player);
            } else {
                self.active_lift = None;
            }
        } else if let Some(index) = (0..self.lifts.len()).find(|i| {
            self.lifts[*i]
                .interaction_box()
                .collides(player_collision_box)
        }) {
            self.active_lift = Some(index);
        }
        self.water.update(delta_time);
    }

    pub fn draw(&self, images: &Images) {
        self.definition
            .background
            .iter()
            .for_each(|i| i.draw([0, 0]));
        self.player.draw(&images.player_images);
        self.lifts.iter().for_each(|l| l.draw(&images.lift_image));
        self.definition
            .foreground
            .iter()
            .for_each(|i| i.draw([0, 0]));
        self.water.draw(&images.water_images);
    }

    fn resolve_collisions(&mut self) {
        let mut iter = 0;
        while let Some(penetration) = self.collides_against_terrain() {
            iter += 1;
            if iter > PENETRATION_RESOLUTION_MAX_ITER {
                println!(
                    "Exhausted number of iteration for inter-penetration resolution ({})",
                    PENETRATION_RESOLUTION_MAX_ITER
                );
                return;
            }
            self.player.move_by(penetration);
            if penetration.y < 0. {
                self.player.on_floor_hit();
            } else if penetration.y > 0. {
                self.player.on_roof_hit();
            }
        }
    }

    fn collides_against_terrain(&self) -> Option<Vector> {
        let player_collision_box = self.player.collision_box();
        let terrain = coords(player_collision_box)
            .filter(|c| matches!(self.definition.grid.get(*c), Some(Cell::Terrain)))
            .map(|[x, y]| {
                Aabb::from_min_max([x as f32, y as f32], [(x + 1) as f32, (y + 1) as f32])
            });
        let lifts = self.lifts.iter().map(|l| l.collision_box());
        player_collision_box
            .max_penetration(terrain.chain(lifts))
            .map(Into::into)
    }

    fn collides_against_hazard(&self) -> bool {
        if self.player.position().y > self.water.position() {
            return true;
        }
        let player_collision_box = self.player.collision_box();
        let hazards = coords(player_collision_box)
            .filter(|c| matches!(self.definition.grid.get(*c), Some(Cell::Hazard)))
            .map(|[x, y]| {
                Aabb::from_min_max(
                    [x as f32 + 0.1, y as f32 + 0.1],
                    [x as f32 + 0.8, x as f32 + 0.8],
                )
            });
        player_collision_box.collides_any(hazards)
    }
}

impl From<Definition> for Level {
    fn from(definition: Definition) -> Self {
        let player = Player::new(definition.player_start);
        let lifts = definition
            .lifts
            .iter()
            .copied()
            .map(|(base, height)| Lift::new(base, height))
            .collect();
        Self {
            definition,
            player,
            lifts,
            active_lift: None,
            water: Water::new(),
        }
    }
}

fn coords(bounding_box: Aabb) -> impl Iterator<Item = [usize; 2]> {
    let [min_x, max_x] = bounding_box.x.into();
    let [min_y, max_y] = bounding_box.y.into();
    ((libm::floorf(min_x) as usize)..=(libm::ceilf(max_x) as usize)).flat_map(move |x| {
        ((libm::floorf(min_y) as usize)..=(libm::ceilf(max_y) as usize)).map(move |y| [x, y])
    })
}

pub struct Definition {
    pub background: [Image; 2],
    pub foreground: [Image; 2],
    pub player_start: Vector,
    pub grid: Grid<Cell>,
    pub lifts: Vec<(Vector, f32)>,
}

impl Definition {
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
