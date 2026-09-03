use std::sync::Arc;

use editor::Editor;
use editor::actions::{ToggleDiagnostics, ToggleInlineDiagnostics};
use fs::Fs;
use gpui::{Action, App, Context, Entity, Focusable, WeakEntity, Window};
use project::project_settings::{DiagnosticSeverity, ProjectSettings};
use settings::{GitDiffBaseSetting, Settings, update_settings_file};
use ui::{ContextMenu, ContextMenuEntry, DocumentationSide, prelude::*};
use vim_mode_setting::{HelixModeSetting, VimModeSetting};
use workspace::Workspace;

use super::{
    QuickActionBarItem, QuickActionElement, QuickActionMenu, QuickActionTarget, VisibilityTrigger,
};

pub(super) struct EditorSettingsMenu;

#[derive(PartialEq)]
pub(super) struct EditorSettingsMenuContext {
    editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
}

/// Snapshot of everything the menu displays, taken when the menu is opened.
struct EditorSettingsState {
    supports_inlay_hints: bool,
    inlay_hints_enabled: bool,
    inline_values_enabled: bool,
    supports_semantic_tokens: bool,
    semantic_highlights_enabled: bool,
    supports_code_lens: bool,
    code_lens_enabled: bool,
    is_full: bool,
    diagnostics_enabled: bool,
    supports_inline_diagnostics: bool,
    inline_diagnostics_enabled: bool,
    git_blame_inline_enabled: bool,
    show_git_blame_gutter: bool,
    auto_signature_help_enabled: bool,
    show_line_numbers: bool,
    has_edit_prediction_provider: bool,
    show_edit_predictions: bool,
    edit_predictions_enabled_at_cursor: bool,
    supports_minimap: bool,
    minimap_enabled: bool,
    selection_menu_enabled: bool,
    vim_mode_enabled: bool,
    helix_mode_enabled: bool,
    diff_against_default_branch: bool,
}

impl EditorSettingsState {
    fn read(editor: &Entity<Editor>, cx: &mut App) -> Self {
        let supports_inlay_hints = editor.update(cx, |editor, cx| editor.supports_inlay_hints(cx));
        let supports_semantic_tokens =
            editor.update(cx, |editor, cx| editor.supports_semantic_tokens(cx));
        let supports_code_lens = editor.update(cx, |editor, cx| editor.supports_code_lens(cx));

        let editor_value = editor.read(cx);
        let diagnostics_enabled = editor_value.diagnostics_enabled()
            && editor_value.diagnostics_max_severity != DiagnosticSeverity::Off;
        let supports_minimap = editor_value.supports_minimap(cx);

        Self {
            supports_inlay_hints,
            inlay_hints_enabled: editor_value.inlay_hints_enabled(),
            inline_values_enabled: editor_value.inline_values_enabled(),
            supports_semantic_tokens,
            semantic_highlights_enabled: editor_value.semantic_highlights_enabled(),
            supports_code_lens,
            code_lens_enabled: editor_value.code_lens_enabled(),
            is_full: editor_value.mode().is_full(),
            diagnostics_enabled,
            supports_inline_diagnostics: editor_value.inline_diagnostics_enabled(),
            inline_diagnostics_enabled: editor_value.show_inline_diagnostics(),
            git_blame_inline_enabled: editor_value.git_blame_inline_enabled(),
            show_git_blame_gutter: editor_value.show_git_blame_gutter(),
            auto_signature_help_enabled: editor_value.auto_signature_help_enabled(cx),
            show_line_numbers: editor_value.line_numbers_enabled(cx),
            has_edit_prediction_provider: editor_value.edit_prediction_provider().is_some(),
            show_edit_predictions: editor_value.edit_predictions_enabled(),
            edit_predictions_enabled_at_cursor: editor_value.edit_predictions_enabled_at_cursor(cx),
            supports_minimap,
            minimap_enabled: supports_minimap && editor_value.minimap().is_some(),
            selection_menu_enabled: editor_value.selection_menu_enabled(cx),
            vim_mode_enabled: VimModeSetting::get_global(cx).0,
            helix_mode_enabled: HelixModeSetting::get_global(cx).0,
            diff_against_default_branch: ProjectSettings::get_global(cx).git.diff_base
                == GitDiffBaseSetting::DefaultBranch,
        }
    }
}

/// Wraps an editor operation into a menu handler that is a no-op once the editor is gone.
fn with_editor(
    editor: &WeakEntity<Editor>,
    operation: impl Fn(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
) -> impl Fn(&mut Window, &mut App) + 'static {
    let editor = editor.clone();
    move |window, cx| {
        editor
            .update(cx, |editor, cx| operation(editor, window, cx))
            .ok();
    }
}

