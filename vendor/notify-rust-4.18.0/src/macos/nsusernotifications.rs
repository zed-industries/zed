use crate::{error::*, notification::Notification};

pub use mac_notification_sys::error::{ApplicationError, Error as MacOsError, NotificationError};

/// A handle to a shown notification (**`NSUserNotificationCenter`** path).
///
/// This stack is deprecated on macOS 14+, but still works.
/// Enable the `preview-macos-un` feature to use the modern `UNUserNotificationCenter` path instead.
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Option<Notification>,
}

impl NotificationHandle {
    /// Construct a handle wrapping the given notification.
    // TODO: make private in 5.0
    pub fn new(notification: Notification) -> Self {
        Self {
            notification: Some(notification),
        }
    }

    /// Wait for the user to interact with the notification.
    ///
    /// The closure receives the action identifier as a `&str`.
    /// The special value `"__closed"` is passed when the notification is
    /// dismissed without activating any action.
    ///
    /// **Requires the main run loop to be running.** `NSUserNotificationCenter`
    /// delivers delegate callbacks on the main thread; if nothing is pumping the
    /// main run loop this call will block indefinitely.
    pub fn wait_for_action<F>(mut self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        log::trace!("wait_for_action");
        // let n = build_mac_notification(&self.notification);
        // n.wait_for_click(true);
        let Some(notification) = self.notification.take() else {
            return;
        };
        match send_mac_notification(
            &notification,
            Options {
                asynchronous: false,
                force_close_button: false,
                delivery_date: None,
            },
        )
        .unwrap_or(mac_notification_sys::NotificationResponse::None)
        {
            mac_notification_sys::NotificationResponse::ActionButton(ref label) => {
                invocation_closure(label);
            }
            mac_notification_sys::NotificationResponse::Click => invocation_closure("default"),
            mac_notification_sys::NotificationResponse::Reply(ref text) => {
                invocation_closure(text);
            }
            mac_notification_sys::NotificationResponse::CloseButton(_)
            | mac_notification_sys::NotificationResponse::None => invocation_closure("__closed"),
        }
    }

    /// Waits for the user to act on the notification and then calls `handler`
    /// with a typed [`NotificationResponse`](crate::NotificationResponse).
    ///
    /// This is the typed, forward-compatible replacement for
    /// [`wait_for_action`](Self::wait_for_action).
    ///
    /// **Requires the main run loop to be running.** `NSUserNotificationCenter`
    /// delivers delegate callbacks on the main thread; if nothing is pumping the
    /// main run loop this call will block indefinitely.
    pub fn wait_for_response(
        mut self,
        handler: impl crate::response::ResponseHandler,
    ) -> Result<()> {
        use crate::response::{CloseReason, NotificationResponse};

        log::trace!("wait_for_response");
        let Some(notification) = self.notification.take() else {
            return Ok(());
        };

        let response = send_mac_notification(
            &notification,
            Options {
                asynchronous: false,
                force_close_button: false,
                delivery_date: None,
            },
        )?;

        use mac_notification_sys::NotificationResponse as MacResponse;
        use NotificationResponse::*;

        let response = match response {
            MacResponse::ActionButton(label) => Action(label),
            MacResponse::Click => Default,
            MacResponse::Reply(text) => Reply(text),
            MacResponse::CloseButton(_) => Closed(CloseReason::Dismissed),
            MacResponse::None => Closed(CloseReason::Expired),
        };

        handler.call(&response);
        Ok(())
    }

    /// Executes a closure after the notification has closed.
    ///
    /// **Requires the main run loop to be running.** `NSUserNotificationCenter`
    /// delivers delegate callbacks on the main thread; if nothing is pumping the
    /// main run loop this call will block indefinitely.
    pub fn on_close<A>(mut self, handler: impl crate::response::CloseHandler<A>) {
        let Some(notification) = self.notification.take() else {
            return;
        };

        let response = send_mac_notification(
            &notification,
            Options {
                asynchronous: false,
                force_close_button: true,
                delivery_date: None,
            },
        )
        .unwrap_or(mac_notification_sys::NotificationResponse::None);
        log::trace!("response: {response:?}");

        match response {
            mac_notification_sys::NotificationResponse::CloseButton(_) => {
                handler.call(crate::CloseReason::Dismissed);
            }
            mac_notification_sys::NotificationResponse::None => {
                handler.call(crate::CloseReason::Expired);
            }
            // user interacted — not a close event, handler not called
            _ => {}
        }
    }
}

