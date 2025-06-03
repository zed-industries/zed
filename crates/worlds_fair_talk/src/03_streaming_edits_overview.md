**The Streaming Edits Challenge**

What we're building:
- Show AI edits as they happen (character by character)
- Users can see progress and cancel if needed
- No more waiting for 100 lines to generate

The technical challenges:
1. **API Limitation**: Tool calling doesn't stream content
   - JSON values must be complete before sending
   - Can't see edits being generated
   
2. **Parsing Complexity**: XML tags arrive in random chunks
   - `</old_te` + `xt>` (split across network packets)
   - Must buffer and parse incrementally
   
3. **LLM Chaos**: Models don't follow instructions perfectly
   - Wrong closing tags: `<old_text>...</new_text>`
   - Hallucinated code references
   - Inconsistent whitespace

4. **Finding the Code**: LLMs give approximate matches
   - "fn calculate() {" vs "fn  calculate(){"
   - Need fuzzy matching while streaming