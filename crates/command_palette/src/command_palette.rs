use collections::CommandPaletteFilter;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, elements::*, keymap_matcher::Keystroke, Action, AppContext, Drawable, MouseState,
    ViewContext,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use settings::Settings;
use std::cmp;
use util::ResultExt;
use workspace::Workspace;

pub fn init(cx: &mut AppContext) {
    cx.add_action(toggle_command_palette);
    CommandPalette::init(cx);
}

actions!(command_palette, [Toggle]);

pub type CommandPalette = Picker<CommandPaletteDelegate>;

pub struct CommandPaletteDelegate {
    actions: Vec<Command>,
    matches: Vec<StringMatch>,
    selected_ix: usize,
    focused_view_id: usize,
}

pub enum Event {
    Dismissed,
    Confirmed {
        window_id: usize,
        focused_view_id: usize,
        action: Box<dyn Action>,
    },
}

struct Command {
    name: String,
    action: Box<dyn Action>,
    keystrokes: Vec<Keystroke>,
}

fn toggle_command_palette(_: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
    let workspace = cx.handle();
    let focused_view_id = cx.focused_view_id().unwrap_or_else(|| workspace.id());

    cx.defer(move |workspace, cx| {
        workspace.toggle_modal(cx, |_, cx| {
            cx.add_view(|cx| Picker::new(CommandPaletteDelegate::new(focused_view_id, cx), cx))
        });
    });
}

impl CommandPaletteDelegate {
    pub fn new(focused_view_id: usize, cx: &mut ViewContext<Picker<Self>>) -> Self {
        let actions = cx
            .available_actions(focused_view_id)
            .filter_map(|(name, action, bindings)| {
                if cx.has_global::<CommandPaletteFilter>() {
                    let filter = cx.global::<CommandPaletteFilter>();
                    if filter.filtered_namespaces.contains(action.namespace()) {
                        return None;
                    }
                }

                Some(Command {
                    name: humanize_action_name(name),
                    action,
                    keystrokes: bindings
                        .iter()
                        .map(|binding| binding.keystrokes())
                        .last()
                        .map_or(Vec::new(), |keystrokes| keystrokes.to_vec()),
                })
            })
            .collect();

        Self {
            actions,
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
        let candidates = self
            .actions
            .iter()
            .enumerate()
            .map(|(ix, command)| StringMatchCandidate {
                id: ix,
                string: command.name.to_string(),
                char_bag: command.name.chars().collect(),
            })
            .collect::<Vec<_>>();
        cx.spawn(move |picker, mut cx| async move {
            let matches = if query.is_empty() {
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
            picker
                .update(&mut cx, |picker, _| {
                    let delegate = picker.delegate_mut();
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

    fn confirm(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        if !self.matches.is_empty() {
            let window_id = cx.window_id();
            let focused_view_id = self.focused_view_id;
            let action_ix = self.matches[self.selected_ix].candidate_id;
            let action = self.actions.remove(action_ix).action;
            cx.defer(move |_, cx| {
                cx.dispatch_any_action_at(window_id, focused_view_id, action);
            });
        }
        cx.emit(PickerEvent::Dismiss);
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> Element<Picker<Self>> {
        let mat = &self.matches[ix];
        let command = &self.actions[mat.candidate_id];
        let settings = cx.global::<Settings>();
        let theme = &settings.theme;
        let style = theme.picker.item.style_for(mouse_state, selected);
        let key_style = &theme.command_palette.key.style_for(mouse_state, selected);
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
                            (keystroke.alt, "⎇"),
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
            .into_element()
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
    use super::*;
    use editor::Editor;
    use gpui::TestAppContext;
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
    async fn test_command_palette(cx: &mut TestAppContext) {
        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            init(cx);
        });

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let editor = cx.add_view(&workspace, |cx| {
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
                palette
                    .delegate_mut()
                    .update_matches("bcksp".to_string(), cx)
            })
            .await;

        palette.update(cx, |palette, cx| {
            assert_eq!(palette.delegate().matches[0].string, "editor: backspace");
            palette.confirm(&Default::default(), cx);
        });

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
}
