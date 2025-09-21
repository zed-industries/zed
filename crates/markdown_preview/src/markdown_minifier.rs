use html5ever::{
    Attribute, ParseOpts, QualName, parse_document,
    tendril::{Tendril, TendrilSink, fmt::UTF8},
};
use markup5ever_rcdom::{Node, NodeData, RcDom};
use std::{cell::RefCell, io, rc::Rc, str};

/// Defines the minify trait.
#[allow(dead_code)]
pub(crate) trait Minify {
    /// Minifies the source returning the minified HTML5.
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to read from the input reader or unable to
    /// write to the output writer.
    fn minify(&self) -> Result<Vec<u8>, io::Error>;
}

/// Minifies the HTML input to the destination writer.
/// Outputs HTML5; non-HTML5 input will be transformed to HTML5.
///
/// # Errors
///
/// Will return `Err` if unable to read from the input reader or unable to write
/// to the output writer.
#[inline]
#[allow(dead_code)]
pub(crate) fn minify<R: io::Read, W: io::Write>(mut r: &mut R, w: &mut W) -> io::Result<()> {
    Minifier::new(w).minify(&mut r)
}

impl<T> Minify for T
where
    T: AsRef<[u8]>,
{
    #[inline]
    fn minify(&self) -> Result<Vec<u8>, io::Error> {
        let mut minified = vec![];

        minify(&mut self.as_ref(), &mut minified)?;

        Ok(minified)
    }
}

/// Minifier implementation for `io::Write`.
#[allow(clippy::struct_excessive_bools)]
pub struct Minifier<'a, W: io::Write> {
    w: &'a mut W,
    omit_doctype: bool,
    collapse_whitespace: bool,
    preserve_comments: bool,
    preceding_whitespace: bool,
}

/// Holds node positional context.
struct Context<'a> {
    parent: &'a Node,
    parent_context: Option<&'a Context<'a>>,
    left: Option<&'a [Rc<Node>]>,
    right: Option<&'a [Rc<Node>]>,
}

impl<'a> Context<'a> {
    /// Determine whether to trim whitespace.
    /// Uses naive HTML5 whitespace collapsing rules.
    fn trim(&self, preceding_whitespace: bool) -> (bool, bool) {
        (preceding_whitespace || self.trim_left(), self.trim_right())
    }

    fn trim_left(&self) -> bool {
        self.left.map_or_else(
            || is_block_element(self.parent) || self.parent_trim_left(),
            |siblings| {
                siblings
                    .iter()
                    .rev()
                    .find_map(Self::is_block_element)
                    .unwrap_or_else(|| self.parent_trim_left())
            },
        )
    }

    fn parent_trim_left(&self) -> bool {
        self.parent_context.map_or(true, Context::trim_left)
    }

    fn trim_right(&self) -> bool {
        self.right.map_or(true, |siblings| {
            siblings
                .iter()
                .find_map(Self::is_block_element)
                .unwrap_or(true)
        })
    }

    fn next_element(&self) -> Option<&Rc<Node>> {
        self.right.and_then(|siblings| {
            siblings
                .iter()
                .find(|node| matches!(node.data, NodeData::Element { .. }))
        })
    }

    fn is_block_element(node: &Rc<Node>) -> Option<bool> {
        if let NodeData::Element { name, .. } = &node.data {
            Some(is_block_element_name(name.local.as_ref()))
        } else {
            None
        }
    }
}

