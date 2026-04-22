# Implementation plan: multi-content tool results

## Goal

Change the `content` field on `LanguageModelToolResult` from a single
`LanguageModelToolResultContent` into a `Vec<LanguageModelToolResultContent>`,
so that a tool call can carry multiple pieces of content (e.g. a text summary
**and** an image).

The motivating case is MCP: `CallToolResponse.content` is already a `Vec`,
and our bridge currently collapses it to the first text chunk and drops
everything else.

```zed/crates/agent/src/tools/context_server_registry.rs#L392-L408
let mut result = String::new();
for content in response.content {
    match content {
        context_server::types::ToolResponseContent::Text { text } => {
            result.push_str(&text);
        }
        context_server::types::ToolResponseContent::Image { .. } => {
            log::warn!("Ignoring image content from tool response");
        }
        context_server::types::ToolResponseContent::Audio { .. } => {
            log::warn!("Ignoring audio content from tool response");
        }
        context_server::types::ToolResponseContent::Resource { .. } => {
            log::warn!("Ignoring resource content from tool response");
        }
    }
}
```

After this change, MCP tools that return text + image deliver both to the
model (subject to that provider's vision support). Every other part of the
system keeps doing what it does today.

## Current shape

```zed/crates/language_model_core/src/request.rs#L100-L116
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct LanguageModelToolResult {
    pub tool_use_id: LanguageModelToolUseId,
    pub tool_name: Arc<str>,
    pub is_error: bool,
    /// The tool output formatted for presenting to the model
    pub content: LanguageModelToolResultContent,
    /// The raw tool output, if available, often for debugging or extra state for replay
    pub output: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Hash)]
pub enum LanguageModelToolResultContent {
    Text(Arc<str>),
    Image(LanguageModelImage),
}
```

`LanguageModelToolResultContent` has a hand-rolled `Deserialize` that accepts
several wire shapes (plain string, `{"type":"text","text":...}`,
`{"text":...}`, `{"image":...}`, direct image). The new code must preserve
all of those, plus accept the new array shape on the `content` field.

## Decisions

These were agreed on up front. The rest of the plan flows from them.

- **Audio and Resource MCP variants stay dropped.** No new variants are added
  to `LanguageModelToolResultContent`. Audio and Resource parts from MCP
  keep hitting the existing `log::warn!` and getting dropped. The Vec change
  is strictly about text + image.
- **`AgentTool::Output` stays single-valued.** The existing bound
  `type Output: … + Into<LanguageModelToolResultContent>` is kept as-is.
  Built-in tools (`edit_file_tool`, `find_path_tool`, `read_file_tool`,
  `spawn_agent_tool`, `streaming_edit_file_tool`, `web_search_tool`) do not
  change. The `Thread` layer wraps each tool output as `vec![output.into()]`
  when constructing the `LanguageModelToolResult`. Only the MCP bridge
  produces multi-part results in this first iteration.
- **On-disk format: permissive deserializer, no version bump.**
  `DbThread::VERSION` stays at `"0.3.0"`. The `content` field gets a
  `#[serde(with = …)]` helper (or equivalent hand-rolled `Deserialize`) that
  accepts both the old single-value shape and the new array shape and
  normalizes to `Vec`. No explicit upgrade step, no migration pass. This
  matches the pattern already used by
  `LanguageModelToolResultContent::Deserialize`.
- **Keep existing per-provider behavior for non-text parts.** No new
  placeholder strategy, no new `LanguageModel::can_send_image_in_tool_result`
  knob. Each provider does structurally what it does today — we just teach
  every call site to iterate the `Vec`.

## Step 1 — type definition and helpers

File: `crates/language_model_core/src/request.rs`.

- Change the field:

  ```
  pub struct LanguageModelToolResult {
      …
      pub content: Vec<LanguageModelToolResultContent>,
      …
  }
  ```

- Add a permissive deserializer for the `content` field (e.g. a `one_or_many`
  module referenced by `#[serde(with = "one_or_many")]`) that accepts either:
  - a single value in any of the shapes `LanguageModelToolResultContent`
    already accepts, **or**
  - a JSON array of those values,

  and normalizes both to `Vec<LanguageModelToolResultContent>`.

- Add ergonomic conversions so existing construction sites keep compiling:

  ```
  impl From<&str> for Vec<LanguageModelToolResultContent> { … }
  impl From<String> for Vec<LanguageModelToolResultContent> { … }
  impl From<LanguageModelImage> for Vec<LanguageModelToolResultContent> { … }
  impl From<LanguageModelToolResultContent> for Vec<LanguageModelToolResultContent> { … }
  ```

- Add `Vec`-level helpers on `LanguageModelToolResult`:

  ```
  impl LanguageModelToolResult {
      /// Concatenates all `Text` parts; ignores non-text parts.
      pub fn text_contents(&self) -> String { … }

      /// True when the `Vec` is empty or every part is empty.
      pub fn is_content_empty(&self) -> bool { … }
  }
  ```

- Keep `LanguageModelToolResultContent::{to_str, is_empty}` as-is (they still
  make sense on a single element).

- Extend the existing `test_language_model_tool_result_content_deserialization`
  test with cases that feed both the old and new shapes through
  `serde_json::from_value::<LanguageModelToolResult>`, asserting the
  post-deserialize form is always a `Vec`.

## Step 2 — producer updates

Every call site that builds a `LanguageModelToolResult` needs
`content: vec![…]` instead of `content: …`. The `From<…> for Vec<…>` impls
added in step 1 mean `content: "foo".into()` continues to compile.

| File                                                  | Call site                                                 | Change                               |
| ----------------------------------------------------- | --------------------------------------------------------- | ------------------------------------ |
| `crates/agent/src/thread.rs`                          | `Thread::handle_tool_use_event` (unknown tool error)      | wrap in `vec![…]`                    |
| `crates/agent/src/thread.rs`                          | `Thread::handle_tool_use_json_parse_error_event`          | wrap                                 |
| `crates/agent/src/thread.rs`                          | `Thread::run_tool` (lifts `AgentToolOutput::llm_output`)  | wrap                                 |
| `crates/agent/src/thread.rs`                          | `Thread::flush_pending_message` (`TOOL_CANCELED_MESSAGE`) | wrap                                 |
| `crates/agent/src/thread.rs`                          | `AgentMessage::to_request` empty-content guard            | see note below                       |
| `crates/agent/src/db.rs`                              | `DbThread::upgrade_from_agent_1`                          | `content: vec![tool_result.content]` |
| `crates/agent/src/tools/context_server_registry.rs`   | `ContextServerTool::run`                                  | see step 5                           |
| `crates/agent/src/edit_agent/evals.rs`                | `tool_result` helper                                      | wrap                                 |
| `crates/agent/src/tools/evals/streaming_edit_file.rs` | `tool_result` helper                                      | wrap                                 |
| `crates/agent/src/tests/**`                           | ~10 constructor sites                                     | `"foo".into()` keeps working         |
| `crates/open_ai/src/completion.rs`                    | `tests::into_open_ai_response_builds_complete_payload`    | wrap                                 |

`AgentMessage::to_request` has this workaround today:

```zed/crates/agent/src/thread.rs#L585-L591
if tool_result.content.is_empty() {
    tool_result.content = "<Tool returned an empty string>".into();
}
```

Becomes: if `tool_result.is_content_empty()`, replace with
`vec!["<Tool returned an empty string>".into()]`. Keep the comment — the
underlying API still rejects empty tool results.

Note that `legacy_thread::SerializedToolResult` (`content:
LanguageModelToolResultContent`) is **not** updated. It's the pre-`agent_1`
wire format; only the upgrade site in `db.rs` needs to wrap.

## Step 3 — consumer updates (providers + UI)

Every site that reads `tool_result.content` currently matches the enum
directly. They become a loop over the `Vec`. Two buckets:

**Natively multi-part providers.** These already emit `Vec`-shaped
provider-side types; the diff is just "build up the `Vec` in a loop instead
of in a single `match`".

| File                                                                             | Provider-side type                                            |
| -------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| `crates/anthropic/src/completion.rs` (`to_anthropic_content`)                    | `ToolResultContent::Plain` / `Multipart(Vec<ToolResultPart>)` |
| `crates/anthropic/src/completion.rs` (`count_anthropic_tokens_with_tiktoken`)    | token summation                                               |
| `crates/open_ai/src/completion.rs` (`append_message_to_response_items`)          | `ResponseFunctionCallOutputContent::{Text, List}`             |
| `crates/language_models/src/provider/bedrock.rs` (`into_bedrock`)                | `BedrockToolResultBlock.content()` (already a list)           |
| `crates/language_models/src/provider/bedrock.rs` (`get_bedrock_tokens`)          | token summation                                               |
| `crates/language_models/src/provider/copilot_chat.rs` (`into_copilot_chat`)      | `ChatMessageContent::Multipart(Vec<ChatMessagePart>)`         |
| `crates/language_models/src/provider/copilot_chat.rs` (`into_copilot_responses`) | `ResponseFunctionOutput::Content(Vec<…>)`                     |
| `crates/language_models/src/provider/lmstudio.rs` (`to_lmstudio_request`)        | `Vec<MessagePart>`                                            |
| `crates/language_models/src/provider/open_router.rs` (`into_open_router`)        | `Vec<MessagePart>`                                            |

For Anthropic specifically: when the post-refactor `Vec` has exactly one
`Text` part, keep emitting `ToolResultContent::Plain(String)` so the on-wire
bytes are identical to today for all built-in tools. Fall through to
`Multipart` for `≥2` parts or any non-text part.

**Text-only tool-message providers.** These can only carry a single string
in a tool message. Text parts are concatenated; non-text parts keep each
provider's current behavior.

| File                                                                         | Current non-text behavior                                                   | Post-refactor                                                                                                             |
| ---------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `crates/open_ai/src/completion.rs` (`into_open_ai`, legacy Chat Completions) | "cheats" with `MessagePart` multipart                                       | iterate, same cheat per element                                                                                           |
| `crates/google_ai/src/completion.rs` (`into_google::map_content`)            | splits into `FunctionResponsePart` text + extra `InlineDataPart` for images | same split, but concatenate all text parts into the single `{"output": …}` string and emit one `InlineDataPart` per image |
| `crates/language_models/src/provider/deepseek.rs` (`into_deepseek`)          | silently drops images                                                       | silently drops all non-text parts                                                                                         |
| `crates/language_models/src/provider/mistral.rs` (`into_mistral`)            | emits `"[Tool responded with an image, but Zed doesn't support these yet]"` | emit the same placeholder per non-text part                                                                               |
| `crates/language_models/src/provider/ollama.rs` (`to_ollama_request`)        | `tool_result.content.to_str().unwrap_or("")`                                | use `tool_result.text_contents()` (joins all text parts)                                                                  |

Also consumer-side but not a provider:

- `crates/agent/src/thread.rs::AgentMessage::to_markdown` — iterate and
  render each part (each `Text` as a line; each `Image` as `<image />`).

## Step 4 — test-site pattern matches

Around seven test assertions pattern-match on a single-value
`tool_result.content`:

```
match &tool_result.content {
    language_model::LanguageModelToolResultContent::Text(text) => text.to_string(),
    _ => panic!("expected text content in tool result"),
};
```

Sites:

- `crates/agent/src/tests/edit_file_thread_test.rs::test_streaming_edit_json_parse_error_does_not_cause_unsaved_changes`
- `crates/agent/src/tests/mod.rs::test_terminal_tool_cancellation_captures_output`
- `crates/agent/src/tests/mod.rs::test_terminal_tool_stopped_via_terminal_card_button`
- `crates/agent/src/tests/mod.rs::test_terminal_tool_timeout_expires`
- `crates/agent/src/tests/mod.rs::test_streaming_tool_json_parse_error_is_forwarded_to_running_tool`
- `crates/agent/src/thread.rs::tests::test_handle_tool_use_json_parse_error_adds_tool_use_to_content`
- `crates/remote_server/src/remote_editing_tests.rs::test_remote_agent_fs_tool_calls`
- `crates/zed/src/visual_test_runner.rs::run_agent_thread_view_test`

Preferred form: replace with `tool_result.text_contents()` when the test
only cares about the textual content. Where a test needs to assert a specific
element count/shape, use a slice pattern:

```
match tool_result.content.as_slice() {
    [language_model::LanguageModelToolResultContent::Text(text)] => text.to_string(),
    _ => panic!("expected a single text part in tool result"),
}
```

Note that `ReadFileTool`'s `type Output = LanguageModelToolResultContent`
does **not** change (`AgentTool::Output` stays single-valued per Decision 2),
so test helpers like `error_text(content: LanguageModelToolResultContent)` in
`read_file_tool.rs` keep working unchanged.

## Step 5 — flip the MCP bridge

File: `crates/agent/src/tools/context_server_registry.rs::ContextServerTool::run`.

This is the only intentional behavior change.

- Build a `Vec<LanguageModelToolResultContent>` from `response.content`.
- Map `ToolResponseContent::Text { text }` → `LanguageModelToolResultContent::Text(text.into())`.
- Map `ToolResponseContent::Image { data, mime_type }` → `LanguageModelToolResultContent::Image(…)`.
  - Only `image/png` is natively representable in `LanguageModelImage` today;
    for other mime types, fall back to the existing drop-with-warning
    behavior for this iteration.
- Keep the existing `log::warn!` for `Audio` and `Resource` per Decision 1.
- `AgentToolOutput.llm_output` stays single-valued (per Decision 2), but the
  MCP bridge is special: it doesn't go through the `AgentTool::Output` →
  `vec![output.into()]` wrapper. It constructs the `LanguageModelToolResult`
  directly, which means it's the one place that naturally emits a multi-part
  `Vec`.

  Concretely: the existing `AgentToolOutput { llm_output, raw_output }`
  return path is for the "error / single-string summary" case. The
  success-with-multiple-parts path needs to reach into `Thread::run_tool`'s
  `LanguageModelToolResult` construction and pass through a `Vec`. Cleanest
  option is to widen `AgentToolOutput.llm_output` to
  `Vec<LanguageModelToolResultContent>` **only** inside the MCP bridge's
  usage — the built-in tools never hit that path because they return
  `Self::Output` which still lifts through `into()`.

## Step 6 — verification

- `./script/clippy` clean across the touched crates.
- All existing tests pass without behavior changes for built-in tools.
- New tests in `request.rs`:
  - Deserializing `{"content": "foo", …}` (old shape) yields
    `content: vec![Text("foo")]`.
  - Deserializing `{"content": [{"type": "text", "text": "foo"}, {"source": "…"}], …}`
    (new shape) yields `content: vec![Text(…), Image(…)]`.
  - Round-tripping a thread through serialize/deserialize preserves
    multi-part content.
- New test in `tests/mod.rs::test_mcp_tools` (or a sibling): MCP tool
  returning `[Text, Image]` shows up as a `LanguageModelToolResult` with two
  elements and an Anthropic request round-trips it as `Multipart`.

## Out of scope for this change

Captured here so they don't creep in:

- Adding `Audio` / `Resource` variants to `LanguageModelToolResultContent`.
- Broadening `AgentTool::Output` to allow built-in tools to emit multi-part.
  (Do this later if a concrete use case appears.)
- Any `LanguageModel` capability flag for "this provider supports image
  content in tool results".
- Changing `acp_thread::ToolCallContent` / `ContentBlock`. Those are already
  `Vec`-shaped for UI rendering and are distinct from the model-facing
  `LanguageModelToolResultContent`.
- Bumping `DbThread::VERSION` or writing an explicit on-disk migration.
