use core::time::Duration;

use timer::Timer;

pub struct Animation {
    pub current_frame: usize,
    len: usize,
    timer: Timer,
}

impl Animation {
    pub fn new(len: usize, fps: f32) -> Self {
        Self {
            current_frame: 0,
            len,
            timer: Timer::new(Duration::from_secs_f32(1. / fps)),
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.timer.update(delta_time);
        if self.timer.is_finished() {
            self.current_frame = (self.current_frame + 1) % self.len;
        }
    }
}
