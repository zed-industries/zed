Get errors and warnings for the project or a specific file.

This tool can be invoked after a series of edits to determine if further edits are necessary, or if the user asks to fix errors or warnings in their codebase.

When a path is provided, shows all diagnostics for that specific file.
When no path is provided, shows a summary of error and warning counts for all files in the project.

<example>
To get diagnostics for a specific file:
{
    "path": "src/main.rs"
}

To get a project-wide diagnostic summary:
{}
</example>

IMPORTANT: When you're done making changes, you **MUST** get the **project** diagnostics (input: `{}`) at the end of your edits so you can fix any problems you might have introduced. **DO NOT** tell the user you're done before doing this!

You may only attempt to fix these up to 3 times. If you have tried 3 times to fix them, and there are still problems remaining, you must not continue trying to fix them, and must instead tell the user that there are problems remaining - and ask if the user would like you to attempt to solve them further.
