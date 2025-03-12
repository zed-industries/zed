Evaluates the given Lua script in an interpreter with access to the Lua standard library. The tool returns the scripts output to stdout and any error that may have occurred.

Use this tool to explore the current project and edit the user's codebase or operating system as requested.

Additional functions provided:

```lua
--- Search for matches of a regular expression in files.
-- @param pattern The regex pattern to search for (uses Rust's regex syntax)
-- @return An array of tables with 'path' (file path) and 'matches' (array of matching strings)
-- @usage local results = search("function\\s+\\w+")
function search(pattern)
  -- Implementation provided by the tool
end

--- Generates an outline for the given file path, extracting top-level symbols such as functions, classes, exports, and other significant declarations. This provides a structural overview of the file's contents.
-- @param path
function outline(path)
  -- Implementation provided by the tool
end
```
