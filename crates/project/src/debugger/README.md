# Debugger

Zed uses the Debug Adapter Protocol (DAP) to provide debugging functionality across multiple programming languages.
DAP is a standardized protocol that defines how debuggers, editors, and IDEs communicate with each other.
It allows Zed to support various debuggers without needing to implement language-specific debugging logic.
This protocol enables features like setting breakpoints, stepping through code, inspecting variables,
and more, in a consistent manner across different programming languages and runtime environments.

## Supported Debug Adapters

Zed supports a variety of debug adapters for different programming languages:

- JavaScript (node): Enables debugging of Node.js applications, including setting breakpoints, stepping through code, and inspecting variables in JavaScript.

- Python (debugpy): Provides debugging capabilities for Python applications, supporting features like remote debugging, multi-threaded debugging, and Django/Flask application debugging.

- LLDB: A powerful debugger for C, C++, Objective-C, and Swift, offering low-level debugging features and support for Apple platforms.

- GDB: The GNU Debugger, which supports debugging for multiple programming languages including C, C++, Go, and Rust, across various platforms.

- Go (dlv): Delve, a debugger for the Go programming language, offering both local and remote debugging capabilities with full support for Go's runtime and standard library.

- PHP (xdebug): Provides debugging and profiling capabilities for PHP applications, including remote debugging and code coverage analysis.

- Custom: Allows you to configure any debug adapter that supports the Debug Adapter Protocol, enabling debugging for additional languages or specialized environments not natively supported by Zed.

These adapters enable Zed to provide a consistent debugging experience across multiple languages while leveraging the specific features and capabilities of each debugger.

## How To Get Started

To start a debug session, we added few default debug configurations for each supported language that supports generic configuration options. To see all the available debug configurations, you can use the command palette `debugger: start` action, this should list all the available debug configurations.

### Configuration

To create a custom debug configuration you have to create a `.zed/debug.json` file in your project root directory. This file should contain an array of debug configurations, each with a unique label and adapter the other option are optional/required based on the adapter.

```json
[
  {
    // The label for the debug configuration and used to identify the debug session inside the debug panel
    "label": "Example Start debugger config",
    // The debug adapter that Zed should use to debug the program
    "adapter": "custom",
    // Request: defaults to launch
    //  - launch: Zed will launch the program if specified or shows a debug terminal with the right configuration
    //  - attach: Zed will attach to a running program to debug it or when the process_id is not specified we will show a process picker (only supported for node currently)
    "request": "launch",
    // cwd: defaults to the current working directory of your project ($ZED_WORKTREE_ROOT)
    // this field also supports task variables e.g. $ZED_WORKTREE_ROOT
    "cwd": "$ZED_WORKTREE_ROOT",
    // program: The program that you want to debug
    // this fields also support task variables e.g. $ZED_FILE
    // Note: this field should only contain the path to the program you want to debug
    "program": "path_to_program",
    // initialize_args: This field should contain all the adapter specific initialization arguments that are directly send to the debug adapter
    "initialize_args": {
      // "stopOnEntry": true // e.g. to stop on the first line of the program (These args are DAP specific)
    },
    // connection: the connection that a custom debugger should use
    "connection": "stdio",
    // The cli command used to start the debug adapter e.g. `python3`, `node` or the adapter binary
    "command": "path_to_cli"
  }
]
```

### Using Attach [WIP]

Only javascript and lldb supports starting a debug session using attach.

When using the attach request with a process ID the syntax is as follows:

```json
{
  "label": "Attach to Process",
  "adapter": "javascript",
  "request": {
    "attach": {
      "process_id": "12345"
    }
  }
}
```

Without process ID the syntax is as follows:

```json
{
  "label": "Attach to Process",
  "adapter": "javascript",
  "request": {
    "attach": {}
  }
}
```

#### JavaScript Configuration

##### Debug Active File

This configuration allows you to debug a JavaScript file in your project.

```json
{
  "label": "JavaScript: Debug Active File",
  "adapter": "javascript",
  "program": "$ZED_FILE",
  "request": "launch",
  "cwd": "$ZED_WORKTREE_ROOT"
}
```

##### Debug Terminal

This configuration will spawn a debug terminal where you could start you program by typing `node test.js`, and the debug adapter will automatically attach to the process.

```json
{
  "label": "JavaScript: Debug Terminal",
  "adapter": "javascript",
  "request": "launch",
  "cwd": "$ZED_WORKTREE_ROOT",
  // "program": "$ZED_FILE", // optional if you pass this in, you will see the output inside the terminal itself
  "initialize_args": {
    "console": "integratedTerminal"
  }
}
```

#### PHP Configuration

##### Debug Active File

This configuration allows you to debug a PHP file in your project.

