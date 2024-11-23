use anyhow::Result;
use gpui::{
    prelude::*, px, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, Pixels, Task, View, ViewContext, WeakView, WindowContext,
};
use language_model::LanguageModelRegistry;
use language_model_selector::LanguageModelSelector;
use ui::{prelude::*, ButtonLike, Divider, IconButtonShape, Tab, Tooltip};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::{Pane, Workspace};

use crate::chat_editor::ChatEditor;
use crate::{NewChat, ToggleFocus, ToggleModelSelector};

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
    pane: View<Pane>,
    chat_editor: View<ChatEditor>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            workspace.update(&mut cx, |workspace, cx| {
                cx.new_view(|cx| Self::new(workspace, cx))
            })
        })
    }

    fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let pane = cx.new_view(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.project().clone(),
                Default::default(),
                None,
                NewChat.boxed_clone(),
                cx,
            );
            pane.set_can_split(false, cx);
            pane.set_can_navigate(true, cx);

            pane
        });

        Self {
            pane,
            chat_editor: cx.new_view(ChatEditor::new),
        }
    }
}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.pane.focus_handle(cx)
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

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}

    fn pane(&self) -> Option<View<Pane>> {
        Some(self.pane.clone())
    }

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
            .child(h_flex().child(Label::new("Chat Title Goes Here")))
            .child(
                h_flex()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .child(self.render_language_model_selector(cx))
                    .child(Divider::vertical())
                    .child(
                        IconButton::new("new-chat", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .tooltip({
                                let focus_handle = focus_handle.clone();
                                move |cx| {
                                    Tooltip::for_action_in("New Chat", &NewChat, &focus_handle, cx)
                                }
                            })
                            .on_click(move |_event, _cx| {
                                println!("New Chat");
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
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("AssistantPanel2")
            .justify_between()
            .size_full()
            .on_action(cx.listener(|_this, _: &NewChat, _cx| {
                println!("Action: New Chat");
            }))
            .child(self.render_toolbar(cx))
            .child(v_flex().bg(cx.theme().colors().panel_background))
            .child(
                h_flex()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.chat_editor.clone()),
            )
    }
}
