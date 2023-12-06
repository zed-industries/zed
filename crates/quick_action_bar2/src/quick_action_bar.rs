use assistant::{AssistantPanel, InlineAssist};
use editor::Editor;

use gpui::{
    Action, ClickEvent, Div, ElementId, EventEmitter, InteractiveElement, ParentElement, Render,
    Stateful, Styled, Subscription, View, ViewContext, WeakView,
};
use search::BufferSearchBar;
use ui::{prelude::*, ButtonSize, ButtonStyle, Icon, IconButton, IconSize, Tooltip};
use workspace::{
    item::ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};

pub struct QuickActionBar {
    buffer_search_bar: View<BufferSearchBar>,
    active_item: Option<Box<dyn ItemHandle>>,
    _inlay_hints_enabled_subscription: Option<Subscription>,
    workspace: WeakView<Workspace>,
}

impl QuickActionBar {
    pub fn new(buffer_search_bar: View<BufferSearchBar>, workspace: &Workspace) -> Self {
        Self {
            buffer_search_bar,
            active_item: None,
            _inlay_hints_enabled_subscription: None,
            workspace: workspace.weak_handle(),
        }
    }

    #[allow(dead_code)]
    fn active_editor(&self) -> Option<View<Editor>> {
        self.active_item
            .as_ref()
            .and_then(|item| item.downcast::<Editor>())
    }
}

impl Render for QuickActionBar {
    type Element = Stateful<Div>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let buffer_search_bar = self.buffer_search_bar.clone();
        let search_button = QuickActionBarButton::new(
            "toggle buffer search",
            Icon::MagnifyingGlass,
            !self.buffer_search_bar.read(cx).is_dismissed(),
            Box::new(search::buffer_search::Deploy { focus: false }),
            "Buffer Search",
            move |_, cx| {
                buffer_search_bar.update(cx, |search_bar, cx| search_bar.toggle(cx));
            },
        );
        let assistant_button = QuickActionBarButton::new(
            "toggle inline assistant",
            Icon::MagicWand,
            false,
            Box::new(InlineAssist),
            "Inline assistant",
            {
                let workspace = self.workspace.clone();
                move |_, cx| {
                    if let Some(workspace) = workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            AssistantPanel::inline_assist(workspace, &InlineAssist, cx);
                        });
                    }
                }
            },
        );
        h_stack()
            .id("quick action bar")
            .p_1()
            .gap_2()
            .child(search_button)
            .child(assistant_button)
    }
}

impl EventEmitter<ToolbarItemEvent> for QuickActionBar {}

// impl View for QuickActionBar {
//     fn ui_name() -> &'static str {
//         "QuickActionsBar"
//     }

//     fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
//         let Some(editor) = self.active_editor() else {
//             return div();
//         };

//         let mut bar = Flex::row();
//         if editor.read(cx).supports_inlay_hints(cx) {
//             bar = bar.with_child(render_quick_action_bar_button(
//                 0,
//                 "icons/inlay_hint.svg",
//                 editor.read(cx).inlay_hints_enabled(),
//                 (
//                     "Toggle Inlay Hints".to_string(),
//                     Some(Box::new(editor::ToggleInlayHints)),
//                 ),
//                 cx,
//                 |this, cx| {
//                     if let Some(editor) = this.active_editor() {
//                         editor.update(cx, |editor, cx| {
//                             editor.toggle_inlay_hints(&editor::ToggleInlayHints, cx);
//                         });
//                     }
//                 },
//             ));
//         }

//         if editor.read(cx).buffer().read(cx).is_singleton() {
//             let search_bar_shown = !self.buffer_search_bar.read(cx).is_dismissed();
//             let search_action = buffer_search::Deploy { focus: true };

//             bar = bar.with_child(render_quick_action_bar_button(
//                 1,
//                 "icons/magnifying_glass.svg",
//                 search_bar_shown,
//                 (
//                     "Buffer Search".to_string(),
//                     Some(Box::new(search_action.clone())),
//                 ),
//                 cx,
//                 move |this, cx| {
//                     this.buffer_search_bar.update(cx, |buffer_search_bar, cx| {
//                         if search_bar_shown {
//                             buffer_search_bar.dismiss(&buffer_search::Dismiss, cx);
//                         } else {
//                             buffer_search_bar.deploy(&search_action, cx);
//                         }
//                     });
//                 },
//             ));
//         }

//         bar.add_child(render_quick_action_bar_button(
//             2,
//             "icons/magic-wand.svg",
//             false,
//             ("Inline Assist".into(), Some(Box::new(InlineAssist))),
//             cx,
//             move |this, cx| {
//                 if let Some(workspace) = this.workspace.upgrade(cx) {
//                     workspace.update(cx, |workspace, cx| {
//                         AssistantPanel::inline_assist(workspace, &Default::default(), cx);
//                     });
//                 }
//             },
//         ));

