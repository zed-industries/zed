# winrt-notification

[![license](https://img.shields.io/crates/l/tauri-winrt-notification.svg)](https://crates.io/crates/tauri-winrt-notification/)
[![documentation](https://img.shields.io/crates/v/tauri-winrt-notification?style=flat-square)](https://docs.rs/tauri-winrt-notification)


An incomplete wrapper over the WinRT toast api

Tested in Windows 10 and 8.1. Untested in Windows 8, might work.

Todo:
* Add support for Adaptive Content

Known Issues:
* Will not work for Windows 7.

Limitations:
* Windows 8.1 only supports a single image, the last image (icon, hero, image) will be the one on the toast

## Usage

```toml
#Cargo.toml
[dependencies]
tauri-winrt-notification = "0.5.1"
```

## Examples

```rust
extern crate winrt_notification;
use tauri_winrt_notification::{Duration, Sound, Toast};

fn main() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Look at this flip!")
        .text1("(╯°□°）╯︵ ┻━┻")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
```

```rust
extern crate winrt_notification;
use std::path::Path;
use tauri_winrt_notification::{IconCrop, Toast};

fn main() {
    Toast::new("Your AppUserModeId")
        .hero(&Path::new("C:\\absolute\\path\\to\\image.jpeg"), "alt text")
        .icon(
            &Path::new("c:/this/style/works/too/image.png"),
            IconCrop::Circular,
            "alt text",
        )
        .title("Lots of pictures here")
        .text1("One above the text as the hero")
        .text2("One to the left as an icon, and several below")
        .image(&Path::new("c:/photos/sun.png"), "the sun")
        .image(&Path::new("c:/photos/moon.png"), "the moon")
        .sound(None) // will be silent
        .show()
        .expect("unable to toast");
}
```
