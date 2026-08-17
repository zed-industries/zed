/*!
A simple, streaming, partially-validating XML writer that writes XML data into an internal buffer.

## Features

- A simple, bare-minimum, panic-based API.
- Non-allocating API. All methods are accepting either `fmt::Display` or `fmt::Arguments`.
- Nodes auto-closing.

## Example

```rust
use xmlwriter::*;

let opt = Options {
    use_single_quote: true,
    ..Options::default()
};

let mut w = XmlWriter::new(opt);
w.start_element("svg");
w.write_attribute("xmlns", "http://www.w3.org/2000/svg");
w.write_attribute_fmt("viewBox", format_args!("{} {} {} {}", 0, 0, 128, 128));
w.start_element("text");
// We can write any object that implements `fmt::Display`.
w.write_attribute("x", &10);
w.write_attribute("y", &20);
w.write_text_fmt(format_args!("length is {}", 5));

assert_eq!(w.end_document(),
"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 128 128'>
    <text x='10' y='20'>
        length is 5
    </text>
</svg>
");
```
*/

#![doc(html_root_url = "https://docs.rs/xmlwriter/0.1.0")]

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_copy_implementations)]


use std::fmt::{self, Display};
use std::io::Write;
use std::ops::Range;


/// An XML node indention.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Indent {
    /// Disable indention and new lines.
    None,
    /// Indent with spaces. Preferred range is 0..4.
    Spaces(u8),
    /// Indent with tabs.
    Tabs,
}

/// An XML writing options.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Use single quote marks instead of double quote.
    ///
    /// # Examples
    ///
    /// Before:
    ///
    /// ```text
    /// <rect fill="red"/>
    /// ```
    ///
    /// After:
    ///
    /// ```text
    /// <rect fill='red'/>
    /// ```
    ///
    /// Default: disabled
    pub use_single_quote: bool,

    /// Set XML nodes indention.
    ///
    /// # Examples
    ///
    /// `Indent::None`
    /// Before:
    ///
    /// ```text
    /// <svg>
    ///     <rect fill="red"/>
    /// </svg>
    /// ```
    ///
    /// After:
    ///
    /// ```text
    /// <svg><rect fill="red"/></svg>
    /// ```
    ///
    /// Default: 4 spaces
    pub indent: Indent,

    /// Set XML attributes indention.
    ///
    /// # Examples
    ///
    /// `Indent::Spaces(2)`
    ///
    /// Before:
    ///
    /// ```text
    /// <svg>
    ///     <rect fill="red" stroke="black"/>
    /// </svg>
    /// ```
    ///
    /// After:
    ///
    /// ```text
    /// <svg>
    ///     <rect
    ///       fill="red"
    ///       stroke="black"/>
    /// </svg>
    /// ```
    ///
    /// Default: `None`
    pub attributes_indent: Indent,
}

