Edit files in the current project by specifying instructions in natural language.

IMPORTANT NOTE: If there is a find-replace tool, use that instead of this tool! This tool is only to be used as a fallback in case that tool is unavailable. Always prefer that tool if it is available.

When using this tool, you should suggest one coherent edit that can be made to the codebase.

When the set of edits you want to make is large or complex, feel free to invoke this tool multiple times, each time focusing on a specific change you wanna make.

You should use this tool when you want to edit a subset of a file's contents, but not the entire file. You should not use this tool when you want to replace the entire contents of a file with completely different contents, and you absolutely must never use this tool to create new files from scratch. If you ever consider using this tool to create a new file from scratch, for any reason, instead you must reconsider and choose a different approach.

DO NOT call this tool until the code to be edited appears in the conversation! You must use the `read-files` tool or ask the user to add it to context first.
