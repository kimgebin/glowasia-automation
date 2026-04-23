use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

pub struct AutomationMonitor {
    running: AtomicBool,
    interval_ms: AtomicU64,
    last_check: Mutex<Instant>,
    start_time: Mutex<Option<Instant>>,
}

impl AutomationMonitor {
    pub fn new() -> Self {
        Self {
            running: AtomicBool::new(false),
            interval_ms: AtomicU64::new(5000),
            last_check: Mutex::new(Instant::now()),
            start_time: Mutex::new(None),
        }
    }
    
    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
        let mut start = self.start_time.lock().unwrap();
        *start = Some(Instant::now());
        // Reset last check
        let mut last = self.last_check.lock().unwrap();
        *last = Instant::now();
    }
    
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
    
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
    
    pub fn get_interval_ms(&self) -> u64 {
        self.interval_ms.load(Ordering::SeqCst)
    }
    
    pub fn set_interval(&self, seconds: u64) {
        self.interval_ms.store(seconds * 1000, Ordering::SeqCst);
    }
    
    pub fn get_uptime_seconds(&self) -> Option<u64> {
        let start = self.start_time.lock().unwrap();
        start.map(|s| s.elapsed().as_secs())
    }
    
    pub fn get_idle_seconds(&self) -> u64 {
        let last = self.last_check.lock().unwrap();
        last.elapsed().as_secs()
    }
    
    pub fn refresh(&self) {
        let mut last = self.last_check.lock().unwrap();
        *last = Instant::now();
    }
}

impl Default for AutomationMonitor {
    fn default() -> Self {
        Self::new()
    }
}
