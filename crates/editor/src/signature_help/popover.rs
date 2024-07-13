use crate::{Editor, EditorStyle};
use gpui::{
    div, AnyElement, InteractiveElement, IntoElement, ParentElement, Pixels, Size, Styled,
    ViewContext, WeakView,
};
use language::ParsedMarkdown;
use std::cell::RefCell;
use std::rc::Rc;
use ui::{Button, ButtonCommon, ButtonSize, Clickable, Disableable, Label, StyledExt};
use workspace::Workspace;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SignatureHelpPopover {
    signature_help_markdowns: Vec<SignatureHelpMarkdown>,
    current_page: Rc<RefCell<usize>>,
}

#[derive(Clone, Debug)]
pub struct SignatureHelpMarkdown {
    pub signature: ParsedMarkdown,
    pub signature_description: Option<ParsedMarkdown>,
}

impl PartialEq for SignatureHelpMarkdown {
    fn eq(&self, other: &Self) -> bool {
        let signature_str_equality = self.signature.text.as_str() == other.signature.text.as_str();
        let signature_highlight_equality = self.signature.highlights == other.signature.highlights;

        let signature_description_str_equality = match (
            self.signature_description.as_ref(),
            other.signature_description.as_ref(),
        ) {
            (Some(text), Some(other_text)) => text.text.as_str() == other_text.text.as_str(),
            (None, None) => true,
            _ => false,
        };
        signature_str_equality && signature_highlight_equality && signature_description_str_equality
    }
}

impl SignatureHelpPopover {
    pub fn new(
        signature_help_markdowns: Vec<SignatureHelpMarkdown>,
        active_signature: usize,
    ) -> Self {
        Self {
            signature_help_markdowns,
            current_page: Rc::new(RefCell::new(active_signature)),
        }
    }

    fn has_next_page(&self) -> bool {
        let current_page = *self.current_page.borrow();
        current_page + 1 < self.signature_help_markdowns.len()
    }

    fn has_previous_page(&self) -> bool {
        let current_page = *self.current_page.borrow();
        current_page > 0
    }

    pub fn render(
        &mut self,
        style: &EditorStyle,
        max_size: Size<Pixels>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Editor>,
    ) -> AnyElement {
        let Some(SignatureHelpMarkdown {
            signature,
            signature_description,
        }) = self
            .signature_help_markdowns
            .get(*self.current_page.borrow())
        else {
            return div().into_any_element();
        };

        let signature_element = div()
            .id("signature_help_popover")
            .max_w(max_size.width)
            .child(div().p_2().child(crate::render_parsed_markdown(
                "signature_help_popover_content",
                signature,
                style,
                workspace.clone(),
                cx,
            )))
            .into_any_element();
        let boarder = div().border_primary(cx).border_1().into_any_element();

        let signature_help_children = if let Some(signature_description) = signature_description {
            let signature_description_element = div()
                .id("signature_help_popover_description")
                .child(div().p_2().child(crate::render_parsed_markdown(
                    "signature_help_popover_description_content",
                    signature_description,
                    style,
                    workspace.clone(),
                    cx,
                )))
                .into_any_element();
            vec![signature_element, boarder, signature_description_element]
        } else {
            vec![signature_element]
        };
        let signature_help = div()
            .flex()
            .flex_col()
            .children(signature_help_children)
            .into_any_element();

        if self.signature_help_markdowns.len() > 1 {
            let previous_button = div().flex().flex_row().justify_center().child({
                let current_page = self.current_page.clone();
                Button::new("popover_page_button_previous", "↑")
                    .size(ButtonSize::Compact)
                    .disabled(!self.has_previous_page())
                    .on_click(move |_, _| {
                        *current_page.borrow_mut() -= 1;
                    })
                    .into_any_element()
            });

            let page = div()
                .flex()
                .flex_row()
                .justify_center()
                .child(Label::new(format!(
                    "{} / {}",
                    *self.current_page.borrow() + 1,
                    self.signature_help_markdowns.len()
                )));

            let next_button = div().flex().flex_row().justify_center().child({
                let current_page = self.current_page.clone();
                Button::new("popover_page_button_next", "↓")
                    .size(ButtonSize::Compact)
                    .disabled(!self.has_next_page())
                    .on_click(move |_, _| {
                        *current_page.borrow_mut() += 1;
                    })
                    .into_any_element()
            });
            let buttons = div()
                .flex()
                .child(div().p_1().flex().flex_col_reverse().children([
                    next_button,
                    page,
                    previous_button,
                ]))
                .into_any_element();

            let boarder = div().border_primary(cx).border_1().into_any_element();

            div()
                .elevation_2(cx)
                .flex()
                .flex_row()
                .children([buttons, boarder, signature_help])
                .into_any_element()
        } else {
            div()
                .elevation_2(cx)
                .child(signature_help)
                .into_any_element()
        }
    }
}

#[cfg(test)]
impl SignatureHelpPopover {
    pub fn signature_help_markdowns(&self) -> &[SignatureHelpMarkdown] {
        &self.signature_help_markdowns
    }

    pub fn current_page(&self) -> usize {
        *self.current_page.borrow()
    }
}
