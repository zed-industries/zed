use editor::Editor;
use gpui::{
    elements::{Empty, Flex, MouseEventHandler, ParentElement, Svg},
    platform::{CursorStyle, MouseButton},
    Action, AnyElement, Element, Entity, EventContext, View, ViewContext, ViewHandle,
};

use search::{buffer_search, BufferSearchBar};
use workspace::{item::ItemHandle, ToolbarItemLocation, ToolbarItemView, Workspace};

pub struct QuickActionBar {
    workspace: ViewHandle<Workspace>,
    active_item: Option<Box<dyn ItemHandle>>,
}

impl QuickActionBar {
    pub fn new(workspace: ViewHandle<Workspace>) -> Self {
        Self {
            workspace,
            active_item: None,
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
        let Some(editor) = self.active_editor() else { return Empty::new().into_any(); };

        let inlays_enabled = editor.read(cx).inlays_enabled();
        let mut bar = Flex::row().with_child(render_quick_action_bar_button(
            0,
            "icons/hamburger_15.svg",
            inlays_enabled,
            (
                "Toggle inlays".to_string(),
                Some(Box::new(editor::ToggleInlays)),
            ),
            cx,
            |this, cx| {
                if let Some(editor) = this.active_editor() {
                    editor.update(cx, |editor, cx| {
                        editor.toggle_inlays(&editor::ToggleInlays, cx);
                    });
                }
            },
        ));

        if editor.read(cx).buffer().read(cx).is_singleton() {
            let search_action = buffer_search::Deploy { focus: true };

            // TODO kb: this opens the search bar in a differently focused pane (should be the same) + should be toggleable
            let pane = self.workspace.read(cx).active_pane().clone();
            bar = bar.with_child(render_quick_action_bar_button(
                1,
                "icons/magnifying_glass_12.svg",
                false,
                (
                    "Search in buffer".to_string(),
                    Some(Box::new(search_action.clone())),
                ),
                cx,
                move |_, cx| {
                    pane.update(cx, |pane, cx| {
                        BufferSearchBar::deploy(pane, &search_action, cx);
                    });
                },
            ));
        }

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

    MouseEventHandler::<QuickActionBarButton, _>::new(index, cx, |mouse_state, _| {
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
        _: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        match active_pane_item {
            Some(active_item) => {
                dbg!("@@@@@@@@@@ TODO kb", active_item.id());
                self.active_item = Some(active_item.boxed_clone());
                ToolbarItemLocation::PrimaryRight { flex: None }
            }
            None => {
                self.active_item = None;
                ToolbarItemLocation::Hidden
            }
        }
    }
}
