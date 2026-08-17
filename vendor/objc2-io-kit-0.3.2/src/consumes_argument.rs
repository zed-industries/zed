#![allow(non_snake_case, clippy::missing_safety_doc)]
use core::{ffi::c_void, ptr};
use objc2_core_foundation::{CFDictionary, CFRetained};

use crate::{
    io_iterator_t, io_name_t, io_service_t, IONotificationPortRef, IOServiceMatchingCallback,
};

fn consume(matching: Option<CFRetained<CFDictionary>>) -> *mut CFDictionary {
    if let Some(matching) = matching {
        CFRetained::into_raw(matching).as_ptr()
    } else {
        ptr::null_mut()
    }
}

/// Look up a registered IOService object that matches a matching dictionary.
///
/// This is the preferred method of finding IOService objects currently registered by IOKit (that is, objects that have had their registerService() methods invoked). To find IOService objects that aren't yet registered, use an iterator as created by IORegistryEntryCreateIterator(). IOServiceAddMatchingNotification can also supply this information and install a notification of new IOServices. The matching information used in the matching dictionary may vary depending on the class of service being looked up.
///
/// Parameter `mainPort`: The main port obtained from IOMainPort(). Pass kIOMainPortDefault to look up the default main port.
///
/// Parameter `matching`: A CF dictionary containing matching information, of which one reference is always consumed by this function (Note prior to the Tiger release there was a small chance that the dictionary might not be released if there was an error attempting to serialize the dictionary). IOKitLib can construct matching dictionaries for common criteria with helper functions such as IOServiceMatching, IOServiceNameMatching, IOBSDNameMatching.
///
/// Returns: The first service matched is returned on success. The service must be released by the caller.
pub unsafe extern "C-unwind" fn IOServiceGetMatchingService(
    main_port: libc::mach_port_t,
    matching: Option<CFRetained<CFDictionary>>,
) -> io_service_t {
    extern "C-unwind" {
        fn IOServiceGetMatchingService(
            main_port: libc::mach_port_t,
            matching: *mut CFDictionary,
        ) -> io_service_t;
    }

    unsafe { IOServiceGetMatchingService(main_port, consume(matching)) }
}

/// Look up registered IOService objects that match a matching dictionary.
///
/// This is the preferred method of finding IOService objects currently registered by IOKit (that is, objects that have had their registerService() methods invoked). To find IOService objects that aren't yet registered, use an iterator as created by IORegistryEntryCreateIterator(). IOServiceAddMatchingNotification can also supply this information and install a notification of new IOServices. The matching information used in the matching dictionary may vary depending on the class of service being looked up.
///
/// Parameter `mainPort`: The main port obtained from IOMainPort(). Pass kIOMainPortDefault to look up the default main port.
///
/// Parameter `matching`: A CF dictionary containing matching information, of which one reference is always consumed by this function (Note prior to the Tiger release there was a small chance that the dictionary might not be released if there was an error attempting to serialize the dictionary). IOKitLib can construct matching dictionaries for common criteria with helper functions such as IOServiceMatching, IOServiceNameMatching, IOBSDNameMatching.
///
/// Parameter `existing`: An iterator handle, or NULL, is returned on success, and should be released by the caller when the iteration is finished. If NULL is returned, the iteration was successful but found no matching services.
///
/// Returns: A kern_return_t error code.
pub unsafe extern "C-unwind" fn IOServiceGetMatchingServices(
    main_port: libc::mach_port_t,
    matching: Option<CFRetained<CFDictionary>>,
    existing: *mut io_iterator_t,
) -> libc::kern_return_t {
    extern "C-unwind" {
        fn IOServiceGetMatchingServices(
            main_port: libc::mach_port_t,
            matching: *mut CFDictionary,
            existing: *mut io_iterator_t,
        ) -> libc::kern_return_t;
    }

    unsafe { IOServiceGetMatchingServices(main_port, consume(matching), existing) }
}

/// Look up registered IOService objects that match a matching dictionary, and install a notification request of new IOServices that match.
///
/// This is the preferred method of finding IOService objects that may arrive at any time. The type of notification specifies the state change the caller is interested in, on IOService's that match the match dictionary. Notification types are identified by name, and are defined in IOKitKeys.h. The matching information used in the matching dictionary may vary depending on the class of service being looked up.
///
/// Parameter `notifyPort`: A IONotificationPortRef object that controls how messages will be sent when the armed notification is fired. When the notification is delivered, the io_iterator_t representing the notification should be iterated through to pick up all outstanding objects. When the iteration is finished the notification is rearmed. See IONotificationPortCreate.
///
/// Parameter `notificationType`: A notification type from IOKitKeys.h
/// <br>
/// kIOPublishNotification Delivered when an IOService is registered.
/// <br>
/// kIOFirstPublishNotification Delivered when an IOService is registered, but only once per IOService instance. Some IOService's may be reregistered when their state is changed.
/// <br>
/// kIOMatchedNotification Delivered when an IOService has had all matching drivers in the kernel probed and started.
/// <br>
/// kIOFirstMatchNotification Delivered when an IOService has had all matching drivers in the kernel probed and started, but only once per IOService instance. Some IOService's may be reregistered when their state is changed.
/// <br>
/// kIOTerminatedNotification Delivered after an IOService has been terminated.
///
/// Parameter `matching`: A CF dictionary containing matching information, of which one reference is always consumed by this function (Note prior to the Tiger release there was a small chance that the dictionary might not be released if there was an error attempting to serialize the dictionary). IOKitLib can construct matching dictionaries for common criteria with helper functions such as IOServiceMatching, IOServiceNameMatching, IOBSDNameMatching.
///
/// Parameter `callback`: A callback function called when the notification fires.
///
/// Parameter `refCon`: A reference constant for the callbacks use.
///
/// Parameter `notification`: An iterator handle is returned on success, and should be released by the caller when the notification is to be destroyed. The notification is armed when the iterator is emptied by calls to IOIteratorNext - when no more objects are returned, the notification is armed. Note the notification is not armed when first created.
///
/// Returns: A kern_return_t error code.
pub unsafe extern "C-unwind" fn IOServiceAddMatchingNotification(
    notify_port: IONotificationPortRef,
    notification_type: io_name_t,
    matching: Option<CFRetained<CFDictionary>>,
    callback: IOServiceMatchingCallback,
    ref_con: *mut c_void,
    notification: *mut io_iterator_t,
) -> libc::kern_return_t {
    extern "C-unwind" {
        fn IOServiceAddMatchingNotification(
            notify_port: IONotificationPortRef,
            notification_type: io_name_t,
            matching: *mut CFDictionary,
            callback: IOServiceMatchingCallback,
            ref_con: *mut c_void,
            notification: *mut io_iterator_t,
        ) -> libc::kern_return_t;
    }

    unsafe {
        IOServiceAddMatchingNotification(
            notify_port,
            notification_type,
            consume(matching),
            callback,
            ref_con,
            notification,
        )
    }
}
