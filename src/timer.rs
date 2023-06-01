use std::time::Duration;

pub struct Timer {
    duration: Duration,
    time_left: Duration,
    pub ready: bool,
}

impl Timer {
    pub fn new(time: u64) -> Timer {
        let duration = Duration::from_millis(time);

        Timer {
            duration,
            time_left: duration,
            ready: false,
        }
    }

    pub fn reset(&mut self) {
        self.time_left = self.duration;
        self.ready = false;
    }

    pub fn update(&mut self, delta: Duration) {
        if self.ready {
            return;
        }

        if let Some(time_left) = self.time_left.checked_sub(delta) {
            self.time_left = time_left;
        } else {
            self.time_left = Duration::from_millis(0);
            self.ready = true;
        }
    }
}
