//! The [Renderer] and its settings
//!
//! # Example
//!
//! ```
//! # use annotate_snippets::*;
//! # use annotate_snippets::renderer::*;
//! # use annotate_snippets::Level;
//! let report = // ...
//! # &[Group::with_title(
//! #     Level::ERROR
//! #         .primary_title("unresolved import `baz::zed`")
//! #         .id("E0432")
//! # )];
//!
//! let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
//! let output = renderer.render(report);
//! anstream::println!("{output}");
//! ```

pub(crate) mod render;
pub(crate) mod source_map;
pub(crate) mod stylesheet;

mod margin;
mod styled_buffer;

use alloc::string::String;

use crate::Report;

pub(crate) use render::normalize_whitespace;
pub(crate) use render::ElementStyle;
pub(crate) use render::UnderlineParts;
pub(crate) use render::{char_width, num_overlap, LineAnnotation, LineAnnotationType};
pub(crate) use stylesheet::Stylesheet;

pub use anstyle::*;

/// See [`Renderer::term_width`]
pub const DEFAULT_TERM_WIDTH: usize = 140;

const USE_WINDOWS_COLORS: bool = cfg!(windows) && !cfg!(feature = "testing-colors");
const BRIGHT_BLUE: Style = if USE_WINDOWS_COLORS {
    AnsiColor::BrightCyan.on_default()
} else {
    AnsiColor::BrightBlue.on_default()
};
/// [`Renderer::error`] applied by [`Renderer::styled`]
pub const DEFAULT_ERROR_STYLE: Style = AnsiColor::BrightRed.on_default().effects(Effects::BOLD);
/// [`Renderer::warning`] applied by [`Renderer::styled`]
pub const DEFAULT_WARNING_STYLE: Style = if USE_WINDOWS_COLORS {
    AnsiColor::BrightYellow.on_default()
} else {
    AnsiColor::Yellow.on_default()
}
.effects(Effects::BOLD);
/// [`Renderer::info`] applied by [`Renderer::styled`]
pub const DEFAULT_INFO_STYLE: Style = BRIGHT_BLUE.effects(Effects::BOLD);
/// [`Renderer::note`] applied by [`Renderer::styled`]
pub const DEFAULT_NOTE_STYLE: Style = AnsiColor::BrightGreen.on_default().effects(Effects::BOLD);
/// [`Renderer::help`] applied by [`Renderer::styled`]
pub const DEFAULT_HELP_STYLE: Style = AnsiColor::BrightCyan.on_default().effects(Effects::BOLD);
/// [`Renderer::line_num`] applied by [`Renderer::styled`]
pub const DEFAULT_LINE_NUM_STYLE: Style = BRIGHT_BLUE.effects(Effects::BOLD);
/// [`Renderer::emphasis`] applied by [`Renderer::styled`]
pub const DEFAULT_EMPHASIS_STYLE: Style = if USE_WINDOWS_COLORS {
    AnsiColor::BrightWhite.on_default()
} else {
    Style::new()
}
.effects(Effects::BOLD);
/// [`Renderer::none`] applied by [`Renderer::styled`]
pub const DEFAULT_NONE_STYLE: Style = Style::new();
/// [`Renderer::context`] applied by [`Renderer::styled`]
pub const DEFAULT_CONTEXT_STYLE: Style = BRIGHT_BLUE.effects(Effects::BOLD);
/// [`Renderer::addition`] applied by [`Renderer::styled`]
pub const DEFAULT_ADDITION_STYLE: Style = AnsiColor::BrightGreen.on_default();
/// [`Renderer::removal`] applied by [`Renderer::styled`]
pub const DEFAULT_REMOVAL_STYLE: Style = AnsiColor::BrightRed.on_default();

/// The [Renderer] for a [`Report`]
///
/// The caller is expected to detect any relevant terminal features and configure the renderer,
/// including
/// - ANSI Escape code support (always outputted with [`Renderer::styled`])
/// - Terminal width ([`Renderer::term_width`])
/// - Unicode support ([`Renderer::decor_style`])
///
/// # Example
///
/// ```
/// # use annotate_snippets::*;
/// # use annotate_snippets::renderer::*;
/// # use annotate_snippets::Level;
/// let report = // ...
/// # &[Group::with_title(
/// #     Level::ERROR
/// #         .primary_title("unresolved import `baz::zed`")
/// #         .id("E0432")
/// # )];
///
/// let renderer = Renderer::styled();
/// let output = renderer.render(report);
/// anstream::println!("{output}");
/// ```
#[derive(Clone, Debug)]
pub struct Renderer {
    anonymized_line_numbers: bool,
    term_width: usize,
    decor_style: DecorStyle,
    stylesheet: Stylesheet,
    short_message: bool,
}

