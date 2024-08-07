# Developing Extensions

TBD: Document how to create an extension in Zed.

## Extension Capabilities

Extensions are a way to add extend functionality of Zed. Extensions may contain any combination of:

- [grammars](#grammars)
- [languages](#languages)
- [themes](./themes.md)

For example, you can have an extension that provides both a grammar and a language, or one that just provides a theme.

## Extension Structure

A Zed extension is a Git repository that contains an `extension.toml`:

```toml
id = "my-extension"
name = "My extension"
version = "0.0.1"
schema_version = 1
authors = ["Your Name <you@example.com>"]
description = "My cool extension"
repository = "https://github.com/your-name/my-zed-extension"
```

### Grammars

Zed implemented syntax highlighting using tree-sitter grammars. If your extension contains grammars, you can reference the provided grammars in your `extension.toml` like so:

```toml
[grammars.gleam]
repository = "https://github.com/gleam-lang/tree-sitter-gleam"
commit = "58b7cac8fc14c92b0677c542610d8738c373fa81"
```

The `repository` field must specify a repository where the Tree-sitter grammar should be loaded from, and the `commit` field must contain the SHA of the Git commit to use. An extension can provide multiple grammars by referencing multiple tree-sitter repositories.

Upon installation Zed will clone the specified repositories, build and compile the grammars for the WASM target

### Languages config.toml

For each tree-sitter grammar you provide create a `languages/lang_name` directory in your extension. Inside this directory create a `config.toml` file with the following structure:

```toml
name = "Dockerfile"
grammar = "dockerfile"
path_suffixes = ["Dockerfile", "Dockerfile.*"]
line_comments = ["# "]
```

- `name` is the human readable name that will show up in the Select Language dropdown.
- `grammar` is the grammar name as specified in the `extension.toml` and in your `grammar.js` tree-sitter grammar.
- `path_suffixes` (optional) is an array of file suffixes that should be associated with this language. This supports glob patterns like `config/**/*.toml` where `**` matchs 0 or more directories and `*` matches 0 or more characters.
- `line_comments` (optional) is an array of strings that are used to identify line comments in the language.

### Languages tree-sitter queries

Optionally create one or more `*.scm` files inside the `languages/lang_name` directory. These files contain tree-sitter queries that can be used to implement language specific features like syntax highlighting, folding, etc.

See: [Zed Tree Sitter Documentation](../tree-sitter.md)

### Language servers

Zed uses the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) to provide language support. This means, in theory, we can support any language that has an LSP server. If you wish to provide a language server with your extension, you will need to integrate against the [Zed extension API](https://crates.io/crates/zed_extension_api).

Create a Rust library at the root of your extension repository.

Your `Cargo.toml` should look like this:

```toml
[package]
name = "my-extension"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.0.6"
```

Make sure to use the latest version of the `zed_extension_api` available on crates.io.

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

Finally, add an entry to your `extension.toml` with the name of your language server and the language it applies to:

```toml
[language_servers.some-language]
name = "My Extension LSP"
language = "Some Language"
```

For more examples on providing language servers via extensions, take a look at the [`extensions/`](https://github.com/zed-industries/zed/tree/main/extensions) in the Zed repository. The Zed Rust Extension API is also included under [`crates/extension_api`](https://github.com/zed-industries/zed/blob/main/crates/extension_api/README.md) directory.

## Testing your extension

TBD: Document `cmd-shift-x` "Install Dev" Extension. Maybe mention `tail -f ~/Library/Logs/Zed/zed.log` for debugging.

## Submitting your extension

TBD: Document PR to zed-industries/extensions
