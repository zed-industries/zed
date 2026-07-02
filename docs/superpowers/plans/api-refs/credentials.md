# Storing secrets in the OS keychain from Zed (CredentialsProvider API)

# Storing secrets in the OS keychain from Zed

## 1. The `CredentialsProvider` trait

Defined in `/Users/user/zed/crates/credentials_provider/src/credentials_provider.rs` (whole file is 34 lines; crate name `credentials_provider`, lib path `src/credentials_provider.rs`):

```rust
use std::future::Future;
use std::pin::Pin;
use anyhow::Result;
use gpui::AsyncApp;

/// A provider for credentials.
///
/// Used to abstract over reading and writing credentials to some form of
/// persistence (like the system keychain).
pub trait CredentialsProvider: Send + Sync {
    /// Reads the credentials from the provider.
    fn read_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>>;

    /// Writes the credentials to the provider.
    fn write_credentials<'a>(
        &'a self,
        url: &'a str,
        username: &'a str,
        password: &'a [u8],
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>>;

    /// Deletes the credentials from the provider.
    fn delete_credentials<'a>(
        &'a self,
        url: &'a str,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>>;
}
```

Parameter semantics:
- `url` — the keychain lookup key. One credential per `url`. On macOS it becomes `kSecAttrServer` of a `kSecClassInternetPassword` item; on Linux it is the `("url", ...)` attribute in the Secret Service keyring. Conventions in the codebase: Zed client uses the collab server URL (or the `credentials_url` setting override, `client.rs:106-114,371-374`); LLM providers use the provider API endpoint URL (e.g. `https://api.anthropic.com`).
- `username` — the account field stored alongside the secret. Zed client stores the user id as a string (`client.rs:414`); LLM providers store the literal string `"Bearer"` (`api_key.rs:115`).
- `password` — arbitrary secret bytes (`&[u8]`). `read_credentials` returns `Ok(Some((username, password_bytes)))`, `Ok(None)` if absent (also `Ok(None)` if the user cancels the macOS keychain prompt).

The futures are **boxed local (non-`Send`)** and take `&AsyncApp`, so you must await them from a **foreground** async context (`cx.spawn`), never `cx.background_spawn`. The platform impl itself does the blocking keychain I/O on the background executor internally.

## 2. Obtaining an instance — global accessor

`/Users/user/zed/crates/zed_credentials_provider/src/zed_credentials_provider.rs` (crate `zed_credentials_provider`):

```rust
pub struct ZedCredentialsProvider(pub Arc<dyn CredentialsProvider>);   // line 26
impl Global for ZedCredentialsProvider {}                              // line 28

pub fn init_global(cx: &mut App)                                       // line 31 — sets the global
pub fn global(cx: &App) -> Arc<dyn CredentialsProvider>                // line 39
```

`global(cx)` reads `cx.try_global::<ZedCredentialsProvider>()` and, if the global was never set, **falls back to constructing a fresh provider** (`unwrap_or_else(|| new(cx))`, lines 39-43) — so it always works. (`init_global` currently has no callers in the repo; the fallback path is what runs.) Selection logic in `new()` (lines 45-66): `ReleaseChannel::Dev` → `DevelopmentCredentialsProvider` unless env var `ZED_DEVELOPMENT_USE_KEYCHAIN` is set non-empty; Nightly/Preview/Stable/None → `KeychainCredentialsProvider`.

`KeychainCredentialsProvider` (lines 69-101) just forwards to GPUI: e.g.

```rust
async move { cx.update(|cx| cx.read_credentials(url)).await }.boxed_local()
```

Alternative accessor: if you have an `Arc<Client>`, `client.credentials_provider()` returns the same `Arc<dyn CredentialsProvider>` (`/Users/user/zed/crates/client/src/client.rs:602-604`).

Underlying GPUI API (usable directly if you prefer): `App::write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>>`, `App::read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>>`, `App::delete_credentials(&self, url: &str) -> Task<Result<()>>` at `/Users/user/zed/crates/gpui/src/app.rs:1337-1354`; `Platform` trait methods at `/Users/user/zed/crates/gpui/src/platform.rs:245-247`.

## 3. Real call sites

### 3a. Simplest read (agent_servers) — `/Users/user/zed/crates/agent_servers/src/custom.rs:288-303`

```rust
fn api_key_for_gemini_cli(cx: &mut App) -> Task<Result<String>> {
    let env_var = EnvVar::new("GEMINI_API_KEY".into()).or(EnvVar::new("GOOGLE_AI_API_KEY".into()));
    if let Some(key) = env_var.value {
        return Task::ready(Ok(key));
    }
    let credentials_provider = zed_credentials_provider::global(cx);
    let api_url = google_ai::API_URL.to_string();
    cx.spawn(async move |cx| {
        Ok(
            ApiKey::load_from_system_keychain(&api_url, credentials_provider.as_ref(), cx)
                .await?
                .key()
                .to_string(),
        )
    })
}
```

Pattern: grab the `Arc` from the global while you have `&App`, move it into `cx.spawn` (foreground), await there.

### 3b. Write + delete (language_model ApiKeyState) — `/Users/user/zed/crates/language_model/src/api_key.rs:99-134`

