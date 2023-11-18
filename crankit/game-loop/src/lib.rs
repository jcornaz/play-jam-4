#![no_std]

use crankit_input::InputSystem;

pub mod ffi {
    pub use playdate_sys::{
        ffi::{PDSystemEvent as SystemEvent, PlaydateAPI},
        ll_symbols, EventLoopCtrl,
    };
}

#[non_exhaustive]
pub struct Playdate<'a> {
    pub c_api: &'a ffi::PlaydateAPI,
    pub input: InputSystem<'a>,
}

impl<'a> Playdate<'a> {
    /// Create a new instance from a reference to the playdate system API
    ///
    /// # Safety
    ///
    /// * The referenced api must be a valid and initialized playdate api that's safe to use for the lifetime `'a`
    ///
    pub unsafe fn from_c_api(c_api: &'a ffi::PlaydateAPI) -> Self {
        Self {
            c_api,
            input: InputSystem::from_c_api(c_api.system.as_ref().unwrap()),
        }
    }
}

pub trait Game {
    fn new(playdate: &Playdate) -> Self;
    fn update(&mut self, playdate: &Playdate);
}

#[macro_export]
macro_rules! game_loop {
    ($game_type:tt) => {
        mod __playdate_game {
            static mut PLAYDATE: Option<$crate::Playdate<'static>> = None;
            static mut GAME: Option<super::$game_type> = None;

            #[no_mangle]
            fn event_handler(
                api: core::ptr::NonNull<$crate::ffi::PlaydateAPI>,
                event: $crate::ffi::SystemEvent,
                _: u32,
            ) -> $crate::ffi::EventLoopCtrl {
                if event == $crate::ffi::SystemEvent::kEventInit {
                    unsafe {
                        let playdate: $crate::Playdate<'static> =
                            $crate::Playdate::from_c_api(api.as_ref());
                        GAME = Some($crate::Game::new(&playdate));
                        (*playdate.c_api.system).setUpdateCallback.unwrap()(
                            Some(update),
                            core::ptr::null_mut(),
                        );
                        PLAYDATE = Some(playdate);
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
