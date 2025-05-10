use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct PomodoroTimer {
    pub started_at: Option<DateTime<Utc>>,
    pub running: bool,
}

impl PomodoroTimer {
    pub fn new() -> Self {
        PomodoroTimer {
            started_at: None,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(Utc::now());
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.started_at = None;
        self.running = false;
    }

    pub fn toggle(&mut self) {
        if self.running {
            self.stop();
        } else {
            self.start();
        }
    }
}
