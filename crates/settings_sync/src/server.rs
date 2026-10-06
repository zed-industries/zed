use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use client::Client;
use cloud_api_client::{
    ClientApiError, GetSyncedSettingsResponse, JoinSyncedSettingsGroupBody,
    ListSyncedSettingsHistoryResponse, SyncedSettings, SyncedSettingsErasureScope,
    SyncedSettingsHost, UpdateSyncedSettingsBody, UpdateSyncedSettingsHostBody,
    UpdateSyncedSettingsResult,
};
use http_client::StatusCode;

#[derive(Debug)]
pub struct SyncNotImplementedError;

impl std::fmt::Display for SyncNotImplementedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Zed Cloud does not serve settings sync for this account")
    }
}

impl std::error::Error for SyncNotImplementedError {}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchResult {
    pub group_id: Option<String>,
    pub document: Option<SyncedSettings>,
}

#[derive(Debug)]
pub enum PushResult {
    Written {
        version: u64,
        group_id: String,
        document_id: String,
    },
    Conflict {
        group_id: Option<String>,
        current: Option<SyncedSettings>,
    },
    NotRegistered,
}

#[async_trait]
pub trait SettingsSyncServer: Send + Sync {
    fn account_id(&self) -> Option<u64>;
    fn system_id(&self) -> Option<String>;
    async fn fetch(&self, account_id: u64, kind: &str) -> Result<FetchResult>;
    async fn push(&self, account_id: u64, body: UpdateSyncedSettingsBody) -> Result<PushResult>;
    async fn list_hosts(&self, account_id: u64) -> Result<Vec<SyncedSettingsHost>>;
    async fn join_group(
        &self,
        account_id: u64,
        body: JoinSyncedSettingsGroupBody,
    ) -> Result<SyncedSettingsHost>;
    async fn update_host(
        &self,
        account_id: u64,
        host_system_id: &str,
        body: UpdateSyncedSettingsHostBody,
    ) -> Result<SyncedSettingsHost>;
    async fn delete_host(&self, account_id: u64, host_system_id: &str) -> Result<()>;
    async fn list_history(
        &self,
        account_id: u64,
        kind: &str,
        limit: u32,
    ) -> Result<ListSyncedSettingsHistoryResponse>;
    async fn erase(
        &self,
        account_id: u64,
        scope: SyncedSettingsErasureScope,
        expected_group_id: Option<String>,
    ) -> Result<()>;

    fn is_ready(&self) -> bool {
        self.account_id().is_some() && self.system_id().is_some()
    }
}

pub struct CloudSettingsSyncServer {
    client: Arc<Client>,
}

impl CloudSettingsSyncServer {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SettingsSyncServer for CloudSettingsSyncServer {
    fn account_id(&self) -> Option<u64> {
        self.client.cloud_client().user_id().map(u64::from)
    }

    fn system_id(&self) -> Option<String> {
        self.client
            .telemetry()
            .system_id()
            .map(|system_id| system_id.to_string())
    }

    async fn fetch(&self, account_id: u64, kind: &str) -> Result<FetchResult> {
        let response = self
            .client
            .cloud_client()
            .get_synced_settings(cloud_user_id(account_id)?, self.system_id(), kind)
            .await
            .map_err(map_not_implemented)?;
        Ok(FetchResult {
            group_id: response.group_id,
            document: response.synced_settings,
        })
    }

    async fn push(&self, account_id: u64, body: UpdateSyncedSettingsBody) -> Result<PushResult> {
        let result = self
            .client
            .cloud_client()
            .update_synced_settings(cloud_user_id(account_id)?, self.system_id(), body)
            .await;
        Ok(match result {
            Ok(UpdateSyncedSettingsResult::Written(response)) => PushResult::Written {
                version: response.version,
                group_id: response.group_id,
                document_id: response.document_id,
            },
            Ok(UpdateSyncedSettingsResult::Conflict(GetSyncedSettingsResponse {
                group_id,
                synced_settings,
            })) => PushResult::Conflict {
                group_id,
                current: synced_settings,
            },
            Err(ClientApiError::ServerError { status, .. })
                if status == StatusCode::PRECONDITION_FAILED =>
            {
                PushResult::NotRegistered
            }
            Err(error) => return Err(map_not_implemented(error)),
        })
    }

