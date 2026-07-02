# Running tokio-based futures from GPUI code via gpui_tokio (Zed codebase API reference)

# gpui_tokio API reference (Zed @ main, bb48a42983)

## 1. Public API — `crates/gpui_tokio` (single file: `/Users/user/zed/crates/gpui_tokio/src/gpui_tokio.rs`, 100 lines)

Crate manifest (`/Users/user/zed/crates/gpui_tokio/Cargo.toml`): `[lib] path = "src/gpui_tokio.rs"`, deps = `anyhow`, `gpui`, `gpui_util`, `tokio = { workspace = true, features = ["rt", "rt-multi-thread"] }`.

Re-export (line 6): `pub use tokio::task::JoinError;`

### `init` — creates a 2-worker-thread multi-thread runtime and stores it in a GPUI global (lines 8–25)
```rust
/// Initializes the Tokio wrapper using a new Tokio runtime with 2 worker threads.
///
/// If you need more threads (or access to the runtime outside of GPUI), you can create the runtime
/// yourself and pass a Handle to `init_from_handle`.
pub fn init(cx: &mut App) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        // Since we now have two executors, let's try to keep our footprint small
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio");

    let handle = runtime.handle().clone();
    cx.set_global(GlobalTokio { owned_runtime: Some(runtime), handle });
}
```

### `init_from_handle` (lines 27–33)
```rust
/// Initializes the Tokio wrapper using a Tokio runtime handle.
pub fn init_from_handle(cx: &mut App, handle: tokio::runtime::Handle)
```

Internal global (lines 35–48, private): `struct GlobalTokio { owned_runtime: Option<tokio::runtime::Runtime>, handle: tokio::runtime::Handle }` with `impl Global for GlobalTokio {}`; its `Drop` calls `runtime.shutdown_background()`.

### `Tokio` — zero-sized namespace struct (lines 50–100)
```rust
pub struct Tokio {}

impl Tokio {
    /// Spawns the given future on Tokio's thread pool, and returns it via a GPUI task
    /// Note that the Tokio task will be cancelled if the GPUI task is dropped
    pub fn spawn<C, Fut, R>(cx: &C, f: Fut) -> Task<Result<R, JoinError>>
    where
        C: AppContext,
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static;

    /// Same, but flattens: future returns anyhow::Result<R>, JoinError is converted via `?`
    pub fn spawn_result<C, Fut, R>(cx: &C, f: Fut) -> Task<anyhow::Result<R>>
    where
        C: AppContext,
        Fut: Future<Output = anyhow::Result<R>> + Send + 'static,
        R: Send + 'static;

    pub fn handle(cx: &App) -> tokio::runtime::Handle;
}
```
Implementation detail (lines 61–72): `Tokio::spawn` does `tokio.handle.spawn(f)`, wraps the `JoinHandle` in a `cx.background_spawn(...)` GPUI task, and registers a `gpui_util::defer` guard that calls `abort_handle.abort()` — so **dropping the returned GPUI `Task` aborts the tokio task**. Because `C: AppContext`, `cx` may be `&App`, `&mut Context<T>`, `&mut AsyncApp`, or `&mut TestAppContext`. `Tokio::handle` takes `&App` only (use `cx.update(|cx| Tokio::handle(cx))` style from async, or capture the handle earlier).

`Tokio::spawn(...).await` yields `Result<R, JoinError>` (so call sites often `.await??`); `Tokio::spawn_result(...).await` yields `anyhow::Result<R>`.

## 2. Real call sites

### (a) `cloud_api_client` — return the tokio work directly as a GPUI `Task` (`/Users/user/zed/crates/cloud_api_client/src/cloud_api_client.rs:128–155`; import at line 11: `use gpui_tokio::Tokio;`)
```rust
pub fn connect(&self, cx: &App) -> Result<Task<Result<Connection>>> {
    // ... build connect_url, authorization_header ...
    Ok(Tokio::spawn_result(cx, async move {
        let ws = WebSocket::connect(connect_url)
            .with_request(
                request::Builder::new()
                    .header("Authorization", authorization_header)
                    .header(PROTOCOL_VERSION_HEADER_NAME, PROTOCOL_VERSION.to_string()),
            )
            .await?;
        Ok(Connection::new(ws))
    }))
}
```
The tokio-dependent library work (`yawc` WebSocket) runs on the tokio pool; caller awaits the returned `Task<Result<Connection>>` on the GPUI executor.

### (b) `client` — tokio hop inside a `cx.spawn`, result flows back to the GPUI async fn (`/Users/user/zed/crates/client/src/client.rs:1341–1370`)
```rust
cx.spawn(async move |cx| {                       // cx: &mut AsyncApp
    // ...
    let stream = gpui_tokio::Tokio::spawn_result(cx, {
        let rpc_url = rpc_url.clone();
        async move {
            let rpc_host = rpc_url
                .host_str()
                .zip(rpc_url.port_or_known_default())
                .context("missing host in rpc url")?;
            Ok(match proxy {
                Some(proxy) => connect_proxy_stream(&proxy, rpc_host).await?,
                None => Box::new(TcpStream::connect(rpc_host).await?),
            })
        }
    })
    .await?;
    // continues on GPUI executor using `stream`...
})
```

