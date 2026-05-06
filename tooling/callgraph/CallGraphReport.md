# Callgraph Analysis Report

Blocking calls detected inside `async fn` bodies across all crates.

Generated: 2026-05-06 00:38 UTC

---
**Summary**: 81 finding(s) across 11 crate(s).


## `agent_servers` — 2 finding(s)

```
warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/agent_servers/src/e2e_tests.rs:59:5
   │
59 │     std::fs::write(
   │     ^^^^^^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
   │
   = help: use an async fs API or wrap in `smol::unblock`
   = context: async fn `test_path_mentions`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/agent_servers/src/e2e_tests.rs:118:5
    │
118 │     std::fs::write(&foo_path, "Lorem ipsum dolor").expect("failed to write file");
    │     ^^^^^^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `test_tool_call`


```

## `collab` — 2 finding(s)

```
warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/collab/src/main.rs:76:28
   │
76 │             let listener = TcpListener::bind(format!("0.0.0.0:{}", config.http_port))
   │                            ^^^^^^^^^^^^^^^^^ `std::net::TcpListener::bind` is a blocking network operation
   │
   = help: use an async networking library
   = context: async fn `main`

warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/collab/src/seed.rs:94:31
   │
94 │         serde_json::from_str(&fs::read_to_string(github_users_filepath)?)?;
   │                               ^^^^^^^^^^^^^^^^^^ `std::fs::read_to_string` is a blocking filesystem operation
   │
   = help: use an async fs API or wrap in `smol::unblock`
   = context: async fn `seed`


```

## `dap_adapters` — 3 finding(s)

```
warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/dap_adapters/src/python.rs:121:9
    │
121 │         std::fs::create_dir_all(&download_dir)?;
    │         ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `fetch_wheel`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/dap_adapters/src/python.rs:143:26
    │
143 │         let wheel_path = std::fs::read_dir(&download_dir)?
    │                          ^^^^^^^^^^^^^^^^^ `std::fs::read_dir` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `fetch_wheel`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/dap_adapters/src/python.rs:180:9
    │
180 │         std::fs::create_dir_all(&download_dir)?;
    │         ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `maybe_fetch_new_wheel`


```

## `edit_prediction_cli` — 18 finding(s)

```
warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/synthesize.rs:107:5
    │
107 │     std::fs::create_dir_all(&config.output_dir)?;
    │     ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_synthesize`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/synthesize.rs:108:5
    │
108 │     std::fs::create_dir_all(&*FAILED_EXAMPLES_DIR)?;
    │     ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_synthesize`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/synthesize.rs:112:9
    │
112 │         std::fs::remove_file(&*LATEST_FAILED_EXAMPLES_DIR)?;
    │         ^^^^^^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_synthesize`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/synthesize.rs:240:21
    │
240 │                     std::fs::write(&path, spec.to_markdown())?;
    │                     ^^^^^^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `synthesize_repo`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/synthesize.rs:250:37
    │