impl<'a, W> Minifier<'a, W>
where
    W: io::Write,
{
    /// Creates a new `Minifier` instance.
    #[inline]
    pub fn new(w: &'a mut W) -> Self {
        Self {
            w,
            omit_doctype: false,
            collapse_whitespace: true,
            preserve_comments: false,
            preceding_whitespace: false,
        }
    }

    /// Collapse whitespace between elements and in text when whitespace isn't preserved by default.
    /// Enabled by default.
    #[inline]
    #[allow(dead_code)]
    pub fn collapse_whitespace(&mut self, collapse: bool) -> &mut Self {
        self.collapse_whitespace = collapse;
        self
    }

    /// Omit writing the HTML5 doctype.
    /// Disabled by default.
    #[inline]
    #[allow(dead_code)]
    pub fn omit_doctype(&mut self, omit: bool) -> &mut Self {
        self.omit_doctype = omit;
        self
    }

    /// Preserve HTML comments.
    /// Disabled by default.
    #[inline]
    #[allow(dead_code)]
    pub fn preserve_comments(&mut self, preserve: bool) -> &mut Self {
        self.preserve_comments = preserve;
        self
    }

    /// Minifies the given reader input.
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to write to the output writer.
    #[inline]
    #[allow(dead_code)]
    pub fn minify<R: io::Read>(&mut self, mut r: &mut R) -> io::Result<()> {
        let dom = parse_document(RcDom::default(), ParseOpts::default())
            .from_utf8()
            .read_from(&mut r)?;

        if !self.omit_doctype {
            self.w.write_all(b"<!doctype html>")?;
        }

        self.minify_node(&None, &dom.document)
    }

    fn minify_node<'b>(&mut self, ctx: &'b Option<Context>, node: &'b Node) -> io::Result<()> {
        match &node.data {
            NodeData::Text { contents } => {
                // Check if whitespace collapsing disabled
                let contents = contents.borrow();
                let contents = contents.as_ref();

                if !self.collapse_whitespace {
                    return self.w.write_all(contents.as_bytes());
                }

                // Check if parent is whitespace preserving element or contains code (<script>, <style>)
                let (skip_collapse_whitespace, contains_code) =
                    ctx.as_ref().map_or((false, false), |ctx| {
                        if let NodeData::Element { name, .. } = &ctx.parent.data {
                            let name = name.local.as_ref();

                            (preserve_whitespace(name), contains_code(name))
                        } else {
                            (false, false)
                        }
                    });

                if skip_collapse_whitespace {
                    return self.w.write_all(contents.as_bytes());
                }

                if contains_code {
                    return self
                        .w
                        .write_all(contents.trim_matches(is_ascii_whitespace).as_bytes());
                }

                // Early exit if empty to forego expensive trim logic
                if contents.is_empty() {
                    return io::Result::Ok(());
                }

                let (trim_left, trim_right) = ctx
                    .as_ref()
                    .map_or((true, true), |ctx| ctx.trim(self.preceding_whitespace));
                let contents = match (trim_left, trim_right) {
                    (true, true) => contents.trim_matches(is_ascii_whitespace),
                    (true, false) => contents.trim_start_matches(is_ascii_whitespace),
                    (false, true) => contents.trim_end_matches(is_ascii_whitespace),
                    _ => contents,
                };

                // Second empty check after trimming whitespace
                if !contents.is_empty() {
                    // replace \n, \r to ' '
                    let contents = contents
                        .bytes()
                        .map(|c| if matches!(c, b'\n' | b'\r') { b' ' } else { c })
                        .collect::<Vec<u8>>();

                    self.write_collapse_whitespace(&contents, reserved_entity, None)?;

                    self.preceding_whitespace = !trim_right
                        && contents
                            .iter()
                            .last()
                            .map_or(false, u8::is_ascii_whitespace);
                }

                Ok(())
            }

            NodeData::Comment { contents } if self.preserve_comments => {
                self.w.write_all(b"<!--")?;
                self.w.write_all(contents.as_bytes())?;
                self.w.write_all(b"-->")
            }

            NodeData::Document => self.minify_children(ctx, node),

            NodeData::Element { name, attrs, .. } => {
                let attrs = attrs.borrow();
                let tag = name.local.as_ref();

                if is_self_closing(tag) {
                    return self.write_start_tag(name, &attrs);
                }

                let (omit_start_tag, omit_end_tag) =
                    self.omit_tags(ctx, node, tag, attrs.is_empty());

                if !omit_start_tag {
                    self.write_start_tag(name, &attrs)?;
                }

                self.minify_children(ctx, node)?;

                if !omit_end_tag {
                    self.write_end_tag(name)?;
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }

    fn next_is_comment<'b, I>(&self, v: I) -> bool
    where
        I: IntoIterator<Item = &'b Rc<Node>>,
    {
        v.into_iter()
            .find_map(|node| match &node.data {
                NodeData::Text { contents } => {
                    if self.collapse_whitespace && is_whitespace(contents) {
                        // Blocks of whitespace are skipped
                        None
                    } else {
                        Some(false)
                    }
                }
                NodeData::Comment { .. } => Some(self.preserve_comments),
                _ => Some(false),
            })
            .unwrap_or(false)
    }

    fn is_whitespace(&self, s: &RefCell<Tendril<UTF8>>) -> Option<bool> {
        if self.collapse_whitespace && is_whitespace(s) {
            None
        } else {
            Some(
                !s.borrow()
                    .as_bytes()
                    .iter()
                    .next()
                    .map_or(false, u8::is_ascii_whitespace),
            )
        }
    }

    /// Determines if start and end tags can be omitted.
    /// Whitespace rules are ignored if `collapse_whitespace` is enabled.
    #[allow(clippy::too_many_lines)]
    fn omit_tags(
        &self,
        ctx: &Option<Context>,
        node: &Node,
        name: &str,
        empty_attributes: bool,
    ) -> (bool, bool) {
        ctx.as_ref().map_or((false, false), |ctx| match name {
            "html" => {
                // The end tag may be omitted if the <html> element is not immediately followed by a comment.
                let omit_end = ctx.right.map_or(true, |right| !self.next_is_comment(right));
                // The start tag may be omitted if the first thing inside the <html> element is not a comment.
                let omit_start =
                    empty_attributes && omit_end && !self.next_is_comment(&*node.children.borrow());

                (omit_start, omit_end)
            }
            "head" => {
                // The end tag may be omitted if the first thing following the <head> element is not a space character or a comment.
                let omit_end = ctx.right.map_or(true, |right| {
                    right
                        .iter()
                        .find_map(|node| match &node.data {
                            NodeData::Text { contents } => self.is_whitespace(contents),
                            NodeData::Comment { .. } => {
                                if self.preserve_comments {
                                    Some(false)
                                } else {
                                    None
                                }
                            }
                            _ => Some(true),
                        })
                        .unwrap_or(true)
                });
                // The start tag may be omitted if the first thing inside the <head> element is an element.
                let omit_start = empty_attributes
                    && omit_end
                    && node
                        .children
                        .borrow()
                        .iter()
                        .find_map(|node| match &node.data {
                            NodeData::Text { contents } => self.is_whitespace(contents),
                            NodeData::Element { .. } => Some(true),
                            NodeData::Comment { .. } => {
                                if self.preserve_comments {
                                    Some(false)
                                } else {
                                    None
                                }
                            }
                            _ => Some(false),
                        })
                        .unwrap_or(true);

                (omit_start, omit_end)
            }
            "body" => {
                // The start tag may be omitted if the first thing inside it is not a space character, comment, <script> element or <style> element.
                let omit_start = empty_attributes
                    && node
                        .children
                        .borrow()
                        .iter()
                        .find_map(|node| match &node.data {
                            NodeData::Text { contents } => self.is_whitespace(contents),
                            NodeData::Element { name, .. } => {
                                Some(!matches!(name.local.as_ref(), "script" | "style"))
                            }
                            NodeData::Comment { .. } => {
                                if self.preserve_comments {
                                    Some(false)
                                } else {
                                    None
                                }
                            }
                            _ => Some(true),
                        })
                        .unwrap_or(true);
                // The end tag may be omitted if the <body> element has contents or has a start tag, and is not immediately followed by a comment.
                let omit_end = ctx.right.map_or(true, |right| !self.next_is_comment(right));

                (omit_start && omit_end, omit_end)
            }
            "p" => {
                let omit_end = ctx.next_element().map_or(true, |node| {
                    if let NodeData::Element { name, .. } = &node.data {
                        matches!(
                            name.local.as_ref().to_ascii_lowercase().as_str(),
                            "address"
                                | "article"
                                | "aside"
                                | "blockquote"
                                | "div"
                                | "dl"
                                | "fieldset"
                                | "footer"
                                | "form"
                                | "h1"
                                | "h2"
                                | "h3"
                                | "h4"
                                | "h5"
                                | "h6"
                                | "header"
                                | "hr"
                                | "menu"
                                | "nav"
                                | "ol"
                                | "p"
                                | "pre"
                                | "section"
                                | "table"
                                | "ul"
                        )
                    } else {
                        false
                    }
                });

                (false, omit_end)
            }
            // TODO: comprehensive handling of optional end element rules
            _ => (false, optional_end_tag(name)),
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    fn minify_children(&mut self, ctx: &Option<Context>, node: &Node) -> io::Result<()> {
        let children = node.children.borrow();
        let l = children.len();

        children.iter().enumerate().try_for_each(|(i, child)| {
            if self.preceding_whitespace && is_block_element(child) {
                self.preceding_whitespace = false;
            }

            self.minify_node(
                &Some(Context {
                    parent: node,
                    parent_context: ctx.as_ref(),
                    left: if i > 0 { Some(&children[..i]) } else { None },
                    right: if i + 1 < l {
                        Some(&children[i + 1..])
                    } else {
                        None
                    },
                }),
                child,
            )
        })
    }

    fn write_qualified_name(&mut self, name: &QualName) -> io::Result<()> {
        if let Some(prefix) = &name.prefix {
            self.w
                .write_all(prefix.as_ref().to_ascii_lowercase().as_bytes())?;
            self.w.write_all(b":")?;
        }

        self.w
            .write_all(name.local.as_ref().to_ascii_lowercase().as_bytes())
    }

    fn write_start_tag(&mut self, name: &QualName, attrs: &[Attribute]) -> io::Result<()> {
        self.w.write_all(b"<")?;
        self.write_qualified_name(name)?;

        attrs
            .iter()
            .try_for_each(|attr| self.write_attribute(attr))?;

        self.w.write_all(b">")
    }

    fn write_end_tag(&mut self, name: &QualName) -> io::Result<()> {
        self.w.write_all(b"</")?;
        self.write_qualified_name(name)?;
        self.w.write_all(b">")
    }

    fn write_attribute(&mut self, attr: &Attribute) -> io::Result<()> {
        self.w.write_all(b" ")?;
        self.write_qualified_name(&attr.name)?;

        let value = attr.value.as_ref();
        let value = if self.collapse_whitespace {
            value.trim_matches(is_ascii_whitespace)
        } else {
            value
        };

        if value.is_empty() {
            return io::Result::Ok(());
        }

        self.w.write_all(b"=")?;

        let b = value.as_bytes();
        let (unquoted, double, _) =
            b.iter()
                .fold((true, false, false), |(unquoted, double, single), &c| {
                    let (double, single) = (double || c == b'"', single || c == b'\'');
                    let unquoted =
                        unquoted && !double && !single && c != b'=' && !c.is_ascii_whitespace();

                    (unquoted, double, single)
                });

        if unquoted {
            self.w.write_all(b)
        } else if double {
            self.write_attribute_value(b, b"'", reserved_entity_with_apos)
        } else {
            self.write_attribute_value(b, b"\"", reserved_entity)
        }
    }

    fn write_attribute_value<T: AsRef<[u8]>>(
        &mut self,
        v: T,
        quote: &[u8],
        f: EntityFn,
    ) -> io::Result<()> {
        self.w.write_all(quote)?;

        let b = v.as_ref();

        if self.collapse_whitespace {
            self.write_collapse_whitespace(b, f, Some(false))
        } else {
            self.w.write_all(b)
        }?;

        self.w.write_all(quote)
    }

    /// Efficiently writes blocks of content, e.g. a string with no collapsed
    /// whitespace would result in a single write.
    fn write_collapse_whitespace(
        &mut self,
        b: &[u8],
        f: EntityFn,
        preceding_whitespace: Option<bool>,
    ) -> io::Result<()> {
        b.iter()
            .enumerate()
            .try_fold(
                (0, preceding_whitespace.unwrap_or(self.preceding_whitespace)),
                |(pos, preceding_whitespace), (i, &c)| {
                    let is_whitespace = c.is_ascii_whitespace();

                    Ok(if is_whitespace && preceding_whitespace {
                        if i != pos {
                            self.write(&b[pos..i], f)?;
                        }

                        // ASCII whitespace = 1 byte
                        (i + 1, true)
                    } else {
                        (pos, is_whitespace)
                    })
                },
            )
            .and_then(|(pos, _)| {
                if pos < b.len() {
                    self.write(&b[pos..], f)?;
                }

                Ok(())
            })
    }

    fn write(&mut self, b: &[u8], f: EntityFn) -> io::Result<()> {
        b.iter()
            .enumerate()
            .try_fold(0, |pos, (i, &c)| {
                Ok(if let Some(entity) = f(c) {
                    self.w.write_all(&b[pos..i])?;
                    self.w.write_all(entity)?;

                    // Reserved characters are 1 byte
                    i + 1
                } else {
                    pos
                })
            })
            .and_then(|pos| {
                if pos < b.len() {
                    self.w.write_all(&b[pos..])?;
                }

                Ok(())
            })
    }
}

type EntityFn = fn(u8) -> Option<&'static [u8]>;

const fn reserved_entity(v: u8) -> Option<&'static [u8]> {
    match v {
        b'<' => Some(b"&lt;"),
        b'>' => Some(b"&gt;"),
        b'&' => Some(b"&#38;"),
        _ => None,
    }
}

const fn reserved_entity_with_apos(v: u8) -> Option<&'static [u8]> {
    if v == b'\'' {
        Some(b"&#39;")
    } else {
        reserved_entity(v)
    }
}

fn is_whitespace(s: &RefCell<Tendril<UTF8>>) -> bool {
    s.borrow().as_bytes().iter().all(u8::is_ascii_whitespace)
}

fn is_block_element_name(name: &str) -> bool {
    matches!(
        name,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "body"
            | "br"
            | "details"
            | "dialog"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "head"
            | "header"
            | "hgroup"
            | "hr"
            | "html"
            | "li"
            | "link"
            | "main"
            | "meta"
            | "nav"
            | "ol"
            | "option"
            | "p"
            | "pre"
            | "script"
            | "section"
            | "source"
            | "table"
            | "td"
            | "th"
            | "title"
            | "tr"
            | "ul"
    )
}

fn is_block_element(node: &Node) -> bool {
    match &node.data {
        NodeData::Element { name, .. } => is_block_element_name(name.local.as_ref()),
        NodeData::Document => true,
        _ => false,
    }
}

#[allow(clippy::missing_const_for_fn)]
fn is_ascii_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

fn preserve_whitespace(name: &str) -> bool {
    matches!(name, "pre" | "textarea")
}

fn contains_code(name: &str) -> bool {
    matches!(name, "script" | "style")
}

fn is_self_closing(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
            | "command"
            | "keygen"
            | "menuitem"
    )
}

fn optional_end_tag(name: &str) -> bool {
    matches!(
        name,
        "basefont"
            | "colgroup"
            | "dd"
            | "dt"
            | "frame"
            | "isindex"
            | "li"
            | "option"
            | "p"
            | "tbody"
            | "td"
            | "tfoot"
            | "th"
            | "thead"
            | "tr"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_write_collapse_whitespace() {
        for &(input, expected, preceding_whitespace) in &[
            ("", "", false),
            ("  ", " ", false),
            ("   ", " ", false),
            ("   ", "", true),
            (" x      y  ", " x y ", false),
            (" x      y  ", "x y ", true),
            (" x   \n  \t \n   y  ", " x y ", false),
            (" x   \n  \t \n   y  ", "x y ", true),
        ] {
            let mut w = vec![];
            let mut minifier = Minifier::new(&mut w);
            minifier.preceding_whitespace = preceding_whitespace;
            minifier
                .write_collapse_whitespace(
                    input.as_bytes(),
                    reserved_entity,
                    Some(preceding_whitespace),
                )
                .unwrap();

            let s = str::from_utf8(&w).unwrap();

            assert_eq!(expected, s);
        }
    }

    #[test]
    fn test_omit_tags() {
        for &(input, expected, collapse_whitespace, preserve_comments) in &[
            // <html>
            ("<html>", "", true, false),
            // Comments ignored
            ("<html><!-- -->", "", true, false),
            // Comments preserved
            ("<html>     <!-- -->    ", "<html><!-- -->", true, true),
            ("<html><!-- --></html>", "<html><!-- -->", true, true),
            (
                "<html><!-- --></html><!-- -->",
                "<html><!-- --></html><!-- -->",
                true,
                true,
            ),
            (
                "<html>    <!-- -->    </html>    <!-- -->    ",
                "<html><!-- --></html><!-- -->",
                true,
                true,
            ),
            (
                "<html>    <!-- -->    </html>    <!-- -->    ",
                // <body> is implicitly added to the DOM
                "<html><!-- --><body>        </html><!-- -->",
                false,
                true,
            ),
            // <head>
            (
                "<html>   <head>   <title>A</title>     </head>   <body><p>     B  </p> </body>",
                "<title>A</title><p>B",
                true,
                false,
            ),
            (
                "<html>   <head>   <title>A</title>     </head>   <body><p>     B  </p> </body>",
                "<head>   <title>A</title>     </head>   <p>     B   ",
                false,
                false,
            ),
            (
                "<html>   <head><!-- -->   <title>A</title>     </head>   <body><p>     B  </p> </body>",
                "<head><!-- --><title>A</title><p>B",
                true,
                true,
            ),
            // <body>
            ("<body>", "", true, false),
            (
                "<body>    <script>let x = 1;</script>   ",
                "<body><script>let x = 1;</script>",
                true,
                false,
            ),
            (
                "<body>        <style>body{margin:1em}</style>",
                "<body><style>body{margin:1em}</style>",
                true,
                false,
            ),
            ("<body>    <p>A", "<p>A", true, false),
            ("<body id=main>    <p>A", "<body id=main><p>A", true, false),
            // Retain whitespace, whitespace before <p>
            (
                "    <body>    <p>A      ",
                "<body>    <p>A      ",
                false,
                false,
            ),
            // Retain whitespace, touching <p>
            ("<body><p>A</body>", "<p>A", false, false),
            // Comments ignored
            ("<body><p>A</body><!-- -->", "<p>A", false, false),
            // Comments preserved
            (
                "<body><p>A</body><!-- -->",
                "<body><p>A</body><!-- -->",
                false,
                true,
            ),
            // Retain end tag if touching inline element
            (
                "<p>Some text</p><button></button>",
                "<p>Some text</p><button></button>",
                false,
                false,
            ),
        ] {
            let mut w = vec![];
            let mut minifier = Minifier::new(&mut w);
            minifier
                .omit_doctype(true)
                .collapse_whitespace(collapse_whitespace)
                .preserve_comments(preserve_comments);
            minifier.minify(&mut input.as_bytes()).unwrap();

            let s = str::from_utf8(&w).unwrap();

            assert_eq!(expected, s);
        }
    }
}
