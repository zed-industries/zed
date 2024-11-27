use anyhow::Context;
use util::ResultExt;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Memory::{
            CreateFileMappingW, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ,
            FILE_MAP_WRITE, PAGE_READWRITE,
        },
        Threading::{
            CreateEventW, OpenEventW, SetEvent, EVENT_MODIFY_STATE, SYNCHRONIZATION_ACCESS_RIGHTS,
        },
    },
};
use windows_core::{Owned, HSTRING};

use super::app_identifier::{
    get_app_dock_action_event_identifier, get_app_dock_action_shared_memory_identifier,
};

const APP_SHARED_MEMORY_MAX_SIZE: usize = 1024;

pub(crate) const APP_DOCK_ACTION_ARGUMENT: &str = "dock-action";

pub(crate) fn create_dock_action_event() -> Owned<HANDLE> {
    unsafe {
        Owned::new(
            CreateEventW(
                None,
                false,
                false,
                &HSTRING::from(get_app_dock_action_event_identifier()),
            )
            .expect("Unable to create single instance event."),
        )
    }
}

pub(crate) fn create_dock_action_shared_memory() -> Owned<HANDLE> {
    unsafe {
        Owned::new(
            CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                None,
                PAGE_READWRITE,
                0,
                APP_SHARED_MEMORY_MAX_SIZE as u32,
                &HSTRING::from(get_app_dock_action_shared_memory_identifier()),
            )
            .expect("Unable to create shared memory"),
        )
    }
}

pub(crate) fn read_dock_action_argument(shared_memory_handle: HANDLE) -> String {
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
pub fn send_dock_action_message(message: &str) {
    write_dock_action_argument(message);
    set_dock_action_event();
}

fn write_dock_action_argument(message: &str) {
    if message.len() > APP_SHARED_MEMORY_MAX_SIZE {
        log::error!(
            "The length of the message to send should be less than {APP_SHARED_MEMORY_MAX_SIZE}"
        );
        return;
    }
    unsafe {
        let msg = message.as_bytes();
        let memory_handle = OpenFileMappingW(
            FILE_MAP_WRITE.0,
            false,
            &HSTRING::from(get_app_dock_action_shared_memory_identifier()),
        )
        .unwrap();
        let memory_addr = MapViewOfFile(memory_handle, FILE_MAP_WRITE, 0, 0, 0);
        // Clear the buffer first
        let empty_buffer = vec![0u8; APP_SHARED_MEMORY_MAX_SIZE];
        std::ptr::copy_nonoverlapping(
            empty_buffer.as_ptr(),
            memory_addr.Value as _,
            empty_buffer.len(),
        );
        std::ptr::copy_nonoverlapping(msg.as_ptr(), memory_addr.Value as _, msg.len());
        UnmapViewOfFile(memory_addr).log_err();
        CloseHandle(memory_handle).log_err();
    }
}

fn set_dock_action_event() {
    unsafe {
        if let Some(event) = OpenEventW(
            SYNCHRONIZATION_ACCESS_RIGHTS(EVENT_MODIFY_STATE.0),
            false,
            &HSTRING::from(get_app_dock_action_event_identifier()),
        )
        .context("Unable to open single instance event, is there an instance already running?")
        .log_err()
        {
            SetEvent(event).log_err();
        };
    }
}
