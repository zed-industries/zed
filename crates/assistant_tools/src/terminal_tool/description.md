Executes a shell one-liner and returns the combined output.

This tool spawns a process using the user's shell, reads from stdout and stderr (preserving the order of writes), and returns a string with the combined output result.

The output results will be shown to the user already, only list it again if necessary, avoid being redundant.

Make sure you use the `cd` parameter to navigate to one of the root directories of the project. NEVER do it as part of the `command` itself, otherwise it will error.

Do not use this tool for commands that run indefinitely, such as servers (like `npm run start`, `npm run dev`, `python -m http.server`, etc) or file watchers that don't terminate on their own.

Remember that each invocation of this tool will spawn a new shell process, so you can't rely on any state from previous invocations.
