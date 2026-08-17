# WinSafe

[![Crates.io](https://img.shields.io/crates/v/winsafe.svg)](https://crates.io/crates/winsafe)
[![Docs.rs](https://docs.rs/winsafe/badge.svg)](https://docs.rs/winsafe)
[![Lines of code](https://tokei.rs/b1/github/rodrigocfd/winsafe)](https://github.com/rodrigocfd/winsafe)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Windows API and GUI in safe, idiomatic Rust.

WinSafe has:

* low-level Win32 API constants, functions and structs;
* high-level structs to build native Win32 GUI applications.

WinSafe documentation:

| Branch | Docs |
| - | - |
| Stable | [docs.rs/winsafe](https://docs.rs/winsafe) |
| Nightly (master) | [rodrigocfd.github.io/winsafe/winsafe](https://rodrigocfd.github.io/winsafe/winsafe/) |

## Current status

Native FFI items implemented:

| Native FFI item | Count |
| - | -: |
| Functions | 744 |
| Structs | 227 |
| Constants | 13,317 |
| Window messages | 651 |
| Handles | 45 |
| COM interfaces | 82 |
| COM methods | 450 |

High-level GUI controls:

* User custom window/dialog – main, modal, modeless, control, message-only.
* Native controls – button, check box, combo box, date and time picker, edit, label, list box, list view, month calendar, progress bar, radio button, status bar, tab, track bar, tree view, up down.

## Usage

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
winsafe = { version = "0.0.19", features = [] }
```

You can, alternatively, use the Nightly (master) branch [directly](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories), to get the latest features right away:

```toml
[dependencies]
winsafe = { git = "https://github.com/rodrigocfd/winsafe", features = [] }
```

Then you must enable the [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) you want to be included – these modules are named after native Windows DLL and library names, mostly.

The following Cargo features are available so far:

| Feature | Description |
| - | - |
| `comctl` | ComCtl32.dll, for [Common Controls](https://learn.microsoft.com/en-us/windows/win32/api/_controls/) |
| `dshow` | [DirectShow](https://learn.microsoft.com/en-us/windows/win32/directshow/directshow) |
| `dwm` | Dwmapi.dll, the [Desktop Window Manager](https://learn.microsoft.com/en-us/windows/win32/dwm/dwm-overview) |
| `dxgi` | [DirectX Graphics Infrastructure](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dx-graphics-dxgi) |
| `gdi` | Gdi32.dll, the [Windows GDI](https://learn.microsoft.com/en-us/windows/win32/gdi/windows-gdi) |
| **`gui`** | **The WinSafe high-level GUI abstractions** |
| `kernel` | Kernel32.dll, Advapi32.dll and Ktmw32.dll – all others will include it |
| `mf` | [Media Foundation](https://learn.microsoft.com/en-us/windows/win32/medfound/microsoft-media-foundation-sdk) |
| `ole` | OLE and basic COM support |
| `oleaut` | [OLE Automation](https://learn.microsoft.com/en-us/windows/win32/api/_automat/) |
| `shell` | Shell32.dll and Shlwapi.dll, the COM-based [Windows Shell](https://learn.microsoft.com/en-us/windows/win32/shell/shell-entry) |
| `taskschd` | [Task Scheduler](https://learn.microsoft.com/en-us/windows/win32/taskschd/task-scheduler-start-page) |
| `user` | User32.dll and ComDlg32.dll, the basic Windows GUI support |
| `uxtheme` | UxTheme.dll, extended window theming |
| `version` | Version.dll, to manipulate *.exe version info |

Although WinSafe already has a lot of Win32 APIs, it doesn't have *everything*, simply because Win32 API is gigantic. So if you're looking for a comprehensive Win32 coverage, take a look at [winapi](https://crates.io/crates/winapi) or [windows](https://crates.io/crates/windows) crates, which are *unsafe*, but have everything.

## Example

**Note:** You can find several examples in the dedicated repo: [github.com/rodrigocfd/winsafe-examples](https://github.com/rodrigocfd/winsafe-examples)

WinSafe allows you to create windows in two ways:

* programmatically defining parameters; or
* [loading dialogs](https://github.com/rodrigocfd/winsafe-examples/tree/master/03_dialog_resources) from a `.res` file created with a WYSIWYG resource editor.

The [example below](https://github.com/rodrigocfd/winsafe-examples/tree/master/01_button_click/) creates a window  with a button programmatically. Note how the click event is handled with a closure:

![Example 01](https://raw.githubusercontent.com/rodrigocfd/winsafe-examples/master/01_button_click/screen.gif)

```toml
[dependencies]
winsafe = { version = "0.0.19", features = ["gui"] }
```

```rust
#![windows_subsystem = "windows"]

use winsafe::{self as w, prelude::*, gui};

fn main() {
    let my = MyWindow::new(); // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}


#[derive(Clone)]
pub struct MyWindow {
    wnd:       gui::WindowMain, // responsible for managing the window
    btn_hello: gui::Button,     // a button
}

impl MyWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new( // instantiate the window manager
            gui::WindowMainOpts {
                title: "My window title".to_owned(),
                size: (300, 150),
                ..Default::default() // leave all other options as default
            },
        );

        let btn_hello = gui::Button::new(
            &wnd, // the window manager is the parent of our button
            gui::ButtonOpts {
                text: "&Click me".to_owned(),
                position: (20, 20),
                ..Default::default()
            },
        );

        let new_self = Self { wnd, btn_hello };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        let wnd = self.wnd.clone(); // clone so it can be passed into the closure
        self.btn_hello.on().bn_clicked(move || {
            wnd.hwnd().SetWindowText("Hello, world!")?;
            Ok(())
        });
    }
}
```

## License

Licensed under [MIT license](https://opensource.org/licenses/MIT), see [LICENSE.md](LICENSE.md) for details.
