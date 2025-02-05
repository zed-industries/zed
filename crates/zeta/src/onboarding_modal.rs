use std::{sync::Arc, time::Duration};

use crate::ZED_PREDICT_DATA_COLLECTION_CHOICE;
use client::{Client, UserStore};
use db::kvp::KEY_VALUE_STORE;
use feature_flags::FeatureFlagAppExt as _;
use fs::Fs;
use gpui::{
    ease_in_out, svg, Animation, AnimationExt as _, ClickEvent, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, MouseDownEvent, Render,
};
use language::language_settings::{AllLanguageSettings, InlineCompletionProvider};
use settings::{update_settings_file, Settings};
use ui::{prelude::*, Checkbox, TintColor};
use util::ResultExt;
use workspace::{notifications::NotifyTaskExt, ModalView, Workspace};

/// Introduces user to Zed's Edit Prediction feature and terms of service
pub struct ZedPredictModal {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    sign_in_status: SignInStatus,
    terms_of_service: bool,
    data_collection_expanded: bool,
    data_collection_opted_in: bool,
}

#[derive(PartialEq, Eq)]
enum SignInStatus {
    /// Signed out or signed in but not from this modal
    Idle,
    /// Authentication triggered from this modal
    Waiting,
    /// Signed in after authentication from this modal
    SignedIn,
}

