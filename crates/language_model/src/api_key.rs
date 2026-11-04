use anyhow::{Result, anyhow};
use credentials_provider::CredentialsProvider;
use env_var::EnvVar;
use futures::{FutureExt, channel::oneshot, future};
use gpui::{App, AppContext, AsyncApp, Context, Global, SharedString, Subscription, Task};
use gpui_util::ResultExt as _;
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use crate::AuthenticateError;

/// Manages a single API key for a language model provider. API keys either come from environment
/// variables or the system keychain.
///
/// Keys from the system keychain are associated with a provider URL, and this ensures that they are
/// only used with that URL.
pub struct ApiKeyState {
    pub url: SharedString,
    env_var: EnvVar,
    load_status: LoadStatus,
    load_task: Option<ApiKeyLoad>,
}

#[derive(Debug, Clone)]
pub enum LoadStatus {
    NotLoaded,
    NotPresent,
    Error(String),
    Loaded(ApiKey),
}

#[derive(Debug, Clone)]
pub struct ApiKey {
    source: ApiKeySource,
    key: Arc<str>,
}

pub fn set_credentials_read_gate<G: Global>(is_disabled: fn(&App) -> bool, cx: &mut App) {
    cx.set_global(CredentialsReadGate {
        is_disabled,
        subscribe: |cx, is_disabled, sender| {
            let mut sender = Some(sender);
            cx.observe_global::<G>(move |cx| {
                if !is_disabled(cx)
                    && let Some(sender) = sender.take()
                {
                    sender.send(()).ok();
                }
            })
        },
    });
}

pub async fn read_credentials(
    provider: &dyn CredentialsProvider,
    url: &str,
    cx: &AsyncApp,
) -> Result<Option<(String, Vec<u8>)>> {
    while let Some((receiver, _subscription)) = cx.update(|cx| {
        let gate = *cx.try_global::<CredentialsReadGate>()?;
        if !(gate.is_disabled)(cx) {
            return None;
        }
        let (sender, receiver) = oneshot::channel();
        let subscription = (gate.subscribe)(cx, gate.is_disabled, sender);
        Some((receiver, subscription))
    }) {
        receiver.await?;
    }
    provider.read_credentials(url, cx).await
}

impl ApiKeyState {
    pub fn new(url: SharedString, env_var: EnvVar) -> Self {
        Self {
            url,
            env_var,
            load_status: LoadStatus::NotLoaded,
            load_task: None,
        }
    }

    pub fn has_key(&self) -> bool {
        matches!(self.load_status, LoadStatus::Loaded { .. })
    }

    pub fn env_var_name(&self) -> &SharedString {
        &self.env_var.name
    }

    pub fn is_from_env_var(&self) -> bool {
        match &self.load_status {
            LoadStatus::Loaded(ApiKey {
                source: ApiKeySource::EnvVar { .. },
                ..
            }) => true,
            _ => false,
        }
    }

    /// Get the stored API key, verifying that it is associated with the URL. Returns `None` if
    /// there is no key or for URL mismatches, and the mismatch case is logged.
    ///
    /// To avoid URL mismatches, expects that `load_if_needed` or `handle_url_change` has been
    /// called with this URL.
    pub fn key(&self, url: &str) -> Option<Arc<str>> {
        let api_key = match &self.load_status {
            LoadStatus::Loaded(api_key) => api_key,
            _ => return None,
        };
        if url == self.url.as_str() {
            Some(api_key.key.clone())
        } else if let ApiKeySource::EnvVar(var_name) = &api_key.source {
            log::warn!(
                "{} is now being used with URL {}, when initially it was used with URL {}",
                var_name,
                url,
                self.url
            );
            Some(api_key.key.clone())
        } else {
            // bug case because load_if_needed should be called whenever the url may have changed
            log::error!(
                "bug: Attempted to use API key associated with URL {} instead with URL {}",
                self.url,
                url
            );
            None
        }
    }

