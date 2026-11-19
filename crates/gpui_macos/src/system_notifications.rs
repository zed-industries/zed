//! System notifications through the `UNUserNotificationCenter` API.
//!
//! Everything here runs lazily: nothing touches the notification center (and
//! so nothing can trigger the authorization prompt or the framework's
//! not-in-a-bundle abort) until the application posts a notification or
//! registers a response callback.
//!
//! Notification-center calls can synchronously wait for the macOS service.
//! A single worker owns the center so those waits never block the UI, while
//! preserving the order of posting and dismissing notifications.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::{Sender, channel};

use block2::RcBlock;
use futures::StreamExt as _;
use futures::channel::{mpsc, oneshot};
use gpui::{
    BackgroundExecutor, ForegroundExecutor, SharedString, SystemNotification,
    SystemNotificationAction, SystemNotificationResponse, Task,
};
use objc2::rc::{Retained, autoreleasepool};
use objc2::runtime::{Bool, ProtocolObject};
use objc2::{AnyThread, DefinedClass, define_class, msg_send};
use objc2_foundation::{NSArray, NSBundle, NSError, NSObject, NSObjectProtocol, NSSet, NSString};
use objc2_user_notifications::{
    UNAuthorizationOptions, UNMutableNotificationContent, UNNotification, UNNotificationAction,
    UNNotificationActionOptions, UNNotificationCategory, UNNotificationCategoryOptions,
    UNNotificationDefaultActionIdentifier, UNNotificationPresentationOptions,
    UNNotificationRequest, UNNotificationResponse, UNUserNotificationCenter,
    UNUserNotificationCenterDelegate,
};

type ResponseCallback = Rc<RefCell<Option<Box<dyn FnMut(SystemNotificationResponse)>>>>;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) enum NotificationCommand {
    Show(SystemNotification),
    Dismiss(SharedString),
}

pub(crate) struct SystemNotificationState {
    initialized: bool,
    commands: Option<Sender<(NotificationCommand, oneshot::Sender<()>)>>,
    callback: ResponseCallback,
    /// The center's `delegate` property is weak; keeping the delegate retained
    /// here keeps response handling alive for the app's lifetime.
    _delegate: Option<Retained<NotificationResponseDelegate>>,
    _response_task: Option<Task<()>>,
}

impl SystemNotificationState {
    pub(crate) fn new() -> Self {
        Self {
            initialized: false,
            commands: None,
            callback: Rc::new(RefCell::new(None)),
            _delegate: None,
            _response_task: None,
        }
    }

    pub(crate) fn send(
        &mut self,
        foreground_executor: &ForegroundExecutor,
        background_executor: &BackgroundExecutor,
        command: NotificationCommand,
    ) -> Task<()> {
        self.initialize(foreground_executor);
        let Some(commands) = &self.commands else {
            return Task::ready(());
        };
        let (completed, completion) = oneshot::channel();
        if let Err(error) = commands.send((command, completed)) {
            log::error!("system notification worker disconnected: {error}");
            return Task::ready(());
        }
        background_executor.spawn(async move {
            if let Err(error) = completion.await {
                log::error!(
                    "system notification worker stopped before completing a command: {error}"
                );
            }
        })
    }

    pub(crate) fn on_response(
        &mut self,
        executor: &ForegroundExecutor,
        callback: Box<dyn FnMut(SystemNotificationResponse)>,
    ) {
        self.initialize(executor);
        *self.callback.borrow_mut() = Some(callback);
    }

