mod classifier;
mod engine;
mod merge;
mod server;
mod sync_path;

#[cfg(test)]
mod settings_sync_tests;

use std::sync::Arc;

use client::Client;
use cloud_api_client::websocket_protocol::MessageToClient;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, Entity, Global};
use settings::{RegisterSetting, Settings, SettingsStore};

pub use classifier::{ContainerView, DocumentClassifier};
pub use engine::{SettingsSyncEngine, SettingsSyncEvent, SyncBase, SyncState, SyncedDocument};
pub use merge::{
    Conflict, ExclusionPattern, ExclusionSet, PathMap, SyncOp, ThreeWayMerge, apply_ops_to_text,
    diff_paths, drop_prefix_overlaps, flatten_doc, merge_three_way, unflatten, value_at_path,
};
pub use server::{
    CloudSettingsSyncServer, PushResult, SettingsSyncServer, SyncNotImplementedError,
};
#[cfg(any(test, feature = "test-support"))]
pub use server::{FakeSettingsSyncBackend, FakeSettingsSyncServer};
pub use sync_path::SyncPath;

const NON_MIGRATION_SCHEMA_EPOCH_BUMPS: u64 = 2;

pub fn settings_schema_epoch() -> u64 {
    migrator::settings_migrations_count() as u64 + NON_MIGRATION_SCHEMA_EPOCH_BUMPS
}

#[derive(Clone, Debug, PartialEq, RegisterSetting)]
pub struct SettingsSyncSettings {
    pub enabled: bool,
    pub exclude: Vec<String>,
}

impl Settings for SettingsSyncSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = content.settings_sync.clone().unwrap_or_default();
        Self {
            enabled: content.enabled.unwrap_or(false),
            exclude: content.exclude.unwrap_or_default(),
        }
    }
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
        move |message, cx| {
            if let MessageToClient::SyncedSettingsChanged {
                group_id,
                kind,
                version,
            } = message
            {
                engine
                    .update(cx, |engine, cx| {
                        engine.handle_remote_changed(group_id, kind, *version, cx)
                    })
                    .ok();
            }
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

    let mut was_enabled = SettingsSyncSettings::get_global(cx).enabled;
    cx.observe_global::<SettingsStore>({
        let engine = engine.downgrade();
        move |cx| {
            let enabled = SettingsSyncSettings::get_global(cx).enabled;
            if enabled != was_enabled {
                was_enabled = enabled;
                if enabled {
                    // TODO kb cloud: consent dialog, group picker, and
                    // merge/replace preview diff before the first sync.
                    engine.update(cx, |engine, cx| engine.unpause(cx)).ok();
                }
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
