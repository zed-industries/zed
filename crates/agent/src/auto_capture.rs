use std::sync::OnceLock;

pub struct AutoCaptureConfig {
    handles: Vec<&'static str>,
}

impl AutoCaptureConfig {
    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<AutoCaptureConfig> = OnceLock::new();

        CONFIG.get_or_init(|| {
            AutoCaptureConfig {
                handles: vec![
                    "tmickleydoyle",
                    // Add more handles here as needed
                ],
            }
        })
    }

    pub fn should_track(&self, github_handle: &str) -> bool {
        self.handles.contains(&github_handle)
    }
}
