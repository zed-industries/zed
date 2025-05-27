This tool opens a file or URL with the default application associated with it on the user's operating system:
- On macOS, it's equivalent to the `open` command
- On Windows, it's equivalent to `start`
- On Linux, it uses something like `xdg-open`, `gio open`, `gnome-open`, `kde-open`, `wslview` as appropriate

For example, it can open a web browser with a URL, open a PDF file with the default PDF viewer, etc.

You MUST ONLY use this tool when the user has explicitly requested opening something. You MUST NEVER assume that
the user would like for you to use this tool.
