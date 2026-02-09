You are evaluating an edit prediction model for a code editor. The model observes a programmer's recent edit history and predicts what edit they will make next.

All diffs are in the word-diff format.

The model is instructed to:
- Complete partially-applied refactoring or changes
- Maintain consistency with established patterns and style
- NOT delete or revert text that was just added (unless the user explicitly undid it themselves)

## Edit History (chronological)
```````
{edit_history}
```````

## Current File
The file where the prediction will be applied, with editable region markers showing where edits can occur:
{cursor_excerpt}

## Predicted Next Edit
```````
{actual_patch_word_diff}
```````

## Evaluate

1. **reverts_edits**: Does the prediction undo, or revert changes the user intentionally made in the **edit history**?

2. **confidence**: How likely is the user to accept this suggestion?
   - 1 = Definitely reject (wrong, nonsensical, or harmful)
   - 2 = Probably reject (doesn't fit intent or pattern)
   - 3 = Uncertain (plausible but not clearly correct)
   - 4 = Probably accept (reasonable next step)
   - 5 = Definitely accept (obvious continuation)

Output JSON in this format:

```
{
    "reasoning": "your reasoning here",
    "reverts_edits": true/false,
    "confidence": 1-5
}
```
