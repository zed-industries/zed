use std::{fmt, sync::Arc};

use async_lock::{RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard};
use cloud_api_types::OrganizationId;

use crate::{ClientApiError, CloudApiClient};

#[derive(Clone, Default)]
pub struct LlmApiToken(Arc<RwLock<Option<CachedLlmApiToken>>>);

struct CachedLlmApiToken {
    /// The organization ID the token was minted for.
    organization_id: OrganizationId,
    token: String,
}

impl fmt::Debug for CachedLlmApiToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CachedLlmApiToken")
            .field("organization_id", &self.organization_id)
            .field("token", &"<redacted>")
            .finish()
    }
}

impl LlmApiToken {
    /// Returns the cached LLM token, fetching a fresh one if none has been
    /// cached yet or if the cached token was minted for a different
    /// organization. The returned token is not validated; callers must
    /// be prepared to refresh it (via [`LlmApiToken::refresh`]) if the
    /// server rejects it.
    pub async fn cached(
        &self,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<String, ClientApiError> {
        let lock = self.0.upgradable_read().await;
        if let Some(CachedLlmApiToken {
            organization_id: cached_organization_id,
            token,
        }) = lock.as_ref()
            && *cached_organization_id == organization_id
        {
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
        organization_id: OrganizationId,
    ) -> Result<String, ClientApiError> {
        Self::fetch(self.0.write().await, client, system_id, organization_id).await
    }

    pub async fn clear(&self) {
        *self.0.write().await = None;
    }

    /// Clears the existing token before attempting to fetch a new one.
    ///
    /// Used when switching organizations so that a failed refresh doesn't
    /// leave a token for the wrong organization.
    pub async fn clear_and_refresh(
        &self,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<String, ClientApiError> {
        let mut lock = self.0.write().await;
        *lock = None;
        Self::fetch(lock, client, system_id, organization_id).await
    }

    async fn fetch(
        mut lock: RwLockWriteGuard<'_, Option<CachedLlmApiToken>>,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<String, ClientApiError> {
        let result = client
            .create_llm_token(system_id, organization_id.clone())
            .await;
        match result {
            Ok(response) => {
                *lock = Some(CachedLlmApiToken {
                    organization_id,
                    token: response.token.0.clone(),
                });

                Ok(response.token.0)
            }
            Err(err) => {
                *lock = None;
                Err(err)
            }
        }
    }
}
