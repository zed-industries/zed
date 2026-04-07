use std::sync::Arc;

use cloud_api_types::OrganizationId;
use smol::lock::{RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard};

use crate::{ClientApiError, CloudApiClient};

#[derive(Clone, Default)]
pub struct LlmApiToken(Arc<RwLock<Option<String>>>);

impl LlmApiToken {
    pub async fn acquire(
        &self,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: Option<OrganizationId>,
    ) -> Result<String, ClientApiError> {
        let lock = self.0.upgradable_read().await;
        if let Some(token) = lock.as_ref() {
            Ok(token.to_string())
        } else {
            Self::fetch(
                RwLockUpgradableReadGuard::upgrade(lock).await,
                client,
                system_id,
                organization_id,
            )
            .await
        }
    }

    pub async fn refresh(
        &self,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: Option<OrganizationId>,
    ) -> Result<String, ClientApiError> {
        Self::fetch(self.0.write().await, client, system_id, organization_id).await
    }

    /// Clears the existing token before attempting to fetch a new one.
    ///
    /// Used when switching organizations so that a failed refresh doesn't
    /// leave a token for the wrong organization.
    pub async fn clear_and_refresh(
        &self,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: Option<OrganizationId>,
    ) -> Result<String, ClientApiError> {
        let mut lock = self.0.write().await;
        *lock = None;
        Self::fetch(lock, client, system_id, organization_id).await
    }

    async fn fetch(
        mut lock: RwLockWriteGuard<'_, Option<String>>,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: Option<OrganizationId>,
    ) -> Result<String, ClientApiError> {
        let result = client.create_llm_token(system_id, organization_id).await;
        match result {
            Ok(response) => {
                *lock = Some(response.token.0.clone());
                Ok(response.token.0)
            }
            Err(err) => {
                *lock = None;
                Err(err)
            }
        }
    }
}
