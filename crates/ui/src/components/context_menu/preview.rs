use gpui::{Anchor, Entity, FocusHandle};

use crate::{ListItemSpacing, PopoverMenu, prelude::*, right_click_menu};

use super::{ContextMenu, ContextMenuEntry, ContextMenuSubmenu, DocumentationSide};

#[derive(Clone, PartialEq, gpui::Action)]
#[action(namespace = context_menu_preview, no_json, no_register)]
struct PreviewAction;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Example {
    Standard,
    Persistent,
    Submenu,
    Removal,
    ParentRefresh,
    Nested,
    Content,
    Placement,
}

impl Example {
    const ALL: [Self; 8] = [
        Self::Standard,
        Self::Persistent,
        Self::Submenu,
        Self::Removal,
        Self::ParentRefresh,
        Self::Nested,
        Self::Content,
        Self::Placement,
    ];

    fn title(self) -> &'static str {
        match self {
            Self::Standard => "Standard menus",
            Self::Persistent => "Persistent menus",
            Self::Submenu => "Persistent submenus",
            Self::Removal => "Removing submenu entries",
            Self::ParentRefresh => "Refreshing the parent",
            Self::Nested => "Nested menus",
            Self::Content => "Content, width, and spacing",
            Self::Placement => "Placement and right-click menus",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum TriggerState {
    #[default]
    Enabled,
    Disabled,
    Removed,
}

struct ExampleValues {
    archived: bool,
    confirmations: usize,
    filters: Vec<&'static str>,
    trigger_moved: bool,
    trigger_state: TriggerState,
}

impl Default for ExampleValues {
    fn default() -> Self {
        Self {
            archived: false,
            confirmations: 0,
            filters: vec!["Assigned to me", "In progress", "Updated today"],
            trigger_moved: false,
            trigger_state: TriggerState::Enabled,
        }
    }
}

struct MenuPreview {
    example: Example,
    values: ExampleValues,
    focus_handle: FocusHandle,
}

impl MenuPreview {
    fn new(example: Example, cx: &mut Context<Self>) -> Self {
        Self {
            example,
            values: ExampleValues::default(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn handler(
        preview: &Entity<Self>,
        update: impl Fn(&mut ExampleValues) + 'static,
    ) -> impl Fn(&mut Window, &mut App) + 'static {
        let preview = preview.clone();
        move |_, cx| {
            preview.update(cx, |this, cx| {
                update(&mut this.values);
                this.values.confirmations += 1;
                cx.notify();
            });
        }
    }

    fn open_menu(
        preview: &Entity<Self>,
        spacing: ListItemSpacing,
        width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ContextMenu> {
        let example = preview.read(cx).example;
        let builder = {
            let preview = preview.clone();
            move |menu: ContextMenu, _: &mut Window, cx: &mut Context<ContextMenu>| {
                Self::build_menu(menu, &preview, cx)
                    .spacing(spacing)
                    .when_some(width, |menu, width| menu.fixed_width(width.into()))
            }
        };
        if matches!(example, Example::Persistent | Example::ParentRefresh) {
            ContextMenu::build_persistent(window, cx, builder)
        } else {
            ContextMenu::build(window, cx, builder)
        }
    }

    fn filter_entries(menu: ContextMenu, preview: &Entity<Self>, cx: &App) -> ContextMenu {
        let values = &preview.read(cx).values;
        let confirmations = values.confirmations;
        menu.keep_open_on_confirm(true)
            .header(format!("Confirmed {confirmations} times"))
            .item(
                ContextMenuEntry::new("Include archived")
                    .toggleable(IconPosition::Start, values.archived)
                    .handler(Self::handler(preview, |values| {
                        values.archived = !values.archived;
                    })),
            )
            .item(
                ContextMenuEntry::new("Archived-only option")
                    .disabled(!values.archived)
                    .handler(Self::handler(preview, |_| {})),
            )
            .separator()
            .custom_entry(
                move |_, _| {
                    h_flex()
                        .w_full()
                        .gap_3()
                        .justify_between()
                        .child(Label::new("Custom counter row"))
                        .child(Label::new(confirmations.to_string()).color(Color::Muted))
                        .into_any_element()
                },
                Self::handler(preview, |_| {}),
            )
    }

    fn build_menu(menu: ContextMenu, preview: &Entity<Self>, cx: &App) -> ContextMenu {
        let this = preview.read(cx);
        match this.example {
            Example::Standard => menu
                .context(this.focus_handle.clone())
                .header("Commands")
                .action("Dispatch preview action", Box::new(PreviewAction))
                .item(
                    ContextMenuEntry::new("Disabled command")
                        .disabled(true)
                        .handler(Self::handler(preview, |_| {})),
                )
                .label("This label is not selectable")
                .separator()
                .submenu("Ordinary submenu", {
                    let preview = preview.clone();
                    move |menu, _, _| {
                        menu.entry("Run child command", None, Self::handler(&preview, |_| {}))
                    }
                })
                .item(
                    ContextMenuSubmenu::new("Disabled submenu", |menu, _, _| {
                        menu.label("This submenu should not open")
                    })
                    .disabled(true),
                )
                .entry("Last command", None, Self::handler(preview, |_| {})),
            Example::Persistent => Self::filter_entries(menu, preview, cx),
            Example::Submenu => menu
                .submenu_with_icon("Filters", IconName::Filter, {
                    let preview = preview.clone();
                    move |menu, _, cx| Self::filter_entries(menu, &preview, cx)
                })
                .entry("Parent command", None, Self::handler(preview, |_| {})),
            Example::Removal => menu
                .submenu("Active filters", {
                    let preview = preview.clone();
                    move |menu, _, cx| {
                        let values = &preview.read(cx).values;
                        menu.keep_open_on_confirm(true)
                            .header(format!("{} remaining", values.filters.len()))
                            .label("Select a filter to remove it")
                            .extend(values.filters.iter().map(|&filter| {
                                ContextMenuEntry::new(filter)
                                    .toggleable(IconPosition::End, true)
                                    .handler(Self::handler(&preview, move |values| {
                                        values.filters.retain(|value| *value != filter)
                                    }))
                            }))
                    }
                })
                .entry("Parent command", None, Self::handler(preview, |_| {})),
            Example::ParentRefresh => {
                let values = &this.values;
                menu.keep_open_on_confirm(false)
                    .entry("Before filters", None, Self::handler(preview, |_| {}))
                    .when(values.trigger_moved, |menu| {
                        menu.entry("Inserted neighbor", None, Self::handler(preview, |_| {}))
                    })
                    .when(values.trigger_state != TriggerState::Removed, |menu| {
                        menu.item(
                            ContextMenuSubmenu::new(
                                format!("Filters ({} changes)", values.confirmations),
                                {
                                    let preview = preview.clone();
                                    move |menu, _, cx| {
                                        Self::filter_entries(menu, &preview, cx)
                                            .rebuild_parent_on_confirm(true)
                                            .separator()
                                            .entry(
                                                "Move trigger",
                                                None,
                                                Self::handler(&preview, |values| {
                                                    values.trigger_moved = !values.trigger_moved;
                                                }),
                                            )
                                            .entry(
                                                "Disable trigger",
                                                None,
                                                Self::handler(&preview, |values| {
                                                    values.trigger_state = TriggerState::Disabled;
                                                }),
                                            )
                                            .entry(
                                                "Remove trigger",
                                                None,
                                                Self::handler(&preview, |values| {
                                                    values.trigger_state = TriggerState::Removed;
                                                }),
                                            )
                                    }
                                },
                            )
                            .id("filters")
                            .icon(IconName::Filter)
                            .disabled(values.trigger_state == TriggerState::Disabled),
                        )
                    })
                    .entry("After filters", None, Self::handler(preview, |_| {}))
            }
            Example::Nested => menu.submenu("Filters", {
                let preview = preview.clone();
                move |menu, _, _| {
                    menu.submenu("Status", {
                        let preview = preview.clone();
                        move |menu, _, cx| Self::filter_entries(menu, &preview, cx)
                    })
                    .entry(
                        "Intermediate command",
                        None,
                        Self::handler(&preview, |_| {}),
                    )
                }
            }),
            Example::Content => menu
                .header("Content variants")
                .item(
                    ContextMenuEntry::new("Leading colored icon")
                        .icon(IconName::Filter)
                        .icon_color(Color::Accent)
                        .handler(Self::handler(preview, |_| {})),
                )
                .item(
                    ContextMenuEntry::new("Trailing icon")
                        .icon(IconName::ArrowUpRight)
                        .icon_position(IconPosition::End)
                        .handler(Self::handler(preview, |_| {})),
                )
                .toggleable_entry(
                    "Checked option",
                    true,
                    IconPosition::End,
                    None,
                    Self::handler(preview, |_| {}),
                )
                .separator()
                .item(
                    ContextMenuEntry::new("Documentation on the right")
                        .documentation_aside(DocumentationSide::Right, |_| {
                            v_flex()
                                .p_3()
                                .gap_1()
                                .child(Label::new("Documentation aside"))
                                .child(
                                    Label::new("Shown on hover or keyboard selection.")
                                        .size(LabelSize::Small),
                                )
                                .into_any_element()
                        })
                        .handler(Self::handler(preview, |_| {})),
                )
                .item(
                    ContextMenuEntry::new("Disabled with documentation")
                        .disabled(true)
                        .documentation_aside(DocumentationSide::Left, |_| {
                            div()
                                .p_3()
                                .child(Label::new("Unavailable, but still explains why."))
                                .into_any_element()
                        }),
                )
                .custom_row(|_, _| {
                    Label::new("Non-selectable custom row")
                        .color(Color::Muted)
                        .size(LabelSize::Small)
                        .into_any_element()
                })
                .custom_entry(
                    |_, _| {
                        h_flex()
                            .gap_2()
                            .child(Icon::new(IconName::Check).color(Color::Success))
                            .child(Label::new("Selectable custom row"))
                            .into_any_element()
                    },
                    Self::handler(preview, |_| {}),
                )
                .separator()
                .end_slot_action(Box::new(PreviewAction))
                .entry_with_end_slot(
                    "Always-visible end slot",
                    None,
                    Self::handler(preview, |_| {}),
                    IconName::Settings,
                    "Secondary action".into(),
                    Self::handler(preview, |_| {}),
                )
                .entry_with_end_slot_on_hover(
                    "Hover-only end slot",
                    None,
                    Self::handler(preview, |_| {}),
                    IconName::Settings,
                    "Secondary action".into(),
                    Self::handler(preview, |_| {}),
                ),
            Example::Placement => menu
                .header("Window edges")
                .entry("First command", None, Self::handler(preview, |_| {}))
                .submenu("Open toward available space", {
                    let preview = preview.clone();
                    move |menu, _, _| {
                        menu.entry("Submenu command", None, Self::handler(&preview, |_| {}))
                            .entry(
                                "Another submenu command",
                                None,
                                Self::handler(&preview, |_| {}),
                            )
                    }
                })
                .entry("Last command", None, Self::handler(preview, |_| {})),
        }
    }

    fn trigger(
        label: &'static str,
        spacing: ListItemSpacing,
        width: Option<Pixels>,
        anchor: Anchor,
        cx: &Context<Self>,
    ) -> Div {
        let preview = cx.entity();
        div()
            .flex()
            .self_start()
            .debug_selector(move || label.to_string())
            .child(
                PopoverMenu::new(label)
                    .trigger(Button::new(label, label).style(ButtonStyle::Outlined))
                    .anchor(anchor)
                    .attach(match anchor {
                        Anchor::TopRight => Anchor::BottomRight,
                        _ => Anchor::BottomLeft,
                    })
                    .menu(move |window, cx| {
                        Some(Self::open_menu(&preview, spacing, width, window, cx))
                    }),
            )
    }
}

impl Render for MenuPreview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let controls = match self.example {
            Example::Content => h_flex()
                .flex_wrap()
                .gap_2()
                .child(Self::trigger(
                    "Extra dense · 240px",
                    ListItemSpacing::ExtraDense,
                    Some(px(240.)),
                    Anchor::TopLeft,
                    cx,
                ))
                .child(Self::trigger(
                    "Dense · automatic width",
                    ListItemSpacing::Dense,
                    None,
                    Anchor::TopLeft,
                    cx,
                ))
                .child(Self::trigger(
                    "Sparse · 360px",
                    ListItemSpacing::Sparse,
                    Some(px(360.)),
                    Anchor::TopLeft,
                    cx,
                ))
                .into_any_element(),
            Example::Placement => {
                let preview = cx.entity();
                v_flex()
                    .w_full()
                    .gap_3()
                    .child(
                        h_flex()
                            .justify_between()
                            .child(Self::trigger(
                                "Left edge",
                                ListItemSpacing::Dense,
                                None,
                                Anchor::TopLeft,
                                cx,
                            ))
                            .child(Self::trigger(
                                "Right edge",
                                ListItemSpacing::Dense,
                                None,
                                Anchor::TopRight,
                                cx,
                            )),
                    )
                    .child(
                        right_click_menu("placement-surface")
                            .trigger(|_, _, cx| {
                                v_flex()
                                    .w_full()
                                    .h(px(160.))
                                    .items_center()
                                    .justify_center()
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .rounded_md()
                                    .child(Label::new("Right-click menu").color(Color::Muted))
                            })
                            .menu(move |window, cx| {
                                Self::open_menu(&preview, ListItemSpacing::Dense, None, window, cx)
                            }),
                    )
                    .into_any_element()
            }
            _ => Self::trigger(
                "Open menu",
                ListItemSpacing::Dense,
                None,
                Anchor::TopLeft,
                cx,
            )
            .into_any_element(),
        };

        v_flex()
            .id("context-menu-example")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &PreviewAction, _, cx| {
                this.values.confirmations += 1;
                cx.notify();
            }))
            .w_full()
            .child(controls)
    }
}

impl Component for ContextMenu {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> &'static str {
        ""
    }

