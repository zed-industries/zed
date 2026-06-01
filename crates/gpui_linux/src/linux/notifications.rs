use anyhow::{Result, anyhow};
use ashpd::desktop::notification::{
    Notification, NotificationProxy, Priority as PortalNotificationPriority,
};
use gpui::{
    BackgroundExecutor, SystemNotification, SystemNotificationId, SystemNotificationPermission,
    SystemNotificationPriority, Task,
};

pub(super) fn system_notification_permission(
    executor: BackgroundExecutor,
) -> Task<Result<SystemNotificationPermission>> {
    executor.spawn(async move {
        match NotificationProxy::new().await {
            Ok(_) => Ok(SystemNotificationPermission::Granted),
            Err(ashpd::Error::PortalNotFound(_)) => Ok(SystemNotificationPermission::Unsupported),
            Err(error) => Err(error.into()),
        }
    })
}

pub(super) fn request_system_notification_permission(
    executor: BackgroundExecutor,
) -> Task<Result<SystemNotificationPermission>> {
    system_notification_permission(executor)
}

pub(super) fn show_system_notification(
    executor: BackgroundExecutor,
    notification: SystemNotification,
) -> Task<Result<()>> {
    executor.spawn(async move {
        let proxy = NotificationProxy::new()
            .await
            .map_err(map_notification_portal_error)?;
        let id = notification.id.0.clone();
        let portal_notification = portal_notification(notification);
        proxy
            .add_notification(id.as_str(), portal_notification)
            .await
            .map_err(map_notification_portal_error)
    })
}

pub(super) fn remove_system_notification(
    executor: BackgroundExecutor,
    id: SystemNotificationId,
) -> Task<Result<()>> {
    executor.spawn(async move {
        let proxy = NotificationProxy::new()
            .await
            .map_err(map_notification_portal_error)?;
        proxy
            .remove_notification(id.0.as_str())
            .await
            .map_err(map_notification_portal_error)
    })
}

fn portal_notification(notification: SystemNotification) -> Notification {
    let mut portal_notification = Notification::new(notification.title.as_str())
        .priority(portal_priority(notification.priority));

    if let Some(body) = notification.body {
        portal_notification = portal_notification.body(body.as_str());
    }

    portal_notification
}

fn portal_priority(priority: SystemNotificationPriority) -> PortalNotificationPriority {
    match priority {
        SystemNotificationPriority::Low => PortalNotificationPriority::Low,
        SystemNotificationPriority::Normal => PortalNotificationPriority::Normal,
        SystemNotificationPriority::High => PortalNotificationPriority::High,
        SystemNotificationPriority::Urgent => PortalNotificationPriority::Urgent,
    }
}

fn map_notification_portal_error(error: ashpd::Error) -> anyhow::Error {
    match error {
        ashpd::Error::PortalNotFound(_) => {
            anyhow!("system notifications are not supported by xdg-desktop-portal")
        }
        error => error.into(),
    }
}