impl Default for Options {
    #[inline]
    fn default() -> Self {
        Options {
            use_single_quote: false,
            indent: Indent::Spaces(4),
            attributes_indent: Indent::None,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Empty,
    Document,
    Attributes,
}

struct DepthData {
    range: Range<usize>,
    has_children: bool,
}


/// An XML writer.
pub struct XmlWriter {
    buf: Vec<u8>,
    state: State,
    preserve_whitespaces: bool,
    depth_stack: Vec<DepthData>,
    opt: Options,
}

impl XmlWriter {
    #[inline]
    fn from_vec(buf: Vec<u8>, opt: Options) -> Self {
        XmlWriter {
            buf,
            state: State::Empty,
            preserve_whitespaces: false,
            depth_stack: Vec::new(),
            opt,
        }
    }

    /// Creates a new `XmlWriter`.
    #[inline]
    pub fn new(opt: Options) -> Self {
        Self::from_vec(Vec::new(), opt)
    }

    /// Creates a new `XmlWriter` with a specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize, opt: Options) -> Self {
        Self::from_vec(Vec::with_capacity(capacity), opt)
    }

    /// Writes an XML declaration.
    ///
    /// `<?xml version="1.0" encoding="UTF-8" standalone="no"?>`
    ///
    /// # Panics
    ///
    /// - When called twice.
    #[inline(never)]
    pub fn write_declaration(&mut self) {
        if self.state != State::Empty {
            panic!("declaration was already written");
        }

        // Pretend that we are writing an element.
        self.state = State::Attributes;

        // <?xml version='1.0' encoding='UTF-8' standalone='yes'?>
        self.push_str("<?xml");
        self.write_attribute("version", "1.0");
        self.write_attribute("encoding", "UTF-8");
        self.write_attribute("standalone", "no");
        self.push_str("?>");

        self.state = State::Document;
    }

    /// Writes a comment string.
    pub fn write_comment(&mut self, text: &str) {
        self.write_comment_fmt(format_args!("{}", text));
    }

    /// Writes a formatted comment.
    #[inline(never)]
    pub fn write_comment_fmt(&mut self, fmt: fmt::Arguments) {
        if self.state == State::Attributes {
            self.write_open_element();
        }

        if self.state != State::Empty {
            self.write_new_line();
        }

        self.write_node_indent();

        // <!--text-->
        self.push_str("<!--");
        self.buf.write_fmt(fmt).unwrap(); // TODO: check content
        self.push_str("-->");

        if self.state == State::Attributes {
            self.depth_stack.push(DepthData {
                range: 0..0,
                has_children: false,
            });
        }

        self.state = State::Document;
    }

    /// Starts writing a new element.
    ///
    /// This method writes only the `<tag-name` part.
    #[inline(never)]
    pub fn start_element(&mut self, name: &str) {
        if self.state == State::Attributes {
            self.write_open_element();
        }

        if self.state != State::Empty {
            self.write_new_line();
        }

        if !self.preserve_whitespaces {
            self.write_node_indent();
        }

        self.push_byte(b'<');
        let start = self.buf.len();
        self.push_str(name);

        self.depth_stack.push(DepthData {
            range: start..self.buf.len(),
            has_children: false,
        });

        self.state = State::Attributes;
    }

    /// Writes an attribute.
    ///
    /// Quotes in the value will be escaped.
    ///
    /// # Panics
    ///
    /// - When called before `start_element()`.
    /// - When called after `close_element()`.
    ///
    /// # Example
    ///
    /// ```
    /// use xmlwriter::*;
    ///
    /// let mut w = XmlWriter::new(Options::default());
    /// w.start_element("svg");
    /// w.write_attribute("x", "5");
    /// w.write_attribute("y", &5);
    /// assert_eq!(w.end_document(), "<svg x=\"5\" y=\"5\"/>\n");
    /// ```
    pub fn write_attribute<V: Display + ?Sized>(&mut self, name: &str, value: &V) {
        self.write_attribute_fmt(name, format_args!("{}", value));
    }

    /// Writes a formatted attribute value.
    ///
    /// Quotes in the value will be escaped.
    ///
    /// # Panics
    ///
    /// - When called before `start_element()`.
    /// - When called after `close_element()`.
    ///
    /// # Example
    ///
    /// ```
    /// use xmlwriter::*;
    ///
    /// let mut w = XmlWriter::new(Options::default());
    /// w.start_element("rect");
    /// w.write_attribute_fmt("fill", format_args!("url(#{})", "gradient"));
    /// assert_eq!(w.end_document(), "<rect fill=\"url(#gradient)\"/>\n");
    /// ```
    #[inline(never)]
    pub fn write_attribute_fmt(&mut self, name: &str, fmt: fmt::Arguments) {
        if self.state != State::Attributes {
            panic!("must be called after start_element()");
        }

        self.write_attribute_prefix(name);
        let start = self.buf.len();
        self.buf.write_fmt(fmt).unwrap();
        self.escape_attribute_value(start);
        self.write_quote();
    }

    /// Writes a raw attribute value.
    ///
    /// Closure provides a mutable reference to an internal buffer.
    ///
    /// **Warning:** this method is an escape hatch for cases when you need to write
    /// a lot of data very fast.
    ///
    /// # Panics
    ///
    /// - When called before `start_element()`.
    /// - When called after `close_element()`.
    ///
    /// # Example
    ///
    /// ```
    /// use xmlwriter::*;
    ///
    /// let mut w = XmlWriter::new(Options::default());
    /// w.start_element("path");
    /// w.write_attribute_raw("d", |buf| buf.extend_from_slice(b"M 10 20 L 30 40"));
    /// assert_eq!(w.end_document(), "<path d=\"M 10 20 L 30 40\"/>\n");
    /// ```
    #[inline(never)]
    pub fn write_attribute_raw<F>(&mut self, name: &str, f: F)
        where F: FnOnce(&mut Vec<u8>)
    {
        if self.state != State::Attributes {
            panic!("must be called after start_element()");
        }

        self.write_attribute_prefix(name);
        let start = self.buf.len();
        f(&mut self.buf);
        self.escape_attribute_value(start);
        self.write_quote();
    }

    #[inline(never)]
    fn write_attribute_prefix(&mut self, name: &str) {
        if self.opt.attributes_indent == Indent::None {
            self.push_byte(b' ');
        } else {
            self.push_byte(b'\n');

            let depth = self.depth_stack.len();
            if depth > 0 {
                self.write_indent(depth - 1, self.opt.indent);
            }

            self.write_indent(1, self.opt.attributes_indent);
        }

        self.push_str(name);
        self.push_byte(b'=');
        self.write_quote();
    }

    /// Escapes the attribute value string.
    ///
    /// - " -> &quot;
    /// - ' -> &apos;
    #[inline(never)]
    fn escape_attribute_value(&mut self, mut start: usize) {
        let quote = if self.opt.use_single_quote { b'\'' } else { b'"' };
        while let Some(idx) = self.buf[start..].iter().position(|c| *c == quote) {
            let i = start + idx;
            let s = if self.opt.use_single_quote { b"&apos;" } else { b"&quot;" };
            self.buf.splice(i..i+1, s.iter().cloned());
            start = i + 6;
        }
    }

    /// Sets the preserve whitespaces flag.
    ///
    /// - If set, text nodes will be written as is.
    /// - If not set, text nodes will be indented.
    ///
    /// Can be set at any moment.
    ///
    /// # Example
    ///
    /// ```
    /// use xmlwriter::*;
    ///
    /// let mut w = XmlWriter::new(Options::default());
    /// w.start_element("html");
    /// w.start_element("p");
    /// w.write_text("text");
    /// w.end_element();
    /// w.start_element("p");
    /// w.set_preserve_whitespaces(true);
    /// w.write_text("text");
    /// w.end_element();
    /// w.set_preserve_whitespaces(false);
    /// assert_eq!(w.end_document(),
    /// "<html>
    ///     <p>
    ///         text
    ///     </p>
    ///     <p>text</p>
    /// </html>
    /// ");
    /// ```
    pub fn set_preserve_whitespaces(&mut self, preserve: bool) {
        self.preserve_whitespaces = preserve;
    }

    /// Writes a text node.
    ///
    /// See `write_text_fmt()` for details.
    pub fn write_text(&mut self, text: &str) {
        self.write_text_fmt(format_args!("{}", text));
    }

    /// Writes a formatted text node.
    ///
    /// `<` will be escaped.
    ///
    /// # Panics
    ///
    /// - When called not after `start_element()`.
    #[inline(never)]
    pub fn write_text_fmt(&mut self, fmt: fmt::Arguments) {
        if self.state == State::Empty || self.depth_stack.is_empty() {
            panic!("must be called after start_element()");
        }

        if self.state == State::Attributes {
            self.write_open_element();
        }

        if self.state != State::Empty {
            self.write_new_line();
        }

        self.write_node_indent();

        let start = self.buf.len();
        self.buf.write_fmt(fmt).unwrap();
        self.escape_text(start);

        if self.state == State::Attributes {
            self.depth_stack.push(DepthData {
                range: 0..0,
                has_children: false,
            });
        }

        self.state = State::Document;
    }

    fn escape_text(&mut self, mut start: usize) {
        while let Some(idx) = self.buf[start..].iter().position(|c| *c == b'<') {
            let i = start + idx;
            self.buf.splice(i..i+1, b"&lt;".iter().cloned());
            start = i + 4;
        }
    }

    /// Closes an open element.
    #[inline(never)]
    pub fn end_element(&mut self) {
        if let Some(depth) = self.depth_stack.pop() {
            if depth.has_children {
                if !self.preserve_whitespaces {
                    self.write_new_line();
                    self.write_node_indent();
                }

                self.push_str("</");

                for i in depth.range {
                    self.push_byte(self.buf[i]);
                }

                self.push_byte(b'>');
            } else {
                self.push_str("/>");
            }
        }

        self.state = State::Document;
    }

    /// Closes all open elements and returns an internal XML buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use xmlwriter::*;
    ///
    /// let mut w = XmlWriter::new(Options::default());
    /// w.start_element("svg");
    /// w.start_element("g");
    /// w.start_element("rect");
    /// assert_eq!(w.end_document(),
    /// "<svg>
    ///     <g>
    ///         <rect/>
    ///     </g>
    /// </svg>
    /// ");
    /// ```
    pub fn end_document(mut self) -> String {
        while !self.depth_stack.is_empty() {
            self.end_element();
        }

        self.write_new_line();

        // The only way it can fail is if an invalid data
        // was written via `write_attribute_raw()`.
        String::from_utf8(self.buf).unwrap()
    }

    #[inline]
    fn push_byte(&mut self, c: u8) {
        self.buf.push(c);
    }

    #[inline]
    fn push_str(&mut self, text: &str) {
        self.buf.extend_from_slice(text.as_bytes());
    }

    #[inline]
    fn get_quote_char(&self) -> u8 {
        if self.opt.use_single_quote { b'\'' } else { b'"' }
    }

    #[inline]
    fn write_quote(&mut self) {
        self.push_byte(self.get_quote_char());
    }

    fn write_open_element(&mut self) {
        if let Some(depth) = self.depth_stack.last_mut() {
            depth.has_children = true;
            self.push_byte(b'>');

            self.state = State::Document;
        }
    }

    fn write_node_indent(&mut self) {
        self.write_indent(self.depth_stack.len(), self.opt.indent);
    }

    fn write_indent(&mut self, depth: usize, indent: Indent) {
        if indent == Indent::None || self.preserve_whitespaces {
            return;
        }

        for _ in 0..depth {
            match indent {
                Indent::None => {}
                Indent::Spaces(n) => {
                    for _ in 0..n {
                        self.push_byte(b' ');
                    }
                }
                Indent::Tabs => self.push_byte(b'\t'),
            }
        }
    }

    fn write_new_line(&mut self) {
        if self.opt.indent != Indent::None && !self.preserve_whitespaces {
            self.push_byte(b'\n');
        }
    }
}
