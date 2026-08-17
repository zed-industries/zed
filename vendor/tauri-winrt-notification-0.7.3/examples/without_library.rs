// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// How to create a toast without using this library
use std::path::Path;

// You need to have the windows crate in your Cargo.toml
// with the following features:
//    "Data_Xml_Dom"
//    "UI_Notifications"
use windows::{
    Data::Xml::Dom::XmlDocument, UI::Notifications::ToastNotification,
    UI::Notifications::ToastNotificationManager,
};

pub use windows::core::{Error, HSTRING};

// In a real project you'd use quick-xml::escape::escape
#[path = "../src/xml_escape.rs"]
mod xml_escape;

fn main() {
    do_toast().expect("not sure if this is actually failable");
    // this is a hack to workaround toasts not showing up if the application closes too quickly
    // you can put this in do_toast if you want.
    std::thread::sleep(std::time::Duration::from_millis(10));
}

fn do_toast() -> windows::core::Result<()> {
    let toast_xml = XmlDocument::new()?;

    toast_xml.LoadXml(&HSTRING::from(
        format!(r#"<toast duration="long">
                <visual>
                    <binding template="ToastGeneric">
                        <text id="1">title</text>
                        <text id="2">first line</text>
                        <text id="3">third line</text>
                        <image placement="appLogoOverride" hint-crop="circle" src="file:///c:/path_to_image_above_toast.jpg" alt="alt text" />
                        <image placement="Hero" src="file:///C:/path_to_image_in_toast.jpg" alt="alt text2" />
                        <image id="1" src="file:///{}" alt="another_image" />
                    </binding>
                </visual>
                <audio src="ms-winsoundevent:Notification.SMS" />
                <!-- <audio silent="true" /> -->
            </toast>"#,
            xml_escape::escape(Path::new("C:\\path_to_image_in_toast.jpg").display().to_string()),
    ))).expect("the xml is malformed");

    // Create the toast and attach event listeners
    let toast_template = ToastNotification::CreateToastNotification(&toast_xml)?;

    // If you have a valid app id, (ie installed using wix) then use it here.
    let toast_notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(
        "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe",
    ))?;

    // Show the toast.
    // Note this returns success in every case, including when the toast isn't shown.
    toast_notifier.Show(&toast_template)
}
