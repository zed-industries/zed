use assistant::{assistant_panel::InlineAssist, AssistantPanel};
use editor::Editor;
use gpui::{
    elements::{Empty, Flex, MouseEventHandler, ParentElement, Svg},
    platform::{CursorStyle, MouseButton},
    Action, AnyElement, Element, Entity, EventContext, Subscription, View, ViewContext, ViewHandle,
    WeakViewHandle,
};

use search::{buffer_search, BufferSearchBar};
use workspace::{item::ItemHandle, ToolbarItemLocation, ToolbarItemView, Workspace};

pub struct QuickActionBar {
    buffer_search_bar: ViewHandle<BufferSearchBar>,
    active_item: Option<Box<dyn ItemHandle>>,
    _inlay_hints_enabled_subscription: Option<Subscription>,
    workspace: WeakViewHandle<Workspace>,
}

impl QuickActionBar {
    pub fn new(buffer_search_bar: ViewHandle<BufferSearchBar>, workspace: &Workspace) -> Self {
        Self {
            buffer_search_bar,
            active_item: None,
            _inlay_hints_enabled_subscription: None,
            workspace: workspace.weak_handle(),
        }
    }

    fn active_editor(&self) -> Option<ViewHandle<Editor>> {
        self.active_item
            .as_ref()
            .and_then(|item| item.downcast::<Editor>())
    }
}

impl Entity for QuickActionBar {
    type Event = ();
}

impl View for QuickActionBar {
    fn ui_name() -> &'static str {
        "QuickActionsBar"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
        let Some(editor) = self.active_editor() else {
            return Empty::new().into_any();
        };

        let mut bar = Flex::row();
        if editor.read(cx).supports_inlay_hints(cx) {
            bar = bar.with_child(render_quick_action_bar_button(
                0,
                "icons/inlay_hint.svg",
                editor.read(cx).inlay_hints_enabled(),
                (
                    "Toggle Inlay Hints".to_string(),
                    Some(Box::new(editor::ToggleInlayHints)),
                ),
                cx,
                |this, cx| {
                    if let Some(editor) = this.active_editor() {
                        editor.update(cx, |editor, cx| {
                            editor.toggle_inlay_hints(&editor::ToggleInlayHints, cx);
                        });
                    }
                },
            ));
        }

        if editor.read(cx).buffer().read(cx).is_singleton() {
            let search_bar_shown = !self.buffer_search_bar.read(cx).is_dismissed();
            let search_action = buffer_search::Deploy { focus: true };

            bar = bar.with_child(render_quick_action_bar_button(
                1,
                "icons/magnifying_glass.svg",
                search_bar_shown,
                (
                    "Buffer Search".to_string(),
                    Some(Box::new(search_action.clone())),
                ),
                cx,
                move |this, cx| {
                    this.buffer_search_bar.update(cx, |buffer_search_bar, cx| {
                        if search_bar_shown {
                            buffer_search_bar.dismiss(&buffer_search::Dismiss, cx);
                        } else {
                            buffer_search_bar.deploy(&search_action, cx);
                        }
                    });
                },
            ));
        }

        bar.add_child(render_quick_action_bar_button(
            2,
            "icons/magic-wand.svg",
            false,
            ("Inline Assist".into(), Some(Box::new(InlineAssist))),
            cx,
            move |this, cx| {
                if let Some(workspace) = this.workspace.upgrade(cx) {
                    workspace.update(cx, |workspace, cx| {
                        AssistantPanel::inline_assist(workspace, &Default::default(), cx);
                    });
                }
            },
        ));

        bar.into_any()
    }
}

fn render_quick_action_bar_button<
    F: 'static + Fn(&mut QuickActionBar, &mut EventContext<QuickActionBar>),
>(
    index: usize,
    icon: &'static str,
    toggled: bool,
    tooltip: (String, Option<Box<dyn Action>>),
    cx: &mut ViewContext<QuickActionBar>,
    on_click: F,
) -> AnyElement<QuickActionBar> {
    enum QuickActionBarButton {}

    let theme = theme::current(cx);
    let (tooltip_text, action) = tooltip;

    MouseEventHandler::new::<QuickActionBarButton, _>(index, cx, |mouse_state, _| {
        let style = theme
            .workspace
            .toolbar
            .toggleable_tool
            .in_state(toggled)
            .style_for(mouse_state);
        Svg::new(icon)
            .with_color(style.color)
            .constrained()
            .with_width(style.icon_width)
            .aligned()
            .constrained()
            .with_width(style.button_width)
            .with_height(style.button_width)
            .contained()
            .with_style(style.container)
    })
    .with_cursor_style(CursorStyle::PointingHand)
    .on_click(MouseButton::Left, move |_, pane, cx| on_click(pane, cx))
    .with_tooltip::<QuickActionBarButton>(index, tooltip_text, action, theme.tooltip.clone(), cx)
    .into_any_named("quick action bar button")
}

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
                    ToolbarItemLocation::PrimaryRight { flex: None }
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
