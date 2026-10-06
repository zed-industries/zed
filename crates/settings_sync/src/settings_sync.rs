mod classifier;
mod engine;
mod merge;
mod server;
mod strategy;
mod sync_path;

#[cfg(test)]
mod settings_sync_tests;

use std::sync::Arc;

use client::Client;
use cloud_api_client::websocket_protocol::MessageToClient;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, Entity, Global};
use settings::SettingsStore;

pub use classifier::{ContainerView, DocumentClassifier};
pub use engine::{
    ConflictBatch, FirstSyncChoice, FirstSyncPreview, HISTORY_LIMIT, PauseReason, PendingDecision,
    SettingsSyncEngine, SettingsSyncEvent, SettingsSyncStatus, SyncActivity, SyncBase, SyncState,
    SyncedDocument, SyncedSettingsHistory, SyncedSettingsHosts,
};
pub use merge::{
    Conflict, ExclusionPattern, ExclusionSet, PathMap, SyncOp, ThreeWayMerge, apply_ops_to_text,
    apply_target_to_text, diff_paths, flatten_doc, merge_three_way, unflatten, value_at_path,
};
pub use server::{
    CloudSettingsSyncServer, FetchResult, PushResult, SettingsSyncServer, SyncNotImplementedError,
};
#[cfg(any(test, feature = "test-support"))]
pub use server::{FAKE_ACCOUNT_ID, FakeSettingsSyncBackend, FakeSettingsSyncServer};
pub use strategy::{SettingsJsonStrategy, SyncStrategy};
pub use sync_path::SyncPath;

const NON_MIGRATION_SCHEMA_EPOCH_BUMPS: u64 = 5;

pub fn settings_schema_epoch() -> u64 {
    migrator::settings_migrations_count() as u64 + NON_MIGRATION_SCHEMA_EPOCH_BUMPS
}

struct GlobalSettingsSyncEngine(Entity<SettingsSyncEngine>);

impl Global for GlobalSettingsSyncEngine {}

pub fn init(client: &Arc<Client>, fs: Arc<dyn Fs>, cx: &mut App) {
    let server = Arc::new(CloudSettingsSyncServer::new(client.clone()));
    let engine = cx.new(|cx| {
        SettingsSyncEngine::new(
            server,
            fs,
            SyncedDocument::user_settings(),
            paths::settings_sync_dir().join("state.json"),
            cx,
        )
    });

    client.add_message_to_client_handler({
        let engine = engine.downgrade();
        move |message, cx| match message {
            MessageToClient::SyncedSettingsChanged {
                group_id,
                kind,
                version,
                document_id,
            } => {
                engine
                    .update(cx, |engine, cx| {
                        engine.handle_remote_changed(group_id, kind, *version, document_id, cx)
                    })
                    .ok();
            }
            MessageToClient::SyncedSettingsInvalidated => {
                engine
                    .update(cx, |engine, cx| engine.handle_remote_invalidated(cx))
                    .ok();
            }
            MessageToClient::UserUpdated
            | MessageToClient::NotificationsUpdated
            | MessageToClient::SettingsUpdated => {}
        }
    });

    let mut status = client.status();
    cx.spawn({
        let engine = engine.downgrade();
        async move |cx| {
            while let Some(status) = status.next().await {
                if status.is_connected()
                    && engine
                        .update(cx, |engine, cx| engine.schedule_sync(cx))
                        .is_err()
                {
                    break;
                }
            }
        }
    })
    .detach();

    let mut cloud_connection_id = client.cloud_connection_id();
    cx.spawn({
        let engine = engine.downgrade();
        async move |cx| {
            while let Some(connection_id) = cloud_connection_id.next().await {
                if connection_id > 0
                    && engine
                        .update(cx, |engine, cx| engine.schedule_sync(cx))
                        .is_err()
                {
                    break;
                }
            }
        }
    })
    .detach();

    let mut was_enabled = sync_enabled(cx);
    cx.observe_global::<SettingsStore>({
        let engine = engine.downgrade();
        move |cx| {
            let enabled = sync_enabled(cx);
            if enabled != was_enabled {
                was_enabled = enabled;
                engine
                    .update(cx, |engine, cx| {
                        if enabled {
                            engine.unpause(cx);
                        } else {
                            engine.clear_pending_decision(cx);
                        }
                    })
                    .ok();
            }
        }
    })
    .detach();

    cx.set_global(GlobalSettingsSyncEngine(engine));
}

pub fn engine(cx: &App) -> Option<Entity<SettingsSyncEngine>> {
    cx.try_global::<GlobalSettingsSyncEngine>()
        .map(|global| global.0.clone())
}

pub fn default_host_label() -> String {
    sysinfo::System::host_name()
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "This device".to_string())
}

pub(crate) fn sync_enabled(cx: &App) -> bool {
    cx.global::<SettingsStore>()
        .raw_user_settings()
        .and_then(|settings| settings.content.settings_sync.as_ref()?.enabled)
        .unwrap_or(false)
}