    fn initialize(&mut self, executor: &ForegroundExecutor) {
        if self.initialized {
            return;
        }
        self.initialized = true;

        // UserNotifications aborts outside an app bundle. Checking the bundle
        // does not contact the notification service.
        if NSBundle::mainBundle().bundleIdentifier().is_none() {
            log::info!("system notifications disabled: not running from an app bundle");
            return;
        }

        let (sender, mut receiver) = mpsc::unbounded();
        // Install the delegate before returning so a response delivered right
        // after registration (e.g. the click that launched the app) has
        // somewhere to go. Setting the delegate doesn't wait on the
        // notification service, unlike the calls handled by the worker.
        let delegate = autoreleasepool(|_| {
            let delegate = NotificationResponseDelegate::new(sender);
            UNUserNotificationCenter::currentNotificationCenter()
                .setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
            delegate
        });
        self._delegate = Some(delegate);

        let (commands, pending_commands) = channel::<(NotificationCommand, oneshot::Sender<()>)>();
        let worker = std::thread::Builder::new()
            .name("system-notifications".into())
            .spawn(move || {
                let mut center = autoreleasepool(|_| NotificationCenter::new());
                for (command, completed) in pending_commands {
                    autoreleasepool(|_| match command {
                        NotificationCommand::Show(notification) => center.show(notification),
                        NotificationCommand::Dismiss(tag) => center.dismiss(&tag),
                    });
                    // The caller may stop waiting without cancelling the operation.
                    _ = completed.send(());
                }
            });
        if let Err(error) = worker {
            log::error!("failed to start system notification worker: {error}");
            return;
        }
        self.commands = Some(commands);

        // Responses arrive from the delegate on an arbitrary thread; this task
        // hands them to the registered callback on the main thread.
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

struct NotificationCenter {
    center: Retained<UNUserNotificationCenter>,
    /// The union of every action set registered so far. Categories are shared
    /// by notifications with identical actions because macOS replaces the
    /// entire registered set whenever a category is added.
    categories:
        HashMap<Vec<SystemNotificationAction>, (SharedString, Retained<UNNotificationCategory>)>,
    authorization_requested: bool,
}

impl NotificationCenter {
    fn new() -> Self {
        Self {
            center: UNUserNotificationCenter::currentNotificationCenter(),
            categories: HashMap::new(),
            authorization_requested: false,
        }
    }

    fn request_authorization(&mut self) {
        if self.authorization_requested {
            return;
        }
        self.authorization_requested = true;
        let completion = RcBlock::new(|granted: Bool, error: *mut NSError| {
            // SAFETY: when non-null, `error` is a valid `NSError` for the
            // duration of the callback.
            if let Some(error) = unsafe { error.as_ref() } {
                log::warn!(
                    "system notification authorization failed: {}",
                    error.localizedDescription()
                );
            } else if !granted.as_bool() {
                log::info!("system notification authorization denied");
            }
        });
        self.center
            .requestAuthorizationWithOptions_completionHandler(
                UNAuthorizationOptions::Alert | UNAuthorizationOptions::Sound,
                &completion,
            );
    }

    fn show(&mut self, notification: SystemNotification) {
        self.request_authorization();

        let content = UNMutableNotificationContent::new();
        content.setTitle(&NSString::from_str(&notification.title));
        content.setBody(&NSString::from_str(&notification.body));
        if !notification.actions.is_empty() {
            let category_identifier = self.register_category(&notification.actions);
            content.setCategoryIdentifier(&NSString::from_str(&category_identifier));
        }

        // A nil trigger delivers immediately. Reusing the tag as the request
        // identifier makes a newer notification for the same tag replace the
        // older one.
        let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
            &NSString::from_str(&notification.tag),
            &content,
            None,
        );
        let completion = RcBlock::new(|error: *mut NSError| {
            // SAFETY: when non-null, `error` is a valid `NSError` for the
            // duration of the callback.
            if let Some(error) = unsafe { error.as_ref() } {
                log::warn!(
                    "failed to deliver system notification: {}",
                    error.localizedDescription()
                );
            }
        });
        self.center
            .addNotificationRequest_withCompletionHandler(&request, Some(&completion));
    }

    fn register_category(&mut self, actions: &[SystemNotificationAction]) -> SharedString {
        if let Some((identifier, _category)) = self.categories.get(actions) {
            return identifier.clone();
        }

        let identifier = SharedString::from(format!(
            "gpui-system-notification-{}",
            self.categories.len()
        ));
        let platform_actions: Vec<Retained<UNNotificationAction>> = actions
            .iter()
            .map(|action| {
                UNNotificationAction::actionWithIdentifier_title_options(
                    &NSString::from_str(&action.id),
                    &NSString::from_str(&action.label),
                    UNNotificationActionOptions::empty(),
                )
            })
            .collect();
        let category =
            UNNotificationCategory::categoryWithIdentifier_actions_intentIdentifiers_options(
                &NSString::from_str(&identifier),
                &NSArray::from_retained_slice(&platform_actions),
                &NSArray::new(),
                UNNotificationCategoryOptions::empty(),
            );

        self.categories
            .insert(actions.to_vec(), (identifier.clone(), category));
        let all: Vec<Retained<UNNotificationCategory>> = self
            .categories
            .values()
            .map(|(_identifier, category)| category.clone())
            .collect();
        self.center
            .setNotificationCategories(&NSSet::from_retained_slice(&all));
        identifier
    }

    fn dismiss(&self, tag: &str) {
        let identifiers = NSArray::from_retained_slice(&[NSString::from_str(tag)]);
        self.center
            .removePendingNotificationRequestsWithIdentifiers(&identifiers);
        self.center
            .removeDeliveredNotificationsWithIdentifiers(&identifiers);
    }
}

struct DelegateIvars {
    sender: mpsc::UnboundedSender<SystemNotificationResponse>,
}

define_class!(
    // SAFETY: `NSObject` has no subclassing requirements and
    // `NotificationResponseDelegate` does not implement `Drop`.
    #[unsafe(super(NSObject))]
    #[ivars = DelegateIvars]
    struct NotificationResponseDelegate;

    unsafe impl NSObjectProtocol for NotificationResponseDelegate {}

    unsafe impl UNUserNotificationCenterDelegate for NotificationResponseDelegate {
        // Called when the user activates a delivered notification, possibly on
        // a non-main thread; the channel hop hands the response to the
        // foreground task holding the receiver.
        #[unsafe(method(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:))]
        fn did_receive_notification_response(
            &self,
            _center: &UNUserNotificationCenter,
            response: &UNNotificationResponse,
            completion_handler: &block2::DynBlock<dyn Fn()>,
        ) {
            let tag = response.notification().request().identifier().to_string();
            let action = response.actionIdentifier();
            // The other well-known identifier, `UNNotificationDismissActionIdentifier`,
            // is only delivered for categories opting into dismiss callbacks,
            // which we never request.
            let action_id = if &*action == unsafe { UNNotificationDefaultActionIdentifier } {
                None
            } else {
                Some(SharedString::from(action.to_string()))
            };
            self.ivars()
                .sender
                .unbounded_send(SystemNotificationResponse {
                    tag: SharedString::from(tag),
                    action_id,
                })
                .ok();
            completion_handler.call(());
        }

        // Without this, macOS suppresses banners while the app is frontmost.
        // Posting is the application's decision; whether the app is focused is
        // its criterion to apply, not the platform's.
        #[unsafe(method(userNotificationCenter:willPresentNotification:withCompletionHandler:))]
        fn will_present_notification(
            &self,
            _center: &UNUserNotificationCenter,
            _notification: &UNNotification,
            completion_handler: &block2::DynBlock<dyn Fn(UNNotificationPresentationOptions)>,
        ) {
            completion_handler
                .call((UNNotificationPresentationOptions::Banner
                    | UNNotificationPresentationOptions::List,));
        }
    }
);

impl NotificationResponseDelegate {
    fn new(sender: mpsc::UnboundedSender<SystemNotificationResponse>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(DelegateIvars { sender });
        // SAFETY: `NSObject`'s `init` is its designated initializer.
        unsafe { msg_send![super(this), init] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt as _;

    #[gpui::test]
    async fn test_notification_commands_do_not_wait_for_the_service(cx: &mut gpui::TestAppContext) {
        let (sender, receiver) = channel();
        let mut state = SystemNotificationState {
            initialized: true,
            commands: Some(sender),
            ..SystemNotificationState::new()
        };
        let notification = SystemNotification {
            tag: "thread".into(),
            title: "Finished".into(),
            body: "Ready for input".into(),
            actions: vec![],
        };

        // No service consumes commands until after every UI call has returned.
        let mut tasks = [
            state.send(
                &cx.foreground_executor,
                &cx.background_executor,
                NotificationCommand::Show(notification.clone()),
            ),
            state.send(
                &cx.foreground_executor,
                &cx.background_executor,
                NotificationCommand::Dismiss("thread".into()),
            ),
            state.send(
                &cx.foreground_executor,
                &cx.background_executor,
                NotificationCommand::Show(notification.clone()),
            ),
        ];
        for task in &mut tasks {
            assert!(task.now_or_never().is_none());
        }

        let (commands, completions): (Vec<_>, Vec<_>) = receiver.try_iter().unzip();
        assert_eq!(
            commands,
            vec![
                NotificationCommand::Show(notification.clone()),
                NotificationCommand::Dismiss("thread".into()),
                NotificationCommand::Show(notification),
            ]
        );
        for completed in completions {
            completed
                .send(())
                .expect("caller is waiting for completion");
        }
        for task in tasks {
            task.await;
        }
    }
}
