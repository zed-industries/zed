# Rust

Rust support is available natively in Zed.

- Tree-sitter: [tree-sitter/tree-sitter-rust](https://github.com/tree-sitter/tree-sitter-rust)
- Language Server: [rust-lang/rust-analyzer](https://github.com/rust-lang/rust-analyzer)
- Debug Adapter: [CodeLLDB](https://github.com/vadimcn/codelldb) (primary), [GDB](https://sourceware.org/gdb/) (secondary, not available on Apple silicon)

<!--
TBD: Polish Rust Docs. Zed is a good rust editor, good Rust docs make it look like we care about Rust (we do!)
TBD: Users may not know what inlayHints, don't start there.
TBD: Provide explicit examples not just `....`
-->

## Inlay Hints

The following configuration can be used to change the inlay hint settings for `rust-analyzer` in Rust:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "inlayHints": {
          "maxLength": null,
          "lifetimeElisionHints": {
            "enable": "skip_trivial",
            "useParameterNames": true
          },
          "closureReturnTypeHints": {
            "enable": "always"
          }
        }
      }
    }
  }
}
```

See [Inlay Hints](https://rust-analyzer.github.io/book/features.html#inlay-hints) in the Rust Analyzer Manual for more information.

## Target directory

The `rust-analyzer` target directory can be set in `initialization_options`:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "rust": {
          "analyzerTargetDir": true
        }
      }
    }
  }
}
```

A `true` setting will set the target directory to `target/rust-analyzer`. You can set a custom directory with a string like `"target/analyzer"` instead of `true`.

## Binary

You can configure which `rust-analyzer` binary Zed should use.

By default, Zed will try to find a `rust-analyzer` in your `$PATH` and try to use that. If that binary successfully executes `rust-analyzer --help`, it's used. Otherwise, Zed will fall back to installing its own `rust-analyzer` version and using that.

If you want to disable Zed looking for a `rust-analyzer` binary, you can set `ignore_system_version` to `true` in your `settings.json`:

```json
{
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "ignore_system_version": true
      }
    }
  }
}
```

If you want to use a binary in a custom location, you can specify a `path` and optional `arguments`:

```json
{
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "path": "/Users/example/bin/rust-analyzer",
        "arguments": []
      }
    }
  }
}
```

This `"path"` has to be an absolute path.

## Alternate Targets

If want rust-analyzer to provide diagnostics for a target other than you current platform (e.g. for windows when running on macOS) you can use the following Zed lsp settings:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "target": "x86_64-pc-windows-msvc"
        }
      }
    }
  }
}
```

If you are using `rustup` and you can find a list of available target triples (`aarch64-apple-darwin`, `x86_64-unknown-linux-gnu`, etc) by running:

```sh
rustup target list --installed
```

## LSP tasks

Zed provides tasks using tree-sitter, but rust-analyzer has an LSP extension method for querying file-related tasks via LSP.
This is enabled by default and can be configured as

```json
"lsp": {
  "rust-analyzer": {
    "enable_lsp_tasks": true,
  }
}
```

## Manual Cargo Diagnostics fetch

By default, rust-analyzer has `checkOnSave: true` enabled, which causes every buffer save to trigger a `cargo check --workspace --all-targets` command.
If disabled with `checkOnSave: false` (see the example of the server configuration json above), it's still possible to fetch the diagnostics manually, with the `editor: run/clear/cancel flycheck` commands in Rust files to refresh cargo diagnostics; the project diagnostics editor will also refresh cargo diagnostics with `editor: run flycheck` command when the setting is enabled.

## More server configuration

<!--
TBD: Is it possible to specify RUSTFLAGS? https://github.com/zed-industries/zed/issues/14334
-->

Rust-analyzer [manual](https://rust-analyzer.github.io/book/) describes various features and configuration options for rust-analyzer language server.
Rust-analyzer in Zed runs with the default parameters.

### Large projects and performance

One of the main caveats that might cause extensive resource usage on large projects, is the combination of the following features:

```
rust-analyzer.checkOnSave (default: true)
    Run the check command for diagnostics on save.
```

```
rust-analyzer.check.workspace (default: true)
    Whether --workspace should be passed to cargo check. If false, -p <package> will be passed instead.
