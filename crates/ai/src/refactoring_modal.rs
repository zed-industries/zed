use crate::refactoring_assistant::RefactoringAssistant;
use collections::HashSet;
use editor::{
    display_map::{BlockContext, BlockDisposition, BlockProperties, BlockStyle},
    scroll::autoscroll::{Autoscroll, AutoscrollStrategy},
    Editor,
};
use gpui::{
    actions, elements::*, platform::MouseButton, AnyViewHandle, AppContext, Entity, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use std::sync::Arc;
use workspace::Workspace;

actions!(assistant, [Refactor]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(RefactoringModal::deploy);
    cx.add_action(RefactoringModal::confirm);
    cx.add_action(RefactoringModal::cancel);
}

enum Event {
    Dismissed,
}

struct RefactoringModal {
    active_editor: WeakViewHandle<Editor>,
    prompt_editor: ViewHandle<Editor>,
    has_focus: bool,
}

impl Entity for RefactoringModal {
    type Event = Event;
}

impl View for RefactoringModal {
    fn ui_name() -> &'static str {
        "RefactoringModal"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        ChildView::new(&self.prompt_editor, cx)
            .mouse::<Self>(0)
            .on_click_out(MouseButton::Left, |_, _, cx| cx.emit(Event::Dismissed))
            .on_click_out(MouseButton::Right, |_, _, cx| cx.emit(Event::Dismissed))
            .into_any()
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_focus = true;
        cx.focus(&self.prompt_editor);
    }

    fn focus_out(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.prompt_editor.is_focused(cx) {
            self.has_focus = false;
            cx.emit(Event::Dismissed);
        }
    }
}

impl RefactoringModal {
    fn deploy(workspace: &mut Workspace, _: &Refactor, cx: &mut ViewContext<Workspace>) {
        if let Some(active_editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        {
            active_editor.update(cx, |editor, cx| {
                let position = editor.selections.newest_anchor().head();
                let prompt_editor = cx.add_view(|cx| {
                    Editor::single_line(
                        Some(Arc::new(|theme| theme.assistant.modal.editor.clone())),
                        cx,
                    )
                });
                let active_editor = cx.weak_handle();
                let refactoring = cx.add_view(|_| RefactoringModal {
                    active_editor,
                    prompt_editor,
                    has_focus: false,
                });
                cx.focus(&refactoring);

                let block_id = editor.insert_blocks(
                    [BlockProperties {
                        style: BlockStyle::Flex,
                        position,
                        height: 2,
                        render: Arc::new({
                            let refactoring = refactoring.clone();
                            move |cx: &mut BlockContext| {
                                ChildView::new(&refactoring, cx)
                                    .contained()
                                    .with_padding_left(cx.gutter_width)
                                    .aligned()
                                    .left()
                                    .into_any()
                            }
                        }),
                        disposition: BlockDisposition::Below,
                    }],
                    Some(Autoscroll::Strategy(AutoscrollStrategy::Newest)),
                    cx,
                )[0];
                cx.subscribe(&refactoring, move |_, refactoring, event, cx| {
                    let Event::Dismissed = event;
                    if let Some(active_editor) = refactoring.read(cx).active_editor.upgrade(cx) {
                        cx.window_context().defer(move |cx| {
                            active_editor.update(cx, |editor, cx| {
                                editor.remove_blocks(HashSet::from_iter([block_id]), None, cx);
                            })
                        });
                    }
                })
                .detach();
            });
        }
    }

    fn cancel(&mut self, _: &editor::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if let Some(editor) = self.active_editor.upgrade(cx) {
            let prompt = self.prompt_editor.read(cx).text(cx);
            RefactoringAssistant::update(cx, |assistant, cx| {
                assistant.refactor(&editor, &prompt, cx);
            });
            cx.emit(Event::Dismissed);
        }
    }
}