### (c) `extension_host` — `Tokio::spawn` awaited with `??`, plus a long-lived tokio task kept alive by storing the GPUI Task (`/Users/user/zed/crates/extension_host/src/wasm_host.rs:707–727`)
```rust
cx.spawn(async move |cx| {
    let (zed_api_version, component) = compile_task.await?;
    // Run wasi-dependent operations on tokio.
    // wasmtime_wasi internally uses tokio for I/O operations.
    let (extension_task, manifest, work_dir, tx, zed_api_version) =
        gpui_tokio::Tokio::spawn(cx, load_extension(zed_api_version, component)).await??;
    // Run the extension message loop on tokio since extension
    // calls may invoke wasi functions that require a tokio runtime.
    let task = Arc::new(gpui_tokio::Tokio::spawn(cx, extension_task));
    Ok(WasmExtension { manifest, work_dir, tx, zed_api_version, _task: task })
})
```
(Storing `_task` keeps the tokio loop alive; dropping it aborts, per the defer guard.)

### (d) Other notable patterns
- `livekit_client` `Room::connect` (`/Users/user/zed/crates/livekit_client/src/livekit_client.rs:52–81`): `let (room, mut events) = Tokio::spawn(cx, async move { livekit::Room::connect(&url, &token, config).await }).await??;` with `cx: &mut AsyncApp`.
- `language_models` bedrock provider (`/Users/user/zed/crates/language_models/src/provider/bedrock.rs:636–640`): `let task = Tokio::spawn(cx, bedrock::stream_completion(runtime_client, request, extra_headers)); async move { task.await.map_err(...)? }.boxed()`. It also stores the raw handle in a struct field: `handle: Tokio::handle(cx)` (line 432).
- **`Tokio::handle(cx).enter()` guard pattern** for constructing tokio-dependent clients synchronously (e.g. reqwest needs an ambient runtime), `/Users/user/zed/crates/zed/src/main.rs:511–516`:
```rust
let http = {
    let _guard = Tokio::handle(cx).enter();
    ReqwestClient::proxy_and_user_agent(proxy_url, &user_agent)
        .expect("could not start HTTP client")
};
cx.set_http_client(Arc::new(http));
```
Same pattern in `/Users/user/zed/crates/remote_server/src/server.rs:689`, `/Users/user/zed/crates/eval_cli/src/headless.rs:57`, `/Users/user/zed/crates/edit_prediction_cli/src/headless.rs:55`. `call/src/call_impl/room.rs:1740` uses `let guard = Tokio::handle(cx);` (then `guard.enter()`).

## 3. Where `init` runs at startup, and what forgetting it does

- Main app: `/Users/user/zed/crates/zed/src/main.rs:495` — `gpui_tokio::init(cx);` inside `app.run(...)`, early (right after `release_channel::init`, before HTTP client / settings-dependent subsystems). Also `/Users/user/zed/crates/zed/src/zed.rs:5554` (test init path).
- Headless binaries: `/Users/user/zed/crates/remote_server/src/server.rs:657`, `/Users/user/zed/crates/eval_cli/src/headless.rs:39`, `/Users/user/zed/crates/edit_prediction_cli/src/headless.rs:37`.
- Tests must call it themselves, e.g. `/Users/user/zed/crates/agent/src/tests/mod.rs:3858`, `/Users/user/zed/crates/language_models/src/language_models.rs:442`, `/Users/user/zed/crates/extension_host/src/wasm_host.rs:1013`, `/Users/user/zed/crates/collab/tests/integration/test_server.rs:171`.

**If a crate forgets init:** `Tokio::spawn`/`spawn_result` call `cx.read_global::<GlobalTokio>` and `Tokio::handle` calls `GlobalTokio::global(cx)`; a missing global hits `App::global` → `panic!("no state of type {} exists", type_name::<G>())` (`/Users/user/zed/crates/gpui/src/app.rs:1817–1821`, same message at 1839). So the first `Tokio::*` call panics at runtime with `no state of type gpui_tokio::GlobalTokio exists`. This is a per-`App` global — every binary/test `App` needs its own `gpui_tokio::init(cx)` (or `init_from_handle`). Calling `init` twice just replaces the global (old owned runtime is shut down via `Drop` → `shutdown_background()`).

## 4. Workspace tokio / postgres dependency audit

Root `/Users/user/zed/Cargo.toml` `[workspace.dependencies]`:
- `tokio = { version = "1" }` (line 784) — resolves to a **single** `tokio 1.52.1` in Cargo.lock (line 18786; exactly one `name = "tokio"` entry).
- `tokio-socks = { version = "0.5.2", default-features = false, features = ["futures-io", "tokio"] }` (line 785).
- `gpui_tokio = { path = "crates/gpui_tokio" }` (line 358).
- Cargo.lock also contains transitive tokio-family crates: `tokio-io`, `tokio-macros`, `tokio-native-tls`, `tokio-rustls` (x2 versions), `tokio-stream`, `tokio-tungstenite` (x3 versions), `tokio-util`.

Postgres:
- **No `tokio-postgres` and no `rust-postgres`/`postgres` crate anywhere in Cargo.lock.** The only postgres-related lock entries are `sqlx-postgres` (Cargo.lock lines 17306, 17382, 17437).
- The only workspace member with a postgres client is **`collab`** (the server, not the editor), via sea-orm/sqlx — `/Users/user/zed/crates/collab/Cargo.toml`:
  - line 56: `sea-orm = { version = "=1.1.10", features = ["sqlx-postgres", "postgres-array", "runtime-tokio-rustls", "with-uuid"] }`
  - line 62: `sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "json", "time", "uuid", "any"] }`
  - line 17: feature `sqlite = ["sea-orm/sqlx-sqlite", "sqlx/sqlite"]` (dev/tests use sqlite; dev-deps lines 122, 127).
- Neither `sea-orm` nor `sqlx` is in root `[workspace.dependencies]` — collab pins them directly. So any editor-side crate wanting a postgres client would add a new dependency; `sqlx` (runtime-tokio) is precedent-compatible with `gpui_tokio`'s runtime via `Tokio::spawn`/`Tokio::handle(cx).enter()`.