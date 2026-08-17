use std::fmt;

sequence!(
    /// Resets all attributes.
    /// 
    /// This sequence resets all attributes previously set by the:
    ///
    /// * [`SetAttribute`](struct.SetAttribute.html)
    /// * [`SetForegroundColor`](struct.SetBackgroundColor.html)
    /// * [`SetBackgroundColor`](struct.SetForegroundColor.html)
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::ResetAttributes;
    /// 
    /// let mut stdout = stdout();
    /// write!(stdout, "{}", ResetAttributes);
    /// ```
    struct ResetAttributes => sgr!("0")
);

/// A display attribute.
///
/// This is **NOT** a full ANSI sequence. `Attribute` must be used along with
/// the [`SetAttribute`](struct.SetAttribute.html).
///
/// # Examples
///
/// ```no_run
/// use std::io::{stdout, Write};
/// use anes::{Attribute, SetAttribute};
///
/// let mut stdout = stdout();
/// write!(stdout, "{}Bold text", SetAttribute(Attribute::Bold));
/// ```
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Attribute {
    /// Bold (increased) intensity.
    Bold = 1,
    /// Faint (decreased) intensity.
    Faint = 2,
    /// Normal intensity (turns off `Bold` and/or `Faint`).
    Normal = 22,

    /// Italic.
    Italic = 3,
    /// Turns off `Italic`.
    ItalicOff = 23,

    /// Underlined text.
    Underline = 4,
    /// Turns off `Underline`.
    UnderlineOff = 24,

    /// Blinking text.
    Blink = 5,
    /// Turns off blinking text (`Blink`).
    BlinkOff = 25,

    /// Reverse foreground & background colors.
    Reverse = 7,
    /// Turns off `Reverse`.
    ReverseOff = 27,

    /// Concealed (hidden).
    Conceal = 8,
    /// Turns off `Conceal`.
    ConcealOff = 28,

    /// Crossed.
    Crossed = 9,
    /// Turns off `Crossed`.
    CrossedOff = 29,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}

sequence!(
    /// Sets the display attribute.
    ///
    /// See the [`Attribute`](enum.Attribute.html) enum for a list of attributes you can (un)set.
    ///
    /// The [`ResetAttributes`](struct.ResetAttributes.html) sequence can be used to turn off all
    /// attributes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{stdout, Write};
    /// use anes::{Attribute, SetAttribute};
    ///
    /// let mut stdout = stdout();
    /// write!(stdout, "{}Blinking text", SetAttribute(Attribute::Blink));
    /// ```
    struct SetAttribute(Attribute) =>
    |this, f| write!(f, sgr!("{}"), this.0)
);

#[cfg(test)]
test_sequences!(
    set_attribute(
        SetAttribute(Attribute::Bold) => "\x1B[1m",
        SetAttribute(Attribute::Faint) => "\x1B[2m",
        SetAttribute(Attribute::Normal) => "\x1B[22m",

        SetAttribute(Attribute::Italic) => "\x1B[3m",
        SetAttribute(Attribute::ItalicOff) => "\x1B[23m",

        SetAttribute(Attribute::Underline) => "\x1B[4m",
        SetAttribute(Attribute::UnderlineOff) => "\x1B[24m",

        SetAttribute(Attribute::Blink) => "\x1B[5m",
        SetAttribute(Attribute::BlinkOff) => "\x1B[25m",

        SetAttribute(Attribute::Reverse) => "\x1B[7m",
        SetAttribute(Attribute::ReverseOff) => "\x1B[27m",

        SetAttribute(Attribute::Conceal) => "\x1B[8m",
        SetAttribute(Attribute::ConcealOff) => "\x1B[28m",

        SetAttribute(Attribute::Crossed) => "\x1B[9m",
        SetAttribute(Attribute::CrossedOff) => "\x1B[29m",
    ),
    reset_attributes(
        ResetAttributes => "\x1B[0m",
    )
);
