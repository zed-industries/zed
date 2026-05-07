# Dylint hits by lint and crate

Generated from:

```sh
RUSTFLAGS="-A entity_update_in_render" cargo dylint --path tooling/lints -- --workspace
```

Total warnings across the workspace: **640** (excluding the suppressed `entity_update_in_render` lint). Crates with zero hits are omitted.

## Summary

| Lint | Total hits | Crates affected |
| --- | ---: | ---: |
| `shared_string_from_str_literal` | 459 | 50 |
| `async_block_without_await` | 180 | 63 |
| `notify_in_render` | 1 | 1 |

## `shared_string_from_str_literal` (459 hits, 50 crates)

| Crate | Hits |
| --- | ---: |
| `agent_ui` | 67 |
| `ui` | 64 |
| `extensions_ui` | 37 |
| `language_models` | 28 |
| `agent` | 27 |
| `settings_ui` | 26 |
| `git_ui` | 22 |
| `repl` | 17 |
| `zed` | 16 |
| `editor` | 11 |
| `collab_ui` | 10 |
| `debugger_ui` | 8 |
| `recent_projects` | 8 |
| `auto_update_ui` | 7 |
| `git_graph` | 7 |
| `languages` | 7 |
| `onboarding` | 7 |
| `project` | 7 |
| `workspace` | 7 |
| `rules_library` | 6 |
| `theme` | 6 |
| `agent_servers` | 5 |
| `language_tools` | 5 |
| `copilot_ui` | 4 |
| `gpui` | 4 |
| `acp_tools` | 3 |
| `csv_preview` | 3 |
| `diagnostics` | 3 |
| `edit_prediction_ui` | 3 |
| `keymap_editor` | 3 |
| `project_panel` | 3 |
| `search` | 3 |
| `sidebar` | 3 |
| `title_bar` | 3 |
| `component_preview` | 2 |
| `git` | 2 |
| `open_path_prompt` | 2 |
| `acp_thread` | 1 |
| `copilot` | 1 |
| `debugger_tools` | 1 |
| `language_model` | 1 |
| `language_model_core` | 1 |
| `markdown_preview` | 1 |
| `miniprofiler_ui` | 1 |
| `picker` | 1 |
| `prettier` | 1 |
| `remote_connection` | 1 |
| `svg_preview` | 1 |
| `tab_switcher` | 1 |
| `terminal_view` | 1 |

## `async_block_without_await` (180 hits, 63 crates)

| Crate | Hits |
| --- | ---: |
| `language` | 18 |
| `project` | 18 |
| `fs` | 13 |
| `editor` | 12 |
| `agent_ui` | 11 |
| `agent` | 10 |
| `livekit_client` | 7 |
| `prompt_store` | 5 |
| `copilot_chat` | 4 |
| `http_client` | 4 |
| `rpc` | 4 |
| `action_log` | 3 |
| `copilot` | 3 |
| `edit_prediction` | 3 |
| `extension_host` | 3 |
| `repl` | 3 |
| `terminal` | 3 |
| `client` | 2 |
| `edit_prediction_context` | 2 |
| `fuzzy` | 2 |
| `fuzzy_nucleo` | 2 |
| `git_ui` | 2 |
| `google_ai` | 2 |
| `open_ai` | 2 |
| `outline_panel` | 2 |
| `remote` | 2 |
| `vim` | 2 |
| `acp_thread` | 1 |
| `anthropic` | 1 |
| `audio` | 1 |
| `auto_update` | 1 |
| `buffer_diff` | 1 |
| `collab` | 1 |
| `context_server` | 1 |
| `csv_preview` | 1 |
| `dap` | 1 |
| `debugger_ui` | 1 |
| `deepseek` | 1 |
| `diagnostics` | 1 |
| `git` | 1 |
| `gpui` | 1 |
| `gpui_macos` | 1 |
| `journal` | 1 |
| `keymap_editor` | 1 |
| `language_model` | 1 |
| `language_models` | 1 |
| `lmstudio` | 1 |
| `lsp` | 1 |
| `markdown` | 1 |
| `mistral` | 1 |
| `multi_buffer` | 1 |
| `open_router` | 1 |
| `opencode` | 1 |
| `project_panel` | 1 |
| `remote_server` | 1 |
| `search` | 1 |
| `settings_ui` | 1 |
| `svg_preview` | 1 |
| `system_specs` | 1 |
| `terminal_view` | 1 |
| `theme` | 1 |
| `worktree` | 1 |
| `zed` | 1 |

## `notify_in_render` (1 hits, 1 crates)

| Crate | Hits |
| --- | ---: |
| `debugger_ui` | 1 |
