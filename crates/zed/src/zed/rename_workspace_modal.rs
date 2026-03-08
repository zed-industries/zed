use editor::Editor;
use gpui::{AppContext as _, DismissEvent, Entity, EventEmitter, Focusable, Styled};
use ui::{
    App, Context, HeadlineSize, InteractiveElement, IntoElement, ParentElement, Render, StyledExt,
    Window, div, h_flex, rems, v_flex,
};
use workspace::ModalView;

pub struct RenameWorkspaceModal {
    editor: Entity<Editor>,
    workspace: gpui::WeakEntity<workspace::Workspace>,
}

impl EventEmitter<DismissEvent> for RenameWorkspaceModal {}
impl ModalView for RenameWorkspaceModal {}

impl Focusable for RenameWorkspaceModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl RenameWorkspaceModal {
    pub fn new(
        current_name: String,
        workspace: gpui::WeakEntity<workspace::Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if !current_name.is_empty() {
                editor.set_text(current_name, window, cx);
                editor.select_all(&editor::actions::SelectAll, window, cx);
            }
            editor.set_placeholder_text("Workspace name…", window, cx);
            editor
        });
        Self { editor, workspace }
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let new_name = self.editor.read(cx).text(cx);
        let name = if new_name.trim().is_empty() {
            None
        } else {
            Some(new_name)
        };
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.set_custom_name(name, window, cx);
            })
            .ok();
        cx.emit(DismissEvent);
    }
}

impl Render for RenameWorkspaceModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RenameWorkspaceModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .w(rems(34.))
            .child(
                h_flex()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .w_full()
                    .gap_1p5()
                    .child(
                        ui::Headline::new("Rename Workspace")
                            .size(HeadlineSize::XSmall),
                    ),
            )
            .child(
                div()
                    .px_3()
                    .pb_3()
                    .w_full()
                    .child(self.editor.clone()),
            )
    }
}
