Copies a file or directory in the project, and returns confirmation that the copy succeeded.
Directory contents will be copied recursively (like `cp -r`).

This tool should be used when it's desirable to create a copy of a file or directory without modifying the original.
It's much more efficient than doing this by separately reading and then writing the file or directory's contents,
so this tool should be preferred over that approach whenever copying is the goal.
