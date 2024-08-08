# Developing Language Extensions

Language support in Zed has several components:

- Language metadata and configuration
- Grammar
- Queries
- Language servers

## Language Metadata

Each language supported by Zed must be defined in a subdirectory inside the `languages` directory of your extension.

This subdirectory must contain a file called `config.toml` file with the following structure:

```toml
name = "My Language"
grammar = "my-language"
path_suffixes = ["myl"]
line_comments = ["# "]
```

- `name` is the human readable name that will show up in the Select Language dropdown.
- `grammar` is the name of a grammar. Grammars are registered separately, described below.
- `path_suffixes` (optional) is an array of file suffixes that should be associated with this language. This supports glob patterns like `config/**/*.toml` where `**` matches 0 or more directories and `*` matches 0 or more characters.
- `line_comments` (optional) is an array of strings that are used to identify line comments in the language.

TBD: Document `language_name/config.toml` keys

- line_comments, block_comment
- autoclose_before
- brackets (start, end, close, newline, not_in: ["comment", "string"])
- tab_size, hard_tabs
- word_characters
- prettier_parser_name
- opt_into_language_servers
- first_line_pattern
- code_fence_block_name
- scope_opt_in_language_servers
- increase_indent_pattern, decrease_indent_pattern
- collapsed_placeholder

## Grammar

A grammar controls how a language is parsed. As shown above, the language configuration file supplies the _name_ of a grammar. Each grammar needs to be registered separately. You can denote the provided grammars in your `extension.toml` like so:

```toml
[grammars.gleam]
repository = "https://github.com/gleam-lang/tree-sitter-gleam"
commit = "58b7cac8fc14c92b0677c542610d8738c373fa81"
```

The `repository` field must specify a repository where the Tree-sitter grammar should be loaded from, and the `commit` field must contain the SHA of the Git commit to use. An extension can provide multiple grammars by referencing multiple tree-sitter repositories.

## Tree-sitter Queries

Zed uses syntax trees produced by these grammars for several editor features. Tree-sitter queries are used to define how information should be extracted from the syntax trees. Zed recognizes the following queries:

- `highlights.scm` - defines syntax highlighting rules
- `injections.scm` - defines regions that should be parsed into another syntax tree.
- `outline.scm` - extracts symbols defined in a file
- `indents.scm` - controls automatic indentation
- `runnables.scm` - defines functions that can be run using tasks.
- `brackets.scm` - identifies pairs of tokens that are considered brackets

For more information about Tree-sitter queries in general, see [the Tree-sitter documentation](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries).

## Language Servers

Zed uses the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) to provide advanced language support.

An extension may provide any number of language servers. To provide a language server from your extension, add an entry to your `extension.toml` with the name of your language server and the language it applies to:

```toml
[language_servers.my-language]
name = "My Language LSP"
language = "My Language"
```

Then, in the Rust code for your extension, implement the `language_server_command` method on your extension:

```rust
impl zed::Extension for MyExtension {
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: get_path_to_language_server_executable()?,
            args: get_args_for_language_server()?,
            env: get_env_for_language_server()?,
        })
    }
}
```

You can customize the handling of the language server using several optional methods in the `Extension` trait. For example, you can control how completions are styled using the `label_for_completion` method. For a complete list of methods, see the [API docs for the Zed extension API](https://docs.rs/zed_extension_api).