fn language_features_section(
    menu: ContextMenu,
    state: &EditorSettingsState,
    editor: &WeakEntity<Editor>,
) -> ContextMenu {
    menu.when(state.supports_inlay_hints, |menu| {
        menu.toggleable_entry(
            "Inlay Hints",
            state.inlay_hints_enabled,
            IconPosition::Start,
            Some(editor::actions::ToggleInlayHints.boxed_clone()),
            with_editor(editor, |editor, window, cx| {
                editor.toggle_inlay_hints(&editor::actions::ToggleInlayHints, window, cx);
            }),
        )
        .toggleable_entry(
            "Inline Values",
            state.inline_values_enabled,
            IconPosition::Start,
            Some(editor::actions::ToggleInlineValues.boxed_clone()),
            with_editor(editor, |editor, window, cx| {
                editor.toggle_inline_values(&editor::actions::ToggleInlineValues, window, cx);
            }),
        )
    })
    .when(state.supports_semantic_tokens, |menu| {
        menu.toggleable_entry(
            "Semantic Highlights",
            state.semantic_highlights_enabled,
            IconPosition::Start,
            Some(editor::actions::ToggleSemanticHighlights.boxed_clone()),
            with_editor(editor, |editor, window, cx| {
                editor.toggle_semantic_highlights(
                    &editor::actions::ToggleSemanticHighlights,
                    window,
                    cx,
                );
            }),
        )
    })
    .when(state.supports_code_lens, |menu| {
        menu.toggleable_entry(
            "Code Lens",
            state.code_lens_enabled,
            IconPosition::Start,
            Some(editor::actions::ToggleCodeLens.boxed_clone()),
            with_editor(editor, |editor, window, cx| {
                editor.toggle_code_lens_action(&editor::actions::ToggleCodeLens, window, cx);
            }),
        )
    })
    .when(state.supports_minimap, |menu| {
        menu.toggleable_entry(
            "Minimap",
            state.minimap_enabled,
            IconPosition::Start,
            Some(editor::actions::ToggleMinimap.boxed_clone()),
            with_editor(editor, |editor, window, cx| {
                editor.toggle_minimap(&editor::actions::ToggleMinimap, window, cx);
            }),
        )
    })
    .when(state.has_edit_prediction_provider, |menu| {
        menu.item(
            ContextMenuEntry::new("Edit Predictions")
                .toggleable(
                    IconPosition::Start,
                    state.edit_predictions_enabled_at_cursor && state.show_edit_predictions,
                )
                .disabled(!state.edit_predictions_enabled_at_cursor)
                .action(editor::actions::ToggleEditPrediction.boxed_clone())
                .handler(with_editor(editor, |editor, window, cx| {
                    editor.toggle_edit_predictions(
                        &editor::actions::ToggleEditPrediction,
                        window,
                        cx,
                    );
                }))
                .when(!state.edit_predictions_enabled_at_cursor, |entry| {
                    entry.documentation_aside(DocumentationSide::Left, |_| {
                        Label::new(
                            "You can't toggle edit predictions for this file \
                            as it is within the excluded files list.",
                        )
                        .into_any_element()
                    })
                }),
        )
    })
}

fn diagnostics_section(
    menu: ContextMenu,
    state: &EditorSettingsState,
    editor: &WeakEntity<Editor>,
) -> ContextMenu {
    menu.when(state.is_full, |menu| {
        menu.toggleable_entry(
            "Diagnostics",
            state.diagnostics_enabled,
            IconPosition::Start,
            Some(ToggleDiagnostics.boxed_clone()),
            with_editor(editor, |editor, window, cx| {
                editor.toggle_diagnostics(&ToggleDiagnostics, window, cx);
            }),
        )
        .when(state.supports_inline_diagnostics, |menu| {
            menu.item(
                ContextMenuEntry::new("Inline Diagnostics")
                    .toggleable(
                        IconPosition::Start,
                        state.diagnostics_enabled && state.inline_diagnostics_enabled,
                    )
                    .action(ToggleInlineDiagnostics.boxed_clone())
                    .handler(with_editor(editor, |editor, window, cx| {
                        editor.toggle_inline_diagnostics(&ToggleInlineDiagnostics, window, cx);
                    }))
                    .when(!state.diagnostics_enabled, |entry| {
                        entry
                            .disabled(true)
                            .documentation_aside(DocumentationSide::Left, |_| {
                                Label::new(
                                    "Inline diagnostics are not available until \
                                    regular diagnostics are enabled.",
                                )
                                .into_any_element()
                            })
                    }),
            )
        })
        .separator()
    })
}

