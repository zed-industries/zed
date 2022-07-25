Design notes:

This crate is split into two conceptual halves:
- The terminal.rs file and the ./src/mappings/ folder, these contain the code for interacting with Alacritty and maintaining the pty event loop. Some behavior in this file is constrained by terminal protocols and standards. The Zed init function is also placed here.
- Everything else. These other files integrate the `Terminal` struct created in terminal.rs into the rest of GPUI. The main entry point for GPUI is the terminal_view.rs file and the modal.rs file. 

Terminals are created externally, and so can fail in unexpected ways However, GPUI currently does not have an API for models than can fail to instantiate. `TerminalBuilder` solves this by using Rust's type system to split `Terminal` instantiation into a 2 step process: first attempt to create the file handles with `TerminalBuilder::new()`, check the result, then call `TerminalBuilder::subscribe(cx)` from within a model context.
The TerminalView struct abstracts over failed and successful terminals, and provides other constructs a standardized way of instantiating an always-successful terminal view.