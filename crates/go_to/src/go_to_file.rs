use editor::Editor;
use gpui::{
    actions, div, prelude::*, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    Model, Render, Subscription, View, ViewContext,
};
use project::{FileNumber, Project};
use ui::{h_flex, rems, v_flex, ActiveTheme, Color, Label, LabelCommon, SharedString, StyledExt};
use workspace::{ModalView, Workspace};

actions!(go_to_file, [Toggle]);

pub struct GoToFile {
    number_editor: View<Editor>,
    project: Model<Project>,
    current_text: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl ModalView for GoToFile {}

impl FocusableView for GoToFile {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.number_editor.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for GoToFile {}

impl GoToFile {
    pub(crate) fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(move |workspace, _: &Toggle, cx| {
            let project = workspace.project().clone();
            workspace.toggle_modal(cx, |cx| Self::new(project, cx));
        });
    }

    fn new(project: Model<Project>, cx: &mut ViewContext<Self>) -> Self {
        let number_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Enter file number...", cx);
            editor
        });

        let current_text = format!("Total files: {}", 10).into();
        let number_editor_change = cx.subscribe(&number_editor, Self::on_number_editor_event);

        Self {
            number_editor,
            project,
            current_text,
            _subscriptions: vec![number_editor_change],
        }
    }

    fn on_number_editor_event(
        &mut self,
        _: View<Editor>,
        event: &editor::EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::EditorEvent::Blurred => cx.emit(DismissEvent),
            editor::EditorEvent::InputHandled {
                utf16_range_to_replace: _,
                text: _,
            } => {
                let input = self.number_editor.read(cx).text(cx);
                if let Some(last_char) = input.trim().chars().last() {
                    if last_char == 'j' || last_char == 'k' {
                        self.confirm(&menu::Confirm, cx);
                    }
                }
            }
            _ => {}
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if let Some(file_number) = self.file_number_from_input(cx) {
            self.project.update(cx, |_, cx| {
                cx.emit(project::Event::OpenNumberedFile(file_number))
            });
        };
        cx.emit(DismissEvent);
    }

    fn file_number_from_input(&self, cx: &mut ViewContext<Self>) -> Option<FileNumber> {
        let input = self.number_editor.read(cx).text(cx);
        let mut trimmed = input.trim().to_string();

        if trimmed.is_empty() {
            return None;
        }

        let last_char = trimmed.pop()?;
        Some(match last_char {
            'j' => {
                let num = trimmed.parse().unwrap_or(1);
                FileNumber::Relative(num, true)
            }
            'k' => {
                let num = trimmed.parse().unwrap_or(1);
                FileNumber::Relative(num, false)
            }
            _ => input
                .trim()
                .parse::<isize>()
                .ok()?
                .abs()
                .try_into()
                .ok()
                .map(FileNumber::Absolute)?,
        })
    }
}

impl Render for GoToFile {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(24.))
            .elevation_2(cx)
            .key_context("GoToFile")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .child(
                div()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .px_2()
                    .py_1()
                    .child(self.number_editor.clone()),
            )
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .gap_1()
                    .child(Label::new("File:").color(Color::Muted))
                    .child(Label::new(self.current_text.clone()).color(Color::Muted)),
            )
    }
}
