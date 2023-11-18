#![no_std]

mod interop {
    #[cfg(feature = "playdate-sys-v02")]
    mod playdate_sys_v02;
}

use core::time::Duration;

/// System capable of tracking elapsed time since last reset
pub trait ElapsedTime {
    /// Returns the duration since last [`reset_elapsed_time`] was called.
    fn elapsed_time(&self) -> Duration;

    /// Resets the timer and return the elapsed time since last reset.
    fn reset_elapsed_time(&self) -> Duration;
}

/// System capable of returning the absolute current time
pub trait AbsoluteTime {
    /// Returns the time elapsed since midnight (hour 0), January 1, 2000.
    fn elapsed_since_epoch(&self) -> Duration;
}
