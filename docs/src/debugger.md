# Debugger (Beta)

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

- Ruby (rdbg): Provides debugging capabilities for Ruby applications

These adapters enable Zed to provide a consistent debugging experience across multiple languages while leveraging the specific features and capabilities of each debugger.

## Getting Started

For basic debugging you can set up a new configuration by opening the `New Session Modal` either via the `debugger: start` (default: f4) or clicking the plus icon at the top right of the debug panel.

For more advanced use cases you can create debug configurations by directly editing the `.zed/debug.json` file in your project root directory.

You can then use the `New Session Modal` to select a configuration then start debugging.

### Configuration

While configuration fields are debug adapter dependent, most adapters support the following fields.

```json
[
  {
    // The label for the debug configuration and used to identify the debug session inside the debug panel & new session modal
    "label": "Example Start debugger config",
    // The debug adapter that Zed should use to debug the program
    "adapter": "Example adapter name",
    // Request:
    //  - launch: Zed will launch the program if specified or shows a debug terminal with the right configuration
    //  - attach: Zed will attach to a running program to debug it or when the process_id is not specified we will show a process picker (only supported for node currently)
    "request": "launch",
    // program: The program that you want to debug
    // This field supports path resolution with ~ or . symbols
    "program": "path_to_program",
    // cwd: defaults to the current working directory of your project ($ZED_WORKTREE_ROOT)
    "cwd": "$ZED_WORKTREE_ROOT"
  }
]
```

#### Task Variables

All configuration fields support task variables. See [Tasks](./tasks.md)

## Breakpoints

Zed currently supports these types of breakpoints

- Standard Breakpoints: Stop at the breakpoint when it's hit
- Log Breakpoints: Output a log message instead of stopping at the breakpoint when it's hit
- Conditional Breakpoints: Stop at the breakpoint when it's hit if the condition is met
- Hit Breakpoints: Stop at the breakpoint when it's hit a certain number of times

Standard breakpoints can be toggled by left clicking on the editor gutter or using the Toggle Breakpoint action. Right clicking on a breakpoint or on a code runner symbol brings up the breakpoint context menu. This has options for toggling breakpoints and editing log breakpoints.

Other kinds of breakpoints can be toggled/edited by right clicking on the breakpoint icon in the gutter and selecting the desired option.

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

The Debugger supports the following theme options:

    /// Color used to accent some of the debugger's elements
    /// Only accents breakpoint & breakpoint related symbols right now

**debugger.accent**: Color used to accent breakpoint & breakpoint related symbols
**editor.debugger_active_line.background**: Background color of active debug line
