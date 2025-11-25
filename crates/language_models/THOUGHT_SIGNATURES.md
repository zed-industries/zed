# Thought Signatures Implementation for Gemini 3 Models

## Problem Statement

Gemini 3 models (like `gemini-3-pro-preview`) fail when using tool calls through OpenRouter and Copilot with the error:

```
Unable to submit request because function call `default_api:list_directory` in the 2. content block is missing a `thought_signature`.
```

The error occurs AFTER the first tool call is executed and we send back the tool results with conversation history.

## Background

### What are Thought Signatures?

Thought signatures are a validation mechanism used by Gemini reasoning models. When the model performs "thinking" (reasoning) before making a tool call, it generates a cryptographic signature of that reasoning. This signature must be preserved and sent back in subsequent requests to maintain the integrity of the conversation flow.

### API Formats Involved

There are three different API formats in play:

1. **Google AI Native API** - Uses `Part` objects including `FunctionCallPart` with a `thought_signature` field
2. **OpenRouter/Copilot Chat Completions API** - OpenAI-compatible format with `tool_calls` array
3. **Copilot Responses API** - A separate format with streaming `reasoning_details`

## Current Architecture

### Data Flow

1. **Model Response** → Contains tool calls with reasoning
2. **Zed Event Stream** → Emits `LanguageModelCompletionEvent::ToolUse` events
3. **Agent** → Collects events and constructs `LanguageModelRequestMessage` objects
4. **Provider** → Converts messages back to provider-specific format
5. **API Request** → Sent back to the provider with conversation history

### Key Data Structures

```rust
// Core message structure
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
    pub cache: bool,
    pub reasoning_details: Option<serde_json::Value>, // Added for thought signatures
}

// Tool use structure
pub struct LanguageModelToolUse {
    pub id: LanguageModelToolUseId,
    pub name: Arc<str>,
    pub raw_input: String,
    pub input: serde_json::Value,
    pub is_input_complete: bool,
    pub thought_signature: Option<String>, // NOT USED - wrong approach
}
```

## What We Tried (That Didn't Work)

### Attempt 1: `thought_signature` as field on ToolCall
We added `thought_signature` as a field on the `ToolCall` structure itself.

**Result:** 400 Bad Request - OpenRouter/Copilot don't support this field at the ToolCall level.

### Attempt 2: `thought_signature` inside `function` object
We moved `thought_signature` inside the `function` object of the tool call.

```json
{
  "function": {
    "name": "...",
    "arguments": "...",
    "thought_signature": "..."
  }
}
```

**Result:** 400 Bad Request - Still rejected.

### Attempt 3: Using camelCase `thoughtSignature`
Tried both snake_case and camelCase variants.

**Result:** No difference, still rejected.

## The Correct Approach (From OpenRouter Documentation)

