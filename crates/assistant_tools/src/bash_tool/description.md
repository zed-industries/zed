Executes a bash one-liner and returns the combined output.

This tool spawns a bash process IN THE SPECIFIED WORKING DIRECTORY, combines stdout and stderr into one interleaved stream as they are produced (preserving the order of writes), and captures that stream into a string which is returned.

WARNING: **NEVER** use 'cd' commands to navigate to the working directory - this is automatically handled by the 'working_directory' parameter. Only use 'cd' to navigate to subdirectories within the specified working directory.

Remember that each invocation of this tool will spawn a new bash process, so you can't rely on any state from previous invocations.
