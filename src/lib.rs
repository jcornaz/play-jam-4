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

type Vec2 = math2d::Vector<f32>;

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
