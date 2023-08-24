- Style the current inline editor
- Find a way to understand whether we want to refactor or append, or both. (function calls)
- Add a system prompt that makes GPT an expert of language X
- Provide context around the cursor/selection. We should try to fill the context window as much as possible (try to fill half of it so that we can spit out another half)
- When you hit escape, the assistant should stop.
- When you hit undo and you undo a transaction from the assistant, we should stop generating.
- Keep the inline editor around until the assistant is done. Add a cancel button to stop, and and undo button to undo the whole thing. (Interactive<IconButton>)


# 9:39 AM

- Hit `ctrl-enter`

- Puts me in assistant mode with the selected text highlighted in a special color. If text was selected, I'm in transformation mode.
- If there's no selection, put me on the line below, aligned with the indent of the line.
- Enter starts generation
- Ctrl-enter inserts a newline
- Once generations starts, enter "confirms it" by dismissing the inline editor.
- Escape in the inline editor cancels/undoes/dismisses.
- To generate text in reference to other text, we can *mark* text.


- Hit ctrl-enter deploys an edit prompt
    - Empty selection (cursor) => append text
        - On end of line: Edit prompt on end of line.
        - Middle of line: Edit prompt near cursor head on a different line
    - Non-empty selection => refactor
        - Edit prompt near cursor head on a different line
        - What was selected when you hit ctrl-enter is colored.
- Selection is cleared and cursor is moved to prompt input
- When cursor is inside a prompt
    - Escape cancels/undoes
    - Enter confirms
- Multicursor
    - Run the same prompt for every selection in parallel
    - Position the prompt editor at the newest cursor
- Follow up ship: Marks
    - Global across all buffers
    - Select text, hit a binding
    - That text gets added to the marks
        - Simplest: Marks are a set, and you add to them with this binding.
        - Could this be a stack? That might be too much.
    - When you hit ctrl-enter to generate / transform text, we include the marked text in the context.

- During inference, always send marked text.
- During inference, send as much context as possible given the user's desired generation length.

- This would assume a convenient binding for setting the generation length.


~~~~~~~~~

Dial up / dial down how much context we send
Dial up / down your max generation length.


------- (merge to main)

- Text in the prompt should soft wrap

----------- (maybe pause)

- Excurse outside of the editor without dismissing it... kind of like a message in the assistant.
