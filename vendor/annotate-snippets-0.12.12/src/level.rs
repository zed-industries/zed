//! [`Level`] constants for easy importing

use alloc::borrow::Cow;

use anstyle::Style;

use crate::renderer::stylesheet::Stylesheet;
use crate::snippet::{ERROR_TXT, HELP_TXT, INFO_TXT, NOTE_TXT, WARNING_TXT};
use crate::{Message, OptionCow, Title};

/// Default `error:` [`Level`]
pub const ERROR: Level<'_> = Level {
    name: None,
    level: LevelInner::Error,
};

/// Default `warning:` [`Level`]
pub const WARNING: Level<'_> = Level {
    name: None,
    level: LevelInner::Warning,
};

/// Default `info:` [`Level`]
pub const INFO: Level<'_> = Level {
    name: None,
    level: LevelInner::Info,
};

/// Default `note:` [`Level`]
pub const NOTE: Level<'_> = Level {
    name: None,
    level: LevelInner::Note,
};

/// Default `help:` [`Level`]
pub const HELP: Level<'_> = Level {
    name: None,
    level: LevelInner::Help,
};

/// Severity level for [`Title`]s and [`Message`]s
///
/// # Example
///
/// ```rust
/// # use annotate_snippets::*;
/// let report = &[
///     Level::ERROR.primary_title("mismatched types").id("E0308")
///         .element(Level::NOTE.message("expected reference")),
///     Group::with_title(
///         Level::HELP.secondary_title("function defined here")
///     ),
/// ];
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Level<'a> {
    pub(crate) name: Option<Option<Cow<'a, str>>>,
    pub(crate) level: LevelInner,
}

/// # Constructors
impl<'a> Level<'a> {
    pub const ERROR: Level<'a> = ERROR;
    pub const WARNING: Level<'a> = WARNING;
    pub const INFO: Level<'a> = INFO;
    pub const NOTE: Level<'a> = NOTE;
    pub const HELP: Level<'a> = HELP;
}

impl<'a> Level<'a> {
    /// For the primary, or root cause, [`Group`][crate::Group] (the first) in a [`Report`][crate::Report]
    ///
    /// See [`Group::with_title`][crate::Group::with_title]
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn primary_title(self, text: impl Into<Cow<'a, str>>) -> Title<'a> {
        Title {
            level: self,
            id: None,
            text: text.into(),
            allows_styling: false,
        }
    }

    /// For any secondary, or context, [`Group`][crate::Group]s (subsequent) in a [`Report`][crate::Report]
    ///
    /// See [`Group::with_title`][crate::Group::with_title]
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is allowed to be styled, as such all
    /// text is considered "trusted input" and has no normalizations applied to
    /// it. [`normalize_untrusted_str`](crate::normalize_untrusted_str) can be
    /// used to normalize untrusted text before it is passed to this function.
    ///
    /// </div>
    pub fn secondary_title(self, text: impl Into<Cow<'a, str>>) -> Title<'a> {
        Title {
            level: self,
            id: None,
            text: text.into(),
            allows_styling: true,
        }
    }

    /// A text [`Element`][crate::Element] in a [`Group`][crate::Group]
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is allowed to be styled, as such all
    /// text is considered "trusted input" and has no normalizations applied to
    /// it. [`normalize_untrusted_str`](crate::normalize_untrusted_str) can be
    /// used to normalize untrusted text before it is passed to this function.
    ///
    /// </div>
    pub fn message(self, text: impl Into<Cow<'a, str>>) -> Message<'a> {
        Message {
            level: self,
            text: text.into(),
        }
    }

    pub(crate) fn as_str(&'a self) -> &'a str {
        match (&self.name, self.level) {
            (Some(Some(name)), _) => name.as_ref(),
            (Some(None), _) => "",
            (None, LevelInner::Error) => ERROR_TXT,
            (None, LevelInner::Warning) => WARNING_TXT,
            (None, LevelInner::Info) => INFO_TXT,
            (None, LevelInner::Note) => NOTE_TXT,
            (None, LevelInner::Help) => HELP_TXT,
        }
    }

    pub(crate) fn style(&self, stylesheet: &Stylesheet) -> Style {
        self.level.style(stylesheet)
    }
}

/// # Customize the `Level`
impl<'a> Level<'a> {
    /// Replace the name describing this [`Level`]
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[allow(clippy::needless_doctest_main)]
    #[doc = include_str!("../examples/custom_level.rs")]
    /// ```
    #[doc = include_str!("../examples/custom_level.svg")]
    pub fn with_name(self, name: impl Into<OptionCow<'a>>) -> Level<'a> {
        Level {
            name: Some(name.into().0),
            level: self.level,
        }
    }

    /// Do not show the [`Level`]s name
    ///
    /// Useful for:
    /// - Another layer of the application will include the level (e.g. when rendering errors)
    /// - [`Message`]s that are part of a previous [`Group`][crate::Group] [`Element`][crate::Element]s
    ///
    /// # Example
    ///
    /// ```rust
    /// # use annotate_snippets::{Group, Snippet, AnnotationKind, Level};
    ///let source = r#"fn main() {
    ///     let b: &[u8] = include_str!("file.txt");    //~ ERROR mismatched types
    ///     let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
    /// }"#;
    /// let report = &[
    ///     Level::ERROR.primary_title("mismatched types").id("E0308")
    ///         .element(
    ///             Snippet::source(source)
    ///                 .path("$DIR/mismatched-types.rs")
    ///                 .annotation(
    ///                     AnnotationKind::Primary
    ///                         .span(105..131)
    ///                         .label("expected `&str`, found `&[u8; 0]`"),
    ///                 )
    ///                 .annotation(
    ///                     AnnotationKind::Context
    ///                         .span(98..102)
    ///                         .label("expected due to this"),
    ///                 ),
    ///         )
    ///         .element(
    ///             Level::NOTE
    ///                 .no_name()
    ///                 .message("expected reference `&str`\nfound reference `&'static [u8; 0]`"),
    ///         ),
    /// ];
    /// ```
    pub fn no_name(self) -> Level<'a> {
        self.with_name(None::<&str>)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LevelInner {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

impl LevelInner {
    pub(crate) fn style(self, stylesheet: &Stylesheet) -> Style {
        match self {
            LevelInner::Error => stylesheet.error,
            LevelInner::Warning => stylesheet.warning,
            LevelInner::Info => stylesheet.info,
            LevelInner::Note => stylesheet.note,
            LevelInner::Help => stylesheet.help,
        }
    }
}
