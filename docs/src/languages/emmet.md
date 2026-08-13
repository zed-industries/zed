---
title: Emmet
description: "Configure Emmet language support in Zed, including language servers, formatting, and debugging."
---

# Emmet

Emmet support is available through the [Emmet extension](https://github.com/zed-extensions/emmet).

[Emmet](https://emmet.io/) is a web-developer’s toolkit that can greatly improve your HTML & CSS workflow.

- Language Server: [olrtg/emmet-language-server](https://github.com/olrtg/emmet-language-server)

## Wrap with abbreviation

With the Emmet extension installed, you can wrap the current selections in an expanded [Emmet abbreviation](https://docs.emmet.io/abbreviations/):

1. Select the text to wrap. With an empty selection, the enclosing HTML element is wrapped; if no enclosing element can be determined, the current line is wrapped instead.
2. Run `editor: wrap with abbreviation` from the command palette. The last used abbreviation is prefilled and selected, so typing replaces it.
3. Type an abbreviation, for example `div.wrapper>ul>li*3`, into the input that appears below the selection. A preview of the expansion is shown under the input as you type.
4. Press `enter` to apply, or `escape` to cancel. If the abbreviation produces no expansion, the error is shown next to the input and the input stays open so you can correct it.

For a single selection, the expansion is inserted as a snippet and the cursor is placed at the end of the expansion. With multiple selections, each selection is wrapped separately; overlapping selections are merged and wrapped once.

The [documented wrap behaviors](https://docs.emmet.io/actions/wrap-with-abbreviation/) are supported:

- Wrapping individual lines: mark the repeating element with `*` (without a number), for example `ul>li*`, and each selected line is wrapped in its own element. `$` numbering works too: `ul>li.item$*>a`.
- Removing list markers: append the trim filter `|t`, for example `ul>li*|t`, to strip list markers such as `*`, `-`, or `1.` from the wrapped lines.
- Controlling output position: use the `$#` placeholder inside attribute values or text nodes, for example `ul>li[title=$#]*>{$#}`.
- The [comment filter](https://docs.emmet.io/filters/comment/) `|c` and the [BEM filter](https://docs.emmet.io/filters/bem/) `|bem` are also supported and can be combined, for example `div#page>p.intro|c|bem`.

Selections that partially cover an opening or closing tag are expanded to the whole element before wrapping. Wrapping is a markup action: it is not offered in stylesheets, where Emmet provides completions instead.

The action has no default keybinding. To add one, bind it in your keymap:

```json [keymap]
[
  {
    "context": "Editor && mode == full",
    "bindings": {
      "ctrl-alt-w": "editor::WrapWithAbbreviation"
    }
  }
]
```

<!--
TBD: Document Emmet usage in zed with: HTML, PHP, ERB, Javascript, TSX, CSS
-->
