This is a tool for editing files. For moving or renaming files, you should generally use the `terminal` tool with the 'mv' command instead. For larger edits, use the `create_file` tool to overwrite files.

Before using this tool:

1. Use the `read_file` tool to understand the file's contents and context

2. Verify the directory path is correct (only applicable when creating new files):
   - Use the `list_directory` tool to verify the parent directory exists and is the correct location

When using this tool, you MUST group coherent edits together and include all of them in a single call. Add the full context needed for a small model to understand the edits.
