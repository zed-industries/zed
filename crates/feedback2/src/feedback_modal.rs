use editor::Editor;
use gpui::{
    div, rems, AppContext, DismissEvent, Div, EventEmitter, FocusHandle, FocusableView, Render,
    ViewContext,
};
use ui::{prelude::*, Button, ButtonStyle, Label, Tooltip};
use workspace::Workspace;

use crate::feedback_editor::GiveFeedback;

pub struct FeedbackModal {
    editor: View<Editor>,
    tmp_focus_handle: FocusHandle, // TODO: should be editor.focus_handle(cx)
}

impl FocusableView for FeedbackModal {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.tmp_focus_handle.clone()
    }
}
impl EventEmitter<DismissEvent> for FeedbackModal {}

impl FeedbackModal {
    pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let _handle = cx.view().downgrade();
        workspace.register_action(move |workspace, _: &GiveFeedback, cx| {
            workspace.toggle_modal(cx, move |cx| FeedbackModal::new(cx));
        });
    }

    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let line_editor = cx.build_view(|cx| Editor::single_line(cx));
        let line_editor_change = cx.subscribe(&line_editor, Self::on_line_editor_event);

        // let editor = active_editor.read(cx);
        // let cursor = editor.selections.last::<Point>(cx).head();
        // let last_line = editor.buffer().read(cx).snapshot(cx).max_point().row;
        // let scroll_position = active_editor.update(cx, |editor, cx| editor.scroll_position(cx));

        // let current_text = format!(
        //     "line {} of {} (column {})",
        //     cursor.row + 1,
        //     last_line + 1,
        //     cursor.column + 1,
        // );
        Self {
            editor: line_editor,
            tmp_focus_handle: cx.focus_handle(),
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
        let dismiss = cx.listener(|_, _, cx| cx.emit(DismissEvent));

        v_stack()
            .elevation_3(cx)
            .min_w(rems(40.))
            .max_w(rems(96.))
            .h(rems(40.))
            .p_2()
            .gap_2()
            .child(h_stack().child(Label::new("Give Feedback").color(Color::Default)))
            .child(
                div()
                    .flex_1()
                    .bg(cx.theme().colors().editor_background)
                    .border()
                    .border_color(cx.theme().colors().border)
                    .child("editor"),
            )
            .child(
                h_stack()
                    .justify_end()
                    .gap_1()
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
                            }),
                    ),
            )

        // Header
        // - has some info, maybe some links
        // Body
        // - Markdown Editor
        // - Email address
        // Footer
        // - CTA buttons (Send, Cancel)

        // div()
        //     .elevation_2(cx)
        //     .key_context(
        //         "FeedbackModal
        //         ",
        //     )
        //     .on_action(cx.listener(Self::cancel))
        //     .on_action(cx.listener(Self::confirm))
        //     .w_96()
        //     .child(
        //         v_stack()
        //             .px_1()
        //             .pt_0p5()
        //             .gap_px()
        //             .child(
        //                 v_stack()
        //                     .py_0p5()
        //                     .px_1()
        //                     .child(div().px_1().py_0p5().child(self.line_editor.clone())),
        //             )
        //             .child(
        //                 div()
        //                     .h_px()
        //                     .w_full()
        //                     .bg(cx.theme().colors().element_background),
        //             )
        //             .child(
        //                 h_stack()
        //                     .justify_between()
        //                     .px_2()
        //                     .py_1()
        //                     .child(Label::new(self.current_text.clone()).color(Color::Muted)),
        //             ),
        //     )
    }
}
