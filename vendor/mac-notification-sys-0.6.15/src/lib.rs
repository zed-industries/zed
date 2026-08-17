//! A very thin wrapper around NSNotifications
#![deny(deref_nullptr)]
#![deny(invalid_value)]
#![deny(invalid_from_utf8)]
#![deny(never_type_fallback_flowing_into_unsafe)]
#![deny(ptr_to_integer_transmute_in_consts)]
#![deny(static_mut_refs)]
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]
// The extern "C" callbacks called from ObjC unavoidably take raw pointer arguments.
// They cannot be marked `unsafe` (Rust forbids unsafe extern "C" fn that are exported),
// yet they must dereference those pointers — suppress the lint crate-wide for this pattern.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use error::{ApplicationError, NotificationError, NotificationResult};
pub use notification::{MainButton, Notification, NotificationResponse, Sound};
use objc2_foundation::NSString;
use std::{
    ops::Deref,
    sync::{Arc, Condvar, Mutex, Once, atomic::AtomicBool},
};

mod bridge;
pub mod error;
mod notification;
mod pending_guard;

mod sys {
    use objc2_foundation::{NSDictionary, NSString};
    #[link(name = "notify")]
    unsafe extern "C" {
        pub fn sendNotification(
            title: *const NSString,
            subtitle: *const NSString,
            message: *const NSString,
            options: *const NSDictionary<NSString, NSString>,
            notification_id: *const u8,
            should_wait: bool,
        );
        pub fn setApplication(newbundleIdentifier: *const NSString) -> bool;
        pub fn getBundleIdentifier(appName: *const NSString) -> *const NSString;
        pub fn setupDelegate();
    }
}

static INIT_APPLICATION_SET: Once = Once::new();

/// Delivers a new notification
///
/// Returns a `NotificationError` if a notification could not be delivered
///
/// # Example:
///
/// ```no_run
/// # use mac_notification_sys::*;
/// // deliver a silent notification
/// let _ = send_notification("Title", None, "This is the body", None).unwrap();
/// ```
pub fn send_notification(
    title: &str,
    subtitle: Option<&str>,
    message: &str,
    options: Option<&Notification>,
) -> NotificationResult<NotificationResponse> {
    if let Some(options) = &options {
        if let Some(delivery_date) = options.delivery_date {
            ensure!(
                delivery_date >= time::OffsetDateTime::now_utc().unix_timestamp() as f64,
                NotificationError::ScheduleInThePast
            );
        }
    };

    ensure_application_set()?;
    ensure_delegate_initiated();

    let should_wait = options.map(|o| o.needs_response()).unwrap_or(false);
    let options_dict = options.unwrap_or(&Notification::new()).to_dictionary();

    let id: [u8; 16] = uuid::Uuid::new_v4().into_bytes();

    let entry = Arc::new(pending_guard::PendingEntry {
        result: Mutex::new(NotificationResponse::None),
        done: AtomicBool::new(!should_wait),
        condvar: Condvar::new(),
        delivered: Mutex::new(false),
        delivered_cv: Condvar::new(),
    });
    pending_guard::pending()
        .lock()
        .unwrap()
        .insert(id, Arc::clone(&entry));
    // PendingGuard performs the sole remove — both on the normal path and on panic.
    // Reading the result from `entry` directly avoids a second remove call.
    let _guard = pending_guard::PendingGuard { id };

    unsafe {
        sys::sendNotification(
            NSString::from_str(title).deref(),
            NSString::from_str(subtitle.unwrap_or("")).deref(),
            NSString::from_str(message).deref(),
            options_dict.deref(),
            id.as_ptr(),
            should_wait,
        );
    }

    let result = entry.result.lock().unwrap().clone();
    Ok(result)
}

/// Search for a possible BundleIdentifier of a given appname.
/// Defaults to "com.apple.Finder" if no BundleIdentifier is found.
pub fn get_bundle_identifier_or_default(app_name: &str) -> String {
    get_bundle_identifier(app_name).unwrap_or_else(|| "com.apple.Finder".to_string())
}

/// Search for a BundleIdentifier of an given appname.
pub fn get_bundle_identifier(app_name: &str) -> Option<String> {
    unsafe { sys::getBundleIdentifier(NSString::from_str(app_name).deref()).as_ref() }
        .map(NSString::to_string)
}

/// Sets the application if not already set
fn ensure_application_set() -> NotificationResult<()> {
    if INIT_APPLICATION_SET.is_completed() {
        return Ok(());
    };
    let bundle = get_bundle_identifier_or_default("use_default");
    set_application(&bundle)
}

fn ensure_delegate_initiated() {
    // `sharedDelegate` in ObjC is already guarded by `dispatch_once`; calling it here
    // is idempotent and thread-safe without an extra Rust-side Once.
    unsafe { sys::setupDelegate() };
}

/// Set the application which delivers or schedules a notification
pub fn set_application(bundle_ident: &str) -> NotificationResult<()> {
    let mut result = Err(ApplicationError::AlreadySet(bundle_ident.into()).into());
    INIT_APPLICATION_SET.call_once(|| {
        let was_set = unsafe { sys::setApplication(NSString::from_str(bundle_ident).deref()) };
        result = if was_set {
            Ok(())
        } else {
            Err(ApplicationError::CouldNotSet(bundle_ident.into()).into())
        };
    });
    result
}
