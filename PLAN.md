# Plan: Emit partial tool input events for all capable providers

## Background

`LanguageModelToolUse` has an `is_input_complete: bool` field. When `false`, the event
represents a partial/in-progress tool call that the UI can show progressively. When `true`,
the arguments are final.

`AnthropicEventMapper` is currently the only mapper that emits `is_input_complete: false`
events. It does this on every `InputJsonDelta` event by:

1. Appending the fragment to the accumulated `input_json` string.
2. Running `partial_json_fixer::fix_json` on the accumulated string to repair truncated JSON
   (e.g. closing unclosed brackets).
3. Attempting `serde_json::Value::from_str` on the repaired string.
4. If parsing succeeds, emitting a `ToolUse` event with `is_input_complete: false`.
5. On the block-end event, emitting a final `ToolUse` event with `is_input_complete: true`.

Several other providers receive incremental argument fragments on the wire but silently
accumulate them and only emit one `is_input_complete: true` event at the end. This plan
brings them in line with Anthropic's behaviour.

---

## Wire-protocol investigation

### OpenAI Chat Completions — `crates/open_ai/src/open_ai.rs`

The stream delivers `ResponseStreamEvent { choices: Vec<ChoiceDelta> }` per SSE line.
Each `ChoiceDelta.delta.tool_calls` is `Option<Vec<ToolCallChunk>>` where:

```zed2/crates/open_ai/src/open_ai.rs#L447-461
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
}
```

The first chunk for a tool call contains `id`, `type`, `name`, and an empty `arguments`.
Subsequent chunks contain only `arguments` fragments. The final chunk has
`finish_reason: "tool_calls"`.

**Confirmed: argument fragments are streamed incrementally. ✅**

---

### OpenAI Responses API — `crates/open_ai/src/responses.rs`

Has a dedicated event type for argument streaming:

```zed2/crates/open_ai/src/responses.rs#L153-168
#[serde(rename = "response.function_call_arguments.delta")]
FunctionCallArgumentsDelta {
    item_id: String,
    output_index: usize,
    delta: String,
    sequence_number: Option<u64>,
},
#[serde(rename = "response.function_call_arguments.done")]
FunctionCallArgumentsDone {
    item_id: String,
    output_index: usize,
    arguments: String,
    sequence_number: Option<u64>,
},
```

`id` and `name` are available from the preceding `OutputItemAdded` event
(`ResponseFunctionToolCall.call_id`, `.name`), so they are guaranteed present before
any `FunctionCallArgumentsDelta` fires.

**Confirmed: dedicated streaming event with guaranteed context. ✅**

---

### DeepSeek — `crates/deepseek/src/deepseek.rs`

`StreamDelta.tool_calls: Option<Vec<ToolCallChunk>>` with `FunctionChunk.arguments: Option<String>`.
Identical pattern to OpenAI Chat Completions.

**Confirmed: argument fragments are streamed incrementally. ✅**

---

### Mistral — `crates/mistral/src/mistral.rs`

`StreamDelta.tool_calls: Option<Vec<ToolCallChunk>>` with `FunctionChunk.arguments: Option<String>`.
Identical pattern to OpenAI Chat Completions.

**Confirmed: argument fragments are streamed incrementally. ✅**

---

### LM Studio — `crates/lmstudio/src/lmstudio.rs`

`ResponseMessageDelta.tool_calls: Option<Vec<ToolCallChunk>>` with
`FunctionChunk.arguments: Option<String>`. Identical pattern to OpenAI Chat Completions.

Known quirk: LM Studio only sends `name` in the first chunk; subsequent chunks have an
empty string in the `name` field. The existing mapper already guards against overwriting
`entry.name` with an empty string, so partial emission is safe once `name` is non-empty.

**Confirmed: argument fragments are streamed incrementally. ✅**

---

### OpenRouter — `crates/open_router/src/open_router.rs`

`ResponseMessageDelta.tool_calls: Option<Vec<ToolCallChunk>>` with
`FunctionChunk.arguments: Option<String>`. Identical pattern to OpenAI Chat Completions.

**Confirmed: argument fragments are streamed incrementally. ✅**

---

### Copilot Chat (chat completions path) — `crates/copilot_chat/src/copilot_chat.rs`

`ResponseDelta.tool_calls: Vec<ToolCallChunk>` with `FunctionChunk.arguments: Option<String>`.
Identical pattern to OpenAI Chat Completions.

