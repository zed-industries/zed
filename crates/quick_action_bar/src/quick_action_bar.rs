use assistant::assistant_settings::AssistantSettings;
use assistant::{AssistantPanel, InlineAssist};
use editor::actions::{
    AddSelectionAbove, AddSelectionBelow, DuplicateLineDown, GoToDiagnostic, GoToHunk,
    GoToPrevDiagnostic, GoToPrevHunk, MoveLineDown, MoveLineUp, SelectAll, SelectLargerSyntaxNode,
    SelectNext, SelectSmallerSyntaxNode, ToggleGoToLine, ToggleOutline,
};
use editor::{Editor, EditorSettings};

use gpui::{
    anchored, deferred, Action, AnchorCorner, ClickEvent, DismissEvent, ElementId, EventEmitter,
    InteractiveElement, ParentElement, Render, Styled, Subscription, View, ViewContext, WeakView,
};
use search::{buffer_search, BufferSearchBar};
use settings::{Settings, SettingsStore};
use ui::{
    prelude::*, ButtonSize, ButtonStyle, ContextMenu, IconButton, IconName, IconSize, Tooltip,
};
use workspace::{
    item::ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};

pub struct QuickActionBar {
    buffer_search_bar: View<BufferSearchBar>,
    toggle_settings_menu: Option<View<ContextMenu>>,
    toggle_selections_menu: Option<View<ContextMenu>>,
    active_item: Option<Box<dyn ItemHandle>>,
    _inlay_hints_enabled_subscription: Option<Subscription>,
    workspace: WeakView<Workspace>,
    show: bool,
}

impl QuickActionBar {
    pub fn new(
        buffer_search_bar: View<BufferSearchBar>,
        workspace: &Workspace,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            buffer_search_bar,
            toggle_settings_menu: None,
            toggle_selections_menu: None,
            active_item: None,
            _inlay_hints_enabled_subscription: None,
            workspace: workspace.weak_handle(),
            show: true,
        };
        this.apply_settings(cx);
        cx.observe_global::<SettingsStore>(|this, cx| this.apply_settings(cx))
            .detach();
        this
    }

    fn active_editor(&self) -> Option<View<Editor>> {
        self.active_item
            .as_ref()
            .and_then(|item| item.downcast::<Editor>())
    }

    fn apply_settings(&mut self, cx: &mut ViewContext<Self>) {
        let new_show = EditorSettings::get_global(cx).toolbar.quick_actions;
        if new_show != self.show {
            self.show = new_show;
            cx.emit(ToolbarItemEvent::ChangeLocation(
                self.get_toolbar_item_location(),
            ));
        }
    }

    fn get_toolbar_item_location(&self) -> ToolbarItemLocation {
        if self.show && self.active_editor().is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn render_menu_overlay(menu: &View<ContextMenu>) -> Div {
        div().absolute().bottom_0().right_0().size_0().child(
            deferred(
                anchored()
                    .anchor(AnchorCorner::TopRight)
                    .child(menu.clone()),
            )
            .with_priority(1),
        )
    }
}

