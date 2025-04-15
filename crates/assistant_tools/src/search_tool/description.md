Searches the project for files, code symbols, and text content in a unified way.

This tool combines the functionality of path searching, regex searching within files, and code symbol searching, allowing for powerful and flexible code exploration.

The tool can:
1. Find files by matching path patterns
2. Search for text content within files using regular expressions
3. Find code symbols (functions, classes, variables, etc.) in specific files or across the project

Results are paginated with matches per page varying based on the search mode.

When searching for files, it will return paths in the project matching the path pattern.

When searching for text content, it will return file paths, line numbers, and context for each match.

When searching for code symbols, it will return a hierarchical outline or a flat list of symbols with their locations.

Use this tool when you need to find specific files, code symbols, or text patterns across your project.

You should very strongly prefer to use "output": "symbols" when searching for code symbols such as functions, types, classes, etc. This is because "output": "symbols" uses the language server to perform a semantic search of the project, which will give you more accurate results than "output": "text".

<good_example>
To find where a class named "Item" is defined in the project:

{
  "output": "symbols",
  "query": "Item"
}
</good_example>

<bad_example>
This is the incorrect way to find where a class named "Item" is defined in the project:

{
  "output": "text",
  "query": "class.*Item"
}
</bad_example>
