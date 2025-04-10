Get errors and warnings for the project or a specific file and apply quickfixes automatically where possible.

This tool can be invoked to find diagnostics in your code and automatically apply available quickfixes to resolve them. Quickfixes are code edits suggested by language servers that can automatically fix common issues.

When a path is provided, it checks that specific file for diagnostics and applies quickfixes.
When no path is provided, it finds diagnostics project-wide and applies quickfixes where possible.

<example>
To automatically fix issues in a specific file:
{
    "path": "src/main.rs"
}

To find and fix issues across the entire project:
{}
</example>