impl ZedPredictModal {
    pub fn toggle(
        workspace: &mut Workspace,
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |_window, cx| Self {
            user_store,
            client,
            fs,
            focus_handle: cx.focus_handle(),
            sign_in_status: SignInStatus::Idle,
            terms_of_service: false,
            data_collection_expanded: false,
            data_collection_opted_in: false,
        });
    }

    fn view_terms(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("https://zed.dev/terms-of-service");
        cx.notify();
    }

    fn view_blog(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("https://zed.dev/blog/"); // TODO Add the link when live
        cx.notify();
    }

    fn inline_completions_doc(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("https://zed.dev/docs/configuring-zed#inline-completions");
        cx.notify();
    }

    fn accept_and_enable(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let task = self
            .user_store
            .update(cx, |this, cx| this.accept_terms_of_service(cx));

        cx.spawn(|this, mut cx| async move {
            task.await?;

            let mut data_collection_opted_in = false;
            this.update(&mut cx, |this, _cx| {
                data_collection_opted_in = this.data_collection_opted_in;
            })
            .ok();

            KEY_VALUE_STORE
                .write_kvp(
                    ZED_PREDICT_DATA_COLLECTION_CHOICE.into(),
                    data_collection_opted_in.to_string(),
                )
                .await
                .log_err();

            this.update(&mut cx, |this, cx| {
                update_settings_file::<AllLanguageSettings>(this.fs.clone(), cx, move |file, _| {
                    file.features
                        .get_or_insert(Default::default())
                        .inline_completion_provider = Some(InlineCompletionProvider::Zed);
                });

                cx.emit(DismissEvent);
            })
        })
        .detach_and_notify_err(window, cx);
    }

    fn sign_in(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let client = self.client.clone();
        self.sign_in_status = SignInStatus::Waiting;

        cx.spawn(move |this, mut cx| async move {
            let result = client.authenticate_and_connect(true, &cx).await;

            let status = match result {
                Ok(_) => SignInStatus::SignedIn,
                Err(_) => SignInStatus::Idle,
            };

            this.update(&mut cx, |this, cx| {
                this.sign_in_status = status;
                cx.notify()
            })?;

            result
        })
        .detach_and_notify_err(window, cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ZedPredictModal {}

impl Focusable for ZedPredictModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ZedPredictModal {}

impl Render for ZedPredictModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let base = v_flex()
            .id("zed predict tos")
            .key_context("ZedPredictModal")
            .w(px(440.))
            .p_4()
            .relative()
            .gap_2()
            .overflow_hidden()
            .elevation_3(cx)
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(|_, _: &menu::Cancel, _window, cx| {
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, _cx| {
                this.focus_handle.focus(window);
            }))
            .child(
                div()
                    .p_1p5()
                    .absolute()
                    .top_1()
                    .left_1p5()
                    .right_0()
                    .h(px(200.))
                    .child(
                        svg()
                            .path("icons/zed_predict_bg.svg")
                            .text_color(cx.theme().colors().icon_disabled)
                            .w(px(418.))
                            .h(px(128.))
                            .overflow_hidden(),
                    ),
            )
            .child(
                h_flex()
                    .w_full()
                    .mb_2()
                    .justify_between()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Label::new("Introducing Zed AI's")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Headline::new("Edit Prediction").size(HeadlineSize::Large)),
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
                            .pr_4()
                            .child(tab(0).ml_neg_20())
                            .child(tab(1))
                            .child(tab(2).ml_20())
                    }),
            )
            .child(h_flex().absolute().top_2().right_2().child(
                IconButton::new("cancel", IconName::X).on_click(cx.listener(
                    |_, _: &ClickEvent, _window, cx| {
                        cx.emit(DismissEvent);
                    },
                )),
            ));

        let blog_post_button = if cx.is_staff() {
            Some(
                Button::new("view-blog", "Read the Blog Post")
                    .full_width()
                    .icon(IconName::ArrowUpRight)
                    .icon_size(IconSize::Indicator)
                    .icon_color(Color::Muted)
                    .on_click(cx.listener(Self::view_blog)),
            )
        } else {
            // TODO: put back when blog post is published
            None
        };

        if self.user_store.read(cx).current_user().is_some() {
            let copy = match self.sign_in_status {
                SignInStatus::Idle => "Get accurate and instant edit predictions at every keystroke. Before setting Zed as your edit prediction provider:",
                SignInStatus::SignedIn => "Almost there! Ensure you:",
                SignInStatus::Waiting => unreachable!(),
            };

            let accordion_icons = if self.data_collection_expanded {
                (IconName::ChevronUp, IconName::ChevronDown)
            } else {
                (IconName::ChevronDown, IconName::ChevronUp)
            };

            fn label_item(label_text: impl Into<SharedString>) -> impl Element {
                Label::new(label_text).color(Color::Muted).into_element()
            }

            fn info_item(label_text: impl Into<SharedString>) -> impl Element {
                h_flex()
                    .gap_2()
                    .child(Icon::new(IconName::Check).size(IconSize::XSmall))
                    .child(label_item(label_text))
            }

            fn multiline_info_item<E1: Into<SharedString>, E2: IntoElement>(
                first_line: E1,
                second_line: E2,
            ) -> impl Element {
                v_flex()
                    .child(info_item(first_line))
                    .child(div().pl_5().child(second_line))
            }

            base.child(Label::new(copy).color(Color::Muted))
                .child(
                    h_flex()
                        .child(
                            Checkbox::new("tos-checkbox", self.terms_of_service.into())
                                .fill()
                                .label("Read and accept the")
                                .on_click(cx.listener(move |this, state, _window, cx| {
                                    this.terms_of_service = *state == ToggleState::Selected;
                                    cx.notify()
                                })),
                        )
                        .child(
                            Button::new("view-tos", "Terms of Service")
                                .icon(IconName::ArrowUpRight)
                                .icon_size(IconSize::Indicator)
                                .icon_color(Color::Muted)
                                .on_click(cx.listener(Self::view_terms)),
                        ),
                )
                .child(
                    v_flex()
                        .child(
                            h_flex()
                                .child(
                                    Checkbox::new(
                                        "training-data-checkbox",
                                        self.data_collection_opted_in.into(),
                                    )
                                    .label("Optionally share training data (OSS-only).")
                                    .fill()
                                    .on_click(cx.listener(
                                        move |this, state, _window, cx| {
                                            this.data_collection_opted_in =
                                                *state == ToggleState::Selected;
                                            cx.notify()
                                        },
                                    )),
                                )
                                .child(
                                    Button::new("learn-more", "Learn More")
                                        .icon(accordion_icons.0)
                                        .icon_size(IconSize::Indicator)
                                        .icon_color(Color::Muted)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.data_collection_expanded =
                                                !this.data_collection_expanded;
                                            cx.notify()
                                        })),
                                ),
                        )
                        .when(self.data_collection_expanded, |element| {
                            element.child(
                                v_flex()
                                    .mt_2()
                                    .p_2()
                                    .rounded_md()
                                    .bg(cx.theme().colors().editor_background.opacity(0.5))
                                    .border_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .child(
                                        div().child(
                                            Label::new("To improve edit predictions, help fine-tune Zed's model by sharing data from the open-source projects you work on.")
                                                .mb_1()
                                        )
                                    )
                                    .child(info_item(
                                        "We ask this exclusively for open-source projects.",
                                    ))
                                    .child(info_item(
                                        "Zed automatically detects if your project is open-source.",
                                    ))
                                    .child(info_item(
                                        "This setting is valid for all OSS projects you open in Zed.",
                                    ))
                                    .child(info_item("Toggle it anytime via the status bar menu."))
                                    .child(multiline_info_item(
                                        "Files that can contain sensitive data, like `.env`, are",
                                        h_flex()
                                            .child(label_item("excluded by default via the"))
                                            .child(
                                                Button::new("doc-link", "disabled_globs").on_click(
                                                    cx.listener(Self::inline_completions_doc),
                                                ),
                                            )
                                            .child(label_item("setting.")),
                                    )),
                            )
                        }),
                )
                .child(
                    v_flex()
                        .mt_2()
                        .gap_2()
                        .w_full()
                        .child(
                            Button::new("accept-tos", "Enable Edit Predictions")
                                .disabled(!self.terms_of_service)
                                .style(ButtonStyle::Tinted(TintColor::Accent))
                                .full_width()
                                .on_click(cx.listener(Self::accept_and_enable)),
                        )
                        .children(blog_post_button),
                )
        } else {
            base.child(
                Label::new("To set Zed as your edit prediction provider, please sign in.")
                    .color(Color::Muted),
            )
            .child(
                v_flex()
                    .mt_2()
                    .gap_2()
                    .w_full()
                    .child(
                        Button::new("accept-tos", "Sign in with GitHub")
                            .disabled(self.sign_in_status == SignInStatus::Waiting)
                            .style(ButtonStyle::Tinted(TintColor::Accent))
                            .full_width()
                            .on_click(cx.listener(Self::sign_in)),
                    )
                    .children(blog_post_button),
            )
        }
    }
}
