use editor::Editor;
use gpui::{
    elements::{Empty, Flex, MouseEventHandler, ParentElement, Svg},
    platform::{CursorStyle, MouseButton},
    Action, AnyElement, Element, Entity, EventContext, View, ViewContext, ViewHandle,
};

use search::{buffer_search, BufferSearchBar};
use workspace::{item::ItemHandle, Pane, ToolbarItemLocation, ToolbarItemView};

pub struct QuickActionBar {
    pane: ViewHandle<Pane>,
    active_item: Option<Box<dyn ItemHandle>>,
}

impl QuickActionBar {
    pub fn new(pane: ViewHandle<Pane>) -> Self {
        Self {
            pane,
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
            ("Inlays".to_string(), Some(Box::new(editor::ToggleInlays))),
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
            let buffer_search_bar = self
                .pane
                .read(cx)
                .toolbar()
                .read(cx)
                .item_of_type::<BufferSearchBar>();
            let search_bar_shown = buffer_search_bar
                .as_ref()
                .map(|bar| !bar.read(cx).is_dismissed())
                .unwrap_or(false);

            let search_action = buffer_search::Deploy { focus: true };

            bar = bar.with_child(render_quick_action_bar_button(
                1,
                "icons/magnifying_glass_12.svg",
                search_bar_shown,
                (
                    "Buffer search".to_string(),
                    // TODO kb no keybinding is shown for search + toggle inlays does not update icon color
                    Some(Box::new(search_action.clone())),
                ),
                cx,
                move |this, cx| {
                    if search_bar_shown {
                        if let Some(buffer_search_bar) = buffer_search_bar.as_ref() {
                            buffer_search_bar.update(cx, |buffer_search_bar, cx| {
                                buffer_search_bar.dismiss(&buffer_search::Dismiss, cx);
                            });
                        }
                    } else {
                        this.pane.update(cx, |pane, cx| {
                            BufferSearchBar::deploy(pane, &search_action, cx);
                        });
                    }
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
        _: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        match active_pane_item {
            Some(active_item) => {
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
