use std::{ops::Deref, rc::Rc};

use crate::{
    div, opaque_grey, white, AnyElement, AppContext, Element, FocusHandle, InteractiveElement,
    ParentElement, PromptLevel, StatefulInteractiveElement, Styled, Window,
};

/// Use this function in conjunction with [AppContext::set_prompt_renderer] to force
/// GPUI to always use the fallback prompt renderer.
pub fn fallback_prompt_renderer(
    _level: PromptLevel,
    message: &str,
    detail: Option<&str>,
    actions: &[&str],
    focus_handle: FocusHandle,
    confirm: Rc<dyn Fn(usize, &mut Window)>,
    _window: &mut Window,
    _cx: &mut AppContext,
) -> Box<dyn Fn(&mut Window, &mut AppContext) -> AnyElement> {
    let message = message.to_string();
    let detail = detail.map(|s| s.to_string());
    let actions: Vec<String> = actions.iter().map(|&s| s.to_string()).collect();

    Box::new(move |_window, _cx| {
        let prompt = div()
            .cursor_default()
            .track_focus(&focus_handle)
            .w_72()
            .bg(white())
            .rounded_lg()
            .overflow_hidden()
            .p_3()
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_around()
                    .child(div().overflow_hidden().child(message.clone())),
            )
            .children(detail.clone().map(|detail| {
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_around()
                    .text_sm()
                    .mb_2()
                    .child(div().child(detail))
            }))
            .children(actions.iter().enumerate().map(|(ix, action)| {
                div()
                    .flex()
                    .flex_row()
                    .justify_around()
                    .border_1()
                    .border_color(opaque_grey(0.2, 0.5))
                    .mt_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .text_sm()
                    .child(action.clone())
                    .id(ix)
                    .on_click({
                        let confirm = confirm.clone();
                        move |_, window, _| confirm(ix, window)
                    })
            }));

        div()
            .size_full()
            .child(
                div()
                    .size_full()
                    .bg(opaque_grey(0.5, 0.6))
                    .absolute()
                    .top_0()
                    .left_0(),
            )
            .child(
                div()
                    .size_full()
                    .absolute()
                    .top_0()
                    .left_0()
                    .flex()
                    .flex_col()
                    .justify_around()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .justify_around()
                            .child(prompt),
                    ),
            )
            .into_any()
    })
}

pub(crate) enum PromptBuilder {
    Default,
    Custom(
        Box<
            dyn Fn(
                PromptLevel,
                &str,
                Option<&str>,
                &[&str],
                FocusHandle,
                Rc<dyn Fn(usize, &mut Window)>,
                &mut Window,
                &mut AppContext,
            ) -> Box<dyn Fn(&mut Window, &mut AppContext) -> AnyElement>,
        >,
    ),
}

impl Deref for PromptBuilder {
    type Target = dyn Fn(
        PromptLevel,
        &str,
        Option<&str>,
        &[&str],
        FocusHandle,
        Rc<dyn Fn(usize, &mut Window)>,
        &mut Window,
        &mut AppContext,
    ) -> Box<dyn Fn(&mut Window, &mut AppContext) -> AnyElement>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Default => &fallback_prompt_renderer,
            Self::Custom(f) => f,
        }
    }
}