```

```
rust-analyzer.cargo.allTargets (default: true)
    Pass --all-targets to cargo invocation
```

Which would mean that every time Zed saves, a `cargo check --workspace --all-targets` command is run, checking the entire project (workspace), lib, doc, test, bin, bench and [other targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html).

While that works fine on small projects, it does not scale well.

The alternatives would be to use [tasks](../tasks.md), as Zed already provides a `cargo check --workspace --all-targets` task and the ability to cmd/ctrl-click on the terminal output to navigate to the error, and limit or turn off the check on save feature entirely.

Check on save feature is responsible for returning part of the diagnostics based on cargo check output, so turning it off will limit rust-analyzer with its own [diagnostics](https://rust-analyzer.github.io/book/diagnostics.html).

Consider more `rust-analyzer.cargo.` and `rust-analyzer.check.` and `rust-analyzer.diagnostics.` settings from the manual for more fine-grained configuration.
Here's a snippet for Zed settings.json (the language server will restart automatically after the `lsp.rust-analyzer` section is edited and saved):

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        // get more cargo-less diagnostics from rust-analyzer,
        // which might include false-positives (those can be turned off by their names)
        "diagnostics": {
          "experimental": {
            "enable": true
          }
        },
        // To disable the checking entirely
        // (ignores all cargo and check settings below)
        "checkOnSave": false,
        // To check the `lib` target only.
        "cargo": {
          "allTargets": false
        },
        // Use `-p` instead of `--workspace` for cargo check
        "check": {
          "workspace": false
        }
      }
    }
  }
}
```

### Multi-project workspaces

If you want rust-analyzer to analyze multiple Rust projects in the same folder that are not listed in `[members]` in the Cargo workspace,
you can list them in `linkedProjects` in the local project settings:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "linkedProjects": ["./path/to/a/Cargo.toml", "./path/to/b/Cargo.toml"]
      }
    }
  }
}
```

### Snippets

There's a way get custom completion items from rust-analyzer, that will transform the code according to the snippet body:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "completion": {
          "snippets": {
            "custom": {
              "Arc::new": {
                "postfix": "arc",
                "body": ["Arc::new(${receiver})"],
                "requires": "std::sync::Arc",
                "scope": "expr"
              },
              "Some": {
                "postfix": "some",
                "body": ["Some(${receiver})"],
                "scope": "expr"
              },
              "Ok": {
                "postfix": "ok",
                "body": ["Ok(${receiver})"],
                "scope": "expr"
              },
              "Rc::new": {
                "postfix": "rc",
                "body": ["Rc::new(${receiver})"],
                "requires": "std::rc::Rc",
                "scope": "expr"
              },
              "Box::pin": {
                "postfix": "boxpin",
                "body": ["Box::pin(${receiver})"],
                "requires": "std::boxed::Box",
                "scope": "expr"
              },
              "vec!": {
                "postfix": "vec",
                "body": ["vec![${receiver}]"],
                "description": "vec![]",
                "scope": "expr"
              }
            }
          }
        }
      }
    }
  }
}
```

## Debugging

Zed supports debugging Rust binaries and tests out of the box. Run {#action debugger::Start} ({#kb debugger::Start}) to launch one of these preconfigured debug tasks.

For more control, you can add debug configurations to `.zed/debug.json`. See the examples below.

### Build binary then debug

```json
[
  {
    "label": "Build & Debug native binary",
    "build": {
      "command": "cargo",
      "args": ["build"]
    },
    "program": "$ZED_WORKTREE_ROOT/target/debug/binary",
    // sourceLanguages is required for CodeLLDB (not GDB) when using Rust
    "sourceLanguages": ["rust"],
    "request": "launch",
    "adapter": "CodeLLDB"
  }
]
```

### Automatically locate a debug target based on build command

When you use `cargo build` or `cargo test` as the build command, Zed can infer the path to the output binary.

```json
[
  {
    "label": "Build & Debug native binary",
    "adapter": "CodeLLDB",
    "build": {
      "command": "cargo",
      "args": ["build"]
    },
    // sourceLanguages is required for CodeLLDB (not GDB) when using Rust
    "sourceLanguages": ["rust"]
  }
]
```