    async fn list_hosts(&self, account_id: u64) -> Result<Vec<SyncedSettingsHost>> {
        let response = self
            .client
            .cloud_client()
            .list_synced_settings_hosts(cloud_user_id(account_id)?, self.system_id())
            .await
            .map_err(map_not_implemented)?;
        Ok(response.hosts)
    }

    async fn join_group(
        &self,
        account_id: u64,
        body: JoinSyncedSettingsGroupBody,
    ) -> Result<SyncedSettingsHost> {
        Ok(self
            .client
            .cloud_client()
            .join_synced_settings_group(cloud_user_id(account_id)?, self.system_id(), body)
            .await
            .map_err(map_not_implemented)?)
    }

    async fn update_host(
        &self,
        account_id: u64,
        host_system_id: &str,
        body: UpdateSyncedSettingsHostBody,
    ) -> Result<SyncedSettingsHost> {
        Ok(self
            .client
            .cloud_client()
            .update_synced_settings_host(
                cloud_user_id(account_id)?,
                self.system_id(),
                host_system_id,
                body,
            )
            .await
            .map_err(map_not_implemented)?)
    }

    async fn delete_host(&self, account_id: u64, host_system_id: &str) -> Result<()> {
        Ok(self
            .client
            .cloud_client()
            .delete_synced_settings_host(
                cloud_user_id(account_id)?,
                self.system_id(),
                host_system_id,
            )
            .await
            .map_err(map_not_implemented)?)
    }

    async fn list_history(
        &self,
        account_id: u64,
        kind: &str,
        limit: u32,
    ) -> Result<ListSyncedSettingsHistoryResponse> {
        Ok(self
            .client
            .cloud_client()
            .list_synced_settings_history(cloud_user_id(account_id)?, self.system_id(), kind, limit)
            .await
            .map_err(map_not_implemented)?)
    }

    async fn erase(
        &self,
        account_id: u64,
        scope: SyncedSettingsErasureScope,
        expected_group_id: Option<String>,
    ) -> Result<()> {
        match self
            .client
            .cloud_client()
            .erase_synced_settings(
                cloud_user_id(account_id)?,
                self.system_id(),
                scope,
                expected_group_id,
            )
            .await
        {
            Ok(()) => Ok(()),
            Err(ClientApiError::ServerError { status, .. }) if status == StatusCode::CONFLICT => {
                Err(anyhow::anyhow!(
                    "this device moved to another sync group, review the sync status and try again"
                ))
            }
            Err(error) => Err(map_not_implemented(error)),
        }
    }
}

fn cloud_user_id(account_id: u64) -> Result<u32> {
    Ok(u32::try_from(account_id)?)
}

fn map_not_implemented(error: ClientApiError) -> anyhow::Error {
    match &error {
        ClientApiError::ServerError { status, .. } if *status == StatusCode::NOT_FOUND => {
            anyhow::Error::new(SyncNotImplementedError)
        }
        _ => anyhow::Error::from(error),
    }
}

#[cfg(any(test, feature = "test-support"))]
pub use fake::{FAKE_ACCOUNT_ID, FakeSettingsSyncBackend, FakeSettingsSyncServer};

#[cfg(any(test, feature = "test-support"))]
mod fake {
    use std::collections::{BTreeMap, HashMap};

    use anyhow::anyhow;
    use chrono::{DateTime, Utc};
    use cloud_api_client::{SyncedSettingsGroupTarget, SyncedSettingsRevision, Timestamp};
    use futures::channel::oneshot;
    use parking_lot::Mutex;

