# C++

C++ support is available natively in Zed.

- Tree Sitter: [tree-sitter/tree-sitter-cpp](https://github.com/tree-sitter/tree-sitter-cpp)
- Language Server: [clangd/clangd](https://github.com/clangd/clangd)

## Binary

You can configure which `clangd` binary Zed should use.

To use a binary in a custom location, add the following to your `settings.json`:

```json
{
  "lsp": {
    "clangd": {
      "binary": {
        "path": "/path/to/clangd",
        "arguments": []
      }
    }
  }
}
```

If you want to disable Zed looking for a `clangd` binary, you can set `ignore_system_version` to `true`:

```json
{
  "lsp": {
    "clangd": {
      "binary": {
        "ignore_system_version": true
      }
    }
  }
}
```

## Arguments

You can pass any number of arguments to clangd. To see a full set of available options, run `clangd --help` from the command line. For example with `--function-arg-placeholders=0` completions contain only parentheses for function calls, while the default (`--function-arg-placeholders=1`) completions also contain placeholders for method parameters.

```json
{
  "lsp": {
    "clangd": {
      "binary": {
        "path": "/path/to/clangd",
        "arguments": ["--function-arg-placeholders=0"]
      }
    }
  }
}
```

## More server configuration

In the root of your project, it is generally common to create a `.clangd` file to set extra configuration.

```text
CompileFlags:
  Add:
    - "--include-directory=/path/to/include"
Diagnostics:
  MissingIncludes: Strict
  UnusedIncludes: Strict
```

For more advanced usage of clangd configuration file, take a look into their [official page](https://clangd.llvm.org/config.html).