250 │                     if let Err(e) = std::fs::write(&path, content) {
    │                                     ^^^^^^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `synthesize_repo`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/predict.rs:200:29
    │
200 │                             fs::write(run_dir.join("prediction_prompt.md"), &prompt)?;
    │                             ^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_prediction`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/predict.rs:216:29
    │
216 │                             fs::write(run_dir.join("prediction_response.md"), &output)?;
    │                             ^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_prediction`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/predict.rs:252:9
    │
252 │         fs::create_dir_all(&run_dir)?;
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_prediction`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/predict.rs:254:13
    │
254 │             fs::remove_file(&*LATEST_EXAMPLE_RUN_DIR)?;
    │             ^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_prediction`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/predict.rs:761:9
    │
761 │         std::thread::sleep(poll_interval);
    │         ^^^^^^^^^^^^^^^^^^ `std::thread::sleep` is a blocking thread operation
    │
    = help: use `cx.background_executor().timer(duration).await`
    = context: async fn `wait_for_batches`

warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/git.rs:80:13
   │
80 │             std::fs::remove_dir_all(&repo_path).ok();
   │             ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::remove_dir_all` is a blocking filesystem operation
   │
   = help: use an async fs API or wrap in `smol::unblock`
   = context: async fn `ensure_repo_cloned`

warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/git.rs:86:9
   │
86 │         std::fs::create_dir_all(&repo_path)?;
   │         ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
   │
   = help: use an async fs API or wrap in `smol::unblock`
   = context: async fn `ensure_repo_cloned`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/load_project.rs:277:13
    │
277 │             fs::remove_file(&worktree_lock_path).ok();
    │             ^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `setup_worktree`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/load_project.rs:280:13
    │
280 │             fs::remove_file(&repo_lock_path).ok();
    │             ^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `setup_worktree`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/load_project.rs:292:13
    │
292 │             fs::remove_dir_all(&repo_dir).ok();
    │             ^^^^^^^^^^^^^^^^^^ `std::fs::remove_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `setup_worktree`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/load_project.rs:298:9
    │
298 │         fs::create_dir_all(&repo_dir)?;
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `setup_worktree`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/load_project.rs:332:13
    │
332 │             fs::remove_dir_all(&worktree_path).ok();
    │             ^^^^^^^^^^^^^^^^^^ `std::fs::remove_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `setup_worktree`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/edit_prediction_cli/src/repair.rs:510:9
    │
510 │         std::thread::sleep(poll_interval);
    │         ^^^^^^^^^^^^^^^^^^ `std::thread::sleep` is a blocking thread operation
    │
    = help: use `cx.background_executor().timer(duration).await`
    = context: async fn `wait_for_batches`


```

## `eval_cli` — 2 finding(s)

```
warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/eval_cli/src/main.rs:498:25
    │
498 │         if let Err(e) = std::fs::write(dir.join("thread.md"), markdown) {
    │                         ^^^^^^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_agent`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/eval_cli/src/main.rs:506:33
    │
506 │                 if let Err(e) = std::fs::write(dir.join("thread.json"), json) {
    │                                 ^^^^^^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `run_agent`


```

## `extension` — 14 finding(s)

```
warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:99:9
   │
99 │         fs::create_dir_all(&self.cache_dir).context("failed to create cache dir")?;
   │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
   │
   = help: use an async fs API or wrap in `smol::unblock`
   = context: async fn `compile_extension`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:113:40
    │
113 │             let debug_adapter_schema = fs::read_to_string(&debug_adapter_schema_path)
    │                                        ^^^^^^^^^^^^^^^^^^ `std::fs::read_to_string` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `compile_extension`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:154:34
    │
154 │         let cargo_toml_content = fs::read_to_string(extension_dir.join("Cargo.toml"))?;
    │                                  ^^^^^^^^^^^^^^^^^^ `std::fs::read_to_string` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `compile_rust_extension`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:202:31
    │
202 │         let component_bytes = fs::read(&wasm_path)
    │                               ^^^^^^^^ `std::fs::read` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `compile_rust_extension`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:215:9
    │
215 │         fs::write(extension_file.clone(), &component_bytes)
    │         ^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `compile_rust_extension`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:313:13
    │
313 │             fs::create_dir_all(directory).with_context(|| {
    │             ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `checkout_repo`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:427:12
    │
427 │         if fs::metadata(&clang_path).is_ok_and(|metadata| metadata.is_file()) {
    │            ^^^^^^^^^^^^ `std::fs::metadata` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `install_wasi_sdk_if_needed`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:433:9
    │
433 │         fs::remove_dir_all(&wasi_sdk_dir).ok();
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::remove_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `install_wasi_sdk_if_needed`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:434:9
    │
434 │         fs::remove_dir_all(&tar_out_dir).ok();
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::remove_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `install_wasi_sdk_if_needed`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:435:9
    │
435 │         fs::create_dir_all(&tar_out_dir).context("failed to create extraction directory")?;
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `install_wasi_sdk_if_needed`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:473:9
    │
473 │         fs::remove_file(&tar_gz_path).ok();
    │         ^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `install_wasi_sdk_if_needed`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:475:25
    │
475 │         let inner_dir = fs::read_dir(&tar_out_dir)?
    │                         ^^^^^^^^^^^^ `std::fs::read_dir` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `install_wasi_sdk_if_needed`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:480:9
    │
480 │         fs::rename(&inner_dir, &wasi_sdk_dir).context("failed to move extracted wasi dir")?;
    │         ^^^^^^^^^^ `std::fs::rename` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `install_wasi_sdk_if_needed`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension/src/extension_builder.rs:481:9
    │
481 │         fs::remove_dir_all(&tar_out_dir).ok();
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::remove_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `install_wasi_sdk_if_needed`


```

## `extension_cli` — 18 finding(s)

```
warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:93:5
   │
93 │     fs::remove_dir_all(&archive_dir).ok();
   │     ^^^^^^^^^^^^^^^^^^ `std::fs::remove_dir_all` is a blocking filesystem operation
   │
   = help: use an async fs API or wrap in `smol::unblock`
   = context: async fn `main`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:123:5
    │
123 │     fs::remove_dir_all(&archive_dir)?;
    │     ^^^^^^^^^^^^^^^^^^ `std::fs::remove_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `main`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:124:5
    │
124 │     fs::write(output_dir.join("manifest.json"), manifest_json.as_bytes())?;
    │     ^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `main`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:135:5
    │
135 │     fs::create_dir_all(output_dir).context("failed to create output dir")?;
    │     ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:138:5
    │
138 │     fs::write(output_dir.join("extension.toml"), &manifest_toml)
    │     ^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:142:9
    │
142 │         fs::copy(
    │         ^^^^^^^^ `std::fs::copy` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:152:9
    │
152 │         fs::create_dir_all(&output_grammars_dir)?;
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:156:13
    │
156 │             fs::copy(
    │             ^^^^^^^^ `std::fs::copy` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:166:9
    │
166 │         fs::create_dir_all(&output_themes_dir)?;
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:169:13
    │
169 │             fs::copy(
    │             ^^^^^^^^ `std::fs::copy` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:179:9
    │
179 │         fs::create_dir_all(&output_icon_themes_dir)?;
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:182:13
    │
182 │             fs::copy(
    │             ^^^^^^^^ `std::fs::copy` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:196:9
    │
196 │         fs::create_dir_all(&output_icons_dir)?;
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:217:17
    │
217 │                 fs::create_dir_all(parent)?;
    │                 ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:220:13
    │
220 │             fs::copy(&source_icon, &dest_icon)
    │             ^^^^^^^^ `std::fs::copy` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:227:9
    │
227 │         fs::create_dir_all(&output_languages_dir)?;
    │         ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:254:13
    │
254 │             fs::create_dir_all(output_dir.join(parent))?;
    │             ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/extension_cli/src/main.rs:278:17
    │
278 │                 fs::create_dir_all(output_dir.join(parent))?;
    │                 ^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `copy_extension_resources`


```

## `fs` — 10 finding(s)

```
warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/fs/src/fs.rs:967:17
    │
967 │                 std::fs::read_to_string(&path)
    │                 ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::read_to_string` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `load`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/fs/src/fs.rs:977:33
    │
977 │             .spawn(async move { std::fs::read(path) })
    │                                 ^^^^^^^^^^^^^ `std::fs::read` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `load_bytes`

warning[blocking-in-async]: blocking call in async context
     ┌─ /home/mrg/zed/crates/fs/src/fs.rs:1056:17
     │
1056 │                 std::fs::write(path, contents)?;
     │                 ^^^^^^^^^^^^^^ `std::fs::write` is a blocking filesystem operation
     │
     = help: use an async fs API or wrap in `smol::unblock`
     = context: async fn `write`

warning[blocking-in-async]: blocking call in async context
     ┌─ /home/mrg/zed/crates/fs/src/fs.rs:1070:30
     │
1070 │                 let result = std::fs::canonicalize(&path);
     │                              ^^^^^^^^^^^^^^^^^^^^^ `std::fs::canonicalize` is a blocking filesystem operation
     │
     = help: use an async fs API or wrap in `smol::unblock`
     = context: async fn `canonicalize`

warning[blocking-in-async]: blocking call in async context
     ┌─ /home/mrg/zed/crates/fs/src/fs.rs:1080:33
     │
1080 │             .spawn(async move { std::fs::metadata(path).is_ok_and(|metadata| metadata.is_file()) })
     │                                 ^^^^^^^^^^^^^^^^^ `std::fs::metadata` is a blocking filesystem operation
     │
     = help: use an async fs API or wrap in `smol::unblock`
     = context: async fn `is_file`

warning[blocking-in-async]: blocking call in async context
     ┌─ /home/mrg/zed/crates/fs/src/fs.rs:1087:33
     │
1087 │             .spawn(async move { std::fs::metadata(path).is_ok_and(|metadata| metadata.is_dir()) })
     │                                 ^^^^^^^^^^^^^^^^^ `std::fs::metadata` is a blocking filesystem operation
     │
     = help: use an async fs API or wrap in `smol::unblock`
     = context: async fn `is_dir`

warning[blocking-in-async]: blocking call in async context
     ┌─ /home/mrg/zed/crates/fs/src/fs.rs:1095:33
     │
1095 │             .spawn(async move { std::fs::symlink_metadata(&path_buf) })
     │                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::symlink_metadata` is a blocking filesystem operation
     │
     = help: use an async fs API or wrap in `smol::unblock`
     = context: async fn `metadata`

warning[blocking-in-async]: blocking call in async context
     ┌─ /home/mrg/zed/crates/fs/src/fs.rs:1113:37
     │
1113 │                 .spawn(async move { std::fs::metadata(path_buf) })
     │                                     ^^^^^^^^^^^^^^^^^ `std::fs::metadata` is a blocking filesystem operation
     │
     = help: use an async fs API or wrap in `smol::unblock`
     = context: async fn `metadata`

warning[blocking-in-async]: blocking call in async context
     ┌─ /home/mrg/zed/crates/fs/src/fs.rs:1167:33
     │
1167 │             .spawn(async move { std::fs::read_link(&path) })
     │                                 ^^^^^^^^^^^^^^^^^^ `std::fs::read_link` is a blocking filesystem operation
     │
     = help: use an async fs API or wrap in `smol::unblock`
     = context: async fn `read_link`

warning[blocking-in-async]: blocking call in async context
     ┌─ /home/mrg/zed/crates/fs/src/fs.rs:1179:37
     │
1179 │                 .spawn(async move { std::fs::read_dir(path) })
     │                                     ^^^^^^^^^^^^^^^^^ `std::fs::read_dir` is a blocking filesystem operation
     │
     = help: use an async fs API or wrap in `smol::unblock`
     = context: async fn `read_dir`


```

## `remote_server` — 5 finding(s)

```
warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/remote_server/src/server.rs:935:9
    │
935 │         std::fs::remove_file(&paths.stdin_socket).map_err(SpawnServerError::RemoveStdinSocket)?;
    │         ^^^^^^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `spawn_server`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/remote_server/src/server.rs:938:9
    │
938 │         std::fs::remove_file(&paths.stdout_socket).map_err(SpawnServerError::RemoveStdoutSocket)?;
    │         ^^^^^^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `spawn_server`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/remote_server/src/server.rs:941:9
    │
941 │         std::fs::remove_file(&paths.stderr_socket).map_err(SpawnServerError::RemoveStderrSocket)?;
    │         ^^^^^^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `spawn_server`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/remote_server/src/server.rs:963:9
    │
963 │         std::thread::sleep(wait_duration);
    │         ^^^^^^^^^^^^^^^^^^ `std::thread::sleep` is a blocking thread operation
    │
    = help: use `cx.background_executor().timer(duration).await`
    = context: async fn `spawn_server`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/remote_server/src/headless_project.rs:917:28
    │
917 │             let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    │                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::net::TcpListener::bind` is a blocking network operation
    │
    = help: use an async networking library
    = context: async fn `handle_spawn_kernel`


```

## `util` — 4 finding(s)

```
warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/util/src/archive.rs:46:13
   │
46 │             std::fs::create_dir_all(&path)
   │             ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
   │
   = help: use an async fs API or wrap in `smol::unblock`
   = context: async fn `extract_zip`

warning[blocking-in-async]: blocking call in async context
   ┌─ /home/mrg/zed/crates/util/src/archive.rs:52:13
   │
52 │             std::fs::create_dir_all(parent_dir)
   │             ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
   │
   = help: use an async fs API or wrap in `smol::unblock`
   = context: async fn `extract_zip`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/util/src/archive.rs:112:13
    │
112 │             std::fs::create_dir_all(&path)
    │             ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `extract_seekable_zip`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/util/src/archive.rs:118:13
    │
118 │             std::fs::create_dir_all(parent_dir)
    │             ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `extract_seekable_zip`


```

## `zed` — 3 finding(s)

```
warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/zed/src/zed/visual_tests.rs:145:13
    │
145 │             std::fs::create_dir_all(parent)?;
    │             ^^^^^^^^^^^^^^^^^^^^^^^ `std::fs::create_dir_all` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `capture_and_save_screenshot`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/zed/src/reliability.rs:256:13
    │
256 │             fs::remove_file(child_path).ok();
    │             ^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `upload_previous_minidumps`

warning[blocking-in-async]: blocking call in async context
    ┌─ /home/mrg/zed/crates/zed/src/reliability.rs:257:13
    │
257 │             fs::remove_file(json_path).ok();
    │             ^^^^^^^^^^^^^^^ `std::fs::remove_file` is a blocking filesystem operation
    │
    = help: use an async fs API or wrap in `smol::unblock`
    = context: async fn `upload_previous_minidumps`


```

