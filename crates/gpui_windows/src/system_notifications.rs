//! System notifications as Windows toast notifications.

use std::cell::RefCell;
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash as _, Hasher as _};
use std::rc::Rc;

use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::{
    ForegroundExecutor, SharedString, SystemNotification, SystemNotificationResponse, Task,
};
use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::TypedEventHandler;
use windows::UI::Notifications::{
    ToastActivatedEventArgs, ToastNotification, ToastNotificationManager, ToastNotifier,
};
use windows::core::{IInspectable, Interface as _, h};

type ResponseCallback = Rc<RefCell<Option<Box<dyn FnMut(SystemNotificationResponse)>>>>;

pub(crate) struct SystemNotificationState {
    notifier: Option<ToastNotifier>,
    active_toasts: HashMap<SharedString, ToastNotification>,
    response_sender: mpsc::UnboundedSender<SystemNotificationResponse>,
    response_receiver: Option<mpsc::UnboundedReceiver<SystemNotificationResponse>>,
    callback: ResponseCallback,
    _response_task: Option<Task<()>>,
}

impl SystemNotificationState {
    pub(crate) fn new() -> Self {
        let (response_sender, response_receiver) = mpsc::unbounded();
        Self {
            notifier: None,
            active_toasts: HashMap::new(),
            response_sender,
            response_receiver: Some(response_receiver),
            callback: Rc::new(RefCell::new(None)),
            _response_task: None,
        }
    }

    pub(crate) fn show(
        &mut self,
        has_package_identity: bool,
        app_identity: Option<(&str, &str)>,
        notification: SystemNotification,
    ) -> windows::core::Result<()> {
        let Some(notifier) = self.notifier(has_package_identity, app_identity)? else {
            return Ok(());
        };

        let document = toast_document(&notification)?;
        let toast = ToastNotification::CreateToastNotification(&document)?;
        // Windows caps toast tags at 64 characters (post-Creators Update),
        // so hash the arbitrary GPUI tag down to a fixed-width value.
        let tag = {
            let mut hasher = DefaultHasher::new();
            notification.tag.hash(&mut hasher);
            format!("{:016x}", hasher.finish())
        };
        toast.SetTag(&tag.into())?;

        let sender = self.response_sender.clone();
        let response_tag = notification.tag.clone();
        toast.Activated(&TypedEventHandler::<ToastNotification, IInspectable>::new(
            move |_sender, arguments| {
                let action_id = arguments
                    .as_ref()
                    .and_then(|arguments| arguments.cast::<ToastActivatedEventArgs>().ok())
                    .and_then(|arguments| arguments.Arguments().ok())
                    .filter(|arguments| !arguments.is_empty())
                    .map(|arguments| SharedString::from(arguments.to_string()));
                sender
                    .unbounded_send(SystemNotificationResponse {
                        tag: response_tag.clone(),
                        action_id,
                    })
                    .ok();
                Ok(())
            },
        ))?;

        if let Some(previous) = self.active_toasts.remove(&notification.tag) {
            notifier.Hide(&previous)?;
        }
        notifier.Show(&toast)?;
        self.active_toasts.insert(notification.tag, toast);
        Ok(())
    }

    pub(crate) fn dismiss(&mut self, tag: &str) {
        let Some(toast) = self.active_toasts.remove(tag) else {
            return;
        };
        let Some(notifier) = &self.notifier else {
            return;
        };
        if let Err(error) = notifier.Hide(&toast) {
            log::warn!("failed to dismiss system notification: {error}");
        }
    }

    pub(crate) fn on_response(
        &mut self,
        executor: &ForegroundExecutor,
        callback: Box<dyn FnMut(SystemNotificationResponse)>,
    ) {
        *self.callback.borrow_mut() = Some(callback);

        if let Some(mut receiver) = self.response_receiver.take() {
            let callback = self.callback.clone();
            self._response_task = Some(executor.spawn(async move {
                while let Some(response) = receiver.next().await {
                    // Take the callback out for the call: it may re-enter the
                    // platform (e.g. to dismiss the notification it was told
                    // about) or replace itself.
                    let taken = callback.borrow_mut().take();
                    if let Some(mut taken) = taken {
                        taken(response);
                        callback.borrow_mut().get_or_insert(taken);
                    }
                }
            }));
        }
    }

    fn notifier(
        &mut self,
        has_package_identity: bool,
        app_identity: Option<(&str, &str)>,
    ) -> windows::core::Result<Option<ToastNotifier>> {
        if let Some(notifier) = &self.notifier {
            return Ok(Some(notifier.clone()));
        }

        let notifier = if has_package_identity {
            ToastNotificationManager::CreateToastNotifier()?
        } else {
            let Some((app_identifier, app_name)) = app_identity else {
                log::warn!(
                    "cannot show a system notification without an app identity; \
                     call `App::set_app_identity` during startup"
                );
                return Ok(None);
            };
            register_app_user_model_id(app_identifier, app_name);
            ToastNotificationManager::CreateToastNotifierWithId(&app_identifier.into())?
        };

        self.notifier = Some(notifier.clone());
        Ok(Some(notifier))
    }
}

fn toast_document(notification: &SystemNotification) -> windows::core::Result<XmlDocument> {
    let document = XmlDocument::new()?;
    let toast = document.CreateElement(h!("toast"))?;
    document.AppendChild(&toast)?;

    let visual = document.CreateElement(h!("visual"))?;
    toast.AppendChild(&visual)?;
    let binding = document.CreateElement(h!("binding"))?;
    binding.SetAttribute(h!("template"), h!("ToastGeneric"))?;
    visual.AppendChild(&binding)?;
    for text in [&notification.title, &notification.body] {
        let element = document.CreateElement(h!("text"))?;
        element.SetInnerText(&text.as_ref().into())?;
        binding.AppendChild(&element)?;
    }

    if !notification.actions.is_empty() {
        let actions = document.CreateElement(h!("actions"))?;
        toast.AppendChild(&actions)?;
        for action in &notification.actions {
            let action_element = document.CreateElement(h!("action"))?;
            action_element.SetAttribute(h!("content"), &action.label.as_ref().into())?;
            action_element.SetAttribute(h!("arguments"), &action.id.as_ref().into())?;
            actions.AppendChild(&action_element)?;
        }
    }

    let audio = document.CreateElement(h!("audio"))?;
    audio.SetAttribute(h!("silent"), h!("true"))?;
    toast.AppendChild(&audio)?;
    Ok(document)
}

/// Registers the app's AUMID so toasts display correctly for an unpackaged app without a Start Menu shortcut.
fn register_app_user_model_id(app_identifier: &str, app_name: &str) {
    let result = windows_registry::CURRENT_USER
        .create(format!(r"Software\Classes\AppUserModelId\{app_identifier}"))
        .and_then(|key| key.set_string("DisplayName", app_name));
    if let Err(error) = result {
        log::warn!("failed to register AppUserModelID; notifications may not display: {error}");
    }
}
