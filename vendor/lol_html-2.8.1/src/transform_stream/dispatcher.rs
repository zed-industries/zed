use crate::base::{Bytes, Range, SharedEncoding, SourceLocation};
use crate::html::{LocalName, Namespace};
use crate::html_content::{TextChunk, TextType};
use crate::parser::{
    ActionError, ActionResult, AttributeBuffer, Lexeme, LexemeSink, NonTagContentLexeme,
    NonTagContentTokenOutline, ParserDirective, ParserOutputSink, TagHintSink, TagLexeme,
    TagTokenOutline,
};
use crate::rewritable_units::TextDecoder;
use crate::rewritable_units::ToTokenResult;
use crate::rewritable_units::{DocumentEnd, Serialize, ToToken, Token, TokenCaptureFlags};
use crate::rewriter::RewritingError;
use encoding_rs::Encoding;

pub(crate) struct AuxStartTagInfo<'i> {
    pub input: &'i Bytes<'i>,
    pub attr_buffer: &'i AttributeBuffer,
    pub self_closing: bool,
}

type AuxStartTagInfoRequest<C> =
    Box<dyn FnOnce(&mut C, AuxStartTagInfo<'_>) -> ActionResult<TokenCaptureFlags> + Send>;

// Pub only for integration tests
#[allow(private_interfaces)]
pub enum DispatcherError<C> {
    InfoRequest(AuxStartTagInfoRequest<C>),
    RewritingError(RewritingError),
}

// Pub only for integration tests
pub type StartTagHandlingResult<C> = Result<TokenCaptureFlags, DispatcherError<C>>;

// Pub only for integration tests
pub trait TransformController: Sized {
    fn initial_capture_flags(&self) -> TokenCaptureFlags;
    fn handle_start_tag(
        &mut self,
        name: LocalName<'_>,
        ns: Namespace,
    ) -> StartTagHandlingResult<Self>;
    fn handle_end_tag(&mut self, name: LocalName<'_>) -> TokenCaptureFlags;
    fn handle_token(&mut self, token: &mut Token<'_>) -> Result<(), RewritingError>;
    fn handle_end(&mut self, document_end: &mut DocumentEnd<'_>) -> Result<(), RewritingError>;
    fn should_emit_content(&self) -> bool;
}

/// Defines an interface for the [`HtmlRewriter`]'s output.
///
/// Implemented for [`Fn`] and [`FnMut`].
///
/// [`HtmlRewriter`]: struct.HtmlRewriter.html
/// [`Fn`]: https://doc.rust-lang.org/std/ops/trait.Fn.html
/// [`FnMut`]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
pub trait OutputSink {
    /// Handles rewriter's output chunk.
    ///
    /// # Note
    /// The last chunk of the output has zero length.
    fn handle_chunk(&mut self, chunk: &[u8]);
}

impl<F: FnMut(&[u8])> OutputSink for F {
    fn handle_chunk(&mut self, chunk: &[u8]) {
        self(chunk);
    }
}

// Pub only for integration tests
pub struct Dispatcher<C, O> {
    delegate: DispatcherDelegate<C, O>,
    text_decoder: TextDecoder,
    last_text_type: TextType,
    got_flags_from_hint: bool,
    pending_element_aux_info_req: Option<AuxStartTagInfoRequest<C>>,
    encoding: SharedEncoding,
}

/// Fields split out of `Dispatcher` for borrow checking of event handlers
struct DispatcherDelegate<C, O> {
    transform_controller: C,
    output_sink: O,
    remaining_content_start: usize,
    capture_flags: TokenCaptureFlags,
    emission_enabled: bool,
}

impl<C, O> DispatcherDelegate<C, O>
where
    C: TransformController,
    O: OutputSink,
{
    fn flush_remaining_input(&mut self, input: &[u8], consumed_byte_count: usize) {
        if self.emission_enabled {
            let output = input
                .get(self.remaining_content_start..consumed_byte_count)
                .unwrap_or_default();

            if !output.is_empty() {
                self.output_sink.handle_chunk(output);
            }
        }

        self.remaining_content_start = 0;
    }

    fn finish(&mut self, encoding: &'static Encoding, input: &[u8]) -> Result<(), RewritingError> {
        self.flush_remaining_input(input, input.len());

        let mut document_end = DocumentEnd::new(&mut self.output_sink, encoding);

        self.transform_controller.handle_end(&mut document_end)?;

        // NOTE: output the finalizing chunk.
        self.output_sink.handle_chunk(&[]);

        Ok(())
    }

    fn lexeme_consumed<T>(&mut self, lexeme: &Lexeme<'_, T>) {
        let lexeme_range = lexeme.raw_range();

        let chunk_range = Range {
            start: self.remaining_content_start,
            end: lexeme_range.start,
        };

        let chunk = lexeme.input().slice(chunk_range);

        if self.emission_enabled && !chunk.is_empty() {
            self.output_sink.handle_chunk(&chunk);
        }

        self.remaining_content_start = lexeme_range.end;
    }

    #[inline]
    fn token_produced(&mut self, mut token: Token<'_>) -> Result<(), RewritingError> {
        trace!(@output token);

        self.transform_controller.handle_token(&mut token)?;

        if self.emission_enabled {
            token.into_bytes(&mut |c| self.output_sink.handle_chunk(c))?;
        }
        Ok(())
    }

    fn text_token_produced(
        &mut self,
        text: &str,
        encoding: &'static Encoding,
        text_type: TextType,
        is_last_in_node: bool,
        source_location: SourceLocation,
    ) -> Result<(), RewritingError> {
        let mut token = Token::TextChunk(TextChunk::new(
            text,
            text_type,
            is_last_in_node,
            encoding,
            source_location,
        ));

        trace!(@output token);

        self.transform_controller.handle_token(&mut token)?;

        if self.emission_enabled {
            token.into_bytes(&mut |c| self.output_sink.handle_chunk(c))?;
        }
        Ok(())
    }

    #[inline]
    fn should_stop_removing_element_content(&self) -> bool {
        !self.emission_enabled && self.transform_controller.should_emit_content()
    }
}

impl<C, O> Dispatcher<C, O>
where
    C: TransformController,
    O: OutputSink,
{
    #[inline]
    pub fn new(transform_controller: C, output_sink: O, encoding: SharedEncoding) -> Self {
        let capture_flags = transform_controller.initial_capture_flags();

        Self {
            delegate: DispatcherDelegate {
                transform_controller,
                output_sink,
                capture_flags,
                remaining_content_start: 0,
                emission_enabled: true,
            },
            text_decoder: TextDecoder::new(SharedEncoding::clone(&encoding)),
            last_text_type: TextType::Data,
            encoding,
            got_flags_from_hint: false,
            pending_element_aux_info_req: None,
        }
    }

    #[inline(never)]
    fn try_produce_token_from_lexeme<'i, T>(&mut self, lexeme: &Lexeme<'i, T>) -> ActionResult
    where
        Lexeme<'i, T>: ToToken,
    {
        match lexeme.to_token(&mut self.delegate.capture_flags, self.encoding.get()) {
            ToTokenResult::Token(token) => {
                self.delegate.lexeme_consumed(lexeme);
                self.delegate.token_produced(token)?;
            }
            ToTokenResult::Text(text_type) => {
                self.delegate.lexeme_consumed(lexeme);
                self.last_text_type = text_type;
                self.text_decoder.feed_text(
                    lexeme.spanned(),
                    false,
                    &mut |text, is_last, encoding, source_location| {
                        self.delegate.text_token_produced(
                            text,
                            encoding,
                            self.last_text_type,
                            is_last,
                            source_location,
                        )
                    },
                )?;
            }
            ToTokenResult::None => {}
        }
        Ok(())
    }

    #[inline]
    const fn get_next_parser_directive(&self) -> ParserDirective {
        if !self.delegate.capture_flags.is_empty() {
            ParserDirective::Lex
        } else {
            ParserDirective::WherePossibleScanForTagsOnly
        }
    }

    fn adjust_capture_flags_for_tag_lexeme(&mut self, lexeme: &TagLexeme<'_>) -> ActionResult {
        let input = lexeme.input();

        macro_rules! get_flags_from_aux_info_res {
            ($handler:expr, $attributes:expr, $self_closing:expr) => {
                $handler(
                    &mut self.delegate.transform_controller,
                    AuxStartTagInfo {
                        input,
                        attr_buffer: $attributes,
                        self_closing: $self_closing,
                    },
                )
            };
        }

        let capture_flags = match self.pending_element_aux_info_req.take() {
            // NOTE: tag hint was produced for the tag, but
            // attributes and self closing flag were requested.
            Some(aux_info_req) => match *lexeme.token_outline() {
                TagTokenOutline::StartTag {
                    ref attributes,
                    self_closing,
                    ..
                } => {
                    get_flags_from_aux_info_res!(aux_info_req, &attributes, self_closing)
                }
                TagTokenOutline::EndTag { .. } => Err(ActionError::internal(
                    "Tag should be a start tag at this point",
                )),
            },

            // NOTE: tag hint hasn't been produced for the tag, because
            // parser is not in the tag scan mode.
            None => match *lexeme.token_outline() {
                TagTokenOutline::StartTag {
                    name,
                    name_hash,
                    ns,
                    ref attributes,
                    self_closing,
                } => {
                    let name = LocalName::new(*input, name, name_hash);

                    match self
                        .delegate
                        .transform_controller
                        .handle_start_tag(name, ns)
                    {
                        Ok(flags) => Ok(flags),
                        Err(DispatcherError::InfoRequest(aux_info_req)) => {
                            get_flags_from_aux_info_res!(aux_info_req, &attributes, self_closing)
                        }
                        Err(DispatcherError::RewritingError(e)) => Err(e.into()),
                    }
                }

                TagTokenOutline::EndTag { name, name_hash } => {
                    let name = LocalName::new(*input, name, name_hash);
                    Ok(self.delegate.transform_controller.handle_end_tag(name))
                }
            },
        };

        match capture_flags {
            Ok(flags) => {
                self.delegate.capture_flags = flags;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn apply_capture_flags_from_hint_and_get_next_parser_directive(
        &mut self,
        flags: TokenCaptureFlags,
    ) -> ParserDirective {
        self.delegate.capture_flags = flags;
        self.got_flags_from_hint = true;
        self.get_next_parser_directive()
    }

    /// Emit text chunk with is_last_in_node
    #[inline]
    fn flush_pending_captured_text(&mut self) -> Result<(), RewritingError> {
        self.text_decoder
            .flush_pending(&mut |text, is_last, encoding, source_location| {
                self.delegate.text_token_produced(
                    text,
                    encoding,
                    self.last_text_type,
                    is_last,
                    source_location,
                )
            })
    }

    pub fn flush_remaining_input(&mut self, input: &[u8], consumed_byte_count: usize) {
        self.delegate
            .flush_remaining_input(input, consumed_byte_count);
    }

    pub fn finish(&mut self, input: &[u8]) -> Result<(), RewritingError> {
        self.delegate.finish(self.encoding.get(), input)
    }
}

impl<C, O> LexemeSink for Dispatcher<C, O>
where
    C: TransformController,
    O: OutputSink,
{
    fn handle_tag(&mut self, lexeme: &TagLexeme<'_>) -> ActionResult<ParserDirective> {
        // NOTE: flush pending text before reporting tag to the transform controller.
        // Otherwise, transform controller can enable or disable text handlers too early.
        // In case of start tag, newly matched element text handlers
        // will receive leftovers from the previous match. And, in case of end tag,
        // handlers will be disabled before the receive the finalizing chunk.
        self.flush_pending_captured_text()?;

        if self.got_flags_from_hint {
            self.got_flags_from_hint = false;
        } else {
            self.adjust_capture_flags_for_tag_lexeme(lexeme)?;
        }

        if let TagTokenOutline::EndTag { .. } = lexeme.token_outline() {
            if self.delegate.should_stop_removing_element_content() {
                self.delegate.emission_enabled = true;
                self.delegate.remaining_content_start = lexeme.raw_range().start;
            }
        }

        self.try_produce_token_from_lexeme(lexeme)?;
        self.delegate.emission_enabled = self.delegate.transform_controller.should_emit_content();

        Ok(self.get_next_parser_directive())
    }

    #[inline]
    fn handle_non_tag_content(&mut self, lexeme: &NonTagContentLexeme<'_>) -> ActionResult {
        match lexeme.token_outline() {
            Some(NonTagContentTokenOutline::Text(_)) => {}
            // when it's None, it still needs a flush for CDATA
            _ => self.flush_pending_captured_text()?,
        }
        self.try_produce_token_from_lexeme(lexeme)
    }
}

impl<C, O> TagHintSink for Dispatcher<C, O>
where
    C: TransformController,
    O: OutputSink,
{
    fn handle_start_tag_hint(
        &mut self,
        name: LocalName<'_>,
        ns: Namespace,
    ) -> Result<ParserDirective, RewritingError> {
        match self
            .delegate
            .transform_controller
            .handle_start_tag(name, ns)
        {
            Ok(flags) => {
                Ok(self.apply_capture_flags_from_hint_and_get_next_parser_directive(flags))
            }
            Err(DispatcherError::InfoRequest(aux_info_req)) => {
                self.got_flags_from_hint = false;
                self.pending_element_aux_info_req = Some(aux_info_req);

                Ok(ParserDirective::Lex)
            }
            Err(DispatcherError::RewritingError(e)) => Err(e),
        }
    }

    fn handle_end_tag_hint(
        &mut self,
        name: LocalName<'_>,
    ) -> Result<ParserDirective, RewritingError> {
        self.flush_pending_captured_text()?;

        let mut flags = self.delegate.transform_controller.handle_end_tag(name);

        // NOTE: if emission was disabled (i.e. we've been removing element content)
        // we need to request the end tag lexeme, to ensure that we have it.
        // Otherwise, if we have unfinished end tag in the end of input we'll emit
        // it where we shouldn't.
        if self.delegate.should_stop_removing_element_content() {
            flags |= TokenCaptureFlags::NEXT_END_TAG;
        }

        Ok(self.apply_capture_flags_from_hint_and_get_next_parser_directive(flags))
    }
}

impl<C, O> ParserOutputSink for Dispatcher<C, O>
where
    C: TransformController,
    O: OutputSink,
{
}
