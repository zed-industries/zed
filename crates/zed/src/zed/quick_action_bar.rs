mod markdown_preview;
mod repl_menu;

use assistant::assistant_settings::AssistantSettings;
use assistant::AssistantPanel;
use editor::actions::{
    AddSelectionAbove, AddSelectionBelow, DuplicateLineDown, GoToDiagnostic, GoToHunk,
    GoToPrevDiagnostic, GoToPrevHunk, MoveLineDown, MoveLineUp, SelectAll, SelectLargerSyntaxNode,
    SelectNext, SelectSmallerSyntaxNode, ToggleGoToLine, ToggleOutline,
};
use editor::{Editor, EditorSettings};
use gpui::{
    Action, AnchorCorner, ClickEvent, ElementId, EventEmitter, FocusHandle, FocusableView,
    InteractiveElement, ParentElement, Render, Styled, Subscription, View, ViewContext, WeakView,
};
use search::{buffer_search, BufferSearchBar};
use settings::{Settings, SettingsStore};
use ui::{
    prelude::*, ButtonStyle, ContextMenu, IconButton, IconButtonShape, IconName, IconSize,
    PopoverMenu, PopoverMenuHandle, Tooltip,
};
use workspace::{
    item::ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};
use zed_actions::InlineAssist;

pub struct QuickActionBar {
    _inlay_hints_enabled_subscription: Option<Subscription>,
    active_item: Option<Box<dyn ItemHandle>>,
    buffer_search_bar: View<BufferSearchBar>,
    show: bool,
    toggle_selections_handle: PopoverMenuHandle<ContextMenu>,
    toggle_settings_handle: PopoverMenuHandle<ContextMenu>,
    workspace: WeakView<Workspace>,
}

impl QuickActionBar {
    pub fn new(
        buffer_search_bar: View<BufferSearchBar>,
        workspace: &Workspace,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            _inlay_hints_enabled_subscription: None,
            active_item: None,
            buffer_search_bar,
            show: true,
            toggle_selections_handle: Default::default(),
            toggle_settings_handle: Default::default(),
            workspace: workspace.weak_handle(),
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
            show_git_blame_gutter,
            auto_signature_help_enabled,
        ) = {
            let editor = editor.read(cx);
            let selection_menu_enabled = editor.selection_menu_enabled(cx);
            let inlay_hints_enabled = editor.inlay_hints_enabled();
            let supports_inlay_hints = editor.supports_inlay_hints(cx);
            let git_blame_inline_enabled = editor.git_blame_inline_enabled();
            let show_git_blame_gutter = editor.show_git_blame_gutter();
            let auto_signature_help_enabled = editor.auto_signature_help_enabled(cx);

            (
                selection_menu_enabled,
                inlay_hints_enabled,
                supports_inlay_hints,
                git_blame_inline_enabled,
                show_git_blame_gutter,
                auto_signature_help_enabled,
            )
        };

        let focus_handle = editor.read(cx).focus_handle(cx);

