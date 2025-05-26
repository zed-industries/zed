use gpui::{ClickEvent, Corner, CursorStyle, Entity, Hsla, MouseButton};

use crate::{ContextMenu, PopoverMenu, prelude::*};

use super::PopoverMenuHandle;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DropdownStyle {
    #[default]
    Solid,
    Ghost,
}

enum LabelKind {
    Text(SharedString),
    Element(AnyElement),
}

#[derive(IntoElement, RegisterComponent)]
pub struct DropdownMenu {
    id: ElementId,
    label: LabelKind,
    style: DropdownStyle,
    menu: Entity<ContextMenu>,
    full_width: bool,
    disabled: bool,
    handle: Option<PopoverMenuHandle<ContextMenu>>,
}

impl DropdownMenu {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        menu: Entity<ContextMenu>,
    ) -> Self {
        Self {
            id: id.into(),
            label: LabelKind::Text(label.into()),
            style: DropdownStyle::default(),
            menu,
            full_width: false,
            disabled: false,
            handle: None,
        }
    }

    pub fn new_with_element(
        id: impl Into<ElementId>,
        label: AnyElement,
        menu: Entity<ContextMenu>,
    ) -> Self {
        Self {
            id: id.into(),
            label: LabelKind::Element(label),
            style: DropdownStyle::default(),
            menu,
            full_width: false,
            disabled: false,
            handle: None,
        }
    }

    pub fn style(mut self, style: DropdownStyle) -> Self {
        self.style = style;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn handle(mut self, handle: PopoverMenuHandle<ContextMenu>) -> Self {
        self.handle = Some(handle);
        self
    }
}

impl Disableable for DropdownMenu {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        PopoverMenu::new(self.id)
            .full_width(self.full_width)
            .menu(move |_window, _cx| Some(self.menu.clone()))
            .trigger(
                DropdownMenuTrigger::new(self.label)
                    .full_width(self.full_width)
                    .disabled(self.disabled)
                    .style(self.style),
            )
            .attach(Corner::BottomLeft)
            .when_some(self.handle.clone(), |el, handle| el.with_handle(handle))
    }
}

impl Component for DropdownMenu {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn name() -> &'static str {
        "DropdownMenu"
    }

    fn description() -> Option<&'static str> {
        Some(
            "A dropdown menu displays a list of actions or options. A dropdown menu is always activated by clicking a trigger (or via a keybinding).",
        )
    }

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let menu = ContextMenu::build(window, cx, |this, _, _| {
            this.entry("Option 1", None, |_, _| {})
                .entry("Option 2", None, |_, _| {})
                .entry("Option 3", None, |_, _| {})
                .separator()
                .entry("Option 4", None, |_, _| {})
        });

        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Usage",
                        vec![
                            single_example(
                                "Default",
                                DropdownMenu::new("default", "Select an option", menu.clone())
                                    .into_any_element(),
                            ),
                            single_example(
                                "Full Width",
                                DropdownMenu::new(
                                    "full-width",
                                    "Full Width Dropdown",
                                    menu.clone(),
                                )
                                .full_width(true)
                                .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "States",
                        vec![single_example(
                            "Disabled",
                            DropdownMenu::new("disabled", "Disabled Dropdown", menu.clone())
                                .disabled(true)
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DropdownTriggerStyle {
    pub bg: Hsla,
}

impl DropdownTriggerStyle {
    pub fn for_style(style: DropdownStyle, cx: &App) -> Self {
        let colors = cx.theme().colors();
        let bg = match style {
            DropdownStyle::Solid => colors.editor_background,
            DropdownStyle::Ghost => colors.ghost_element_background,
        };
        Self { bg }
    }
}

#[derive(IntoElement)]
struct DropdownMenuTrigger {
    label: LabelKind,
    full_width: bool,
    selected: bool,
    disabled: bool,
    style: DropdownStyle,
    cursor_style: CursorStyle,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl DropdownMenuTrigger {
    pub fn new(label: LabelKind) -> Self {
        Self {
            label,
            full_width: false,
            selected: false,
            disabled: false,
            style: DropdownStyle::default(),
            cursor_style: CursorStyle::default(),
            on_click: None,
        }
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn style(mut self, style: DropdownStyle) -> Self {
        self.style = style;
        self
    }
}

impl Disableable for DropdownMenuTrigger {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for DropdownMenuTrigger {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Clickable for DropdownMenuTrigger {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl RenderOnce for DropdownMenuTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled;

        let style = DropdownTriggerStyle::for_style(self.style, cx);

        h_flex()
            .id("dropdown-menu-trigger")
            .justify_between()
            .rounded_sm()
            .bg(style.bg)
            .pl_2()
            .pr_1p5()
            .py_0p5()
            .gap_2()
            .min_w_20()
            .map(|el| {
                if self.full_width {
                    el.w_full()
                } else {
                    el.flex_none().w_auto()
                }
            })
            .map(|el| {
                if disabled {
                    el.cursor_not_allowed()
                } else {
                    el.cursor_pointer()
                }
            })
            .child(match self.label {
                LabelKind::Text(text) => Label::new(text)
                    .color(if disabled {
                        Color::Disabled
                    } else {
                        Color::Default
                    })
                    .into_any_element(),
                LabelKind::Element(element) => element,
            })
            .child(
                Icon::new(IconName::ChevronUpDown)
                    .size(IconSize::XSmall)
                    .color(if disabled {
                        Color::Disabled
                    } else {
                        Color::Muted
                    }),
            )
            .when_some(self.on_click.filter(|_| !disabled), |el, on_click| {
                el.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                    .on_click(move |event, window, cx| {
                        cx.stop_propagation();
                        (on_click)(event, window, cx)
                    })
            })
    }
}
