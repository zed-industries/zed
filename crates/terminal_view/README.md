# Terminal View

## Design Notes

This crate is split into two conceptual halves:
- The terminal.rs file and the src/mappings/ folder, these contain the code for interacting with terminal emulator backends and maintaining the pty event loop. Some behavior in this file is constrained by terminal protocols and standards. The Zed init function is also placed here.
- Everything else. These other files integrate the `Terminal` struct created in terminal.rs into the rest of GPUI. The main entry point for GPUI is the terminal_view.rs file and the modal.rs file.

ttys are created externally, and so can fail in unexpected ways. However, GPUI currently does not have an API for models than can fail to instantiate. `TerminalBuilder` solves this by using Rust's type system to split tty instantiation into a 2 step process: first attempt to create the file handles with `TerminalBuilder::new()`, check the result, then call `TerminalBuilder::subscribe(cx)` from within a model context.

The TerminalView struct abstracts over failed and successful terminals, passing focus through to the associated view and allowing clients to build a terminal without worrying about errors.

## Ghostty Backend

The optional `libghostty-vt` backend is more than a packaging swap. It lets Zed delegate VT parsing, terminal state, render snapshots, and mode-aware input encoding to Ghostty's terminal core while preserving Zed's existing `TerminalContent` and `TerminalView` integration surface.

What Zed gains from the backend:

- A terminal core with an embeddable API for render rows, cells, modes, cursor state, scrollback, title and bell callbacks, device attributes, size reports, and color-scheme responses.
- Mode-aware key, focus, and mouse encoding based on Ghostty's current terminal state, reducing the amount of escape-sequence behavior Zed has to own locally.
- More structured handling for the OSC sequences Zed integrates with today. Ghostty owns the terminal stream and Zed observes or adapts OSC 7 working-directory reports, OSC 8 hyperlinks, OSC 52 clipboard operations, and OSC 4/10/11/12 palette set/query/reset behavior so they continue to flow through Zed services and settings.
- Render data that maps cleanly into existing frontend behavior: cursor shape and blink state, SGR styles, wide cells, hyperlinks, selection and copy, find, scroll-to-match, and vi-mode cursor/selection all pass through the same `TerminalContent` adapter.
- A narrower path for future terminal conformance improvements. Advancing the pinned Ghostty source and checked-in bindings can bring in upstream terminal-protocol fixes without Zed reimplementing each behavior in its own backend.

The backend does not remove Zed-specific integration. PTY lifecycle, app-level clipboard policy, working-directory behavior, hyperlink activation, selection UX, and TerminalView/Agent Panel UI still live in Zed. It also does not mean every OSC path is automatically handled by Ghostty; some sequences are intentionally observed in Zed because they need to call Zed services or preserve existing user-facing behavior.

## Input

There are currently many distinct paths for getting keystrokes to the terminal:

1. Terminal specific characters and bindings. Things like ctrl-a mapping to ASCII control character 1, ANSI escape codes associated with the function keys, etc. These are caught with a raw key-down handler in the element and are processed immediately. This is done with the `try_keystroke()` method on Terminal

2. GPU Action handlers. GPUI clobbers a few vital keys by adding bindings to them in the global context. These keys are synthesized and then dispatched through the same `try_keystroke()` API as the above mappings

3. IME text. When the special character mappings fail, we pass the keystroke back to GPUI to hand it to the IME system. This comes back to us in the `View::replace_text_in_range()` method, and we then send that to the terminal directly, bypassing `try_keystroke()`.

4. Pasted text has a separate pathway.

Generally, there's a distinction between 'keystrokes that need to be mapped' and 'strings which need to be written'. I've attempted to unify these under the '.try_keystroke()' API and the `.input()` API (which try_keystroke uses) so we have consistent input handling across the terminal
