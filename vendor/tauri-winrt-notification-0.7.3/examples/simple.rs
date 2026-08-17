// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{process::exit, thread::sleep, time::Duration as StdDuration};

use tauri_winrt_notification::{Duration, Sound, Toast, ToastDismissalReason};

fn main() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Look at this flip!")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .on_activated(move |action| {
            match action {
                Some(action) => println!("You've clicked {action}!"),
                None => println!("You've clicked me!"),
            }
            exit(0);
        })
        .add_button("Yes", "yes")
        .add_button("No", "no")
        .on_dismissed(|reason| {
            match reason {
                Some(ToastDismissalReason::UserCanceled) => println!("UserCanceled"),
                Some(ToastDismissalReason::ApplicationHidden) => println!("ApplicationHidden"),
                Some(ToastDismissalReason::TimedOut) => println!("TimedOut"),
                _ => println!("Unknown"),
            }
            exit(0)
        })
        .show()
        .expect("unable to send notification");

    println!("Waiting 10 seconds for the notification to be clicked...");
    sleep(StdDuration::from_secs(10));
    println!("The notification wasn't clicked!");
}
