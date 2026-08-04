use editor::Editor;
use futures::channel::oneshot;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render,
    SharedString, Task, WeakEntity, Window,
};
use menu::{Cancel, Confirm};
use ui::prelude::*;
use workspace::{ModalView, Workspace};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GitRefModalResult {
    Branch {
        name: String,
    },
    Tag {
        name: String,
        message: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GitRefModalKind {
    Branch,
    Tag,
}

pub(crate) struct GitRefModal {
    kind: GitRefModalKind,
    editor: Entity<Editor>,
    message_editor: Option<Entity<Editor>>,
    error: Option<SharedString>,
    result: Option<oneshot::Sender<GitRefModalResult>>,
}

impl GitRefModal {
    pub(crate) fn open(
        kind: GitRefModalKind,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Option<GitRefModalResult>> {
        let (sender, receiver) = oneshot::channel();
        window.spawn(cx, async move |cx| {
            let _ = workspace.update_in(cx, |workspace, window, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    GitRefModal::new(kind, sender, window, cx)
                })
            });
            receiver.await.ok()
        })
    }

    fn new(
        kind: GitRefModalKind,
        result: oneshot::Sender<GitRefModalResult>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(
                match kind {
                    GitRefModalKind::Branch => "Branch name",
                    GitRefModalKind::Tag => "Tag name",
                },
                window,
                cx,
            );
            editor
        });
        let message_editor = (kind == GitRefModalKind::Tag).then(|| {
            cx.new(|cx| {
                let mut editor = Editor::auto_height(1, 4, window, cx);
                editor.set_placeholder_text("Tag message (optional)", window, cx);
                editor
            })
        });

        Self {
            kind,
            editor,
            message_editor,
            error: None,
            result: Some(result),
        }
    }

    fn cancel(&mut self, _: &Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        self.result.take();
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let name = self.editor.read(cx).text(cx);
        if let Err(error) = validate_ref_name(&name) {
            self.error = Some((*error).into());
            cx.notify();
            return;
        }

        let result = match self.kind {
            GitRefModalKind::Branch => GitRefModalResult::Branch { name },
            GitRefModalKind::Tag => {
                let message = self
                    .message_editor
                    .as_ref()
                    .map(|editor| editor.read(cx).text(cx))
                    .filter(|message| !message.trim().is_empty());
                GitRefModalResult::Tag { name, message }
            }
        };
        self.result.take().map(|sender| sender.send(result));
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for GitRefModal {}
impl ModalView for GitRefModal {}

impl Focusable for GitRefModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for GitRefModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = match self.kind {
            GitRefModalKind::Branch => "Create Branch",
            GitRefModalKind::Tag => "Create Tag",
        };

        v_flex()
            .key_context("GitRefModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(Headline::new(title).size(HeadlineSize::XSmall)),
            )
            .child(div().px_3().pb_2().child(self.editor.clone()))
            .when_some(self.message_editor.clone(), |this, editor| {
                this.child(div().px_3().pb_2().child(editor))
            })
            .when_some(self.error.clone(), |this, error| {
                this.child(
                    div()
                        .px_3()
                        .pb_2()
                        .child(Label::new(error).color(Color::Error)),
                )
            })
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .px_3()
                    .pb_3()
                    .child(
                        Button::new("cancel", "Cancel")
                            .style(ButtonStyle::OutlinedGhost)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.cancel(&Cancel, window, cx);
                            })),
                    )
                    .child(
                        Button::new("confirm", "Create")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&Confirm, window, cx);
                            })),
                    ),
            )
    }
}

pub(crate) fn validate_ref_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("A ref name is required");
    }
    if name.trim() != name {
        return Err("Ref names cannot start or end with whitespace");
    }
    if name == "@" || name.starts_with('-') {
        return Err("This is not a valid Git ref name");
    }
    if name.ends_with('/') || name.ends_with('.') || name.ends_with(".lock") {
        return Err("Ref names cannot end with '/', '.', or '.lock'");
    }
    if name.contains("//") || name.contains("..") || name.contains("@{") {
        return Err("Ref name contains an invalid sequence");
    }
    if name
        .chars()
        .any(|ch| ch.is_control() || " ~^:?*[\\\u{7f}".contains(ch))
    {
        return Err("Ref name contains an invalid character");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ref_names_reject_empty_and_git_syntax_delimiters() {
        for name in [
            "",
            "   ",
            "feature..name",
            "feature~1",
            "feature:name",
            "feature name",
        ] {
            assert!(
                validate_ref_name(name).is_err(),
                "{name:?} should be rejected"
            );
        }
    }

    #[test]
    fn ref_names_accept_normal_branch_and_tag_names() {
        assert!(validate_ref_name("feature/native-git-graph").is_ok());
        assert!(validate_ref_name("v1.2.3").is_ok());
    }

    #[test]
    fn tag_modal_result_keeps_optional_message() {
        let result = GitRefModalResult::Tag {
            name: "v1.2.3".to_string(),
            message: Some("release".to_string()),
        };

        assert_eq!(
            result,
            GitRefModalResult::Tag {
                name: "v1.2.3".to_string(),
                message: Some("release".to_string()),
            }
        );
    }
}