impl Render for QuickActionBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(editor) = self.active_editor() else {
            return div().id("empty quick action bar");
        };

        let (
            selection_menu_enabled,
            inlay_hints_enabled,
            supports_inlay_hints,
            git_blame_inline_enabled,
        ) = {
            let editor = editor.read(cx);
            let selection_menu_enabled = editor.selection_menu_enabled(cx);
            let inlay_hints_enabled = editor.inlay_hints_enabled();
            let supports_inlay_hints = editor.supports_inlay_hints(cx);
            let git_blame_inline_enabled = editor.git_blame_inline_enabled();

            (
                selection_menu_enabled,
                inlay_hints_enabled,
                supports_inlay_hints,
                git_blame_inline_enabled,
            )
        };

        let search_button = editor.is_singleton(cx).then(|| {
            QuickActionBarButton::new(
                "toggle buffer search",
                IconName::MagnifyingGlass,
                !self.buffer_search_bar.read(cx).is_dismissed(),
                Box::new(buffer_search::Deploy::find()),
                "Buffer Search",
                {
                    let buffer_search_bar = self.buffer_search_bar.clone();
                    move |_, cx| {
                        buffer_search_bar.update(cx, |search_bar, cx| {
                            search_bar.toggle(&buffer_search::Deploy::find(), cx)
                        });
                    }
                },
            )
        });

        let assistant_button = QuickActionBarButton::new(
            "toggle inline assistant",
            IconName::MagicWand,
            false,
            Box::new(InlineAssist),
            "Inline Assist",
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

        let editor_selections_dropdown = selection_menu_enabled.then(|| {
            IconButton::new("toggle_editor_selections_icon", IconName::TextCursor)
                .size(ButtonSize::Compact)
                .icon_size(IconSize::Small)
                .style(ButtonStyle::Subtle)
                .selected(self.toggle_selections_menu.is_some())
                .on_click({
                    let focus = editor.focus_handle(cx);
                    cx.listener(move |quick_action_bar, _, cx| {
                        let focus = focus.clone();
                        let menu = ContextMenu::build(cx, move |menu, _| {
                            menu.context(focus.clone())
                                .action("Select All", Box::new(SelectAll))
                                .action(
                                    "Select Next Occurrence",
                                    Box::new(SelectNext {
                                        replace_newest: false,
                                    }),
                                )
                                .action("Expand Selection", Box::new(SelectLargerSyntaxNode))
                                .action("Shrink Selection", Box::new(SelectSmallerSyntaxNode))
                                .action("Add Cursor Above", Box::new(AddSelectionAbove))
                                .action("Add Cursor Below", Box::new(AddSelectionBelow))
                                .separator()
                                .action("Go to Symbol", Box::new(ToggleOutline))
                                .action("Go to Line/Column", Box::new(ToggleGoToLine))
                                .separator()
                                .action("Next Problem", Box::new(GoToDiagnostic))
                                .action("Previous Problem", Box::new(GoToPrevDiagnostic))
                                .separator()
                                .action("Next Hunk", Box::new(GoToHunk))
                                .action("Previous Hunk", Box::new(GoToPrevHunk))
                                .separator()
                                .action("Move Line Up", Box::new(MoveLineUp))
                                .action("Move Line Down", Box::new(MoveLineDown))
                                .action("Duplicate Selection", Box::new(DuplicateLineDown))
                        });
                        cx.subscribe(&menu, |quick_action_bar, _, _: &DismissEvent, _cx| {
                            quick_action_bar.toggle_selections_menu = None;
                        })
                        .detach();
                        quick_action_bar.toggle_selections_menu = Some(menu);
                    })
                })
                .when(self.toggle_selections_menu.is_none(), |this| {
                    this.tooltip(|cx| Tooltip::text("Selection Controls", cx))
                })
        });

        let editor_settings_dropdown =
            IconButton::new("toggle_editor_settings_icon", IconName::Sliders)
                .size(ButtonSize::Compact)
                .icon_size(IconSize::Small)
                .style(ButtonStyle::Subtle)
                .selected(self.toggle_settings_menu.is_some())
                .on_click({
                    let editor = editor.clone();
                    cx.listener(move |quick_action_bar, _, cx| {
                        let menu = ContextMenu::build(cx, |mut menu, _| {
                            if supports_inlay_hints {
                                menu = menu.toggleable_entry(
                                    "Show Inlay Hints",
                                    inlay_hints_enabled,
                                    Some(editor::actions::ToggleInlayHints.boxed_clone()),
                                    {
                                        let editor = editor.clone();
                                        move |cx| {
                                            editor.update(cx, |editor, cx| {
                                                editor.toggle_inlay_hints(
                                                    &editor::actions::ToggleInlayHints,
                                                    cx,
                                                );
                                            });
                                        }
                                    },
                                );
                            }

                            menu = menu.toggleable_entry(
                                "Show Git Blame Inline",
                                git_blame_inline_enabled,
                                Some(editor::actions::ToggleGitBlameInline.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |cx| {
                                        editor.update(cx, |editor, cx| {
                                            editor.toggle_git_blame_inline(
                                                &editor::actions::ToggleGitBlameInline,
                                                cx,
                                            )
                                        });
                                    }
                                },
                            );

                            menu = menu.toggleable_entry(
                                "Show Selection Menu",
                                selection_menu_enabled,
                                Some(editor::actions::ToggleSelectionMenu.boxed_clone()),
                                {
                                    let editor = editor.clone();
                                    move |cx| {
                                        editor.update(cx, |editor, cx| {
                                            editor.toggle_selection_menu(
                                                &editor::actions::ToggleSelectionMenu,
                                                cx,
                                            )
                                        });
                                    }
                                },
                            );

                            menu
                        });
                        cx.subscribe(&menu, |quick_action_bar, _, _: &DismissEvent, _cx| {
                            quick_action_bar.toggle_settings_menu = None;
                        })
                        .detach();
                        quick_action_bar.toggle_settings_menu = Some(menu);
                    })
                })
                .when(self.toggle_settings_menu.is_none(), |this| {
                    this.tooltip(|cx| Tooltip::text("Editor Controls", cx))
                });

        h_flex()
            .id("quick action bar")
            .gap_3()
            .child(
                h_flex()
                    .gap_1p5()
                    .children(search_button)
                    .children(editor_selections_dropdown)
                    .when(
                        AssistantSettings::get_global(cx).enabled
                            && AssistantSettings::get_global(cx).button,
                        |bar| bar.child(assistant_button),
                    ),
            )
            .child(editor_settings_dropdown)
            .when_some(
                self.toggle_settings_menu.as_ref(),
                |el, toggle_settings_menu| {
                    el.child(Self::render_menu_overlay(toggle_settings_menu))
                },
            )
            .when_some(
                self.toggle_selections_menu.as_ref(),
                |el, toggle_selections_menu| {
                    el.child(Self::render_menu_overlay(toggle_selections_menu))
                },
            )
    }
}

impl EventEmitter<ToolbarItemEvent> for QuickActionBar {}

#[derive(IntoElement)]
struct QuickActionBarButton {
    id: ElementId,
    icon: IconName,
    toggled: bool,
    action: Box<dyn Action>,
    tooltip: SharedString,
    on_click: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
}

impl QuickActionBarButton {
    fn new(
        id: impl Into<ElementId>,
        icon: IconName,
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
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for QuickActionBarButton {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        let tooltip = self.tooltip.clone();
        let action = self.action.boxed_clone();

        IconButton::new(self.id.clone(), self.icon)
            .size(ButtonSize::Compact)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .selected(self.toggled)
            .tooltip(move |cx| Tooltip::for_action(tooltip.clone(), &*action, cx))
            .on_click(move |event, cx| (self.on_click)(event, cx))
    }
}

impl ToolbarItemView for QuickActionBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        self.active_item = active_pane_item.map(ItemHandle::boxed_clone);
        if let Some(active_item) = active_pane_item {
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
            }
        }
        self.get_toolbar_item_location()
    }
}
