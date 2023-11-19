use collections::{CommandPaletteFilter, HashMap};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, div, prelude::*, Action, AppContext, Component, Dismiss, Div, FocusHandle, Keystroke,
    ManagedView, ParentElement, Render, Styled, View, ViewContext, VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use std::{
    cmp::{self, Reverse},
    sync::Arc,
};
use theme::ActiveTheme;
use ui::{h_stack, v_stack, HighlightedLabel, KeyBinding, StyledExt};
use util::{
    channel::{parse_zed_link, ReleaseChannel, RELEASE_CHANNEL},
    ResultExt,
};
use workspace::Workspace;
use zed_actions::OpenZedURL;

actions!(Toggle);

pub fn init(cx: &mut AppContext) {
    cx.set_global(HitCounts::default());
    cx.observe_new_views(CommandPalette::register).detach();
}

pub struct CommandPalette {
    picker: View<Picker<CommandPaletteDelegate>>,
}

impl CommandPalette {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, _: &Toggle, cx| {
            let Some(previous_focus_handle) = cx.focused() else {
                return;
            };
            workspace.toggle_modal(cx, move |cx| CommandPalette::new(previous_focus_handle, cx));
        });
    }

    fn new(previous_focus_handle: FocusHandle, cx: &mut ViewContext<Self>) -> Self {
        let filter = cx.try_global::<CommandPaletteFilter>();

        let commands = cx
            .available_actions()
            .into_iter()
            .filter_map(|action| {
                let name = gpui::remove_the_2(action.name());
                let namespace = name.split("::").next().unwrap_or("malformed action name");
                if filter.is_some_and(|f| f.filtered_namespaces.contains(namespace)) {
                    return None;
                }

                Some(Command {
                    name: humanize_action_name(&name),
                    action,
                    keystrokes: vec![], // todo!()
                })
            })
            .collect();

        let delegate =
            CommandPaletteDelegate::new(cx.view().downgrade(), commands, previous_focus_handle);

        let picker = cx.build_view(|cx| Picker::new(delegate, cx));
        Self { picker }
    }
}

impl ManagedView for CommandPalette {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for CommandPalette {
    type Element = Div<Self>;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        v_stack().w_96().child(self.picker.clone())
    }
}

pub type CommandPaletteInterceptor =
    Box<dyn Fn(&str, &AppContext) -> Option<CommandInterceptResult>>;

pub struct CommandInterceptResult {
    pub action: Box<dyn Action>,
    pub string: String,
    pub positions: Vec<usize>,
}

pub struct CommandPaletteDelegate {
    command_palette: WeakView<CommandPalette>,
    commands: Vec<Command>,
    matches: Vec<StringMatch>,
    selected_ix: usize,
    previous_focus_handle: FocusHandle,
}

struct Command {
    name: String,
    action: Box<dyn Action>,
    keystrokes: Vec<Keystroke>,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            action: self.action.boxed_clone(),
            keystrokes: self.keystrokes.clone(),
        }
    }
}
/// Hit count for each command in the palette.
/// We only account for commands triggered directly via command palette and not by e.g. keystrokes because
/// if an user already knows a keystroke for a command, they are unlikely to use a command palette to look for it.
#[derive(Default)]
struct HitCounts(HashMap<String, usize>);

impl CommandPaletteDelegate {
    fn new(
        command_palette: WeakView<CommandPalette>,
        commands: Vec<Command>,
        previous_focus_handle: FocusHandle,
    ) -> Self {
        Self {
            command_palette,
            matches: vec![],
            commands,
            selected_ix: 0,
            previous_focus_handle,
        }
    }
}

impl PickerDelegate for CommandPaletteDelegate {
    type ListItem = Div<Picker<Self>>;

