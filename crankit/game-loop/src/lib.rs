#![no_std]

pub mod ffi {
    pub use playdate_sys::{
        ffi::{PDSystemEvent as SystemEvent, PlaydateAPI},
        ll_symbols, EventLoopCtrl,
    };
}

pub trait Game {
    fn new() -> Self;

    fn update(&mut self);
}

#[macro_export]
macro_rules! game_loop {
    ($game_type:tt) => {
        mod __playdate_game {
            use super::$game_type;
            use core::ptr::NonNull;
            use $crate::ffi::{EventLoopCtrl, PlaydateAPI, SystemEvent};
            static mut GAME: Option<$game_type> = None;

            #[no_mangle]
            fn event_handler(
                api: NonNull<PlaydateAPI>,
                event: SystemEvent,
                _: u32,
            ) -> EventLoopCtrl {
                if unsafe { GAME.is_none() } {
                    let state: $game_type = $crate::Game::new();
                    unsafe { GAME = Some(state) }
                }
                if event == $crate::ffi::SystemEvent::kEventInit {
                    unsafe {
                        (*api.as_ref().system).setUpdateCallback.unwrap()(
                            Some(update),
                            core::ptr::null_mut(),
                        );
                    }
                }
                EventLoopCtrl::Continue
            }

            extern "C" fn update(_user_data: *mut core::ffi::c_void) -> i32 {
                unsafe {
                    $crate::Game::update(GAME.as_mut().unwrap());
                };
                1
            }

            $crate::ffi::ll_symbols!();
        }
    };
}
