//! A terminal related ANSI escape sequences.

sequence!(
    /// Resizes the text area to the given width and height in characters.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ResizeTextArea;
    ///
    /// let mut stdout = stdout();
    /// // Resize the terminal to 80x25
    /// write!(stdout, "{}", ResizeTextArea(80, 25));
    /// ```
    struct ResizeTextArea(u16, u16) =>
    |this, f| write!(f, csi!("8;{};{}t"), this.1, this.0)
);

sequence!(
    /// Tells the terminal to start reporting mouse events.
    /// 
    /// Mouse events are not reported by default.
    struct EnableMouseEvents => concat!(
        csi!("?1000h"),
        csi!("?1002h"),
        csi!("?1015h"),
        csi!("?1006h")
    )
);

sequence!(
    /// Tells the terminal to stop reporting mouse events.
    struct DisableMouseEvents => concat!(
        csi!("?1006l"),
        csi!("?1015l"),
        csi!("?1002l"),
        csi!("?1000l")
    )
);

#[cfg(test)]
test_sequences!(
    resize_text_area(
        ResizeTextArea(80, 25) => "\x1B[8;25;80t",
        ResizeTextArea(1, 1) => "\x1B[8;1;1t",
    ),
    enable_mouse_events(
        EnableMouseEvents => "\x1B[?1000h\x1B[?1002h\x1B[?1015h\x1B[?1006h",
    ),
    disable_mouse_events(
        DisableMouseEvents => "\x1B[?1006l\x1B[?1015l\x1B[?1002l\x1B[?1000l",
    )    
);
