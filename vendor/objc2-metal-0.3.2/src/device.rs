use crate::MTLDevice;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSArray;

/// Returns all Metal devices in the system.
///
/// On macOS and macCatalyst, this API will not cause the system to switch
/// devices and leaves the decision about which GPU to use up to the
/// application based on whatever criteria it deems appropriate.
///
/// On iOS, tvOS and visionOS, this API returns an array containing the same
/// device that MTLCreateSystemDefaultDevice would have returned, or an empty
/// array if it would have failed.
#[inline]
#[allow(unexpected_cfgs)]
pub extern "C-unwind" fn MTLCopyAllDevices() -> Retained<NSArray<ProtocolObject<dyn MTLDevice>>> {
    // MTLCopyAllDevices is always available on macOS and Mac Catalyst, but
    // only available recently on iOS 18.0 / tvOS 18.0 / visionOS 2.0.
    //
    // Instead, we do the fallback to MTLCreateSystemDefaultDevice on those
    // platforms that they do themselves on newer systems.
    //
    // TODO: Use something like <https://github.com/rust-lang/rfcs/pull/3750>
    // to call the actual API when available.
    #[cfg(any(target_os = "macos", target_env = "macabi"))]
    {
        extern "C-unwind" {
            fn MTLCopyAllDevices() -> *mut NSArray<ProtocolObject<dyn MTLDevice>>;
        }

        let ret = unsafe { MTLCopyAllDevices() };
        // SAFETY: Marked NS_RETURNS_RETAINED (and has `Copy` in the name).
        unsafe { Retained::from_raw(ret) }
            .expect("function was marked as returning non-null, but actually returned NULL")
    }
    #[cfg(not(any(target_os = "macos", target_env = "macabi")))]
    {
        let device = crate::MTLCreateSystemDefaultDevice();
        let slice: &[_] = if let Some(device) = device.as_deref() {
            &[device]
        } else {
            &[]
        };
        NSArray::from_slice(slice)
    }
}