    use super::*;

    pub const FAKE_ACCOUNT_ID: u64 = 1;

    #[derive(Default)]
    pub struct FakeSettingsSyncBackend {
        state: Mutex<BackendState>,
    }

    #[derive(Default)]
    struct BackendState {
        docs: HashMap<String, SyncedSettings>,
        revisions: HashMap<String, Vec<SyncedSettingsRevision>>,
        hosts: Vec<SyncedSettingsHost>,
        racing_docs: HashMap<String, SyncedSettings>,
        rotated_groups: HashMap<String, String>,
        next_group: usize,
        next_document: usize,
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
        system_id: String,
        account_id: Mutex<Option<u64>>,
        groups_by_account: Mutex<HashMap<u64, Option<String>>>,
    }

    impl FakeSettingsSyncServer {
        pub fn new() -> Arc<Self> {
            Self::in_group(Arc::new(FakeSettingsSyncBackend::default()), "fake-group")
        }

        pub fn in_group(backend: Arc<FakeSettingsSyncBackend>, group_id: &str) -> Arc<Self> {
            Self::with_group(backend, Some(group_id))
        }

        pub fn unregistered(backend: Arc<FakeSettingsSyncBackend>) -> Arc<Self> {
            Self::with_group(backend, None)
        }

        fn with_group(backend: Arc<FakeSettingsSyncBackend>, group_id: Option<&str>) -> Arc<Self> {
            let system_id = format!("system-{}", backend.state.lock().hosts.len());
            if let Some(group_id) = group_id {
                backend.state.lock().hosts.push(SyncedSettingsHost {
                    system_id: system_id.clone(),
                    label: system_id.clone(),
                    group_id: group_id.to_string(),
                    last_synced_at: None,
                    last_synced_version: None,
                });
            }
            Arc::new(Self {
                backend,
                system_id,
                account_id: Mutex::new(Some(FAKE_ACCOUNT_ID)),
                groups_by_account: Mutex::new(HashMap::from_iter([(
                    FAKE_ACCOUNT_ID,
                    group_id.map(ToString::to_string),
                )])),
            })
        }

        pub fn group_id(&self) -> Option<String> {
            let account_id = (*self.account_id.lock())?;
            self.account_group(account_id)
        }

        pub fn doc(&self) -> Option<SyncedSettings> {
            let group_id = self.group_id()?;
            self.backend.state.lock().docs.get(&group_id).cloned()
        }

        pub fn set_doc(&self, doc: SyncedSettings) {
            self.backend
                .state
                .lock()
                .docs
                .insert(doc.group_id.clone(), doc);
        }

        pub fn set_account(&self, account_id: Option<u64>, group_id: Option<&str>) {
            *self.account_id.lock() = account_id;
            if let Some(account_id) = account_id {
                self.groups_by_account
                    .lock()
                    .insert(account_id, group_id.map(ToString::to_string));
            }
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
                .insert(doc.group_id.clone(), doc);
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

        fn check_account(&self, account_id: u64) -> Result<()> {
            if *self.account_id.lock() == Some(account_id) {
                Ok(())
            } else {
                Err(anyhow::Error::from(ClientApiError::NotSignedIn))
            }
        }

        fn account_group(&self, account_id: u64) -> Option<String> {
            let mut group_id = self
                .groups_by_account
                .lock()
                .get(&account_id)
                .cloned()
                .flatten()?;
            let state = self.backend.state.lock();
            while let Some(rotated) = state.rotated_groups.get(&group_id) {
                group_id = rotated.clone();
            }
            Some(group_id)
        }

        fn set_account_group(&self, account_id: u64, group_id: Option<String>) {
            self.groups_by_account.lock().insert(account_id, group_id);
        }
    }

    #[async_trait]
    impl SettingsSyncServer for FakeSettingsSyncServer {
        fn account_id(&self) -> Option<u64> {
            *self.account_id.lock()
        }

