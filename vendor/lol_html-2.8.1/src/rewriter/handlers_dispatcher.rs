use super::ElementDescriptor;
use super::settings::*;
use crate::rewritable_units::{DocumentEnd, Element, StartTag, Token, TokenCaptureFlags};
use crate::selectors_vm::{MatchId, MatchInfo};
use std::num::NonZero;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SelectorHandlersLocator {
    pub element_handler_idx: Option<Locator>,
    pub comment_handler_idx: Option<Locator>,
    pub text_handler_idx: Option<Locator>,
}

/// It's index+1, allows efficient Option<Locator> (unfortunately Rust has no non-FFFF type)
pub(crate) type Locator = NonZero<u32>;

fn locator_to_idx(locator: Locator) -> usize {
    (locator.get() - 1) as usize
}

struct HandlerVecItem<H> {
    handler: H,
    user_count: u32,
}

struct HandlerVec<H> {
    items: Vec<HandlerVecItem<H>>,
    user_count: u32,
}

impl<H> Default for HandlerVec<H> {
    fn default() -> Self {
        Self {
            items: Vec::default(),
            user_count: 0,
        }
    }
}

impl<H> HandlerVec<H> {
    #[inline]
    pub fn push(&mut self, handler: H, always_active: bool) -> Option<Locator> {
        let item = HandlerVecItem {
            handler,
            user_count: u32::from(always_active),
        };

        self.user_count += item.user_count;
        self.items.push(item);

        let locator = self.items.len().try_into().ok().and_then(NonZero::new);
        debug_assert!(locator.is_some());
        locator
    }

    #[inline]
    pub fn inc_user_count(&mut self, idx: Locator) {
        let Some(item) = self.items.get_mut(locator_to_idx(idx)) else {
            debug_assert!(false);
            return;
        };
        item.user_count += 1;
        self.user_count += 1;
    }

    #[inline]
    pub fn dec_user_count(&mut self, idx: Locator) {
        let Some(item) = self.items.get_mut(locator_to_idx(idx)) else {
            debug_assert!(false);
            return;
        };
        debug_assert!(item.user_count > 0);
        debug_assert!(self.user_count > 0);
        item.user_count -= 1;
        self.user_count -= 1;
    }

    #[inline]
    pub const fn has_active(&self) -> bool {
        self.user_count > 0
    }

    #[inline]
    pub fn for_each_active(
        &mut self,
        mut cb: impl FnMut(&mut H) -> HandlerResult,
    ) -> HandlerResult {
        for item in &mut self.items {
            if item.user_count > 0 {
                cb(&mut item.handler)?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn do_for_each_active_and_deactivate(
        &mut self,
        mut cb: impl FnMut(&mut H) -> HandlerResult,
    ) -> HandlerResult {
        for item in &mut self.items {
            if item.user_count > 0 {
                cb(&mut item.handler)?;
                self.user_count -= item.user_count;
                item.user_count = 0;
            }
        }

        Ok(())
    }

    pub fn do_for_each_active_and_remove_tail(
        &mut self,
        mut cb: impl FnMut(H) -> HandlerResult,
    ) -> HandlerResult {
        // already-handled end tag handlers may be first, and they must not be removed
        if let Some(first) = self.items.iter().position(|item| item.user_count > 0) {
            // Must drop everything after, as remove() would change indexes anyway, breaking locators.
            // rev() is for backwards-compat with previous implementation.
            for item in self.items.drain(first..).rev() {
                if item.user_count > 0 {
                    self.user_count -= item.user_count;
                    cb(item.handler)?;
                }
            }
        }
        debug_assert_eq!(self.user_count, 0);
        Ok(())
    }
}

pub(crate) struct ContentHandlersDispatcher<'h, H: HandlerTypes> {
    doctype_handlers: HandlerVec<H::DoctypeHandler<'h>>,
    comment_handlers: HandlerVec<H::CommentHandler<'h>>,
    text_handlers: HandlerVec<H::TextHandler<'h>>,
    end_tag_handlers: HandlerVec<H::EndTagHandler<'static>>,
    element_handlers: HandlerVec<H::ElementHandler<'h>>,
    end_handlers: HandlerVec<H::EndHandler<'h>>,
    next_element_can_have_content: bool,
    matched_elements_with_removed_content: usize,
    /// Dense index by match_id
    locators: Vec<SelectorHandlersLocator>,
}

impl<H: HandlerTypes> Default for ContentHandlersDispatcher<'_, H> {
    fn default() -> Self {
        ContentHandlersDispatcher {
            doctype_handlers: Default::default(),
            comment_handlers: Default::default(),
            text_handlers: Default::default(),
            end_tag_handlers: Default::default(),
            element_handlers: Default::default(),
            end_handlers: Default::default(),
            next_element_can_have_content: false,
            matched_elements_with_removed_content: 0,
            locators: Vec::new(),
        }
    }
}

impl<'h, H: HandlerTypes> ContentHandlersDispatcher<'h, H> {
    #[inline]
    pub fn add_document_content_handlers(&mut self, handlers: DocumentContentHandlers<'h, H>) {
        if let Some(handler) = handlers.doctype {
            self.doctype_handlers.push(handler, true);
        }

        if let Some(handler) = handlers.comments {
            self.comment_handlers.push(handler, true);
        }

        if let Some(handler) = handlers.text {
            self.text_handlers.push(handler, true);
        }

        if let Some(handler) = handlers.end {
            self.end_handlers.push(handler, true);
        }
    }

