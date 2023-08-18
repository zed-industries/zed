use collections::HashMap;
use editor::Editor;
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, Task, View, ViewContext, ViewHandle,
};
use std::sync::Arc;
use workspace::{Modal, Workspace};

actions!(assistant, [Refactor]);

fn init(cx: &mut AppContext) {
    cx.set_global(RefactoringAssistant::new());
    cx.add_action(RefactoringModal::deploy);
}

pub struct RefactoringAssistant {
    pending_edits_by_editor: HashMap<usize, Task<Option<()>>>,
}

impl RefactoringAssistant {
    fn new() -> Self {
        Self {
            pending_edits_by_editor: Default::default(),
        }
    }

    fn refactor(&mut self, editor: &ViewHandle<Editor>, prompt: &str, cx: &mut AppContext) {}
}

struct RefactoringModal {
    prompt_editor: ViewHandle<Editor>,
    has_focus: bool,
}

impl Entity for RefactoringModal {
    type Event = ();
}

impl View for RefactoringModal {
    fn ui_name() -> &'static str {
        "RefactoringModal"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        todo!()
    }

    fn focus_in(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = true;
    }

    fn focus_out(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }
}

impl Modal for RefactoringModal {
    fn has_focus(&self) -> bool {
        self.has_focus
    }

    fn dismiss_on_event(event: &Self::Event) -> bool {
        todo!()
    }
}

impl RefactoringModal {
    fn deploy(workspace: &mut Workspace, _: &Refactor, cx: &mut ViewContext<Workspace>) {
        workspace.toggle_modal(cx, |_, cx| {
            let prompt_editor = cx.add_view(|cx| {
                Editor::auto_height(
                    4,
                    Some(Arc::new(|theme| theme.search.editor.input.clone())),
                    cx,
                )
            });
            cx.add_view(|_| RefactoringModal {
                prompt_editor,
                has_focus: false,
            })
        });
    }
}

// ABCDEFG
// XCDEFG
//
//