        fn system_id(&self) -> Option<String> {
            Some(self.system_id.clone())
        }

        async fn fetch(&self, account_id: u64, kind: &str) -> Result<FetchResult> {
            self.check_account(account_id)?;
            let group_id = self.account_group(account_id);
            let gate = self.backend.state.lock().fetch_gate.take();
            if let Some(gate) = gate {
                gate.await.ok();
            }
            let mut state = self.backend.state.lock();
            if state.unimplemented {
                return Err(anyhow::Error::new(SyncNotImplementedError));
            }
            state.fetch_count += 1;
            let document = group_id
                .as_ref()
                .and_then(|group_id| state.docs.get(group_id))
                .filter(|synced_settings| synced_settings.kind == kind)
                .cloned();
            Ok(FetchResult { group_id, document })
        }

        async fn push(
            &self,
            account_id: u64,
            body: UpdateSyncedSettingsBody,
        ) -> Result<PushResult> {
            self.check_account(account_id)?;
            let gate = self.backend.state.lock().push_gate.take();
            if let Some(gate) = gate {
                gate.await.ok();
            }
            let group_id = self.account_group(account_id);
            let mut state = self.backend.state.lock();
            if state.unimplemented {
                return Err(anyhow::Error::new(SyncNotImplementedError));
            }
            state.push_count += 1;
            let Some(group_id) = group_id else {
                return Ok(PushResult::NotRegistered);
            };
            if state.always_conflict || body.expected_group_id != group_id {
                state.conflict_count += 1;
                return Ok(PushResult::Conflict {
                    current: state.docs.get(&group_id).cloned(),
                    group_id: Some(group_id),
                });
            }
            if let Some(racing_doc) = state.racing_docs.remove(&group_id) {
                state.docs.insert(group_id.clone(), racing_doc);
            }
            let current = state
                .docs
                .get(&group_id)
                .filter(|synced_settings| synced_settings.kind == body.kind)
                .map(|synced_settings| {
                    (synced_settings.version, synced_settings.document_id.clone())
                });
            let base = body.base_version.zip(body.base_document_id.clone());
            let base_is_consistent = body.base_version.is_some() == body.base_document_id.is_some();
            if !base_is_consistent || base != current {
                state.conflict_count += 1;
                return Ok(PushResult::Conflict {
                    current: state.docs.get(&group_id).cloned(),
                    group_id: Some(group_id),
                });
            }
            let (version, document_id) = match current {
                Some((version, document_id)) => (version + 1, document_id),
                None => {
                    state.next_document += 1;
                    (1, format!("document-{}", state.next_document))
                }
            };
            state
                .revisions
                .entry(group_id.clone())
                .or_default()
                .push(SyncedSettingsRevision {
                    version,
                    created_at: Timestamp(DateTime::<Utc>::UNIX_EPOCH),
                    system_id: Some(self.system_id.clone()),
                    schema_epoch: body.schema_epoch,
                    doc: body.doc.clone(),
                });
            state.docs.insert(
                group_id.clone(),
                SyncedSettings {
                    group_id: group_id.clone(),
                    document_id: document_id.clone(),
                    kind: body.kind,
                    version,
                    schema_epoch: body.schema_epoch,
                    doc: body.doc,
                    updated_by_system_id: Some(self.system_id.clone()),
                    provenance: BTreeMap::new(),
                },
            );
            Ok(PushResult::Written {
                version,
                group_id,
                document_id,
            })
        }

        async fn list_hosts(&self, account_id: u64) -> Result<Vec<SyncedSettingsHost>> {
            self.check_account(account_id)?;
            Ok(self.backend.state.lock().hosts.clone())
        }

