# Rust

Rust support is available natively in Zed.

- Tree Sitter: [tree-sitter/tree-sitter-rust](https://github.com/tree-sitter/tree-sitter-rust)
- Language Server: [rust-lang/rust-analyzer](https://github.com/rust-lang/rust-analyzer)

<!--
TBD: Polish Rust Docs. Zed is a good rust editor, good Rust docs make it look like we care about Rust (we do!)
TBD: Users may not know what inlayHints, don't start there.
TBD: Provide explicit examples not just `....`
-->

## Inlay Hints

The following configuration can be used to enable inlay hints for rust:

```json
"inlayHints": {
  "maxLength": null,
  "lifetimeElisionHints": {
  "useParameterNames": true,
    "enable": "skip_trivial"
  },
  "closureReturnTypeHints": {
    "enable": "always"
  }
}
```

to make the language server send back inlay hints when Zed has them enabled in the settings.

Use

```json
"lsp": {
  "rust-analyzer": {
    "initialization_options": {
      ....
    }
  }
}
```

to override these settings.

See [Inlay Hints](https://rust-analyzer.github.io/manual.html#inlay-hints) in the Rust Analyzer Manual for more information.

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

To use a binary in a custom location, add the following to your `settings.json`:

```json
{
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "path": "/Users/example/bin/rust-analyzer",
        "args": []
      }
    }
  }
}
```

To use a binary that is on your `$PATH`, add the following to your `settings.json`:

```json
{
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "path_lookup": true
      }
    }
  }
}
```

## More server configuration

<!--
TBD: Is it possible to specify RUSTFLAGS? https://github.com/zed-industries/zed/issues/14334
-->

Rust-analyzer [manual](https://rust-analyzer.github.io/manual.html) describes various features and configuration options for rust-analyzer language server.
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

Check on save feature is responsible for returning part of the diagnostics based on cargo check output, so turning it off will limit rust-analyzer with its own [diagnostics](https://rust-analyzer.github.io/manual.html#diagnostics).

Consider more `rust-analyzer.cargo.` and `rust-analyzer.check.` and `rust-analyzer.diagnostics.` settings from the manual for more fine-grained configuration.
Here's a snippet for Zed settings.json (the language server will restart automatically after the `lsp.rust-analyzer` section is edited and saved):

```json5
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
```

### Snippets

There's a way get custom completion items from rust-analyzer, that will transform the code according to the snippet body:

```json
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
```
