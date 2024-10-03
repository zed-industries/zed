# Outline Panel

In addition to the modal outline (`cmd-shift-o`), Zed offers an outline panel. The outline panel can be deployed via `cmd-shift-b`, or via the `Outline Panel` button in the status bar.

When viewing a "singleton" buffer, the outline panel works similarly to that of the outline modal - it displays the outline of the current buffer's symbols, as reported by tree-sitter. Clicking on an entry allows you to jump to the associated section in the file. The outline view will also automatically scroll to the section associated with the current cursor position within the file.

![Using the outline panel in a singleton buffer](https://zed.dev/img/outline-panel/singleton.png)

The outline panel begins to shine when used within the context of a multi-buffer. Here are some examples:

![Using the outline panel in a project search multi-buffer](https://zed.dev/img/outline-panel/project-search.png)

![Using the outline panel while viewing project diagnostics multi-buffer](https://zed.dev/img/outline-panel/project-diagnostics.png)

![Using the outline panel while viewing `find all references` multi-buffer](https://zed.dev/img/outline-panel/find-all-references.png)

The outline view provides a great way to quickly navigate to specific parts of your code and helps you maintain context when working with large result sets in multi-buffers.
