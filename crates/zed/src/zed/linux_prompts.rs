use gpui::{
    div, opaque_grey, AppContext, EventEmitter, FocusHandle, FocusableView, FontWeight,
    InteractiveElement, IntoElement, ParentElement, PromptHandle, PromptLevel, PromptResponse,
    Render, RenderablePromptHandle, Styled, ViewContext, VisualContext, WindowContext,
};
use ui::{
    h_flex, v_flex, ButtonCommon, ButtonStyle, Clickable, ElevationIndex, FluentBuilder, LabelSize,
    TintColor,
};
use workspace::ui::StyledExt;

pub fn init(cx: &mut AppContext) {
    cx.set_prompt_builder(fallback_prompt_renderer)
}
/// Use this function in conjunction with [AppContext::set_prompt_renderer] to force
/// GPUI to always use the fallback prompt renderer.
pub fn fallback_prompt_renderer(
    level: PromptLevel,
    message: &str,
    detail: Option<&str>,
    actions: &[&str],
    handle: PromptHandle,
    cx: &mut WindowContext,
) -> RenderablePromptHandle {
    let renderer = cx.new_view({
        |cx| FallbackPromptRenderer {
            _level: level,
            message: message.to_string(),
            detail: detail.map(ToString::to_string),
            actions: actions.iter().map(ToString::to_string).collect(),
            focus: cx.focus_handle(),
        }
    });

    handle.with_view(renderer, cx)
}

/// The default GPUI fallback for rendering prompts, when the platform doesn't support it.
pub struct FallbackPromptRenderer {
    _level: PromptLevel,
    message: String,
    detail: Option<String>,
    actions: Vec<String>,
    focus: FocusHandle,
}
impl FallbackPromptRenderer {
    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        cx.emit(PromptResponse(0));
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.actions.iter().position(|a| a == "Cancel") {
            cx.emit(PromptResponse(ix));
        }
    }
}
impl Render for FallbackPromptRenderer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let prompt = v_flex()
            .key_context("Prompt")
            .cursor_default()
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .elevation_3(cx)
            .w_72()
            .overflow_hidden()
            .p_4()
            .gap_4()
            .font_family("Zed Sans")
            .child(
                div()
                    .w_full()
                    .font_weight(FontWeight::BOLD)
                    .child(self.message.clone())
                    .text_color(ui::Color::Default.color(cx)),
            )
            .children(self.detail.clone().map(|detail| {
                div()
                    .w_full()
                    .text_xs()
                    .text_color(ui::Color::Muted.color(cx))
                    .child(detail)
            }))
            .child(h_flex().justify_end().gap_2().children(
                self.actions.iter().enumerate().rev().map(|(ix, action)| {
                    ui::Button::new(ix, action.clone())
                        .label_size(LabelSize::Large)
                        .style(ButtonStyle::Filled)
                        .when(ix == 0, |el| {
                            el.style(ButtonStyle::Tinted(TintColor::Accent))
                        })
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(cx.listener(move |_, _, cx| {
                            cx.emit(PromptResponse(ix));
                        }))
                }),
            ));

        div()
            .size_full()
            .occlude()
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
    }
}

impl EventEmitter<PromptResponse> for FallbackPromptRenderer {}

impl FocusableView for FallbackPromptRenderer {
    fn focus_handle(&self, _: &crate::AppContext) -> FocusHandle {
        self.focus.clone()
    }
}
