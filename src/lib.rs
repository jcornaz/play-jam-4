#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod level;

use core::ptr::NonNull;

use crankit_input::button_state;
use crankit_time::reset_elapsed_time;
use level::Level;
use playdate_sys::{
    ffi::{PDSystemEvent as SystemEvent, PlaydateAPI},
    ll_symbols, EventLoopCtrl,
};

type Vec2 = math2d::Vector;

struct Game {
    level: Level,
}

impl Game {
    fn new() -> Self {
        Self {
            level: Level::load(0).unwrap(),
        }
    }

    fn update_and_draw(&mut self) {
        let _delta_time = reset_elapsed_time();
        let _buttons = button_state();
        self.level.walls_image.draw([0, 0]);
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
