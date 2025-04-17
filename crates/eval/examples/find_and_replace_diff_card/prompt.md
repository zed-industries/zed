Look at the `find_replace_file_tool.rs`. I want to implement a card for it. The card should be a brand new `Entity` with a `Render` implementation.

The card should show a diff. It should be a beautifully presented diff. The card "box" should look like what we show for markdown codeblocks (look at `MarkdownElement`). I want to see a red background for lines that were deleted and a green background for lines that were added. We should have a div per diff line.
