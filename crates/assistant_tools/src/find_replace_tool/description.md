Find one unique part of a file in the project and replace that text with new text.

This tool is the preferred way to make edits to files *except* when making a rename. When making a rename specifically, the rename tool must always be used instead.

If you have multiple edits to make, including edits across multiple files, then make a plan to respond with a single message containing a batch of calls to this tool - one call for each find/replace operation.

You should only use this tool when you want to edit a subset of a file's contents, but not the entire file. You should not use this tool when you want to replace the entire contents of a file with completely different contents. You also should not use this tool when you want to move or rename a file. You absolutely must NEVER use this tool to create new files from scratch. If you ever consider using this tool to create a new file from scratch, for any reason, instead you must reconsider and choose a different approach.

DO NOT call this tool until the code to be edited appears in the conversation! You must use another tool to read the file's contents into the conversation, or ask the user to add it to context first.

Never call this tool with identical "find" and "replace" strings. Instead, stop and think about what you actually want to do.

REMEMBER: You can use this tool after you just used the `create_file` tool. It's better to edit the file you just created than to recreate a new file from scratch.
