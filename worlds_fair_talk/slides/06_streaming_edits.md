# Streaming Edits

Show the model edits as they happen, token by token.

## Challenges

1. Tool calling doesn't stream
   - JSON values must be complete before they are streamed
   - We can't use tool calling alone if we want to see streaming text
   - We ask it to stream `<old_text>` and `<new_text>` blocks

2. Parsing Complexity: XML tags arrive in random chunks
   - `</old_te` + `xt>` (split across network packets)
   - Must buffer and parse incrementally

3. Imperfect Model Behavior: Models don't follow instructions perfectly
   - Wrong closing tags: `<old_text>...</new_text>`
   - Inconsistent indentation and whitespace
   - Escaping
