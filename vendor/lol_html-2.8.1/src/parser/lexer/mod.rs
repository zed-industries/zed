#[macro_use]
mod actions;

mod conditions;
mod lexeme;

pub(crate) use self::lexeme::*;
use crate::base::{Align, Bytes, Range};
use crate::html::{LocalNameHash, Namespace, TextType};
use crate::parser::state_machine::{ActionResult, FeedbackDirective, StateMachine, StateResult};
use crate::parser::{ParserContext, ParserDirective, ParsingAmbiguityError, TreeBuilderFeedback};

pub(crate) trait LexemeSink {
    fn handle_tag(&mut self, lexeme: &TagLexeme<'_>) -> ActionResult<ParserDirective>;
    fn handle_non_tag_content(&mut self, lexeme: &NonTagContentLexeme<'_>) -> ActionResult;
}

pub(crate) type State<S> = fn(&mut Lexer<S>, context: &mut ParserContext<S>, &[u8]) -> StateResult;

pub(crate) type AttributeBuffer = Vec<AttributeOutline>;

pub(crate) struct Lexer<S> {
    next_pos: usize,
    is_last_input: bool,
    lexeme_start: usize,
    token_part_start: usize,
    cdata_allowed: bool,
    state: State<S>,
    current_tag_token: Option<TagTokenOutline>,
    current_non_tag_content_token: Option<NonTagContentTokenOutline>,
    current_attr: Option<AttributeOutline>,
    last_start_tag_name_hash: LocalNameHash,
    closing_quote: u8,
    last_text_type: TextType,
    feedback_directive: FeedbackDirective,
}

impl<S: LexemeSink> Lexer<S> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_pos: 0,
            is_last_input: false,
            lexeme_start: 0,
            token_part_start: 0,
            cdata_allowed: false,
            state: Self::data_state,
            current_tag_token: None,
            current_non_tag_content_token: None,
            current_attr: None,
            last_start_tag_name_hash: LocalNameHash::default(),
            closing_quote: b'"',
            last_text_type: TextType::Data,
            feedback_directive: FeedbackDirective::None,
        }
    }

    fn try_get_tree_builder_feedback(
        &mut self,
        context: &mut ParserContext<S>,
        token: &TagTokenOutline,
    ) -> Result<Option<TreeBuilderFeedback>, ParsingAmbiguityError> {
        Ok(match self.feedback_directive.take() {
            FeedbackDirective::ApplyUnhandledFeedback(feedback) => Some(feedback),
            FeedbackDirective::Skip => None,
            FeedbackDirective::None => {
                Some({
                    match *token {
                        TagTokenOutline::StartTag { name_hash, .. } => context
                            .tree_builder_simulator
                            .get_feedback_for_start_tag(name_hash)?,
                        TagTokenOutline::EndTag { name_hash, .. } => context
                            .tree_builder_simulator
                            .get_feedback_for_end_tag(name_hash),
                    }
                })
            }
        })
    }

    fn handle_tree_builder_feedback(
        &mut self,
        context: &mut ParserContext<S>,
        feedback: TreeBuilderFeedback,
        lexeme: &TagLexeme<'_>,
    ) {
        match feedback {
            TreeBuilderFeedback::SwitchTextType(text_type) => self.set_last_text_type(text_type),
            TreeBuilderFeedback::SetAllowCdata(cdata_allowed) => self.cdata_allowed = cdata_allowed,
            TreeBuilderFeedback::RequestLexeme(mut callback) => {
                let feedback = callback(&mut context.tree_builder_simulator, lexeme);

                self.handle_tree_builder_feedback(context, feedback, lexeme);
            }
            TreeBuilderFeedback::None => (),
        }
    }

    #[inline]
    fn emit_lexeme(
        &mut self,
        context: &mut ParserContext<S>,
        lexeme: &NonTagContentLexeme<'_>,
    ) -> ActionResult {
        trace!(@output lexeme);

        self.lexeme_start = lexeme.raw_range().end;

        context.output_sink.handle_non_tag_content(lexeme)?;
        Ok(())
    }

    #[inline]
    fn emit_tag_lexeme(
        &mut self,
        context: &mut ParserContext<S>,
        lexeme: &TagLexeme<'_>,
    ) -> ActionResult<ParserDirective> {
        trace!(@output lexeme);

        self.lexeme_start = lexeme.raw_range().end;

        context.output_sink.handle_tag(lexeme)
    }

    #[inline]
    #[must_use]
    fn create_lexeme_with_raw<'i, T>(
        &self,
        previously_consumed_byte_count: usize,
        input: &'i [u8],
        token: T,
        raw_end: usize,
    ) -> Lexeme<'i, T> {
        Lexeme::new(
            previously_consumed_byte_count,
            Bytes::new(input),
            token,
            Range {
                start: self.lexeme_start,
                end: raw_end,
            },
        )
    }

    #[inline]
    #[must_use]
    fn create_lexeme_with_raw_inclusive<'i, T>(
        &self,
        previously_consumed_byte_count: usize,
        input: &'i [u8],
        token: T,
    ) -> Lexeme<'i, T> {
        let raw_end = self.pos() + 1;

        self.create_lexeme_with_raw(previously_consumed_byte_count, input, token, raw_end)
    }

    #[inline]
    #[must_use]
    fn create_lexeme_with_raw_exclusive<'i, T>(
        &self,
        previously_consumed_byte_count: usize,
        input: &'i [u8],
        token: T,
    ) -> Lexeme<'i, T> {
        let raw_end = self.pos();

        self.create_lexeme_with_raw(previously_consumed_byte_count, input, token, raw_end)
    }
}

impl<S: LexemeSink> StateMachine for Lexer<S> {
    impl_common_sm_accessors!();
    impl_common_input_cursor_methods!();

    #[inline]
    fn set_state(&mut self, state: State<S>) {
        self.state = state;
    }

    #[inline]
    fn state(&self) -> State<S> {
        self.state
    }

    #[inline]
    fn get_consumed_byte_count(&self, _input: &[u8]) -> usize {
        self.lexeme_start
    }

    fn adjust_for_next_input(&mut self) {
        self.token_part_start.align(self.lexeme_start);
        self.current_tag_token.align(self.lexeme_start);
        self.current_non_tag_content_token.align(self.lexeme_start);
        self.current_attr.align(self.lexeme_start);

        self.lexeme_start = 0;
    }

    #[inline]
    fn adjust_to_bookmark(&mut self, pos: usize, feedback_directive: FeedbackDirective) {
        self.lexeme_start = pos;
        self.feedback_directive = feedback_directive;
    }

    #[inline]
    fn enter_ch_sequence_matching(&mut self) {
        trace!(@noop);
    }

    #[inline]
    fn leave_ch_sequence_matching(&mut self) {
        trace!(@noop);
    }
}
