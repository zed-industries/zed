# Swift

Swift language support in Zed is provided by the community-maintained [Swift extension](https://github.com/zed-extensions/swift).
Report issues to: [https://github.com/zed-extensions/swift/issues](https://github.com/zed-extensions/swift/issues)

- Tree-sitter: [alex-pinkus/tree-sitter-swift](https://github.com/alex-pinkus/tree-sitter-swift)
- Language Server: [swiftlang/sourcekit-lsp](https://github.com/swiftlang/sourcekit-lsp)
- Debug Adapter: [`lldb-dap`](https://github.com/swiftlang/llvm-project/blob/next/lldb/tools/lldb-dap/README.md)

## Language Server Configuration

You can modify the behavior of SourceKit LSP by creating a `.sourcekit-lsp/config.json` under your home directory or in your project root. See [SourceKit-LSP configuration file](https://github.com/swiftlang/sourcekit-lsp/blob/main/Documentation/Configuration%20File.md) for complete documentation.

## Debugging

The Swift extension provides a debug adapter for debugging Swift code.
Zed's name for the adapter (in the UI and `debug.json`) is `Swift`, and under the hood it uses [`lldb-dap`](https://github.com/swiftlang/llvm-project/blob/next/lldb/tools/lldb-dap/README.md), as provided by the Swift toolchain.
The extension tries to find an `lldb-dap` binary using `swiftly`, using `xcrun`, and by searching `$PATH`, in that order of preference.
The extension doesn't attempt to download `lldb-dap` if it's not found.

### Examples

#### Build and debug a Swift binary

```json [debug]
[
  {
    "label": "Debug Swift",
    "build": {
      "command": "swift",
      "args": ["build"]
    },
    "program": "$ZED_WORKTREE_ROOT/swift-app/.build/arm64-apple-macosx/debug/swift-app",
    "request": "launch",
    "adapter": "Swift"
  }
]
```
