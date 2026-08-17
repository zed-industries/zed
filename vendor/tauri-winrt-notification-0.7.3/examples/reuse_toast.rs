// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri_winrt_notification::{Duration, Sound, Toast};

fn main() {
    let duration = Duration::Short;
    let sound = Some(Sound::SMS);

    let toast = Toast::new(Toast::POWERSHELL_APP_ID)
        .title("first toast")
        .text1("line1")
        .duration(duration)
        .sound(sound);

    toast
        .show()
        // silently consume errors
        .expect("notification failed");

    toast
        .show()
        // silently consume errors
        .expect("notification failed");
}
