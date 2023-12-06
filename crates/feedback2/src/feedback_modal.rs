use std::ops::RangeInclusive;

use editor::{Editor, EditorEvent};
use gpui::{
    div, red, rems, AppContext, DismissEvent, Div, EventEmitter, FocusHandle, FocusableView, Model,
    Render, View, ViewContext,
};
use language::Buffer;
use project::Project;
use ui::{prelude::*, Button, ButtonStyle, Label, Tooltip};
use util::ResultExt;
use workspace::{item::Item, Workspace};

use crate::{feedback_editor::GiveFeedback, system_specs::SystemSpecs, OpenZedCommunityRepo};

const FEEDBACK_CHAR_LIMIT: RangeInclusive<usize> = 10..=5000;
const FEEDBACK_SUBMISSION_ERROR_TEXT: &str =
    "Feedback failed to submit, see error log for details.";

pub struct FeedbackModal {
    system_specs: SystemSpecs,
    feedback_editor: View<Editor>,
    email_address_editor: View<Editor>,
    project: Model<Project>,
    pub allow_submission: bool,
    character_count: usize,
}

impl FocusableView for FeedbackModal {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.feedback_editor.focus_handle(cx)
    }
}
impl EventEmitter<DismissEvent> for FeedbackModal {}

impl FeedbackModal {
    pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let _handle = cx.view().downgrade();
        workspace.register_action(move |workspace, _: &GiveFeedback, cx| {
            let markdown = workspace
                .app_state()
                .languages
                .language_for_name("Markdown");

            let project = workspace.project().clone();

            cx.spawn(|workspace, mut cx| async move {
                let markdown = markdown.await.log_err();
                let buffer = project
                    .update(&mut cx, |project, cx| {
                        project.create_buffer("", markdown, cx)
                    })?
                    .expect("creating buffers on a local workspace always succeeds");

                workspace.update(&mut cx, |workspace, cx| {
                    let system_specs = SystemSpecs::new(cx);

                    workspace.toggle_modal(cx, move |cx| {
                        FeedbackModal::new(system_specs, project, buffer, cx)
                    });
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        });
    }

    pub fn new(
        system_specs: SystemSpecs,
        project: Model<Project>,
        buffer: Model<Buffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let email_address_editor = cx.build_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Email address (optional)", cx);
            editor
        });
        let feedback_editor = cx.build_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, Some(project.clone()), cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor
        });

        cx.subscribe(
            &feedback_editor,
            |this, editor, event: &EditorEvent, cx| match event {
                EditorEvent::Edited => {
                    this.character_count = editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .expect("Feedback editor is never a multi-buffer")
                        .read(cx)
                        .len();
                    cx.notify();
                }
                _ => {}
            },
        )
        .detach();

