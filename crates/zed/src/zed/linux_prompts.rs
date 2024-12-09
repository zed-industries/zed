use gpui::{
    div, AppContext, EventEmitter, FocusHandle, FocusableView, FontWeight, InteractiveElement,
    IntoElement, ParentElement, PromptHandle, PromptLevel, PromptResponse, Refineable, Render,
    RenderablePromptHandle, Styled, TextStyleRefinement, View, ViewContext, VisualContext,
    WindowContext,
};
use markdown::{Markdown, MarkdownStyle};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    h_flex, v_flex, ActiveTheme, ButtonCommon, ButtonStyle, Clickable, ElevationIndex,
    FluentBuilder, LabelSize, TintColor,
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
            actions: actions.iter().map(ToString::to_string).collect(),
            focus: cx.focus_handle(),
            active_action_id: 0,
            detail: detail.filter(|text| !text.is_empty()).map(|text| {
                cx.new_view(|cx| {
                    let settings = ThemeSettings::get_global(cx);
                    let mut base_text_style = cx.text_style();
                    base_text_style.refine(&TextStyleRefinement {
                        font_family: Some(settings.ui_font.family.clone()),
                        font_size: Some(settings.ui_font_size.into()),
                        color: Some(ui::Color::Muted.color(cx)),
                        ..Default::default()
                    });
                    let markdown_style = MarkdownStyle {
                        base_text_style,
                        selection_background_color: { cx.theme().players().local().selection },
                        ..Default::default()
                    };
                    Markdown::new(text.to_string(), markdown_style, None, None, cx)
                })
            }),
        }
    });

    handle.with_view(renderer, cx)
}

/// The default GPUI fallback for rendering prompts, when the platform doesn't support it.
pub struct FallbackPromptRenderer {
    _level: PromptLevel,
    message: String,
    actions: Vec<String>,
    focus: FocusHandle,
    active_action_id: usize,
    detail: Option<View<Markdown>>,
}

impl FallbackPromptRenderer {
    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        cx.emit(PromptResponse(self.active_action_id));
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.actions.iter().position(|a| a == "Cancel") {
            cx.emit(PromptResponse(ix));
        }
    }

    fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        self.active_action_id = self.actions.len().saturating_sub(1);
        cx.notify();
    }

    fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        self.active_action_id = 0;
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        if self.active_action_id > 0 {
            self.active_action_id -= 1;
        } else {
            self.active_action_id = self.actions.len().saturating_sub(1);
        }
        cx.notify();
    }

    fn select_prev(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        self.active_action_id = (self.active_action_id + 1) % self.actions.len();
        cx.notify();
    }
}

impl Render for FallbackPromptRenderer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let font_family = settings.ui_font.family.clone();
        let prompt = v_flex()
            .key_context("Prompt")
            .cursor_default()
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .elevation_3(cx)
            .w_72()
            .overflow_hidden()
            .p_4()
            .gap_4()
            .font_family(font_family)
            .child(
                div()
                    .w_full()
                    .font_weight(FontWeight::BOLD)
                    .child(self.message.clone())
                    .text_color(ui::Color::Default.color(cx)),
            )
            .children(
                self.detail
                    .clone()
                    .map(|detail| div().w_full().text_xs().child(detail)),
            )
            .child(h_flex().justify_end().gap_2().children(
                self.actions.iter().enumerate().rev().map(|(ix, action)| {
                    ui::Button::new(ix, action.clone())
                        .label_size(LabelSize::Large)
                        .style(ButtonStyle::Filled)
                        .when(ix == self.active_action_id, |el| {
                            el.style(ButtonStyle::Tinted(TintColor::Accent))
                        })
                        .layer(ElevationIndex::ModalSurface)
                        .on_click(cx.listener(move |_, _, cx| {
                            cx.emit(PromptResponse(ix));
                        }))
                }),
            ));

        div().size_full().occlude().child(
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
