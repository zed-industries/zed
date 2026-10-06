use gpui::{Action as _, App, AppContext as _, Context, Entity, WeakEntity, Window};
use settings_sync::{Conflict, ConflictBatch, PauseReason, SettingsSyncEngine, SettingsSyncEvent};
use workspace::notifications::{
    NotificationId, NotifyResultExt as _, show_app_notification,
    simple_message_notification::MessageNotification,
};

use super::settings_sync_ui::{self, format_sync_value};

pub fn init(cx: &mut App) {
    let Some(engine) = settings_sync::engine(cx) else {
        return;
    };
    settings_sync_ui::init(cx);
    let mut conflict_batch_id = 0_usize;
    cx.subscribe(&engine, move |engine, event, cx| match event {
        SettingsSyncEvent::ConflictsResolved(batch) => {
            conflict_batch_id += 1;
            show_conflicts_notification(engine.downgrade(), batch.clone(), conflict_batch_id, cx);
        }
        SettingsSyncEvent::SyncFailed(error) => {
            let engine = engine.downgrade();
            let message = format!("Settings sync failed: {error}");
            show_app_notification(
                NotificationId::unique::<SettingsSyncFailedNotification>(),
                cx,
                move |cx| {
                    let engine = engine.clone();
                    cx.new(|cx| {
                        MessageNotification::new(message.clone(), cx)
                            .primary_message("Retry")
                            .primary_on_click(move |_, cx| unpause(&engine, cx))
                            .secondary_message("Open Status")
                            .secondary_on_click(|window, cx| {
                                window.dispatch_action(settings_sync_ui::OpenStatus.boxed_clone(), cx);
                            })
                    })
                },
            );
        }
        SettingsSyncEvent::Paused(PauseReason::RepeatedConflicts) => {
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
                        .primary_on_click(move |_, cx| unpause(&engine, cx))
                    })
                },
            );
        }
        SettingsSyncEvent::Paused(PauseReason::ApplyFailed) => {
            let engine = engine.downgrade();
            show_app_notification(
                NotificationId::unique::<SettingsSyncPausedNotification>(),
                cx,
                move |cx| {
                    let engine = engine.clone();
                    cx.new(|cx| {
                        MessageNotification::new(
                            "Settings sync is paused: synced changes could not be applied safely \
                             to settings.json, for example because it has duplicate keys. \
                             Fix the file, then retry.",
                            cx,
                        )
                        .primary_message("Open Settings")
                        .primary_on_click(|window, cx| {
                            window.dispatch_action(zed_actions::OpenSettingsFile.boxed_clone(), cx);
                        })
                        .secondary_message("Retry")
                        .secondary_on_click(move |_, cx| unpause(&engine, cx))
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
        SettingsSyncEvent::DecisionRequired => {
            settings_sync_ui::show_decision_modal(engine, cx);
        }
        SettingsSyncEvent::Erased => {
            show_app_notification(
                NotificationId::unique::<SettingsSyncErasedNotification>(),
                cx,
                move |cx| {
                    cx.new(|cx| {
                        MessageNotification::new(
                            "Synced settings erased from Zed Cloud; settings sync is now off on this device",
                            cx,
                        )
                    })
                },
            );
        }
    })
    .detach();
}

struct SettingsSyncConflictsNotification;
struct SettingsSyncPausedNotification;
struct SettingsSyncUpdateRequiredNotification;
struct SettingsSyncErasedNotification;
struct SettingsSyncFailedNotification;

fn show_conflicts_notification(
    engine: WeakEntity<SettingsSyncEngine>,
    batch: ConflictBatch,
    batch_id: usize,
    cx: &mut App,
) {
    let message = conflicts_message(&batch.conflicts);
    let single_conflict = batch.conflicts.len() == 1;
    show_app_notification(
        NotificationId::composite::<SettingsSyncConflictsNotification>(batch_id),
        cx,
        move |cx| {
            let batch = batch.clone();
            let engine = engine.clone();
            cx.new(|cx| {
                let notification = MessageNotification::new(message.clone(), cx);
                let use_mine = move |_: &mut Window, cx: &mut Context<MessageNotification>| {
                    if let Some(engine) = engine.upgrade() {
                        revert_conflicts(engine, batch.clone(), cx);
                    }
                };
                if single_conflict {
                    notification
                        .primary_message("Use Mine")
                        .primary_on_click(use_mine)
                } else {
                    notification
                        .primary_message("Review")
                        .primary_on_click(|window, cx| {
                            window.dispatch_action(settings_sync_ui::OpenStatus.boxed_clone(), cx);
                        })
                        .secondary_message("Use Mine for All")
                        .secondary_on_click(use_mine)
                }
            })
        },
    );
}

fn conflicts_message(conflicts: &[Conflict]) -> String {
    let [conflict] = conflicts else {
        let count = conflicts.len();
        return format!("Settings sync kept the synced values for {count} conflicting settings");
    };
    let path = &conflict.path;
    let kept = match &conflict.remote {
        Some(value) => {
            let value = format_sync_value(value);
            format!("Settings sync kept {value} for {path}")
        }
        None => format!("Settings sync removed {path}"),
    };
    let changed_by = conflict
        .changed_by
        .as_ref()
        .map(|host| format!(" (changed on {host})"))
        .unwrap_or_default();
    let yours = match &conflict.local {
        Some(value) => {
            let value = format_sync_value(value);
            format!("yours was {value}")
        }
        None => String::from("you had it unset"),
    };
    format!("{kept}{changed_by}; {yours}")
}

fn revert_conflicts(engine: Entity<SettingsSyncEngine>, batch: ConflictBatch, cx: &mut App) {
    let task = engine.update(cx, |engine, cx| engine.revert_conflicts(batch, cx));
    cx.spawn(async move |cx| {
        let result = task.await;
        cx.update(|cx| result.notify_app_err(cx));
    })
    .detach();
}

fn unpause(engine: &WeakEntity<SettingsSyncEngine>, cx: &mut App) {
    engine.update(cx, |engine, cx| engine.unpause(cx)).ok();
}
