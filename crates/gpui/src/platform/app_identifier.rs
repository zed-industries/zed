use std::sync::OnceLock;

use windows::Win32::Foundation::MAX_PATH;

static APP_IDENTIFIER: OnceLock<String> = OnceLock::new();
static APP_MUTEX_IDENTIFIER: OnceLock<String> = OnceLock::new();
static APP_EVENT_IDENTIFIER: OnceLock<String> = OnceLock::new();
static APP_SHARED_MEMORY_IDENTIFIER: OnceLock<String> = OnceLock::new();

/// TODO:
pub const APP_SHARED_MEMORY_MAX_SIZE: usize = 1024;

/// TODO:
pub fn register_app_identifier(app_identifier: &str) {
    APP_IDENTIFIER.get_or_init(|| app_identifier.to_string());
}

fn get_app_identifier() -> &'static str {
    APP_IDENTIFIER.get_or_init(|| {
        let rand_number = rand::random::<u32>();
        let random_identifier = format!("Gpui-App-Identifier-{}", rand_number);
        log::error!(
            "No app identifier is set, call register_app_identifier first. Using {} instead.",
            random_identifier
        );
        random_identifier
    })
}

/// TODO:
pub fn get_app_instance_mutex_identifier() -> &'static str {
    APP_MUTEX_IDENTIFIER.get_or_init(|| {
        let identifier = format!("Local\\{}-Instance-Mutex", get_app_identifier());
        if identifier.len() as u32 > MAX_PATH {
            panic!("The length of app instance mutex identifier `{identifier}` is limited to {MAX_PATH} characters.");
        }
        identifier
    })
}

/// TODO:
pub fn get_app_instance_event_identifier() -> &'static str {
    APP_EVENT_IDENTIFIER.get_or_init(|| {
        let identifier = format!("Local\\{}-Instance-Event", get_app_identifier());
        if identifier.len() as u32 > MAX_PATH {
            panic!("The length of app instance event identifier `{identifier}` is limited to {MAX_PATH} characters.");
        }
        identifier
    })
}

/// TODO:
pub fn get_app_shared_memory_identifier() -> &'static str {
    APP_SHARED_MEMORY_IDENTIFIER.get_or_init(|| {
        let identifier = format!("Local\\{}-Shared-Memory", get_app_identifier());
        if identifier.len() as u32 > MAX_PATH {
            panic!("The length of app shared memory identifier `{identifier}` is limited to {MAX_PATH} characters.");
        }
        identifier
    })
}
