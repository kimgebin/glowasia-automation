use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    pub hide_webdriver: bool,
    pub randomize_canvas: bool,
    pub randomize_webgl: bool,
    pub fake_timezone: bool,
    pub fake_locale: bool,
    pub fake_plugins: bool,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            hide_webdriver: true,
            randomize_canvas: true,
            randomize_webgl: true,
            fake_timezone: true,
            fake_locale: true,
            fake_plugins: true,
        }
    }
}

impl StealthConfig {
    pub fn get_chrome_args(&self) -> Vec<&'static str> {
        let mut args = vec![
            "--disable-blink-features=AutomationControlled",
            "--no-sandbox",
            "--disable-setuid-sandbox",
            "--disable-dev-shm-usage",
            "--disable-accelerated-2d-canvas",
            "--no-first-run",
            "--no-zygote",
            "--disable-gpu",
        ];
        
        if self.hide_webdriver {
            args.push("--disable-webdriver");
        }
        
        args
    }
    
    pub fn get_user_agent(&self) -> String {
        // Random real-looking user agents
        let agents = vec![
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        ];
        
        agents[rand::random::<usize>() % agents.len()].to_string()
    }
    
    pub fn get_viewport(&self) -> (u32, u32) {
        // Common viewport sizes
        let viewports = vec![
            (1920, 1080),
            (1366, 768),
            (1440, 900),
            (1536, 864),
            (1280, 720),
        ];
        
        viewports[rand::random::<usize>() % viewports.len()]
    }
}