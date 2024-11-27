use gpui::app_identifier::{get_app_single_instance_mutex_identifier, register_app_identifier};
use release_channel::ReleaseChannel;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, ERROR_ALREADY_EXISTS},
        System::Threading::CreateMutexW,
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
            &HSTRING::from(get_app_single_instance_mutex_identifier()),
        )
        .expect("Unable to create instance mutex.")
    };
    let last_err = unsafe { GetLastError() };
    last_err != ERROR_ALREADY_EXISTS
}
