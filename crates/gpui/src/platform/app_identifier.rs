use std::sync::OnceLock;

use windows::Win32::Foundation::MAX_PATH;

static APP_IDENTIFIER: OnceLock<String> = OnceLock::new();
static APP_EVENT_IDENTIFIER: OnceLock<String> = OnceLock::new();
static APP_SHARED_MEMORY_IDENTIFIER: OnceLock<String> = OnceLock::new();

/// TODO:
pub const APP_SHARED_MEMORY_MAX_SIZE: usize = 1024;

/// TODO:
pub fn register_app_identifier(app_identifier: &str) {
    APP_IDENTIFIER.get_or_init(|| app_identifier.to_string());
}

/// TODO:
pub fn get_app_instance_event_identifier() -> &'static str {
    APP_EVENT_IDENTIFIER.get_or_init(|| {
        let identifier = format!("Local\\{}-Instance-Event", APP_EVENT_IDENTIFIER.get().unwrap());
        if identifier.len() as u32 > MAX_PATH {
            panic!("The length of app instance event identifier `{identifier}` is limited to {MAX_PATH} characters.");
        }
        identifier
    })
}

/// TODO:
pub fn get_app_shared_memory_identifier() -> &'static str {
    APP_SHARED_MEMORY_IDENTIFIER.get_or_init(|| {
        let identifier = format!("Local\\{}-Shared-Memory", APP_EVENT_IDENTIFIER.get().unwrap());
        if identifier.len() as u32 > MAX_PATH {
            panic!("The length of app shared memory identifier `{identifier}` is limited to {MAX_PATH} characters.");
        }
        identifier
    })
}
