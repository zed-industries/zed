//! Heavily copied from:
//! <https://github.com/rust-lang/rust/pull/138944>
//!
//! Once in MSRV, we should be able to replace this with just using that
//! symbol.
use core::ffi::{c_char, c_int, c_uint, c_void, CStr};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU32, Ordering};
use std::env;
use std::ffi::CString;
use std::fs;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use super::OSVersion;

/// The deployment target for the current OS.
pub(crate) const DEPLOYMENT_TARGET: OSVersion = {
    // Intentionally use `#[cfg]` guards instead of `cfg!` here, to avoid
    // recompiling when unrelated environment variables change.
    #[cfg(target_os = "macos")]
    let var = option_env!("MACOSX_DEPLOYMENT_TARGET");
    #[cfg(target_os = "ios")] // Also used on Mac Catalyst.
    let var = option_env!("IPHONEOS_DEPLOYMENT_TARGET");
    #[cfg(target_os = "tvos")]
    let var = option_env!("TVOS_DEPLOYMENT_TARGET");
    #[cfg(target_os = "watchos")]
    let var = option_env!("WATCHOS_DEPLOYMENT_TARGET");
    #[cfg(target_os = "visionos")]
    let var = option_env!("XROS_DEPLOYMENT_TARGET");

    if let Some(var) = var {
        OSVersion::from_str(var)
    } else {
        // Default operating system version.
        // See <https://github.com/rust-lang/rust/blob/1e5719bdc40bb553089ce83525f07dfe0b2e71e9/compiler/rustc_target/src/spec/base/apple/mod.rs#L207-L215>
        //
        // Note that we cannot do as they suggest, and use
        // `rustc --print=deployment-target`, as this has to work at `const`
        // time.
        #[allow(clippy::if_same_then_else)]
        let os_min = if cfg!(target_os = "macos") {
            (10, 12, 0)
        } else if cfg!(target_os = "ios") {
            (10, 0, 0)
        } else if cfg!(target_os = "tvos") {
            (10, 0, 0)
        } else if cfg!(target_os = "watchos") {
            (5, 0, 0)
        } else if cfg!(target_os = "visionos") {
            (1, 0, 0)
        } else {
            panic!("unknown Apple OS")
        };

        // On certain targets it makes sense to raise the minimum OS version.
        //
        // See <https://github.com/rust-lang/rust/blob/1e5719bdc40bb553089ce83525f07dfe0b2e71e9/compiler/rustc_target/src/spec/base/apple/mod.rs#L217-L231>
        //
        // Note that we cannot do all the same checks as `rustc` does, because
        // we have no way of knowing if the architecture is `arm64e` without
        // reading the target triple itself (and we want to get rid of build
        // scripts).
        #[allow(clippy::if_same_then_else)]
        let min = if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            (11, 0, 0)
        } else if cfg!(all(
            target_os = "ios",
            target_arch = "aarch64",
            target_abi_macabi
        )) {
            (14, 0, 0)
        } else if cfg!(all(
            target_os = "ios",
            target_arch = "aarch64",
            target_simulator
        )) {
            (14, 0, 0)
        } else if cfg!(all(target_os = "tvos", target_arch = "aarch64")) {
            (14, 0, 0)
        } else if cfg!(all(target_os = "watchos", target_arch = "aarch64")) {
            (7, 0, 0)
        } else {
            os_min
        };

        OSVersion {
            major: min.0,
            minor: min.1,
            patch: min.2,
        }
    }
};

