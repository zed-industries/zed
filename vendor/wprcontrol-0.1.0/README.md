# wprcontrol-rs

Rust bindings for the Windows Performance Recorder (WPR) Control API (`WindowsPerformanceRecorderControl.h`) from the Windows Performance Toolkit.

[WPRControl API Reference](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/wprcontrol-api-reference)

### Example: Start and stop a WPR recording

```rust
use windows::Win32::Foundation::VARIANT_FALSE;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoInitializeEx};
use windows_core::{BSTR, Interface};
use wprcontrol::*;

fn create_wpr<T: Interface>(clsid: &windows_core::GUID) -> windows_core::Result<T> {
    unsafe {
        WPRCCreateInstanceUnderInstanceName::<_, T>(
            &BSTR::new(),
            clsid,
            None,
            CLSCTX_INPROC_SERVER.0 as u32,
        )
    }
}

fn main() -> windows_core::Result<()> {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok()? };

    // Create WPR objects
    let collection: IProfileCollection = create_wpr(&CProfileCollection)?;
    let control: IControlManager = create_wpr(&CControlManager)?;

    // Load a built-in profile
    let profile: IProfile = create_wpr(&CProfile)?;
    unsafe {
        profile.LoadFromFile(&BSTR::from("CPU.Verbose.File"), &BSTR::new())?;
        collection.Add(&profile, VARIANT_FALSE)?;
    }

    // Start recording
    unsafe { control.Start(&collection)? };

    // ... do work ...

    // Stop and save the trace
    unsafe {
        control.Stop(&BSTR::from("trace.etl"), &collection, None)?;
    }

    Ok(())
}
```