        async fn join_group(
            &self,
            account_id: u64,
            body: JoinSyncedSettingsGroupBody,
        ) -> Result<SyncedSettingsHost> {
            self.check_account(account_id)?;
            let mut state = self.backend.state.lock();
            let group_id = match body.group {
                SyncedSettingsGroupTarget::Existing { group_id } => group_id,
                SyncedSettingsGroupTarget::New => {
                    state.next_group += 1;
                    format!("group-{}", state.next_group)
                }
            };
            state.hosts.retain(|host| host.system_id != self.system_id);
            let host = SyncedSettingsHost {
                system_id: self.system_id.clone(),
                label: body.label,
                group_id: group_id.clone(),
                last_synced_at: None,
                last_synced_version: None,
            };
            state.hosts.push(host.clone());
            drop(state);
            self.set_account_group(account_id, Some(group_id));
            Ok(host)
        }

        async fn update_host(
            &self,
            account_id: u64,
            host_system_id: &str,
            body: UpdateSyncedSettingsHostBody,
        ) -> Result<SyncedSettingsHost> {
            self.check_account(account_id)?;
            let mut state = self.backend.state.lock();
            let host = state
                .hosts
                .iter_mut()
                .find(|host| host.system_id == host_system_id)
                .ok_or_else(|| anyhow!("unknown host {host_system_id}"))?;
            if let Some(label) = body.label {
                host.label = label;
            }
            if let Some(SyncedSettingsGroupTarget::Existing { group_id }) = body.group {
                host.group_id = group_id;
            }
            let host = host.clone();
            drop(state);
            if host_system_id == self.system_id {
                self.set_account_group(account_id, Some(host.group_id.clone()));
            }
            Ok(host)
        }

        async fn delete_host(&self, account_id: u64, host_system_id: &str) -> Result<()> {
            self.check_account(account_id)?;
            self.backend
                .state
                .lock()
                .hosts
                .retain(|host| host.system_id != host_system_id);
            if host_system_id == self.system_id {
                self.set_account_group(account_id, None);
            }
            Ok(())
        }

        async fn list_history(
            &self,
            account_id: u64,
            kind: &str,
            limit: u32,
        ) -> Result<ListSyncedSettingsHistoryResponse> {
            self.check_account(account_id)?;
            let group_id = self.account_group(account_id);
            let state = self.backend.state.lock();
            let revisions = group_id
                .as_ref()
                .filter(|group_id| {
                    state
                        .docs
                        .get(*group_id)
                        .is_none_or(|synced_settings| synced_settings.kind == kind)
                })
                .and_then(|group_id| state.revisions.get(group_id))
                .map(|revisions| {
                    revisions
                        .iter()
                        .rev()
                        .take(limit as usize)
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            Ok(ListSyncedSettingsHistoryResponse {
                group_id,
                revisions,
            })
        }

        async fn erase(
            &self,
            account_id: u64,
            scope: SyncedSettingsErasureScope,
            expected_group_id: Option<String>,
        ) -> Result<()> {
            self.check_account(account_id)?;
            let group_id = self.account_group(account_id);
            let mut state = self.backend.state.lock();
            match scope {
                SyncedSettingsErasureScope::Group => {
                    let Some(group_id) =
                        group_id.filter(|group_id| expected_group_id.as_ref() == Some(group_id))
                    else {
                        return Err(anyhow!("this device moved to another sync group"));
                    };
                    state.docs.remove(&group_id);
                    state.revisions.remove(&group_id);
                    state.next_group += 1;
                    let rotated_group_id = format!("group-{}", state.next_group);
                    for host in &mut state.hosts {
                        if host.group_id == group_id {
                            host.group_id = rotated_group_id.clone();
                        }
                    }
                    state.rotated_groups.insert(group_id, rotated_group_id);
                }
                SyncedSettingsErasureScope::Account => {
                    state.docs.clear();
                    state.revisions.clear();
                    state.hosts.clear();
                    drop(state);
                    self.set_account_group(account_id, None);
                }
            }
            Ok(())
        }
    }
}