impl Renderer {
    /// No terminal styling
    pub const fn plain() -> Self {
        Self {
            anonymized_line_numbers: false,
            term_width: DEFAULT_TERM_WIDTH,
            decor_style: DecorStyle::Ascii,
            stylesheet: Stylesheet::plain(),
            short_message: false,
        }
    }

    /// Default terminal styling
    ///
    /// If ANSI escape codes are not supported, either
    /// - Call [`Renderer::plain`] instead
    /// - Strip them after the fact, like with [`anstream`](https://docs.rs/anstream/latest/anstream/)
    ///
    /// # Note
    ///
    /// When testing styled terminal output, see the [`testing-colors` feature](crate#features)
    pub const fn styled() -> Self {
        Self {
            stylesheet: Stylesheet {
                error: DEFAULT_ERROR_STYLE,
                warning: DEFAULT_WARNING_STYLE,
                info: DEFAULT_INFO_STYLE,
                note: DEFAULT_NOTE_STYLE,
                help: DEFAULT_HELP_STYLE,
                line_num: DEFAULT_LINE_NUM_STYLE,
                emphasis: DEFAULT_EMPHASIS_STYLE,
                none: DEFAULT_NONE_STYLE,
                context: DEFAULT_CONTEXT_STYLE,
                addition: DEFAULT_ADDITION_STYLE,
                removal: DEFAULT_REMOVAL_STYLE,
            },
            ..Self::plain()
        }
    }

    /// Abbreviate the message
    pub const fn short_message(mut self, short_message: bool) -> Self {
        self.short_message = short_message;
        self
    }

    /// Set the width to render within
    ///
    /// Affects the rendering of [`Snippet`][crate::Snippet]s
    pub const fn term_width(mut self, term_width: usize) -> Self {
        self.term_width = term_width;
        self
    }

    /// Set the character set used for rendering decor
    pub const fn decor_style(mut self, decor_style: DecorStyle) -> Self {
        self.decor_style = decor_style;
        self
    }

    /// Anonymize line numbers
    ///
    /// When enabled, line numbers are replaced with `LL` which is useful for tests.
    ///
    /// # Example
    ///
    /// ```text
    ///   --> $DIR/whitespace-trimming.rs:4:193
    ///    |
    /// LL | ...                   let _: () = 42;
    ///    |                                   ^^ expected (), found integer
    ///    |
    /// ```
    pub const fn anonymized_line_numbers(mut self, anonymized_line_numbers: bool) -> Self {
        self.anonymized_line_numbers = anonymized_line_numbers;
        self
    }
}

impl Renderer {
    /// Render a diagnostic [`Report`]
    pub fn render(&self, groups: Report<'_>) -> String {
        render::render(self, groups)
    }
}

/// Customize [`Renderer::styled`]
impl Renderer {
    /// Override the output style for [error][crate::Level::ERROR]
    pub const fn error(mut self, style: Style) -> Self {
        self.stylesheet.error = style;
        self
    }

    /// Override the output style for [warnings][crate::Level::WARNING]
    pub const fn warning(mut self, style: Style) -> Self {
        self.stylesheet.warning = style;
        self
    }

    /// Override the output style for [info][crate::Level::INFO]
    pub const fn info(mut self, style: Style) -> Self {
        self.stylesheet.info = style;
        self
    }

    /// Override the output style for [notes][crate::Level::NOTE]
    pub const fn note(mut self, style: Style) -> Self {
        self.stylesheet.note = style;
        self
    }

    /// Override the output style for [help][crate::Level::HELP]
    pub const fn help(mut self, style: Style) -> Self {
        self.stylesheet.help = style;
        self
    }

    /// Override the output style for line numbers in the [`Snippet`][crate::Snippet] gutter
    pub const fn line_num(mut self, style: Style) -> Self {
        self.stylesheet.line_num = style;
        self
    }

    /// Override the output style for emphasis for the
    /// [`primary_title`][crate::Level::primary_title]
    pub const fn emphasis(mut self, style: Style) -> Self {
        self.stylesheet.emphasis = style;
        self
    }

    /// Override the output style for [`AnnotationKind::Context`][crate::AnnotationKind::Context]
    pub const fn context(mut self, style: Style) -> Self {
        self.stylesheet.context = style;
        self
    }

    /// Override the output style for [`Patch`][crate::Patch] additions
    pub const fn addition(mut self, style: Style) -> Self {
        self.stylesheet.addition = style;
        self
    }

    /// Override the output style for [`Patch`][crate::Patch] removals
    pub const fn removal(mut self, style: Style) -> Self {
        self.stylesheet.removal = style;
        self
    }

    /// Override the output style for all other text
    pub const fn none(mut self, style: Style) -> Self {
        self.stylesheet.none = style;
        self
    }
}

/// The character set for rendering for decor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecorStyle {
    Ascii,
    Unicode,
}

