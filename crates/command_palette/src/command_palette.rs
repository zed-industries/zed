use collections::{CommandPaletteFilter, HashMap};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, anyhow::anyhow, elements::*, keymap_matcher::Keystroke, Action, AnyWindowHandle,
    AppContext, Element, MouseState, ViewContext,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use std::cmp::{self, Reverse};
use util::ResultExt;
use workspace::Workspace;

pub fn init(cx: &mut AppContext) {
    cx.add_action(toggle_command_palette);
    CommandPalette::init(cx);
}

actions!(command_palette, [Toggle]);

pub type CommandPalette = Picker<CommandPaletteDelegate>;

pub type CommandPaletteInterceptor =
    Box<dyn Fn(&str, &AppContext) -> Option<CommandInterceptResult>>;

pub struct CommandInterceptResult {
    pub action: Box<dyn Action>,
    pub string: String,
    pub positions: Vec<usize>,
}

pub struct CommandPaletteDelegate {
    actions: Vec<Command>,
    matches: Vec<StringMatch>,
    selected_ix: usize,
    focused_view_id: usize,
}

pub enum Event {
    Dismissed,
    Confirmed {
        window: AnyWindowHandle,
        focused_view_id: usize,
        action: Box<dyn Action>,
    },
}
struct Command {
    name: String,
    action: Box<dyn Action>,
    keystrokes: Vec<Keystroke>,
}

/// Hit count for each command in the palette.
/// We only account for commands triggered directly via command palette and not by e.g. keystrokes because
/// if an user already knows a keystroke for a command, they are unlikely to use a command palette to look for it.
#[derive(Default)]
struct HitCounts(HashMap<String, usize>);

fn toggle_command_palette(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
    let focused_view_id = cx.focused_view_id().unwrap_or_else(|| cx.view_id());
    workspace.toggle_modal(cx, |_, cx| {
        cx.add_view(|cx| Picker::new(CommandPaletteDelegate::new(focused_view_id), cx))
    });
}

impl CommandPaletteDelegate {
    pub fn new(focused_view_id: usize) -> Self {
        Self {
            actions: Default::default(),
            matches: vec![],
            selected_ix: 0,
            focused_view_id,
        }
    }
}

impl PickerDelegate for CommandPaletteDelegate {
    fn placeholder_text(&self) -> std::sync::Arc<str> {
        "Execute a command...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_ix = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let view_id = self.focused_view_id;
        let window = cx.window();
        cx.spawn(move |picker, mut cx| async move {
            let mut actions = window
                .available_actions(view_id, &cx)
                .into_iter()
                .flatten()
                .filter_map(|(name, action, bindings)| {
                    let filtered = cx.read(|cx| {
                        if cx.has_global::<CommandPaletteFilter>() {
                            let filter = cx.global::<CommandPaletteFilter>();
                            filter.filtered_namespaces.contains(action.namespace())
                        } else {
                            false
                        }
                    });

                    if filtered {
                        None
                    } else {
                        Some(Command {
                            name: humanize_action_name(name),
                            action,
                            keystrokes: bindings
                                .iter()
                                .map(|binding| binding.keystrokes())
                                .last()
                                .map_or(Vec::new(), |keystrokes| keystrokes.to_vec()),
                        })
                    }
                })
                .collect::<Vec<_>>();
            let mut actions = cx.read(move |cx| {
                let hit_counts = cx.optional_global::<HitCounts>();
                actions.sort_by_key(|action| {
                    (
                        Reverse(hit_counts.and_then(|map| map.0.get(&action.name)).cloned()),
                        action.name.clone(),
                    )
                });
                actions
            });
            let candidates = actions
                .iter()
                .enumerate()
                .map(|(ix, command)| StringMatchCandidate {
                    id: ix,
                    string: command.name.to_string(),
                    char_bag: command.name.chars().collect(),
                })
                .collect::<Vec<_>>();
            let mut matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    10000,
                    &Default::default(),
                    cx.background(),
                )
                .await
            };
            let intercept_result = cx.read(|cx| {
                if cx.has_global::<CommandPaletteInterceptor>() {
                    cx.global::<CommandPaletteInterceptor>()(&query, cx)
                } else {
                    None
                }
            });
            if let Some(CommandInterceptResult {
                action,
                string,
                positions,
            }) = intercept_result
            {
                if let Some(idx) = matches
                    .iter()
                    .position(|m| actions[m.candidate_id].action.id() == action.id())
                {
                    matches.remove(idx);
                }
                actions.push(Command {
                    name: string.clone(),
                    action,
                    keystrokes: vec![],
                });
                matches.insert(
                    0,
                    StringMatch {
                        candidate_id: actions.len() - 1,
                        string,
                        positions,
                        score: 0.0,
                    },
                )
            }
            picker
                .update(&mut cx, |picker, _| {
                    let delegate = picker.delegate_mut();
                    delegate.actions = actions;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        delegate.selected_ix = 0;
                    } else {
                        delegate.selected_ix =
                            cmp::min(delegate.selected_ix, delegate.matches.len() - 1);
                    }
                })
                .log_err();
        })
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if !self.matches.is_empty() {
            let window = cx.window();
            let focused_view_id = self.focused_view_id;
            let action_ix = self.matches[self.selected_ix].candidate_id;
            let command = self.actions.remove(action_ix);
            cx.update_default_global(|hit_counts: &mut HitCounts, _| {
                *hit_counts.0.entry(command.name).or_default() += 1;
            });
            let action = command.action;

            cx.app_context()
                .spawn(move |mut cx| async move {
                    window
                        .dispatch_action(focused_view_id, action.as_ref(), &mut cx)
                        .ok_or_else(|| anyhow!("window was closed"))
                })
                .detach_and_log_err(cx);
        }
        cx.emit(PickerEvent::Dismiss);
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> AnyElement<Picker<Self>> {
        let mat = &self.matches[ix];
        let command = &self.actions[mat.candidate_id];
        let theme = theme::current(cx);
        let style = theme.picker.item.in_state(selected).style_for(mouse_state);
        let key_style = &theme.command_palette.key.in_state(selected);
        let keystroke_spacing = theme.command_palette.keystroke_spacing;

        Flex::row()
            .with_child(
                Label::new(mat.string.clone(), style.label.clone())
                    .with_highlights(mat.positions.clone()),
            )
            .with_children(command.keystrokes.iter().map(|keystroke| {
                Flex::row()
                    .with_children(
                        [
                            (keystroke.ctrl, "^"),
                            (keystroke.alt, "⌥"),
                            (keystroke.cmd, "⌘"),
                            (keystroke.shift, "⇧"),
                        ]
                        .into_iter()
                        .filter_map(|(modifier, label)| {
                            if modifier {
                                Some(
                                    Label::new(label, key_style.label.clone())
                                        .contained()
                                        .with_style(key_style.container),
                                )
                            } else {
                                None
                            }
                        }),
                    )
                    .with_child(
                        Label::new(keystroke.key.clone(), key_style.label.clone())
                            .contained()
                            .with_style(key_style.container),
                    )
                    .contained()
                    .with_margin_left(keystroke_spacing)
                    .flex_float()
            }))
            .contained()
            .with_style(style.container)
            .into_any()
    }
}

