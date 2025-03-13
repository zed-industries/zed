Executes a bash one-liner and returns the combined output.

This tool spawns a bash process, combines stdout and stderr into one interleaved stream as they are produced (preserving the order of writes), and captures that stream into a string which is returned.

The bash process will be spawned in the user's home directory. If you want to run commands in a given top-level project directory, use the absolute path of that directory and `cd` into it before executing anything else.

Remember that each invocation of this tool will spawn a new bash process, so you can't rely on any state from previous invocations.
