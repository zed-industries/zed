use std::{sync::Arc, time::Duration};

use crate::Zeta;
use client::{Client, UserStore};
use feature_flags::FeatureFlagAppExt as _;
use fs::Fs;
use gpui::{
    ease_in_out, svg, Animation, AnimationExt as _, ClickEvent, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, MouseDownEvent, Render,
};
use language::language_settings::{AllLanguageSettings, InlineCompletionProvider};
use settings::{update_settings_file, Settings};
use ui::{prelude::*, Checkbox, TintColor, Tooltip};
use workspace::{notifications::NotifyTaskExt, ModalView, Workspace};
use worktree::Worktree;

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
    worktrees: Vec<Entity<Worktree>>,
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
        let worktrees = workspace.visible_worktrees(cx).collect();

        workspace.toggle_modal(window, cx, |_window, cx| Self {
            user_store,
            client,
            fs,
            focus_handle: cx.focus_handle(),
            sign_in_status: SignInStatus::Idle,
            terms_of_service: false,
            data_collection_expanded: false,
            data_collection_opted_in: false,
            worktrees,
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

            this.update(&mut cx, |this, cx| {
                update_settings_file::<AllLanguageSettings>(this.fs.clone(), cx, move |file, _| {
                    file.features
                        .get_or_insert(Default::default())
                        .inline_completion_provider = Some(InlineCompletionProvider::Zed);
                });

                if this.worktrees.is_empty() {
                    cx.emit(DismissEvent);
                    return ();
                }

                Zeta::register(this.client.clone(), this.user_store.clone(), cx).update(
                    cx,
                    |zeta, cx| {
                        for worktree in this.worktrees.iter() {
                            zeta.update_data_collection_choice(
                                worktree.read(cx).abs_path().as_ref(),
                                |_| this.data_collection_opted_in.into(),
                                cx,
                            );
                        }
                    },
                );

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
            .w(px(440.))
            .p_4()
            .relative()
            .gap_2()
            .overflow_hidden()
            .elevation_3(cx)
            .id("zed predict tos")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::cancel))
            .key_context("ZedPredictModal")
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
                SignInStatus::Idle => "Get accurate and helpful edit predictions at every keystroke. Before setting Zed as your inline completions provider:",
                SignInStatus::SignedIn => "Almost there! Ensure you:",
                SignInStatus::Waiting => unreachable!(),
            };

            fn info_item(label_text: impl Into<SharedString>) -> impl Element {
                h_flex()
                    .gap_2()
                    .child(Icon::new(IconName::Check).size(IconSize::XSmall))
                    .child(Label::new(label_text).color(Color::Muted))
            }

            fn multiline_info_item<E1: Into<SharedString>, E2: IntoElement>(
                label_element: E1,
                label_element_second: E2,
            ) -> impl Element {
                v_flex()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Icon::new(IconName::Check).size(IconSize::XSmall))
                            .child(Label::new(label_element).color(Color::Muted)),
                    )
                    .child(div().pl_5().child(label_element_second))
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
                                    .fill()
                                    .when(self.worktrees.is_empty(), |element| {
                                        element.disabled(true).tooltip(move |window, cx| {
                                            Tooltip::with_meta(
                                                "No Project Open",
                                                None,
                                                "Open a project to enable this option.",
                                                window,
                                                cx,
                                            )
                                        })
                                    })
                                    .label("Optionally share training data.")
                                    .on_click(cx.listener(
                                        move |this, state, _window, cx| {
                                            this.data_collection_opted_in =
                                                *state == ToggleState::Selected;
                                            cx.notify()
                                        },
                                    )),
                                )
                                // TODO: show each worktree if more than 1
                                .child(
                                    Button::new("learn-more", "Learn More")
                                        .icon(if self.data_collection_expanded {
                                            IconName::ChevronUp
                                        } else {
                                            IconName::ChevronDown
                                        })
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
                                    .child(info_item(
                                        "Help fine-tune Zed's model to enable better predictions.",
                                    ))
                                    .child(info_item(
                                        "No data is ever captured while this setting is off.",
                                    ))
                                    .child(info_item("This is a per-project setting."))
                                    .child(info_item("Toggle it anytime via the status bar menu."))
                                    .child(multiline_info_item(
                                        "Files that can contain sensitive data, like `env` are",
                                        h_flex()
                                            .child(
                                                Label::new("excluded via the").color(Color::Muted),
                                            )
                                            .child(
                                                Button::new("doc-link", "disabled_globs").on_click(
                                                    cx.listener(Self::inline_completions_doc),
                                                ),
                                            )
                                            .child(Label::new("setting.").color(Color::Muted)),
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
                Label::new("To set Zed as your inline completions provider, please sign in.")
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
