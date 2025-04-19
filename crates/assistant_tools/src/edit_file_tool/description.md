This is a tool for editing files. For moving or renaming files, you should generally use the `terminal` tool with the 'mv' command instead. For larger edits, use the `create_file` tool to overwrite files.

Before using this tool:

1. Use the `read_file` tool to understand the file's contents and context

2. Verify the directory path is correct (only applicable when creating new files):
   - Use the `list_directory` tool to verify the parent directory exists and is the correct location

To make a file edit, provide the following:
1. path: The full path to the file you wish to modify in the project. This path must include the root directory in the project.
2. old_string: The text to replace (must be unique within the file, and must match the file contents exactly, including all whitespace and indentation)
3. new_string: The edited text, which will replace the old_string in the file.

The tool will replace ONE occurrence of old_string with new_string in the specified file.

CRITICAL REQUIREMENTS FOR USING THIS TOOL:

1. UNIQUENESS: The old_string MUST uniquely identify the specific instance you want to change. This means:
   - Include AT LEAST 3-5 lines of context BEFORE the change point
   - Include AT LEAST 3-5 lines of context AFTER the change point
   - Include all whitespace, indentation, and surrounding code exactly as it appears in the file

2. SINGLE INSTANCE: This tool can only change ONE instance at a time. If you need to change multiple instances:
   - Make separate calls to this tool for each instance
   - Each call must uniquely identify its specific instance using extensive context

3. VERIFICATION: Before using this tool:
   - Check how many instances of the target text exist in the file
   - If multiple instances exist, gather enough context to uniquely identify each one
   - Plan separate tool calls for each instance

WARNING: If you do not follow these requirements:
   - The tool will fail if old_string matches multiple locations
   - The tool will fail if old_string doesn't match exactly (including whitespace)
   - You may change the wrong instance if you don't include enough context

When making edits:
   - Ensure the edit results in idiomatic, correct code
   - Do not leave the code in a broken state
   - Always use fully-qualified project paths (starting with the name of one of the project's root directories)

If you want to create a new file, use the `create_file` tool instead of this tool. Don't pass an empty `old_string`.

Remember: when making multiple file edits in a row to the same file, you should prefer to send all edits in a single message with multiple calls to this tool, rather than multiple messages with a single call each.
