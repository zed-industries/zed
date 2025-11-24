# Gemini 3 Reasoning Support for Copilot Chat Completions API

## Problem Statement

Gemini 3 models (like `gemini-3-pro-preview`) fail when using tool calls through Copilot with the error:

```
Unable to submit request because function call `default_api:list_directory` in the 2. content block is missing a `thought_signature`.
```

The error occurs AFTER the first tool call is executed and we send back the tool results with conversation history. The model requires that we preserve and send back its "reasoning" data.

## Background

### What is `reasoning_opaque`?

When Gemini 3 models perform reasoning before making a tool call, they generate reasoning data that includes:
- `reasoning_text` - Human-readable reasoning content (optional)
- `reasoning_opaque` - An encrypted/opaque token that must be preserved and sent back

This is similar to how Anthropic models have "thinking" blocks with signatures that must be preserved.

### API Flow

1. **User sends prompt** → Model receives request with tools
2. **Model responds with tool call** → Response includes `reasoning_opaque` in the delta
3. **We execute the tool** → Get the result
4. **We send back conversation history** → **MUST include the `reasoning_opaque`** from step 2
5. **Model continues** → Uses the preserved reasoning context

## What Copilot Sends Us (Response Structure)

From actual Copilot streaming responses with Gemini 3:

```json
{
  "choices": [{
    "index": 0,
    "finish_reason": "tool_calls",
    "delta": {
      "content": null,
      "role": "assistant",
      "tool_calls": [{
        "index": 0,
        "id": "call_MHxRUnpJbnN2SHV2bFNJZnc3bng",
        "function": {
          "name": "list_directory",
          "arguments": "{\"path\":\"deleteme\"}"
        }
      }],
      "reasoning_opaque": "XLn4be0oRXKamQWgyEcgBYpDximdbf/J/dcDmWIhGjZMFaQvOOmSXTqY/zfnRtDCFmZfvsn4W1AG..."
    }
  }]
}
```

Key observations:
- `reasoning_opaque` is at the **delta/message level**, not inside individual tool calls
- The tool calls themselves do NOT have a `thought_signature` field
- There may also be `reasoning_text` with human-readable reasoning content

### Important: Message Merging Requirement

Looking at the CodeCompanion implementation (PR #2419), there's a critical insight:

When the model sends reasoning data and then tool calls, they may come as **separate messages** that need to be **merged** into a single message when sending back:

```lua
-- Check if next message is also from LLM and has tool_calls but no content
-- This indicates tool calls that should be merged with the previous message
if i < #result.messages
  and result.messages[i + 1].role == current.role
  and result.messages[i + 1].tool_calls
  and not result.messages[i + 1].content
then
  -- Merge tool_calls from next message into current
  current.tool_calls = result.messages[i + 1].tool_calls
  i = i + 1 -- Skip the next message since we merged it
end
```

## What We Must Send Back (Request Structure)

Based on the CodeCompanion implementation, when sending back the conversation history, the assistant message with tool calls should look like:

```json
{
  "role": "assistant",
  "content": "LLM's response here",
  "reasoning_text": "Some reasoning here",
  "reasoning_opaque": "XLn4be0oRXKamQWgyEcgBYpDximdbf...",
  "tool_calls": [{
    "id": "call_MHxRUnpJbnN2SHV2bFNJZnc3bng",
    "type": "function",
    "function": {
      "name": "list_directory",
      "arguments": "{\"path\":\"deleteme\"}"
    }
  }]
}
```

Key points:
- `reasoning_opaque` goes at the **message level** (same level as `role`, `content`, `tool_calls`)
- `reasoning_text` may also be included at the message level
- `content` can be `null` if there's no text content
- The `function` object does NOT contain `thought_signature`

## Implementation Plan

### Step 1: Update Response Structures

In `crates/copilot/src/copilot_chat.rs`, add fields to capture reasoning data:

**Update `ResponseDelta`:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDelta {
    pub content: Option<String>,
    pub role: Option<Role>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCallChunk>,
    // Add these fields:
    pub reasoning_opaque: Option<String>,
    pub reasoning_text: Option<String>,
}
```

### Step 2: Update Request Structures

**Update `ChatMessage::Assistant`:**
```rust
pub enum ChatMessage {
    Assistant {
        content: Option<ChatMessageContent>,  // Changed to Option for null support
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
        // Add these fields:
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_opaque: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_text: Option<String>,
    },
    // ... other variants
}
```

**Important:** The `content` field should be `Option<ChatMessageContent>` so it can serialize to `null` instead of `[]` (empty array) when there's no text content.

### Step 3: Update Internal Event/Message Structures

We need to propagate reasoning data through our internal structures.

**In `crates/language_model/src/language_model.rs`**, `LanguageModelToolUse` already has:
```rust
pub struct LanguageModelToolUse {
    pub id: LanguageModelToolUseId,
    pub name: Arc<str>,
    pub raw_input: String,
    pub input: serde_json::Value,
    pub is_input_complete: bool,
    pub thought_signature: Option<String>,  // We can repurpose this
}
```

However, since reasoning is **message-level** not **tool-level**, we may need a different approach. Consider:

1. Store `reasoning_opaque` and `reasoning_text` in `LanguageModelRequestMessage.reasoning_details` (which already exists as `Option<serde_json::Value>`)
2. Or create a new dedicated field

**In `crates/language_model/src/request.rs`:**
```rust
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
    pub cache: bool,
    // Use this existing field, or add new specific fields:
    pub reasoning_details: Option<serde_json::Value>,
}
```

### Step 4: Capture Reasoning from Responses

In `crates/language_models/src/provider/copilot_chat.rs`, in the `map_to_language_model_completion_events` function:

1. Capture `reasoning_opaque` and `reasoning_text` from the delta
2. Store them so they can be associated with tool calls
3. When emitting `LanguageModelCompletionEvent::ToolUse`, include the reasoning data

```rust
// Pseudocode for the mapper:
struct State {
    events: Pin<Box<dyn Send + Stream<Item = Result<ResponseEvent>>>>,
    tool_calls_by_index: HashMap<usize, RawToolCall>,
    reasoning_opaque: Option<String>,  // Add this
    reasoning_text: Option<String>,    // Add this
}