    /// Set or delete the API key in the system keychain.
    pub fn store<Ent: 'static>(
        &mut self,
        url: SharedString,
        key: Option<String>,
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        provider: Arc<dyn CredentialsProvider>,
        cx: &mut Context<Ent>,
    ) -> Task<Result<()>> {
        if self.is_from_env_var() {
            return Task::ready(Err(anyhow!(
                "bug: attempted to store API key in system keychain when API key is from env var",
            )));
        }
        if self.url != url {
            self.url = url.clone();
            self.load_status = LoadStatus::NotLoaded;
            self.load_task = None;
            cx.notify();
        }

        cx.spawn(async move |ent, cx| {
            if let Some(key) = &key {
                provider
                    .write_credentials(&url, "Bearer", key.as_bytes(), cx)
                    .await
                    .log_err();
            } else {
                provider.delete_credentials(&url, cx).await.log_err();
            }
            ent.update(cx, |ent, cx| {
                let key_state = get_this(ent);
                if key_state.url != url {
                    return;
                }
                key_state.load_task = None;
                key_state.load_status = match &key {
                    Some(key) => LoadStatus::Loaded(ApiKey {
                        source: ApiKeySource::SystemKeychain,
                        key: key.as_str().into(),
                    }),
                    None => LoadStatus::NotPresent,
                };
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
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        provider: Arc<dyn CredentialsProvider>,
        cx: &mut Context<Ent>,
    ) {
        if url != self.url {
            if !self.is_from_env_var() {
                // loading will continue even though this result task is dropped
                let _task = self.load_if_needed(url, get_this, provider, cx);
            }
        }
    }

    pub fn load_cached<Ent: 'static>(
        &mut self,
        url: SharedString,
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        provider: Arc<dyn CredentialsProvider>,
        cx: &mut Context<Ent>,
    ) -> Task<Result<(), AuthenticateError>> {
        if let LoadStatus::NotPresent | LoadStatus::Error(_) = &self.load_status
            && self.url == url
        {
            return Task::ready(Ok(()));
        }

        self.load_if_needed(url, get_this, provider, cx)
    }

    /// If needed, loads the API key associated with the given URL from the system keychain. When a
    /// non-empty environment variable is provided, it will be used instead. If called when an API
    /// key was already loaded for a different URL, that key will be cleared before loading.
    ///
    /// Dropping the returned Task does not cancel key loading.
    pub fn load_if_needed<Ent: 'static>(
        &mut self,
        url: SharedString,
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        provider: Arc<dyn CredentialsProvider>,
        cx: &mut Context<Ent>,
    ) -> Task<Result<(), AuthenticateError>> {
        if let LoadStatus::Loaded { .. } = &self.load_status
            && self.url == url
        {
            return Task::ready(Ok(()));
        }

        if let Some(key) = &self.env_var.value
            && !key.is_empty()
        {
            let api_key = ApiKey::from_env(self.env_var.name.clone(), key);
            self.url = url;
            self.load_status = LoadStatus::Loaded(api_key);
            self.load_task = None;
            cx.notify();
            return Task::ready(Ok(()));
        }

        let completion = if let Some(load_task) = &self.load_task
            && self.url == url
        {
            load_task.completion.clone()
        } else {
            let load_task = Self::load(url.clone(), get_this, provider, cx);
            let completion = load_task.completion.clone();
            self.url = url;
            self.load_status = LoadStatus::NotLoaded;
            self.load_task = Some(load_task);
            cx.notify();
            completion
        };

        cx.background_spawn(async move {
            completion.await.ok();
            Ok(())
        })
    }

    fn load<Ent: 'static>(
        url: SharedString,
        get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
        provider: Arc<dyn CredentialsProvider>,
        cx: &Context<Ent>,
    ) -> ApiKeyLoad {
        let (sender, completion) = oneshot::channel();
        let task = cx.spawn(async move |ent, cx| {
            let load_status =
                ApiKey::load_from_system_keychain_impl(&url, provider.as_ref(), cx).await;
            ent.update(cx, |ent, cx| {
                let key_state = get_this(ent);
                key_state.url = url;
                key_state.load_status = load_status;
                key_state.load_task = None;
                cx.notify();
            })
            .ok();
            sender.send(()).ok();
        });
        ApiKeyLoad {
            _task: task,
            completion: completion.shared(),
        }
    }
}

impl ApiKey {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn from_env(env_var_name: SharedString, key: &str) -> Self {
        Self {
            source: ApiKeySource::EnvVar(env_var_name),
            key: key.into(),
        }
    }

