# Repair Request

Your previous prediction has quality issues that need to be addressed. Please generate an improved prediction.

## Quality Feedback

{quality_feedback}

{token_change_info}

## Your Previous Prediction (word-diff format)

`````
{actual_patch_word_diff}
`````

## Instructions

Generate an improved prediction following the same rules and output format from the original instructions. The key rules remain:

- **NEVER undo or revert the user's recent edits** — if a line was removed in the edit history, do NOT restore it
- If your prediction would make the code more similar to what it was BEFORE the user's edit, output `NO_EDITS` instead
- When uncertain, predict only the minimal, high-confidence portion of the edit

## Output Format

Follow the same output format as before, with one addition:

- If the code is complete as-is and no edits should be made, output `NO_EDITS`
- **NEW: If your previous prediction was actually correct** (the quality feedback was overly cautious), output `KEEP_PREVIOUS`:

  `````
  KEEP_PREVIOUS
  `````

  Use `KEEP_PREVIOUS` when you determine the original prediction correctly addresses the user's intent despite the feedback.

**Important:** `NO_EDITS` and `KEEP_PREVIOUS` are NOT interchangeable:
- `NO_EDITS` = make zero changes to the code (discard the previous prediction)
- `KEEP_PREVIOUS` = the previous prediction is correct, use it as-is

## Your Improved Prediction

Briefly explain what was wrong with your previous prediction (or why it was actually correct), then provide the improved output.