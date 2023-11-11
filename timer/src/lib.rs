#![no_std]

use core::time::Duration;

pub struct Timer {
    total: Duration,
    remaining: Duration,
    finished: bool,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            total: duration,
            remaining: duration,
            finished: false,
        }
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.finished = false;
        while elapsed >= self.remaining {
            self.finished = true;
            self.remaining += self.total
        }
        self.remaining -= elapsed;
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_not_finish_before_update() {
        assert!(!Timer::new(Duration::from_secs(1),).is_finished());
    }

    #[test]
    fn is_finished_after_exact_time_passed() {
        let mut timer = Timer::new(Duration::from_secs(1));
        timer.update(Duration::from_secs(1));
        assert!(timer.is_finished());
    }

    #[test]
    fn is_finished_after_more_time_passed() {
        let mut timer = Timer::new(Duration::from_secs(1));
        timer.update(Duration::from_secs(1));
        assert!(timer.is_finished());
    }

    #[test]
    fn is_not_finished_after_less_time_passed() {
        let mut timer = Timer::new(Duration::from_secs(1));
        timer.update(Duration::from_millis(999));
        assert!(!timer.is_finished());
    }

    #[test]
    fn is_finished_after_enough_time_in_multiple_steps() {
        let mut timer = Timer::new(Duration::from_secs(1));
        timer.update(Duration::from_millis(500));
        timer.update(Duration::from_millis(500));
        assert!(timer.is_finished());
    }

    #[test]
    fn is_looping() {
        let mut timer = Timer::new(Duration::from_secs(1));
        timer.update(Duration::from_millis(1200));
        assert!(timer.is_finished());
        timer.update(Duration::from_millis(500));
        assert!(!timer.is_finished());
        timer.update(Duration::from_millis(300));
        assert!(timer.is_finished());
    }
}
