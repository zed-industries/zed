sequence!(
    /// Switches to the alternate buffer.
    /// 
    /// Use the [`SwitchBufferToNormal`](struct.SwitchBufferToNormal.html) sequence to switch
    /// back to the normal buffer.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::{SwitchBufferToAlternate, SwitchBufferToNormal};
    ///
    /// let mut stdout = stdout();
    /// write!(stdout, "{}", SwitchBufferToAlternate);
    /// // Your app on alternate screen
    /// write!(stdout, "{}", SwitchBufferToNormal);
    /// ```
    struct SwitchBufferToAlternate => csi!("?1049h")
);

sequence!(
    /// Switches to the normal buffer.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::{SwitchBufferToAlternate, SwitchBufferToNormal};
    ///
    /// let mut stdout = stdout();
    /// write!(stdout, "{}", SwitchBufferToAlternate);
    /// // Your app on alternate screen
    /// write!(stdout, "{}", SwitchBufferToNormal);
    /// ```
    struct SwitchBufferToNormal => csi!("?1049l")
);

sequence!(
    /// Scrolls up by the given number of rows.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ScrollBufferUp;
    ///
    /// let mut stdout = stdout();
    /// // Scroll up by 5 lines
    /// write!(stdout, "{}", ScrollBufferUp(5));
    /// ```
    struct ScrollBufferUp(u16) =>
    |this, f| write!(f, csi!("{}S"), this.0)
);

sequence!(
    /// Scrolls down by the given number of rows.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ScrollBufferDown;
    ///
    /// let mut stdout = stdout();
    /// // Scroll down by 10 lines
    /// write!(stdout, "{}", ScrollBufferDown(10));
    /// ```
    struct ScrollBufferDown(u16) =>
    |this, f| write!(f, csi!("{}T"), this.0)
);

sequence!(
    /// Clears part of the line.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ClearLine;
    ///
    /// let mut stdout = stdout();
    /// // Clear the whole line
    /// write!(stdout, "{}", ClearLine::All);
    /// ```
    enum ClearLine {
        /// Clears from the cursor position to end of the line.
        Right => csi!("K"),
        /// Clears from the cursor position to beginning of the line.
        Left => csi!("1K"),
        /// Clears the whole line.
        All => csi!("2K"),
    }
);

sequence!(
    /// Clears part of the buffer.
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ClearBuffer;
    ///
    /// let mut stdout = stdout();
    /// // Clear the entire buffer
    /// write!(stdout, "{}", ClearBuffer::All);
    /// ```
    enum ClearBuffer {
        /// Clears from the cursor position to end of the screen.
        Below => csi!("J"),
        /// Clears from the cursor position to beginning of the screen.
        Above => csi!("1J"),
        /// Clears the entire buffer.
        All => csi!("2J"),
        /// Clears the entire buffer and all saved lines in the scrollback buffer.
        SavedLines => csi!("3J"),
    }
);

#[cfg(test)]
test_sequences!(
    switch_buffer_to_alternate(
        SwitchBufferToAlternate => "\x1B[?1049h",
    ),
    switch_buffer_to_main(
        SwitchBufferToNormal => "\x1B[?1049l",
    ),
    scroll_buffer_up(
        ScrollBufferUp(10) => "\x1B[10S",
    ),
    scroll_buffer_down(
        ScrollBufferDown(10) => "\x1B[10T",
    ),
    clear_line(
        ClearLine::Right => "\x1B[K",
        ClearLine::Left => "\x1B[1K",
        ClearLine::All => "\x1B[2K",
    ),
    clear_buffer(
        ClearBuffer::Below => "\x1B[J",
        ClearBuffer::Above => "\x1B[1J",
        ClearBuffer::All => "\x1B[2J",
        ClearBuffer::SavedLines => "\x1B[3J",
    ),
);
