# List History Tool

Enumerate a slice of the conversation thread's messages with stable indices, lightweight previews, and optional full markdown content.

## Purpose

This tool is designed for planning context reduction by allowing you to inspect the conversation history before deciding which messages to archive using the `memory` tool.

## Parameters

- `start` (optional): Inclusive starting message index (default: 0)
- `limit` (optional): Number of messages to enumerate (default: 40, clamped between 1 and 500)
- `max_chars_per_message` (optional): Preview character cap per message (default: 160, clamped between 16 and 4096)
- `include_full_markdown` (optional): If true, appends full text of each listed message after the table (default: false)

## Output Format

The tool produces:

1. A JSON summary block showing total message count and the range being displayed
2. A markdown table with columns:
   - **Idx**: Message index in the conversation
   - **Role**: The role of the message (User, Assistant, System, etc.)
   - **Kind**: Type of message (message, tool_call, tool_result)
   - **Chars**: Character count of the message content
   - **Preview**: Truncated preview of the message content

3. Optionally, full message content for each message in the range

## Usage Example

```json
{
  "start": 0,
  "limit": 50,
  "max_chars_per_message": 200,
  "include_full_markdown": false
}
```

## Typical Workflow

1. Call `list_history` to inspect a range of messages
2. Identify messages that can be archived (e.g., older messages that are no longer immediately relevant)
3. Use the `memory` tool with operation "store" to archive those messages
4. Continue the conversation with reduced context

## When to Use

- When token usage approaches limits (60-70% of context window)
- To review what's been discussed in the conversation
- Before archiving messages to verify the range is correct
- To find specific information by scanning through message previews