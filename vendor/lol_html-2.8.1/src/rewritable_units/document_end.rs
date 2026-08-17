use super::{ContentType, StreamingHandlerSink};
use encoding_rs::Encoding;

use crate::transform_stream::OutputSink;

/// A rewritable unit that represents the end of the document.
///
/// This exposes the [append](#method.append) function that can be used to append content at the
/// end of the document. The content will only be appended after the rewriter has finished processing
/// the final chunk.
pub struct DocumentEnd<'a> {
    output_sink: &'a mut dyn OutputSink,
    encoding: &'static Encoding,
}

impl<'a> DocumentEnd<'a> {
    #[inline]
    #[must_use]
    pub(crate) fn new(output_sink: &'a mut dyn OutputSink, encoding: &'static Encoding) -> Self {
        DocumentEnd {
            output_sink,
            encoding,
        }
    }

    /// Appends `content` at the end of the document.
    ///
    /// Subsequent calls to this method append `content` to the previously inserted content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{end, rewrite_str, RewriteStrSettings};
    /// use lol_html::html_content::{ContentType, DocumentEnd};
    ///
    /// let html = rewrite_str(
    ///     r#"<div id="foo"><!-- content --></div><img>"#,
    ///     RewriteStrSettings {
    ///         document_content_handlers: vec![end!(|end| {
    ///             end.append("<bar>", ContentType::Html);
    ///             end.append("<baz>", ContentType::Text);
    ///             Ok(())
    ///         })],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div id="foo"><!-- content --></div><img><bar>&lt;baz&gt;"#);
    /// ```
    #[inline]
    pub fn append(&mut self, content: &str, content_type: ContentType) {
        StreamingHandlerSink::new(self.encoding, &mut |c| {
            self.output_sink.handle_chunk(c);
        })
        .write_str(content, content_type);
    }
}

#[cfg(test)]
mod tests {
    use crate::html_content::*;
    use crate::rewritable_units::test_utils::*;
    use encoding_rs::{Encoding, UTF_8};

    fn rewrite_on_end(
        html: &[u8],
        encoding: &'static Encoding,
        mut handler: impl FnMut(&mut DocumentEnd<'_>),
    ) -> String {
        let mut handler_called = false;

        let output = rewrite_html(
            html,
            encoding,
            vec![],
            vec![end!(|end| {
                handler_called = true;
                handler(end);

                Ok(())
            })],
        );

        assert!(handler_called, "Handler not called.");

        output
    }

    #[test]
    fn append_to_empty_document() {
        let output = rewrite_on_end(b"", UTF_8, |end| {
            end.append("<div></div>", ContentType::Html);
        });

        assert_eq!(output, "<div></div>");
    }

    #[test]
    fn append_content() {
        for (html, enc) in encoded("<div><h1>Hεllo</h1></div>") {
            let output = rewrite_on_end(&html, enc, |end| {
                end.append("<span>", ContentType::Html);
                end.append("world", ContentType::Text);
                end.append("<foo>", ContentType::Text);
                end.append("</span>", ContentType::Html);
            });

            assert_eq!(
                output,
                "<div><h1>Hεllo</h1></div><span>world&lt;foo&gt;</span>"
            );
        }
    }

    #[test]
    fn append_content_regression() {
        // This prevents a regression where the output sink received an empty chunk
        // before the end of the input stream.
        for (html, enc) in encoded("") {
            let output = rewrite_on_end(&html, enc, |end| {
                end.append("<foo>", ContentType::Text);
            });

            assert_eq!(output, "&lt;foo&gt;");
        }
    }
}