```json
{
  "label": "PHP: Debug Active File",
  "adapter": "php",
  "program": "$ZED_FILE",
  "request": "launch",
  "cwd": "$ZED_WORKTREE_ROOT"
}
```

#### Python Configuration

##### Debug Active File

This configuration allows you to debug a Python file in your project.

```json
{
  "label": "Python: Debug Active File",
  "adapter": "python",
  "program": "$ZED_FILE",
  "request": "launch",
  "cwd": "$ZED_WORKTREE_ROOT"
}
```

#### GDB Configuration

**NOTE:** This configuration is for Linux systems only & intel macbooks.

##### Debug Program

This configuration allows you to debug a program using GDB e.g. Zed itself.

```json
{
  "label": "GDB: Debug program",
  "adapter": "gdb",
  "program": "$ZED_WORKTREE_ROOT/target/debug/zed",
  "request": "launch",
  "cwd": "$ZED_WORKTREE_ROOT"
}
```

#### LLDB Configuration

##### Debug Program

This configuration allows you to debug a program using LLDB e.g. Zed itself.

```json
{
  "label": "LLDB: Debug program",
  "adapter": "lldb",
  "program": "$ZED_WORKTREE_ROOT/target/debug/zed",
  "request": "launch",
  "cwd": "$ZED_WORKTREE_ROOT"
}
```

## Breakpoints

Zed currently supports these types of breakpoints

- Log Breakpoints: Output a log message instead of stopping at the breakpoint when it's hit
- Standard Breakpoints: Stop at the breakpoint when it's hit

Standard breakpoints can be toggled by left clicking on the editor gutter or using the Toggle Breakpoint action. Right clicking on a breakpoint, code action symbol, or code runner symbol brings up the breakpoint context menu. That has options for toggling breakpoints and editing log breakpoints.

Log breakpoints can also be edited/added through the edit log breakpoint action

## Settings

- `stepping_granularity`: Determines the stepping granularity.
- `save_breakpoints`: Whether the breakpoints should be reused across Zed sessions.
- `button`: Whether to show the debug button in the status bar.
- `timeout`: Time in milliseconds until timeout error when connecting to a TCP debug adapter.
- `log_dap_communications`: Whether to log messages between active debug adapters and Zed
- `format_dap_log_messages`: Whether to format dap messages in when adding them to debug adapter logger

### Stepping granularity

- Description: The Step granularity that the debugger will use
- Default: line
- Setting: debugger.stepping_granularity

**Options**

1. Statement - The step should allow the program to run until the current statement has finished executing.
   The meaning of a statement is determined by the adapter and it may be considered equivalent to a line.
   For example 'for(int i = 0; i < 10; i++)' could be considered to have 3 statements 'int i = 0', 'i < 10', and 'i++'.

```json
{
  "debugger": {
    "stepping_granularity": "statement"
  }
}
```

2. Line - The step should allow the program to run until the current source line has executed.

```json
{
  "debugger": {
    "stepping_granularity": "line"
  }
}
```

3. Instruction - The step should allow one instruction to execute (e.g. one x86 instruction).

```json
{
  "debugger": {
    "stepping_granularity": "instruction"
  }
}
```

### Save Breakpoints

- Description: Whether the breakpoints should be saved across Zed sessions.
- Default: true
- Setting: debugger.save_breakpoints

**Options**

`boolean` values

```json
{
  "debugger": {
    "save_breakpoints": true
  }
}
```

### Button

- Description: Whether the button should be displayed in the debugger toolbar.
- Default: true
- Setting: debugger.show_button

**Options**

`boolean` values

```json
{
  "debugger": {
    "show_button": true
  }
}
```

### Timeout

- Description: Time in milliseconds until timeout error when connecting to a TCP debug adapter.
- Default: 2000ms
- Setting: debugger.timeout

**Options**

`integer` values

```json
{
  "debugger": {
    "timeout": 3000
  }
}
```

### Log Dap Communications

- Description: Whether to log messages between active debug adapters and Zed. (Used for DAP development)
- Default: false
- Setting: debugger.log_dap_communications

**Options**

`boolean` values

```json
{
  "debugger": {
    "log_dap_communications": true
  }
}
```

### Format Dap Log Messages

- Description: Whether to format dap messages in when adding them to debug adapter logger. (Used for DAP development)
- Default: false
- Setting: debugger.format_dap_log_messages

**Options**

`boolean` values

```json
{
  "debugger": {
    "format_dap_log_messages": true
  }
}
```

## Theme

The Debugger supports the following theme options

    /// Color used to accent some of the debuggers elements
    /// Only accent breakpoint & breakpoint related symbols right now

**debugger.accent**: Color used to accent breakpoint & breakpoint related symbols
**editor.debugger_active_line.background**: Background color of active debug line
