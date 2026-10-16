---
title: Extension Capabilities
description: "Extension Capabilities for Zed extensions."
---

# Extension Capabilities

The operations that Zed extensions are able to perform are governed by a capability system.

## Restricting capabilities

As a user, you have the option of restricting the capabilities that are granted to extensions.

This is controlled via the `granted_extension_capabilities` setting.

Restricting or removing a capability will cause an error to be returned when an extension attempts to call the corresponding extension API without sufficient capabilities.

For example, to restrict downloads to files from GitHub, set `host` for the `download_file` capability:

```diff
{
  "granted_extension_capabilities": [
    { "kind": "process:exec", "command": "*", "args": ["**"] },
-   { "kind": "download_file", "host": "*", "path": ["**"] },
+   { "kind": "download_file", "host": "github.com", "path": ["**"] },
    { "kind": "npm:install", "package": "*" }
  ]
}
```

If you don't want extensions to be able to perform _any_ capabilities, you can remove all granted capabilities:

```json
{
  "granted_extension_capabilities": []
}
```

> Note that this will likely make many extensions non-functional, at least in their default configuration.

## Capabilities

### `process:exec`

The `process:exec` capability grants extensions the ability to invoke commands using [`zed_extension_api::process::Command`](https://docs.rs/zed_extension_api/latest/zed_extension_api/process/struct.Command.html).

#### Examples

To allow any command to be executed with any arguments:

```toml
{ kind = "process:exec", command = "*", args = ["**"] }
```

To allow a specific command (e.g., `gem`) to be executed with any arguments:

```toml
{ kind = "process:exec", command = "gem", args = ["**"] }
```

### `download_file`

The `download_file` capability grants extensions the ability to download files using [`zed_extension_api::download_file`](https://docs.rs/zed_extension_api/latest/zed_extension_api/fn.download_file.html).

#### Examples

To allow any file to be downloaded:

```toml
{ kind = "download_file", host = "*", path = ["**"] }
```

To allow any file to be downloaded from `github.com`:

```toml
{ kind = "download_file", host = "github.com", path = ["**"] }
```

To allow any file to be downloaded from a specific GitHub repository:

```toml
{ kind = "download_file", host = "github.com", path = ["zed-industries", "zed", "**"] }
```

### `npm:install`

The `npm:install` capability grants extensions the ability to install npm packages using [`zed_extension_api::npm_install_package`](https://docs.rs/zed_extension_api/latest/zed_extension_api/fn.npm_install_package.html).

#### Examples

To allow any npm package to be installed:

```toml
{ kind = "npm:install", package = "*" }
```

To allow a specific npm package (e.g., `typescript`) to be installed:

```toml
{ kind = "npm:install", package = "typescript" }
```

### `network:tcp-local`

The `network:tcp-local` capability permits an extension to open a TCP connection to an explicit loopback host. It is intended for tools such as language REPLs that publish a loopback port. Remote hosts are deliberately not supported.

The extension must declare the same endpoint in its `extension.toml`, and the user must grant it in their settings.

#### Example

To permit an nREPL server listening locally on port 7888:

```toml
{ kind = "network:tcp-local", host = "localhost", port = 7888 }
```

Supported hosts are `localhost`, `127.0.0.1`, and `::1`. When `port` is present it must match exactly; omit it for a service, such as nREPL, that chooses a port dynamically.