/// Get the current OS version.
///
/// # Semantics
///
/// The reported version on macOS might be 10.16 if the SDK version of the binary is less than 11.0.
/// This is a workaround that Apple implemented to handle applications that assumed that macOS
/// versions would always start with "10", see:
/// <https://github.com/apple-oss-distributions/xnu/blob/xnu-11215.81.4/libsyscall/wrappers/system-version-compat.c>
///
/// It _is_ possible to get the real version regardless of the SDK version of the binary, this is
/// what Zig does:
/// <https://github.com/ziglang/zig/blob/0.13.0/lib/std/zig/system/darwin/macos.zig>
///
/// We choose to not do that, and instead follow Apple's behaviour here, and return 10.16 when
/// compiled with an older SDK; the user should instead upgrade their tooling.
///
/// NOTE: `rustc` currently doesn't set the right SDK version when linking with ld64, so this will
/// have the wrong behaviour with `-Clinker=ld` on x86_64. But that's a `rustc` bug:
/// <https://github.com/rust-lang/rust/issues/129432>
#[inline]
pub(crate) fn current_version() -> OSVersion {
    // Cache the lookup for performance.
    //
    // 0.0.0 is never going to be a valid version ("vtool" reports "n/a" on 0 versions), so we use
    // that as our sentinel value.
    static CURRENT_VERSION: AtomicU32 = AtomicU32::new(0);

    // We use relaxed atomics instead of e.g. a `Once`, it doesn't matter if multiple threads end up
    // racing to read or write the version, `lookup_version` should be idempotent and always return
    // the same value.
    //
    // `compiler-rt` uses `dispatch_once`, but that's overkill for the reasons above.
    let version = CURRENT_VERSION.load(Ordering::Relaxed);
    OSVersion::from_u32(if version == 0 {
        let version = lookup_version();
        CURRENT_VERSION.store(version, Ordering::Relaxed);
        version
    } else {
        version
    })
}

/// Look up the os version.
///
/// # Aborts
///
/// Aborts if reading or parsing the version fails (or if the system was out of memory).
///
/// We deliberately choose to abort, as having this silently return an invalid OS version would be
/// impossible for a user to debug.
// The lookup is costly and should be on the cold path because of the cache in `current_version`.
#[cold]
// Micro-optimization: We use `extern "C"` to abort on panic, allowing `current_version` (inlined)
// to be free of unwind handling.
extern "C" fn lookup_version() -> u32 {
    // Try to read from `sysctl` first (faster), but if that fails, fall back to reading the
    // property list (this is roughly what `_availability_version_check` does internally).
    let version = version_from_sysctl().unwrap_or_else(version_from_plist);

    // Try to make it clearer to the optimizer that this will never return 0.
    assert_ne!(version, OSVersion::MIN, "version cannot be 0.0.0");
    version.to_u32()
}

/// Read the version from `kern.osproductversion` or `kern.iossupportversion`.
///
/// This is faster than `version_from_plist`, since it doesn't need to invoke `dlsym`.
fn version_from_sysctl() -> Option<OSVersion> {
    // This won't work in the simulator, as `kern.osproductversion` returns the host macOS version,
    // and `kern.iossupportversion` returns the host macOS' iOSSupportVersion (while you can run
    // simulators with many different iOS versions).
    if cfg!(target_simulator) {
        // Fall back to `version_from_plist` on these targets.
        return None;
    }

    // SAFETY: Same signatures as in `libc`.
    //
    // NOTE: We do not need to link this, that will be done by `std` by linking `libSystem`
    // (which is required on macOS/Darwin).
    extern "C" {
        fn sysctlbyname(
            name: *const c_char,
            oldp: *mut c_void,
            oldlenp: *mut usize,
            newp: *mut c_void,
            newlen: usize,
        ) -> c_uint;
    }

    let sysctl_version = |name: &[u8]| {
        let mut buf: [u8; 32] = [0; 32];
        let mut size = buf.len();
        let ptr = buf.as_mut_ptr().cast();
        let ret = unsafe { sysctlbyname(name.as_ptr().cast(), ptr, &mut size, null_mut(), 0) };
        if ret != 0 {
            // This sysctl is not available.
            return None;
        }
        let buf = &buf[..(size - 1)];

        if buf.is_empty() {
            // The buffer may be empty when using `kern.iossupportversion` on an actual iOS device,
            // or on visionOS when running under "Designed for iPad".
            //
            // In that case, fall back to `kern.osproductversion`.
            return None;
        }

        Some(OSVersion::from_bytes(buf))
    };

    // When `target_os = "ios"`, we may be in many different states:
    // - Native iOS device.
    // - iOS Simulator.
    // - Mac Catalyst.
    // - Mac + "Designed for iPad".
    // - Native visionOS device + "Designed for iPad".
    // - visionOS simulator + "Designed for iPad".
    //
    // Of these, only native, Mac Catalyst and simulators can be differentiated at compile-time
    // (with `target_abi = ""`, `target_abi = "macabi"` and `target_abi = "sim"` respectively).
    //
    // That is, "Designed for iPad" will act as iOS at compile-time, but the `ProductVersion` will
    // still be the host macOS or visionOS version.
    //
    // Furthermore, we can't even reliably differentiate between these at runtime, since
    // `dyld_get_active_platform` isn't publicly available.
    //
    // Fortunately, we won't need to know any of that; we can simply attempt to get the
    // `iOSSupportVersion` (which may be set on native iOS too, but then it will be set to the host
    // iOS version), and if that fails, fall back to the `ProductVersion`.
    if cfg!(target_os = "ios") {
        // https://github.com/apple-oss-distributions/xnu/blob/xnu-11215.81.4/bsd/kern/kern_sysctl.c#L2077-L2100
        if let Some(ios_support_version) = sysctl_version(b"kern.iossupportversion\0") {
            return Some(ios_support_version);
        }

        // On Mac Catalyst, if we failed looking up `iOSSupportVersion`, we don't want to
        // accidentally fall back to `ProductVersion`.
        if cfg!(target_abi_macabi) {
            return None;
        }
    }

    // Introduced in macOS 10.13.4.
    // https://github.com/apple-oss-distributions/xnu/blob/xnu-11215.81.4/bsd/kern/kern_sysctl.c#L2015-L2051
    sysctl_version(b"kern.osproductversion\0")
}

