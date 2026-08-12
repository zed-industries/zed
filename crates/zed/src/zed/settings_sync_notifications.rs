use gpui::{App, AppContext as _, TaskExt as _};
use settings_sync::SettingsSyncEvent;
use util::ResultExt as _;
use workspace::notifications::{
    NotificationId, show_app_notification, simple_message_notification::MessageNotification,
};

struct SettingsSyncConflictsNotification;
struct SettingsSyncPausedNotification;
struct SettingsSyncUpdateRequiredNotification;

pub fn init(cx: &mut App) {
    let Some(engine) = settings_sync::engine(cx) else {
        return;
    };
    let mut conflict_batch_id = 0_usize;
    cx.subscribe(&engine, move |engine, event, cx| match event {
        SettingsSyncEvent::ConflictsResolved(conflicts) => {
            conflict_batch_id += 1;
            let message = if conflicts.len() == 1 {
                format!(
                    "Settings sync kept the remote value for {}",
                    conflicts[0].path
                )
            } else {
                format!(
                    "Settings sync kept the remote values for {} conflicting settings",
                    conflicts.len()
                )
            };
            let conflicts = conflicts.clone();
            let engine = engine.downgrade();
            show_app_notification(
                NotificationId::composite::<SettingsSyncConflictsNotification>(conflict_batch_id),
                cx,
                move |cx| {
                    let conflicts = conflicts.clone();
                    let engine = engine.clone();
                    cx.new(|cx| {
                        MessageNotification::new(message.clone(), cx)
                            .primary_message("Use My Values")
                            .primary_on_click(move |_, cx| {
                                let conflicts = conflicts.clone();
                                engine
                                    .update(cx, |engine, cx| {
                                        engine.revert_conflicts(conflicts, cx).detach_and_log_err(cx)
                                    })
                                    .log_err();
                            })
                    })
                },
            );
        }
        SettingsSyncEvent::Paused => {
            let engine = engine.downgrade();
            show_app_notification(
                NotificationId::unique::<SettingsSyncPausedNotification>(),
                cx,
                move |cx| {
                    let engine = engine.clone();
                    cx.new(|cx| {
                        MessageNotification::new(
                            "Settings sync is paused after repeated conflicting writes",
                            cx,
                        )
                        .primary_message("Retry")
                        .primary_on_click(move |_, cx| {
                            engine
                                .update(cx, |engine, cx| engine.unpause(cx))
                                .log_err();
                        })
                    })
                },
            );
        }
        SettingsSyncEvent::UpdateRequired => {
            show_app_notification(
                NotificationId::unique::<SettingsSyncUpdateRequiredNotification>(),
                cx,
                move |cx| {
                    cx.new(|cx| {
                        MessageNotification::new(
                            "Settings were synced from a newer Zed; update Zed to sync your own changes",
                            cx,
                        )
                    })
                },
            );
        }
    })
    .detach();
}
