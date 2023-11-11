#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod animation;
mod level;
mod player;

use core::ptr::NonNull;

use crankit_graphics::{image::Image, Color};
use crankit_input::button_state;
use crankit_time::reset_elapsed_time;
use grid::Grid;
use level::{Cell, Level};
use playdate_sys::{
    ffi::{PDSystemEvent as SystemEvent, PlaydateAPI},
    ll_symbols, EventLoopCtrl,
};
use player::Player;

type Vector = math2d::Vector<f32>;
type Point = math2d::Point<f32>;
type CellCoord = [usize; 2];

struct Game {
    level_image: Image,
    player_images: player::Images,
    player: Player,
    grid: Grid<Cell>,
}

impl Game {
    fn new() -> Self {
        let level = Level::load(0).unwrap();
        let player_images = player::Images::load().unwrap();
        let player = Player::new(level.player_start);
        Self {
            level_image: level.walls_image,
            player,
            player_images,
            grid: level.grid,
        }
    }

    fn update_and_draw(&mut self) {
        let delta_time = reset_elapsed_time();
        let buttons = button_state();
        crankit_graphics::clear(Color::black());
        self.level_image.draw([0, 0]);
        self.player.update(delta_time, buttons, &self.grid);
        self.player.draw(&self.player_images);
    }
}

const TILE_SIZE: f32 = 16.0;

fn world_to_coord(point: Point) -> CellCoord {
    let v = Vector::from(point) / TILE_SIZE;
    [v.x as usize, v.y as usize]
}

fn coord_to_world([x, y]: CellCoord) -> Point {
    (Vector::new(x as f32, y as f32) * TILE_SIZE).into()
}

static mut GAME: Option<Game> = None;

#[no_mangle]
fn event_handler(api: NonNull<PlaydateAPI>, event: SystemEvent, _: u32) -> EventLoopCtrl {
    if unsafe { GAME.is_none() } {
        let state = Game::new();
        unsafe { GAME = Some(state) }
    }
    if event == SystemEvent::kEventInit {
        unsafe {
            (*api.as_ref().system).setUpdateCallback.unwrap()(Some(update), core::ptr::null_mut());
        }
    }
    EventLoopCtrl::Continue
}

extern "C" fn update(_user_data: *mut core::ffi::c_void) -> i32 {
    unsafe {
        GAME.as_mut().unwrap().update_and_draw();
    };
    1
}

ll_symbols!();
