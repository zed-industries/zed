# Go

Go support is available natively in Zed.

- Tree-sitter: [tree-sitter/tree-sitter-go](https://github.com/tree-sitter/tree-sitter-go)
- Language Server: [golang/tools/tree/master/gopls](https://github.com/golang/tools/tree/master/gopls)
- Debug Adapter: [delve](https://github.com/go-delve/delve)

## Setup

We recommend installing gopls via go's package manager and not via Homebrew or your Linux distribution's package manager.

1. Make sure you have uninstalled any version of gopls you have installed via your package manager:

```sh
# MacOS homebrew
brew remove gopls
# Ubuntu
sudo apt-get remove gopls
sudo snap remove gopls
# Arch
sudo pacman -R gopls
```

2. Install/Update `gopls` to the latest version using the go module tool:

```sh
go install golang.org/x/tools/gopls@latest
```

3. Ensure that `gopls` is in your path:

```sh
which gopls
gopls version
```

If `gopls` is not found you will likely need to add `export PATH="$PATH:$HOME/go/bin"` to your `.zshrc` / `.bash_profile`

## Inlay Hints

Zed sets the following initialization options for inlay hints:

```json [settings]
"hints": {
    "assignVariableTypes": true,
    "compositeLiteralFields": true,
    "compositeLiteralTypes": true,
    "constantValues": true,
    "functionTypeParameters": true,
    "parameterNames": true,
    "rangeVariableTypes": true
}
```

to make the language server send back inlay hints when Zed has them enabled in the settings.

Use

```json [settings]
"lsp": {
    "gopls": {
        "initialization_options": {
            "hints": {
                // ....
            }
        }
    }
}
```

to override these settings.

See [gopls inlayHints documentation](https://github.com/golang/tools/blob/master/gopls/doc/inlayHints.md) for more information.

## Debugging

Zed supports zero-configuration debugging of Go tests and entry points (`func main`). Run {#action debugger::Start} ({#kb debugger::Start}) to see a contextual list of these preconfigured debug tasks.

For more control, you can add debug configurations to `.zed/debug.json`. See below for examples.

### Debug Go Packages

To debug a specific package, you can do so by setting the Delve mode to "debug". In this case "program" should be set to the package name.

```json [debug]
[
  {
    "label": "Go (Delve)",
    "adapter": "Delve",
    "program": "$ZED_FILE",
    "request": "launch",
    "mode": "debug"
  },
  {
    "label": "Run server",
    "adapter": "Delve",
    "request": "launch",
    "mode": "debug",
    // For Delve, the program can be a package name
    "program": "./cmd/server"
    // "args": [],
    // "buildFlags": [],
  }
]
```

### Debug Go Tests

To debug the tests for a package, set the Delve mode to "test".
The "program" is still the package name, and you can use the "buildFlags" to do things like set tags, and the "args" to set args on the test binary. (See `go help testflags` for more information on doing that).

```json [debug]
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

### Build and debug separately

If you need to build your application with a specific command, you can use the "exec" mode of Delve. In this case "program" should point to an executable,
and the "build" command should build that.

```json [debug]
[
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
]
```

### Attaching to an existing instance of Delve

You might find yourself needing to connect to an existing instance of Delve that's not necessarily running on your machine; in such case, you can use `tcp_arguments` to instrument Zed's connection to Delve.

```json [debug]
[
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
    "tcp_connection": { "host": "127.0.0.1", "port": 53412 }
  }
]
```

In such case Zed won't spawn a new instance of Delve, as it opts to use an existing one. The consequence of this is that _there will be no terminal_ in Zed; you have to interact with the Delve instance directly, as it handles stdin/stdout of the debuggee.

## Go Mod

- Tree-sitter: [camdencheek/tree-sitter-go-mod](https://github.com/camdencheek/tree-sitter-go-mod)
- Language Server: N/A

## Go Sum

- Tree-sitter: [amaanq/tree-sitter-go-sum](https://github.com/amaanq/tree-sitter-go-sum)
- Language Server: N/A

## Go Work

- Tree-sitter:
  [tree-sitter-go-work](https://github.com/d1y/tree-sitter-go-work)
- Language Server: N/A
