use crate::ns_string;
use anyhow::{Result, anyhow};
use block::{Block, ConcreteBlock};
use cocoa::{
    base::{BOOL, YES, id, nil},
    foundation::{NSArray, NSBundle, NSInteger, NSUInteger},
};
use futures::channel::oneshot;
use gpui::{
    BackgroundExecutor, SystemNotification, SystemNotificationId, SystemNotificationPermission,
    Task,
};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use std::{
    ffi::{CStr, c_char},
    sync::{Arc, Mutex},
};

const UN_AUTHORIZATION_STATUS_NOT_DETERMINED: NSInteger = 0;
const UN_AUTHORIZATION_STATUS_DENIED: NSInteger = 1;
const UN_AUTHORIZATION_STATUS_AUTHORIZED: NSInteger = 2;
const UN_AUTHORIZATION_STATUS_PROVISIONAL: NSInteger = 3;
const UN_AUTHORIZATION_STATUS_EPHEMERAL: NSInteger = 4;

const UN_AUTHORIZATION_OPTION_ALERT: NSUInteger = 1 << 2;
const UN_NOTIFICATION_PRESENTATION_OPTION_ALERT: NSUInteger = 1 << 2;

pub(super) fn system_notification_permission(
    executor: BackgroundExecutor,
) -> Task<Result<SystemNotificationPermission>> {
    if !has_bundle_identifier() {
        return Task::ready(Ok(SystemNotificationPermission::Unsupported));
    }

    let Some(center) = notification_center() else {
        return Task::ready(Ok(SystemNotificationPermission::Unsupported));
    };

    let (done_tx, done_rx) = oneshot::channel();
    let done_tx = Arc::new(Mutex::new(Some(done_tx)));
    unsafe {
        let done_tx = done_tx.clone();
        let block = ConcreteBlock::new(move |settings: id| {
            let permission = if settings == nil {
                SystemNotificationPermission::Unsupported
            } else {
                let status: NSInteger = msg_send![settings, authorizationStatus];
                permission_from_authorization_status(status)
            };
            send_result(&done_tx, Ok(permission));
        });
        let block = block.copy();
        let _: () = msg_send![center, getNotificationSettingsWithCompletionHandler: block];
    }

    result_task(executor, done_rx)
}

pub(super) fn request_system_notification_permission(
    executor: BackgroundExecutor,
) -> Task<Result<SystemNotificationPermission>> {
    if !has_bundle_identifier() {
        return Task::ready(Ok(SystemNotificationPermission::Unsupported));
    }

    let Some(center) = notification_center() else {
        return Task::ready(Ok(SystemNotificationPermission::Unsupported));
    };

    let (done_tx, done_rx) = oneshot::channel();
    let done_tx = Arc::new(Mutex::new(Some(done_tx)));
    unsafe {
        let done_tx = done_tx.clone();
        let block = ConcreteBlock::new(move |granted: BOOL, error: id| {
            let result = if error == nil {
                if granted == YES {
                    Ok(SystemNotificationPermission::Granted)
                } else {
                    Ok(SystemNotificationPermission::Denied)
                }
            } else {
                Err(anyhow!(
                    "failed to request system notification permission: {}",
                    error_description(error)
                ))
            };
            send_result(&done_tx, result);
        });
        let block = block.copy();
        let _: () = msg_send![
            center,
            requestAuthorizationWithOptions: UN_AUTHORIZATION_OPTION_ALERT
            completionHandler: block
        ];
    }

    result_task(executor, done_rx)
}

pub(super) fn show_system_notification(
    executor: BackgroundExecutor,
    notification: SystemNotification,
) -> Task<Result<()>> {
    if !has_bundle_identifier() {
        return Task::ready(Err(anyhow!(
            "macOS system notifications require a bundled app with a bundle identifier"
        )));
    }

    let Some(center) = notification_center() else {
        return Task::ready(Err(anyhow!("system notifications are not supported")));
    };

    let (done_tx, done_rx) = oneshot::channel();
    let done_tx = Arc::new(Mutex::new(Some(done_tx)));
    unsafe {
        let content: id = msg_send![class!(UNMutableNotificationContent), new];
        let title = ns_string(notification.title.as_ref());
        let _: () = msg_send![content, setTitle: title];
        if let Some(body) = notification.body {
            let body = ns_string(body.as_ref());
            let _: () = msg_send![content, setBody: body];
        }

        let identifier = ns_string(notification.id.0.as_ref());
        let request: id = msg_send![
            class!(UNNotificationRequest),
            requestWithIdentifier: identifier
            content: content
            trigger: nil
        ];
        let _: () = msg_send![content, release];

        let done_tx = done_tx.clone();
        let block = ConcreteBlock::new(move |error: id| {
            let result = if error == nil {
                Ok(())
            } else {
                Err(anyhow!(
                    "failed to show system notification: {}",
                    error_description(error)
                ))
            };
            send_result(&done_tx, result);
        });
        let block = block.copy();
        let _: () = msg_send![center, addNotificationRequest: request withCompletionHandler: block];
    }

    result_task(executor, done_rx)
}

