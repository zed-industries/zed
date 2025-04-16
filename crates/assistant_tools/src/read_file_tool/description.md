Reads the content of the given file in the project.

If the file is too big to read all at once, and neither a start line
nor an end line was specified, then this returns an outline of the
file's symbols (with line numbers) instead of the file's contents,
so that it can be called again with line ranges.