fn display_section(
    menu: ContextMenu,
    state: &EditorSettingsState,
    editor: &WeakEntity<Editor>,
) -> ContextMenu {
    menu.toggleable_entry(
        "Line Numbers",
        state.show_line_numbers,
        IconPosition::Start,
        Some(editor::actions::ToggleLineNumbers.boxed_clone()),
        with_editor(editor, |editor, window, cx| {
            editor.toggle_line_numbers(&editor::actions::ToggleLineNumbers, window, cx);
        }),
    )
    .toggleable_entry(
        "Selection Menu",
        state.selection_menu_enabled,
        IconPosition::Start,
        Some(editor::actions::ToggleSelectionMenu.boxed_clone()),
        with_editor(editor, |editor, window, cx| {
            editor.toggle_selection_menu(&editor::actions::ToggleSelectionMenu, window, cx);
        }),
    )
    .toggleable_entry(
        "Auto Signature Help",
        state.auto_signature_help_enabled,
        IconPosition::Start,
        Some(editor::actions::ToggleAutoSignatureHelp.boxed_clone()),
        with_editor(editor, |editor, window, cx| {
            editor.toggle_auto_signature_help_menu(
                &editor::actions::ToggleAutoSignatureHelp,
                window,
                cx,
            );
        }),
    )
}

fn git_section(
    menu: ContextMenu,
    state: &EditorSettingsState,
    editor: &WeakEntity<Editor>,
    fs: Option<Arc<dyn Fs>>,
) -> ContextMenu {
    menu.toggleable_entry(
        "Inline Git Blame",
        state.git_blame_inline_enabled,
        IconPosition::Start,
        Some(editor::actions::ToggleGitBlameInline.boxed_clone()),
        with_editor(editor, |editor, window, cx| {
            editor.toggle_git_blame_inline(&editor::actions::ToggleGitBlameInline, window, cx);
        }),
    )
    .toggleable_entry(
        "Column Git Blame",
        state.show_git_blame_gutter,
        IconPosition::Start,
        Some(git::Blame.boxed_clone()),
        with_editor(editor, |editor, window, cx| {
            editor.toggle_git_blame(&git::Blame, window, cx);
        }),
    )
    .when_some(fs, |menu, fs| {
        let diff_against_default_branch = state.diff_against_default_branch;
        menu.toggleable_entry(
            "Diff Against Default Branch",
            diff_against_default_branch,
            IconPosition::Start,
            None,
            move |_window, cx| {
                let diff_base = if diff_against_default_branch {
                    GitDiffBaseSetting::Head
                } else {
                    GitDiffBaseSetting::DefaultBranch
                };
                update_settings_file(fs.clone(), cx, move |settings, _| {
                    settings.git.get_or_insert_default().diff_base = Some(diff_base);
                });
            },
        )
    })
}

fn modal_editing_section(menu: ContextMenu, state: &EditorSettingsState) -> ContextMenu {
    let vim_mode_enabled = state.vim_mode_enabled;
    let helix_mode_enabled = state.helix_mode_enabled;

    menu.toggleable_entry(
        "Vim Mode",
        vim_mode_enabled,
        IconPosition::Start,
        None,
        move |window, cx| {
            let new_value = !vim_mode_enabled;
            VimModeSetting::override_global(VimModeSetting(new_value), cx);
            HelixModeSetting::override_global(HelixModeSetting(false), cx);
            window.refresh();
        },
    )
    .toggleable_entry(
        "Helix Mode",
        helix_mode_enabled,
        IconPosition::Start,
        None,
        move |window, cx| {
            let new_value = !helix_mode_enabled;
            HelixModeSetting::override_global(HelixModeSetting(new_value), cx);
            VimModeSetting::override_global(VimModeSetting(false), cx);
            window.refresh();
        },
    )
}

impl QuickActionBarItem for EditorSettingsMenu {
    type Context = EditorSettingsMenuContext;

    const ID: &'static str = "toggle-editor-settings";
    const TRIGGERS: &'static [VisibilityTrigger] = &[];

    fn context(&self, target: &QuickActionTarget, _cx: &mut App) -> Option<Self::Context> {
        Some(EditorSettingsMenuContext {
            editor: target.editor()?.clone(),
            workspace: target.workspace().clone(),
        })
    }

    fn render(
        &self,
        context: &Self::Context,
        _window: &mut Window,
        _cx: &mut App,
    ) -> QuickActionElement {
        let editor = context.editor.clone();
        let workspace = context.workspace.clone();

        QuickActionElement::Dropdown {
            icon: IconName::Filter,
            menu: QuickActionMenu::new("Editor Controls", move |window, cx| {
                let state = EditorSettingsState::read(&editor, cx);
                let focus_handle = editor.focus_handle(cx);
                let fs = workspace
                    .upgrade()
                    .map(|workspace| workspace.read(cx).app_state().fs.clone());
                let editor = editor.downgrade();

                ContextMenu::build(window, cx, |menu, _, _| {
                    let menu = menu.context(focus_handle);
                    let menu = language_features_section(menu, &state, &editor).separator();
                    let menu = diagnostics_section(menu, &state, &editor);
                    let menu = display_section(menu, &state, &editor).separator();
                    let menu = git_section(menu, &state, &editor, fs).separator();
                    modal_editing_section(menu, &state)
                })
            }),
        }
    }
}