    fn preview(window: &mut Window, cx: &mut App) -> AnyElement {
        example_group(
            Example::ALL
                .into_iter()
                .map(|example| {
                    let preview = window.use_keyed_state(
                        (
                            ElementId::Name("context-menu-preview".into()),
                            example.title(),
                        ),
                        cx,
                        |_, cx| MenuPreview::new(example, cx),
                    );
                    single_example(example.title(), preview.into_any_element())
                })
                .collect(),
        )
        .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Modifiers, TestAppContext, VisualTestContext, size};

    use super::*;

    #[gpui::test]
    fn examples_open_record_actions_and_retain_values(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });

        for example in Example::ALL {
            let window = cx.open_window(size(px(900.), px(700.)), |_, cx| {
                MenuPreview::new(example, cx)
            });
            let preview = window.root(cx).expect("preview");
            let mut visual = VisualTestContext::from_window(window.into(), cx);
            let focus_handle = visual.update(|window, cx| {
                let focus_handle = preview.read(cx).focus_handle.clone();
                window.focus(&focus_handle, cx);
                window.draw(cx).clear(cx);
                focus_handle
            });
            let trigger = match example {
                Example::Content => "Dense · automatic width",
                Example::Placement => "Left edge",
                _ => "Open menu",
            };

            for open_count in 1..=2 {
                let bounds = visual.debug_bounds(trigger).expect("menu trigger");
                visual.simulate_click(bounds.center(), Modifiers::default());
                visual.run_until_parked();
                for _ in 0..2 {
                    visual.update(|window, cx| {
                        window.simulate_next_frame(cx);
                        window.draw(cx).clear(cx);
                    });
                }
                visual.update(|window, _| {
                    assert!(!focus_handle.is_focused(window), "{}", example.title());
                });

                if matches!(example, Example::Standard | Example::Persistent) {
                    visual.dispatch_action(menu::SelectFirst);
                    visual.dispatch_action(menu::Confirm);
                    visual.run_until_parked();
                    preview.read_with(&visual, |preview, _| {
                        assert_eq!(
                            preview.values.confirmations,
                            open_count,
                            "{}",
                            example.title()
                        );
                        if example == Example::Persistent {
                            assert_eq!(preview.values.archived, open_count % 2 != 0);
                        }
                    });
                    if example == Example::Persistent {
                        visual.update(|window, _| assert!(!focus_handle.is_focused(window)));
                    }
                }

                if example != Example::Standard {
                    visual.dispatch_action(menu::Cancel);
                }
                visual.run_until_parked();
                visual.update(|window, cx| window.draw(cx).clear(cx));
                visual.update(|window, _| {
                    assert!(focus_handle.is_focused(window), "{}", example.title());
                });
            }
        }
    }
}
