use editor::Editor;
use editor::actions::{
    AddSelectionAbove, AddSelectionBelow, DuplicateLineDown, GoToDiagnostic, GoToHunk,
    GoToPreviousDiagnostic, GoToPreviousHunk, MoveLineDown, MoveLineUp, SelectAll,
    SelectLargerSyntaxNode, SelectNext, SelectSmallerSyntaxNode, ToggleGoToLine,
};
use gpui::{App, Entity, Focusable, Window};
use project::DisableAiSettings;
use settings::Settings;
use ui::{ContextMenu, IconName, prelude::*};
use zed_actions::{agent::AddSelectionToThread, outline::ToggleOutline};

use super::{
    QuickActionBarItem, QuickActionElement, QuickActionMenu, QuickActionTarget, VisibilityTrigger,
};

pub(super) struct SelectionsMenu;

impl QuickActionBarItem for SelectionsMenu {
    type Context = Entity<Editor>;
    const ID: &'static str = "toggle-editor-selections";
    const TRIGGERS: &'static [VisibilityTrigger] =
        &[VisibilityTrigger::Settings, VisibilityTrigger::Editor];

    fn context(&self, target: &QuickActionTarget, cx: &mut App) -> Option<Entity<Editor>> {
        let editor = target.editor()?;
        editor
            .read(cx)
            .selection_menu_enabled(cx)
            .then(|| editor.clone())
    }

    fn render(
        &self,
        editor: &Entity<Editor>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> QuickActionElement {
        let editor = editor.clone();
        QuickActionElement::Dropdown {
            icon: IconName::CursorIBeam,
            menu: QuickActionMenu::new("Selection Controls", move |window, cx| {
                build_menu(&editor, window, cx)
            }),
        }
    }
}

fn build_menu(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) -> Entity<ContextMenu> {
    let has_diff_hunks = editor
        .read(cx)
        .buffer()
        .read(cx)
        .snapshot(cx)
        .has_diff_hunks();
    let has_selection = editor.update(cx, |editor, cx| {
        editor.has_non_empty_selection(&editor.display_snapshot(cx))
    });
    let focus_handle = editor.focus_handle(cx);
    let disable_ai = DisableAiSettings::get_global(cx).disable_ai;

    ContextMenu::build(window, cx, move |menu, _, _| {
        menu.context(focus_handle)
            .action("Select All", Box::new(SelectAll))
            .action(
                "Select Next Occurrence",
                Box::new(SelectNext {
                    replace_newest: false,
                }),
            )
            .action("Expand Selection", Box::new(SelectLargerSyntaxNode))
            .action("Shrink Selection", Box::new(SelectSmallerSyntaxNode))
            .action(
                "Add Cursor Above",
                Box::new(AddSelectionAbove {
                    skip_soft_wrap: true,
                }),
            )
            .action(
                "Add Cursor Below",
                Box::new(AddSelectionBelow {
                    skip_soft_wrap: true,
                }),
            )
            .when(!disable_ai, |this| {
                this.separator().action_disabled_when(
                    !has_selection,
                    "Add to Agent Thread",
                    Box::new(AddSelectionToThread),
                )
            })
            .separator()
            .action("Go to Symbol", Box::new(ToggleOutline))
            .action("Go to Line/Column", Box::new(ToggleGoToLine))
            .separator()
            .action("Next Problem", Box::new(GoToDiagnostic::default()))
            .action(
                "Previous Problem",
                Box::new(GoToPreviousDiagnostic::default()),
            )
            .separator()
            .action_disabled_when(!has_diff_hunks, "Next Hunk", Box::new(GoToHunk))
            .action_disabled_when(!has_diff_hunks, "Previous Hunk", Box::new(GoToPreviousHunk))
            .separator()
            .action("Move Line Up", Box::new(MoveLineUp))
            .action("Move Line Down", Box::new(MoveLineDown))
            .action("Duplicate Selection", Box::new(DuplicateLineDown))
    })
}
