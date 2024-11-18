# Language Extensions

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

- `name` (required) is the human readable name that will show up in the Select Language dropdown.
- `grammar` (required) is the name of a grammar. Grammars are registered separately, described below.
- `path_suffixes` is an array of file suffixes that should be associated with this language. Unlike `file_types` in settings, this does not support glob patterns.
- `line_comments` is an array of strings that are used to identify line comments in the language. This is used for the `editor::ToggleComments` keybind: `{#kb editor::ToggleComments}` for toggling lines of code.
- `tab_size` defines the indentation/tab size used for this language (default is `4`).
- `hard_tabs` whether to indent with tabs (`true`) or spaces (`false`, the default).
- `first_line_pattern` is a regular expression, that in addition to `path_suffixes` (above) or `file_types` in settings can be used to match files which should use this language. For example Zed uses this to identify Shell Scripts by matching the [shebangs lines](https://github.com/zed-industries/zed/blob/main/crates/languages/src/bash/config.toml) in the first line of a script.

<!--
TBD: Document `language_name/config.toml` keys

- autoclose_before
- brackets (start, end, close, newline, not_in: ["comment", "string"])
- word_characters
- prettier_parser_name
- opt_into_language_servers
- code_fence_block_name
- scope_opt_in_language_servers
- increase_indent_pattern, decrease_indent_pattern
- collapsed_placeholder
- auto_indent_on_paste, auto_indent_using_last_non_empty_line
- overrides: `[overrides.element]`, `[overrides.string]`
-->

## Grammar

Zed uses the [Tree-sitter](https://tree-sitter.github.io) parsing library to provide built-in language-specific features. There are grammars available for many languages, and you can also [develop your own grammar](https://tree-sitter.github.io/tree-sitter/creating-parsers#writing-the-grammar). A growing list of Zed features are built using pattern matching over syntax trees with Tree-sitter queries. As mentioned above, every language that is defined in an extension must specify the name of a Tree-sitter grammar that is used for parsing. These grammars are then registered separately in extensions' `extension.toml` file, like this:

```toml
[grammars.gleam]
repository = "https://github.com/gleam-lang/tree-sitter-gleam"
commit = "58b7cac8fc14c92b0677c542610d8738c373fa81"
```

The `repository` field must specify a repository where the Tree-sitter grammar should be loaded from, and the `commit` field must contain the SHA of the Git commit to use. An extension can provide multiple grammars by referencing multiple tree-sitter repositories.

## Tree-sitter Queries

Zed uses the syntax tree produced by the [Tree-sitter](https://tree-sitter.github.io) query language to implement
several features:

- Syntax highlighting
- Bracket matching
- Code outline/structure
- Auto-indentation
- Code injections
- Syntax overrides
- Text redactions
- Runnable code detection

The following sections elaborate on how [Tree-sitter queries](https://tree-sitter.github.io/tree-sitter/using-parsers#query-syntax) enable these
features in Zed, using [JSON syntax](https://www.json.org/json-en.html) as a guiding example.

### Syntax highlighting

In Tree-sitter, the `highlights.scm` file defines syntax highlighting rules for a particular syntax.

Here's an example from a `highlights.scm` for JSON:

```scheme
(string) @string

(pair
  key: (string) @property.json_key)

(number) @number
```

This query marks strings, object keys, and numbers for highlighting. The following is a comprehensive list of captures supported by themes:

| Capture                  | Description                            |
| ------------------------ | -------------------------------------- |
| @attribute               | Captures attributes                    |
| @boolean                 | Captures boolean values                |
| @comment                 | Captures comments                      |
| @comment.doc             | Captures documentation comments        |
| @constant                | Captures constants                     |
| @constructor             | Captures constructors                  |
| @embedded                | Captures embedded content              |
| @emphasis                | Captures emphasized text               |
| @emphasis.strong         | Captures strongly emphasized text      |
| @enum                    | Captures enumerations                  |
| @function                | Captures functions                     |
| @hint                    | Captures hints                         |
| @keyword                 | Captures keywords                      |
| @label                   | Captures labels                        |
| @link_text               | Captures link text                     |
| @link_uri                | Captures link URIs                     |
| @number                  | Captures numeric values                |
| @operator                | Captures operators                     |
| @predictive              | Captures predictive text               |
| @preproc                 | Captures preprocessor directives       |
| @primary                 | Captures primary elements              |
| @property                | Captures properties                    |
| @punctuation             | Captures punctuation                   |
| @punctuation.bracket     | Captures brackets                      |
| @punctuation.delimiter   | Captures delimiters                    |
| @punctuation.list_marker | Captures list markers                  |
| @punctuation.special     | Captures special punctuation           |
| @string                  | Captures string literals               |
| @string.escape           | Captures escaped characters in strings |
| @string.regex            | Captures regular expressions           |
| @string.special          | Captures special strings               |
| @string.special.symbol   | Captures special symbols               |
| @tag                     | Captures tags                          |
| @tag.doctype             | Captures doctypes (e.g., in HTML)      |
| @text.literal            | Captures literal text                  |
| @title                   | Captures titles                        |
| @type                    | Captures types                         |
| @variable                | Captures variables                     |
| @variable.special        | Captures special variables             |
| @variant                 | Captures variants                      |

### Bracket matching

The `brackets.scm` file defines matching brackets.

Here's an example from a `brackets.scm` file for JSON:

```scheme
("[" @open "]" @close)
("{" @open "}" @close)
("\"" @open "\"" @close)
```

This query identifies opening and closing brackets, braces, and quotation marks.

| Capture | Description                                   |
| ------- | --------------------------------------------- |
| @open   | Captures opening brackets, braces, and quotes |
| @close  | Captures closing brackets, braces, and quotes |

### Code outline/structure

The `outline.scm` file defines the structure for the code outline.

Here's an example from an `outline.scm` file for JSON:

```scheme
(pair
  key: (string (string_content) @name)) @item
```

This query captures object keys for the outline structure.

| Capture        | Description                                                                          |
| -------------- | ------------------------------------------------------------------------------------ |
| @name          | Captures the content of object keys                                                  |
| @item          | Captures the entire key-value pair                                                   |
| @context       | Captures elements that provide context for the outline item                          |
| @context.extra | Captures additional contextual information for the outline item                      |
| @annotation    | Captures nodes that annotate outline item (doc comments, attributes, decorators)[^1] |

[^1]: These annotations are used by Assistant when generating code modification steps.

### Auto-indentation

The `indents.scm` file defines indentation rules.

Here's an example from an `indents.scm` file for JSON:

```scheme
(array "]" @end) @indent
(object "}" @end) @indent
```

This query marks the end of arrays and objects for indentation purposes.

| Capture | Description                                        |
| ------- | -------------------------------------------------- |
| @end    | Captures closing brackets and braces               |
| @indent | Captures entire arrays and objects for indentation |

### Code injections

The `injections.scm` file defines rules for embedding one language within another, such as code blocks in Markdown or SQL queries in Python strings.

Here's an example from an `injections.scm` file for Markdown:

```scheme
(fenced_code_block
  (info_string
    (language) @language)
  (code_fence_content) @content)

((inline) @content
 (#set! "language" "markdown-inline"))
```

This query identifies fenced code blocks, capturing the language specified in the info string and the content within the block. It also captures inline content and sets its language to "markdown-inline".

| Capture   | Description                                                |
| --------- | ---------------------------------------------------------- |
| @language | Captures the language identifier for a code block          |
| @content  | Captures the content to be treated as a different language |

Note that we couldn't use JSON as an example here because it doesn't support language injections.

### Syntax overrides

The `overrides.scm` file defines syntactic _scopes_ that can be used to override certain editor settings within specific language constructs.

For example, there is a language-specific setting called `word_characters` that controls which non-alphabetic characters are considered part of a word, for filtering autocomplete suggestions. In JavaScript, "$" and "#" are considered word characters. But when your cursor is within a _string_ in JavaScript, "-" is _also_ considered a word character. To achieve this, the JavaScript `overrides.scm` file contains the following pattern:

```scheme
[
  (string)
  (template_string)
] @string
```

And the JavaScript `config.toml` contains this setting:

```toml
word_characters = ["#", "$"]

[overrides.string]
word_characters = ["-"]
```

You can also disable certain auto-closing brackets in a specific scope. For example, to prevent auto-closing `'` within strings, you could put the following in the JavaScript `config.toml`:

```toml
brackets = [
  { start = "'", end = "'", close = true, newline = false, not_in = ["string"] },
  # other pairs...
]
```

#### Range inclusivity

By default, the ranges defined in `overrides.scm` are _exclusive_. So in the case above, if you cursor was _outside_ the quotation marks delimiting the string, the `string` scope would not take effect. Sometimes, you may want to make the range _inclusive_. You can do this by adding the `.inclusive` suffix to the capture name in the query.

For example, in JavaScript, we also disable auto-closing of single quotes within comments. And the comment scope must extend all the way to the newline after a line comment. To achieve this, the JavaScript `overrides.scm` contains the following pattern:

```scheme
(comment) @comment.inclusive
```

### Text redactions

The `redactions.scm` file defines text redaction rules. When collaborating and sharing your screen, it makes sure that certain syntax nodes are rendered in a redacted mode to avoid them from leaking.

Here's an example from a `redactions.scm` file for JSON:

```scheme
(pair value: (number) @redact)
(pair value: (string) @redact)
(array (number) @redact)
(array (string) @redact)
```

This query marks number and string values in key-value pairs and arrays for redaction.

| Capture | Description                    |
| ------- | ------------------------------ |
| @redact | Captures values to be redacted |

### Runnable code detection

The `runnables.scm` file defines rules for detecting runnable code.

Here's an example from an `runnables.scm` file for JSON:

```scheme
(
    (document
        (object
            (pair
                key: (string
                    (string_content) @_name
                    (#eq? @_name "scripts")
                )
                value: (object
                    (pair
                        key: (string (string_content) @run @script)
                    )
                )
            )
        )
    )
    (#set! tag package-script)
    (#set! tag composer-script)
)
```

This query detects runnable scripts in package.json and composer.json files.

The `@run` capture specifies where the run button should appear in the editor. Other captures, except those prefixed with an underscore, are exposed as environment variables with a prefix of `ZED_CUSTOM_$(capture_name)` when running the code.

| Capture | Description                                            |
| ------- | ------------------------------------------------------ |
| @\_name | Captures the "scripts" key                             |
| @run    | Captures the script name                               |
| @script | Also captures the script name (for different purposes) |

<!--
TBD: `#set! tag`
-->

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