**Confirmed: argument fragments are streamed incrementally. ✅**

---

### Bedrock — uses AWS SDK `ConverseStreamOutput` from `aws_sdk_bedrockruntime`

The SDK delivers `ConverseStreamOutput::ContentBlockDelta` where the delta variant for
tool calls is `ContentBlockDelta::ToolUse(ToolUseBlockDelta)`. Calling `.input()` on
`ToolUseBlockDelta` returns the partial JSON string for that chunk. The `id` and `name`
are available from the preceding `ContentBlockStart` event, so they are always present
before any argument delta arrives.

**Confirmed: argument fragments are streamed incrementally via the AWS SDK. ✅**

---

### Cannot support (protocol sends complete objects, no incremental deltas)

- **Google Gemini** — each SSE chunk contains a complete `FunctionCallPart` with fully
  constructed `args: serde_json::Value`. There is no partial-argument event type.
- **Ollama** — each `ChatResponseDelta.message` contains a fully-formed
  `ChatMessage::Assistant { tool_calls: Option<Vec<OllamaToolCall>> }`. Tool calls arrive
  complete.
- **Copilot Chat (Responses path)** — `copilot_responses::StreamEvent` has no
  `FunctionCallArgumentsDelta` variant. The endpoint only emits `OutputItemDone` with
  completed arguments.

---

## Changes per mapper

### 1. `OpenAiEventMapper` — `crates/language_models/src/provider/open_ai.rs`

**Used by:** OpenAI (Chat Completions path), xAI, Vercel, Vercel AI Gateway,
OpenAI Compatible (chat path), Cloud/xAI backend.

**Current behaviour:** Appends to `RawToolCall.arguments`; emits a single
`is_input_complete: true` event only when `finish_reason == "tool_calls"`.

**Change:** After each `push_str`, if the entry already has a non-empty `id` and `name`,
apply `partial_json_fixer::fix_json` + `serde_json::Value::from_str`. On success, push an
`is_input_complete: false` `ToolUse` event into the returned `events` vec for that chunk.
The final `is_input_complete: true` emission on `finish_reason == "tool_calls"` stays
unchanged.

The `id`/`name` guard is necessary because the first chunk carries `id` and `name` with
an empty `arguments` string; the guard prevents emitting a spurious `{}` event on that
first empty-arguments chunk.

---

### 2. `OpenAiResponseEventMapper` — `crates/language_models/src/provider/open_ai.rs`

**Used by:** OpenAI (Responses path), OpenAI Compatible (responses path),
Cloud/OpenAI backend.

**Current behaviour:** Appends `delta` to `PendingResponseFunctionCall.arguments`;
returns `Vec::new()` (nothing emitted).

**Change:** After `push_str`, look up `call_id` and `name` from the entry (always
available since `OutputItemAdded` fires first). Apply `partial_json_fixer::fix_json` +
`serde_json::Value::from_str`. On success, return a vec with a single
`is_input_complete: false` `ToolUse` event. The `FunctionCallArgumentsDone` branch stays
unchanged.

No `id`/`name` guard needed here — `OutputItemAdded` guarantees both are present before
any delta arrives.

---

### 3. `DeepSeekEventMapper` — `crates/language_models/src/provider/deepseek.rs`

**Change:** Identical to `OpenAiEventMapper`. Emit a partial event after each
`push_str` when `id` and `name` are non-empty and `fix_json` + parse succeeds.

---

### 4. `MistralEventMapper` — `crates/language_models/src/provider/mistral.rs`

**Change:** Same as `OpenAiEventMapper`. Add partial emission inside the
`choice.delta.tool_calls` loop, before the `finish_reason` branch.

---

### 5. `LmStudioEventMapper` — `crates/language_models/src/provider/lmstudio.rs`

**Change:** Same as `OpenAiEventMapper`. The existing empty-name guard already
ensures `entry.name` is correct before `arguments` fragments accumulate, so the
standard `id`/`name` non-empty check applies cleanly.

---

### 6. `OpenRouterEventMapper` — `crates/language_models/src/provider/open_router.rs`

**Change:** Same as `OpenAiEventMapper`.

---

### 7. Copilot Chat `map_to_language_model_completion_events` (chat path) — `crates/language_models/src/provider/copilot_chat.rs`

