use super::handlers_dispatcher::{ContentHandlersDispatcher, Locator};
use super::{HandlerTypes, RewritingError, Settings};
use crate::base::SharedEncoding;
use crate::html::{LocalName, Namespace};
use crate::memory::SharedMemoryLimiter;
use crate::parser::ActionError;
use crate::rewritable_units::{DocumentEnd, Token, TokenCaptureFlags};
use crate::selectors_vm::{
    Ast, AuxStartTagInfoRequest, DenseHashSet, ElementData, SelectorMatchingVm, VmError,
};
use crate::transform_stream::{DispatcherError, StartTagHandlingResult, TransformController};

pub(crate) struct ElementDescriptor {
    pub matched_content_handlers: DenseHashSet,
    pub end_tag_handler_idx: Option<Locator>,
    pub remove_content: bool,
}

impl ElementData for ElementDescriptor {
    #[inline]
    fn matched_ids_mut(&mut self) -> &mut DenseHashSet {
        &mut self.matched_content_handlers
    }

    #[inline]
    fn new() -> Self {
        Self {
            matched_content_handlers: DenseHashSet::new(),
            end_tag_handler_idx: None,
            remove_content: false,
        }
    }
}

pub(crate) struct HtmlRewriteController<'h, H: HandlerTypes> {
    handlers_dispatcher: ContentHandlersDispatcher<'h, H>,
    selector_matching_vm: Option<SelectorMatchingVm<ElementDescriptor>>,
}

impl<'h, H: HandlerTypes> HtmlRewriteController<'h, H> {
    // `HtmlRewriter::new` has a generic `OutputSink`, so inlining this method
    // would needlessly duplicate the code for every sink.
    #[inline(never)]
    pub(super) fn from_settings(
        settings: Settings<'h, '_, H>,
        memory_limiter: &SharedMemoryLimiter,
        encoding: &SharedEncoding,
    ) -> Self {
        let mut selectors_ast = Ast::default();
        let mut dispatcher = ContentHandlersDispatcher::<H>::default();
        let has_selectors =
            !settings.element_content_handlers.is_empty() || settings.adjust_charset_on_meta_tag;

        let charset_adjust_handler = if settings.adjust_charset_on_meta_tag {
            let encoding = SharedEncoding::clone(encoding);
            Some(super::handler_adjust_charset_on_meta_tag(encoding))
        } else {
            None
        };

        let element_content_handlers = charset_adjust_handler
            .into_iter()
            .chain(settings.element_content_handlers);

        for (selector, handlers) in element_content_handlers {
            let match_id = dispatcher.add_selector_associated_handlers(handlers);

            selectors_ast.add_selector(&selector, match_id);
        }

        for handlers in settings.document_content_handlers {
            dispatcher.add_document_content_handlers(handlers);
        }

        let selector_matching_vm = if has_selectors {
            Some(SelectorMatchingVm::new(
                selectors_ast,
                settings.encoding.into(),
                memory_limiter.clone(),
                settings.enable_esi_tags,
            ))
        } else {
            None
        };

        Self::new(dispatcher, selector_matching_vm)
    }

    #[inline]
    pub(crate) const fn new(
        handlers_dispatcher: ContentHandlersDispatcher<'h, H>,
        selector_matching_vm: Option<SelectorMatchingVm<ElementDescriptor>>,
    ) -> Self {
        HtmlRewriteController {
            handlers_dispatcher,
            selector_matching_vm,
        }
    }
}

impl<H: HandlerTypes> HtmlRewriteController<'_, H> {
    #[inline]
    fn respond_to_aux_info_request(
        aux_info_req: AuxStartTagInfoRequest<ElementDescriptor>,
    ) -> StartTagHandlingResult<Self> {
        Err(DispatcherError::InfoRequest(Box::new(
            move |this, aux_info| {
                let Some(vm) = &mut this.selector_matching_vm else {
                    debug_assert!(false);
                    return Err(ActionError::internal("vm req without vm"));
                };
                let mut match_handler = |m| this.handlers_dispatcher.start_matching(&m);

                aux_info_req(vm, aux_info, &mut match_handler)
                    .map_err(RewritingError::MemoryLimitExceeded)?;

                Ok(this.get_capture_flags())
            },
        )))
    }

    #[inline]
    fn get_capture_flags(&self) -> TokenCaptureFlags {
        self.handlers_dispatcher.get_token_capture_flags()
    }
}

impl<H: HandlerTypes> TransformController for HtmlRewriteController<'_, H> {
    #[inline]
    fn initial_capture_flags(&self) -> TokenCaptureFlags {
        self.get_capture_flags()
    }

    fn handle_start_tag(
        &mut self,
        local_name: LocalName<'_>,
        ns: Namespace,
    ) -> StartTagHandlingResult<Self> {
        match self.selector_matching_vm {
            Some(ref mut vm) => {
                let mut match_handler = |m| self.handlers_dispatcher.start_matching(&m);

                match vm.exec_for_start_tag(local_name, ns, &mut match_handler) {
                    Ok(()) => Ok(self.get_capture_flags()),
                    Err(VmError::InfoRequest(req)) => Self::respond_to_aux_info_request(req),
                    Err(VmError::MemoryLimitExceeded(e)) => Err(DispatcherError::RewritingError(
                        RewritingError::MemoryLimitExceeded(e),
                    )),
                }
            }
            // NOTE: fast path - we can skip executing selector matching VM completely
            // and don't need to maintain open element stack if we don't have any selectors.
            None => Ok(self.get_capture_flags()),
        }
    }

    fn handle_end_tag(&mut self, local_name: LocalName<'_>) -> TokenCaptureFlags {
        if let Some(ref mut vm) = self.selector_matching_vm {
            vm.exec_for_end_tag(local_name, |elem_desc| {
                self.handlers_dispatcher.stop_matching(elem_desc);
            });
        }

        self.get_capture_flags()
    }

    #[inline]
    fn handle_token(&mut self, token: &mut Token<'_>) -> Result<(), RewritingError> {
        let current_element_data = self
            .selector_matching_vm
            .as_mut()
            .and_then(SelectorMatchingVm::current_element_data_mut);

        self.handlers_dispatcher
            .handle_token(token, current_element_data)
            .map_err(RewritingError::ContentHandlerError)
    }

    fn handle_end(&mut self, document_end: &mut DocumentEnd<'_>) -> Result<(), RewritingError> {
        self.handlers_dispatcher
            .handle_end(document_end)
            .map_err(RewritingError::ContentHandlerError)
    }

    #[inline]
    fn should_emit_content(&self) -> bool {
        !self
            .handlers_dispatcher
            .has_matched_elements_with_removed_content()
    }
}
