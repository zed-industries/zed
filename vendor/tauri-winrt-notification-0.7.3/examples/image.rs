// Copyright 2017-2022 allenbenz <allenbenz@users.noreply.github.com>
// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::path::Path;
use tauri_winrt_notification::{IconCrop, Toast};

fn main() {
    Toast::new("application that needs a toast with an image")
        .hero(Path::new("C:\\absolute\\path\\to\\image.jpeg"), "alt text")
        .icon(
            Path::new("c:/this/style/works/too/image.png"),
            IconCrop::Circular,
            "alt text",
        )
        .title("Lots of pictures here")
        .text1("One above the text as the hero")
        .text2("One to the left as an icon, and several below")
        .image(Path::new("c:/photos/sun.png"), "the sun")
        .image(Path::new("c:/photos/moon.png"), "the moon")
        .sound(None) // will be silent
        .show()
        .expect("notification failed");
}
