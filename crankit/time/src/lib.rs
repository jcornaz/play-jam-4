#![no_std]

use core::{ptr, time::Duration};

use playdate_sys::api;

unsafe fn system() -> &'static playdate_sys::ffi::playdate_sys {
    api()
        .expect("playdate API not initialized")
        .system
        .as_ref()
        .expect("cannot find playdate system")
}

/// Resets the high-resolution timer and return the elapsed time since last reset.
pub fn reset_elapsed_time() -> Duration {
    let elapsed = elapsed_time();
    unsafe {
        system().resetElapsedTime.unwrap()();
    }
    elapsed
}

/// Returns the duration since last [`reset_elapsed_time`] was called.
pub fn elapsed_time() -> Duration {
    let seconds = unsafe { system().getElapsedTime.unwrap()() };
    Duration::from_secs_f32(seconds)
}

/// Returns the number of milliseconds elapsed since midnight (hour 0), January 1, 2000.
///
/// Useful to seed a random generator
pub fn milliseconds_since_epoch() -> u64 {
    let mut milliseconds = 0;
    let seconds =
        unsafe { system().getSecondsSinceEpoch.unwrap()(ptr::addr_of_mut!(milliseconds)) } as u64;
    seconds * 1000 + (milliseconds as u64)
}
