use gpui::{
    DefiniteLength, FontStyle, FontWeight, HighlightStyle, SharedString, StrikethroughStyle,
    UnderlineStyle, px,
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
    Paragraph(MarkdownParagraph),
    HorizontalRule(Range<usize>),
    Image(Image),
}

impl ParsedMarkdownElement {
    pub fn source_range(&self) -> Option<Range<usize>> {
        Some(match self {
            Self::Heading(heading) => heading.source_range.clone(),
            Self::ListItem(list_item) => list_item.source_range.clone(),
            Self::Table(table) => table.source_range.clone(),
            Self::BlockQuote(block_quote) => block_quote.source_range.clone(),
            Self::CodeBlock(code_block) => code_block.source_range.clone(),
            Self::Paragraph(text) => match text.get(0)? {
                MarkdownParagraphChunk::Text(t) => t.source_range.clone(),
                MarkdownParagraphChunk::Image(image) => image.source_range.clone(),
            },
            Self::HorizontalRule(range) => range.clone(),
            Self::Image(image) => image.source_range.clone(),
        })
    }

    pub fn is_list_item(&self) -> bool {
        matches!(self, Self::ListItem(_))
    }
}

pub type MarkdownParagraph = Vec<MarkdownParagraphChunk>;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum MarkdownParagraphChunk {
    Text(ParsedMarkdownText),
    Image(Image),
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
    /// Whether we can expect nested list items inside of this items `content`.
    pub nested: bool,
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
    pub contents: MarkdownParagraph,
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
    pub header: Vec<ParsedMarkdownTableRow>,
    pub body: Vec<ParsedMarkdownTableRow>,
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ParsedMarkdownTableAlignment {
    #[default]
    None,
    Left,
    Center,
    Right,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdownTableColumn {
    pub col_span: usize,
    pub row_span: usize,
    pub is_header: bool,
    pub children: MarkdownParagraph,
    pub alignment: ParsedMarkdownTableAlignment,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdownTableRow {
    pub columns: Vec<ParsedMarkdownTableColumn>,
}

impl Default for ParsedMarkdownTableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ParsedMarkdownTableRow {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    pub fn with_columns(columns: Vec<ParsedMarkdownTableColumn>) -> Self {
        Self { columns }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ParsedMarkdownBlockQuote {
    pub source_range: Range<usize>,
    pub children: Vec<ParsedMarkdownElement>,
}

#[derive(Debug, Clone)]
pub struct ParsedMarkdownText {
    /// Where the text is located in the source Markdown document.
    pub source_range: Range<usize>,
    /// The text content stripped of any formatting symbols.
    pub contents: SharedString,
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

                if style.link {
                    highlight.underline = Some(UnderlineStyle {
                        thickness: px(1.),
                        ..Default::default()
                    });
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
    /// Whether the text should be stylized as link.
    pub link: bool,
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
            Link::Path { display_path, .. } => write!(f, "{}", display_path.display()),
        }
    }
}

/// A Markdown Image
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Image {
    pub link: Link,
    pub source_range: Range<usize>,
    pub alt_text: Option<SharedString>,
    pub width: Option<DefiniteLength>,
    pub height: Option<DefiniteLength>,
}

impl Image {
    pub fn identify(
        text: String,
        source_range: Range<usize>,
        file_location_directory: Option<PathBuf>,
    ) -> Option<Self> {
        let link = Link::identify(file_location_directory, text)?;
        Some(Self {
            source_range,
            link,
            alt_text: None,
            width: None,
            height: None,
        })
    }

    pub fn set_alt_text(&mut self, alt_text: SharedString) {
        self.alt_text = Some(alt_text);
    }

    pub fn set_width(&mut self, width: DefiniteLength) {
        self.width = Some(width);
    }

    pub fn set_height(&mut self, height: DefiniteLength) {
        self.height = Some(height);
    }
}
