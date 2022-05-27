use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    elements::{ChildView, Flex, Label, ParentElement},
    keymap::Keystroke,
    Action, Element, Entity, MouseState, MutableAppContext, View, ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate};
use settings::Settings;
use std::cmp;
use workspace::Workspace;

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(CommandPalette::toggle);
    Picker::<CommandPalette>::init(cx);
}

actions!(command_palette, [Toggle]);

pub struct CommandPalette {
    picker: ViewHandle<Picker<Self>>,
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

impl CommandPalette {
    pub fn new(focused_view_id: usize, cx: &mut ViewContext<Self>) -> Self {
        let this = cx.weak_handle();
        let actions = cx
            .available_actions(cx.window_id(), focused_view_id)
            .map(|(name, action, bindings)| Command {
                name: humanize_action_name(name),
                action,
                keystrokes: bindings
                    .last()
                    .map_or(Vec::new(), |binding| binding.keystrokes().to_vec()),
            })
            .collect();
        let picker = cx.add_view(|cx| Picker::new(this, cx));
        Self {
            picker,
            actions,
            matches: vec![],
            selected_ix: 0,
            focused_view_id,
        }
    }

    fn toggle(_: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        let workspace = cx.handle();
        let window_id = cx.window_id();
        let focused_view_id = cx.focused_view_id(window_id).unwrap_or(workspace.id());

        cx.as_mut().defer(move |cx| {
            let this = cx.add_view(window_id, |cx| Self::new(focused_view_id, cx));
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(cx, |_, cx| {
                    cx.subscribe(&this, Self::on_event).detach();
                    this
                });
            });
        });
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<Self>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => workspace.dismiss_modal(cx),
            Event::Confirmed {
                window_id,
                focused_view_id,
                action,
            } => {
                let window_id = *window_id;
                let focused_view_id = *focused_view_id;
                let action = (*action).boxed_clone();
                workspace.dismiss_modal(cx);
                cx.as_mut()
                    .defer(move |cx| cx.dispatch_action_at(window_id, focused_view_id, &*action))
            }
        }
    }
}

impl Entity for CommandPalette {
    type Event = Event;
}

impl View for CommandPalette {
    fn ui_name() -> &'static str {
        "CommandPalette"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        ChildView::new(self.picker.clone()).boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.picker);
    }
}

impl PickerDelegate for CommandPalette {
    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Self>) {
        self.selected_ix = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut gpui::ViewContext<Self>,
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
        cx.spawn(move |this, mut cx| async move {
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
            this.update(&mut cx, |this, _| {
                this.matches = matches;
                if this.matches.is_empty() {
                    this.selected_ix = 0;
                } else {
                    this.selected_ix = cmp::min(this.selected_ix, this.matches.len() - 1);
                }
            });
        })
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        if !self.matches.is_empty() {
            let action_ix = self.matches[self.selected_ix].candidate_id;
            cx.emit(Event::Confirmed {
                window_id: cx.window_id(),
                focused_view_id: self.focused_view_id,
                action: self.actions.remove(action_ix).action,
            });
        } else {
            cx.emit(Event::Dismissed);
        }
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> gpui::ElementBox {
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
                    .with_highlights(mat.positions.clone())
                    .boxed(),
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
                                    Label::new(label.into(), key_style.label.clone())
                                        .contained()
                                        .with_style(key_style.container)
                                        .boxed(),
                                )
                            } else {
                                None
                            }
                        }),
                    )
                    .with_child(
                        Label::new(keystroke.key.clone(), key_style.label.clone())
                            .contained()
                            .with_style(key_style.container)
                            .boxed(),
                    )
                    .contained()
                    .with_margin_left(keystroke_spacing)
                    .flex_float()
                    .boxed()
            }))
            .contained()
            .with_style(style.container)
            .boxed()
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project, cx));
        let editor = cx.add_view(window_id, |cx| {
            let mut editor = Editor::single_line(None, cx);
            editor.set_text("abc", cx);
            editor
        });

        workspace.update(cx, |workspace, cx| {
            cx.focus(editor.clone());
            workspace.add_item(Box::new(editor.clone()), cx)
        });

        workspace.update(cx, |workspace, cx| {
            CommandPalette::toggle(workspace, &Toggle, cx)
        });

        let palette = workspace.read_with(cx, |workspace, _| {
            workspace
                .modal()
                .unwrap()
                .clone()
                .downcast::<CommandPalette>()
                .unwrap()
        });

        palette
            .update(cx, |palette, cx| {
                palette.update_matches("bcksp".to_string(), cx)
            })
            .await;

        palette.update(cx, |palette, cx| {
            assert_eq!(palette.matches[0].string, "editor: backspace");
            palette.confirm(cx);
        });

        editor.read_with(cx, |editor, cx| {
            assert_eq!(editor.text(cx), "ab");
        });
    }
}
