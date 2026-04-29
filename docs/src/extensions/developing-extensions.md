---
title: Developing Extensions
description: "Create Zed extensions: languages, themes, debuggers, and more."
---

# Developing Extensions {#developing-extensions}

Zed extensions are Git repositories containing an `extension.toml` manifest. They can provide languages, themes, debuggers, snippets, and MCP servers.

## Extension Features {#extension-features}

Extensions can provide:

- [Languages](./languages.md)
- [Debuggers](./debugger-extensions.md)
- [Themes](./themes.md)
- [Icon Themes](./icon-themes.md)
- [Snippets](./snippets.md)
- [MCP Servers](./mcp-extensions.md)

## Developing an Extension Locally

Before starting to develop an extension for Zed, be sure to [install Rust via rustup](https://www.rust-lang.org/tools/install).

> Rust must be installed via rustup. If you have Rust installed via homebrew or otherwise, installing dev extensions will not work.

When developing an extension, you can use it in Zed without needing to publish it by installing it as a _dev extension_.

From the extensions page, click the `Install Dev Extension` button (or the {#action zed::InstallDevExtension} action) and select the directory containing your extension.

If you need to troubleshoot, check Zed.log ({#action zed::OpenLog}) for additional output. For debug output, close and relaunch Zed from the command line with `zed --foreground`, which shows more verbose INFO-level logs.

If you already have the published version of the extension installed, the published version will be uninstalled prior to the installation of the dev extension. After successful installation, the `Extensions` page will indicate that the upstream extension is "Overridden by dev extension".

## Directory Structure of a Zed Extension

A Zed extension is a Git repository that contains an `extension.toml`. This file must contain some
basic information about the extension:

```toml
id = "my-extension"
name = "My extension"
version = "0.0.1"
schema_version = 1
authors = ["Your Name <you@example.com>"]
description = "Example extension"
repository = "https://github.com/your-name/my-zed-extension"
```

In addition to this, there are several other optional files and directories that can be used to add functionality to a Zed extension. An example directory structure of an extension that provides all capabilities is as follows:

```
my-extension/
  extension.toml
  Cargo.toml
  src/
    lib.rs
  languages/
    my-language/
      config.toml
      highlights.scm
  themes/
    my-theme.json
  snippets/
    snippets.json
    rust.json
```

## WebAssembly

Procedural parts of extensions are written in Rust and compiled to WebAssembly. To develop an extension that includes custom code, include a `Cargo.toml` like this:

```toml
[package]
name = "my-extension"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.1.0"
```

Use the latest version of the [`zed_extension_api`](https://crates.io/crates/zed_extension_api) available on crates.io. Make sure it's still [compatible with Zed versions](https://github.com/zed-industries/zed/blob/main/crates/extension_api#compatible-zed-versions) you want to support.

In the `src/lib.rs` file in your Rust crate you will need to define a struct for your extension and implement the `Extension` trait, as well as use the `register_extension!` macro to register your extension:

```rs
use zed_extension_api as zed;

struct MyExtension {
    // ... state
}

impl zed::Extension for MyExtension {
    // ...
}

zed::register_extension!(MyExtension);
```

> `stdout`/`stderr` is forwarded directly to the Zed process. In order to see `println!`/`dbg!` output from your extension, you can start Zed in your terminal with a `--foreground` flag.

## Forking and cloning the repo

1. Fork the repo

> **Note:** It is very helpful if you fork the `zed-industries/extensions` repo to a personal GitHub account instead of a GitHub organization, as this allows Zed staff to push any needed changes to your PR to expedite the publishing process.

2. Clone the repo to your local machine

```sh
# Substitute the url of your fork here:
# git clone https://github.com/zed-industries/extensions
cd extensions
git submodule init
git submodule update
```

## Extension License Requirements

As of October 1st, 2025, extension repositories must include a license.
The following licenses are accepted:

- [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [BSD 2-Clause](https://opensource.org/license/bsd-2-clause)
- [BSD 3-Clause](https://opensource.org/license/bsd-3-clause)
- [CC BY 4.0](https://creativecommons.org/licenses/by/4.0)
- [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html)
- [GNU LGPLv3](https://www.gnu.org/licenses/lgpl-3.0.en.html)
- [MIT](https://opensource.org/license/mit)
- [Unlicense](https://unlicense.org)
- [zlib](https://opensource.org/license/zlib)

This allows us to distribute the resulting binary produced from your extension code to our users.
Without a valid license, the pull request to add or update your extension in the following steps will fail CI.

Your license file should be at the root of your extension repository. Any filename that has `LICENCE` or `LICENSE` as a prefix (case insensitive) will be inspected to ensure it matches one of the accepted licenses. See the [license validation source code](https://github.com/zed-industries/extensions/blob/main/src/lib/license.js).

> This license requirement applies only to your extension code itself (the code that gets compiled into the extension binary).
> It does not apply to any tools your extension may download or interact with, such as language servers or other external dependencies.
> If your repository contains both extension code and other projects (like a language server), you are not required to relicense those other projects — only the extension code needs to be one of the aforementioned accepted licenses.

## Extension Publishing Prerequisites

Before publishing your extension, make sure that you have chosen a unique extension ID for your extension in the [extension manifest](#directory-structure-of-a-zed-extension).
This will be the primary identifier for your extension and cannot be changed after your extension has been published.
Also, ensure that you have filled out all the required fields in the manifest.

Furthermore, please make sure that your extension fulfills the following preconditions before you move on to publishing your extension:

- Extension IDs and names must not contain the words `zed`, `Zed` or `extension`, since they are all Zed extensions.
- Your extension ID should provide some information on what your extension tries to accomplish. E.g. for themes, it should be suffixed with `-theme`, snippet extensions should be suffixed with `-snippets` and so on. An exception to that rule are extension that provide support for languages or popular tooling that people would expect to find under that ID. You can take a look at the list of [existing extensions](https://github.com/zed-industries/extensions/blob/main/extensions.toml) to get a grasp on how this usually is enforced.
- Extensions should provide something that is not yet available in the marketplace as opposed to fixing something that could be resolved within an existing extension. For example, if you find that an existing extension's support for a language server is not functioning properly, first try contributing a fix to the existing extension as opposed to submitting a new extension immediately.
  - If you receive no response or reaction within the upstream repository within a reasonable amount of time, feel free to submit a pull request that aims to fix said issue. Please ensure that you provide your previous efforts within the pull request to the extensions repository for adding your extension. Zed maintainers will then decide on how to proceed on a case by case basis.
- Extensions that intend to provide a language, debugger or MCP server must not ship the language server as part of the extension. Instead, the extension should either download the language server or check for the availability of the language server in the users environment using the APIs as provided by the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/).
- Themes and icon themes should not be published as part of extensions that provide other features, e.g. language support. Instead, they should be published as a distinct extension. This also applies to theme and icon themes living in the same repository.

Note that non-compliance will be raised during the publishing process by reviewers and delay the release of your extension.

## Publishing your extension

> Prior to publishing your extension, you should have installed as well as tested it locally thoroughly. Note that untested extension submissions where the extension is not functioning at all will be closed eagerly without further feedback.

To publish an extension, open a PR to [the `zed-industries/extensions` repo](https://github.com/zed-industries/extensions).

In your PR, do the following:

1. Add your extension as a Git submodule within the `extensions/` directory under the `extensions/{extension-id}` path

```sh
git submodule add https://github.com/your-username/foobar-zed.git extensions/my-extension
git add extensions/my-extension
```

> All extension submodules must use HTTPS URLs and not SSH URLS (`git@github.com`).

2. Add a new entry to the top-level `extensions.toml` file containing your extension:

```toml
[my-extension]
submodule = "extensions/my-extension"
version = "0.0.1"
```

If your extension is in a subdirectory within the submodule, you can use the `path` field to point to where the extension resides:

```toml
[my-extension]
submodule = "extensions-my-extension"
path = "packages/zed"
version = "0.0.1"
```

> Note that the [required extension license](#extension-license-requirements) must reside at the specified path, a license at the root of the repository will not work. However, you are free to symlink an existing license within the repository or choose an alternative license from the list of accepted licenses for the extension code.

3. Run `pnpm sort-extensions` to ensure `extensions.toml` and `.gitmodules` are sorted

Once your PR is merged, the extension will be packaged and published to the Zed extension registry.

## Updating an extension

To update an extension, open a PR to [the `zed-industries/extensions` repo](https://github.com/zed-industries/extensions).

In your PR do the following:

1. Update the extension's submodule to the commit of the new version. For this, you can run

```sh
# From the root of the repository:
git submodule update --remote extensions/your-extension-name
```

to update your extension to the latest commit available in your remote repository.

2. Update the `version` field for the extension in `extensions.toml`
   - Make sure the `version` matches the one set in `extension.toml` at the particular commit.

If you'd like to automate this process, there is a [community GitHub Action](https://github.com/huacnlee/zed-extension-action) you can use.

> **Note:** If your extension repository has a different license, you'll need to update it to be one of the [accepted extension licenses](#extension-license-requirements) before publishing your update.
