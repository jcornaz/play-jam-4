#![no_std]

pub mod ffi {
    pub use playdate_sys::{
        ffi::{PDSystemEvent as SystemEvent, PlaydateAPI},
        ll_symbols, EventLoopCtrl,
    };
}

pub trait Game {
    fn new(playdate: &ffi::PlaydateAPI) -> Self;
    fn update(&mut self, playdate: &ffi::PlaydateAPI);
}

#[macro_export]
macro_rules! game_loop {
    ($game_type:tt) => {
        mod __playdate_game {
            static mut PLAYDATE: Option<&'static $crate::ffi::PlaydateAPI> = None;
            static mut GAME: Option<super::$game_type> = None;

            #[no_mangle]
            fn event_handler(
                api: core::ptr::NonNull<$crate::ffi::PlaydateAPI>,
                event: $crate::ffi::SystemEvent,
                _: u32,
            ) -> $crate::ffi::EventLoopCtrl {
                if event == $crate::ffi::SystemEvent::kEventInit {
                    unsafe {
                        let api = api.as_ref();
                        PLAYDATE = Some(api);
                        GAME = Some($crate::Game::new(api));
                        (*api.system).setUpdateCallback.unwrap()(
                            Some(update),
                            core::ptr::null_mut(),
                        );
                    }
                }
                $crate::ffi::EventLoopCtrl::Continue
            }

            extern "C" fn update(_user_data: *mut core::ffi::c_void) -> i32 {
                unsafe {
                    let playdate = PLAYDATE.as_ref().unwrap();
                    $crate::Game::update(GAME.as_mut().unwrap(), playdate);
                };
                1
            }

            $crate::ffi::ll_symbols!();
        }
    };
}
