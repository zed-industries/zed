use crate::{ProjectDiagnosticsEditor, ToggleWarnings};
use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    Action, Entity, EventContext, View, ViewContext, WeakViewHandle,
};
use workspace::{item::ItemHandle, ToolbarItemLocation, ToolbarItemView};

pub struct ToolbarControls {
    editor: Option<WeakViewHandle<ProjectDiagnosticsEditor>>,
}

impl Entity for ToolbarControls {
    type Event = ();
}

impl View for ToolbarControls {
    fn ui_name() -> &'static str {
        "ToolbarControls"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let include_warnings = self
            .editor
            .as_ref()
            .and_then(|editor| editor.upgrade(cx))
            .map(|editor| editor.read(cx).include_warnings)
            .unwrap_or(false);
        let tooltip = if include_warnings {
            "Exclude Warnings".into()
        } else {
            "Include Warnings".into()
        };
        Flex::row()
            .with_child(render_toggle_button(
                0,
                "icons/warning.svg",
                include_warnings,
                (tooltip, Some(Box::new(ToggleWarnings))),
                cx,
                move |this, cx| {
                    if let Some(editor) = this.editor.and_then(|editor| editor.upgrade(cx)) {
                        editor.update(cx, |editor, cx| {
                            editor.toggle_warnings(&Default::default(), cx)
                        });
                    }
                },
            ))
            .into_any()
    }
}

impl ToolbarItemView for ToolbarControls {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        if let Some(pane_item) = active_pane_item.as_ref() {
            if let Some(editor) = pane_item.downcast::<ProjectDiagnosticsEditor>() {
                self.editor = Some(editor.downgrade());
                ToolbarItemLocation::PrimaryRight { flex: None }
            } else {
                ToolbarItemLocation::Hidden
            }
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl ToolbarControls {
    pub fn new() -> Self {
        ToolbarControls { editor: None }
    }
}

fn render_toggle_button<
    F: 'static + Fn(&mut ToolbarControls, &mut EventContext<ToolbarControls>),
>(
    index: usize,
    icon: &'static str,
    toggled: bool,
    tooltip: (String, Option<Box<dyn Action>>),
    cx: &mut ViewContext<ToolbarControls>,
    on_click: F,
) -> AnyElement<ToolbarControls> {
    enum Button {}

    let theme = theme::current(cx);
    let (tooltip_text, action) = tooltip;

    MouseEventHandler::new::<Button, _>(index, cx, |mouse_state, _| {
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
    .on_click(MouseButton::Left, move |_, view, cx| on_click(view, cx))
    .with_tooltip::<Button>(index, tooltip_text, action, theme.tooltip.clone(), cx)
    .into_any_named("quick action bar button")
}
