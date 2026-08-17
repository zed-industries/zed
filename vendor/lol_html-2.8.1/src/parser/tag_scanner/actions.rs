use super::*;
use crate::parser::ParserContext;
use crate::parser::state_machine::{ActionError, ActionResult, StateMachineActions};

impl<S: TagHintSink> StateMachineActions for TagScanner<S> {
    type Context = ParserContext<S>;

    impl_common_sm_actions!();

    #[inline]
    fn create_start_tag(&mut self, _context: &mut ParserContext<S>, _input: &[u8]) {
        self.tag_name_start = self.pos();
        self.tag_name_hash = LocalNameHash::new();
    }

    #[inline]
    fn create_end_tag(&mut self, _context: &mut ParserContext<S>, _input: &[u8]) {
        self.tag_name_start = self.pos();
        self.tag_name_hash = LocalNameHash::new();
        self.is_in_end_tag = true;
    }

    #[inline]
    fn mark_tag_start(&mut self, _context: &mut ParserContext<S>, _input: &[u8]) {
        self.tag_start = Some(self.pos());
    }

    #[inline]
    fn unmark_tag_start(&mut self, _context: &mut ParserContext<S>, _input: &[u8]) {
        self.tag_start = None;
    }

    #[inline]
    fn update_tag_name_hash(&mut self, _context: &mut ParserContext<S>, input: &[u8]) {
        if let Some(ch) = input.get(self.pos()).copied() {
            self.tag_name_hash.update(ch);
        }
    }

    #[inline]
    fn finish_tag_name(&mut self, context: &mut ParserContext<S>, input: &[u8]) -> ActionResult {
        let tag_start = self
            .tag_start
            .take()
            .ok_or_else(|| ActionError::internal("Tag start should be set at this point"))?;

        let unhandled_feedback = self.try_apply_tree_builder_feedback(context)?;

        let is_in_end_tag = self.is_in_end_tag;

        self.is_in_end_tag = false;

        if let Some(unhandled_feedback) = unhandled_feedback {
            return self.change_parser_directive(
                tag_start,
                ParserDirective::Lex,
                FeedbackDirective::ApplyUnhandledFeedback(unhandled_feedback),
            );
        }

        match self.emit_tag_hint(context, input, is_in_end_tag)? {
            ParserDirective::WherePossibleScanForTagsOnly => Ok(()),
            ParserDirective::Lex => {
                let feedback_directive = self.take_feedback_directive();

                self.change_parser_directive(tag_start, ParserDirective::Lex, feedback_directive)
            }
        }
    }

    #[inline]
    fn emit_tag(&mut self, _context: &mut ParserContext<S>, _input: &[u8]) -> ActionResult {
        // NOTE: exit from any non-initial text parsing mode always happens on tag emission
        // (except for CDATA, but there is a special action to take care of it).
        let text_type = self
            .pending_text_type_change
            .take()
            .unwrap_or(TextType::Data);

        self.set_last_text_type(text_type);

        Ok(())
    }

    noop_action_with_result!(
        emit_text_and_eof,
        emit_text,
        emit_current_token,
        emit_current_token_and_eof,
        emit_raw_without_token,
        emit_raw_without_token_and_eof
    );

    noop_action!(
        create_doctype,
        create_comment,
        start_token_part,
        mark_comment_text_end,
        set_force_quirks,
        finish_doctype_name,
        finish_doctype_public_id,
        finish_doctype_system_id,
        mark_as_self_closing,
        start_attr,
        finish_attr_name,
        finish_attr_value,
        finish_attr
    );

    #[inline]
    fn shift_comment_text_end_by(
        &mut self,
        _context: &mut ParserContext<S>,
        _input: &[u8],
        _offset: usize,
    ) {
        trace!(@noop);
    }
}
