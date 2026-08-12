use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use client::Client;
use cloud_api_client::{
    ClientApiError, GetSyncedSettingsResponse, SyncedSettings, UpdateSyncedSettingsBody,
    UpdateSyncedSettingsResult,
};

#[derive(Debug)]
pub struct SyncNotImplementedError;

impl std::fmt::Display for SyncNotImplementedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Zed Cloud does not implement settings sync yet")
    }
}

impl std::error::Error for SyncNotImplementedError {}

// TODO kb cloud: 404 is reserved for "endpoint not implemented"; the real
// endpoint must return 200 with a null `synced_settings` for "no doc yet",
// never 404.
fn map_not_implemented(error: ClientApiError) -> anyhow::Error {
    match &error {
        ClientApiError::ServerError { status, .. } if status.as_u16() == 404 => {
            anyhow::Error::new(SyncNotImplementedError)
        }
        _ => error.into(),
    }
}

#[derive(Debug)]
pub enum PushResult {
    Written { version: u64, group_id: String },
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
    // TODO kb cloud: the server must reject sync requests lacking
    // `x-zed-system-id`; the device id is the group-membership key.
    fn is_ready(&self) -> bool {
        self.client.cloud_client().has_credentials() && self.system_id().is_some()
    }

    async fn fetch(&self, kind: &str) -> Result<Option<SyncedSettings>> {
        let response = self
            .client
            .cloud_client()
            .get_synced_settings(self.system_id(), kind)
            .await
            .map_err(map_not_implemented)?;
        Ok(response.synced_settings)
    }

    async fn push(&self, body: UpdateSyncedSettingsBody) -> Result<PushResult> {
        let result = self
            .client
            .cloud_client()
            .update_synced_settings(self.system_id(), body)
            .await
            .map_err(map_not_implemented)?;
        Ok(match result {
            UpdateSyncedSettingsResult::Written(response) => PushResult::Written {
                version: response.version,
                group_id: response.group_id,
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
pub use fake::{FakeSettingsSyncBackend, FakeSettingsSyncServer};

#[cfg(any(test, feature = "test-support"))]
mod fake {
    use std::collections::HashMap;

    use futures::channel::oneshot;
    use parking_lot::Mutex;

    use super::*;

    #[derive(Default)]
    pub struct FakeSettingsSyncBackend {
        state: Mutex<BackendState>,
    }

    #[derive(Default)]
    struct BackendState {
        docs: HashMap<String, SyncedSettings>,
        racing_docs: HashMap<String, SyncedSettings>,
        fetch_count: usize,
        push_count: usize,
        conflict_count: usize,
        always_conflict: bool,
        unimplemented: bool,
        fetch_gate: Option<oneshot::Receiver<()>>,
        push_gate: Option<oneshot::Receiver<()>>,
    }

    pub struct FakeSettingsSyncServer {
        backend: Arc<FakeSettingsSyncBackend>,
        group_id: String,
    }

    impl FakeSettingsSyncServer {
        pub fn new() -> Arc<Self> {
            Self::in_group(Arc::new(FakeSettingsSyncBackend::default()), "fake-group")
        }

        pub fn in_group(backend: Arc<FakeSettingsSyncBackend>, group_id: &str) -> Arc<Self> {
            Arc::new(Self {
                backend,
                group_id: group_id.to_string(),
            })
        }

        pub fn doc(&self) -> Option<SyncedSettings> {
            self.backend.state.lock().docs.get(&self.group_id).cloned()
        }

        pub fn set_doc(&self, doc: SyncedSettings) {
            self.backend
                .state
                .lock()
                .docs
                .insert(self.group_id.clone(), doc);
        }

        pub fn fetch_count(&self) -> usize {
            self.backend.state.lock().fetch_count
        }

        pub fn push_count(&self) -> usize {
            self.backend.state.lock().push_count
        }

        pub fn conflict_count(&self) -> usize {
            self.backend.state.lock().conflict_count
        }

        pub fn queue_racing_doc(&self, doc: SyncedSettings) {
            self.backend
                .state
                .lock()
                .racing_docs
                .insert(self.group_id.clone(), doc);
        }

        pub fn set_always_conflict(&self, always_conflict: bool) {
            self.backend.state.lock().always_conflict = always_conflict;
        }

        pub fn set_unimplemented(&self, unimplemented: bool) {
            self.backend.state.lock().unimplemented = unimplemented;
        }

        pub fn gate_next_fetch(&self) -> oneshot::Sender<()> {
            let (tx, rx) = oneshot::channel();
            self.backend.state.lock().fetch_gate = Some(rx);
            tx
        }

        pub fn gate_next_push(&self) -> oneshot::Sender<()> {
            let (tx, rx) = oneshot::channel();
            self.backend.state.lock().push_gate = Some(rx);
            tx
        }
    }

    #[async_trait]
    impl SettingsSyncServer for FakeSettingsSyncServer {
        fn is_ready(&self) -> bool {
            true
        }

        async fn fetch(&self, kind: &str) -> Result<Option<SyncedSettings>> {
            let gate = self.backend.state.lock().fetch_gate.take();
            if let Some(gate) = gate {
                gate.await.ok();
            }
            let mut state = self.backend.state.lock();
            if state.unimplemented {
                return Err(anyhow::Error::new(SyncNotImplementedError));
            }
            state.fetch_count += 1;
            Ok(state
                .docs
                .get(&self.group_id)
                .cloned()
                .filter(|synced_settings| synced_settings.kind == kind))
        }

        async fn push(&self, body: UpdateSyncedSettingsBody) -> Result<PushResult> {
            let gate = self.backend.state.lock().push_gate.take();
            if let Some(gate) = gate {
                gate.await.ok();
            }
            let mut state = self.backend.state.lock();
            if state.unimplemented {
                return Err(anyhow::Error::new(SyncNotImplementedError));
            }
            state.push_count += 1;
            if state.always_conflict {
                state.conflict_count += 1;
                return Ok(PushResult::Conflict {
                    current: state.docs.get(&self.group_id).cloned(),
                });
            }
            if let Some(racing_doc) = state.racing_docs.remove(&self.group_id) {
                state.docs.insert(self.group_id.clone(), racing_doc);
            }
            let current_version = state
                .docs
                .get(&self.group_id)
                .filter(|synced_settings| synced_settings.kind == body.kind)
                .map(|synced_settings| synced_settings.version);
            if body.base_version != current_version {
                state.conflict_count += 1;
                return Ok(PushResult::Conflict {
                    current: state.docs.get(&self.group_id).cloned(),
                });
            }
            let version = current_version.unwrap_or(0) + 1;
            state.docs.insert(
                self.group_id.clone(),
                SyncedSettings {
                    group_id: self.group_id.clone(),
                    kind: body.kind,
                    version,
                    schema_epoch: body.schema_epoch,
                    doc: body.doc,
                    updated_by_system_id: None,
                },
            );
            Ok(PushResult::Written {
                version,
                group_id: self.group_id.clone(),
            })
        }
    }
}
