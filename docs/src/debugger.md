# Debugger

## Debug Configuration

To debug a program using Zed you must first create a debug configuration within your project located at `.zed/debug.json`

```json
[
  {
    "label": "Example Start debugger config"
    // The debug adapter to use
    // Zed supports javascript, python, lldb, go, and custom out of the box
    "adapter": "custom",
    // request: defaults to launch
    //  - launch: Zed will launch the program to be debugged
    //  - attach: Zed will attach to a running program to debug it
    "request": "launch",
    // cwd: defaults to the current working directory of your project
    // The current working directory to start the debugger from
    // accepts zed task variables e.g. $ZED_WORKTREE_ROOT
    "cwd": "$ZED_WORKTREE_ROOT",
    // program: The program to debug
    // accepts zed task variables
    "program": "path_to_program",
    // Additional initialization arguments to be sent on DAP initialization
    "initialize_args": {

    }
  }
]
```

## Breakpoints

Zed currently supports these types of breakpoints

- Log Breakpoints: Output a log message instead of stopping at the breakpoint when it's hit
- Standard Breakpoints: Stop at the breakpoint when it's hit

Standard breakpoints can be toggled by left clicking on the editor gutter or using the Toggle Breakpoint action. Right clicking on a breakpoint, code action symbol, or code runner symbol brings up the breakpoint context menu. That has options for toggling breakpoints and editing log breakpoints.

Log breakpoints can also be edited/added through the edit log breakpoint action

## Starting a Debugger Session

A debugger session can be started by the Start Debugging action or clicking the "Choose Debugger" button in the debugger panel when there are no active sessions.

Zed supports having multiple sessions
