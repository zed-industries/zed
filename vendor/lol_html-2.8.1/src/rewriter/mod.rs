mod handlers_dispatcher;
mod rewrite_controller;

#[macro_use]
pub(crate) mod settings;

use self::rewrite_controller::{ElementDescriptor, HtmlRewriteController};
pub use self::settings::*;
use crate::base::SharedEncoding;
use crate::memory::{MemoryLimitExceededError, SharedMemoryLimiter};
use crate::parser::ParsingAmbiguityError;
use crate::rewritable_units::Element;
use crate::transform_stream::*;
use encoding_rs::Encoding;
use mime::Mime;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::{self, Debug};
use thiserror::Error;

/// This is an encoding known to be ASCII-compatible.
///
/// Non-ASCII-compatible encodings (`UTF-16LE`, `UTF-16BE`, `ISO-2022-JP` and
/// `replacement`) are not supported by `lol_html`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AsciiCompatibleEncoding(&'static Encoding);

impl AsciiCompatibleEncoding {
    /// Returns `Some` if `Encoding` is ascii-compatible, or `None` otherwise.
    #[must_use]
    pub fn new(encoding: &'static Encoding) -> Option<Self> {
        encoding.is_ascii_compatible().then_some(Self(encoding))
    }

    fn from_mimetype(mime: &Mime) -> Option<Self> {
        let cs = mime.get_param("charset")?;
        Self::new(Encoding::for_label_no_replacement(cs.as_str().as_bytes())?)
    }

    /// Returns the most commonly used UTF-8 encoding.
    #[must_use]
    pub fn utf_8() -> Self {
        Self(encoding_rs::UTF_8)
    }
}

impl From<AsciiCompatibleEncoding> for &'static Encoding {
    fn from(ascii_enc: AsciiCompatibleEncoding) -> &'static Encoding {
        ascii_enc.0
    }
}

impl TryFrom<&'static Encoding> for AsciiCompatibleEncoding {
    type Error = ();

    fn try_from(enc: &'static Encoding) -> Result<Self, ()> {
        Self::new(enc).ok_or(())
    }
}

/// A compound error type that can be returned by [`write`] and [`end`] methods of the rewriter.
///
/// # Note
/// This error is unrecoverable. The rewriter instance will panic on attempt to use it after such an
/// error.
///
/// [`write`]: ../struct.HtmlRewriter.html#method.write
/// [`end`]: ../struct.HtmlRewriter.html#method.end
#[derive(Error, Debug)]
pub enum RewritingError {
    /// See [`MemoryLimitExceededError`].
    ///
    /// [`MemoryLimitExceededError`]: struct.MemoryLimitExceededError.html
    #[error("{0}")]
    MemoryLimitExceeded(MemoryLimitExceededError),

    /// See [`ParsingAmbiguityError`].
    ///
    /// [`ParsingAmbiguityError`]: struct.ParsingAmbiguityError.html
    #[error("{0}")]
    ParsingAmbiguity(ParsingAmbiguityError),

    /// An error that was propagated from one of the content handlers.
    #[error("{0}")]
    ContentHandlerError(Box<dyn StdError + Send + Sync + 'static>),
}

/// A streaming HTML rewriter.
///
/// # Example
/// ```
/// use lol_html::{element, HtmlRewriter, Settings};
///
/// let mut output = vec![];
///
/// {
///     let mut rewriter = HtmlRewriter::new(
///         Settings {
///             element_content_handlers: vec![
///                 // Rewrite insecure hyperlinks
///                 element!("a[href]", |el| {
///                     let href = el
///                         .get_attribute("href")
///                         .unwrap()
///                         .replace("http:", "https:");
///
///                     el.set_attribute("href", &href).unwrap();
///
///                     Ok(())
///                 })
///             ],
///             ..Settings::new()
///         },
///         |c: &[u8]| output.extend_from_slice(c)
///     );
///
///     rewriter.write(b"<div><a href=").unwrap();
///     rewriter.write(b"http://example.com>").unwrap();
///     rewriter.write(b"</a></div>").unwrap();
///     rewriter.end().unwrap();
/// }
///
/// assert_eq!(
///     String::from_utf8(output).unwrap(),
///     r#"<div><a href="https://example.com"></a></div>"#
/// );
/// ```
pub struct HtmlRewriter<'h, O: OutputSink, H: HandlerTypes = LocalHandlerTypes> {
    stream: TransformStream<HtmlRewriteController<'h, H>, O>,
    poisoned: bool,
}

