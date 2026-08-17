// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::{
    ffi,
    utils::{self, CFReleaser},
};
use crate::{Disk, DiskKind};

use core_foundation_sys::array::CFArrayCreate;
use core_foundation_sys::base::kCFAllocatorDefault;
use core_foundation_sys::dictionary::{CFDictionaryGetValueIfPresent, CFDictionaryRef};
use core_foundation_sys::number::{kCFBooleanTrue, CFBooleanRef, CFNumberGetValue};
use core_foundation_sys::string::{self as cfs, CFStringRef};

use libc::c_void;

use std::ffi::{CStr, OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::ptr;

pub(crate) struct DiskInner {
    pub(crate) type_: DiskKind,
    pub(crate) name: OsString,
    pub(crate) file_system: OsString,
    pub(crate) mount_point: PathBuf,
    volume_url: RetainedCFURL,
    pub(crate) total_space: u64,
    pub(crate) available_space: u64,
    pub(crate) is_removable: bool,
}

impl DiskInner {
    pub(crate) fn kind(&self) -> DiskKind {
        self.type_
    }

    pub(crate) fn name(&self) -> &OsStr {
        &self.name
    }

    pub(crate) fn file_system(&self) -> &OsStr {
        &self.file_system
    }

    pub(crate) fn mount_point(&self) -> &Path {
        &self.mount_point
    }

    pub(crate) fn total_space(&self) -> u64 {
        self.total_space
    }

    pub(crate) fn available_space(&self) -> u64 {
        self.available_space
    }

    pub(crate) fn is_removable(&self) -> bool {
        self.is_removable
    }

    pub(crate) fn refresh(&mut self) -> bool {
        unsafe {
            if let Some(requested_properties) = build_requested_properties(&[
                ffi::kCFURLVolumeAvailableCapacityKey,
                ffi::kCFURLVolumeAvailableCapacityForImportantUsageKey,
            ]) {
                match get_disk_properties(&self.volume_url, &requested_properties) {
                    Some(disk_props) => {
                        self.available_space = get_available_volume_space(&disk_props);
                        true
                    }
                    None => {
                        sysinfo_debug!("Failed to get disk properties");
                        false
                    }
                }
            } else {
                sysinfo_debug!("failed to create volume key list, skipping refresh");
                false
            }
        }
    }
}

impl crate::DisksInner {
    pub(crate) fn new() -> Self {
        Self {
            disks: Vec::with_capacity(2),
        }
    }

    pub(crate) fn refresh_list(&mut self) {
        unsafe {
            // SAFETY: We don't keep any Objective-C objects around because we 
            // don't make any direct Objective-C calls in this code.
            with_autorelease(|| {
                get_list(&mut self.disks);
            })
        }
    }

    pub(crate) fn list(&self) -> &[Disk] {
        &self.disks
    }

    pub(crate) fn list_mut(&mut self) -> &mut [Disk] {
        &mut self.disks
    }
}

unsafe fn get_list(container: &mut Vec<Disk>) {
    container.clear();

    let raw_disks = {
        let count = libc::getfsstat(ptr::null_mut(), 0, libc::MNT_NOWAIT);
        if count < 1 {
            return;
        }
        let bufsize = count * std::mem::size_of::<libc::statfs>() as libc::c_int;
        let mut disks = Vec::with_capacity(count as _);
        let count = libc::getfsstat(disks.as_mut_ptr(), bufsize, libc::MNT_NOWAIT);

        if count < 1 {
            return;
        }

        disks.set_len(count as usize);

        disks
    };

    // Create a list of properties about the disk that we want to fetch.
    let requested_properties = match build_requested_properties(&[
        ffi::kCFURLVolumeIsEjectableKey,
        ffi::kCFURLVolumeIsRemovableKey,
        ffi::kCFURLVolumeIsInternalKey,
        ffi::kCFURLVolumeTotalCapacityKey,
        ffi::kCFURLVolumeAvailableCapacityForImportantUsageKey,
        ffi::kCFURLVolumeAvailableCapacityKey,
        ffi::kCFURLVolumeNameKey,
        ffi::kCFURLVolumeIsBrowsableKey,
        ffi::kCFURLVolumeIsLocalKey,
    ]) {
        Some(properties) => properties,
        None => {
            sysinfo_debug!("failed to create volume key list");
            return;
        }
    };

    for c_disk in raw_disks {
        let volume_url = match CFReleaser::new(
            core_foundation_sys::url::CFURLCreateFromFileSystemRepresentation(
                kCFAllocatorDefault,
                c_disk.f_mntonname.as_ptr() as *const _,
                c_disk.f_mntonname.len() as _,
                false as _,
            ),
        ) {
            Some(url) => url,
            None => {
                sysinfo_debug!("getfsstat returned incompatible paths");
                continue;
            }
        };

        let prop_dict = match get_disk_properties(&volume_url, &requested_properties) {
            Some(props) => props,
            None => continue,
        };

        // Future note: There is a difference between `kCFURLVolumeIsBrowsableKey` and the
        // `kCFURLEnumeratorSkipInvisibles` option of `CFURLEnumeratorOptions`. Specifically,
        // the first one considers the writable `Data`(`/System/Volumes/Data`) partition to be
        // browsable, while it is classified as "invisible" by CoreFoundation's volume emumerator.
        let browsable = get_bool_value(
            prop_dict.inner(),
            DictKey::Extern(ffi::kCFURLVolumeIsBrowsableKey),
        )
        .unwrap_or_default();

        // Do not return invisible "disks". Most of the time, these are APFS snapshots, hidden
        // system volumes, etc. Browsable is defined to be visible in the system's UI like Finder,
        // disk utility, system information, etc.
        //
        // To avoid seemingly duplicating many disks and creating an inaccurate view of the system's
        // resources, these are skipped entirely.
        if !browsable {
            continue;
        }

        let local_only = get_bool_value(
            prop_dict.inner(),
            DictKey::Extern(ffi::kCFURLVolumeIsLocalKey),
        )
        .unwrap_or(true);

        // Skip any drive that is not locally attached to the system.
        //
        // This includes items like SMB mounts, and matches the other platform's behavior.
        if !local_only {
            continue;
        }

        let mount_point = PathBuf::from(OsStr::from_bytes(
            CStr::from_ptr(c_disk.f_mntonname.as_ptr()).to_bytes(),
        ));

        if let Some(disk) = new_disk(mount_point, volume_url, c_disk, &prop_dict) {
            container.push(disk);
        }
    }
}

type RetainedCFArray = CFReleaser<core_foundation_sys::array::__CFArray>;
type RetainedCFDictionary = CFReleaser<core_foundation_sys::dictionary::__CFDictionary>;
type RetainedCFURL = CFReleaser<core_foundation_sys::url::__CFURL>;

unsafe fn build_requested_properties(properties: &[CFStringRef]) -> Option<RetainedCFArray> {
    CFReleaser::new(CFArrayCreate(
        ptr::null_mut(),
        properties.as_ptr() as *const *const c_void,
        properties.len() as _,
        &core_foundation_sys::array::kCFTypeArrayCallBacks,
    ))
}

fn get_disk_properties(
    volume_url: &RetainedCFURL,
    requested_properties: &RetainedCFArray,
) -> Option<RetainedCFDictionary> {
    CFReleaser::new(unsafe {
        ffi::CFURLCopyResourcePropertiesForKeys(
            volume_url.inner(),
            requested_properties.inner(),
            ptr::null_mut(),
        )
    })
}

fn get_available_volume_space(disk_props: &RetainedCFDictionary) -> u64 {
    // We prefer `AvailableCapacityForImportantUsage` over `AvailableCapacity` because
    // it takes more of the system's properties into account, like the trash, system-managed caches,
    // etc. It generally also returns higher values too, because of the above, so it's a more
    // accurate representation of what the system _could_ still use.
    unsafe {
        get_int_value(
            disk_props.inner(),
            DictKey::Extern(ffi::kCFURLVolumeAvailableCapacityForImportantUsageKey),
        )
        .filter(|bytes| *bytes != 0)
        .or_else(|| {
            get_int_value(
                disk_props.inner(),
                DictKey::Extern(ffi::kCFURLVolumeAvailableCapacityKey),
            )
        })
    }
    .unwrap_or_default() as u64
}

pub(super) enum DictKey {
    Extern(CFStringRef),
    #[cfg(target_os = "macos")]
    Defined(&'static str),
}

unsafe fn get_dict_value<T, F: FnOnce(*const c_void) -> Option<T>>(
    dict: CFDictionaryRef,
    key: DictKey,
    callback: F,
) -> Option<T> {
    #[cfg(target_os = "macos")]
    let _defined;
    #[allow(clippy::infallible_destructuring_match)]
    let key = match key {
        DictKey::Extern(val) => val,
        #[cfg(target_os = "macos")]
        DictKey::Defined(val) => {
            _defined = CFReleaser::new(cfs::CFStringCreateWithBytesNoCopy(
                kCFAllocatorDefault,
                val.as_ptr(),
                val.len() as _,
                cfs::kCFStringEncodingUTF8,
                false as _,
                core_foundation_sys::base::kCFAllocatorNull,
            ))?;

            _defined.inner()
        }
    };

    let mut value = std::ptr::null();
    if CFDictionaryGetValueIfPresent(dict, key.cast(), &mut value) != 0 {
        callback(value)
    } else {
        None
    }
}

pub(super) unsafe fn get_str_value(dict: CFDictionaryRef, key: DictKey) -> Option<String> {
    get_dict_value(dict, key, |v| {
        let v = v as cfs::CFStringRef;

        let len_utf16 = cfs::CFStringGetLength(v) as usize;
        let len_bytes = len_utf16 * 2; // Two bytes per UTF-16 codepoint.

        let v_ptr = cfs::CFStringGetCStringPtr(v, cfs::kCFStringEncodingUTF8);
        if v_ptr.is_null() {
            // Fallback on CFStringGetString to read the underlying bytes from the CFString.
            let mut buf = vec![0; len_bytes];
            let success = cfs::CFStringGetCString(
                v,
                buf.as_mut_ptr(),
                len_bytes as _,
                cfs::kCFStringEncodingUTF8,
            );

            if success != 0 {
                utils::vec_to_rust(buf)
            } else {
                None
            }
        } else {
            crate::unix::utils::cstr_to_rust_with_size(v_ptr, Some(len_bytes))
        }
    })
}

unsafe fn get_bool_value(dict: CFDictionaryRef, key: DictKey) -> Option<bool> {
    get_dict_value(dict, key, |v| Some(v as CFBooleanRef == kCFBooleanTrue))
}

unsafe fn get_int_value(dict: CFDictionaryRef, key: DictKey) -> Option<i64> {
    get_dict_value(dict, key, |v| {
        let mut val: i64 = 0;
        if CFNumberGetValue(
            v.cast(),
            core_foundation_sys::number::kCFNumberSInt64Type,
            &mut val as *mut i64 as *mut c_void,
        ) {
            Some(val)
        } else {
            None
        }
    })
}

unsafe fn new_disk(
    mount_point: PathBuf,
    volume_url: RetainedCFURL,
    c_disk: libc::statfs,
    disk_props: &RetainedCFDictionary,
) -> Option<Disk> {
    // IOKit is not available on any but the most recent (16+) iOS and iPadOS versions.
    // Due to this, we can't query the medium type. All iOS devices use flash-based storage
    // so we just assume the disk type is an SSD until Rust has a way to conditionally link to
    // IOKit in more recent deployment versions.
    #[cfg(target_os = "macos")]
    let type_ = crate::sys::inner::disk::get_disk_type(&c_disk).unwrap_or(DiskKind::Unknown(-1));
    #[cfg(not(target_os = "macos"))]
    let type_ = DiskKind::SSD;

    // Note: Since we requested these properties from the system, we don't expect
    // these property retrievals to fail.

    let name = get_str_value(
        disk_props.inner(),
        DictKey::Extern(ffi::kCFURLVolumeNameKey),
    )
    .map(OsString::from)?;

    let is_removable = {
        let ejectable = get_bool_value(
            disk_props.inner(),
            DictKey::Extern(ffi::kCFURLVolumeIsEjectableKey),
        )
        .unwrap_or_default();

        let removable = get_bool_value(
            disk_props.inner(),
            DictKey::Extern(ffi::kCFURLVolumeIsRemovableKey),
        )
        .unwrap_or_default();

        let is_removable = ejectable || removable;

        if is_removable {
            is_removable
        } else {
            // If neither `ejectable` or `removable` return `true`, fallback to checking
            // if the disk is attached to the internal system.
            let internal = get_bool_value(
                disk_props.inner(),
                DictKey::Extern(ffi::kCFURLVolumeIsInternalKey),
            )
            .unwrap_or_default();

            !internal
        }
    };

    let total_space = get_int_value(
        disk_props.inner(),
        DictKey::Extern(ffi::kCFURLVolumeTotalCapacityKey),
    )? as u64;

    let available_space = get_available_volume_space(disk_props);

    let file_system = {
        let len = c_disk
            .f_fstypename
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(c_disk.f_fstypename.len());
        OsString::from_vec(
            c_disk.f_fstypename[..len]
                .iter()
                .map(|c| *c as u8)
                .collect(),
        )
    };

    Some(Disk {
        inner: DiskInner {
            type_,
            name,
            file_system,
            mount_point,
            volume_url,
            total_space,
            available_space,
            is_removable,
        },
    })
}


/// Calls the provided closure in the context of a new autorelease pool that is drained
/// before returning.
/// 
/// ## SAFETY:
/// You must not return an Objective-C object that is autoreleased from this function since it
/// will be freed before usable.
unsafe fn with_autorelease<T, F: FnOnce() -> T>(call: F) -> T {
    // NB: This struct exists to help prevent memory leaking if `call` were to panic.
    // Otherwise, the call to `objc_autoreleasePoolPop` would never be made as the stack unwinds.
    // `Drop` destructors for existing types on the stack are run during unwinding, so we can 
    // ensure the autorelease pool is drained by using a RAII pattern here.
    struct DrainPool {
        ctx: *mut c_void,
    }

    impl Drop for DrainPool {
        fn drop(&mut self) {
            // SAFETY: We have not manipulated `pool_ctx` since it was received from a corresponding
            // pool push call.
            unsafe { ffi::objc_autoreleasePoolPop(self.ctx) }
        }
    }

    // SAFETY: Creating a new pool is safe in any context. They can be arbitrarily nested
    // as long as pool objects are not used in deeper layers, but we only have one and don't
    // allow it to leave this scope.
    let _pool_ctx = DrainPool { ctx: unsafe { ffi::objc_autoreleasePoolPush() } };
    call()
    // Pool is drained here before returning
}
