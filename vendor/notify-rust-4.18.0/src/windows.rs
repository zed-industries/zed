use crate::response::NotificationResponse;
use crate::CloseReason;
pub use crate::{error::*, notification::Notification, timeout::Timeout, urgency::Urgency};

use std::{
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
    sync::mpsc::{channel, Receiver},
};

use winrt_notification::{Toast, ToastDismissalReason};

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    let (sender, receiver) = channel();
    let activated_sender = sender.clone();

    let mut toast = build_toast(notification);

    for action in notification.actions.chunks(2) {
        if let [identifier, label] = action {
            toast = toast.add_button(label, identifier);
        }
    }

    toast = toast
        .on_activated(move |action| {
            let response = match action {
                None => NotificationResponse::Default,
                Some(key) => NotificationResponse::Action(key),
            };
            let _ = activated_sender.send(response);
            Ok(())
        })
        .on_dismissed(move |reason| {
            let _ = sender.send(NotificationResponse::Closed(reason.into()));
            Ok(())
        });

    toast
        .show()
        .map_err(|error| Error::from(ErrorKind::Msg(format!("{error:?}"))))?;

    Ok(NotificationHandle {
        notification: notification.clone(),
        events: receiver,
    })
}

fn build_toast(notification: &Notification) -> Toast {
    let sound = match &notification.sound_name {
        Some(chosen_sound_name) => winrt_notification::Sound::from_str(chosen_sound_name).ok(),
        None => None,
    };

    let duration = match notification.timeout {
        Timeout::Default => winrt_notification::Duration::Short,
        Timeout::Never => winrt_notification::Duration::Long,
        Timeout::Milliseconds(t) => {
            if t >= 25000 {
                winrt_notification::Duration::Long
            } else {
                winrt_notification::Duration::Short
            }
        }
    };

    // Map urgency to Windows toast scenario
    // Low/Normal -> Default (standard behavior)
    // Critical -> Reminder (stays on screen until dismissed, matching XDG spec)
    let scenario = match notification.urgency {
        Some(Urgency::Critical) => Some(winrt_notification::Scenario::Reminder),
        Some(Urgency::Low) | Some(Urgency::Normal) | None => None, // Default scenario
    };

    let app_id = notification
        .app_id
        .as_deref()
        .unwrap_or(Toast::POWERSHELL_APP_ID);

    let mut toast = Toast::new(app_id)
        .title(&notification.summary)
        .text1(notification.subtitle.as_ref().map_or("", AsRef::as_ref)) // subtitle
        .text2(&notification.body)
        .sound(sound)
        .duration(duration);

    // Apply scenario only for critical urgency
    if let Some(scenario) = scenario {
        toast = toast.scenario(scenario);
    }
    if let Some(image_path) = &notification.path_to_image {
        toast = toast.image(Path::new(&image_path), "");
    }

    toast
}

/// A handle to a Windows toast notification.
///
/// This keeps the notification data and receiver needed to wait for activation or dismissal
/// events after the toast has been shown.
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Notification,
    events: Receiver<NotificationResponse>,
}

impl NotificationHandle {
    /// Waits for the user to act on the notification and calls `invocation_closure` with the corresponding action identifier.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        match self.events.recv() {
            Ok(NotificationResponse::Action(action)) => invocation_closure(&action),
            Ok(_) => invocation_closure("__closed"),
            Err(_error) => invocation_closure("__closed"),
        }
    }

    /// Waits for the user to act on the notification and calls `handler` with the full action response.
    pub fn wait_for_response(self, handler: impl crate::response::ResponseHandler) -> Result<()> {
        match self.events.recv() {
            Ok(response) => {
                handler.call(&response);
                Ok(())
            }
            Err(error) => Err(Error::from(ErrorKind::Msg(format!(
                "failed to get notification response: {error}"
            )))),
        }
    }

    /// Executes a closure after the notification has closed.
    pub fn on_close<A>(self, handler: impl CloseHandler<A>) {
        while let Ok(event) = self.events.recv() {
            if let NotificationResponse::Closed(reason) = event {
                handler.call(reason);
                break;
            }
        }
    }
}

impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.notification
    }
}

impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.notification
    }
}

impl From<Option<ToastDismissalReason>> for CloseReason {
    fn from(reason: Option<ToastDismissalReason>) -> Self {
        match reason {
            Some(ToastDismissalReason::TimedOut) => CloseReason::Expired,
            Some(ToastDismissalReason::UserCanceled) => CloseReason::Dismissed,
            Some(ToastDismissalReason::ApplicationHidden) => CloseReason::CloseAction,
            Some(_) | None => CloseReason::Other(0),
        }
    }
}

/// Callback for the close event of a Windows notification.
pub trait CloseHandler<T> {
    /// Called with the [`CloseReason`].
    fn call(&self, reason: CloseReason);
}

impl<F> CloseHandler<CloseReason> for F
where
    F: Fn(CloseReason),
{
    fn call(&self, reason: CloseReason) {
        self(reason);
    }
}

impl<F> CloseHandler<()> for F
where
    F: Fn(),
{
    fn call(&self, _: CloseReason) {
        self();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handle_with(event: NotificationResponse) -> NotificationHandle {
        let (sender, receiver) = channel();
        sender.send(event).unwrap();
        NotificationHandle {
            notification: Notification::new(),
            events: receiver,
        }
    }

    #[test]
    fn wait_for_action_returns_custom_action() {
        let handle = handle_with(NotificationResponse::Action("clicked_a".to_owned()));
        let mut actual = String::new();

        handle.wait_for_action(|action| {
            actual = action.to_owned();
        });

        assert_eq!(actual, "clicked_a");
    }

    #[test]
    fn wait_for_action_keeps_closed_compatibility_keyword() {
        let handle = handle_with(NotificationResponse::Closed(CloseReason::Dismissed));
        let mut actual = String::new();

        handle.wait_for_action(|action| {
            actual = action.to_owned();
        });

        assert_eq!(actual, "__closed");
    }

    #[test]
    fn wait_for_response_preserves_close_reason() {
        use crate::response::{CloseReason as Reason, NotificationResponse};

        let handle = handle_with(NotificationResponse::Closed(CloseReason::Expired));
        let mut expired = false;

        handle
            .wait_for_response(|response: &NotificationResponse| {
                expired = matches!(response, NotificationResponse::Closed(Reason::Expired));
            })
            .unwrap();

        assert!(expired);
    }
}
