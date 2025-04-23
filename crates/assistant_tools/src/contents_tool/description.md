Reads the contents of a path on the filesystem.

If the path is a directory, this lists all files and directories within that path.
If the path is a file, this returns the file's contents.

When reading a file, if the file is too big and no line range is specified, an outline of the file's code symbols is listed instead, which can be used to request specific line ranges in a subsequent call.

Similarly, if a directory has too many entries to show at once, a subset of entries will be shown,
and subsequent requests can use starting and ending line numbers to get other subsets.
