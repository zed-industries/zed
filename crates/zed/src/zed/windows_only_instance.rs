use anyhow::Context;
use gpui::app_identifier::{
    get_app_instance_event_identifier, get_app_instance_mutex_identifier, register_app_identifier,
    write_dock_action_argument,
};
use release_channel::ReleaseChannel;
use util::ResultExt;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, ERROR_ALREADY_EXISTS},
        System::Threading::{
            CreateMutexW, OpenEventW, SetEvent, EVENT_MODIFY_STATE, SYNCHRONIZATION_ACCESS_RIGHTS,
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
    unsafe {
        CreateMutexW(
            None,
            true,
            &HSTRING::from(get_app_instance_mutex_identifier()),
        )
        .expect("Unable to create instance mutex.")
    };
    let last_err = unsafe { GetLastError() };
    last_err != ERROR_ALREADY_EXISTS
}

pub(crate) fn send_instance_message(message: &str) {
    write_dock_action_argument(message);
    unsafe {
        if let Some(event) = OpenEventW(
            SYNCHRONIZATION_ACCESS_RIGHTS(EVENT_MODIFY_STATE.0),
            false,
            &HSTRING::from(get_app_instance_event_identifier()),
        )
        .context("Unable to open single instance event, is there an instance already running?")
        .log_err()
        {
            SetEvent(event).log_err();
        };
    }
}
