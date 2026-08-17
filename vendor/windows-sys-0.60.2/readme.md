## windows-sys

The `windows-sys` crate is a zero-overhead fallback for the most demanding situations and primarily where the absolute best compile time is essential. It only includes function declarations (externs), structs, and constants. No convenience helpers, traits, or wrappers are provided.

- [Getting started](https://kennykerr.ca/rust-getting-started/)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
- [Releases](https://github.com/microsoft/windows-rs/releases)
- [Feature search](https://microsoft.github.io/windows-rs/features)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-sys]
version = "0.60"
features = [
    "Win32_Security",
    "Win32_System_Threading",
    "Win32_UI_WindowsAndMessaging",
]
```

Make use of any Windows APIs as needed:

```rust,no_run
use windows_sys::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    unsafe {
        let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(0 as _, s!("Ansi"), s!("Caption"), MB_OK);
        MessageBoxW(0 as _, w!("Wide"), w!("Caption"), MB_OK);
    }
}
```
