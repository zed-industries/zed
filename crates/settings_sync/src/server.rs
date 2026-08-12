use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use client::Client;
use cloud_api_client::{
    GetSyncedSettingsResponse, SyncedSettings, UpdateSyncedSettingsBody, UpdateSyncedSettingsResult,
};

#[derive(Debug)]
pub enum PushResult {
    Written { version: u64 },
    Conflict { current: Option<SyncedSettings> },
}

#[async_trait]
pub trait SettingsSyncServer: Send + Sync {
    fn is_ready(&self) -> bool;
    async fn fetch(&self, kind: &str) -> Result<Option<SyncedSettings>>;
    async fn push(&self, body: UpdateSyncedSettingsBody) -> Result<PushResult>;
}

// TODO kb cloud: talks to `/client/synced_settings`, which Cloud does not
// implement yet.
pub struct CloudSettingsSyncServer {
    client: Arc<Client>,
}

impl CloudSettingsSyncServer {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    fn system_id(&self) -> Option<String> {
        self.client
            .telemetry()
            .system_id()
            .map(|system_id| system_id.to_string())
    }
}

#[async_trait]
impl SettingsSyncServer for CloudSettingsSyncServer {
    fn is_ready(&self) -> bool {
        self.client.cloud_client().has_credentials()
    }

    async fn fetch(&self, kind: &str) -> Result<Option<SyncedSettings>> {
        let response = self
            .client
            .cloud_client()
            .get_synced_settings(self.system_id(), kind)
            .await?;
        Ok(response.synced_settings)
    }

    async fn push(&self, body: UpdateSyncedSettingsBody) -> Result<PushResult> {
        let result = self
            .client
            .cloud_client()
            .update_synced_settings(self.system_id(), body)
            .await?;
        Ok(match result {
            UpdateSyncedSettingsResult::Written(response) => PushResult::Written {
                version: response.version,
            },
            UpdateSyncedSettingsResult::Conflict(GetSyncedSettingsResponse { synced_settings }) => {
                PushResult::Conflict {
                    current: synced_settings,
                }
            }
        })
    }
}

#[cfg(any(test, feature = "test-support"))]
pub use fake::FakeSettingsSyncServer;

#[cfg(any(test, feature = "test-support"))]
mod fake {
    use super::*;
    use parking_lot::Mutex;

    #[derive(Default)]
    pub struct FakeSettingsSyncServer {
        state: Mutex<FakeState>,
    }

    #[derive(Default)]
    struct FakeState {
        doc: Option<SyncedSettings>,
        racing_doc: Option<SyncedSettings>,
        push_count: usize,
        conflict_count: usize,
    }

    impl FakeSettingsSyncServer {
        pub fn new() -> Arc<Self> {
            Arc::new(Self::default())
        }

        pub fn doc(&self) -> Option<SyncedSettings> {
            self.state.lock().doc.clone()
        }

        pub fn set_doc(&self, doc: SyncedSettings) {
            self.state.lock().doc = Some(doc);
        }

        pub fn push_count(&self) -> usize {
            self.state.lock().push_count
        }

        pub fn conflict_count(&self) -> usize {
            self.state.lock().conflict_count
        }

        pub fn queue_racing_doc(&self, doc: SyncedSettings) {
            self.state.lock().racing_doc = Some(doc);
        }
    }

    #[async_trait]
    impl SettingsSyncServer for FakeSettingsSyncServer {
        fn is_ready(&self) -> bool {
            true
        }

        async fn fetch(&self, kind: &str) -> Result<Option<SyncedSettings>> {
            let state = self.state.lock();
            Ok(state
                .doc
                .clone()
                .filter(|synced_settings| synced_settings.kind == kind))
        }

        async fn push(&self, body: UpdateSyncedSettingsBody) -> Result<PushResult> {
            let mut state = self.state.lock();
            state.push_count += 1;
            if let Some(racing_doc) = state.racing_doc.take() {
                state.doc = Some(racing_doc);
            }
            let current_version = state
                .doc
                .as_ref()
                .filter(|synced_settings| synced_settings.kind == body.kind)
                .map(|synced_settings| synced_settings.version);
            if body.base_version != current_version {
                state.conflict_count += 1;
                return Ok(PushResult::Conflict {
                    current: state.doc.clone(),
                });
            }
            let version = current_version.unwrap_or(0) + 1;
            state.doc = Some(SyncedSettings {
                group_id: "fake-group".to_string(),
                kind: body.kind,
                version,
                schema_epoch: body.schema_epoch,
                doc: body.doc,
                updated_by_system_id: None,
            });
            Ok(PushResult::Written { version })
        }
    }
}
