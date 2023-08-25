- Hit ctrl-enter deploys an edit prompt
    - Empty selection (cursor) => append text
        - On end of line: Edit prompt on end of line.
        - [x] Middle of line: Edit prompt near cursor head on a different line
    - Non-empty selection => refactor
        - [x] Edit prompt near cursor head on a different line
        - [x] What was selected when you hit ctrl-enter is colored.
- [x] Add placeholder text
    - If non-empty selection: Enter prompt to transform selected text
    - If empty selection: Enter prompt to generate text
- When cursor is inside a prompt
    - [x] Escape cancels/undoes
    - [x] Enter confirms
- [ ] Selection is cleared and cursor is moved to prompt input
- [ ] Ability to highlight background multiple times for the same type
- [x] Basic Styling
- [ ] Match lowest indentation level of selected lines when inserting an inline assist
- [ ] Look into why insert prompts have a weird indentation sometimes




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
