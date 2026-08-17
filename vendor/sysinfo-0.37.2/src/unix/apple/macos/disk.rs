// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DiskKind;
use crate::sys::{
    disk::{get_int_value, get_str_value},
    macos::{ffi, utils::IOReleaser},
};

use objc2_core_foundation::{CFDictionary, CFRetained, CFString, kCFAllocatorDefault};
use objc2_io_kit::{
    IOBSDNameMatching, IOIteratorNext, IOObjectConformsTo, IORegistryEntryCreateCFProperty,
    IORegistryEntryGetParentEntry, IOServiceGetMatchingServices, io_iterator_t,
    io_registry_entry_t, kIOMasterPortDefault, kIOServicePlane,
};

fn iterate_service_tree<T, F>(bsd_name: &[u8], key: &CFString, eval: F) -> Option<T>
where
    F: Fn(io_registry_entry_t, &CFDictionary) -> Option<T>,
{
    let matching = unsafe { IOBSDNameMatching(kIOMasterPortDefault, 0, bsd_name.as_ptr().cast()) }?;
    let matching = CFRetained::<CFDictionary>::from(&matching);

    let mut service_iterator: io_iterator_t = 0;

    if unsafe {
        IOServiceGetMatchingServices(kIOMasterPortDefault, Some(matching), &mut service_iterator)
    } != libc::KERN_SUCCESS
    {
        return None;
    }

    // Safety: We checked for success, so there is always a valid iterator, even if its empty.
    let service_iterator = unsafe { IOReleaser::new_unchecked(service_iterator) };

    let mut parent_entry: io_registry_entry_t = 0;

    while let Some(mut current_service_entry) =
        IOReleaser::new(unsafe { IOIteratorNext(service_iterator.inner()) })
    {
        // Note: This loop is required in a non-obvious way. Due to device properties existing as a tree
        // in IOKit, we may need an arbitrary number of calls to `IORegistryEntryCreateCFProperty` in order to find
        // the values we are looking for. The function may return nothing if we aren't deep enough into the registry
        // tree, so we need to continue going from child->parent node until its found.
        loop {
            if unsafe {
                IORegistryEntryGetParentEntry(
                    current_service_entry.inner(),
                    kIOServicePlane.as_ptr().cast_mut().cast(),
                    &mut parent_entry,
                )
            } != libc::KERN_SUCCESS
            {
                break;
            }

            current_service_entry = match IOReleaser::new(parent_entry) {
                Some(service) => service,
                // There were no more parents left
                None => break,
            };

            let properties_result = unsafe {
                IORegistryEntryCreateCFProperty(
                    current_service_entry.inner(),
                    Some(key),
                    kCFAllocatorDefault,
                    0,
                )
            };

            if let Some(properties) = properties_result
                && let Ok(properties) = properties.downcast::<CFDictionary>()
                && let Some(result) = eval(parent_entry, &properties)
            {
                return Some(result);
            }
        }
    }

    None
}

pub(crate) fn get_disk_type(bsd_name: &[u8]) -> Option<DiskKind> {
    let characteristics_string =
        CFString::from_static_str(ffi::kIOPropertyDeviceCharacteristicsKey);

    iterate_service_tree(bsd_name, &characteristics_string, |_, properties| {
        let medium = unsafe {
            super::disk::get_str_value(
                properties,
                Some(&CFString::from_static_str(ffi::kIOPropertyMediumTypeKey)),
            )
        }?;

        match medium.as_str() {
            _ if medium == ffi::kIOPropertyMediumTypeSolidStateKey => Some(DiskKind::SSD),
            _ if medium == ffi::kIOPropertyMediumTypeRotationalKey => Some(DiskKind::HDD),
            _ => Some(DiskKind::Unknown(-1)),
        }
    })
}

/// Returns a tuple consisting of the total number of bytes read and written by the specified disk
pub(crate) fn get_disk_io(bsd_name: &[u8]) -> Option<(u64, u64)> {
    let stat_string = CFString::from_static_str(ffi::kIOBlockStorageDriverStatisticsKey);

    iterate_service_tree(bsd_name, &stat_string, |parent_entry, properties| {
        if !unsafe { IOObjectConformsTo(parent_entry, c"IOBlockStorageDriver".as_ptr() as *mut _) }
        {
            return None;
        }

        unsafe {
            let read_bytes = super::disk::get_int_value(
                properties,
                Some(&CFString::from_static_str(
                    ffi::kIOBlockStorageDriverStatisticsBytesReadKey,
                )),
            )?;
            let written_bytes = super::disk::get_int_value(
                properties,
                Some(&CFString::from_static_str(
                    ffi::kIOBlockStorageDriverStatisticsBytesWrittenKey,
                )),
            )?;

            Some((read_bytes, written_bytes))
        }
    })
}