macro_rules! guarded {
    ($self:ident, $expr:expr) => {{
        assert!(
            !$self.poisoned,
            "Attempt to use the HtmlRewriter after a fatal error."
        );

        let res = $expr;

        if res.is_err() {
            $self.poisoned = true;
        }

        res
    }};
}

impl<'h, O: OutputSink, H: HandlerTypes> HtmlRewriter<'h, O, H> {
    /// Constructs a new rewriter with the provided `settings` that writes
    /// the output to the `output_sink`.
    ///
    /// # Note
    ///
    /// For the convenience the [`OutputSink`] trait is implemented for closures.
    ///
    /// [`OutputSink`]: trait.OutputSink.html
    pub fn new<'s>(settings: Settings<'h, 's, H>, output_sink: O) -> Self {
        let preallocated_parsing_buffer_size =
            settings.memory_settings.preallocated_parsing_buffer_size;
        let strict = settings.strict;

        let encoding = SharedEncoding::new(settings.encoding);

        let memory_limiter =
            SharedMemoryLimiter::new(settings.memory_settings.max_allowed_memory_usage);

        let stream = TransformStream::new(TransformStreamSettings {
            transform_controller: HtmlRewriteController::from_settings(
                settings,
                &memory_limiter,
                &encoding,
            ),
            output_sink,
            preallocated_parsing_buffer_size,
            memory_limiter,
            encoding,
            strict,
        });

        HtmlRewriter {
            stream,
            poisoned: false,
        }
    }

    /// Writes a chunk of input data to the rewriter.
    ///
    /// # Panics
    ///  * If previous invocation of the method returned a [`RewritingError`]
    ///    (these errors are unrecovarable).
    ///
    /// [`RewritingError`]: errors/enum.RewritingError.html
    /// [`end`]: struct.HtmlRewriter.html#method.end
    #[inline]
    pub fn write(&mut self, data: &[u8]) -> Result<(), RewritingError> {
        guarded!(self, self.stream.write(data))
    }

    /// Finalizes the rewriting process.
    ///
    /// Should be called once the last chunk of the input is written.
    ///
    /// # Panics
    ///  * If previous invocation of [`write`] returned a [`RewritingError`] (these errors
    ///    are unrecovarable).
    ///
    /// [`RewritingError`]: errors/enum.RewritingError.html
    /// [`write`]: struct.HtmlRewriter.html#method.write
    #[inline]
    pub fn end(mut self) -> Result<(), RewritingError> {
        guarded!(self, self.stream.end())
    }
}

// NOTE: this opaque Debug implementation is required to make
// `.unwrap()` and `.expect()` methods available on Result
// returned by the `HtmlRewriterBuilder.build()` method.
impl<O: OutputSink, H: HandlerTypes> Debug for HtmlRewriter<'_, O, H> {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HtmlRewriter")
    }
}

fn handler_adjust_charset_on_meta_tag<'h, H: HandlerTypes>(
    encoding: SharedEncoding,
) -> (Cow<'h, crate::Selector>, ElementContentHandlers<'h, H>) {
    // HTML5 allows encoding to be set only once
    let mut found = false;

    let handler = move |el: &mut Element<'_, '_, H>| {
        if found {
            return Ok(());
        }

        let charset = el.get_attribute("charset").and_then(|cs| {
            AsciiCompatibleEncoding::new(Encoding::for_label_no_replacement(cs.as_bytes())?)
        });

        let charset = charset.or_else(|| {
            el.get_attribute("http-equiv")
                .filter(|http_equiv| http_equiv.eq_ignore_ascii_case("Content-Type"))
                .and_then(|_| {
                    AsciiCompatibleEncoding::from_mimetype(
                        &el.get_attribute("content")?.parse::<Mime>().ok()?,
                    )
                })
        });

        if let Some(charset) = charset {
            found = true;
            encoding.set(charset);
        }

        Ok(())
    };

    let content_handlers = ElementContentHandlers {
        element: Some(H::new_element_handler(handler)),
        comments: None,
        text: None,
    };

    (Cow::Owned("meta".parse().unwrap()), content_handlers)
}

