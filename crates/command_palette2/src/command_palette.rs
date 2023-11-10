use anyhow::anyhow;
use collections::{CommandPaletteFilter, HashMap};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, div, Action, AnyElement, AnyWindowHandle, AppContext, BorrowWindow, Component, Div,
    Element, EventEmitter, FocusHandle, Keystroke, ParentElement, Render, StatelessInteractive,
    Styled, View, ViewContext, VisualContext, WeakView, WindowContext,
};
use picker::{Picker, PickerDelegate};
use std::cmp::{self, Reverse};
use theme::ActiveTheme;
use ui::{modal, Label};
use util::{
    channel::{parse_zed_link, ReleaseChannel, RELEASE_CHANNEL},
    ResultExt,
};
use workspace::{Modal, ModalEvent, Workspace};
use zed_actions::OpenZedURL;

actions!(Toggle);

pub fn init(cx: &mut AppContext) {
    cx.set_global(HitCounts::default());

    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace.modal_layer().register_modal(Toggle, |cx| {
                let Some(previous_focus_handle) = cx.focused() else {
                    return None;
                };

                Some(cx.build_view(|cx| CommandPalette::new(previous_focus_handle, cx)))
            });
        },
    )
    .detach();
}

pub struct CommandPalette {
    picker: View<Picker<CommandPaletteDelegate>>,
}

impl CommandPalette {
    fn new(previous_focus_handle: FocusHandle, cx: &mut ViewContext<Self>) -> Self {
        let filter = cx.try_global::<CommandPaletteFilter>();

        let commands = cx
            .available_actions()
            .into_iter()
            .filter_map(|action| {
                let name = action.name();
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
            CommandPaletteDelegate::new(cx.view().downgrade(), commands, previous_focus_handle, cx);

        let picker = cx.build_view(|cx| {
            let picker = Picker::new(delegate, cx);
            picker.focus(cx);
            picker
        });
        Self { picker }
    }
}

impl EventEmitter<ModalEvent> for CommandPalette {}
impl Modal for CommandPalette {
    fn focus(&self, cx: &mut WindowContext) {
        self.picker.update(cx, |picker, cx| picker.focus(cx));
    }
}

impl Render for CommandPalette {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        modal(cx).w_96().child(self.picker.clone())
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
        cx: &ViewContext<CommandPalette>,
    ) -> Self {
        Self {
            command_palette,
            commands,
            matches: vec![StringMatch {
                candidate_id: 0,
                score: 0.,
                positions: vec![],
                string: "Foo my bar".into(),
            }],
            selected_ix: 0,
            previous_focus_handle,
        }
    }
}

impl PickerDelegate for CommandPaletteDelegate {
    type ListItem = Div<Picker<Self>>;

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
        cx.focus(&self.previous_focus_handle);
        self.command_palette
            .update(cx, |_, cx| cx.emit(ModalEvent::Dismissed))
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
        let Some(command) = self
            .matches
            .get(ix)
            .and_then(|m| self.commands.get(m.candidate_id))
        else {
            return div();
        };

        div()
            .text_color(colors.text)
            .when(selected, |s| {
                s.border_l_10().border_color(colors.terminal_ansi_yellow)
            })
            .hover(|style| {
                style
                    .bg(colors.element_active)
                    .text_color(colors.text_accent)
            })
            .child(Label::new(command.name.clone()))
    }

    // fn render_match(
    //     &self,
    //     ix: usize,
    //     mouse_state: &mut MouseState,
    //     selected: bool,
    //     cx: &gpui::AppContext,
    // ) -> AnyElement<Picker<Self>> {
    //     let mat = &self.matches[ix];
    //     let command = &self.actions[mat.candidate_id];
    //     let theme = theme::current(cx);
    //     let style = theme.picker.item.in_state(selected).style_for(mouse_state);
    //     let key_style = &theme.command_palette.key.in_state(selected);
    //     let keystroke_spacing = theme.command_palette.keystroke_spacing;

    //     Flex::row()
    //         .with_child(
    //             Label::new(mat.string.clone(), style.label.clone())
    //                 .with_highlights(mat.positions.clone()),
    //         )
    //         .with_children(command.keystrokes.iter().map(|keystroke| {
    //             Flex::row()
    //                 .with_children(
    //                     [
    //                         (keystroke.ctrl, "^"),
    //                         (keystroke.alt, "⌥"),
    //                         (keystroke.cmd, "⌘"),
    //                         (keystroke.shift, "⇧"),
    //                     ]
    //                     .into_iter()
    //                     .filter_map(|(modifier, label)| {
    //                         if modifier {
    //                             Some(
    //                                 Label::new(label, key_style.label.clone())
    //                                     .contained()
    //                                     .with_style(key_style.container),
    //                             )
    //                         } else {
    //                             None
    //                         }
    //                     }),
    //                 )
    //                 .with_child(
    //                     Label::new(keystroke.key.clone(), key_style.label.clone())
    //                         .contained()
    //                         .with_style(key_style.container),
    //                 )
    //                 .contained()
    //                 .with_margin_left(keystroke_spacing)
    //                 .flex_float()
    //         }))
    //         .contained()
    //         .with_style(style.container)
    //         .into_any()
    // }
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
            theme::init(cx);
            language::init(cx);
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            init(cx);
            Project::init_settings(cx);
            app_state
        })
    }
}