//         bar.into_any()
//     }
// }

#[derive(IntoElement)]
struct QuickActionBarButton {
    id: ElementId,
    icon: Icon,
    toggled: bool,
    action: Box<dyn Action>,
    tooltip: SharedString,
    tooltip_meta: Option<SharedString>,
    on_click: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
}

impl QuickActionBarButton {
    fn new(
        id: impl Into<ElementId>,
        icon: Icon,
        toggled: bool,
        action: Box<dyn Action>,
        tooltip: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            toggled,
            action,
            tooltip: tooltip.into(),
            tooltip_meta: None,
            on_click: Box::new(on_click),
        }
    }

    #[allow(dead_code)]
    pub fn meta(mut self, meta: Option<impl Into<SharedString>>) -> Self {
        self.tooltip_meta = meta.map(|meta| meta.into());
        self
    }
}

impl RenderOnce for QuickActionBarButton {
    type Rendered = IconButton;

    fn render(self, _: &mut WindowContext) -> Self::Rendered {
        let tooltip = self.tooltip.clone();
        let action = self.action.boxed_clone();
        let tooltip_meta = self.tooltip_meta.clone();

        IconButton::new(self.id.clone(), self.icon)
            .size(ButtonSize::Compact)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .selected(self.toggled)
            .tooltip(move |cx| {
                if let Some(meta) = &tooltip_meta {
                    Tooltip::with_meta(tooltip.clone(), Some(&*action), meta.clone(), cx)
                } else {
                    Tooltip::for_action(tooltip.clone(), &*action, cx)
                }
            })
            .on_click(move |event, cx| (self.on_click)(event, cx))
    }
}

// fn render_quick_action_bar_button<
//     F: 'static + Fn(&mut QuickActionBar, &mut ViewContext<QuickActionBar>),
// >(
//     index: usize,
//     icon: &'static str,
//     toggled: bool,
//     tooltip: (String, Option<Box<dyn Action>>),
//     cx: &mut ViewContext<QuickActionBar>,
//     on_click: F,
// ) -> AnyElement<QuickActionBar> {
//     enum QuickActionBarButton {}

//     let theme = theme::current(cx);
//     let (tooltip_text, action) = tooltip;

//     MouseEventHandler::new::<QuickActionBarButton, _>(index, cx, |mouse_state, _| {
//         let style = theme
//             .workspace
//             .toolbar
//             .toggleable_tool
//             .in_state(toggled)
//             .style_for(mouse_state);
//         Svg::new(icon)
//             .with_color(style.color)
//             .constrained()
//             .with_width(style.icon_width)
//             .aligned()
//             .constrained()
//             .with_width(style.button_width)
//             .with_height(style.button_width)
//             .contained()
//             .with_style(style.container)
//     })
//     .with_cursor_style(CursorStyle::PointingHand)
//     .on_click(MouseButton::Left, move |_, pane, cx| on_click(pane, cx))
//     .with_tooltip::<QuickActionBarButton>(index, tooltip_text, action, theme.tooltip.clone(), cx)
//     .into_any_named("quick action bar button")
// }

impl ToolbarItemView for QuickActionBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        match active_pane_item {
            Some(active_item) => {
                self.active_item = Some(active_item.boxed_clone());
                self._inlay_hints_enabled_subscription.take();

                if let Some(editor) = active_item.downcast::<Editor>() {
                    let mut inlay_hints_enabled = editor.read(cx).inlay_hints_enabled();
                    let mut supports_inlay_hints = editor.read(cx).supports_inlay_hints(cx);
                    self._inlay_hints_enabled_subscription =
                        Some(cx.observe(&editor, move |_, editor, cx| {
                            let editor = editor.read(cx);
                            let new_inlay_hints_enabled = editor.inlay_hints_enabled();
                            let new_supports_inlay_hints = editor.supports_inlay_hints(cx);
                            let should_notify = inlay_hints_enabled != new_inlay_hints_enabled
                                || supports_inlay_hints != new_supports_inlay_hints;
                            inlay_hints_enabled = new_inlay_hints_enabled;
                            supports_inlay_hints = new_supports_inlay_hints;
                            if should_notify {
                                cx.notify()
                            }
                        }));
                    ToolbarItemLocation::PrimaryRight
                } else {
                    ToolbarItemLocation::Hidden
                }
            }
            None => {
                self.active_item = None;
                ToolbarItemLocation::Hidden
            }
        }
    }
}