        let search_button = editor.is_singleton(cx).then(|| {
            QuickActionBarButton::new(
                "toggle buffer search",
                IconName::MagnifyingGlass,
                !self.buffer_search_bar.read(cx).is_dismissed(),
                Box::new(buffer_search::Deploy::find()),
                focus_handle.clone(),
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
            IconName::ZedAssistant,
            false,
            Box::new(InlineAssist::default()),
            focus_handle.clone(),
            "Inline Assist",
            {
                let workspace = self.workspace.clone();
                move |_, cx| {
                    if let Some(workspace) = workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            AssistantPanel::inline_assist(workspace, &InlineAssist::default(), cx);
                        });
                    }
                }
            },
        );

        let editor_selections_dropdown = selection_menu_enabled.then(|| {
            let focus = editor.focus_handle(cx);
            PopoverMenu::new("editor-selections-dropdown")
                .trigger(
                    IconButton::new("toggle_editor_selections_icon", IconName::CursorIBeam)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .style(ButtonStyle::Subtle)
                        .selected(self.toggle_selections_handle.is_deployed())
                        .when(!self.toggle_selections_handle.is_deployed(), |this| {
                            this.tooltip(|cx| Tooltip::text("Selection Controls", cx))
                        }),
                )
                .with_handle(self.toggle_selections_handle.clone())
                .anchor(AnchorCorner::TopRight)
                .menu(move |cx| {
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
                    Some(menu)
                })
        });

        let editor = editor.downgrade();
        let editor_settings_dropdown = PopoverMenu::new("editor-settings")
            .trigger(
                IconButton::new("toggle_editor_settings_icon", IconName::Sliders)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .selected(self.toggle_settings_handle.is_deployed())
                    .when(!self.toggle_settings_handle.is_deployed(), |this| {
                        this.tooltip(|cx| Tooltip::text("Editor Controls", cx))
                    }),
            )
            .anchor(AnchorCorner::TopRight)
            .with_handle(self.toggle_settings_handle.clone())
            .menu(move |cx| {
                let menu = ContextMenu::build(cx, |mut menu, _| {
                    if supports_inlay_hints {
                        menu = menu.toggleable_entry(
                            "Inlay Hints",
                            inlay_hints_enabled,
                            IconPosition::Start,
                            Some(editor::actions::ToggleInlayHints.boxed_clone()),
                            {
                                let editor = editor.clone();
                                move |cx| {
                                    editor
                                        .update(cx, |editor, cx| {
                                            editor.toggle_inlay_hints(
                                                &editor::actions::ToggleInlayHints,
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            },
                        );
                    }

                    menu = menu.toggleable_entry(
                        "Selection Menu",
                        selection_menu_enabled,
                        IconPosition::Start,
                        Some(editor::actions::ToggleSelectionMenu.boxed_clone()),
                        {
                            let editor = editor.clone();
                            move |cx| {
                                editor
                                    .update(cx, |editor, cx| {
                                        editor.toggle_selection_menu(
                                            &editor::actions::ToggleSelectionMenu,
                                            cx,
                                        )
                                    })
                                    .ok();
                            }
                        },
                    );

                    menu = menu.toggleable_entry(
                        "Auto Signature Help",
                        auto_signature_help_enabled,
                        IconPosition::Start,
                        Some(editor::actions::ToggleAutoSignatureHelp.boxed_clone()),
                        {
                            let editor = editor.clone();
                            move |cx| {
                                editor
                                    .update(cx, |editor, cx| {
                                        editor.toggle_auto_signature_help_menu(
                                            &editor::actions::ToggleAutoSignatureHelp,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        },
                    );

                    menu = menu.separator();

                    menu = menu.toggleable_entry(
                        "Inline Git Blame",
                        git_blame_inline_enabled,
                        IconPosition::Start,
                        Some(editor::actions::ToggleGitBlameInline.boxed_clone()),
                        {
                            let editor = editor.clone();
                            move |cx| {
                                editor
                                    .update(cx, |editor, cx| {
                                        editor.toggle_git_blame_inline(
                                            &editor::actions::ToggleGitBlameInline,
                                            cx,
                                        )
                                    })
                                    .ok();
                            }
                        },
                    );

                    menu = menu.toggleable_entry(
                        "Column Git Blame",
                        show_git_blame_gutter,
                        IconPosition::Start,
                        Some(editor::actions::ToggleGitBlame.boxed_clone()),
                        {
                            let editor = editor.clone();
                            move |cx| {
                                editor
                                    .update(cx, |editor, cx| {
                                        editor
                                            .toggle_git_blame(&editor::actions::ToggleGitBlame, cx)
                                    })
                                    .ok();
                            }
                        },
                    );

                    menu
                });
                Some(menu)
            });

        h_flex()
            .id("quick action bar")
            .gap(DynamicSpacing::Base06.rems(cx))
            .children(self.render_repl_menu(cx))
            .children(self.render_toggle_markdown_preview(self.workspace.clone(), cx))
            .children(search_button)
            .when(
                AssistantSettings::get_global(cx).enabled
                    && AssistantSettings::get_global(cx).button,
                |bar| bar.child(assistant_button),
            )
            .children(editor_selections_dropdown)
            .child(editor_settings_dropdown)
    }
}

impl EventEmitter<ToolbarItemEvent> for QuickActionBar {}

#[derive(IntoElement)]
struct QuickActionBarButton {
    id: ElementId,
    icon: IconName,
    toggled: bool,
    action: Box<dyn Action>,
    focus_handle: FocusHandle,
    tooltip: SharedString,
    on_click: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
}

impl QuickActionBarButton {
    fn new(
        id: impl Into<ElementId>,
        icon: IconName,
        toggled: bool,
        action: Box<dyn Action>,
        focus_handle: FocusHandle,
        tooltip: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            toggled,
            action,
            focus_handle,
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
            .shape(IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .selected(self.toggled)
            .tooltip(move |cx| {
                Tooltip::for_action_in(tooltip.clone(), &*action, &self.focus_handle, cx)
            })
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
