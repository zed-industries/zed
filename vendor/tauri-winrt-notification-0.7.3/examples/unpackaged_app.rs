// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2025 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{thread::sleep, time::Duration};

use tauri_winrt_notification::Toast;
use windows_registry::CURRENT_USER;

// App's AUMID (Application User Model Id), should be unique and no longer than 129 characters.
// https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/send-local-toast-other-apps#aumid-restrictions
const APP_ID: &str = "tauri.UnpackagedAppWinRtNotificationExample";

// App's display name
const APP_NAME: &str = "Unpackaged App";

fn main() -> windows_registry::Result<()> {
    init_registry()?;

    Toast::new(APP_ID)
        .title("Notification from an unpackaged app")
        .scenario(tauri_winrt_notification::Scenario::Reminder)
        .add_button("OK", "ok")
        .show()
        .expect("unable to send notification");

    // The notification won't appear if we clean up registry too early
    sleep(Duration::from_secs(3));
    clean_up_registry()
}

// Create registry key for this example
fn init_registry() -> windows_registry::Result<()> {
    let icon_path = std::env::current_dir()?.join(r"resources\tauri.png");

    let key = CURRENT_USER.create(format!(r"SOFTWARE\Classes\AppUserModelId\{APP_ID}"))?;
    key.set_string("DisplayName", APP_NAME)?;
    key.set_string("IconBackgroundColor", "0")?;
    key.set_hstring("IconUri", &icon_path.as_path().into())
}

// Remove this example from the registry
fn clean_up_registry() -> windows_registry::Result<()> {
    CURRENT_USER.remove_tree(format!(r"SOFTWARE\Classes\AppUserModelId\{APP_ID}"))
}
