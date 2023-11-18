use core::{ptr, time::Duration};

use playdate_sys_v02::ffi::{self, PlaydateAPI};

use crate::{AbsoluteTime, ElapsedTime};

impl ElapsedTime for PlaydateAPI {
    fn elapsed_time(&self) -> Duration {
        unsafe { self.system.as_ref().unwrap().elapsed_time() }
    }

    fn reset_elapsed_time(&self) -> Duration {
        unsafe { self.system.as_ref().unwrap().reset_elapsed_time() }
    }
}

impl AbsoluteTime for PlaydateAPI {
    fn elapsed_since_epoch(&self) -> Duration {
        unsafe { self.system.as_ref().unwrap().elapsed_since_epoch() }
    }
}

impl ElapsedTime for ffi::playdate_sys {
    fn elapsed_time(&self) -> core::time::Duration {
        let seconds = unsafe { self.getElapsedTime.unwrap()() };
        Duration::from_secs_f32(seconds)
    }

    fn reset_elapsed_time(&self) -> core::time::Duration {
        let elapsed = self.elapsed_time();
        unsafe {
            self.resetElapsedTime.unwrap()();
        }
        elapsed
    }
}

impl AbsoluteTime for ffi::playdate_sys {
    fn elapsed_since_epoch(&self) -> core::time::Duration {
        let mut milliseconds = 0;
        let seconds =
            unsafe { self.getSecondsSinceEpoch.unwrap()(ptr::addr_of_mut!(milliseconds)) } as u64;
        Duration::from_secs(seconds) + Duration::from_millis(milliseconds as u64)
    }
}
