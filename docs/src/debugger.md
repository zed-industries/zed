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
    // accepts zed task variables e.g. $ZED_WORKPLACE_ROOT
    "cwd": "$ZED_WORKPLACE_ROOT",
    // program: The program to debug
    // accepts zed task variables
    "program": "path_to_program",
    // Additional initialization arguments to be sent on DAP initialization
    "initialize_args": {

    }
  }
]
```