/// Rewrites given `html` string with the provided `settings`.
///
/// # Example
///
/// ```
/// use lol_html::{rewrite_str, element, RewriteStrSettings};
///
/// let element_content_handlers = vec![
///     // Rewrite insecure hyperlinks
///     element!("a[href]", |el| {
///         let href = el
///             .get_attribute("href")
///             .unwrap()
///             .replace("http:", "https:");
///
///          el.set_attribute("href", &href).unwrap();
///
///          Ok(())
///     })
/// ];
/// let output = rewrite_str(
///     r#"<div><a href="http://example.com"></a></div>"#,
///     RewriteStrSettings {
///         element_content_handlers,
///         ..RewriteStrSettings::new()
///     }
/// ).unwrap();
///
/// assert_eq!(output, r#"<div><a href="https://example.com"></a></div>"#);
/// ```
pub fn rewrite_str<'h, 's, H: HandlerTypes>(
    html: &str,
    settings: impl Into<Settings<'h, 's, H>>,
) -> Result<String, RewritingError> {
    let mut output = vec![];

    let mut rewriter = HtmlRewriter::new(settings.into(), |c: &[u8]| {
        output.extend_from_slice(c);
    });

    rewriter.write(html.as_bytes())?;
    rewriter.end()?;

    // NOTE: it's ok to unwrap here as we guarantee encoding validity of the output
    Ok(String::from_utf8(output).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::TextType;
    use crate::html_content::ContentType;
    use crate::test_utils::{ASCII_COMPATIBLE_ENCODINGS, NON_ASCII_COMPATIBLE_ENCODINGS, Output};
    use encoding_rs::Encoding;
    use itertools::Itertools;
    use static_assertions::assert_impl_all;
    use std::convert::TryInto;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    // Assert that HtmlRewriter with `SendHandlerTypes` is `Send`.
    assert_impl_all!(crate::send::HtmlRewriter<'_, Box<dyn FnMut(&[u8]) + Send + 'static>>: Send);

    fn write_chunks<O: OutputSink>(
        mut rewriter: HtmlRewriter<'_, O>,
        encoding: &'static Encoding,
        chunks: &[&str],
    ) {
        for chunk in chunks {
            let (chunk, _, _) = encoding.encode(chunk);

            rewriter.write(&chunk).unwrap();
        }

        rewriter.end().unwrap();
    }

    fn rewrite_html_bytes(html: &[u8], settings: Settings<'_, '_>) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(html.len());

        let mut rewriter = HtmlRewriter::new(settings, |c: &[u8]| out.extend_from_slice(c));

        rewriter.write(html).unwrap();
        rewriter.end().unwrap();

        out
    }

    #[allow(clippy::drop_non_drop)]
    #[test]
    fn handlers_lifetime_covariance() {
        // This test checks that if you have a handler with a lifetime larger than `'a` then you can
        // use it in a place where a handler of lifetime `'a` is expected. If the code below
        // compiles, then this condition holds.

        let x = AtomicUsize::new(0);

        let el_handler_static = element!("foo", |_| Ok(()));
        let el_handler_local = element!("foo", |_| {
            x.fetch_add(1, Ordering::Relaxed);
            Ok(())
        });

        let doc_handler_static = end!(|_| Ok(()));
        let doc_handler_local = end!(|_| {
            x.fetch_add(1, Ordering::Relaxed);
            Ok(())
        });

        let settings = Settings {
            document_content_handlers: vec![doc_handler_static, doc_handler_local],
            element_content_handlers: vec![el_handler_static, el_handler_local],
            encoding: AsciiCompatibleEncoding::utf_8(),
            strict: false,
            adjust_charset_on_meta_tag: false,
            ..Settings::new()
        };
        let rewriter = HtmlRewriter::new(settings, |_: &[u8]| ());

        drop(rewriter);

        drop(x);
    }

    #[test]
    fn rewrite_html_str() {
        let res = rewrite_str::<LocalHandlerTypes>(
            "<!-- 42 --><div><!--hi--></div>",
            RewriteStrSettings {
                element_content_handlers: vec![
                    element!("div", |el| {
                        el.set_tag_name("span").unwrap();
                        Ok(())
                    }),
                    comments!("div", |c| {
                        c.set_text("hello").unwrap();
                        Ok(())
                    }),
                ],
                ..RewriteStrSettings::new()
            },
        )
        .unwrap();

        assert_eq!(res, "<!-- 42 --><span><!--hello--></span>");
    }

    #[test]
    fn rewrite_incorrect_self_closing() {
        let res = rewrite_str::<LocalHandlerTypes>(
            "<title /></title><div/></div><style /></style><script /></script>
            <br/><br><embed/><embed> <svg><a/><path/><path></path></svg>",
            RewriteStrSettings {
                element_content_handlers: vec![element!("*:not(svg)", |el| {
                    el.set_attribute("s", if el.is_self_closing() { "y" } else { "n" })?;
                    el.set_attribute("c", if el.can_have_content() { "y" } else { "n" })?;
                    el.append("…", ContentType::Text);
                    Ok(())
                })],
                ..RewriteStrSettings::new()
            },
        )
        .unwrap();

        assert_eq!(
            res,
            r#"<title s="y" c="y">…</title><div s="y" c="y">…</div><style s="y" c="y">…</style><script s="y" c="y">…</script>
            <br s="y" c="n" /><br s="n" c="n"><embed s="y" c="n" /><embed s="n" c="n"> <svg><a s="y" c="n" /><path s="y" c="n" /><path s="n" c="y">…</path></svg>"#
        );
    }

    #[test]
    fn rewrite_arbitrary_settings() {
        let res = rewrite_str("<span>Some text</span>", Settings::new()).unwrap();
        assert_eq!(res, "<span>Some text</span>");
    }

    #[test]
    fn non_ascii_compatible_encoding() {
        for encoding in &NON_ASCII_COMPATIBLE_ENCODINGS {
            assert_eq!(AsciiCompatibleEncoding::new(encoding), None);
        }
    }

    #[test]
    fn doctype_info() {
        for &enc in &ASCII_COMPATIBLE_ENCODINGS {
            let mut doctypes = Vec::default();

            {
                let rewriter = HtmlRewriter::new(
                    Settings {
                        document_content_handlers: vec![doctype!(|d| {
                            doctypes.push((d.name(), d.public_id(), d.system_id()));
                            Ok(())
                        })],
                        // NOTE: unwrap() here is intentional; it also tests `Ascii::new`.
                        encoding: enc.try_into().unwrap(),
                        ..Settings::new()
                    },
                    |_: &[u8]| {},
                );

                write_chunks(
                    rewriter,
                    enc,
                    &[
                        "<!doctype html1>",
                        "<!-- test --><div>",
                        r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "#,
                        r#""http://www.w3.org/TR/html4/strict.dtd">"#,
                        "</div><!DoCtYPe ",
                    ],
                );
            }

            assert_eq!(
                doctypes,
                &[
                    (Some("html1".into()), None, None),
                    (
                        Some("html".into()),
                        Some("-//W3C//DTD HTML 4.01//EN".into()),
                        Some("http://www.w3.org/TR/html4/strict.dtd".into())
                    ),
                    (None, None, None),
                ]
            );
        }
    }

    #[test]
    fn rewrite_start_tags() {
        for &enc in &ASCII_COMPATIBLE_ENCODINGS {
            let actual: String = {
                let mut output = Output::new(enc);

                let rewriter = HtmlRewriter::new(
                    Settings {
                        element_content_handlers: vec![element!("*", |el| {
                            el.set_attribute("foo", "bar").unwrap();
                            el.prepend("<test></test>", ContentType::Html);
                            Ok(())
                        })],
                        encoding: enc.try_into().unwrap(),
                        ..Settings::new()
                    },
                    |c: &[u8]| output.push(c),
                );

                write_chunks(
                    rewriter,
                    enc,
                    &[
                        "<!doctype html>\n",
                        "<html>\n",
                        "   <head></head>\n",
                        "   <body>\n",
                        "       <div>Test</div>\n",
                        "   </body>\n",
                        "</html>",
                    ],
                );

                output.into()
            };

            assert_eq!(
                actual,
                concat!(
                    "<!doctype html>\n",
                    "<html foo=\"bar\"><test></test>\n",
                    "   <head foo=\"bar\"><test></test></head>\n",
                    "   <body foo=\"bar\"><test></test>\n",
                    "       <div foo=\"bar\"><test></test>Test</div>\n",
                    "   </body>\n",
                    "</html>",
                )
            );
        }
    }

    #[test]
    fn rewrite_document_content() {
        for &enc in &ASCII_COMPATIBLE_ENCODINGS {
            let actual: String = {
                let mut output = Output::new(enc);

                let rewriter = HtmlRewriter::new(
                    Settings {
                        element_content_handlers: vec![],
                        document_content_handlers: vec![
                            doc_comments!(|c| {
                                c.set_text(&(c.text() + "1337")).unwrap();
                                Ok(())
                            }),
                            doc_text!(|c| {
                                if c.last_in_text_node() {
                                    c.after("BAZ", ContentType::Text);
                                }

                                Ok(())
                            }),
                        ],
                        encoding: enc.try_into().unwrap(),
                        ..Settings::new()
                    },
                    |c: &[u8]| output.push(c),
                );

                write_chunks(
                    rewriter,
                    enc,
                    &[
                        "<!doctype html>\n",
                        "<!-- hey -->\n",
                        "<html>\n",
                        "   <head><!-- aloha --></head>\n",
                        "   <body>\n",
                        "       <div>Test</div>\n",
                        "   </body>\n",
                        "   <!-- bonjour -->\n",
                        "</html>Pshhh",
                    ],
                );

                output.into()
            };

            assert_eq!(
                actual,
                concat!(
                    "<!doctype html>\nBAZ",
                    "<!-- hey 1337-->\nBAZ",
                    "<html>\n",
                    "   BAZ<head><!-- aloha 1337--></head>\n",
                    "   BAZ<body>\n",
                    "       BAZ<div>TestBAZ</div>\n",
                    "   BAZ</body>\n",
                    "   BAZ<!-- bonjour 1337-->\nBAZ",
                    "</html>PshhhBAZ",
                )
            );
        }
    }

    #[test]
    fn rewrite_text_types() {
        for &enc in &ASCII_COMPATIBLE_ENCODINGS {
            let actual: String = {
                let mut output = Output::new(enc);

                let rewriter = HtmlRewriter::new(
                    Settings {
                        element_content_handlers: vec![],
                        document_content_handlers: vec![doc_text!(|c| {
                            let replace = match c.text_type() {
                                TextType::PlainText => 'P',
                                TextType::RCData => 'r',
                                TextType::RawText => 'R',
                                TextType::ScriptData => 'S',
                                TextType::Data => '.',
                                TextType::CDataSection => 'C',
                            };
                            let mut replaced: String = c
                                .as_str()
                                .chars()
                                .map(|c| if c == '\n' { c } else { replace })
                                .collect();
                            if c.last_in_text_node() {
                                replaced.push(';');
                            }
                            c.set_str(replaced);

                            Ok(())
                        })],
                        encoding: enc.try_into().unwrap(),
                        ..Settings::new()
                    },
                    |c: &[u8]| output.push(c),
                );

                write_chunks(
                    rewriter,
                    enc,
                    &[
                        "\n  <!doctype html> <title>rcdata</titlenot> <!--no comment rcdata</title>",
                        "\n   <textarea>rc<x> --><!--no comment </TEXTAREA> ",
                        "\n   body <!--> 1 </> 2 <noscript>nnnn</noscript>",
                        "\n  <script>scr</script> <style>style</style>",
                        "\n  <script><!-- scr --></script> <style>/*<![CDATA[*/ style /*]]>*/</style>",
                        "\n  <svg> body <![CDATA[ cdata ]]> body",
                        "\n  <script>scr</script> <style>style</style>",
                        "\n  <script><!-- com -->s</script> <style>/*<![CDATA[*/ style /*]]>*/</style>",
                        "\n  </svg>",
                    ],
                );

                output.into()
            };

            assert_eq!(
                actual,
                "\
                \n..;<!doctype html>.;<title>rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr;</title>\
                \n...;<textarea>rrrrrrrrrrrrrrrrrrrrrrrr;</TEXTAREA>.\
                \n........;<!-->...;</>...;<noscript>RRRR;</noscript>\
                \n..;<script>SSS;</script>.;<style>RRRRR;</style>\
                \n..;<script>SSSSSSSSSSSS;</script>.;<style>RRRRRRRRRRRRRRRRRRRRRRRRRRR;</style>\
                \n..;<svg>......;<![CDATA[CCCCCCC;]]>.....\
                \n..;<script>...;</script>.;<style>.....;</style>\
                \n..;<script><!-- com -->.;</script>.;<style>..;<![CDATA[CCCCCCCCCCC;]]>..;</style>\
                \n..;</svg>\
                "
            );
        }
    }

    #[test]
    fn handler_invocation_order() {
        let handlers_executed = Arc::new(Mutex::new(Vec::default()));

        macro_rules! create_handlers {
            ($sel:expr, $idx:expr) => {
                element!($sel, {
                    let handlers_executed = ::std::sync::Arc::clone(&handlers_executed);

                    move |_| {
                        handlers_executed.lock().unwrap().push($idx);
                        Ok(())
                    }
                })
            };
        }

        let _res = rewrite_str(
            "<div><span foo></span></div>",
            RewriteStrSettings {
                element_content_handlers: vec![
                    create_handlers!("div span", 0),
                    create_handlers!("div > span", 1),
                    create_handlers!("span", 2),
                    create_handlers!("[foo]", 3),
                    create_handlers!("div span[foo]", 4),
                ],
                ..RewriteStrSettings::new()
            },
        )
        .unwrap();

        assert_eq!(*handlers_executed.lock().unwrap(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn write_esi_tags() {
        let res = rewrite_str(
            "<span><esi:include src=a></span>",
            RewriteStrSettings {
                element_content_handlers: vec![element!("esi\\:include", |el| {
                    el.replace("?", ContentType::Text);
                    Ok(())
                })],
                enable_esi_tags: true,
                ..RewriteStrSettings::new()
            },
        )
        .unwrap();

        assert_eq!(res, "<span>?</span>");
    }

    #[test]
    fn test_rewrite_adjust_charset_on_meta_tag_attribute_charset() {
        use crate::html_content::{ContentType, TextChunk};

        let enthusiastic_text_handler = || {
            doc_text!(move |text: &mut TextChunk<'_>| {
                let new_text = text.as_str().replace('!', "!!!");
                text.replace(&new_text, ContentType::Text);
                Ok(())
            })
        };

        let html: Vec<u8> = [
            r#"<meta charset="windows-1251"><html><head></head><body>I love "#
                .as_bytes()
                .to_vec(),
            vec![0xd5, 0xec, 0xb3, 0xcb, 0xdc],
            br"!</body></html>".to_vec(),
        ]
        .into_iter()
        .concat();

        let expected: Vec<u8> = html
            .iter()
            .copied()
            .flat_map(|c| match c {
                b'!' => vec![b'!', b'!', b'!'],
                c => vec![c],
            })
            .collect();

        let transformed_no_charset_adjustment: Vec<u8> = rewrite_html_bytes(
            &html,
            Settings {
                document_content_handlers: vec![enthusiastic_text_handler()],
                ..Settings::new()
            },
        );

        // Without charset adjustment the response has to be corrupted:
        assert_ne!(transformed_no_charset_adjustment, expected);

        let transformed_charset_adjustment: Vec<u8> = rewrite_html_bytes(
            &html,
            Settings {
                document_content_handlers: vec![enthusiastic_text_handler()],
                adjust_charset_on_meta_tag: true,
                ..Settings::new()
            },
        );

        // If it adapts the charset according to the meta tag everything will be correctly
        // encoded in windows-1251:
        assert_eq!(transformed_charset_adjustment, expected);
    }

    #[test]
    fn test_rewrite_adjust_charset_on_meta_tag_attribute_content_type() {
        use crate::html_content::{ContentType, TextChunk};

        let enthusiastic_text_handler = || {
            doc_text!(move |text: &mut TextChunk<'_>| {
                let new_text = text.as_str().replace('!', "!!!");
                text.replace(&new_text, ContentType::Text);
                Ok(())
            })
        };

        let html: Vec<u8> = [
            r#"<meta http-equiv="conTent-type" content="text/html; charset=windows-1251"><html><head>"#.as_bytes(),
            br#"<meta charset="utf-8"></head><body>I love "#, // second one should be ignored
            &[0xd5, 0xec, 0xb3, 0xcb, 0xdc],
            br"!</body></html>",
        ].concat();

        let expected: Vec<u8> = html
            .iter()
            .copied()
            .flat_map(|c| match c {
                b'!' => vec![b'!', b'!', b'!'],
                c => vec![c],
            })
            .collect();

        let transformed_no_charset_adjustment: Vec<u8> = rewrite_html_bytes(
            &html,
            Settings {
                document_content_handlers: vec![enthusiastic_text_handler()],
                ..Settings::new()
            },
        );

        // Without charset adjustment the response has to be corrupted:
        assert_ne!(transformed_no_charset_adjustment, expected);

        let transformed_charset_adjustment: Vec<u8> = rewrite_html_bytes(
            &html,
            Settings {
                document_content_handlers: vec![enthusiastic_text_handler()],
                adjust_charset_on_meta_tag: true,
                ..Settings::new()
            },
        );

        // If it adapts the charset according to the meta tag everything will be correctly
        // encoded in windows-1251:
        assert_eq!(transformed_charset_adjustment, expected);
    }

    mod fatal_errors {
        use super::*;
        use crate::html_content::Comment;
        use crate::memory::MemoryLimitExceededError;
        use crate::rewritable_units::{Element, TextChunk};

        fn create_rewriter<O: OutputSink>(
            max_allowed_memory_usage: usize,
            output_sink: O,
        ) -> HtmlRewriter<'static, O> {
            HtmlRewriter::new(
                Settings {
                    element_content_handlers: vec![element!("*", |_| Ok(()))],
                    memory_settings: MemorySettings {
                        max_allowed_memory_usage,
                        preallocated_parsing_buffer_size: 0,
                    },
                    ..Settings::new()
                },
                output_sink,
            )
        }

        #[test]
        fn buffer_capacity_limit() {
            const MAX: usize = 100;

            let mut rewriter = create_rewriter(MAX, |_: &[u8]| {});

            // Use two chunks for the stream to force the usage of the buffer and
            // make sure to overflow it.
            let chunk_1 = format!("<img alt=\"{}", "l".repeat(MAX / 2));
            let chunk_2 = format!("{}\" />", "r".repeat(MAX / 2));

            rewriter.write(chunk_1.as_bytes()).unwrap();

            let write_err = rewriter.write(chunk_2.as_bytes()).unwrap_err();

            match write_err {
                RewritingError::MemoryLimitExceeded(e) => assert_eq!(e, MemoryLimitExceededError),
                _ => panic!("{}", write_err),
            }
        }

        #[test]
        #[should_panic(expected = "Attempt to use the HtmlRewriter after a fatal error.")]
        fn poisoning_after_fatal_error() {
            const MAX: usize = 10;

            let mut rewriter = create_rewriter(MAX, |_: &[u8]| {});
            let chunk = format!("<img alt=\"{}", "l".repeat(MAX));

            rewriter.write(chunk.as_bytes()).unwrap_err();
            rewriter.end().unwrap_err();
        }

        #[test]
        fn content_handler_error_propagation() {
            fn assert_err<'h>(
                element_handlers: ElementContentHandlers<'h>,
                document_handlers: DocumentContentHandlers<'h>,
                expected_err: &'static str,
            ) {
                use std::borrow::Cow;

                let mut rewriter = HtmlRewriter::new(
                    Settings {
                        element_content_handlers: vec![(
                            Cow::Owned("*".parse().unwrap()),
                            element_handlers,
                        )],
                        document_content_handlers: vec![document_handlers],
                        ..Settings::new()
                    },
                    |_: &[u8]| {},
                );

                let chunks = [
                    "<!--doc comment--> Doc text",
                    "<div><!--el comment-->El text</div>",
                ];

                let mut err = None;

                for chunk in &chunks {
                    match rewriter.write(chunk.as_bytes()) {
                        Ok(()) => (),
                        Err(e) => {
                            err = Some(e);
                            break;
                        }
                    }
                }

                if err.is_none() {
                    match rewriter.end() {
                        Ok(()) => (),
                        Err(e) => err = Some(e),
                    }
                }

                let err = format!("{}", err.expect("Error expected"));

                assert_eq!(err, expected_err);
            }

            assert_err(
                ElementContentHandlers::default(),
                doc_comments!(|_| Err("Error in doc comment handler".into())),
                "Error in doc comment handler",
            );

            assert_err(
                ElementContentHandlers::default(),
                doc_text!(|_| Err("Error in doc text handler".into())),
                "Error in doc text handler",
            );

            assert_err(
                ElementContentHandlers::default(),
                doc_text!(|_| Err("Error in doctype handler".into())),
                "Error in doctype handler",
            );

            assert_err(
                ElementContentHandlers::default()
                    .element(|_: &mut Element<'_, '_, _>| Err("Error in element handler".into())),
                DocumentContentHandlers::default(),
                "Error in element handler",
            );

            assert_err(
                ElementContentHandlers::default()
                    .comments(|_: &mut Comment<'_>| Err("Error in element comment handler".into())),
                DocumentContentHandlers::default(),
                "Error in element comment handler",
            );

            assert_err(
                ElementContentHandlers::default()
                    .text(|_: &mut TextChunk<'_>| Err("Error in element text handler".into())),
                DocumentContentHandlers::default(),
                "Error in element text handler",
            );
        }
    }
}
