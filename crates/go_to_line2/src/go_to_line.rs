use editor::Editor;
use gpui::{
    actions, div, AppContext, Div, EventEmitter, ParentElement, Render, SharedString,
    StatefulInteractivity, StatelessInteractive, Styled, View, ViewContext, VisualContext,
};
use text::Point;
use theme::ActiveTheme;
use ui::{h_stack, modal, v_stack, Label, LabelColor};
use util::paths::FILE_ROW_COLUMN_DELIMITER;
use workspace::{Modal, ModalEvent, Workspace};

actions!(Toggle);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace
                .modal_layer()
                .register_modal(Toggle, |workspace, cx| {
                    let editor = workspace
                        .active_item(cx)
                        .and_then(|active_item| active_item.downcast::<Editor>())?;

                    Some(cx.build_view(|cx| GoToLine::new(editor, cx)))
                });
            dbg!("hey!");
        },
    )
    .detach();

    // // cx.window_global()
    // // cx.window_global::<Workspace>
    // // cx.window_global::<ActiveEditor>()
    // Workspace::on_init(|workspace, cx| {
    //     workspace.on_open_item()
    // });

    // Editor::on_init(|editor, cx|{

    // })

    // Editor::register_action(|_editor, _: &Toggle, cx| {
    //     dbg!("HEY!");
    //     // let editor = cx.view();
    //     // cx.update_window(cx.window().handle(), |cx, view| {
    //     //     let workspace = view.downcast::<Workspace>();
    //     // })
    //     // workspace.show_modal(cx.build_view(|cx| GoToLine::new(editor, cx)))
    // })
    // cx.global_mut::<ModalRegistry>()
    //     .register_modal(Toggle, |workspace, cx| {
    //     });
}

pub struct GoToLine {
    line_editor: View<Editor>,
    #[allow(unused)] // todo!()
    active_editor: View<Editor>,
}

pub enum Event {
    Dismissed,
}

impl EventEmitter for GoToLine {
    type Event = Event;
}

impl GoToLine {
    pub fn new(active_editor: View<Editor>, cx: &mut ViewContext<Self>) -> Self {
        let line_editor = cx.build_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Find something", cx);
            editor.focus(cx);
            editor
        });
        cx.subscribe(&line_editor, Self::on_line_editor_event)
            .detach();

        Self {
            line_editor,
            active_editor,
        }
    }

    fn on_line_editor_event(
        &mut self,
        _: View<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Blurred => cx.emit(Event::Dismissed),
            editor::Event::BufferEdited { .. } => {
                //  if let Some(point) = self.point_from_query(cx) {
                // todo!()
                // self.active_editor.update(cx, |active_editor, cx| {
                //     let snapshot = active_editor.snapshot(cx).display_snapshot;
                //     let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
                //     let display_point = point.to_display_point(&snapshot);
                //     let row = display_point.row();
                //     active_editor.highlight_rows(Some(row..row + 1));
                //     active_editor.request_autoscroll(Autoscroll::center(), cx);
                // });
                //       cx.notify();
                //  }
            }
            _ => {}
        }
    }

    #[allow(unused)]
    fn point_from_query(&self, cx: &ViewContext<Self>) -> Option<Point> {
        // todo!()
        let line_editor = self.line_editor.read(cx).text(cx);
        let mut components = line_editor
            .splitn(2, FILE_ROW_COLUMN_DELIMITER)
            .map(str::trim)
            .fuse();
        let row = components.next().and_then(|row| row.parse::<u32>().ok())?;
        let column = components.next().and_then(|col| col.parse::<u32>().ok());
        Some(Point::new(
            row.saturating_sub(1),
            column.unwrap_or(0).saturating_sub(1),
        ))
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        println!("CANCLE");
        cx.emit(Event::Dismissed);
    }

    fn confirm(&mut self, _: &menu::Confirm, _cx: &mut ViewContext<Self>) {
        // // if let Some(point) = self.point_from_query(cx) {
        // //     self.active_editor.update(cx, |active_editor, cx| {
        // //         let snapshot = active_editor.snapshot(cx).display_snapshot;
        // //         let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
        // //         active_editor.change_selections(Some(Autoscroll::center()), cx, |s| {
        // //             s.select_ranges([point..point])
        // //         });
        // //     });
        // // }

        // cx.emit(Event::Dismissed);
    }

    fn status_text(&self) -> SharedString {
        "Default text".into()
    }
}

impl Modal for GoToLine {
    fn to_modal_event(&self, e: &Self::Event) -> Option<ModalEvent> {
        match e {
            Event::Dismissed => Some(ModalEvent::Dismissed),
        }
    }
}

