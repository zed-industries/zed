use anyhow::{Context as _, Result, anyhow};
use credentials_provider::CredentialsProvider;
use futures::{FutureExt, future};
use gpui::{AsyncApp, Context, SharedString, Task};
use language_model::AuthenticateError;
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};
use thiserror::Error;
use util::ResultExt as _;
use zed_env_vars::EnvVar;

/// Manages a single API key for a language model provider. API keys either come from environment
/// variables or the system keychain.
///
/// Keys from the system keychain are associated with a provider URL, and this ensures that they are
/// only used with that URL.
pub struct ApiKeyState {
    api_key: Option<ApiKey>,
    load_task: Option<LoadTask>,
}

pub struct ApiKey {
    source: ApiKeySource,
    url: SharedString,
    key: Arc<str>,
}

struct LoadTask {
    url: SharedString,
    task: future::Shared<Task<Result<(), LoadError>>>,
}

impl ApiKeyState {
    pub fn new() -> Self {
        Self {
            api_key: None,
            load_task: None,
        }
    }

    pub fn has_key(&self) -> bool {
        self.api_key.is_some()
    }

    pub fn is_from_env_var(&self) -> bool {
        self.api_key
            .as_ref()
            .map_or(false, |key| matches!(key.source, ApiKeySource::EnvVar(_)))
    }

    /// Get the stored API key, verifying that it is associated with the URL. Returns `None` if
    /// there is no key or for URL mismatches, and the mismatch case is logged.
    ///
    /// To avoid URL mismatches, expects that `load_if_needed` or `handle_url_change` has been
    /// called with this URL.
    pub fn key(&self, url: &str) -> Option<Arc<str>> {
        let api_key = self.api_key.as_ref()?;
        if url == api_key.url.as_str() {
            Some(api_key.key.clone())
        } else if let ApiKeySource::EnvVar(var_name) = &api_key.source {
            log::warn!(
                "{} is now being used with URL {}, when initially it was used with URL {}",
                var_name,
                url,
                api_key.url
            );
            Some(api_key.key.clone())
        } else {
            // bug case because load_if_needed should be called whenever the url may have changed
            log::error!(
                "bug: Attempted to use API key associated with URL {} instead with URL {}",
                api_key.url,
                url
            );
            None
        }
    }

    /// Set or delete the API key in the system keychain.
    pub fn store<Ent: 'static>(
        &mut self,
        url: String,
        key: Option<String>,
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        cx: &Context<Ent>,
    ) -> Task<Result<()>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn(async move |ent, cx| {
            if let Some(key) = &key {
                credentials_provider
                    .write_credentials(&url, "Bearer", key.as_bytes(), cx)
                    .await
                    .log_err();
            } else {
                credentials_provider
                    .delete_credentials(&url, cx)
                    .await
                    .log_err();
            }
            ent.update(cx, |ent, cx| {
                get_this(ent).api_key = None;
                cx.notify();
            })
        })
    }

    /// Reloads the API key if the current API key is associated with a different URL.
    ///
    /// Note that it is not efficient to use this or `load_if_needed` with multiple URLs
    /// interchangeably - URL change should correspond to some user initiated change.
    pub fn handle_url_change<Ent: 'static>(
        &mut self,
        url: SharedString,
        env_var: &EnvVar,
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        cx: &mut Context<Ent>,
    ) {
        if let Some(api_key) = &self.api_key {
            match api_key.source {
                ApiKeySource::EnvVar { .. } => {}
                ApiKeySource::SystemKeychain => {
                    // loading will continue even though this result task is dropped
                    let _task = self.load_if_needed(url, env_var, get_this, cx);
                }
            }
        }
    }

    /// If needed, loads the API key associated with the given URL from the system keychain. When a
    /// non-empty environment variable is provided, it will be used instead. If called when an API
    /// key was already loaded for a different URL, that key will be cleared before loading.
    ///
    /// Dropping the returned Task does not cancel key loading.
    pub fn load_if_needed<Ent: 'static>(
        &mut self,
        url: SharedString,
        env_var: &EnvVar,
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        cx: &mut Context<Ent>,
    ) -> Task<Result<(), AuthenticateError>> {
        if let Some(api_key) = &self.api_key
            && api_key.url == url
        {
            return Task::ready(Ok(()));
        }

        if let Some(key) = &env_var.value
            && !key.is_empty()
        {
            let api_key = ApiKey::from_env(env_var.name.clone(), key, url);
            self.api_key = Some(api_key);
            cx.notify();
            return Task::ready(Ok(()));
        }

        let task = if let Some(load_task) = &self.load_task
            && load_task.url == url
        {
            load_task.task.clone()
        } else {
            let task = Self::load(url.clone(), get_this, cx).shared();
            let load_task = LoadTask {
                url,
                task: task.clone(),
            };
            self.api_key = None;
            self.load_task = Some(load_task);
            cx.notify();
            task
        };

        cx.spawn(async move |_, _cx| task.await.map_err(Into::<AuthenticateError>::into))
    }

    fn load<Ent: 'static>(
        url: SharedString,
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        cx: &Context<Ent>,
    ) -> Task<Result<(), LoadError>> {
        let credentials_provider = <dyn CredentialsProvider>::global(cx);
        cx.spawn({
            async move |ent, cx| {
                let api_key =
                    ApiKey::load_from_system_keychain(url, credentials_provider.as_ref(), cx)
                        .await?;
                ent.update(cx, |ent, cx| {
                    get_this(ent).api_key = Some(api_key);
                    cx.notify();
                })?;
                Ok(())
            }
        })
    }
}

impl ApiKey {
    pub fn from_env(env_var_name: SharedString, key: &str, url: SharedString) -> Self {
        Self {
            source: ApiKeySource::EnvVar(env_var_name),
            url,
            key: key.into(),
        }
    }

    pub async fn load_from_system_keychain(
        url: SharedString,
        credentials_provider: &dyn CredentialsProvider,
        cx: &AsyncApp,
    ) -> Result<Self, LoadError> {
        let (_, api_key) = credentials_provider
            .read_credentials(&url, cx)
            .await?
            .ok_or(LoadError::CredentialsNotFound)?;
        let key =
            str::from_utf8(&api_key).with_context(|| format!("invalid API key for URL {}", url))?;
        Ok(Self {
            source: ApiKeySource::SystemKeychain,
            url,
            key: key.into(),
        })
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

#[derive(Debug, Clone)]
enum ApiKeySource {
    EnvVar(SharedString),
    SystemKeychain,
}

impl Display for ApiKeySource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiKeySource::EnvVar(var) => write!(f, "environment variable {}", var),
            ApiKeySource::SystemKeychain => write!(f, "system keychain"),
        }
    }
}

/// This is similar to `AuthenticateError` excerpt that it implements `Clone` and so can be used in
/// the result of a `Shared` task.
#[derive(Debug, Clone, Error)]
pub enum LoadError {
    #[error("credentials not found")]
    CredentialsNotFound,
    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for LoadError {
    fn from(err: anyhow::Error) -> Self {
        LoadError::Other(err.to_string())
    }
}

impl From<LoadError> for AuthenticateError {
    fn from(err: LoadError) -> Self {
        match err {
            LoadError::CredentialsNotFound => AuthenticateError::CredentialsNotFound,
            LoadError::Other(err) => AuthenticateError::Other(anyhow!(err)),
        }
    }
}
