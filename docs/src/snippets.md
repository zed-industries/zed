---
title: Snippets - Zed
description: Create and use code snippets in Zed with tab stops, placeholders, variables, and language-scoped triggers.
---

# Snippets

Use the {#action snippets::ConfigureSnippets} action to create a new snippets file or edit an existing snippets file for a specified [scope](#scopes).

The snippets are located in `~/.config/zed/snippets` directory to which you can navigate with the {#action snippets::OpenFolder} action.

## Example configuration

```json
{
  // Each snippet must have a name and body, but the prefix and description are optional.
  // The prefix is used to trigger the snippet, but when omitted then the name is used.
  // Use placeholders like $1, $2 or ${1:defaultValue} to define tab stops.
  // The $0 determines the final cursor position.
  // Placeholders with the same value are linked.
  // If the snippet contains the $ symbol outside of a placeholder, it must be escaped with two slashes (e.g. \\$var).
  "Log to console": {
    "prefix": "log",
    "body": ["console.info(\"Hello, ${1:World}!\")", "$0"],
    "description": "Logs to console"
  }
}
```

## Scopes

The scope is determined by the language name in lowercase e.g. `python.json` for Python, `shell script.json` for Shell Script, but there are some exceptions to this rule:

| Scope      | Filename        |
| ---------- | --------------- |
| Global     | snippets.json   |
| JSX        | javascript.json |
| Plain Text | plaintext.json  |

To create JSX snippets you have to use `javascript.json` snippets file, instead of `jsx.json`, but this does not apply to TSX and TypeScript which follow the above rule.

## Variables

Snippet bodies may reference variables with `$NAME` or `${NAME:default}`. When a variable has no value, its default (or the empty string) is used. The following [LSP snippet variables](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#snippet_syntax) are supported:

| Variable                   | Description                                                       |
| -------------------------- | ----------------------------------------------------------------- |
| `TM_SELECTED_TEXT`         | The currently selected text, or the empty string                  |
| `TM_CURRENT_LINE`          | The contents of the current line                                  |
| `TM_CURRENT_WORD`          | The word under the cursor, or the empty string                    |
| `TM_LINE_INDEX`            | The zero-based line number                                        |
| `TM_LINE_NUMBER`           | The one-based line number                                         |
| `TM_FILENAME`              | The filename of the current document                              |
| `TM_FILENAME_BASE`         | The filename of the current document without its extension        |
| `TM_DIRECTORY`             | The directory of the current document                             |
| `TM_FILEPATH`              | The full file path of the current document                        |
| `RELATIVE_FILEPATH`        | The file path relative to the workspace                           |
| `CLIPBOARD`                | The contents of your clipboard                                    |
| `WORKSPACE_NAME`           | The name of the opened workspace or folder                        |
| `WORKSPACE_FOLDER`         | The path of the opened workspace or folder                        |
| `CURSOR_INDEX`             | The zero-based cursor number                                      |
| `CURSOR_NUMBER`            | The one-based cursor number                                       |
| `CURRENT_YEAR`             | The current year                                                  |
| `CURRENT_YEAR_SHORT`       | The current year's last two digits                                |
| `CURRENT_MONTH`            | The month as two digits (e.g. `02`)                               |
| `CURRENT_MONTH_NAME`       | The full name of the month (e.g. `July`)                          |
| `CURRENT_MONTH_NAME_SHORT` | The short name of the month (e.g. `Jul`)                          |
| `CURRENT_DATE`             | The day of the month as two digits (e.g. `08`)                    |
| `CURRENT_DAY_NAME`         | The name of the day (e.g. `Monday`)                               |
| `CURRENT_DAY_NAME_SHORT`   | The short name of the day (e.g. `Mon`)                            |
| `CURRENT_HOUR`             | The current hour in 24-hour clock format                          |
| `CURRENT_MINUTE`           | The current minute as two digits                                  |
| `CURRENT_SECOND`           | The current second as two digits                                  |
| `CURRENT_SECONDS_UNIX`     | The number of seconds since the Unix epoch                        |
| `CURRENT_TIMEZONE_OFFSET`  | The current UTC timezone offset (e.g. `-07:00`)                   |
| `RANDOM`                   | Six random base-10 digits                                         |
| `RANDOM_HEX`               | Six random base-16 digits                                         |
| `UUID`                     | A version 4 UUID                                                  |
| `LINE_COMMENT`             | The line comment token for the current language (e.g. `//`)       |
| `BLOCK_COMMENT_START`      | The block comment start token for the current language (e.g. `/*`)|
| `BLOCK_COMMENT_END`        | The block comment end token for the current language (e.g. `*/`)  |

Variables that depend on a selection or cursor (such as `TM_SELECTED_TEXT`) are resolved per cursor, so each cursor contributes its own value.

## Known Limitations

- Only the first prefix is used when a list of prefixes is passed in.
- Currently only the `json` snippet file format is supported.
