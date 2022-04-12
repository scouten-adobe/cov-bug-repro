use log::info;
use std::time::Instant;

pub struct TimeIt {
    label: &'static str,
    start: Instant,
}

impl TimeIt {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            start: Instant::now(),
        }
    }
}
impl Drop for TimeIt {
    fn drop(&mut self) {
        info!("timing for {}: {:.2?}", self.label, self.start.elapsed());
    }
}
