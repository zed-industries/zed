# C++

C++ support is available natively in Zed.

- Tree-sitter: [tree-sitter/tree-sitter-cpp](https://github.com/tree-sitter/tree-sitter-cpp)
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

## Formatting

By default Zed will use the `clangd` language server for formatting C++ code. The Clangd is the same as the `clang-format` CLI tool. To configure this you can add a `.clang-format` file. For example:

```yaml
---
BasedOnStyle: LLVM
IndentWidth: 4
---
Language: Cpp
# Force pointers to the type for C++.
DerivePointerAlignment: false
PointerAlignment: Left
---
```

See [Clang-Format Style Options](https://clang.llvm.org/docs/ClangFormatStyleOptions.html) for a complete list of options.

You can trigger formatting via {#kb editor::Format} or the `editor: format` action from the command palette or by adding `format_on_save` to your Zed settings:

```json
  "languages": {
    "C++": {
      "format_on_save": "on",
      "tab_size": 2
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

## Compile Commands

For some projects Clangd requires a `compile_commands.json` file to properly analyze your project. This file contains the compilation database that tells clangd how your project should be built.

### CMake Compile Commands

With CMake, you can generate `compile_commands.json` automatically by adding the following line to your `CMakeLists.txt`:

```cmake
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
```

After building your project, CMake will generate the `compile_commands.json` file in the build directory and clangd will automatically pick it up.
