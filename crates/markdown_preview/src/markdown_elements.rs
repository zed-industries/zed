use gpui::{
    px, FontStyle, FontWeight, HighlightStyle, SharedString, StrikethroughStyle, UnderlineStyle,
};
use language::HighlightId;
use std::{fmt::Display, ops::Range, path::PathBuf};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ParsedMarkdownElement {
    Heading(ParsedMarkdownHeading),
    ListItem(ParsedMarkdownListItem),
    Table(ParsedMarkdownTable),
    BlockQuote(ParsedMarkdownBlockQuote),
    CodeBlock(ParsedMarkdownCodeBlock),
    /// A paragraph of text and other inline elements.
    Paragraph(ParsedMarkdownText),
    HorizontalRule(Range<usize>),
}

impl ParsedMarkdownElement {
    pub fn source_range(&self) -> Range<usize> {
        match self {
            Self::Heading(heading) => heading.source_range.clone(),
            Self::ListItem(list_item) => list_item.source_range.clone(),
            Self::Table(table) => table.source_range.clone(),
            Self::BlockQuote(block_quote) => block_quote.source_range.clone(),
            Self::CodeBlock(code_block) => code_block.source_range.clone(),
            Self::Paragraph(text) => text.source_range.clone(),
            Self::HorizontalRule(range) => range.clone(),
        }
    }

    pub fn is_list_item(&self) -> bool {
        matches!(self, Self::ListItem(_))
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdown {
    pub children: Vec<ParsedMarkdownElement>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdownListItem {
    pub source_range: Range<usize>,
    /// How many indentations deep this item is.
    pub depth: u16,
    pub item_type: ParsedMarkdownListItemType,
    pub content: Vec<ParsedMarkdownElement>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ParsedMarkdownListItemType {
    Ordered(u64),
    Task(bool, Range<usize>),
    Unordered,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdownCodeBlock {
    pub source_range: Range<usize>,
    pub language: Option<String>,
    pub contents: SharedString,
    pub highlights: Option<Vec<(Range<usize>, HighlightId)>>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdownHeading {
    pub source_range: Range<usize>,
    pub level: HeadingLevel,
    pub contents: ParsedMarkdownText,
}

#[derive(Debug, PartialEq)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug)]
pub struct ParsedMarkdownTable {
    pub source_range: Range<usize>,
    pub header: ParsedMarkdownTableRow,
    pub body: Vec<ParsedMarkdownTableRow>,
    pub column_alignments: Vec<ParsedMarkdownTableAlignment>,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ParsedMarkdownTableAlignment {
    /// Default text alignment.
    None,
    Left,
    Center,
    Right,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdownTableRow {
    pub children: Vec<ParsedMarkdownText>,
}

impl Default for ParsedMarkdownTableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ParsedMarkdownTableRow {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_children(children: Vec<ParsedMarkdownText>) -> Self {
        Self { children }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdownBlockQuote {
    pub source_range: Range<usize>,
    pub children: Vec<ParsedMarkdownElement>,
}

#[derive(Debug)]
pub struct ParsedMarkdownText {
    /// Where the text is located in the source Markdown document.
    pub source_range: Range<usize>,
    /// The text content stripped of any formatting symbols.
    pub contents: String,
    /// The list of highlights contained in the Markdown document.
    pub highlights: Vec<(Range<usize>, MarkdownHighlight)>,
    /// The regions of the various ranges in the Markdown document.
    pub region_ranges: Vec<Range<usize>>,
    /// The regions of the Markdown document.
    pub regions: Vec<ParsedRegion>,
}

/// A run of highlighted Markdown text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownHighlight {
    /// A styled Markdown highlight.
    Style(MarkdownHighlightStyle),
    /// A highlighted code block.
    Code(HighlightId),
}

impl MarkdownHighlight {
    /// Converts this [`MarkdownHighlight`] to a [`HighlightStyle`].
    pub fn to_highlight_style(&self, theme: &theme::SyntaxTheme) -> Option<HighlightStyle> {
        match self {
            MarkdownHighlight::Style(style) => {
                let mut highlight = HighlightStyle::default();

                if style.italic {
                    highlight.font_style = Some(FontStyle::Italic);
                }

                if style.underline {
                    highlight.underline = Some(UnderlineStyle {
                        thickness: px(1.),
                        ..Default::default()
                    });
                }

                if style.strikethrough {
                    highlight.strikethrough = Some(StrikethroughStyle {
                        thickness: px(1.),
                        ..Default::default()
                    });
                }

                if style.weight != FontWeight::default() {
                    highlight.font_weight = Some(style.weight);
                }

                Some(highlight)
            }

            MarkdownHighlight::Code(id) => id.style(theme),
        }
    }
}

/// The style for a Markdown highlight.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MarkdownHighlightStyle {
    /// Whether the text should be italicized.
    pub italic: bool,
    /// Whether the text should be underlined.
    pub underline: bool,
    /// Whether the text should be struck through.
    pub strikethrough: bool,
    /// The weight of the text.
    pub weight: FontWeight,
}

/// A parsed region in a Markdown document.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedRegion {
    /// Whether the region is a code block.
    pub code: bool,
    /// The link contained in this region, if it has one.
    pub link: Option<Link>,
}

/// A Markdown link.
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Link {
    /// A link to a webpage.
    Web {
        /// The URL of the webpage.
        url: String,
    },
    /// A link to a path on the filesystem.
    Path {
        /// The path as provided in the Markdown document.
        display_path: PathBuf,
        /// The absolute path to the item.
        path: PathBuf,
    },
}

impl Link {
    pub fn identify(file_location_directory: Option<PathBuf>, text: String) -> Option<Link> {
        if text.starts_with("http") {
            return Some(Link::Web { url: text });
        }

        let path = PathBuf::from(&text);
        if path.is_absolute() && path.exists() {
            return Some(Link::Path {
                display_path: path.clone(),
                path,
            });
        }

        if let Some(file_location_directory) = file_location_directory {
            let display_path = path;
            let path = file_location_directory.join(text);
            if path.exists() {
                return Some(Link::Path { display_path, path });
            }
        }

        None
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Link::Web { url } => write!(f, "{}", url),
            Link::Path {
                display_path,
                path: _,
            } => write!(f, "{}", display_path.display()),
        }
    }
}