fn humanize_action_name(name: &str) -> String {
    let capacity = name.len() + name.chars().filter(|c| c.is_uppercase()).count();
    let mut result = String::with_capacity(capacity);
    for char in name.chars() {
        if char == ':' {
            if result.ends_with(':') {
                result.push(' ');
            } else {
                result.push(':');
            }
        } else if char == '_' {
            result.push(' ');
        } else if char.is_uppercase() {
            if !result.ends_with(' ') {
                result.push(' ');
            }
            result.extend(char.to_lowercase());
        } else {
            result.push(char);
        }
    }
    result
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name)
            .field("keystrokes", &self.keystrokes)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use editor::Editor;
    use gpui::{executor::Deterministic, TestAppContext};
    use project::Project;
    use workspace::{AppState, Workspace};

    #[test]
    fn test_humanize_action_name() {
        assert_eq!(
            humanize_action_name("editor::GoToDefinition"),
            "editor: go to definition"
        );
        assert_eq!(
            humanize_action_name("editor::Backspace"),
            "editor: backspace"
        );
        assert_eq!(
            humanize_action_name("go_to_line::Deploy"),
            "go to line: deploy"
        );
    }

    #[gpui::test]
    async fn test_command_palette(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let workspace = window.root(cx);
        let editor = window.add_view(cx, |cx| {
            let mut editor = Editor::single_line(None, cx);
            editor.set_text("abc", cx);
            editor
        });

        workspace.update(cx, |workspace, cx| {
            cx.focus(&editor);
            workspace.add_item(Box::new(editor.clone()), cx)
        });

        workspace.update(cx, |workspace, cx| {
            toggle_command_palette(workspace, &Toggle, cx);
        });

        let palette = workspace.read_with(cx, |workspace, _| {
            workspace.modal::<CommandPalette>().unwrap()
        });

        palette
            .update(cx, |palette, cx| {
                // Fill up palette's command list by running an empty query;
                // we only need it to subsequently assert that the palette is initially
                // sorted by command's name.
                palette.delegate_mut().update_matches("".to_string(), cx)
            })
            .await;

        palette.update(cx, |palette, _| {
            let is_sorted =
                |actions: &[Command]| actions.windows(2).all(|pair| pair[0].name <= pair[1].name);
            assert!(is_sorted(&palette.delegate().actions));
        });

        palette
            .update(cx, |palette, cx| {
                palette
                    .delegate_mut()
                    .update_matches("bcksp".to_string(), cx)
            })
            .await;

        palette.update(cx, |palette, cx| {
            assert_eq!(palette.delegate().matches[0].string, "editor: backspace");
            palette.confirm(&Default::default(), cx);
        });
        deterministic.run_until_parked();
        editor.read_with(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "ab");
        });

        // Add namespace filter, and redeploy the palette
        cx.update(|cx| {
            cx.update_default_global::<CommandPaletteFilter, _, _>(|filter, _| {
                filter.filtered_namespaces.insert("editor");
            })
        });

        workspace.update(cx, |workspace, cx| {
            toggle_command_palette(workspace, &Toggle, cx);
        });

        // Assert editor command not present
        let palette = workspace.read_with(cx, |workspace, _| {
            workspace.modal::<CommandPalette>().unwrap()
        });

        palette
            .update(cx, |palette, cx| {
                palette
                    .delegate_mut()
                    .update_matches("bcksp".to_string(), cx)
            })
            .await;

        palette.update(cx, |palette, _| {
            assert!(palette.delegate().matches.is_empty())
        });
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            theme::init((), cx);
            language::init(cx);
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            init(cx);
            Project::init_settings(cx);
            app_state
        })
    }
}
