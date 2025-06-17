# Debugger (Beta)

Zed uses the Debug Adapter Protocol (DAP) to provide debugging functionality across multiple programming languages.
DAP is a standardized protocol that defines how debuggers, editors, and IDEs communicate with each other.
It allows Zed to support various debuggers without needing to implement language-specific debugging logic.
This protocol enables features like setting breakpoints, stepping through code, inspecting variables,
and more, in a consistent manner across different programming languages and runtime environments.

> We currently offer onboarding support for users. We are eager to hear from you if you encounter any issues or have suggestions for improvement for our debugging experience.
> You can schedule a call via [Cal.com](https://cal.com/team/zed-research/debugger)

## Supported Debug Adapters

Zed supports a variety of debug adapters for different programming languages out of the box:

- JavaScript ([vscode-js-debug](https://github.com/microsoft/vscode-js-debug.git)): Enables debugging of Node.js applications, including setting breakpoints, stepping through code, and inspecting variables in JavaScript.

- Python ([debugpy](https://github.com/microsoft/debugpy.git)): Provides debugging capabilities for Python applications, supporting features like remote debugging, multi-threaded debugging, and Django/Flask application debugging.

- LLDB ([CodeLLDB](https://github.com/vadimcn/codelldb.git)): A powerful debugger for C, C++, Objective-C, and Swift, offering low-level debugging features and support for Apple platforms.

- GDB ([GDB](https://sourceware.org/gdb/)): The GNU Debugger, which supports debugging for multiple programming languages including C, C++, Go, and Rust, across various platforms.

- Go ([Delve](https://github.com/go-delve/delve)): Delve, a debugger for the Go programming language, offering both local and remote debugging capabilities with full support for Go's runtime and standard library.

- PHP ([Xdebug](https://xdebug.org/)): Provides debugging and profiling capabilities for PHP applications, including remote debugging and code coverage analysis.

- Ruby ([rdbg](https://github.com/ruby/debug)): Provides debugging for Ruby.

These adapters enable Zed to provide a consistent debugging experience across multiple languages while leveraging the specific features and capabilities of each debugger.

> Is your desired debugger not listed? You can install a [Debug Adapter extension](https://zed.dev/extensions?filter=debug-adapters) to add support for your favorite debugger.
> If that's not enough, you can contribute by creating an extension yourself. Check out our [debugger extensions](extensions/debugger-extensions.md) documentation for more information.

## Getting Started

For basic debugging, you can set up a new configuration by opening the `New Session Modal` either via the `debugger: start` (default: f4) or by clicking the plus icon at the top right of the debug panel.

For more advanced use cases, you can create debug configurations by directly editing the `.zed/debug.json` file in your project root directory.

You can then use the `New Session Modal` to select a configuration and start debugging.

### Launching & Attaching

Zed debugger offers two ways to debug your program; you can either _launch_ a new instance of your program or _attach_ to an existing process.
Which one you choose depends on what you are trying to achieve.

When launching a new instance, Zed (and the underlying debug adapter) can often do a better job at picking up the debug information compared to attaching to an existing process, since it controls the lifetime of a whole program. Running unit tests or a debug build of your application is a good use case for launching.

Compared to launching, attaching to an existing process might seem inferior, but that's far from truth; there are cases where you cannot afford to restart your program, because e.g. the bug is not reproducible outside of a production environment or some other circumstances.

## Configuration

While configuration fields are debug adapter-dependent, most adapters support the following fields:

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

All configuration fields support task variables. See [Tasks Variables](./tasks.md#variables)

### Build tasks

Zed also allows embedding a Zed task in a `build` field that is run before the debugger starts. This is useful for setting up the environment or running any necessary setup steps before the debugger starts.

```json
[
  {
    "label": "Build Binary",
    "adapter": "CodeLLDB",
    "program": "path_to_program",
    "request": "launch",
    "build": {
      "command": "make",
      "args": ["build", "-j8"]
    }
  }
]
```

Build tasks can also refer to the existing tasks by unsubstituted label:

```json
[
  {
    "label": "Build Binary",
    "adapter": "CodeLLDB",
    "program": "path_to_program",
    "request": "launch",
    "build": "my build task" // Or "my build task for $ZED_FILE"
  }
]
```

### Automatic scenario creation

Given a Zed task, Zed can automatically create a scenario for you. Automatic scenario creation also powers our scenario creation from gutter.
Automatic scenario creation is currently supported for Rust, Go and Python. Javascript/TypeScript support being worked on.

### Example Configurations

#### Go

```json
[
  {
    "label": "Go (Delve)",
    "adapter": "Delve",
    "program": "$ZED_FILE",
    "request": "launch",
    "mode": "debug"
  }
]
```

#### JavaScript

##### Debug Active File

```json
[
  {
    "label": "Debug with node",
    "adapter": "JavaScript",
    "program": "$ZED_FILE",
    "request": "launch",
    "console": "integratedTerminal",
    "type": "pwa-node"
  }
]
```

##### Attach debugger to a server running in web browser (`npx serve`)

Given an externally-ran web server (e.g. with `npx serve` or `npx live-server`) one can attach to it and open it with a browser.

```json
[
  {
    "label": "Inspect ",
    "adapter": "JavaScript",
    "type": "pwa-chrome",
    "request": "launch",
    "url": "http://localhost:5500", // Fill your URL here.
    "program": "$ZED_FILE",
    "webRoot": "${ZED_WORKTREE_ROOT}"
  }
]
```

#### Python

##### Debug Active File

```json
[
  {
    "label": "Python Active File",
    "adapter": "Debugpy",
    "program": "$ZED_FILE",
    "request": "launch"
  }
]
```

##### Flask App

For a common Flask Application with a file structure similar to the following:

```
.venv/
app/
  init.py
  main.py
  routes.py
templates/
  index.html
static/
  style.css
requirements.txt
```

the following configuration can be used:

```json
[
  {
    "label": "Python: Flask",
    "adapter": "Debugpy",
    "request": "launch",
    "module": "app",
    "cwd": "$ZED_WORKTREE_ROOT",
    "env": {
      "FLASK_APP": "app",
      "FLASK_DEBUG": "1"
    },
    "args": [
      "run",
      "--reload", // Enables Flask reloader that watches for file changes
      "--debugger" // Enables Flask debugger
    ],
    "autoReload": {
      "enable": true
    },
    "jinja": true,
    "justMyCode": true
  }
]
```

#### Rust/C++/C

##### Using pre-built binary

```json
[
  {
    "label": "Debug native binary",
    "program": "$ZED_WORKTREE_ROOT/build/binary",
    "request": "launch",
    "adapter": "CodeLLDB" // GDB is available on non arm macs as well as linux
  }
]
```

##### Build binary then debug

```json
[
  {
    "label": "Build & Debug native binary",
    "build": {
      "command": "cargo",
      "args": ["build"]
    },
    "program": "$ZED_WORKTREE_ROOT/target/debug/binary",
    "request": "launch",
    "adapter": "CodeLLDB" // GDB is available on non arm macs as well as linux
  }
]
```

#### TypeScript

##### Attach debugger to a server running in web browser (`npx serve`)

Given an externally-ran web server (e.g. with `npx serve` or `npx live-server`) one can attach to it and open it with a browser.

```json
[
  {
    "label": "Launch Chromee (TypeScript)",
    "adapter": "JavaScript",
    "type": "pwa-chrome",
    "request": "launch",
    "url": "http://localhost:5500",
    "program": "$ZED_FILE",
    "webRoot": "${ZED_WORKTREE_ROOT}",
    "sourceMaps": true,
    "build": {
      "command": "npx",
      "args": ["tsc"]
    }
  }
]
```

#### Go

Zed uses [delve](https://github.com/go-delve/delve?tab=readme-ov-file) to debug Go applications. Zed will automatically create debug scenarios for `func main` in your main packages, and also
for any tests, so you can use the Play button in the gutter to debug these without configuration.

##### Debug Go Packages

To debug a specific package, you can do so by setting the Delve mode to "debug". In this case "program" should be set to the package name.

```json
[
  {
    "label": "Run server",
    "adapter": "Delve",
    "request": "launch",
    "mode": "debug",
    // For Delve, the program is the package name
    "program": "./cmd/server"
    // "args": [],
    // "buildFlags": [],
  }
]
```

##### Debug Go Tests

To debug the tests for a package, set the Delve mode to "test". The "program" is still the package name, and you can use the "buildFlags" to do things like set tags, and the "args" to set args on the test binary. (See `go help testflags` for more information on doing that).

```json
[
  {
    "label": "Run integration tests",
    "adapter": "Delve",
    "request": "launch",
    "mode": "test",
    "program": ".",
    "buildFlags": ["-tags", "integration"]
    // To filter down to just the test your cursor is in:
    // "args": ["-test.run", "$ZED_SYMBOL"]
  }
]
```

##### Build and debug separately

If you need to build your application with a specific command, you can use the "exec" mode of Delve. In this case "program" should point to an executable,
and the "build" command should build that.

```json
{
  "label": "Debug Prebuilt Unit Tests",
  "adapter": "Delve",
  "request": "launch",
  "mode": "exec",
  "program": "${ZED_WORKTREE_ROOT}/__debug_unit",
  "args": ["-test.v", "-test.run=${ZED_SYMBOL}"],
  "build": {
    "command": "go",
    "args": [
      "test",
      "-c",
      "-tags",
      "unit",
      "-gcflags\"all=-N -l\"",
      "-o",
      "__debug_unit",
      "./pkg/..."
    ]
  }
}
```

##### Attaching to an existing instance of Delve

You might find yourself needing to connect to an existing instance of Delve that's not necessarily running on your machine; in such case, you can use `tcp_arguments` to instrument Zed's connection to Delve.

````
{
  "adapter": "Delve",
  "label": "Connect to a running Delve instance",
  "program": "/Users/zed/Projects/language_repositories/golang/hello/hello",
  "cwd": "/Users/zed/Projects/language_repositories/golang/hello",
  "args": [],
  "env": {},
  "request": "launch",
  "mode": "exec",
  "stopOnEntry": false,
  "tcp_connection": { "host": "123.456.789.012", "port": 53412 }
}
```

In such case Zed won't spawn a new instance of Delve, as it opts to use an existing one. The consequence of this is that *there will be no terminal* in Zed; you have to interact with the Delve instance directly, as it handles stdin/stdout of the debuggee.

### Ruby

To run a ruby task in the debugger, you will need to configure it in the `.zed/debug.json` file in your project. We don't yet have automatic detection of ruby tasks, nor do we support connecting to an existing process.

The configuration should look like this:

```json
{
  {
    "adapter": "Ruby",
    "label": "Run CLI",
    "script": "cli.rb"
    // If you want to customize how the script is run (for example using bundle exec)
    // use "command" instead.
    // "command": "bundle exec cli.rb"
    //
    // "args": []
    // "env": {}
    // "cwd": ""
  }
}
````

## Breakpoints

To set a breakpoint, simply click next to the line number in the editor gutter.
Breakpoints can be tweaked depending on your needs; to access additional options of a given breakpoint, right-click on the breakpoint icon in the gutter and select the desired option.
At present, you can:

- Add a log to a breakpoint, which will output a log message whenever that breakpoint is hit.
- Make the breakpoint conditional, which will only stop at the breakpoint when the condition is met. The syntax for conditions is adapter-specific.
- Add a hit count to a breakpoint, which will only stop at the breakpoint after it's hit a certain number of times.
- Disable a breakpoint, which will prevent it from being hit while leaving it visible in the gutter.

Some debug adapters (e.g. CodeLLDB and JavaScript) will also _verify_ whether your breakpoints can be hit; breakpoints that cannot be hit are surfaced more prominently in the UI.

All breakpoints enabled for a given project are also listed in "Breakpoints" item in your debugging session UI. From "Breakpoints" item in your UI you can also manage exception breakpoints.
The debug adapter will then stop whenever an exception of a given kind occurs. Which exception types are supported depends on the debug adapter.

## Settings

- `dock`: Determines the position of the debug panel in the UI.
- `stepping_granularity`: Determines the stepping granularity.
- `save_breakpoints`: Whether the breakpoints should be reused across Zed sessions.
- `button`: Whether to show the debug button in the status bar.
- `timeout`: Time in milliseconds until timeout error when connecting to a TCP debug adapter.
- `log_dap_communications`: Whether to log messages between active debug adapters and Zed.
- `format_dap_log_messages`: Whether to format DAP messages when adding them to the debug adapter logger.

### Dock

- Description: The position of the debug panel in the UI.
- Default: `bottom`
- Setting: debugger.dock

**Options**

1. `left` - The debug panel will be docked to the left side of the UI.
2. `right` - The debug panel will be docked to the right side of the UI.
3. `bottom` - The debug panel will be docked to the bottom of the UI.

```json
"debugger": {
  "dock": "bottom"
},
```

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
- Default: 2000
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

- Description: Whether to format DAP messages when adding them to the debug adapter logger. (Used for DAP development)
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

**debugger.accent**: Color used to accent breakpoint & breakpoint-related symbols
**editor.debugger_active_line.background**: Background color of active debug line
