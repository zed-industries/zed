use gpui::{Corner, Entity, Pixels, Point};

use crate::{ContextMenu, PopoverMenu, prelude::*};

use super::PopoverMenuHandle;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DropdownStyle {
    #[default]
    Solid,
    Outlined,
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
    trigger_size: ButtonSize,
    style: DropdownStyle,
    menu: Entity<ContextMenu>,
    full_width: bool,
    disabled: bool,
    handle: Option<PopoverMenuHandle<ContextMenu>>,
    attach: Option<Corner>,
    offset: Option<Point<Pixels>>,
    tab_index: Option<isize>,
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
            trigger_size: ButtonSize::Default,
            style: DropdownStyle::default(),
            menu,
            full_width: false,
            disabled: false,
            handle: None,
            attach: None,
            offset: None,
            tab_index: None,
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
            trigger_size: ButtonSize::Default,
            style: DropdownStyle::default(),
            menu,
            full_width: false,
            disabled: false,
            handle: None,
            attach: None,
            offset: None,
            tab_index: None,
        }
    }

    pub fn trigger_size(mut self, size: ButtonSize) -> Self {
        self.trigger_size = size;
        self
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

    /// Defines which corner of the handle to attach the menu's anchor to.
    pub fn attach(mut self, attach: Corner) -> Self {
        self.attach = Some(attach);
        self
    }

    /// Offsets the position of the menu by that many pixels.
    pub fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn tab_index(mut self, arg: isize) -> Self {
        self.tab_index = Some(arg);
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
        let button_style = match self.style {
            DropdownStyle::Solid => ButtonStyle::Filled,
            DropdownStyle::Outlined => ButtonStyle::Outlined,
            DropdownStyle::Ghost => ButtonStyle::Transparent,
        };

        let full_width = self.full_width;
        let trigger_size = self.trigger_size;

        let button = match self.label {
            LabelKind::Text(text) => Button::new(self.id.clone(), text)
                .style(button_style)
                .icon(IconName::ChevronUpDown)
                .icon_position(IconPosition::End)
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .when(full_width, |this| this.full_width())
                .size(trigger_size)
                .disabled(self.disabled),
            LabelKind::Element(_element) => Button::new(self.id.clone(), "")
                .style(button_style)
                .icon(IconName::ChevronUpDown)
                .icon_position(IconPosition::End)
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .when(full_width, |this| this.full_width())
                .size(trigger_size)
                .disabled(self.disabled),
        }
        .when_some(self.tab_index, |this, tab_index| this.tab_index(tab_index));

        PopoverMenu::new((self.id.clone(), "popover"))
            .full_width(self.full_width)
            .menu(move |_window, _cx| Some(self.menu.clone()))
            .trigger(button)
            .attach(match self.attach {
                Some(attach) => attach,
                None => Corner::BottomRight,
            })
            .when_some(self.offset, |this, offset| this.offset(offset))
            .when_some(self.handle, |this, handle| this.with_handle(handle))
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
                        "Styles",
                        vec![
                            single_example(
                                "Outlined",
                                DropdownMenu::new("outlined", "Outlined Dropdown", menu.clone())
                                    .style(DropdownStyle::Outlined)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Ghost",
                                DropdownMenu::new("ghost", "Ghost Dropdown", menu.clone())
                                    .style(DropdownStyle::Ghost)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "States",
                        vec![single_example(
                            "Disabled",
                            DropdownMenu::new("disabled", "Disabled Dropdown", menu)
                                .disabled(true)
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
