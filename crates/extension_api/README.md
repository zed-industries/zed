# The Zed Rust Extension API

This crate lets you write extensions for Zed in Rust.

## Extension Manifest

You'll need an `extension.toml` file at the root of your extension directory, with the following structure:

```toml
id = "my-extension"
name = "My Extension"
description = "..."
version = "0.0.1"
schema_version = 1
authors = ["Your Name <you@example.com>"]
repository = "https://github.com/your/extension-repository"
```

## Cargo metadata

Zed extensions are packaged as WebAssembly files. In your Cargo.toml, you'll
need to set your `crate-type` accordingly:

```toml
[dependencies]
zed_extension_api = "0.5.0"

[lib]
crate-type = ["cdylib"]
```

## Implementing an Extension

To define your extension, create a type that implements the `Extension` trait, and register it.

```rust
use zed_extension_api as zed;

struct MyExtension {
    // ... state
}

impl zed::Extension for MyExtension {
    // ...
}

zed::register_extension!(MyExtension);
```

## Testing your extension

To run your extension in Zed as you're developing it:

- Open the extensions view using the `zed: extensions` action in the command palette.
- Click the `Install Dev Extension` button in the top right
- Choose the path to your extension directory.

## Compatible Zed versions

Extensions created using newer versions of the Zed extension API won't be compatible with older versions of Zed.

Here is the compatibility of the `zed_extension_api` with versions of Zed:

| Zed version | `zed_extension_api` version |
| ----------- | --------------------------- |
| `0.186.x`   | `0.0.1` - `0.5.0`           |
| `0.184.x`   | `0.0.1` - `0.4.0`           |
| `0.178.x`   | `0.0.1` - `0.3.0`           |
| `0.162.x`   | `0.0.1` - `0.2.0`           |
| `0.149.x`   | `0.0.1` - `0.1.0`           |
| `0.131.x`   | `0.0.1` - `0.0.6`           |
| `0.130.x`   | `0.0.1` - `0.0.5`           |
| `0.129.x`   | `0.0.1` - `0.0.4`           |
| `0.128.x`   | `0.0.1`                     |
