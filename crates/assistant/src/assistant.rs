use editor::Editor;
use gpui::{
    actions, elements::*, CursorStyle, Entity, MouseButton, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;
use theme;
use workspace::{
    item::{Item, ItemHandle},
    StatusItemView, Workspace,
};

actions!(assisltant, [DeployAssistant]);

pub struct Assistant {
    composer: ViewHandle<Editor>,
    message_list: ListState,
    messages: Vec<Message>,
}

pub struct Message {
    text: String,
}

pub struct AssistantButton {
    workspace: WeakViewHandle<Workspace>,
    active: bool,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(AssistantButton::deploy_assistant);
}

impl Assistant {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let composer = cx.add_view(|cx| Editor::single_line(None, cx));

        let messages = vec![
            Message {
                text: "Hello World".into(),
            },
            Message {
                text: "I Am Skynet".into(),
            },
            Message {
                text: "Prepare To Die".into(),
            },
        ];

        Self {
            composer,
            message_list: ListState::new(
                messages.len(),
                Orientation::Bottom,
                512.,
                cx,
                |this, ix, cx| {
                    let style = &cx.global::<Settings>().theme.assistant;
                    let text = this.messages[ix].text.clone();
                    Text::new(text, style.assistant_message.text.clone())
                        .contained()
                        .with_style(style.assistant_message.container)
                        .boxed()
                },
            ),
            messages,
        }
    }
}

impl Entity for Assistant {
    type Event = ();
}

impl View for Assistant {
    fn ui_name() -> &'static str {
        "Assistant"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        Flex::column()
            .with_child(List::new(self.message_list.clone()).boxed())
            .with_child(ChildView::new(&self.composer, cx).boxed())
            .boxed()
    }
}

impl Item for Assistant {
    fn tab_content(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &gpui::AppContext,
    ) -> ElementBox {
        Label::new("Assistant", style.label.clone()).boxed()
    }
}

impl AssistantButton {
    pub fn new(workspace: ViewHandle<Workspace>) -> Self {
        Self {
            workspace: workspace.downgrade(),
            active: false,
        }
    }

    fn deploy_assistant(&mut self, _: &DeployAssistant, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade(cx) {
            workspace.update(cx, |workspace, cx| {
                let assistant = workspace.items_of_type::<Assistant>(cx).next();
                if let Some(assistant) = assistant {
                    workspace.activate_item(&assistant, cx);
                } else {
                    workspace.show_dock(true, cx);
                    let assistant = cx.add_view(|cx| Assistant::new(cx));
                    workspace.add_item_to_dock(Box::new(assistant.clone()), cx);
                }
            })
        }
    }
}

impl Entity for AssistantButton {
    type Event = ();
}

impl View for AssistantButton {
    fn ui_name() -> &'static str {
        "AssistantButton"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let active = self.active;
        let theme = cx.global::<Settings>().theme.clone();
        Stack::new()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, |state, _| {
                    let style = &theme
                        .workspace
                        .status_bar
                        .sidebar_buttons
                        .item
                        .style_for(state, active);

                    Svg::new("icons/assistant_12.svg")
                        .with_color(style.icon_color)
                        .constrained()
                        .with_width(style.icon_size)
                        .aligned()
                        .constrained()
                        .with_width(style.icon_size)
                        .with_height(style.icon_size)
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(DeployAssistant)
                })
                .with_tooltip::<Self, _>(
                    0,
                    "Assistant".into(),
                    Some(Box::new(DeployAssistant)),
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed(),
            )
            .boxed()
    }
}

impl StatusItemView for AssistantButton {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _: &mut gpui::ViewContext<Self>,
    ) {
    }
}
