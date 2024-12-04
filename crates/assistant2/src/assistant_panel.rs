use std::sync::Arc;

use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use client::zed_urls;
use collections::HashMap;
use gpui::{
    list, prelude::*, px, Action, AnyElement, AppContext, AsyncWindowContext, Empty, EventEmitter,
    FocusHandle, FocusableView, FontWeight, ListAlignment, ListState, Model, Pixels,
    StyleRefinement, Subscription, Task, TextStyleRefinement, View, ViewContext, WeakView,
    WindowContext,
};
use language::LanguageRegistry;
use language_model::{LanguageModelRegistry, Role};
use language_model_selector::LanguageModelSelector;
use markdown::{Markdown, MarkdownStyle};
use settings::Settings;
use theme::ThemeSettings;
use ui::{prelude::*, ButtonLike, Divider, IconButtonShape, Tab, Tooltip};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

use crate::message_editor::MessageEditor;
use crate::thread::{MessageId, Thread, ThreadError, ThreadEvent};
use crate::thread_store::ThreadStore;
use crate::{NewThread, ToggleFocus, ToggleModelSelector};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<AssistantPanel>(cx);
            });
        },
    )
    .detach();
}

pub struct AssistantPanel {
    workspace: WeakView<Workspace>,
    language_registry: Arc<LanguageRegistry>,
    #[allow(unused)]
    thread_store: Model<ThreadStore>,
    thread: Model<Thread>,
    thread_messages: Vec<MessageId>,
    rendered_messages_by_id: HashMap<MessageId, View<Markdown>>,
    thread_list_state: ListState,
    message_editor: View<MessageEditor>,
    tools: Arc<ToolWorkingSet>,
    last_error: Option<ThreadError>,
    _subscriptions: Vec<Subscription>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let tools = Arc::new(ToolWorkingSet::default());
            let thread_store = workspace
                .update(&mut cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    ThreadStore::new(project, tools.clone(), cx)
                })?
                .await?;

            workspace.update(&mut cx, |workspace, cx| {
                cx.new_view(|cx| Self::new(workspace, thread_store, tools, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        thread_store: Model<ThreadStore>,
        tools: Arc<ToolWorkingSet>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let thread = cx.new_model(|cx| Thread::new(tools.clone(), cx));
        let subscriptions = vec![
            cx.observe(&thread, |_, _, cx| cx.notify()),
            cx.subscribe(&thread, Self::handle_thread_event),
        ];

        Self {
            workspace: workspace.weak_handle(),
            language_registry: workspace.project().read(cx).languages().clone(),
            thread_store,
            thread: thread.clone(),
            thread_messages: Vec::new(),
            rendered_messages_by_id: HashMap::default(),
            thread_list_state: ListState::new(0, ListAlignment::Bottom, px(1024.), {
                let this = cx.view().downgrade();
                move |ix, cx: &mut WindowContext| {
                    this.update(cx, |this, cx| this.render_message(ix, cx))
                        .unwrap()
                }
            }),
            message_editor: cx.new_view(|cx| MessageEditor::new(thread, cx)),
            tools,
            last_error: None,
            _subscriptions: subscriptions,
        }
    }

    fn new_thread(&mut self, cx: &mut ViewContext<Self>) {
        let tools = self.thread.read(cx).tools().clone();
        let thread = cx.new_model(|cx| Thread::new(tools, cx));
        let subscriptions = vec![
            cx.observe(&thread, |_, _, cx| cx.notify()),
            cx.subscribe(&thread, Self::handle_thread_event),
        ];

        self.message_editor = cx.new_view(|cx| MessageEditor::new(thread.clone(), cx));
        self.thread = thread;
        self.thread_messages.clear();
        self.thread_list_state.reset(0);
        self.rendered_messages_by_id.clear();
        self._subscriptions = subscriptions;

        self.message_editor.focus_handle(cx).focus(cx);
    }

    fn handle_thread_event(
        &mut self,
        _: Model<Thread>,
        event: &ThreadEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ThreadEvent::ShowError(error) => {
                self.last_error = Some(error.clone());
            }
            ThreadEvent::StreamedCompletion => {}
            ThreadEvent::StreamedAssistantText(message_id, text) => {
                if let Some(markdown) = self.rendered_messages_by_id.get_mut(&message_id) {
                    markdown.update(cx, |markdown, cx| {
                        markdown.append(text, cx);
                    });
                }
            }
            ThreadEvent::MessageAdded(message_id) => {
                let old_len = self.thread_messages.len();
                self.thread_messages.push(*message_id);
                self.thread_list_state.splice(old_len..old_len, 1);

                if let Some(message_text) = self
                    .thread
                    .read(cx)
                    .message(*message_id)
                    .map(|message| message.text.clone())
                {
                    let theme_settings = ThemeSettings::get_global(cx);

                    let mut text_style = cx.text_style();
                    text_style.refine(&TextStyleRefinement {
                        font_family: Some(theme_settings.ui_font.family.clone()),
                        font_size: Some(TextSize::Default.rems(cx).into()),
                        color: Some(cx.theme().colors().text),
                        ..Default::default()
                    });

                    let markdown_style = MarkdownStyle {
                        base_text_style: text_style,
                        syntax: cx.theme().syntax().clone(),
                        selection_background_color: cx.theme().players().local().selection,
                        code_block: StyleRefinement {
                            text: Some(TextStyleRefinement {
                                font_family: Some(theme_settings.buffer_font.family.clone()),
                                font_size: Some(theme_settings.buffer_font_size.into()),
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        ..Default::default()
                    };

                    let markdown = cx.new_view(|cx| {
                        Markdown::new(
                            message_text,
                            markdown_style,
                            Some(self.language_registry.clone()),
                            None,
                            cx,
                        )
                    });
                    self.rendered_messages_by_id.insert(*message_id, markdown);
                }

                cx.notify();
            }
            ThreadEvent::UsePendingTools => {
                let pending_tool_uses = self
                    .thread
                    .read(cx)
                    .pending_tool_uses()
                    .into_iter()
                    .filter(|tool_use| tool_use.status.is_idle())
                    .cloned()
                    .collect::<Vec<_>>();

                for tool_use in pending_tool_uses {
                    if let Some(tool) = self.tools.tool(&tool_use.name, cx) {
                        let task = tool.run(tool_use.input, self.workspace.clone(), cx);

                        self.thread.update(cx, |thread, cx| {
                            thread.insert_tool_output(
                                tool_use.assistant_message_id,
                                tool_use.id.clone(),
                                task,
                                cx,
                            );
                        });
                    }
                }
            }
            ThreadEvent::ToolFinished { .. } => {}
        }
    }
}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanel2"
    }

    fn position(&self, _cx: &WindowContext) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, _position: DockPosition, _cx: &mut ViewContext<Self>) {}

    fn size(&self, _cx: &WindowContext) -> Pixels {
        px(640.)
    }

    fn set_size(&mut self, _size: Option<Pixels>, _cx: &mut ViewContext<Self>) {}

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

impl AssistantPanel {
    fn render_toolbar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        h_flex()
            .id("assistant-toolbar")
            .justify_between()
            .gap(DynamicSpacing::Base08.rems(cx))
            .h(Tab::container_height(cx))
            .px(DynamicSpacing::Base08.rems(cx))
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(h_flex().child(Label::new("Thread Title Goes Here")))
            .child(
                h_flex()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .child(self.render_language_model_selector(cx))
                    .child(Divider::vertical())
                    .child(
                        IconButton::new("new-thread", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in(
                                        "New Thread",
                                        &NewThread,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .on_click(move |_event, _cx| {
                                println!("New Thread");
                            }),
                    )
                    .child(
                        IconButton::new("open-history", IconName::HistoryRerun)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip(move |cx| Tooltip::text("Open History", cx))
                            .on_click(move |_event, _cx| {
                                println!("Open History");
                            }),
                    )
                    .child(
                        IconButton::new("configure-assistant", IconName::Settings)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip(move |cx| Tooltip::text("Configure Assistant", cx))
                            .on_click(move |_event, _cx| {
                                println!("Configure Assistant");
                            }),
                    ),
            )
    }

    fn render_language_model_selector(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_provider = LanguageModelRegistry::read_global(cx).active_provider();
        let active_model = LanguageModelRegistry::read_global(cx).active_model();

        LanguageModelSelector::new(
            |model, _cx| {
                println!("Selected {:?}", model.name());
            },
            ButtonLike::new("active-model")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .w_full()
                        .gap_0p5()
                        .child(
                            div()
                                .overflow_x_hidden()
                                .flex_grow()
                                .whitespace_nowrap()
                                .child(match (active_provider, active_model) {
                                    (Some(provider), Some(model)) => h_flex()
                                        .gap_1()
                                        .child(
                                            Icon::new(
                                                model.icon().unwrap_or_else(|| provider.icon()),
                                            )
                                            .color(Color::Muted)
                                            .size(IconSize::XSmall),
                                        )
                                        .child(
                                            Label::new(model.name().0)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .into_any_element(),
                                    _ => Label::new("No model selected")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element(),
                                }),
                        )
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .color(Color::Muted)
                                .size(IconSize::XSmall),
                        ),
                )
                .tooltip(move |cx| Tooltip::for_action("Change Model", &ToggleModelSelector, cx)),
        )
    }

    fn render_message(&self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let message_id = self.thread_messages[ix];
        let Some(message) = self.thread.read(cx).message(message_id) else {
            return Empty.into_any();
        };

        let Some(markdown) = self.rendered_messages_by_id.get(&message_id) else {
            return Empty.into_any();
        };

        let (role_icon, role_name) = match message.role {
            Role::User => (IconName::Person, "You"),
            Role::Assistant => (IconName::ZedAssistant, "Assistant"),
            Role::System => (IconName::Settings, "System"),
        };

        div()
            .id(("message-container", ix))
            .p_2()
            .child(
                v_flex()
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .rounded_md()
                    .child(
                        h_flex()
                            .justify_between()
                            .p_1p5()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(role_icon).size(IconSize::Small))
                                    .child(Label::new(role_name).size(LabelSize::Small)),
                            ),
                    )
                    .child(v_flex().p_1p5().text_ui(cx).child(markdown.clone())),
            )
            .into_any()
    }

    fn render_last_error(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        let last_error = self.last_error.as_ref()?;

        Some(
            div()
                .absolute()
                .right_3()
                .bottom_12()
                .max_w_96()
                .py_2()
                .px_3()
                .elevation_2(cx)
                .occlude()
                .child(match last_error {
                    ThreadError::PaymentRequired => self.render_payment_required_error(cx),
                    ThreadError::MaxMonthlySpendReached => {
                        self.render_max_monthly_spend_reached_error(cx)
                    }
                    ThreadError::Message(error_message) => {
                        self.render_error_message(error_message, cx)
                    }
                })
                .into_any(),
        )
    }

    fn render_payment_required_error(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        const ERROR_MESSAGE: &str = "Free tier exceeded. Subscribe and add payment to continue using Zed LLMs. You'll be billed at cost for tokens used.";

        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(Label::new("Free Usage Exceeded").weight(FontWeight::MEDIUM)),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_24()
                    .overflow_y_scroll()
                    .child(Label::new(ERROR_MESSAGE)),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(Button::new("subscribe", "Subscribe").on_click(cx.listener(
                        |this, _, cx| {
                            this.last_error = None;
                            cx.open_url(&zed_urls::account_url(cx));
                            cx.notify();
                        },
                    )))
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            this.last_error = None;
                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn render_max_monthly_spend_reached_error(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        const ERROR_MESSAGE: &str = "You have reached your maximum monthly spend. Increase your spend limit to continue using Zed LLMs.";

        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(Label::new("Max Monthly Spend Reached").weight(FontWeight::MEDIUM)),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_24()
                    .overflow_y_scroll()
                    .child(Label::new(ERROR_MESSAGE)),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(
                        Button::new("subscribe", "Update Monthly Spend Limit").on_click(
                            cx.listener(|this, _, cx| {
                                this.last_error = None;
                                cx.open_url(&zed_urls::account_url(cx));
                                cx.notify();
                            }),
                        ),
                    )
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            this.last_error = None;
                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn render_error_message(
        &self,
        error_message: &SharedString,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(
                        Label::new("Error interacting with language model")
                            .weight(FontWeight::MEDIUM),
                    ),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_32()
                    .overflow_y_scroll()
                    .child(Label::new(error_message.clone())),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            this.last_error = None;
                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("AssistantPanel2")
            .justify_between()
            .size_full()
            .on_action(cx.listener(|this, _: &NewThread, cx| {
                this.new_thread(cx);
            }))
            .child(self.render_toolbar(cx))
            .child(list(self.thread_list_state.clone()).flex_1())
            .child(
                h_flex()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.message_editor.clone()),
            )
            .children(self.render_last_error(cx))
    }
}
