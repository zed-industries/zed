use std::sync::Arc;

use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use gpui::{
    prelude::*, px, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, Model, Pixels, Subscription, Task, View, ViewContext, WeakView, WindowContext,
};
use language_model::{LanguageModelRegistry, Role};
use language_model_selector::LanguageModelSelector;
use ui::{prelude::*, ButtonLike, Divider, IconButtonShape, Tab, Tooltip};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

use crate::message_editor::MessageEditor;
use crate::thread::{Message, Thread, ThreadEvent};
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
    #[allow(unused)]
    thread_store: Model<ThreadStore>,
    thread: Model<Thread>,
    message_editor: View<MessageEditor>,
    tools: Arc<ToolWorkingSet>,
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
            thread_store,
            thread: thread.clone(),
            message_editor: cx.new_view(|cx| MessageEditor::new(thread, cx)),
            tools,
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
            ThreadEvent::StreamedCompletion => {}
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

    fn render_message(&self, message: Message, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let (role_icon, role_name) = match message.role {
            Role::User => (IconName::Person, "You"),
            Role::Assistant => (IconName::ZedAssistant, "Assistant"),
            Role::System => (IconName::Settings, "System"),
        };

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
            .child(v_flex().p_1p5().child(Label::new(message.text.clone())))
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let messages = self.thread.read(cx).messages().cloned().collect::<Vec<_>>();

        v_flex()
            .key_context("AssistantPanel2")
            .justify_between()
            .size_full()
            .on_action(cx.listener(|this, _: &NewThread, cx| {
                this.new_thread(cx);
            }))
            .child(self.render_toolbar(cx))
            .child(
                v_flex()
                    .id("message-list")
                    .gap_2()
                    .size_full()
                    .p_2()
                    .overflow_y_scroll()
                    .bg(cx.theme().colors().panel_background)
                    .children(
                        messages
                            .into_iter()
                            .map(|message| self.render_message(message, cx)),
                    ),
            )
            .child(
                h_flex()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.message_editor.clone()),
            )
    }
}