impl Render for GoToLine {
    type Element = Div<Self, StatefulInteractivity<Self>>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        modal(cx)
            .id("go to line")
            .on_action(Self::cancel)
            .on_action(Self::confirm)
            .w_96()
            .child(
                v_stack()
                    .px_1()
                    .pt_0p5()
                    .gap_px()
                    .child(
                        v_stack()
                            .py_0p5()
                            .px_1()
                            .child(div().px_1().py_0p5().child(self.line_editor.clone())),
                    )
                    .child(
                        div()
                            .h_px()
                            .w_full()
                            .bg(cx.theme().colors().element_background),
                    )
                    .child(
                        h_stack()
                            .justify_between()
                            .px_2()
                            .py_1()
                            .child(Label::new(self.status_text()).color(LabelColor::Muted)),
                    ),
            )
    }
}

// pub struct GoToLine {
//     //line_editor: View<Editor>,
//     active_editor: View<Editor>,
//     prev_scroll_position: Option<gpui::Point<Pixels>>,
//     cursor_point: Point,
//     max_point: Point,
//     has_focus: bool,
// }

// pub enum Event {
//     Dismissed,
// }

// impl GoToLine {
//     pub fn new(active_editor: View<Editor>, cx: &mut ViewContext<Self>) -> Self {

//         let (scroll_position, cursor_point, max_point) = active_editor.update(cx, |editor, cx| {
//             let scroll_position = editor.scroll_position(cx);
//             let buffer = editor.buffer().read(cx).snapshot(cx);
//             (
//                 Some(scroll_position),
//                 editor.selections.newest(cx).head(),
//                 buffer.max_point(),
//             )
//         });

//         cx.on_release(|_, on_release| {}).detach();

//         Self {
//             //line_editor,
//             active_editor,
//             prev_scroll_position: scroll_position,
//             cursor_point,
//             max_point,
//             has_focus: false,
//         }
//     }

//     fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
//         cx.emit(Event::Dismissed);
//     }

//     fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
//         self.prev_scroll_position.take();
//         if let Some(point) = self.point_from_query(cx) {
//             self.active_editor.update(cx, |active_editor, cx| {
//                 let snapshot = active_editor.snapshot(cx).display_snapshot;
//                 let point = snapshot.buffer_snapshot.clip_point(point, Bias::Left);
//                 active_editor.change_selections(Some(Autoscroll::center()), cx, |s| {
//                     s.select_ranges([point..point])
//                 });
//             });
//         }

//         cx.emit(Event::Dismissed);
//     }

// impl EventEmitter for GoToLine {
//     type Event = Event;
// }

// impl Entity for GoToLine {
//     fn release(&mut self, cx: &mut AppContext) {
//         let scroll_position = self.prev_scroll_position.take();
//         self.active_editor.window().update(cx, |cx| {
//             self.active_editor.update(cx, |editor, cx| {
//                 editor.highlight_rows(None);
//                 if let Some(scroll_position) = scroll_position {
//                     editor.set_scroll_position(scroll_position, cx);
//                 }
//             })
//         });
//     }
// }

// impl Render for GoToLine {
//     type Element = Div<Self>;

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
//         // todo!()
//         div()
//     }
// }

// impl View for GoToLine {
//     fn ui_name() -> &'static str {
//         "GoToLine"
//     }

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         let theme = &theme::current(cx).picker;

//         let label = format!(
//             "{}{FILE_ROW_COLUMN_DELIMITER}{} of {} lines",
//             self.cursor_point.row + 1,
//             self.cursor_point.column + 1,
//             self.max_point.row + 1
//         );

//         Flex::new(Axis::Vertical)
//             .with_child(
//                 ChildView::new(&self.line_editor, cx)
//                     .contained()
//                     .with_style(theme.input_editor.container),
//             )
//             .with_child(
//                 Label::new(label, theme.no_matches.label.clone())
//                     .contained()
//                     .with_style(theme.no_matches.container),
//             )
//             .contained()
//             .with_style(theme.container)
//             .constrained()
//             .with_max_width(500.0)
//             .into_any_named("go to line")
//     }

//     fn focus_in(&mut self, _: AnyView, cx: &mut ViewContext<Self>) {
//         self.has_focus = true;
//         cx.focus(&self.line_editor);
//     }

//     fn focus_out(&mut self, _: AnyView, _: &mut ViewContext<Self>) {
//         self.has_focus = false;
//     }
// }

// impl Modal for GoToLine {
//     fn has_focus(&self) -> bool {
//         self.has_focus
//     }

//     fn dismiss_on_event(event: &Self::Event) -> bool {
//         matches!(event, Event::Dismissed)
//     }
// }