        Self {
            system_specs: system_specs.clone(),
            feedback_editor,
            email_address_editor,
            project,
            allow_submission: true,
            character_count: 0,
        }
    }

    // fn release(&mut self, cx: &mut WindowContext) {
    //     let scroll_position = self.prev_scroll_position.take();
    //     self.active_editor.update(cx, |editor, cx| {
    //         editor.highlight_rows(None);
    //         if let Some(scroll_position) = scroll_position {
    //             editor.set_scroll_position(scroll_position, cx);
    //         }
    //         cx.notify();
    //     })
    // }

    // fn on_feedback_editor_event(
    //     &mut self,
    //     _: View<Editor>,
    //     event: &editor::EditorEvent,
    //     cx: &mut ViewContext<Self>,
    // ) {
    //     match event {
    //         // todo!() this isn't working...
    //         editor::EditorEvent::Blurred => cx.emit(DismissEvent),
    //         editor::EditorEvent::BufferEdited { .. } => self.highlight_current_line(cx),
    //         _ => {}
    //     }
    // }

    // fn highlight_current_line(&mut self, cx: &mut ViewContext<Self>) {
    //     if let Some(point) = self.point_from_query(cx) {
    //         self.active_editor.update(cx, |active_editor, cx| {
    //             let snapshot = active_editor.snapshot(cx).display_snapshot;
    //             let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
    //             let display_point = point.to_display_point(&snapshot);
    //             let row = display_point.row();
    //             active_editor.highlight_rows(Some(row..row + 1));
    //             active_editor.request_autoscroll(Autoscroll::center(), cx);
    //         });
    //         cx.notify();
    //     }
    // }

    // fn point_from_query(&self, cx: &ViewContext<Self>) -> Option<Point> {
    //     let line_editor = self.line_editor.read(cx).text(cx);
    //     let mut components = line_editor
    //         .splitn(2, FILE_ROW_COLUMN_DELIMITER)
    //         .map(str::trim)
    //         .fuse();
    //     let row = components.next().and_then(|row| row.parse::<u32>().ok())?;
    //     let column = components.next().and_then(|col| col.parse::<u32>().ok());
    //     Some(Point::new(
    //         row.saturating_sub(1),
    //         column.unwrap_or(0).saturating_sub(1),
    //     ))
    // }

    // fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
    //     cx.emit(DismissEvent);
    // }

    // fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
    //     if let Some(point) = self.point_from_query(cx) {
    //         self.active_editor.update(cx, |editor, cx| {
    //             let snapshot = editor.snapshot(cx).display_snapshot;
    //             let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
    //             editor.change_selections(Some(Autoscroll::center()), cx, |s| {
    //                 s.select_ranges([point..point])
    //             });
    //             editor.focus(cx);
    //             cx.notify();
    //         });
    //         self.prev_scroll_position.take();
    //     }

    //     cx.emit(DismissEvent);
    // }
}

impl Render for FeedbackModal {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let character_count_error = (self.character_count < *FEEDBACK_CHAR_LIMIT.start())
            || (self.character_count > *FEEDBACK_CHAR_LIMIT.end());

        let dismiss = cx.listener(|_, _, cx| cx.emit(DismissEvent));
        // let open_community_issues =
        //     cx.listener(|_, _, cx| cx.dispatch_action(Box::new(OpenZedCommunityRepo)));
        // let open_community_discussions = cx.listener(|_, _, cx| cx.emit(DismissEvent));

        v_stack()
            .elevation_3(cx)
            .min_w(rems(40.))
            .max_w(rems(96.))
            .h(rems(40.))
            .p_2()
            .gap_2()
            .child(
                v_stack().child(
                    div()
                        .size_full()
                        .border()
                        .border_color(red())
                        .child(Label::new("Give Feedback").color(Color::Default))
                        .child(Label::new("This editor supports markdown").color(Color::Muted)),
                ),
            )
            .child(
                div()
                    .flex_1()
                    .bg(cx.theme().colors().editor_background)
                    .border()
                    .border_color(cx.theme().colors().border)
                    .child(self.feedback_editor.clone()),
            )
            .child(
                div().border().border_color(red()).child(
                    Label::new(format!(
                        "{} / {} Characters",
                        self.character_count,
                        FEEDBACK_CHAR_LIMIT.end()
                    ))
                    .color(Color::Default),
                ),
            )
            .child(                div()
                .bg(cx.theme().colors().editor_background)
                .border()
                .border_color(cx.theme().colors().border)
                .child(self.email_address_editor.clone())
            )
            .child(
                h_stack()
                    .justify_between()
                    .gap_1()
                    .child(Button::new("community_repo", "Community Repo")
                        .style(ButtonStyle::Filled)
                        .color(Color::Muted)
                        // .on_click(cx.dispatch_action(Box::new(OpenZedCommunityRepo)))
                    )
                    .child(h_stack().justify_between().gap_1()
                        .child(
                            Button::new("cancel_feedback", "Cancel")
                                .style(ButtonStyle::Subtle)
                                .color(Color::Muted)
                                .on_click(dismiss),
                        )
                        .child(
                            Button::new("send_feedback", "Send Feedback")
                                .color(Color::Accent)
                                .style(ButtonStyle::Filled)
                                .tooltip(|cx| {
                                    Tooltip::with_meta(
                                        "Submit feedback to the Zed team.",
                                        None,
                                        "Provide an email address if you want us to be able to reply.",
                                        cx,
                                    )
                                })
                                .when(character_count_error, |this| this.disabled(true)),
                        ),
                    )

            )
    }
}
