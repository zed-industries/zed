use anyhow::{Result, anyhow};
use gpui::{
    BackgroundExecutor, SystemNotification, SystemNotificationId, SystemNotificationPermission,
    SystemNotificationPriority, Task,
};
use uuid::Uuid;
use windows::{
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::{
        NotificationSetting, ToastNotification, ToastNotificationManager, ToastNotificationPriority,
    },
    Win32::{
        Foundation::{E_FAIL, ERROR_FILE_NOT_FOUND},
        System::Com::CoTaskMemFree,
        UI::Shell::GetCurrentProcessExplicitAppUserModelID,
    },
    core::HSTRING,
};

const NOTIFICATION_GROUP: &str = "gpui.system-notification";
const NOTIFICATION_NAMESPACE: Uuid = Uuid::from_u128(0x341d32df_a3b8_48a5_93c5_d5cd6305eb75);
const TAG_LENGTH: usize = 16;

pub(crate) fn system_notification_permission(
    executor: BackgroundExecutor,
    headless: bool,
) -> Task<Result<SystemNotificationPermission>> {
    if headless {
        return Task::ready(Ok(SystemNotificationPermission::Unsupported));
    }

    executor.spawn(async move {
        let Some(app_user_model_id) = current_app_user_model_id()? else {
            return Ok(SystemNotificationPermission::Unsupported);
        };

        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&app_user_model_id)?;
        Ok(permission_from_notification_setting(notifier.Setting()?))
    })
}

pub(crate) fn request_system_notification_permission(
    executor: BackgroundExecutor,
    headless: bool,
) -> Task<Result<SystemNotificationPermission>> {
    system_notification_permission(executor, headless)
}

pub(crate) fn show_system_notification(
    executor: BackgroundExecutor,
    headless: bool,
    notification: SystemNotification,
) -> Task<Result<()>> {
    if headless {
        return Task::ready(Err(anyhow!(
            "system notifications are not supported in headless mode"
        )));
    }

    executor.spawn(async move {
        let Some(app_user_model_id) = current_app_user_model_id()? else {
            return Err(anyhow!(
                "Windows system notifications require a process AppUserModelID"
            ));
        };

        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&app_user_model_id)?;
        let toast = toast_notification(notification)?;
        notifier.Show(&toast)?;
        Ok(())
    })
}

pub(crate) fn remove_system_notification(
    executor: BackgroundExecutor,
    headless: bool,
    id: SystemNotificationId,
) -> Task<Result<()>> {
    if headless {
        return Task::ready(Ok(()));
    }

    executor.spawn(async move {
        let Some(app_user_model_id) = current_app_user_model_id()? else {
            return Ok(());
        };

        let tag = HSTRING::from(notification_tag(&id));
        let group = HSTRING::from(NOTIFICATION_GROUP);
        ToastNotificationManager::History()?.RemoveGroupedTagWithId(
            &tag,
            &group,
            &app_user_model_id,
        )?;
        Ok(())
    })
}

fn current_app_user_model_id() -> Result<Option<HSTRING>> {
    let app_user_model_id = unsafe {
        match GetCurrentProcessExplicitAppUserModelID() {
            Ok(app_user_model_id) => app_user_model_id,
            Err(error) if missing_app_user_model_id(&error) => return Ok(None),
            Err(error) => return Err(error.into()),
        }
    };

    if app_user_model_id.is_null() {
        return Ok(None);
    }

    let app_user_model_id_string = unsafe { app_user_model_id.to_string() };
    unsafe {
        CoTaskMemFree(Some(app_user_model_id.0 as _));
    }

    let app_user_model_id_string = app_user_model_id_string?;
    if app_user_model_id_string.is_empty() {
        Ok(None)
    } else {
        Ok(Some(HSTRING::from(app_user_model_id_string)))
    }
}

fn missing_app_user_model_id(error: &windows::core::Error) -> bool {
    error.code() == ERROR_FILE_NOT_FOUND.to_hresult() || error.code() == E_FAIL
}

fn toast_notification(notification: SystemNotification) -> Result<ToastNotification> {
    let tag = HSTRING::from(notification_tag(&notification.id));
    let group = HSTRING::from(NOTIFICATION_GROUP);
    let priority = toast_priority(notification.priority);
    let content = toast_xml(notification)?;
    let toast = ToastNotification::CreateToastNotification(&content)?;
    toast.SetTag(&tag)?;
    toast.SetGroup(&group)?;
    if let Some(priority) = priority {
        toast.SetPriority(priority)?;
    }
    Ok(toast)
}