```rust
pub fn store<Ent: 'static>(
    &mut self,
    url: SharedString,
    key: Option<String>,                      // None = delete
    get_this: impl Fn(&mut Ent) -> &mut Self + 'static,
    provider: Arc<dyn CredentialsProvider>,
    cx: &Context<Ent>,
) -> Task<Result<()>> {
    ...
    cx.spawn(async move |ent, cx| {
        if let Some(key) = &key {
            provider
                .write_credentials(&url, "Bearer", key.as_bytes(), cx)
                .await
                .log_err();
        } else {
            provider.delete_credentials(&url, cx).await.log_err();
        }
        ent.update(cx, |ent, cx| { /* update LoadStatus, cx.notify() */ })
    })
}
```

And the raw read (`api_key.rs:250-272`):

```rust
let read_result = credentials_provider.read_credentials(&url, cx).await;   // cx: &AsyncApp
let api_key = match read_result {
    Ok(Some((_, api_key))) => api_key,     // ignores username, takes Vec<u8>
    Ok(None) => return LoadStatus::NotPresent,
    Err(err) => return LoadStatus::Error(err.to_string()),
};
let key = str::from_utf8(&api_key) ...
```

### 3c. Zed client sign-in credentials — `/Users/user/zed/crates/client/src/client.rs:355-433`

`ClientCredentialsProvider::new(cx: &App)` stores `provider: zed_credentials_provider::global(cx)` (line 362). Read (lines 377-400): `self.provider.read_credentials(&credentials_url, cx).await.log_err().flatten()?` then parses `user_id` from the returned username `String` and the access token from the `Vec<u8>`. Write (lines 403-421):

```rust
self.provider
    .write_credentials(&credentials_url, &user_id.to_string(), access_token.as_bytes(), cx)
    .await
```

Used from `Client::authenticate_and_connect` (`client.rs:893-930`), always inside async code holding `&AsyncApp`.

### 3d. Getting the global from an `AsyncApp` — `/Users/user/zed/crates/project/src/context_server_store.rs:744-751`

```rust
cx.spawn(async move |_this, cx| {
    let credentials_provider = cx.update(|cx| zed_credentials_provider::global(cx));
    if let Err(err) = Self::clear_session(&credentials_provider, &server_url, &cx).await {
        log::warn!("{} failed to clear OAuth session on removal: {}", id, err);
    }
})
.detach();
```

### 3e. Higher-level helper for API keys (recommended for provider-style secrets)

`ApiKeyState` in `/Users/user/zed/crates/language_model/src/api_key.rs` wraps the whole lifecycle (env-var override, dedup of in-flight loads, URL-change handling):

- `ApiKeyState::new(url: SharedString, env_var: EnvVar) -> Self` (line 39)
- `pub fn key(&self, url: &str) -> Option<Arc<str>>` (line 71)
- `pub fn store<Ent>(...) -> Task<Result<()>>` (line 99) — write/delete
- `pub fn handle_url_change<Ent>(...)` (line 140)
- `pub fn load_if_needed<Ent>(...) -> Task<Result<(), AuthenticateError>>` (line 160)
- `pub async fn load_from_system_keychain(url, &dyn CredentialsProvider, &AsyncApp) -> Result<ApiKey, AuthenticateError>` (line 240)

Usage in `/Users/user/zed/crates/language_models/src/provider/anthropic.rs:60-101` (`State::set_api_key` calls `self.api_key_state.store(api_url, api_key, |this| &mut this.api_key_state, credentials_provider, cx)`; `State::authenticate` calls `load_if_needed`). The `credentials_provider: Arc<dyn CredentialsProvider>` there comes from `client.credentials_provider()` in `/Users/user/zed/crates/language_models/src/language_models.rs:39`.

## 4. Dev-mode fallback and platform notes

- **Dev fallback**: `DevelopmentCredentialsProvider` (`zed_credentials_provider.rs:110-181`) stores a `HashMap<String, (String, Vec<u8>)>` as JSON in `paths::config_dir().join("development_credentials")`. It exists to avoid repeated macOS keychain prompts for unsigned dev builds; explicitly not secure. Active only on `ReleaseChannel::Dev`; setting env var `ZED_DEVELOPMENT_USE_KEYCHAIN=1` opts in to the real keychain in dev (`zed_credentials_provider.rs:22-24,46-59`).
- **macOS** (`/Users/user/zed/crates/gpui_macos/src/platform.rs:1082-1186`): Security.framework, `kSecClassInternetPassword` keyed by `kSecAttrServer = url`; write does `SecItemUpdate` first, falling back to `SecItemAdd` on `errSecItemNotFound`; read returns `Ok(None)` for both `errSecItemNotFound` and `errSecUserCanceled`. All three run on `self.background_executor().spawn(...)`.
- **Linux** (`/Users/user/zed/crates/gpui_linux/src/linux/platform.rs:619-676`): `oo7::Keyring::new().await` + `keyring.unlock().await`, items matched by attribute `("url", &url)`; requires a Secret Service (or key portal) implementation to be present on the system.
- **Tests**: `TestPlatform`/visual-test platform stubs (`/Users/user/zed/crates/gpui/src/platform/test/platform.rs:480-490`) do not persist; in GPUI tests inject a fake `CredentialsProvider` instead (see `FakeCredentialsProvider` in `/Users/user/zed/crates/language_models/src/language_models.rs:409`).
- Trait futures are non-`Send` (`boxed_local`) — awaiting must happen on the foreground thread via `cx.spawn`; the blocking OS keychain work is already backgrounded inside the platform layer.