impl Drop for NotificationHandle {
    fn drop(&mut self) {
        log::trace!(
            "not using handle, sending immediately: {:#?}",
            self.notification
        );
        let Some(notification) = self.notification.take() else {
            return;
        };

        send_mac_notification(
            &notification,
            Options {
                asynchronous: true,
                force_close_button: false,
                delivery_date: None,
            },
        )
        .ok();
    }
}

struct Options {
    asynchronous: bool,
    force_close_button: bool,
    delivery_date: Option<f64>,
}

fn send_mac_notification(
    notification: &Notification,

    Options {
        asynchronous,
        force_close_button,
        delivery_date,
    }: Options,
) -> Result<mac_notification_sys::NotificationResponse> {
    // actions is a flat [id, label, id, label, …] vec
    let labels: Vec<&str> = notification
        .actions
        .chunks(2)
        .filter_map(|pair| pair.get(1).map(|s| s.as_str()))
        .collect();
    // label -> id map for translating the response back
    let label_to_id: std::collections::HashMap<&str, &str> = notification
        .actions
        .chunks(2)
        .filter_map(|pair| match pair {
            [id, label] => Some((label.as_str(), id.as_str())),
            _ => None,
        })
        .collect();

    let main_button = match labels.as_slice() {
        [] => None,
        [single] => Some(mac_notification_sys::MainButton::SingleAction(single)),
        // "Options" is a neutral button title not present in labels, so no visual duplicate.
        // Clicking the button directly (idx == LONG_MAX in ObjC) returns "Options"; we remap
        // that below to the first action's id.
        _ => Some(mac_notification_sys::MainButton::DropdownActions(
            "Options", &labels,
        )),
    };

    let mut n = mac_notification_sys::Notification::default();
    n.title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref());
    if let Some(ref btn) = main_button {
        n.main_button(btn.clone());
    }

    if asynchronous {
        log::trace!("Notification will be sent asynchronously");
    } else {
        log::trace!("Notification will be sent synchronously");
        if force_close_button {
            // on_close needs a close button to block until the user dismisses
            n.close_button("Close");
        }
    }
    n.asynchronous(asynchronous);

    if let Some(ref image_path) = notification.path_to_image {
        n.content_image(image_path);
    }
    if let Some(delivery_date) = delivery_date {
        n.delivery_date(delivery_date);
    }

    let response = n.send().map_err(Into::<Error>::into)?;
    // translate ActionButton(label) -> ActionButton(id)
    let response = match response {
        mac_notification_sys::NotificationResponse::ActionButton(ref label) => {
            if let Some(&id) = label_to_id.get(label.as_str()) {
                mac_notification_sys::NotificationResponse::ActionButton(id.to_owned())
            } else if let Some(first_id) = notification.actions.first().map(|s| s.as_str()) {
                // label didn't match (e.g. user clicked the dropdown button title directly);
                // fall back to the first action
                mac_notification_sys::NotificationResponse::ActionButton(first_id.to_owned())
            } else {
                response
            }
        }
        other => other,
    };
    Ok(response)
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    Ok(
        #[allow(deprecated)]
        NotificationHandle::new(notification.clone()),
    )
}

pub(crate) fn schedule_notification(
    notification: &Notification,
    delivery_date: f64,
) -> Result<NotificationHandle> {
    send_mac_notification(
        notification,
        Options {
            asynchronous: true,
            force_close_button: false,
            delivery_date: Some(delivery_date),
        },
    )?;
    Ok(
        #[allow(deprecated)]
        NotificationHandle { notification: None },
    )
}
