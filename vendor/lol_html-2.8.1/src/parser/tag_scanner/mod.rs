#[macro_use]
mod actions;
mod conditions;

use crate::base::{Align, Bytes, Range};
use crate::html::{LocalName, LocalNameHash, Namespace, TextType};
use crate::parser::state_machine::{FeedbackDirective, StateMachine, StateResult};
use crate::parser::{ParserContext, ParserDirective, ParsingAmbiguityError, TreeBuilderFeedback};
use crate::rewriter::RewritingError;
use std::cmp::min;

pub(crate) trait TagHintSink {
    fn handle_start_tag_hint(
        &mut self,
        name: LocalName<'_>,
        ns: Namespace,
    ) -> Result<ParserDirective, RewritingError>;
    fn handle_end_tag_hint(
        &mut self,
        name: LocalName<'_>,
    ) -> Result<ParserDirective, RewritingError>;
}

pub(crate) type State<S> =
    fn(&mut TagScanner<S>, context: &mut ParserContext<S>, &[u8]) -> StateResult;

/// Tag scanner skips the majority of lexer operations and, thus,
/// is faster. It also has much less requirements for buffering which makes it more
/// prone to bailouts caused by buffer exhaustion (actually it buffers only tag names).
///
/// Tag scanner produces tag previews as an output which serve as a hint for
/// the matcher which can then switch to the lexer if required.
///
/// It's not guaranteed that tag preview will actually produce the token in the end
/// of the input (e.g. `<div` will produce a tag preview, but not tag token). However,
/// it's not a concern for our use case as no content will be erroneously captured
/// in this case.
pub(crate) struct TagScanner<S> {
    next_pos: usize,
    is_last_input: bool,
    tag_start: Option<usize>,
    ch_sequence_matching_start: Option<usize>,
    tag_name_start: usize,
    is_in_end_tag: bool,
    tag_name_hash: LocalNameHash,
    last_start_tag_name_hash: LocalNameHash,
    cdata_allowed: bool,
    state: State<S>,
    closing_quote: u8,
    pending_text_type_change: Option<TextType>,
    last_text_type: TextType,
}

impl<S: TagHintSink> TagScanner<S> {
    pub fn new() -> Self {
        Self {
            next_pos: 0,
            is_last_input: false,
            tag_start: None,
            ch_sequence_matching_start: None,
            tag_name_start: 0,
            is_in_end_tag: false,
            tag_name_hash: LocalNameHash::default(),
            last_start_tag_name_hash: LocalNameHash::default(),
            cdata_allowed: false,
            state: Self::data_state,
            closing_quote: b'"',
            pending_text_type_change: None,
            last_text_type: TextType::Data,
        }
    }

    fn emit_tag_hint(
        &mut self,
        context: &mut ParserContext<S>,
        input: &[u8],
        is_in_end_tag: bool,
    ) -> Result<ParserDirective, RewritingError> {
        let name_range = Range {
            start: self.tag_name_start,
            end: self.pos(),
        };

        let name = LocalName::new(Bytes::new(input), name_range, self.tag_name_hash);

        trace!(@output name);

        if is_in_end_tag {
            context.output_sink.handle_end_tag_hint(name)
        } else {
            self.last_start_tag_name_hash = self.tag_name_hash;

            let ns = context.tree_builder_simulator.current_ns();

            context.output_sink.handle_start_tag_hint(name, ns)
        }
    }

    #[inline]
    fn try_apply_tree_builder_feedback(
        &mut self,
        context: &mut ParserContext<S>,
    ) -> Result<Option<TreeBuilderFeedback>, ParsingAmbiguityError> {
        let feedback = if self.is_in_end_tag {
            context
                .tree_builder_simulator
                .get_feedback_for_end_tag(self.tag_name_hash)
        } else {
            context
                .tree_builder_simulator
                .get_feedback_for_start_tag(self.tag_name_hash)?
        };

        Ok(match feedback {
            TreeBuilderFeedback::SwitchTextType(text_type) => {
                // NOTE: we can't switch type immediately as we are in the middle of tag parsing.
                // So, we need to switch later on the `emit_tag` action.
                self.pending_text_type_change = Some(text_type);
                None
            }
            TreeBuilderFeedback::SetAllowCdata(cdata_allowed) => {
                self.cdata_allowed = cdata_allowed;
                None
            }
            TreeBuilderFeedback::RequestLexeme(_) => Some(feedback),
            TreeBuilderFeedback::None => None,
        })
    }

    #[inline]
    fn take_feedback_directive(&mut self) -> FeedbackDirective {
        self.pending_text_type_change
            .take()
            .map_or(FeedbackDirective::Skip, |text_type| {
                FeedbackDirective::ApplyUnhandledFeedback(TreeBuilderFeedback::SwitchTextType(
                    text_type,
                ))
            })
    }
}

impl<S: TagHintSink> StateMachine for TagScanner<S> {
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
    fn get_consumed_byte_count(&self, input: &[u8]) -> usize {
        // NOTE: if we are in character sequence matching we need
        // to block from the position where matching starts. We don't
        // need to do that manually in the lexer because it
        // always blocks all bytes starting from lexeme start and it's
        // guaranteed that character sequence matching occurs withih
        // lexeme boundaries.
        match (self.tag_start, self.ch_sequence_matching_start) {
            (Some(tag_start), Some(ch_sequence_matching_start)) => {
                min(tag_start, ch_sequence_matching_start)
            }
            (Some(tag_start), None) => tag_start,
            (None, Some(ch_sequence_matching_start)) => ch_sequence_matching_start,
            (None, None) => input.len(),
        }
    }

    fn adjust_for_next_input(&mut self) {
        if let Some(tag_start) = self.tag_start {
            self.tag_name_start.align(tag_start);
            self.tag_start = Some(0);
        }
    }

    #[inline]
    fn adjust_to_bookmark(&mut self, _pos: usize, _feedback_directive: FeedbackDirective) {
        trace!(@noop);
    }

    #[inline]
    fn enter_ch_sequence_matching(&mut self) {
        self.ch_sequence_matching_start = Some(self.pos());
    }

    #[inline]
    fn leave_ch_sequence_matching(&mut self) {
        self.ch_sequence_matching_start = None;
    }
}
