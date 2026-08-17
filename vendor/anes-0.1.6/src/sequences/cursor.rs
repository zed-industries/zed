//! A terminal cursor related ANSI escape sequences.

sequence!(
    /// Saves the cursor position.
    /// 
    /// Use the [`RestoreCursorPosition`](struct.RestoreCursorPosition.html) sequence to
    /// restore the cursor position.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::{SaveCursorPosition, RestoreCursorPosition};
    ///
    /// let mut stdout = stdout();
    /// // Save cursor position
    /// write!(stdout, "{}", SaveCursorPosition);
    /// 
    /// // Your app
    /// 
    /// // Restore cursor position
    /// write!(stdout, "{}", RestoreCursorPosition);
    /// ```    
    struct SaveCursorPosition => esc!("7")    
);

sequence!(
    /// Restores the cursor position.
    /// 
    /// Use the [`SaveCursorPosition`](struct.SaveCursorPosition.html) sequence to
    /// save the cursor position.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::{SaveCursorPosition, RestoreCursorPosition};
    ///
    /// let mut stdout = stdout();
    /// // Save cursor position
    /// write!(stdout, "{}", SaveCursorPosition);
    /// 
    /// // Your app
    /// 
    /// // Restore cursor position
    /// write!(stdout, "{}", RestoreCursorPosition);
    /// ```
    struct RestoreCursorPosition => esc!("8")
);

sequence!(
    /// Hides the cursor.
    /// 
    /// Use the [`ShowCursor`](struct.ShowCursor.html) sequence to show the cursor.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::HideCursor;
    ///
    /// let mut stdout = stdout();
    /// // Hide cursor
    /// write!(stdout, "{}", HideCursor);
    /// ```
    struct HideCursor => csi!("?25l")
);

sequence!(
    /// Shows the cursor.
    /// 
    /// Use the [`HideCursor`](struct.HideCursor.html) sequence to hide the cursor.    
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ShowCursor;
    ///
    /// let mut stdout = stdout();
    /// // Show cursor
    /// write!(stdout, "{}", ShowCursor);
    /// ```
    struct ShowCursor => csi!("?25h")
);

sequence!(
    /// Enables the cursor blinking.
    /// 
    /// Use the [`DisableCursorBlinking`](struct.DisableCursorBlinking.html) sequence to disable
    /// cursor blinking.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::EnableCursorBlinking;
    ///
    /// let mut stdout = stdout();
    /// // Enable cursor blinking
    /// write!(stdout, "{}", EnableCursorBlinking);
    /// ```
    struct EnableCursorBlinking => csi!("?12h")
);

sequence!(
    /// Disables the cursor blinking.
    /// 
    /// Use the [`EnableCursorBlinking`](struct.EnableCursorBlinking.html) sequence to enable
    /// cursor blinking.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::DisableCursorBlinking;
    ///
    /// let mut stdout = stdout();
    /// // Disable cursor blinking
    /// write!(stdout, "{}", DisableCursorBlinking);
    /// ```
    struct DisableCursorBlinking => csi!("?12l")
);

sequence!(
    /// Moves the cursor to the given location (column, row).
    ///
    /// # Notes
    ///
    /// Top/left cell is represented as `1, 1` (`column, row`).
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::MoveCursorTo;
    ///
    /// let mut stdout = stdout();
    /// // Move cursor to top left cell
    /// write!(stdout, "{}", MoveCursorTo(1, 1));
    /// ```
    struct MoveCursorTo(u16, u16) =>
    |this, f| write!(f, csi!("{};{}H"), this.1, this.0)
);

sequence!(
    /// Moves the cursor up by the given number of rows.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::MoveCursorUp;
    ///
    /// let mut stdout = stdout();
    /// // Move cursor up by 5 rows
    /// write!(stdout, "{}", MoveCursorUp(5));
    /// ```
    struct MoveCursorUp(u16) =>
    |this, f| write!(f, csi!("{}A"), this.0)
);

sequence!(
    /// Moves the cursor down by the given number of rows.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::MoveCursorDown;
    ///
    /// let mut stdout = stdout();
    /// // Move cursor down by 5 rows
    /// write!(stdout, "{}", MoveCursorDown(5));
    /// ```
    struct MoveCursorDown(u16) =>
    |this, f| write!(f, csi!("{}B"), this.0)
);