// When processing delta:
if let Some(opaque) = delta.reasoning_opaque {
    state.reasoning_opaque = Some(opaque);
}
if let Some(text) = delta.reasoning_text {
    state.reasoning_text = Some(text);
}

// When emitting tool use events, attach the reasoning
```

### Step 5: Send Reasoning Back in Requests

In `crates/language_models/src/provider/copilot_chat.rs`, in the `into_copilot_chat` function:

When building `ChatMessage::Assistant` for messages that have tool calls:

```rust
messages.push(ChatMessage::Assistant {
    content: if text_content.is_empty() {
        None  // Serializes to null, not []
    } else {
        Some(text_content.into())
    },
    tool_calls,
    reasoning_opaque: /* get from message's reasoning_details or tool_use */,
    reasoning_text: /* get from message's reasoning_details or tool_use */,
});
```

### Step 6: Handle Message Merging (If Needed)

If Copilot sends reasoning and tool calls as separate streaming events that result in separate internal messages, we may need to merge them when constructing the request.

Look at the message construction logic and ensure that:
- If an assistant message has reasoning but no tool calls, AND
- The next message is also assistant with tool calls but no content
- Then merge them into a single message

## Files to Modify

1. **`crates/copilot/src/copilot_chat.rs`**
   - Add `reasoning_opaque` and `reasoning_text` to `ResponseDelta`
   - Add `reasoning_opaque` and `reasoning_text` to `ChatMessage::Assistant`
   - Change `content` in `ChatMessage::Assistant` to `Option<ChatMessageContent>`
   - Update any pattern matches that break due to the Option change

2. **`crates/language_models/src/provider/copilot_chat.rs`**
   - Update `map_to_language_model_completion_events` to capture reasoning
   - Update `into_copilot_chat` to include reasoning in requests
   - Possibly add message merging logic

3. **`crates/language_model/src/request.rs`** (maybe)
   - Decide how to store reasoning data in `LanguageModelRequestMessage`
   - Could use existing `reasoning_details` field or add new fields

4. **`crates/language_model/src/language_model.rs`** (maybe)
   - May need to add a new event type for reasoning, OR
   - Ensure reasoning can be attached to tool use events

## Testing

1. Test with Gemini 3 Pro Preview through Copilot
2. Trigger a tool call (e.g., ask "what files are in this directory?")
3. Verify the first request succeeds and returns with `reasoning_opaque`
4. Verify the second request (with tool results) includes the `reasoning_opaque`
5. Verify the model successfully continues and doesn't return a 400 error

## Debug Logging Recommendations

Add `eprintln!` statements to trace:
1. When `reasoning_opaque` is received from Copilot
2. When `reasoning_opaque` is stored/attached to tool use
3. The full JSON of requests being sent (to verify structure)
4. The full JSON of responses received

## References

- [CodeCompanion PR #2419](https://github.com/olimorris/codecompanion.nvim/pull/2419) - Working implementation in Lua
- [Original Zed Issue #43024](https://github.com/zed-industries/zed/issues/43024)
- [Google Thought Signatures Documentation](https://ai.google.dev/gemini-api/docs/thinking#signatures)

## Key Insight from CodeCompanion

The CodeCompanion implementation shows the exact structure:

**Receiving:**
```lua
-- In parse_message_meta function:
if extra.reasoning_text then
  data.output.reasoning = data.output.reasoning or {}
  data.output.reasoning.content = extra.reasoning_text
end
if extra.reasoning_opaque then
  data.output.reasoning = data.output.reasoning or {}
  data.output.reasoning.opaque = extra.reasoning_opaque
end
```

**Sending back:**
```lua
-- In form_messages function:
if current.reasoning then
  if current.reasoning.content then
    current.reasoning_text = current.reasoning.content
  end
  if current.reasoning.opaque then
    current.reasoning_opaque = current.reasoning.opaque
  end
  current.reasoning = nil
end
```

The key is that `reasoning_text` and `reasoning_opaque` are **top-level fields** on the assistant message when sent back to the API.