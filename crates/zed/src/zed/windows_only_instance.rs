use gpui::app_identifier::{get_app_instance_event_identifier, register_app_identifier};
use release_channel::ReleaseChannel;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, ERROR_ALREADY_EXISTS},
        System::Threading::CreateEventW,
    },
};

fn register_zed_identifier() {
    match *release_channel::RELEASE_CHANNEL {
        ReleaseChannel::Dev => register_app_identifier("Zed-Editor-Dev"),
        ReleaseChannel::Nightly => register_app_identifier("Zed-Editor-Nightly"),
        ReleaseChannel::Preview => register_app_identifier("Zed-Editor-Preview"),
        ReleaseChannel::Stable => register_app_identifier("Zed-Editor-Stable"),
    }
}

pub fn check_single_instance() -> bool {
    if *db::ZED_STATELESS || *release_channel::RELEASE_CHANNEL == ReleaseChannel::Dev {
        return true;
    }

    register_zed_identifier();
    check_single_instance_event()
}

fn check_single_instance_event() -> bool {
    unsafe {
        CreateEventW(
            None,
            false,
            false,
            &HSTRING::from(get_app_instance_event_identifier()),
        )
        .expect("Unable to create instance sync event")
    };
    let last_err = unsafe { GetLastError() };
    last_err != ERROR_ALREADY_EXISTS
}