**Change:** Same as `OpenAiEventMapper`. Inside the `for tool_call in delta.tool_calls`
loop, after accumulating `arguments`, attempt partial emission if `id` and `name` are
non-empty.

---

### 8. Bedrock `map_to_language_model_completion_events` — `crates/language_models/src/provider/bedrock.rs`

**Current behaviour:** Appends `fragment.input()` to `RawToolUse.input_json`;
emits nothing until `ContentBlockStop`.

**Change:** After `push_str`, look up the `RawToolUse` for this block index (`id` and
`name` are guaranteed present from the preceding `ContentBlockStart`). Apply
`partial_json_fixer::fix_json` + `serde_json::Value::from_str`. On success, yield an
`is_input_complete: false` `ToolUse` event.

Because this mapper uses `stream::unfold` and currently returns
`Option<(Option<LanguageModelCompletionEvent>, State)>` — yielding at most one event per
AWS SDK event — the return type of the inner `result` binding must be changed from
`Option<Event>` to `Vec<Event>` so that the `ContentBlockDelta` arm can yield a partial
event without consuming the state machine (the final `is_input_complete: true` is still
emitted from `ContentBlockStop`).

---

## `supports_streaming_tools()` updates

`supports_streaming_tools()` currently gates the inline assistant's tool-based code
generation path (see `CodegenAlternative::use_streaming_tools`). After the above changes,
the following providers genuinely support streaming tool inputs and can return `true`:

| Provider                        | Notes                                           |
| ------------------------------- | ----------------------------------------------- |
| `OpenAiLanguageModel`           | Both Chat and Responses mappers will be updated |
| `XAiLanguageModel`              | Shares `OpenAiEventMapper`                      |
| `VercelLanguageModel`           | Shares `OpenAiEventMapper`                      |
| `VercelAiGatewayLanguageModel`  | Shares `OpenAiEventMapper`                      |
| `OpenAiCompatibleLanguageModel` | Both paths updated                              |
| `DeepSeekLanguageModel`         | Own mapper, updated                             |
| `MistralLanguageModel`          | Own mapper, updated                             |
| `LmStudioLanguageModel`         | Own mapper, updated                             |
| `OpenRouterLanguageModel`       | Own mapper, updated                             |
| `CopilotChatLanguageModel`      | Chat path updated; Responses path cannot        |
| `BedrockModel`                  | Own mapper, updated                             |

`CloudLanguageModel` already delegates to `self.model.supports_streaming_tools` fetched
from the server, so no code change needed there — the flag just needs to be set server-side
for models whose upstream provider is now updated (OpenAI, xAI).

`GoogleLanguageModel` and `OllamaLanguageModel` remain `false` (protocol limitation).

---

## Suggested implementation order

1. `OpenAiResponseEventMapper` — cleanest case: dedicated `FunctionCallArgumentsDelta`
   event, `id`/`name` guaranteed by `OutputItemAdded`, no guard needed.
2. `OpenAiEventMapper` — next; covers xAI, Vercel, Vercel AI Gateway, and OpenAI
   Compatible chat path for free.
3. `DeepSeekEventMapper`, `MistralEventMapper`, `LmStudioEventMapper`,
   `OpenRouterEventMapper` — mechanical copies of the same pattern.
4. Copilot Chat chat-path mapper — same pattern, different file.
5. Bedrock — requires changing the `unfold` inner result from `Option<Event>` to
   `Vec<Event>`.
6. `supports_streaming_tools()` — flip the flag for all updated providers once the
   mappers are confirmed correct.

---

## Testing

Each mapper already has unit tests that feed mock event sequences and assert on the
resulting `LanguageModelCompletionEvent` vec. For each changed mapper:

- Add a test that feeds multiple `arguments` delta chunks (none of which is valid JSON
  alone) followed by the finish event, and asserts:
  1. Partial `ToolUse` events with `is_input_complete: false` are emitted for chunks
     where the accumulated JSON becomes parseable after fixing.
  2. The final event has `is_input_complete: true` with the correct complete `input`.
- Add a test where the first delta chunk is already valid JSON (e.g. `{}`) to confirm
  a partial event is emitted immediately.
- Existing tests that assert a single final `ToolUse` event must be updated to account
  for the leading partial events.

The Bedrock mapper is tested via `map_to_language_model_completion_events`; the same
patterns apply.
