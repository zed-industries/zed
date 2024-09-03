use gpui::app_identifier::{
    get_app_instance_event_identifier, get_app_shared_memory_identifier, register_app_identifier,
    APP_SHARED_MEMORY_MAX_SIZE,
};
use release_channel::ReleaseChannel;
use util::ResultExt;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS},
        Storage::FileSystem::SYNCHRONIZE,
        System::{
            Memory::{MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_WRITE},
            Threading::{
                CreateEventW, CreateMutexW, OpenEventW, SetEvent, EVENT_MODIFY_STATE,
                SYNCHRONIZATION_ACCESS_RIGHTS,
            },
        },
    },
};

pub fn register_zed_identifier() {
    match *release_channel::RELEASE_CHANNEL {
        ReleaseChannel::Dev => register_app_identifier("Zed-Editor-Dev"),
        ReleaseChannel::Nightly => register_app_identifier("Zed-Editor-Nightly"),
        ReleaseChannel::Preview => register_app_identifier("Zed-Editor-Preview"),
        ReleaseChannel::Stable => register_app_identifier("Zed-Editor-Stable"),
    };
}

pub fn check_single_instance() -> bool {
    // if *db::ZED_STATELESS || *release_channel::RELEASE_CHANNEL == ReleaseChannel::Dev {
    //     return true;
    // }

    // register_zed_identifier();
    check_single_instance_event()
}

pub fn send_instance_message(message: &str) {
    send_message_to_other_instance(message);
    unsafe {
        let event = OpenEventW(
            SYNCHRONIZATION_ACCESS_RIGHTS(EVENT_MODIFY_STATE.0),
            false,
            &HSTRING::from(get_app_instance_event_identifier()),
        )
        .expect("Unable to open single instance event, is there an instance already running?");
        SetEvent(event).log_err();
    }
}

fn check_single_instance_event() -> bool {
    unsafe {
        CreateMutexW(
            None,
            true,
            // &HSTRING::from(get_app_instance_event_identifier()),
            &HSTRING::from("Zed-Single_instance-Test"),
        )
        // CreateEventW(
        //     None,
        //     false,
        //     false,
        //     &HSTRING::from(get_app_instance_event_identifier()),
        // )
        .expect("Unable to create instance sync event")
    };
    let last_err = unsafe { GetLastError() };
    last_err != ERROR_ALREADY_EXISTS
}

fn send_message_to_other_instance(message: &str) {
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
        std::ptr::copy_nonoverlapping(msg.as_ptr(), memory_addr.Value as _, msg.len());
        UnmapViewOfFile(memory_addr).log_err();
        CloseHandle(pipe).log_err();
    }
}
