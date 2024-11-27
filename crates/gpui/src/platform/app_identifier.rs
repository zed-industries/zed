use std::sync::OnceLock;

use util::ResultExt;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, MAX_PATH},
    System::Memory::{
        MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ, FILE_MAP_WRITE,
    },
};
use windows_core::HSTRING;

static APP_IDENTIFIER: OnceLock<String> = OnceLock::new();
static APP_MUTEX_IDENTIFIER: OnceLock<String> = OnceLock::new();
static APP_EVENT_IDENTIFIER: OnceLock<String> = OnceLock::new();
static APP_SHARED_MEMORY_IDENTIFIER: OnceLock<String> = OnceLock::new();

/// TODO:
pub const APP_SHARED_MEMORY_MAX_SIZE: usize = 1024;

/// TODO:
pub const APP_DOCK_ACTION_ARGUMENT: &str = "dock-action";

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

/// TODO:
pub fn read_dock_action_argument(shared_memory_handle: HANDLE) -> String {
    unsafe {
        let memory_addr = MapViewOfFile(shared_memory_handle, FILE_MAP_READ, 0, 0, 0);
        let string = String::from_utf8_lossy(std::slice::from_raw_parts(
            memory_addr.Value as *const _ as _,
            APP_SHARED_MEMORY_MAX_SIZE,
        ))
        .trim_matches('\0')
        .to_string();
        UnmapViewOfFile(memory_addr).log_err();
        string
    }
}

/// TODO:
pub fn write_dock_action_argument(message: &str) {
    if message.len() > APP_SHARED_MEMORY_MAX_SIZE {
        log::error!(
            "The length of the message to send should be less than {APP_SHARED_MEMORY_MAX_SIZE}"
        );
        return;
    }
    unsafe {
        let msg = message.as_bytes();
        let pipe = OpenFileMappingW(
            FILE_MAP_WRITE.0,
            false,
            &HSTRING::from(get_app_shared_memory_identifier()),
        )
        .unwrap();
        let memory_addr = MapViewOfFile(pipe, FILE_MAP_WRITE, 0, 0, 0);
        // Clear the buffer first
        let empty_buffer = vec![0u8; APP_SHARED_MEMORY_MAX_SIZE];
        std::ptr::copy_nonoverlapping(
            empty_buffer.as_ptr(),
            memory_addr.Value as _,
            empty_buffer.len(),
        );
        std::ptr::copy_nonoverlapping(msg.as_ptr(), memory_addr.Value as _, msg.len());
        UnmapViewOfFile(memory_addr).log_err();
        CloseHandle(pipe).log_err();
    }
}
