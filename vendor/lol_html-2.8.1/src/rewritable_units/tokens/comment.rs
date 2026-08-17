use super::{Mutations, Token};
use crate::base::{Bytes, BytesCow};
use crate::base::{SourceLocation, SpannedRawBytes};
use crate::errors::RewritingError;
use crate::html_content::{StreamingHandler, StreamingHandlerSink};
use crate::rewritable_units::StringChunk;
use encoding_rs::Encoding;
use std::any::Any;
use std::fmt::{self, Debug};
use thiserror::Error;

/// An error that occurs when invalid value is provided for the HTML comment text.
#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum CommentTextError {
    /// The provided value contains the `-->` character sequence that preemptively closes the comment.
    #[error("Comment text shouldn't contain comment closing sequence (`-->`).")]
    CommentClosingSequence,

    /// The provided value contains a character that can't be represented in the document's [`encoding`].
    ///
    /// [`encoding`]: ../struct.Settings.html#structfield.encoding
    #[error(
        "Comment text contains a character that can't be represented in the document's character encoding."
    )]
    UnencodableCharacter,
}

/// An HTML comment rewritable unit.
///
/// Exposes API for examination and modification of a parsed HTML comment.
pub struct Comment<'i> {
    text: BytesCow<'i>,
    raw: SpannedRawBytes<'i>,
    encoding: &'static Encoding,
    mutations: Mutations,
    user_data: Box<dyn Any>,
}

impl<'i> Comment<'i> {
    #[inline]
    #[must_use]
    pub(super) fn new_token(
        text: Bytes<'i>,
        raw: SpannedRawBytes<'i>,
        encoding: &'static Encoding,
    ) -> Token<'i> {
        Token::Comment(Comment {
            text: text.into(),
            raw,
            encoding,
            mutations: Mutations::new(),
            user_data: Box::new(()),
        })
    }

    /// Returns the text of the comment.
    #[inline]
    #[must_use]
    pub fn text(&self) -> String {
        self.text.as_string(self.encoding)
    }

    #[inline(always)]
    pub(crate) fn encoding(&self) -> &'static Encoding {
        self.encoding
    }

    /// Sets the text of the comment.
    #[inline]
    pub fn set_text(&mut self, text: &str) -> Result<(), CommentTextError> {
        if text.contains("-->") {
            Err(CommentTextError::CommentClosingSequence)
        } else {
            // NOTE: if character can't be represented in the given
            // encoding then encoding_rs replaces it with a numeric
            // character reference. Character references are not
            // supported in comments, so we need to bail.
            match BytesCow::owned_from_str_without_replacements(text, self.encoding) {
                Ok(text) => {
                    self.text = text;
                    self.raw.set_modified();

                    Ok(())
                }
                Err(_) => Err(CommentTextError::UnencodableCharacter),
            }
        }
    }

    /// Inserts `content` before the comment.
    ///
    /// Consequent calls to the method append `content` to the previously inserted content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, comments, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div><!-- foo --></div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             comments!("div", |c| {
    ///                 c.before("<!-- 42 -->", ContentType::Html);
    ///                 c.before("bar", ContentType::Text);
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div><!-- 42 -->bar<!-- foo --></div>"#);
    /// ```
    #[inline]
    pub fn before(&mut self, content: &str, content_type: crate::rewritable_units::ContentType) {
        self.mutations
            .mutate()
            .content_before
            .push_back(StringChunk::from_str(content, content_type));
    }

    /// Inserts content from a [`StreamingHandler`] before the comment.
    ///
    /// Consequent calls to the method append to the previously inserted content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    #[inline]
    pub fn streaming_before(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .content_before
            .push_back(StringChunk::stream(string_writer));
    }

    /// Inserts `content` after the comment.
    ///
    /// Consequent calls to the method prepend `content` to the previously inserted content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, comments, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div><!-- foo --></div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             comments!("div", |c| {
    ///                 c.after("Bar", ContentType::Text);
    ///                 c.after("Qux", ContentType::Text);
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div><!-- foo -->QuxBar</div>"#);
    /// ```
    #[inline]
    pub fn after(&mut self, content: &str, content_type: crate::rewritable_units::ContentType) {
        self.mutations
            .mutate()
            .content_after
            .push_front(StringChunk::from_str(content, content_type));
    }

    /// Inserts content from a [`StreamingHandler`] after the comment.
    ///
    /// Consequent calls to the method prepend to the previously inserted content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    #[inline]
    pub fn streaming_after(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .content_after
            .push_front(StringChunk::stream(string_writer));
    }

    /// Replaces the comment with the `content`.
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    ///
    /// # Example
    ///
    /// ```
    /// use lol_html::{rewrite_str, comments, RewriteStrSettings};
    /// use lol_html::html_content::ContentType;
    ///
    /// let html = rewrite_str(
    ///     r#"<div><!-- foo --></div>"#,
    ///     RewriteStrSettings {
    ///         element_content_handlers: vec![
    ///             comments!("div", |c| {
    ///                 c.replace("Bar", ContentType::Text);
    ///                 c.replace("Qux", ContentType::Text);
    ///
    ///                 Ok(())
    ///             })
    ///         ],
    ///         ..RewriteStrSettings::new()
    ///     }
    /// ).unwrap();
    ///
    /// assert_eq!(html, r#"<div>Qux</div>"#);
    /// ```
    #[inline]
    pub fn replace(&mut self, content: &str, content_type: crate::rewritable_units::ContentType) {
        self.mutations
            .mutate()
            .replace(StringChunk::from_str(content, content_type));
    }

    /// Replaces the comment with the content from a [`StreamingHandler`].
    ///
    /// Consequent calls to the method overwrite previous replacement content.
    ///
    /// Use the [`streaming!`] macro to make a `StreamingHandler` from a closure.
    #[inline]
    pub fn streaming_replace(&mut self, string_writer: Box<dyn StreamingHandler + Send + 'static>) {
        self.mutations
            .mutate()
            .replace(StringChunk::stream(string_writer));
    }

    /// Removes the comment.
    #[inline]
    pub fn remove(&mut self) {
        self.mutations.mutate().remove();
    }

    /// Returns `true` if the comment has been replaced or removed.
    #[inline]
    #[must_use]
    pub fn removed(&self) -> bool {
        self.mutations.removed()
    }

    #[inline]
    fn serialize_self(&self, sink: &mut StreamingHandlerSink<'_>) -> Result<(), RewritingError> {
        let output_handler = sink.output_handler();

        if let Some(raw) = self.raw.original() {
            output_handler(raw);
        } else {
            output_handler(b"<!--");
            output_handler(&self.text);
            output_handler(b"-->");
        }
        Ok(())
    }

    /// Position of this comment in the source document, before any rewriting
    #[must_use]
    pub fn source_location(&self) -> SourceLocation {
        self.raw.source_location()
    }
}

