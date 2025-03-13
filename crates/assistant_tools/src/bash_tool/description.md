Executes a bash one-liner and returns the combined output.

This tool spawns a bash process, combines stdout and stderr into one interleaved stream as they are produced (preserving the order of writes), and captures that stream into a string which is returned.

Remember that each invocation of this tool will spawn a new bash process, so you can't rely on any state from previous invocations.