    pub async fn load_from_system_keychain(
        url: &str,
        credentials_provider: &dyn CredentialsProvider,
        cx: &AsyncApp,
    ) -> Result<Self, AuthenticateError> {
        Self::load_from_system_keychain_impl(url, credentials_provider, cx)
            .await
            .into_authenticate_result()
    }

    async fn load_from_system_keychain_impl(
        url: &str,
        credentials_provider: &dyn CredentialsProvider,
        cx: &AsyncApp,
    ) -> LoadStatus {
        if url.is_empty() {
            return LoadStatus::NotPresent;
        }
        let read_result = read_credentials(credentials_provider, url, cx).await;
        let api_key = match read_result {
            Ok(Some((_, api_key))) => api_key,
            Ok(None) => return LoadStatus::NotPresent,
            Err(err) => return LoadStatus::Error(err.to_string()),
        };
        let key = match str::from_utf8(&api_key) {
            Ok(key) => key,
            Err(_) => return LoadStatus::Error(format!("API key for URL {url} is not utf8")),
        };
        LoadStatus::Loaded(Self {
            source: ApiKeySource::SystemKeychain,
            key: key.into(),
        })
    }
}

impl LoadStatus {
    fn into_authenticate_result(self) -> Result<ApiKey, AuthenticateError> {
        match self {
            LoadStatus::Loaded(api_key) => Ok(api_key),
            LoadStatus::NotLoaded | LoadStatus::NotPresent => {
                Err(AuthenticateError::CredentialsNotFound)
            }
            LoadStatus::Error(err) => Err(AuthenticateError::Other(anyhow!(err))),
        }
    }
}

#[derive(Clone, Copy)]
struct CredentialsReadGate {
    is_disabled: fn(&App) -> bool,
    subscribe: fn(&mut App, fn(&App) -> bool, oneshot::Sender<()>) -> Subscription,
}

impl Global for CredentialsReadGate {}

struct ApiKeyLoad {
    _task: Task<()>,
    completion: future::Shared<oneshot::Receiver<()>>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use credentials_provider::test_support::FakeCredentialsProvider;
    use gpui::{Entity, TestAppContext};

    const URL: &str = "provider";
    const OTHER_URL: &str = "other-provider";

    #[gpui::test]
    async fn test_cached_miss_and_explicit_retry(cx: &mut TestAppContext) {
        for fail_reads in [false, true] {
            let test = KeyTest::new(&[], cx);
            test.provider.state.lock().fail_reads = fail_reads;
            test.load(URL, cx).await.expect("initial load");
            {
                let mut credentials = test.provider.state.lock();
                credentials.fail_reads = false;
                credentials
                    .keys
                    .insert(URL.to_string(), b"late-key".to_vec());
            }
            test.load(URL, cx).await.expect("cached miss");
            assert_eq!(test.key(URL, cx), None);
            test.provider.assert_reads(&[URL]);
            let url = SharedString::from(URL);
            let retry = test.state.update(cx, |state, cx| {
                state.load_if_needed(url, |state| state, test.provider.clone(), cx)
            });
            retry.await.expect("explicit retry");
            assert_eq!(test.key(URL, cx).as_deref(), Some("late-key"));
            test.load(URL, cx).await.expect("cached key");
            test.provider.assert_reads(&[URL, URL]);
        }
    }

    #[gpui::test]
    async fn test_disabled_gate_and_superseded_loads(cx: &mut TestAppContext) {
        cx.update(|cx| {
            set_credentials_read_gate::<DisabledAi>(|cx| cx.global::<DisabledAi>().0, cx);
        });
        let test = KeyTest::new(&[(OTHER_URL, "latest-key")], cx);
        set_disabled_ai(true, cx);
        let superseded = test.load(URL, cx);
        cx.run_until_parked();
        drop(test.load(OTHER_URL, cx));
        cx.run_until_parked();
        test.provider.assert_reads(&[]);
        set_disabled_ai(false, cx);
        set_disabled_ai(true, cx);
        cx.run_until_parked();
        test.provider.assert_reads(&[]);
        let release = test.provider.defer_read(OTHER_URL);
        set_disabled_ai(false, cx);
        cx.run_until_parked();
        drop(test.load(OTHER_URL, cx));
        set_disabled_ai(true, cx);
        cx.run_until_parked();
        test.provider.assert_reads(&[OTHER_URL]);
        release.send(()).expect("started read remains live");
        cx.run_until_parked();
        superseded.await.expect("superseded load completes");
        assert_eq!(test.key(OTHER_URL, cx).as_deref(), Some("latest-key"));
        test.provider.assert_reads(&[OTHER_URL]);
    }

