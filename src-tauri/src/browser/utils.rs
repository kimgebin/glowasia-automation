use std::time::Duration;
use rand::Rng;

/// Generate random delay between min and max milliseconds
pub fn random_delay(min_ms: u64, max_ms: u64) -> Duration {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(min_ms..=max_ms);
    Duration::from_millis(delay)
}

/// Generate random numeric string (for fake fingerprinting)
pub fn random_numeric(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen_range(0..10).to_string()).collect()
}