pub(super) fn remove_system_notification(
    _executor: BackgroundExecutor,
    id: SystemNotificationId,
) -> Task<Result<()>> {
    if !has_bundle_identifier() {
        return Task::ready(Ok(()));
    }

    let Some(center) = notification_center() else {
        return Task::ready(Ok(()));
    };

    unsafe {
        let identifier = ns_string(id.0.as_ref());
        let identifiers = NSArray::arrayWithObject(nil, identifier);
        let _: () =
            msg_send![center, removePendingNotificationRequestsWithIdentifiers: identifiers];
        let _: () = msg_send![center, removeDeliveredNotificationsWithIdentifiers: identifiers];
    }

    Task::ready(Ok(()))
}

pub(super) unsafe fn set_delegate(delegate: id) {
    unsafe {
        if !has_bundle_identifier() {
            return;
        }

        if let Some(center) = notification_center() {
            let _: () = msg_send![center, setDelegate: delegate];
        }
    }
}

pub(super) extern "C" fn will_present_notification(
    _this: &mut Object,
    _: objc::runtime::Sel,
    _: id,
    _: id,
    completion_handler: id,
) {
    unsafe {
        if completion_handler == nil {
            return;
        }

        let completion_handler = &*(completion_handler as *const Block<(NSUInteger,), ()>);
        completion_handler.call((UN_NOTIFICATION_PRESENTATION_OPTION_ALERT,));
    }
}

fn result_task<T: Send + 'static>(
    executor: BackgroundExecutor,
    done_rx: oneshot::Receiver<Result<T>>,
) -> Task<Result<T>> {
    executor.spawn(async move { done_rx.await.map_err(|error| anyhow!(error))? })
}

fn send_result<T>(done_tx: &Arc<Mutex<Option<oneshot::Sender<Result<T>>>>>, result: Result<T>) {
    match done_tx.lock() {
        Ok(mut done_tx) => {
            if let Some(done_tx) = done_tx.take()
                && done_tx.send(result).is_err()
            {
                log::debug!("system notification task was dropped before completion");
            }
        }
        Err(error) => log::error!("failed to complete system notification task: {error}"),
    }
}

fn notification_center() -> Option<id> {
    unsafe {
        let center: id = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
        (center != nil).then_some(center)
    }
}

fn has_bundle_identifier() -> bool {
    unsafe {
        let bundle: id = NSBundle::mainBundle();
        if bundle == nil {
            return false;
        }

        let bundle_identifier: id = msg_send![bundle, bundleIdentifier];
        bundle_identifier != nil
    }
}

fn permission_from_authorization_status(status: NSInteger) -> SystemNotificationPermission {
    match status {
        UN_AUTHORIZATION_STATUS_NOT_DETERMINED => SystemNotificationPermission::NotDetermined,
        UN_AUTHORIZATION_STATUS_DENIED => SystemNotificationPermission::Denied,
        UN_AUTHORIZATION_STATUS_AUTHORIZED
        | UN_AUTHORIZATION_STATUS_PROVISIONAL
        | UN_AUTHORIZATION_STATUS_EPHEMERAL => SystemNotificationPermission::Granted,
        _ => SystemNotificationPermission::Unsupported,
    }
}

unsafe fn error_description(error: id) -> String {
    unsafe {
        let message: id = msg_send![error, localizedDescription];
        let code: NSInteger = msg_send![error, code];
        let domain: id = msg_send![error, domain];
        let domain = string_from_ns_string(domain);
        let message = string_from_ns_string(message);

        match (domain, message) {
            (Some(domain), Some(message)) => format!("{domain} error {code}: {message}"),
            (Some(domain), None) => format!("{domain} error {code}"),
            (None, Some(message)) => format!("error {code}: {message}"),
            (None, None) => format!("{error:?}"),
        }
    }
}

unsafe fn string_from_ns_string(string: id) -> Option<String> {
    unsafe {
        if string == nil {
            return None;
        }

        let bytes: *const c_char = msg_send![string, UTF8String];
        (!bytes.is_null()).then(|| CStr::from_ptr(bytes).to_string_lossy().into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_authorization_status() {
        assert_eq!(
            permission_from_authorization_status(UN_AUTHORIZATION_STATUS_NOT_DETERMINED),
            SystemNotificationPermission::NotDetermined
        );
        assert_eq!(
            permission_from_authorization_status(UN_AUTHORIZATION_STATUS_DENIED),
            SystemNotificationPermission::Denied
        );
        assert_eq!(
            permission_from_authorization_status(UN_AUTHORIZATION_STATUS_AUTHORIZED),
            SystemNotificationPermission::Granted
        );
        assert_eq!(
            permission_from_authorization_status(UN_AUTHORIZATION_STATUS_PROVISIONAL),
            SystemNotificationPermission::Granted
        );
        assert_eq!(
            permission_from_authorization_status(NSInteger::MAX),
            SystemNotificationPermission::Unsupported
        );
    }
}

#[link(name = "UserNotifications", kind = "framework")]
unsafe extern "C" {}