impl_serialize!(Comment);
impl_user_data!(Comment<'_>);

impl Debug for Comment<'_> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Comment")
            .field("text", &self.text())
            .field("at", &self.source_location())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::*;
    use crate::html_content::*;
    use crate::rewritable_units::test_utils::*;
    use encoding_rs::{EUC_JP, Encoding, UTF_8};

    fn rewrite_comment(
        html: &[u8],
        encoding: &'static Encoding,
        mut handler: impl FnMut(&mut Comment<'_>),
    ) -> String {
        let mut handler_called = false;

        let output = rewrite_html(
            html,
            encoding,
            vec![],
            vec![doc_comments!(|c| {
                handler_called = true;
                handler(c);
                Ok(())
            })],
        );

        assert!(handler_called);

        output
    }

    #[test]
    fn sanitizer_bypass() {
        // HTML allows comments to be closed in multiple invalid ways
        let out = rewrite_comment(b"<!--><img> surprise-->", UTF_8, |c| {
            c.set_text("comment closes early").unwrap();
        });
        assert_eq!("<!--comment closes early--><img> surprise-->", out);
        let out = rewrite_comment(b"<!-- --!><p><!---></p>", UTF_8, |c| {
            c.set_text("unusual ending").unwrap();
        });
        assert_eq!("<!--unusual ending--><p><!--unusual ending--></p>", out);
    }

    #[test]
    fn sanitizer_bypass2() {
        let out = rewrite_comment(b"<?xml >s<img src=x onerror=alert(1)> ?>", UTF_8, |c| {
            c.set_text("pie is a lie!").unwrap();
        });
        assert_eq!("<!--pie is a lie!-->s<img src=x onerror=alert(1)> ?>", out);
    }

    #[test]
    fn comment_closing_sequence_in_text() {
        rewrite_comment(b"<!-- foo -->", UTF_8, |c| {
            let err = c.set_text("foo -- bar --> baz").unwrap_err();

            assert_eq!(err, CommentTextError::CommentClosingSequence);
        });
    }

    #[test]
    fn encoding_unmappable_chars_in_text() {
        rewrite_comment(b"<!-- foo -->", EUC_JP, |c| {
            let err = c.set_text("foo\u{00F8}bar").unwrap_err();

            assert_eq!(err, CommentTextError::UnencodableCharacter);
        });
    }

    #[test]
    fn user_data() {
        rewrite_comment(b"<!-- foo -->", UTF_8, |c| {
            c.set_user_data(42usize);

            assert_eq!(*c.user_data().downcast_ref::<usize>().unwrap(), 42usize);

            *c.user_data_mut().downcast_mut::<usize>().unwrap() = 1337usize;

            assert_eq!(*c.user_data().downcast_ref::<usize>().unwrap(), 1337usize);
        });
    }

    mod serialization {
        use super::*;

        const HTML: &str = "<!-- fooé -- bar -->";

        macro_rules! test {
            ($handler:expr, $expected:expr) => {
                for (html, enc) in encoded(HTML) {
                    assert_eq!(rewrite_comment(&html, enc, $handler), $expected);
                }
            };
        }

        #[test]
        fn parsed() {
            test!(|_| {}, "<!-- fooé -- bar -->");
        }

        #[test]
        fn modified_text() {
            test!(
                |c| {
                    c.set_text("42é <!-").unwrap();
                },
                "<!--42é <!--->"
            );
        }

        #[test]
        fn with_prepends_and_appends() {
            test!(
                |c| {
                    c.before("<span>", ContentType::Text);
                    c.before("<div>Hey</div>", ContentType::Html);
                    c.before("<foo>", ContentType::Html);
                    c.after("</foo>", ContentType::Html);
                    c.after("<!-- 42é -->", ContentType::Html);
                    c.after("<foo & bar>", ContentType::Text);
                },
                concat!(
                    "&lt;span&gt;<div>Hey</div><foo><!-- fooé -- bar -->",
                    "&lt;foo &amp; bar&gt;<!-- 42é --></foo>",
                )
            );
        }

        #[test]
        fn removed() {
            test!(
                |c| {
                    assert!(!c.removed());

                    c.remove();

                    assert!(c.removed());

                    c.before("<before>", ContentType::Html);
                    c.streaming_after(Box::new(|s: &mut StreamingHandlerSink<'_>| {
                        s.write_str("<af", ContentType::Html);
                        s.write_str("ter>", ContentType::Html);
                        Ok(())
                    }));
                },
                "<before><after>"
            );
        }

        #[test]
        fn replaced_with_text() {
            test!(
                |c| {
                    c.before("<before>", ContentType::Html);
                    c.after("<after>", ContentType::Html);

                    assert!(!c.removed());

                    c.replace("<div></div>", ContentType::Html);
                    c.replace("<!--42-->", ContentType::Html);
                    c.streaming_replace(streaming!(|h| {
                        h.write_str("<foo &", ContentType::Text);
                        h.write_str(" bar>", ContentType::Text);
                        Ok(())
                    }));

                    assert!(c.removed());
                },
                "<before>&lt;foo &amp; bar&gt;<after>"
            );
        }

        #[test]
        fn replaced_with_html() {
            test!(
                |c| {
                    c.before("<before>", ContentType::Html);
                    c.after("<after>", ContentType::Html);

                    assert!(!c.removed());

                    c.replace("<div></div>", ContentType::Html);
                    c.replace("<!--42-->", ContentType::Html);
                    c.replace("<foo & bar>", ContentType::Html);

                    assert!(c.removed());
                },
                "<before><foo & bar><after>"
            );
        }
    }
}
