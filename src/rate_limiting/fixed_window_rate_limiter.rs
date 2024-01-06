use std::collections::HashMap;
use std::time::{Duration, Instant};

/**
Fixed window rate limiter
 */

pub struct FixedWindowRateLimiter {
    window_start: Instant,
    window_duration: Duration,
    limit: usize,
    request_count: usize,
}

impl FixedWindowRateLimiter {
    pub fn new(window_duration: Duration, limit: usize) -> Self {
        Self {
            window_start: Instant::now(),
            window_duration,
            limit,
            request_count: 0,
        }
    }

    pub fn request_allowed(&mut self) -> bool {
        self.reset_window();

        if self.request_count < self.limit {
            self.request_count += 1;
            true
        } else {
            false
        }
    }

    fn reset_window(&mut self) {
        if self.window_start.elapsed() >= self.window_duration {
            self.window_start = Instant::now();
            self.request_count = 0
        }
    }

    pub fn get_count(&mut self) -> usize {
        self.reset_window();
        self.request_count
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_rate_limiter_allow() {
        let window_duration = Duration::from_secs(1);
        let mut rate_limiter = FixedWindowRateLimiter::new(window_duration, 2);

        assert!(rate_limiter.request_allowed());
        assert!(rate_limiter.request_allowed());
        assert!(!rate_limiter.request_allowed());

        sleep(window_duration);

        assert!(rate_limiter.request_allowed());
        assert!(rate_limiter.request_allowed());
        assert!(!rate_limiter.request_allowed());
    }

    #[test]
    fn test_rate_limiter_count_reset() {
        let window_duration = Duration::from_secs(1);
        let mut rate_limiter = FixedWindowRateLimiter::new(window_duration, 2);

        // Perform two operations
        assert!(rate_limiter.request_allowed());
        assert!(rate_limiter.request_allowed());

        sleep(window_duration);

        // Check that the count is reset after the window reset
        assert_eq!(rate_limiter.get_count(), 0);
    }
}