impl DecorStyle {
    fn col_separator(&self) -> char {
        match self {
            DecorStyle::Ascii => '|',
            DecorStyle::Unicode => '│',
        }
    }

    fn note_separator(&self, is_cont: bool) -> &str {
        match self {
            DecorStyle::Ascii => "= ",
            DecorStyle::Unicode if is_cont => "├ ",
            DecorStyle::Unicode => "╰ ",
        }
    }

    fn multi_suggestion_separator(&self) -> &'static str {
        match self {
            DecorStyle::Ascii => "|",
            DecorStyle::Unicode => "├╴",
        }
    }

    fn file_start(&self, is_first: bool, alone: bool) -> &'static str {
        match self {
            DecorStyle::Ascii => "--> ",
            DecorStyle::Unicode if is_first && alone => " ─▸ ",
            DecorStyle::Unicode if is_first => " ╭▸ ",
            DecorStyle::Unicode => " ├▸ ",
        }
    }

    fn secondary_file_start(&self) -> &'static str {
        match self {
            DecorStyle::Ascii => "::: ",
            DecorStyle::Unicode => " ⸬  ",
        }
    }

    fn diff(&self) -> char {
        match self {
            DecorStyle::Ascii => '~',
            DecorStyle::Unicode => '±',
        }
    }

    fn margin(&self) -> &'static str {
        match self {
            DecorStyle::Ascii => "...",
            DecorStyle::Unicode => "…",
        }
    }

    fn underline(&self, is_primary: bool) -> UnderlineParts {
        //               X0 Y0
        // label_start > ┯━━━━ < underline
        //               │ < vertical_text_line
        //               text

        //    multiline_start_down ⤷ X0 Y0
        //            top_left > ┌───╿──┘ < top_right_flat
        //           top_left > ┏│━━━┙ < top_right
        // multiline_vertical > ┃│
        //                      ┃│   X1 Y1
        //                      ┃│   X2 Y2
        //                      ┃└────╿──┘ < multiline_end_same_line
        //        bottom_left > ┗━━━━━┥ < bottom_right_with_text
        //   multiline_horizontal ^   `X` is a good letter

        // multiline_whole_line > ┏ X0 Y0
        //                        ┃   X1 Y1
        //                        ┗━━━━┛ < multiline_end_same_line

        // multiline_whole_line > ┏ X0 Y0
        //                        ┃ X1 Y1
        //                        ┃  ╿ < multiline_end_up
        //                        ┗━━┛ < bottom_right

        match (self, is_primary) {
            (DecorStyle::Ascii, true) => UnderlineParts {
                style: ElementStyle::UnderlinePrimary,
                underline: '^',
                label_start: '^',
                vertical_text_line: '|',
                multiline_vertical: '|',
                multiline_horizontal: '_',
                multiline_whole_line: '/',
                multiline_start_down: '^',
                bottom_right: '|',
                top_left: ' ',
                top_right_flat: '^',
                bottom_left: '|',
                multiline_end_up: '^',
                multiline_end_same_line: '^',
                multiline_bottom_right_with_text: '|',
            },
            (DecorStyle::Ascii, false) => UnderlineParts {
                style: ElementStyle::UnderlineSecondary,
                underline: '-',
                label_start: '-',
                vertical_text_line: '|',
                multiline_vertical: '|',
                multiline_horizontal: '_',
                multiline_whole_line: '/',
                multiline_start_down: '-',
                bottom_right: '|',
                top_left: ' ',
                top_right_flat: '-',
                bottom_left: '|',
                multiline_end_up: '-',
                multiline_end_same_line: '-',
                multiline_bottom_right_with_text: '|',
            },
            (DecorStyle::Unicode, true) => UnderlineParts {
                style: ElementStyle::UnderlinePrimary,
                underline: '━',
                label_start: '┯',
                vertical_text_line: '│',
                multiline_vertical: '┃',
                multiline_horizontal: '━',
                multiline_whole_line: '┏',
                multiline_start_down: '╿',
                bottom_right: '┙',
                top_left: '┏',
                top_right_flat: '┛',
                bottom_left: '┗',
                multiline_end_up: '╿',
                multiline_end_same_line: '┛',
                multiline_bottom_right_with_text: '┥',
            },
            (DecorStyle::Unicode, false) => UnderlineParts {
                style: ElementStyle::UnderlineSecondary,
                underline: '─',
                label_start: '┬',
                vertical_text_line: '│',
                multiline_vertical: '│',
                multiline_horizontal: '─',
                multiline_whole_line: '┌',
                multiline_start_down: '│',
                bottom_right: '┘',
                top_left: '┌',
                top_right_flat: '┘',
                bottom_left: '└',
                multiline_end_up: '│',
                multiline_end_same_line: '┘',
                multiline_bottom_right_with_text: '┤',
            },
        }
    }
}