/// Look up the current OS version(s) from `/System/Library/CoreServices/SystemVersion.plist`.
///
/// More specifically, from the `ProductVersion` and `iOSSupportVersion` keys, and from
/// `$IPHONE_SIMULATOR_ROOT/System/Library/CoreServices/SystemVersion.plist` on the simulator.
///
/// This file was introduced in macOS 10.3, which is well below the minimum supported version by
/// `rustc`, which is (at the time of writing) macOS 10.12.
///
/// # Implementation
///
/// We do roughly the same thing in here as `compiler-rt`, and dynamically look up CoreFoundation
/// utilities for parsing PLists (to avoid having to re-implement that in here, as pulling in a full
/// PList parser into `std` seems costly).
///
/// If this is found to be undesirable, we _could_ possibly hack it by parsing the PList manually
/// (it seems to use the plain-text "xml1" encoding/format in all versions), but that seems brittle.
fn version_from_plist() -> OSVersion {
    // The root directory relative to where all files are located.
    let root = if cfg!(target_simulator) {
        PathBuf::from(env::var_os("IPHONE_SIMULATOR_ROOT").expect(
            "environment variable `IPHONE_SIMULATOR_ROOT` must be set when executing under simulator",
        ))
    } else {
        PathBuf::from("/")
    };

    // Read `SystemVersion.plist`. Always present on Apple platforms, reading it cannot fail.
    let path = root.join("System/Library/CoreServices/SystemVersion.plist");
    let plist_buffer = fs::read(&path).unwrap_or_else(|e| panic!("failed reading {path:?}: {e}"));
    parse_version_from_plist(&root, &plist_buffer)
}

