// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{thread::sleep, time::Duration as StdDuration};
use tauri_winrt_notification::{Duration, NotificationUpdateResult, Progress, Toast};

fn main() {
    let mut progress = Progress {
        tag: "my_tag".to_string(),
        title: "video.mp4".to_string(),
        status: "Transferring files...".to_string(),
        value: 0.0,
        value_string: "0/1000 MB".to_string(),
    };

    let toast = Toast::new(Toast::POWERSHELL_APP_ID)
        .title("File Transfer from Phone")
        .text1("Transferring files to your computer...")
        .progress(&progress)
        .duration(Duration::Long);
    toast.show().expect("notification failed");

    for i in 1..=10 {
        sleep(StdDuration::from_secs(1));

        progress.value = i as f32 / 10.0;
        progress.value_string = format!("{}/1000 MB", i * 100);

        if i == 10 {
            progress.status = String::from("Completed");
        };

        if let Ok(update_result) = toast.set_progress(&progress) {
            match update_result {
                NotificationUpdateResult::Succeeded => {
                    println!("notification updated successfully.");
                }
                NotificationUpdateResult::Failed => {
                    println!("failed to update notification")
                }
                NotificationUpdateResult::NotificationNotFound => {
                    println!("notification not found. Please ensure the notification ID and Tag are correct.");
                }
                _ => println!("unknown notification update result"),
            }
        };
    }
}
