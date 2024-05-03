use std::ops::Range;

use pulldown_cmark::{LinkType, MetadataBlockKind, TagEnd};
use ui::SharedString;

/// A static-lifetime equivalent of pulldown_cmark::Event so we can cache the
/// parse result for rendering without resorting to unsafe lifetime coercion.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// Start of a tagged element. Events that are yielded after this event
    /// and before its corresponding `End` event are inside this element.
    /// Start and end events are guaranteed to be balanced.
    Start(Tag),
    /// End of a tagged element.
    End(TagEnd),
    /// A text node.
    Text(Range<usize>),
    /// An inline code node.
    Code(Range<usize>),
    /// An HTML node.
    Html(Range<usize>),
    /// An inline HTML node.
    InlineHtml(Range<usize>),
    /// A reference to a footnote with given label, which may or may not be defined
    /// by an event with a `Tag::FootnoteDefinition` tag. Definitions and references to them may
    /// occur in any order.
    FootnoteReference(Range<usize>),
    /// A soft line break.
    SoftBreak,
    /// A hard line break.
    HardBreak,
    /// A horizontal ruler.
    Rule,
    /// A task list marker, rendered as a checkbox in HTML. Contains a true when it is checked.
    TaskListMarker(bool),
}

impl From<pulldown_cmark::Event<'_>> for Event {
    fn from(event: pulldown_cmark::Event) -> Self {
        match event {
            pulldown_cmark::Event::Start(tag) => Event::Start(tag.into()),
            pulldown_cmark::Event::End(tag) => Event::End(TagEnd::from(tag)),
            pulldown_cmark::Event::Text(text) => Event::Text(text.into()),
            pulldown_cmark::Event::Code(code) => Event::Code(code.into()),
            pulldown_cmark::Event::Html(html) => Event::Html(html.into()),
            pulldown_cmark::Event::InlineHtml(inline_html) => Event::InlineHtml(inline_html.into()),
            pulldown_cmark::Event::FootnoteReference(footnote) => {
                Event::FootnoteReference(footnote.into())
            }
            pulldown_cmark::Event::SoftBreak => Event::SoftBreak,
            pulldown_cmark::Event::HardBreak => Event::HardBreak,
            pulldown_cmark::Event::Rule => Event::Rule,
            pulldown_cmark::Event::TaskListMarker(checked) => Event::TaskListMarker(checked),
        }
    }
}

/// Tags for elements that can contain other elements.
#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    /// A paragraph of text and other inline elements.
    Paragraph,

    /// A heading, with optional identifier, classes and custom attributes.
    /// The identifier is prefixed with `#` and the last one in the attributes
    /// list is chosen, classes are prefixed with `.` and custom attributes
    /// have no prefix and can optionally have a value (`myattr` o `myattr=myvalue`).
    Heading {
        level: HeadingLevel,
        id: Option<SharedString>,
        classes: Vec<SharedString>,
        /// The first item of the tuple is the attr and second one the value.
        attrs: Vec<(SharedString, Option<SharedString>)>,
    },

    BlockQuote,

    /// A code block.
    CodeBlock(CodeBlockKind),

    /// A HTML block.
    HtmlBlock,

    /// A list. If the list is ordered the field indicates the number of the first item.
    /// Contains only list items.
    List(Option<u64>), // TODO: add delim and tight for ast (not needed for html)

    /// A list item.
    Item,

    /// A footnote definition. The value contained is the footnote's label by which it can
    /// be referred to.
    #[cfg_attr(feature = "serde", serde(borrow))]
    FootnoteDefinition(SharedString),

    /// A table. Contains a vector describing the text-alignment for each of its columns.
    Table(Vec<Alignment>),

    /// A table header. Contains only `TableCell`s. Note that the table body starts immediately
    /// after the closure of the `TableHead` tag. There is no `TableBody` tag.
    TableHead,

    /// A table row. Is used both for header rows as body rows. Contains only `TableCell`s.
    TableRow,
    TableCell,

    // span-level tags
    Emphasis,
    Strong,
    Strikethrough,

    /// A link.
    Link {
        link_type: LinkType,
        dest_url: SharedString,
        title: SharedString,
        /// Identifier of reference links, e.g. `world` in the link `[hello][world]`.
        id: SharedString,
    },

    /// An image. The first field is the link type, the second the destination URL and the third is a title,
    /// the fourth is the link identifier.
    Image {
        link_type: LinkType,
        dest_url: SharedString,
        title: SharedString,
        /// Identifier of reference links, e.g. `world` in the link `[hello][world]`.
        id: SharedString,
    },

    /// A metadata block.
    MetadataBlock(MetadataBlockKind),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CodeBlockKind {
    Indented,
    /// The value contained in the tag describes the language of the code, which may be empty.
    Fenced(SharedString),
}

impl From<pulldown_cmark::Tag<'_>> for Tag {
    fn from(tag: pulldown_cmark::Tag) -> Self {
        match tag {
            pulldown_cmark::Tag::Paragraph => Tag::Paragraph,
            pulldown_cmark::Tag::Heading {
                level,
                id,
                classes,
                attrs,
            } => {
                let id = id.map(|id| SharedString::from(id.into_string()));
                let classes = classes
                    .into_iter()
                    .map(|c| SharedString::from(c.into_string()))
                    .collect();
                let attrs = attrs
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            SharedString::from(key.into_string()),
                            value.map(|v| SharedString::from(v.into_string())),
                        )
                    })
                    .collect();
                Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                }
            }
            pulldown_cmark::Tag::BlockQuote => Tag::BlockQuote,
            pulldown_cmark::Tag::CodeBlock(kind) => match kind {
                pulldown_cmark::CodeBlockKind::Indented => Tag::CodeBlock(CodeBlockKind::Indented),
                pulldown_cmark::CodeBlockKind::Fenced(info) => Tag::CodeBlock(
                    CodeBlockKind::Fenced(SharedString::from(info.into_string())),
                ),
            },
            pulldown_cmark::Tag::List(start_number) => Tag::List(start_number),
            pulldown_cmark::Tag::Item => Tag::Item,
            pulldown_cmark::Tag::FootnoteDefinition(label) => {
                Tag::FootnoteDefinition(SharedString::from(label.to_string()))
            }
            pulldown_cmark::Tag::Table(alignments) => Tag::Table(alignments),
            pulldown_cmark::Tag::TableHead => Tag::TableHead,
            pulldown_cmark::Tag::TableRow => Tag::TableRow,
            pulldown_cmark::Tag::TableCell => Tag::TableCell,
            pulldown_cmark::Tag::Emphasis => Tag::Emphasis,
            pulldown_cmark::Tag::Strong => Tag::Strong,
            pulldown_cmark::Tag::Strikethrough => Tag::Strikethrough,
            pulldown_cmark::Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            } => Tag::Link {
                link_type,
                dest_url: SharedString::from(dest_url.into_string()),
                title: SharedString::from(title.into_string()),
                id: SharedString::from(id.into_string()),
            },
            pulldown_cmark::Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            } => Tag::Image {
                link_type,
                dest_url: SharedString::from(dest_url.into_string()),
                title: SharedString::from(title.into_string()),
                id: SharedString::from(id.into_string()),
            },
            pulldown_cmark::Tag::HtmlBlock => Tag::HtmlBlock,
            pulldown_cmark::Tag::MetadataBlock(kind) => Tag::MetadataBlock(kind),
        }
    }
}
