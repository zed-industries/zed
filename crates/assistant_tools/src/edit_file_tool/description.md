This is a tool for editing files. For moving or renaming files, you should generally use the `terminal` tool with the 'mv' command instead. For larger edits, use the `create_file` tool to overwrite files.

Before using this tool:

1. Use the `read_file` tool to understand the file's contents and context

2. Verify the directory path is correct (only applicable when creating new files):
   - Use the `list_directory` tool to verify the parent directory exists and is the correct location

Use this tool to propose an edit to an existing file. This will be read by a less intelligent model, which will quickly apply the edit. You should make it clear what the edit is, while also minimizing the unchanged code you write. When writing the edit, you should specify each edit in sequence, with the special comment `// ... existing code ...` to represent unchanged code in between edited lines.

For example:

One paragraph of clear instructions describing the nature of the change in English. Then some code to indicate where and how to make the change.

// ... existing code ...
FIRST_EDIT
// ... existing code ...
SECOND_EDIT
// ... existing code ...
THIRD_EDIT
// ... existing code ...

You should still bias towards repeating as few lines of the original file as possible to convey the change. But, each edit should contain sufficient context of unchanged lines around the code you're editing to resolve ambiguity.

DO NOT omit spans of pre-existing code (or comments) without using the `// ... existing code ...` comment to indicate its absence. If you omit the existing code comment, the model may inadvertently delete these lines. Make sure it is clear what the edit should be, and where it should be applied.

When you want to delete a piece of code, it's enough to indicate a few lines above and below the code you want to delete.

You should specify the arguments in the following order:

- [path]
- [display_description]
- [edit_instructions]
