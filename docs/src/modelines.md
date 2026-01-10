# Modelines

Modelines are special comments at the beginning or end of a file that configure editor settings for that specific file. Zed supports both Vim and Emacs modeline formats, allowing you to specify settings like tab size, indentation style, and file type directly within your files.

## Configuration

Use the [`modeline_lines`](./configuring-zed.md#modeline-lines) setting to control how many lines Zed searches for modelines:

```json [settings]
{
  "modeline_lines": 5
}
```

Set to `0` to disable modeline parsing entirely.

## Emacs

Zed has some compatibility support for [Emacs file variables](https://www.gnu.org/software/emacs/manual/html_node/emacs/Specifying-File-Variables.html).

Example:

```python
# -*- mode: python; tab-width: 4; indent-tabs-mode: nil; -*-
```

### Supported Emacs Variables

| Variable                   | Description                    | Zed Setting                                                           |
| -------------------------- | ------------------------------ | --------------------------------------------------------------------- |
| `mode`                     | Major mode/language            | Language detection                                                    |
| `tab-width`                | Tab display width              | [`tab_size`](./configuring-zed.md#tab-size)                           |
| `fill-column`              | Line wrap column               | [`preferred_line_length`](./configuring-zed.md#preferred-line-length) |
| `indent-tabs-mode`         | `nil` for spaces, `t` for tabs | [`hard_tabs`](./configuring-zed.md#hard-tabs)                         |
| `electric-indent-mode`     | Auto-indentation               | [`auto_indent`](./configuring-zed.md#auto-indent)                     |
| `require-final-newline`    | Ensure final newline           | [`ensure_final_newline`](./configuring-zed.md#ensure-final-newline)   |
| `show-trailing-whitespace` | Show trailing whitespace       | [`show_whitespaces`](./configuring-zed.md#show-whitespaces)           |

## Vim

Zed has some compatibility support for [Vim modeline](https://vimhelp.org/options.txt.html#modeline).

Example:

```python
# vim: set ft=python ts=4 sw=4 et:
```

### Supported Vim Options

| Option         | Aliases | Description                       | Zed Setting                                                           |
| -------------- | ------- | --------------------------------- | --------------------------------------------------------------------- |
| `filetype`     | `ft`    | File type/language                | Language detection                                                    |
| `tabstop`      | `ts`    | Number of spaces a tab counts for | [`tab_size`](./configuring-zed.md#tab-size)                           |
| `textwidth`    | `tw`    | Maximum line width                | [`preferred_line_length`](./configuring-zed.md#preferred-line-length) |
| `expandtab`    | `et`    | Use spaces instead of tabs        | [`hard_tabs`](./configuring-zed.md#hard-tabs)                         |
| `noexpandtab`  | `noet`  | Use tabs instead of spaces        | [`hard_tabs`](./configuring-zed.md#hard-tabs)                         |
| `autoindent`   | `ai`    | Enable auto-indentation           | [`auto_indent`](./configuring-zed.md#auto-indent)                     |
| `noautoindent` | `noai`  | Disable auto-indentation          | [`auto_indent`](./configuring-zed.md#auto-indent)                     |
| `endofline`    | `eol`   | Ensure final newline              | [`ensure_final_newline`](./configuring-zed.md#ensure-final-newline)   |
| `noendofline`  | `noeol` | Disable final newline             | [`ensure_final_newline`](./configuring-zed.md#ensure-final-newline)   |

## Notes

- The first kilobyte of a file is searched for modelines.
- Emacs modelines take precedence over Vim modelines when both are present.
- Modelines in the first few lines take precedence over those at the end of the file.