According to [OpenRouter's documentation](https://openrouter.ai/docs/use-cases/reasoning-tokens#preserving-reasoning-blocks):

### Key Insight: `reasoning_details` is a message-level array

The thought signature is NOT a property of individual tool calls. Instead, it's part of a `reasoning_details` array that belongs to the entire assistant message:

```json
{
  "role": "assistant",
  "content": null,
  "tool_calls": [
    {
      "id": "call_123",
      "type": "function",
      "function": {
        "name": "list_directory",
        "arguments": "{...}"
      }
    }
  ],
  "reasoning_details": [
    {
      "type": "reasoning.text",
      "text": "Let me think through this step by step...",
      "signature": "sha256:abc123...",
      "id": "reasoning-text-1",
      "format": "anthropic-claude-v1",
      "index": 0
    }
  ]
}
```

### `reasoning_details` Structure

The array can contain three types of objects:

1. **reasoning.summary** - High-level summary of reasoning
2. **reasoning.encrypted** - Encrypted/redacted reasoning data
3. **reasoning.text** - Raw text reasoning with optional signature

Each object has:
- `type`: One of the three types above
- `id`: Unique identifier
- `format`: Format version (e.g., "anthropic-claude-v1", "openai-responses-v1")
- `index`: Sequential index
- `signature`: (for reasoning.text) The cryptographic signature we need to preserve

## What We've Implemented So Far

### 1. Added `reasoning_details` field to core structures

✅ `LanguageModelRequestMessage` now has `reasoning_details: Option<serde_json::Value>`

### 2. Added `reasoning_details` to OpenRouter structs

✅ `RequestMessage::Assistant` has `reasoning_details` field
✅ `ResponseMessageDelta` has `reasoning_details` field

### 3. Updated `into_open_router` to send `reasoning_details`

✅ When building requests, we now attach `reasoning_details` from the message to the Assistant message

### 4. Added mapper to capture `reasoning_details` from responses

✅ `OpenRouterEventMapper` now has a `reasoning_details` field
✅ We capture it from `choice.delta.reasoning_details`

### 5. Added debugging

✅ `eprintln!` statements in both OpenRouter and Copilot to log requests and responses

## What's Still Missing

### The Critical Gap: Event → Message Flow

The problem is in how events become messages. Our current flow:

1. ✅ We capture `reasoning_details` from the API response
2. ❌ We store it in `OpenRouterEventMapper` but never emit it
3. ❌ The agent constructs messages from events, but has no way to get the `reasoning_details`
4. ❌ When sending the next request, `message.reasoning_details` is `None`

### What We Need to Do

#### Option A: Add a new event type

Add a `LanguageModelCompletionEvent::ReasoningDetails(serde_json::Value)` event that gets emitted when we receive reasoning details. The agent would need to:

1. Collect this event along with tool use events
2. When constructing the assistant message, attach the reasoning_details to it

#### Option B: Store reasoning_details with tool use events

Modify the flow so that when we emit tool use events, we somehow associate the `reasoning_details` with them. This is tricky because:
- `reasoning_details` is per-message, not per-tool
- Multiple tools can be in one message
- We emit events one at a time

#### Option C: Store at a higher level

Have the agent or provider layer handle this separately from the event stream. For example:
- The provider keeps track of reasoning_details for messages it processes
- When building the next request, it looks up the reasoning_details for assistant messages that had tool calls

## Current Status

### What Works
- ✅ Code compiles
- ✅ `reasoning_details` field exists throughout the stack
- ✅ We capture `reasoning_details` from responses
- ✅ We send `reasoning_details` in requests (if present)

### What Doesn't Work
- ❌ `reasoning_details` never makes it from the response to the request
- ❌ The error still occurs because we're sending `null` for `reasoning_details`

### Evidence from Error Message

The error says:
```
function call `default_api:list_directory` in the 2. content block is missing a `thought_signature`
```

This means:
1. We're successfully making the first request (works)
2. The model responds with tool calls including reasoning_details (works)
3. We execute the tools (works)
4. We send back the conversation history (works)
5. BUT the assistant message in that history is missing the reasoning_details (broken)
6. Google/Vertex validates the message and rejects it (error)

## Next Steps

1. **Choose an approach** - Decide between Option A, B, or C above
2. **Implement the data flow** - Ensure `reasoning_details` flows from response → events → message → request
3. **Test with debugging** - Use the `eprintln!` statements to verify:
   - That we receive `reasoning_details` in the response
   - That we include it in the next request
4. **Apply to Copilot** - Once working for OpenRouter, apply the same pattern to Copilot
5. **Handle edge cases**:
   - What if there are multiple tool calls in one message?
   - What if reasoning_details is empty/null?
   - What about other providers (Anthropic, etc.)?

## Files Modified

- `crates/language_model/src/request.rs` - Added `reasoning_details` to `LanguageModelRequestMessage`
- `crates/open_router/src/open_router.rs` - Added `reasoning_details` to request/response structs
- `crates/language_models/src/provider/open_router.rs` - Added capture and send logic
- `crates/copilot/src/copilot_responses.rs` - Already had `thought_signature` support
- Various test files - Added `reasoning_details: None` to fix compilation

## SOLUTION: Copilot Chat Completions API Implementation

### Discovery: Gemini 3 Uses Chat Completions API, Not Responses API

Initial plan assumed routing Gemini 3 to Responses API would work, but testing revealed:
- **Gemini 3 models do NOT support the Responses API** through Copilot
- Error: `{"error":{"message":"model gemini-3-pro-preview is not supported via Responses API.","code":"unsupported_api_for_model"}}`
- Gemini 3 ONLY supports the Chat Completions API

### Key Finding: `reasoning_opaque` Location in JSON

Through detailed logging and JSON inspection, discovered Copilot sends thought signatures in Chat Completions API:
- Field name: **`reasoning_opaque`** (not `thought_signature`)
- Location: **At the `delta` level**, NOT at the `tool_calls` level!

JSON structure from Copilot response:
```json
{
  "choices": [{
    "delta": {
      "role": "assistant",
      "tool_calls": [{
        "function": {"arguments": "...", "name": "list_directory"},
        "id": "call_...",
        "index": 0,
        "type": "function"
      }],
      "reasoning_opaque": "sPsUMpfe1YZXLkbc0TNW/mJLT..."  // <-- HERE!
    }
  }]
}
```

### Implementation Status

#### ✅ Completed Changes

1. **Added `reasoning_opaque` field to `ResponseDelta`** (`crates/copilot/src/copilot_chat.rs`)
   ```rust
   pub struct ResponseDelta {
       pub content: Option<String>,
       pub role: Option<Role>,
       pub tool_calls: Vec<ToolCallChunk>,
       pub reasoning_opaque: Option<String>,  // Added this
   }
   ```

2. **Added `thought_signature` fields to Chat Completions structures** (`crates/copilot/src/copilot_chat.rs`)
   - `FunctionContent` now has `thought_signature: Option<String>`
   - `FunctionChunk` now has `thought_signature: Option<String>`

3. **Updated mapper to capture `reasoning_opaque` from delta** (`crates/language_models/src/provider/copilot_chat.rs`)
   - Captures `reasoning_opaque` from `delta.reasoning_opaque`
   - Applies it to all tool calls in that delta
   - Stores in `thought_signature` field of accumulated tool call

4. **Verified thought signature is being sent back**
   - Logs show: `📤 Chat Completions: Sending tool call list_directory with thought_signature: Some("sPsUMpfe...")`
   - Signature is being included in subsequent requests

#### ❌ Current Issue: Still Getting 400 Error

Despite successfully capturing and sending back the thought signature, Copilot still returns:
```
400 Bad Request {"error":{"message":"invalid request body","code":"invalid_request_body"}}
```

This happens on the SECOND request (after tool execution), when sending conversation history back.

### Debug Logging Added

Current logging shows the full flow:
- `📥 Chat Completions: Received reasoning_opaque (length: XXX)` - Successfully captured
- `🔍 Tool call chunk: index=..., id=..., has_function=...` - Delta processing
- `📤 Chat Completions: Emitting ToolUse for ... with thought_signature: Some(...)` - Event emission
- `📤 Chat Completions: Sending tool call ... with thought_signature: Some(...)` - Sending back
- `📤 Chat Completions Request JSON: {...}` - Full request being sent
- `📥 Chat Completions Response Event: {...}` - Full response received

### Potential Issues to Investigate

1. **Field name mismatch on send**: We're sending `thought_signature` but should we send `reasoning_opaque`?
   - We added `thought_signature` to `FunctionContent` 
   - But Copilot might expect `reasoning_opaque` in the request just like it sends it

2. **Serialization issue**: Check if serde is properly serializing the field
   - Added `#[serde(skip_serializing_if = "Option::is_none")]` - might be skipping it?
   - Should verify field appears in actual JSON being sent

3. **Location issue**: Even when sending back, should `reasoning_opaque` be at delta level?
   - Currently putting it in `function.thought_signature`
   - Might need to be at a different level in the request structure

4. **Format validation**: The signature is a base64-encoded string ~1464 characters
   - Copilot might be validating the signature format/content
   - Could be rejecting it if it's malformed or in wrong structure

### Next Steps to Debug

1. **Check actual JSON being sent**: Look at the `📤 Chat Completions Request JSON` logs
   - Search for `thought_signature` in the JSON
   - Verify it's actually in the serialized output (not skipped)
   - Check its exact location in the JSON structure

2. **Try renaming field**: Change `thought_signature` to `reasoning_opaque` in request structures
   - In `FunctionContent` struct
   - In `FunctionChunk` struct
   - See if Copilot expects same field name in both directions

3. **Compare request format to response format**: 
   - Response has `reasoning_opaque` at delta level
   - Request might need it at function level OR delta level
   - May need to restructure where we put it

4. **Test with tool choice parameter**: Some APIs are sensitive to request structure
   - Try with/without `tool_choice` parameter
   - Try with minimal conversation history

5. **Check Copilot API documentation**: 
   - Search for official docs on `reasoning_opaque` handling
   - Look for examples of tool calls with reasoning/thinking in Copilot API

### Files Modified

- ✅ `crates/copilot/src/copilot_chat.rs` - Added `reasoning_opaque` to `ResponseDelta`, `thought_signature` to function structs
- ✅ `crates/language_models/src/provider/copilot_chat.rs` - Capture and send logic with debug logging
- ⏳ Still need to verify serialization and field naming

### References

- [OpenRouter Reasoning Tokens Documentation](https://openrouter.ai/docs/use-cases/reasoning-tokens)
- [Google Thought Signatures Documentation](https://ai.google.dev/gemini-api/docs/thinking#signatures)
- [Original Issue #43024](https://github.com/zed-industries/zed/issues/43024)
## ✅ FINAL FIX (2025-01-21)

### The Critical Issues Found

After testing, we discovered TWO problems:

1. **Wrong Location**: We were sending `thought_signature` inside the `function` object, but Copilot expects `reasoning_opaque` at the **message level**
2. **Wrong Content Format**: We were sending `"content": []` (empty array), but Copilot expects `"content": null` when there are tool calls

### The Solution

#### Issue 1: Message-Level Field
- **Added** `reasoning_opaque: Option<String>` to `ChatMessage::Assistant`
- **Removed** `thought_signature` from `FunctionContent` (it doesn't belong there)
- **Updated** request builder to collect signature from first tool use and pass at message level

#### Issue 2: Null vs Empty Array
- **Changed** `content` field type from `ChatMessageContent` to `Option<ChatMessageContent>`
- **Set** `content: None` when we have tool calls and no text (serializes to `null`)
- **Set** `content: Some(text)` when we have text content

### Correct Request Format

```json
{
  "role": "assistant",
  "content": null,  // ✅ Explicit null, not []
  "tool_calls": [{
    "id": "call_...",
    "type": "function",
    "function": {
      "name": "list_directory",
      "arguments": "{\"path\":\"deleteme\"}"
      // NO thought_signature here!
    }
  }],
  "reasoning_opaque": "XLn4be0..."  // ✅ At message level!
}
```

### Files Modified in Final Fix

- `zed/crates/copilot/src/copilot_chat.rs`:
  - Added `reasoning_opaque` to `ChatMessage::Assistant`
  - Changed `content` to `Option<ChatMessageContent>`
  - Fixed vision detection pattern match
- `zed/crates/language_models/src/provider/copilot_chat.rs`:
  - Collect `reasoning_opaque` from first tool use
  - Pass to Assistant message, not function
  - Set `content: None` for tool-only messages
  - Removed function-level thought_signature handling

### Compilation Status

✅ All packages compile successfully

Ready for testing!
