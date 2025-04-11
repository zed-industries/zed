1. Add a new visual `DiffCard` entity to render diffs with proper styling
2. Create a new module `find_replace_tool` with `diff_card.rs` and `mod.rs` files
3. Modify the `FindReplaceFileTool` to return the new visual card entity instead of plain text diff
4. Implement colorful diff rendering with:
  - Red background for deleted lines
  - Green background for added lines
  - Proper spacing and border styling
5. Change the tool output to use `ToolOutput` struct with both message and entity
