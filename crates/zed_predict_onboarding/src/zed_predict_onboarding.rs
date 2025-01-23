//! AI service Terms of Service acceptance modal.

use std::{sync::Arc, time::Duration};

use client::UserStore;
use fs::Fs;
use gpui::{
    ease_in_out, svg, Animation, AnimationExt as _, AppContext, ClickEvent, DismissEvent,
    EventEmitter, FocusHandle, FocusableView, Model, MouseDownEvent, Render, View,
};
use language::language_settings::{AllLanguageSettings, InlineCompletionProvider};
use settings::{update_settings_file, Settings};
use ui::{prelude::*, TintColor};
use workspace::{ModalView, Workspace};

/// Terms of acceptance for AI inline prediction.
pub struct ZedPredictOnboarding {
    workspace: View<Workspace>,
    user_store: Model<UserStore>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
}

impl ZedPredictOnboarding {
    fn new(
        workspace: View<Workspace>,
        user_store: Model<UserStore>,
        fs: Arc<dyn Fs>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        ZedPredictOnboarding {
            workspace,
            user_store,
            fs,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn toggle(
        workspace: View<Workspace>,
        user_store: Model<UserStore>,
        fs: Arc<dyn Fs>,
        cx: &mut WindowContext,
    ) {
        workspace.update(cx, |this, cx| {
            let workspace = cx.view().clone();
            this.toggle_modal(cx, |cx| {
                ZedPredictOnboarding::new(workspace, user_store, fs, cx)
            });
        });
    }

    fn view_terms(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        cx.open_url("https://zed.dev/terms-of-service");
        cx.notify();
    }

    fn view_blog(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        cx.open_url("https://zed.dev/blog/"); // TODO Add the link when live
        cx.notify();
    }

    fn accept_and_enable(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        let task = self
            .user_store
            .update(cx, |this, cx| this.accept_terms_of_service(cx));

        let workspace = self.workspace.clone();

        cx.spawn(|this, mut cx| async move {
            match task.await {
                Ok(_) => this.update(&mut cx, |this, cx| {
                    update_settings_file::<AllLanguageSettings>(
                        this.fs.clone(),
                        cx,
                        move |file, _| {
                            file.features
                                .get_or_insert(Default::default())
                                .inline_completion_provider = Some(InlineCompletionProvider::Zed);
                        },
                    );

                    cx.emit(DismissEvent);
                }),
                Err(err) => workspace.update(&mut cx, |this, cx| {
                    this.show_error(&err, cx);
                }),
            }
        })
        .detach_and_log_err(cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ZedPredictOnboarding {}

impl FocusableView for ZedPredictOnboarding {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ZedPredictOnboarding {}

impl Render for ZedPredictOnboarding {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w_96()
            .p_4()
            .relative()
            .gap_2()
            .overflow_hidden()
            .elevation_3(cx)
            .id("zed predict tos")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::cancel))
            .key_context("ZedPredictOnboarding")
            .on_action(cx.listener(|_, _: &menu::Cancel, cx| {
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, cx| {
                cx.focus(&this.focus_handle);
            }))
            .child(
                h_flex()
                    .max_h_32()
                    .p_1()
                    .overflow_hidden()
                    .flex_wrap()
                    .gap_1()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    // TODO: replace with single SVG
                    .children((0..254).enumerate().map(|(index, _)| {
                        let opacity = 0.24 - (index as f32 * 0.0016);
                        svg()
                            .path("icons/zed_predict.svg")
                            .text_color(cx.theme().colors().icon_disabled)
                            .w(px(14.))
                            .h(px(14.))
                            .opacity(opacity.max(0.001))
                    })),
            )
            .child(
                h_flex()
                    .w_full()
                    .mb_2()
                    .justify_between()
                    .child(
                        v_flex()
                            .child(
                                Label::new("Introducing Zed AI's")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(h_flex().child(Headline::new("Edit Prediction"))),
                    )
                    .child({
                        let tab = |n: usize| {
                            let text_color = cx.theme().colors().text;
                            let border_color = cx.theme().colors().text_accent.opacity(0.4);

                            h_flex().child(
                                h_flex()
                                    .px_4()
                                    .py_0p5()
                                    .bg(cx.theme().colors().editor_background)
                                    .border_1()
                                    .border_color(border_color)
                                    .rounded_md()
                                    .font(theme::ThemeSettings::get_global(cx).buffer_font.clone())
                                    .text_size(TextSize::XSmall.rems(cx))
                                    .text_color(text_color)
                                    .child("tab")
                                    .with_animation(
                                        ElementId::Integer(n),
                                        Animation::new(Duration::from_secs(2)).repeat(),
                                        move |tab, delta| {
                                            let delta = (delta - 0.15 * n as f32) / 0.7;
                                            let delta = 1.0 - (0.5 - delta).abs() * 2.;
                                            let delta = ease_in_out(delta.clamp(0., 1.));
                                            let delta = 0.1 + 0.9 * delta;

                                            tab.border_color(border_color.opacity(delta))
                                                .text_color(text_color.opacity(delta))
                                        },
                                    ),
                            )
                        };

                        v_flex()
                            .gap_2()
                            .items_center()
                            .child(tab(0).ml_neg_20())
                            .child(tab(1))
                            .child(tab(2).ml_20())
                    }),
            )
            .child(h_flex().absolute().top_2().right_2().child(
                IconButton::new("cancel", IconName::X).on_click(cx.listener(
                    |_, _: &ClickEvent, cx| {
                        cx.emit(DismissEvent);
                    },
                )),
            ))
            .child(
                Label::new("Read and accept the terms of service to set Zed as your inline completions provider.").color(Color::Muted)
            )
            .child(
                v_flex()
                    .mt_2()
                    .gap_2()
                    .w_full()
                    .child(
                        Button::new("accept-tos", "Tab to Accept and Enable")
                            .style(ButtonStyle::Tinted(TintColor::Accent))
                            .full_width()
                            .on_click(cx.listener(Self::accept_and_enable)),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .gap_1()
                            .child(
                                div()
                                    .w_full()
                                    .child(
                                        Button::new("view-tos", "Terms of Service")
                                            .full_width()
                                            .icon(IconName::ArrowUpRight)
                                            .icon_size(IconSize::Indicator)
                                            .icon_color(Color::Muted)
                                            .on_click(cx.listener(Self::view_terms)),
                                    ),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .child(
                                        Button::new("blog-post", "Read the Blog Post")
                                            .full_width()
                                            .icon(IconName::ArrowUpRight)
                                            .icon_size(IconSize::Indicator)
                                            .icon_color(Color::Muted)
                                            .on_click(cx.listener(Self::view_blog)),
                                    ),
                            ),
                    )
            )
    }
}