/// Split out from [`version_from_plist`] to allow for testing.
#[allow(non_upper_case_globals, non_snake_case)]
fn parse_version_from_plist(root: &Path, plist_buffer: &[u8]) -> OSVersion {
    const RTLD_LAZY: c_int = 0x1;
    const RTLD_LOCAL: c_int = 0x4;

    extern "C" {
        fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        fn dlerror() -> *mut c_char;
        fn dlclose(handle: *mut c_void) -> c_int;
    }

    // Link to the CoreFoundation dylib, and look up symbols from that.
    // We explicitly use non-versioned path here, to allow this to work on older iOS devices.
    let cf_path = root.join("System/Library/Frameworks/CoreFoundation.framework/CoreFoundation");

    let cf_path =
        CString::new(cf_path.into_os_string().into_vec()).expect("failed allocating string");
    let cf_handle = unsafe { dlopen(cf_path.as_ptr(), RTLD_LAZY | RTLD_LOCAL) };
    if cf_handle.is_null() {
        let err = unsafe { CStr::from_ptr(dlerror()) };
        panic!("could not open CoreFoundation.framework: {err:?}");
    }
    let _cf_handle_free = Deferred(|| {
        // Ignore errors when closing. This is also what `libloading` does:
        // https://docs.rs/libloading/0.8.6/src/libloading/os/unix/mod.rs.html#374
        let _ = unsafe { dlclose(cf_handle) };
    });

    macro_rules! dlsym {
        (
            unsafe fn $name:ident($($param:ident: $param_ty:ty),* $(,)?) $(-> $ret:ty)?;
        ) => {{
            let ptr = unsafe {
                dlsym(
                    cf_handle,
                    concat!(stringify!($name), '\0').as_bytes().as_ptr().cast(),
                )
            };
            if ptr.is_null() {
                let err = unsafe { CStr::from_ptr(dlerror()) };
                panic!("could not find function {}: {err:?}", stringify!($name));
            }
            // SAFETY: Just checked that the symbol isn't NULL, and caller verifies that the
            // signature is correct.
            unsafe {
                core::mem::transmute::<
                    *mut c_void,
                    unsafe extern "C" fn($($param_ty),*) $(-> $ret)?,
                >(ptr)
            }
        }};
    }

    // MacTypes.h
    type Boolean = u8;
    // CoreFoundation/CFBase.h
    type CFTypeID = usize;
    type CFOptionFlags = usize;
    type CFIndex = isize;
    type CFTypeRef = *mut c_void;
    type CFAllocatorRef = CFTypeRef;
    const kCFAllocatorDefault: CFAllocatorRef = null_mut();
    // Available: in all CF versions.
    let allocator_null = unsafe { dlsym(cf_handle, b"kCFAllocatorNull\0".as_ptr().cast()) };
    if allocator_null.is_null() {
        let err = unsafe { CStr::from_ptr(dlerror()) };
        panic!("could not find kCFAllocatorNull: {err:?}");
    }
    let kCFAllocatorNull = unsafe { *allocator_null.cast::<CFAllocatorRef>() };
    let CFRelease = dlsym!(
        // Available: in all CF versions.
        unsafe fn CFRelease(cf: CFTypeRef);
    );
    let CFGetTypeID = dlsym!(
        // Available: in all CF versions.
        unsafe fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    );
    // CoreFoundation/CFError.h
    type CFErrorRef = CFTypeRef;
    // CoreFoundation/CFData.h
    type CFDataRef = CFTypeRef;
    let CFDataCreateWithBytesNoCopy = dlsym!(
        // Available: in all CF versions.
        unsafe fn CFDataCreateWithBytesNoCopy(
            allocator: CFAllocatorRef,
            bytes: *const u8,
            length: CFIndex,
            bytes_deallocator: CFAllocatorRef,
        ) -> CFDataRef;
    );
    // CoreFoundation/CFPropertyList.h
    const kCFPropertyListImmutable: CFOptionFlags = 0;
    type CFPropertyListFormat = CFIndex;
    type CFPropertyListRef = CFTypeRef;
    let CFPropertyListCreateWithData = dlsym!(
        // Available: since macOS 10.6.
        unsafe fn CFPropertyListCreateWithData(
            allocator: CFAllocatorRef,
            data: CFDataRef,
            options: CFOptionFlags,
            format: *mut CFPropertyListFormat,
            error: *mut CFErrorRef,
        ) -> CFPropertyListRef;
    );
    // CoreFoundation/CFString.h
    type CFStringRef = CFTypeRef;
    type CFStringEncoding = u32;
    const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
    let CFStringGetTypeID = dlsym!(
        // Available: in all CF versions.
        unsafe fn CFStringGetTypeID() -> CFTypeID;
    );
    let CFStringCreateWithCStringNoCopy = dlsym!(
        // Available: in all CF versions.
        unsafe fn CFStringCreateWithCStringNoCopy(
            alloc: CFAllocatorRef,
            c_str: *const c_char,
            encoding: CFStringEncoding,
            contents_deallocator: CFAllocatorRef,
        ) -> CFStringRef;
    );
    let CFStringGetCString = dlsym!(
        // Available: in all CF versions.
        unsafe fn CFStringGetCString(
            the_string: CFStringRef,
            buffer: *mut c_char,
            buffer_size: CFIndex,
            encoding: CFStringEncoding,
        ) -> Boolean;
    );
    // CoreFoundation/CFDictionary.h
    type CFDictionaryRef = CFTypeRef;
    let CFDictionaryGetTypeID = dlsym!(
        // Available: in all CF versions.
        unsafe fn CFDictionaryGetTypeID() -> CFTypeID;
    );
    let CFDictionaryGetValue = dlsym!(
        // Available: in all CF versions.
        unsafe fn CFDictionaryGetValue(
            the_dict: CFDictionaryRef,
            key: *const c_void,
        ) -> *const c_void;
    );

    // MARK: Done declaring symbols.

    let plist_data = unsafe {
        CFDataCreateWithBytesNoCopy(
            kCFAllocatorDefault,
            plist_buffer.as_ptr(),
            plist_buffer.len() as CFIndex,
            kCFAllocatorNull,
        )
    };
    assert!(!plist_data.is_null(), "failed creating data");
    let _plist_data_release = Deferred(|| unsafe { CFRelease(plist_data) });

    let plist = unsafe {
        CFPropertyListCreateWithData(
            kCFAllocatorDefault,
            plist_data,
            kCFPropertyListImmutable,
            null_mut(), // Don't care about the format of the PList.
            null_mut(), // Don't care about the error data.
        )
    };
    assert!(
        !plist.is_null(),
        "failed reading PList in SystemVersion.plist"
    );
    let _plist_release = Deferred(|| unsafe { CFRelease(plist) });

    assert_eq!(
        unsafe { CFGetTypeID(plist) },
        unsafe { CFDictionaryGetTypeID() },
        "SystemVersion.plist did not contain a dictionary at the top level"
    );
    let plist = plist as CFDictionaryRef;

    let get_string_key = |plist, lookup_key: &[u8]| {
        let cf_lookup_key = unsafe {
            CFStringCreateWithCStringNoCopy(
                kCFAllocatorDefault,
                lookup_key.as_ptr().cast(),
                kCFStringEncodingUTF8,
                kCFAllocatorNull,
            )
        };
        assert!(!cf_lookup_key.is_null(), "failed creating CFString");
        let _lookup_key_release = Deferred(|| unsafe { CFRelease(cf_lookup_key) });

        let value = unsafe { CFDictionaryGetValue(plist, cf_lookup_key) as CFTypeRef };
        // ^ getter, so don't release.
        if value.is_null() {
            return None;
        }

        assert_eq!(
            unsafe { CFGetTypeID(value) },
            unsafe { CFStringGetTypeID() },
            "key in SystemVersion.plist must be a string"
        );
        let value = value as CFStringRef;

        let mut version_str = [0u8; 32];
        let ret = unsafe {
            CFStringGetCString(
                value,
                version_str.as_mut_ptr().cast::<c_char>(),
                version_str.len() as CFIndex,
                kCFStringEncodingUTF8,
            )
        };
        assert_ne!(ret, 0, "failed getting string from CFString");

        let version_str =
            CStr::from_bytes_until_nul(&version_str).expect("failed converting to CStr");

        Some(OSVersion::from_bytes(version_str.to_bytes()))
    };

    // Same logic as in `version_from_sysctl`.
    if cfg!(target_os = "ios") {
        if let Some(ios_support_version) = get_string_key(plist, b"iOSSupportVersion\0") {
            return ios_support_version;
        }

        // Force Mac Catalyst to use iOSSupportVersion (do not fall back to ProductVersion).
        if cfg!(target_abi_macabi) {
            panic!("expected iOSSupportVersion in SystemVersion.plist");
        }
    }

    // On all other platforms, we can find the OS version by simply looking at `ProductVersion`.
    get_string_key(plist, b"ProductVersion\0")
        .expect("expected ProductVersion in SystemVersion.plist")
}

struct Deferred<F: FnMut()>(F);

impl<F: FnMut()> Drop for Deferred<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::String;
    use std::process::Command;

    #[test]
    fn sysctl_same_as_in_plist() {
        if let Some(version) = version_from_sysctl() {
            assert_eq!(version, version_from_plist());
        }
    }

    #[test]
    fn read_version() {
        assert!(OSVersion::MIN < current_version(), "version cannot be min");
        assert!(current_version() < OSVersion::MAX, "version cannot be max");
    }

    #[test]
    #[cfg_attr(
        not(target_os = "macos"),
        ignore = "`sw_vers` is only available on macOS"
    )]
    fn compare_against_sw_vers() {
        let expected = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .unwrap()
            .stdout;
        let expected = String::from_utf8(expected).unwrap();
        let expected = OSVersion::from_str(expected.trim());

        let actual = current_version();
        assert_eq!(expected, actual);
    }
}
