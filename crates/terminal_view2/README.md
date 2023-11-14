Design notes:

This crate is split into two conceptual halves:
- The terminal.rs file and the src/mappings/ folder, these contain the code for interacting with Alacritty and maintaining the pty event loop. Some behavior in this file is constrained by terminal protocols and standards. The Zed init function is also placed here.
- Everything else. These other files integrate the `Terminal` struct created in terminal.rs into the rest of GPUI. The main entry point for GPUI is the terminal_view.rs file and the modal.rs file.

ttys are created externally, and so can fail in unexpected ways. However, GPUI currently does not have an API for models than can fail to instantiate. `TerminalBuilder` solves this by using Rust's type system to split tty instantiation into a 2 step process: first attempt to create the file handles with `TerminalBuilder::new()`, check the result, then call `TerminalBuilder::subscribe(cx)` from within a model context.

The TerminalView struct abstracts over failed and successful terminals, passing focus through to the associated view and allowing clients to build a terminal without worrying about errors.

#Input

There are currently many distinct paths for getting keystrokes to the terminal:

1. Terminal specific characters and bindings. Things like ctrl-a mapping to ASCII control character 1, ANSI escape codes associated with the function keys, etc. These are caught with a raw key-down handler in the element and are processed immediately. This is done with the `try_keystroke()` method on Terminal

2. GPU Action handlers. GPUI clobbers a few vital keys by adding bindings to them in the global context. These keys are synthesized and then dispatched through the same `try_keystroke()` API as the above mappings

3. IME text. When the special character mappings fail, we pass the keystroke back to GPUI to hand it to the IME system. This comes back to us in the `View::replace_text_in_range()` method, and we then send that to the terminal directly, bypassing `try_keystroke()`.

4. Pasted text has a separate pathway.

Generally, there's a distinction between 'keystrokes that need to be mapped' and 'strings which need to be written'. I've attempted to unify these under the '.try_keystroke()' API and the `.input()` API (which try_keystroke uses) so we have consistent input handling across the terminal