    #[gpui::test]
    async fn test_store_cancels_stale_read(cx: &mut TestAppContext) {
        for stored_key in [Some("saved-key"), None] {
            let test = KeyTest::new(&[(URL, "old-key")], cx);
            let release = test.provider.defer_read(URL);
            let load = test.load(URL, cx);
            cx.run_until_parked();
            test.store(stored_key, cx).await.expect("mutation");
            cx.run_until_parked();
            assert_eq!(release.send(()), Err(()));
            load.await.expect("cancelled read completes");
            test.load(URL, cx).await.expect("cached mutation");
            assert_eq!(test.key(URL, cx).as_deref(), stored_key);
            test.provider.assert_reads(&[URL]);
        }
    }

    #[gpui::test]
    async fn test_deferred_mutations_follow_url_identity(cx: &mut TestAppContext) {
        for stored_key in [Some("saved-key"), None] {
            let test = KeyTest::new(&[(URL, "old-key"), (OTHER_URL, "other-key")], cx);
            test.load(URL, cx).await.expect("initial load");
            let release = test.provider.defer_next_mutation();
            let mutation = test.store(stored_key, cx);
            cx.run_until_parked();
            let release_read = test.provider.defer_read(OTHER_URL);
            let load = test.load(OTHER_URL, cx);
            cx.run_until_parked();
            release.send(()).expect("pending mutation");
            mutation.await.expect("mutation for inactive URL");
            drop(test.load(OTHER_URL, cx));
            cx.run_until_parked();
            assert_eq!(test.key(OTHER_URL, cx), None);
            release_read.send(()).expect("other URL read remains live");
            load.await.expect("other URL load");
            assert_eq!(test.key(OTHER_URL, cx).as_deref(), Some("other-key"));
            assert_eq!(
                test.provider.state.lock().keys.get(URL).map(Vec::as_slice),
                stored_key.map(str::as_bytes)
            );
            test.provider.assert_reads(&[URL, OTHER_URL]);
        }
    }

    type LoadTask = Task<Result<(), AuthenticateError>>;

    struct DisabledAi(bool);

    impl Global for DisabledAi {}

    struct KeyTest {
        state: Entity<ApiKeyState>,
        provider: Arc<FakeCredentialsProvider>,
    }

    impl KeyTest {
        fn new(keys: &[(&str, &str)], cx: &mut TestAppContext) -> Self {
            let provider = Arc::new(FakeCredentialsProvider::default());
            provider.state.lock().keys.extend(
                keys.iter()
                    .map(|(url, key)| (url.to_string(), key.as_bytes().to_vec())),
            );
            let state = cx.new(|_| {
                ApiKeyState::new(
                    SharedString::default(),
                    EnvVar {
                        name: SharedString::new_static("TEST_API_KEY"),
                        value: None,
                    },
                )
            });
            Self { state, provider }
        }

        fn load(&self, url: &'static str, cx: &mut TestAppContext) -> LoadTask {
            let url = SharedString::from(url);
            self.state.update(cx, |state, cx| {
                state.load_cached(url, |state| state, self.provider.clone(), cx)
            })
        }

        fn store(&self, key: Option<&str>, cx: &mut TestAppContext) -> Task<Result<()>> {
            let url = SharedString::from(URL);
            let key = key.map(String::from);
            self.state.update(cx, |state, cx| {
                state.store(url, key, |state| state, self.provider.clone(), cx)
            })
        }

        fn key(&self, url: &str, cx: &TestAppContext) -> Option<Arc<str>> {
            self.state.read_with(cx, |state, _| state.key(url))
        }
    }

    fn set_disabled_ai(disabled: bool, cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(DisabledAi(disabled)));
    }
}