sequence!(
    /// Moves the cursor right by the given number of columns.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::MoveCursorRight;
    ///
    /// let mut stdout = stdout();
    /// // Move cursor right by 5 columns
    /// write!(stdout, "{}", MoveCursorRight(5));
    /// ```
    struct MoveCursorRight(u16) =>
    |this, f| write!(f, csi!("{}C"), this.0)
);

sequence!(
    /// Moves the cursor left by the given number of columns.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::MoveCursorLeft;
    ///
    /// let mut stdout = stdout();
    /// // Move cursor left by 5 columns
    /// write!(stdout, "{}", MoveCursorLeft(5));
    /// ```
    struct MoveCursorLeft(u16) =>
    |this, f| write!(f, csi!("{}D"), this.0)
);

sequence!(
    /// Moves the cursor to beginning of line the given number of lines down.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::MoveCursorToNextLine;
    ///
    /// let mut stdout = stdout();
    /// // Move cursor down by 2 rows and the move it to the first column
    /// write!(stdout, "{}", MoveCursorToNextLine(2));
    /// ```
    /// 
    /// The previous example does the same thing as the following one:
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::{MoveCursorDown, MoveCursorToColumn};
    ///
    /// let mut stdout = stdout();
    /// write!(stdout, "{}{}", MoveCursorDown(2), MoveCursorToColumn(1));
    /// ```
    struct MoveCursorToNextLine(u16) =>
    |this, f| write!(f, csi!("{}E"), this.0)
);

sequence!(
    /// Moves the cursor to beginning of line the given number of lines up.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::MoveCursorToPreviousLine;
    ///
    /// let mut stdout = stdout();
    /// // Move cursor up by 2 rows and the move it to the first column
    /// write!(stdout, "{}", MoveCursorToPreviousLine(2));
    /// ```
    /// 
    /// The previous example does the same thing as the following one:
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::{MoveCursorUp, MoveCursorToColumn};
    ///
    /// let mut stdout = stdout();
    /// write!(stdout, "{}{}", MoveCursorUp(2), MoveCursorToColumn(1));
    /// ```
    struct MoveCursorToPreviousLine(u16) =>
    |this, f| write!(f, csi!("{}F"), this.0)
);

sequence!(
    /// Moves the cursor to the given column.
    ///
    /// # Notes
    ///
    /// Beginning of the line (left cell) is represented as `1`.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::MoveCursorToColumn;
    ///
    /// let mut stdout = stdout();
    /// // Move cursor to the 10th column (same row)
    /// write!(stdout, "{}", MoveCursorToColumn(10));
    /// ```
    struct MoveCursorToColumn(u16) =>
    |this, f| write!(f, csi!("{}G"), this.0)
);

// TODO Enhance example with Parser to show how to retrieve it
sequence!(
    /// Asks for the current cursor position.
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ReportCursorPosition;
    ///
    /// let mut stdout = stdout();
    /// write!(stdout, "{}", ReportCursorPosition);
    /// ```
    struct ReportCursorPosition => csi!("6n")
);

#[cfg(test)]
test_sequences!(
    save_cursor_position(
        SaveCursorPosition => "\x1B7",
    ),
    restore_cursor_position(
        RestoreCursorPosition => "\x1B8",
    ),
    hide_cursor(
        HideCursor => "\x1B[?25l",
    ),
    show_cursor(
        ShowCursor => "\x1B[?25h",
    ),
    disable_cursor_blinking(
        DisableCursorBlinking => "\x1B[?12l",
    ),
    enable_cursor_blinking(
        EnableCursorBlinking => "\x1B[?12h",
    ),
    move_cursor_up(
        MoveCursorUp(10) => "\x1B[10A",
    ),
    move_cursor_down(
        MoveCursorDown(10) => "\x1B[10B",
    ),
    move_cursor_right(
        MoveCursorRight(10) => "\x1B[10C",
    ),
    move_cursor_left(
        MoveCursorLeft(10) => "\x1B[10D",
    ),
    move_cursor_to(
        MoveCursorTo(5, 10) => "\x1B[10;5H",
    ),
    move_cursor_to_next_line(
        MoveCursorToNextLine(5) => "\x1B[5E",
    ),
    move_cursor_to_previous_line(
        MoveCursorToPreviousLine(5) => "\x1B[5F",
    ),
    move_cursor_to_column(
        MoveCursorToColumn(1) => "\x1B[1G",
    ),
    report_cursor_position(
        ReportCursorPosition => "\x1B[6n",
    )
);