    fn placeholder_text(&self) -> Arc<str> {
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
        let mut commands = self.commands.clone();

        cx.spawn(move |picker, mut cx| async move {
            cx.read_global::<HitCounts, _>(|hit_counts, _| {
                commands.sort_by_key(|action| {
                    (
                        Reverse(hit_counts.0.get(&action.name).cloned()),
                        action.name.clone(),
                    )
                });
            })
            .ok();

            let candidates = commands
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
                    cx.background_executor().clone(),
                )
                .await
            };

            let mut intercept_result = cx
                .try_read_global(|interceptor: &CommandPaletteInterceptor, cx| {
                    (interceptor)(&query, cx)
                })
                .flatten();

            if *RELEASE_CHANNEL == ReleaseChannel::Dev {
                if parse_zed_link(&query).is_some() {
                    intercept_result = Some(CommandInterceptResult {
                        action: OpenZedURL { url: query.clone() }.boxed_clone(),
                        string: query.clone(),
                        positions: vec![],
                    })
                }
            }
            if let Some(CommandInterceptResult {
                action,
                string,
                positions,
            }) = intercept_result
            {
                if let Some(idx) = matches
                    .iter()
                    .position(|m| commands[m.candidate_id].action.type_id() == action.type_id())
                {
                    matches.remove(idx);
                }
                commands.push(Command {
                    name: string.clone(),
                    action,
                    keystrokes: vec![],
                });
                matches.insert(
                    0,
                    StringMatch {
                        candidate_id: commands.len() - 1,
                        string,
                        positions,
                        score: 0.0,
                    },
                )
            }
            picker
                .update(&mut cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.commands = commands;
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

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.command_palette
            .update(cx, |_, cx| cx.emit(Dismiss))
            .log_err();
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if self.matches.is_empty() {
            self.dismissed(cx);
            return;
        }
        let action_ix = self.matches[self.selected_ix].candidate_id;
        let command = self.commands.swap_remove(action_ix);
        cx.update_global(|hit_counts: &mut HitCounts, _| {
            *hit_counts.0.entry(command.name).or_default() += 1;
        });
        let action = command.action;
        cx.focus(&self.previous_focus_handle);
        cx.dispatch_action(action);
        self.dismissed(cx);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Self::ListItem {
        let colors = cx.theme().colors();
        let Some(r#match) = self.matches.get(ix) else {
            return div();
        };
        let Some(command) = self.commands.get(r#match.candidate_id) else {
            return div();
        };

        div()
            .px_1()
            .text_color(colors.text)
            .text_ui()
            .bg(colors.ghost_element_background)
            .rounded_md()
            .when(selected, |this| this.bg(colors.ghost_element_selected))
            .hover(|this| this.bg(colors.ghost_element_hover))
            .child(
                h_stack()
                    .justify_between()
                    .child(HighlightedLabel::new(
                        command.name.clone(),
                        r#match.positions.clone(),
                    ))
                    .children(KeyBinding::for_action(&*command.action, cx)),
            )
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
        let app_state = init_test(cx);

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

        let editor = cx.build_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_text("abc", cx);
            editor
        });

        workspace.update(cx, |workspace, cx| {
            workspace.add_item(Box::new(editor.clone()), cx);
            editor.update(cx, |editor, cx| editor.focus(cx))
        });

        cx.simulate_keystrokes("cmd-shift-p");

        let palette = workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<CommandPalette>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        });

        palette.update(cx, |palette, _| {
            assert!(palette.delegate.commands.len() > 5);
            let is_sorted =
                |actions: &[Command]| actions.windows(2).all(|pair| pair[0].name <= pair[1].name);
            assert!(is_sorted(&palette.delegate.commands));
        });

        cx.simulate_input("bcksp");

        palette.update(cx, |palette, _| {
            assert_eq!(palette.delegate.matches[0].string, "editor: backspace");
        });

        cx.simulate_keystrokes("enter");

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<CommandPalette>(cx).is_none());
            assert_eq!(editor.read(cx).text(cx), "ab")
        });

        // Add namespace filter, and redeploy the palette
        cx.update(|cx| {
            cx.set_global(CommandPaletteFilter::default());
            cx.update_global::<CommandPaletteFilter, _>(|filter, _| {
                filter.filtered_namespaces.insert("editor");
            })
        });

        cx.simulate_keystrokes("cmd-shift-p");
        cx.simulate_input("bcksp");

        let palette = workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<CommandPalette>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        });
        palette.update(cx, |palette, _| {
            assert!(palette.delegate.matches.is_empty())
        });
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            init(cx);
            Project::init_settings(cx);
            settings::load_default_keymap(cx);
            app_state
        })
    }
}