fn toast_xml(notification: SystemNotification) -> Result<XmlDocument> {
    let document = XmlDocument::new()?;
    let toast = document.CreateElement(&HSTRING::from("toast"))?;
    let visual = document.CreateElement(&HSTRING::from("visual"))?;
    let binding = document.CreateElement(&HSTRING::from("binding"))?;
    binding.SetAttribute(&HSTRING::from("template"), &HSTRING::from("ToastGeneric"))?;

    append_text_node(&document, &binding, notification.title.as_ref())?;
    if let Some(body) = notification.body {
        append_text_node(&document, &binding, body.as_ref())?;
    }

    visual.AppendChild(&binding)?;
    toast.AppendChild(&visual)?;
    document.AppendChild(&toast)?;
    Ok(document)
}

fn append_text_node(
    document: &XmlDocument,
    binding: &windows::Data::Xml::Dom::XmlElement,
    value: &str,
) -> Result<()> {
    let text = document.CreateElement(&HSTRING::from("text"))?;
    text.SetInnerText(&HSTRING::from(value))?;
    binding.AppendChild(&text)?;
    Ok(())
}

fn notification_tag(id: &SystemNotificationId) -> String {
    Uuid::new_v5(&NOTIFICATION_NAMESPACE, id.0.as_bytes())
        .simple()
        .to_string()
        .chars()
        .take(TAG_LENGTH)
        .collect()
}

fn toast_priority(priority: SystemNotificationPriority) -> Option<ToastNotificationPriority> {
    match priority {
        SystemNotificationPriority::Low | SystemNotificationPriority::Normal => None,
        SystemNotificationPriority::High | SystemNotificationPriority::Urgent => {
            Some(ToastNotificationPriority::High)
        }
    }
}

fn permission_from_notification_setting(
    setting: NotificationSetting,
) -> SystemNotificationPermission {
    if setting == NotificationSetting::Enabled {
        SystemNotificationPermission::Granted
    } else if setting == NotificationSetting::DisabledForApplication
        || setting == NotificationSetting::DisabledForUser
        || setting == NotificationSetting::DisabledByGroupPolicy
        || setting == NotificationSetting::DisabledByManifest
    {
        SystemNotificationPermission::Denied
    } else {
        SystemNotificationPermission::Unsupported
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::SharedString;

    #[test]
    fn notification_tag_is_deterministic_and_length_limited() {
        let id = SystemNotificationId(SharedString::from(
            "a-very-long-notification-id-that-exceeds-windows-tag-limits",
        ));
        let tag = notification_tag(&id);

        assert_eq!(tag, notification_tag(&id));
        assert_eq!(tag.len(), TAG_LENGTH);
        assert!(tag.chars().all(|character| character.is_ascii_hexdigit()));
    }

    #[test]
    fn notification_tag_distinguishes_ids() {
        let first = SystemNotificationId(SharedString::from("first"));
        let second = SystemNotificationId(SharedString::from("second"));

        assert_ne!(notification_tag(&first), notification_tag(&second));
    }

    #[test]
    fn maps_notification_priority() {
        assert_eq!(toast_priority(SystemNotificationPriority::Low), None);
        assert_eq!(toast_priority(SystemNotificationPriority::Normal), None);
        assert_eq!(
            toast_priority(SystemNotificationPriority::High),
            Some(ToastNotificationPriority::High)
        );
        assert_eq!(
            toast_priority(SystemNotificationPriority::Urgent),
            Some(ToastNotificationPriority::High)
        );
    }

    #[test]
    fn maps_notification_setting() {
        assert_eq!(
            permission_from_notification_setting(NotificationSetting::Enabled),
            SystemNotificationPermission::Granted
        );
        assert_eq!(
            permission_from_notification_setting(NotificationSetting::DisabledForApplication),
            SystemNotificationPermission::Denied
        );
        assert_eq!(
            permission_from_notification_setting(NotificationSetting::DisabledForUser),
            SystemNotificationPermission::Denied
        );
        assert_eq!(
            permission_from_notification_setting(NotificationSetting::DisabledByGroupPolicy),
            SystemNotificationPermission::Denied
        );
        assert_eq!(
            permission_from_notification_setting(NotificationSetting::DisabledByManifest),
            SystemNotificationPermission::Denied
        );
        assert_eq!(
            permission_from_notification_setting(NotificationSetting(i32::MAX)),
            SystemNotificationPermission::Unsupported
        );
    }

    #[test]
    fn maps_missing_app_user_model_id_errors() {
        assert!(missing_app_user_model_id(&windows::core::Error::from(
            ERROR_FILE_NOT_FOUND.to_hresult()
        )));
        assert!(missing_app_user_model_id(&windows::core::Error::from(
            E_FAIL
        )));
    }
}
