# The Streaming Edits

Show AI edits as they happen, token by token

## Challenges

1. **API Limitations**: Tool calling doesn't stream content
   - JSON values must be complete before they are streamed
   - We can't use tool calling alone if we want to see streaming text

2. **Parsing Complexity**: XML tags arrive in random chunks
   - `</old_te` + `xt>` (split across network packets)
   - Must buffer and parse incrementally

3. **LLM Chaos**: Models don't follow instructions perfectly
   - Wrong closing tags: `<old_text>...</new_text>`
   - Escaping
   - Inconsistent indentation and whitespace

4. **Finding the Code**: LLMs give approximate matches
   - "fn calculate() {" vs "fn  calculate(){"
   - Need fuzzy matching while streaming
