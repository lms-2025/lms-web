use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub window_size: u64,  // time window in seconds
    pub max_requests: u64, // maximum requests allowed in the time window
    pub current_requests: u64,
    pub last_reset: u64, // time of the last reset
}

impl RateLimit {
    pub fn new(window_size: u64, max_requests: u64) -> Self {
        let current_time = chrono::Utc::now().timestamp() as u64;
        RateLimit {
            window_size,
            max_requests,
            current_requests: 0,
            last_reset: current_time,
        }
    }

    pub fn is_rate_limited(&self, current_time: u64) -> bool {
        if self.should_reset(current_time) {
            return false;
        }
        self.current_requests >= self.max_requests
    }

    pub fn increment(&mut self, current_time: u64) {
        if self.should_reset(current_time) {
            self.last_reset = current_time;
            self.current_requests = 0;
        }
        self.current_requests += 1;
    }

    pub fn remaining_requests(&self, current_time: u64) -> u64 {
        if self.should_reset(current_time) {
            return self.max_requests;
        }
        self.max_requests.saturating_sub(self.current_requests)
    }

    pub fn time_until_reset(&self, current_time: u64) -> u64 {
        if self.should_reset(current_time) {
            return 0;
        }
        let time_passed = current_time - self.last_reset;
        self.window_size.saturating_sub(time_passed)
    }

    pub fn reset(&mut self, current_time: u64) {
        self.last_reset = current_time;
        self.current_requests = 0;
    }

    fn should_reset(&self, current_time: u64) -> bool {
        current_time - self.last_reset >= self.window_size
    }
}

impl Default for RateLimit {
    fn default() -> Self {
        Self::new(60, 100) // Default: 100 requests per minute
    }
}