    #[inline]
    pub fn add_selector_associated_handlers(
        &mut self,
        handlers: ElementContentHandlers<'h, H>,
    ) -> MatchId {
        let match_id = self.locators.len() as MatchId;
        self.locators.push(SelectorHandlersLocator {
            element_handler_idx: handlers
                .element
                .and_then(|h| self.element_handlers.push(h, false)),
            comment_handler_idx: handlers
                .comments
                .and_then(|h| self.comment_handlers.push(h, false)),
            text_handler_idx: handlers
                .text
                .and_then(|h| self.text_handlers.push(h, false)),
        });
        match_id
    }

    #[inline]
    pub const fn has_matched_elements_with_removed_content(&self) -> bool {
        self.matched_elements_with_removed_content > 0
    }

    #[inline]
    pub fn start_matching(&mut self, match_info: &MatchInfo) {
        let Some(locator) = self.locators.get(match_info.match_id as usize) else {
            debug_assert!(false);
            return;
        };

        if match_info.with_content {
            if let Some(idx) = locator.comment_handler_idx {
                self.comment_handlers.inc_user_count(idx);
            }

            if let Some(idx) = locator.text_handler_idx {
                self.text_handlers.inc_user_count(idx);
            }
        }

        if let Some(idx) = locator.element_handler_idx {
            self.element_handlers.inc_user_count(idx);
        }

        self.next_element_can_have_content = match_info.with_content;
    }

    #[inline]
    pub fn stop_matching(&mut self, elem_desc: ElementDescriptor) {
        for match_id in elem_desc.matched_content_handlers.iter() {
            let Some(locator) = self.locators.get(match_id as usize) else {
                debug_assert!(false);
                continue;
            };

            if let Some(idx) = locator.comment_handler_idx {
                self.comment_handlers.dec_user_count(idx);
            }

            if let Some(idx) = locator.text_handler_idx {
                self.text_handlers.dec_user_count(idx);
            }
        }

        if let Some(idx) = elem_desc.end_tag_handler_idx {
            self.end_tag_handlers.inc_user_count(idx);
        }

        if elem_desc.remove_content {
            self.matched_elements_with_removed_content -= 1;
        }
    }

    pub fn handle_start_tag(
        &mut self,
        start_tag: &mut StartTag<'_>,
        current_element_data: Option<&mut ElementDescriptor>,
    ) -> HandlerResult {
        if self.matched_elements_with_removed_content > 0 {
            start_tag.remove();
        }

        let mut element = Element::new(start_tag, self.next_element_can_have_content);

        self.element_handlers
            .do_for_each_active_and_deactivate(|h| h(&mut element))?;

        if self.next_element_can_have_content {
            if let Some(elem_desc) = current_element_data {
                if element.should_remove_content() {
                    elem_desc.remove_content = true;
                    self.matched_elements_with_removed_content += 1;
                }

                debug_assert!(element.can_have_content());
                if let Some(handler) = element.into_end_tag_handler() {
                    elem_desc.end_tag_handler_idx = self.end_tag_handlers.push(handler, false);
                }
            }
        }

        Ok(())
    }

    pub fn handle_token(
        &mut self,
        token: &mut Token<'_>,
        current_element_data: Option<&mut ElementDescriptor>,
    ) -> HandlerResult {
        match token {
            Token::Doctype(doctype) => self.doctype_handlers.for_each_active(|h| h(doctype)),
            Token::StartTag(start_tag) => self.handle_start_tag(start_tag, current_element_data),
            Token::EndTag(end_tag) => self
                .end_tag_handlers
                .do_for_each_active_and_remove_tail(|h| h(end_tag)),
            Token::TextChunk(text) => self.text_handlers.for_each_active(|h| h(text)),
            Token::Comment(comment) => self.comment_handlers.for_each_active(|h| h(comment)),
        }
    }

    pub fn handle_end(&mut self, document_end: &mut DocumentEnd<'_>) -> HandlerResult {
        self.end_handlers
            .do_for_each_active_and_remove_tail(|h| h(document_end))
    }

    #[inline]
    pub fn get_token_capture_flags(&self) -> TokenCaptureFlags {
        let mut flags = TokenCaptureFlags::empty();

        if self.doctype_handlers.has_active() {
            flags |= TokenCaptureFlags::DOCTYPES;
        }

        if self.comment_handlers.has_active() {
            flags |= TokenCaptureFlags::COMMENTS;
        }

        if self.text_handlers.has_active() {
            flags |= TokenCaptureFlags::TEXT;
        }

        if self.end_tag_handlers.has_active() {
            flags |= TokenCaptureFlags::NEXT_END_TAG;
        }

        if self.element_handlers.has_active() {
            flags |= TokenCaptureFlags::NEXT_START_TAG;
        }

        flags
    }
}

#[test]
fn locator_size() {
    assert!(size_of::<SelectorHandlersLocator>() <= 12);
}
