//! System notifications as Windows toast notifications.
//!
//! Toasts require an AppUserModelID (set through
//! `Platform::set_app_identity`); without one, notifications are dropped
//! with a warning. Unpackaged apps additionally need the AppUserModelID
//! registered in the per-user registry, which happens automatically before
//! the first toast is shown.

use std::cell::RefCell;
use std::rc::Rc;

use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::{
    ForegroundExecutor, SharedString, SystemNotification, SystemNotificationResponse, Task,
};
use tauri_winrt_notification::Toast;

type ResponseCallback = Rc<RefCell<Option<Box<dyn FnMut(SystemNotificationResponse)>>>>;

pub(crate) struct SystemNotificationState {
    /// The application identity whose registry entry has been written this
    /// session, if any.
    registered_app_identity: RefCell<Option<(String, String)>>,
    response_sender: mpsc::UnboundedSender<SystemNotificationResponse>,
    response_receiver: RefCell<Option<mpsc::UnboundedReceiver<SystemNotificationResponse>>>,
    callback: ResponseCallback,
    _response_task: RefCell<Option<Task<()>>>,
}

impl SystemNotificationState {
    pub(crate) fn new() -> Self {
        let (response_sender, response_receiver) = mpsc::unbounded();
        Self {
            registered_app_identity: RefCell::new(None),
            response_sender,
            response_receiver: RefCell::new(Some(response_receiver)),
            callback: Rc::new(RefCell::new(None)),
            _response_task: RefCell::new(None),
        }
    }

    pub(crate) fn show(
        &self,
        app_identity: Option<(&str, &str)>,
        notification: SystemNotification,
    ) {
        let Some((app_identifier, app_name)) = app_identity else {
            log::warn!(
                "cannot show a system notification without an app identity; \
                 call `App::set_app_identity` during startup"
            );
            return;
        };
        self.register_app_user_model_id(app_identifier, app_name);

        let mut toast = Toast::new(app_identifier)
            .title(&notification.title)
            .text1(&notification.body)
            .sound(None);
        for action in &notification.actions {
            toast = toast.add_button(&action.label, &action.id);
        }

        let sender = self.response_sender.clone();
        let tag = notification.tag;
        let result = toast
            .on_activated(move |action| {
                // A button press reports the button's argument; activating the
                // toast body reports no argument.
                let action_id = action
                    .filter(|action| !action.is_empty())
                    .map(SharedString::from);
                sender
                    .unbounded_send(SystemNotificationResponse {
                        tag: tag.clone(),
                        action_id,
                    })
                    .ok();
                Ok(())
            })
            .show();
        if let Err(error) = result {
            log::warn!("failed to show system notification: {error}");
        }
    }

    pub(crate) fn dismiss(&self, _tag: &str) {
        // `tauri_winrt_notification` exposes no handle to remove a shown toast
        // from the Action Center; stale notifications age out.
    }

    pub(crate) fn on_response(
        &self,
        executor: &ForegroundExecutor,
        callback: Box<dyn FnMut(SystemNotificationResponse)>,
    ) {
        *self.callback.borrow_mut() = Some(callback);

        // Responses arrive on WinRT threads; this task hands them to the
        // registered callback on the main thread.
        if let Some(mut receiver) = self.response_receiver.borrow_mut().take() {
            let callback = self.callback.clone();
            *self._response_task.borrow_mut() = Some(executor.spawn(async move {
                while let Some(response) = receiver.next().await {
                    // Take the callback out for the call: it may re-enter the
                    // platform or replace itself.
                    let taken = callback.borrow_mut().take();
                    if let Some(mut taken) = taken {
                        taken(response);
                        callback.borrow_mut().get_or_insert(taken);
                    }
                }
            }));
        }
    }

    /// Toasts from unpackaged apps display under an AppUserModelID that must
    /// be registered in the per-user registry (an installer's Start Menu
    /// shortcut is the other way to register one, but gpui apps may ship
    /// without an installer).
    fn register_app_user_model_id(&self, app_identifier: &str, app_name: &str) {
        let mut registered = self.registered_app_identity.borrow_mut();
        if registered
            .as_ref()
            .is_some_and(|(identifier, name)| identifier == app_identifier && name == app_name)
        {
            return;
        }
        match write_app_user_model_id_registry_entry(app_identifier, app_name) {
            Ok(()) => *registered = Some((app_identifier.to_string(), app_name.to_string())),
            Err(error) => log::warn!(
                "failed to register AppUserModelId; notifications may not display: {error}"
            ),
        }
    }
}

fn write_app_user_model_id_registry_entry(
    app_identifier: &str,
    app_name: &str,
) -> windows_registry::Result<()> {
    let key = windows_registry::CURRENT_USER
        .create(format!(r"Software\Classes\AppUserModelId\{app_identifier}"))?;
    key.set_string("DisplayName", app_name)?;
    Ok(())
}
