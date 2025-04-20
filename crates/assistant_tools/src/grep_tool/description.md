Searches the contents of files in the project with a regular expression

- Prefer this tool to path search when searching for symbols in the project, because you won't need to guess what path it's in.
- Supports full regex syntax (eg. "log.*Error", "function\\s+\\w+", etc.)
- Pass an `include_pattern` if you know how to narrow your search on the files system
- Never use this tool to search for paths. Only search file contents with this tool.
- Use this tool when you need to find files containing specific patterns
- Results are paginated with 20 matches per page. Use the optional 'offset' parameter to request subsequent pages.
