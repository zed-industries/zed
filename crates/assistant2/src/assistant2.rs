use editor::Editor;
use gpui::{list, AnyElement, ListAlignment, ListState, Render, View};
use semantic_index::SearchResult;
use ui::prelude::*;

pub struct AssistantPanel {
    chat: View<AssistantChat>,
}

impl AssistantPanel {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let chat = cx.new_view(AssistantChat::new);
        Self { chat }
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().v_flex().child(self.chat.clone())
    }
}

struct AssistantChat {
    messages: Vec<AssistantMessage>,
    list_state: ListState,
}

impl AssistantChat {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        let messages = vec![AssistantMessage::User {
            body: cx.new_view(|cx| {
                let editor = Editor::auto_height(80, cx);
                editor
                    .buffer()
                    .update(cx, |buffer, cx| buffer.as_singleton().unwrap());
                editor
            }),
            contexts: Vec::new(),
        }];

        let this = cx.view().downgrade();
        let list_state = ListState::new(
            messages.len(),
            ListAlignment::Top,
            px(1024.),
            move |ix, cx| {
                this.update(cx, |this, cx| this.render_message(ix, cx))
                    .unwrap()
            },
        );

        Self {
            messages,
            list_state,
        }
    }

    fn render_message(&self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        match &self.messages[ix] {
            AssistantMessage::User { body, contexts } => body.clone().into_any_element(),
            AssistantMessage::Assistant { body } => body.clone().into_any_element(),
        }
    }
}

impl Render for AssistantChat {
    fn render(
        &mut self,
        cx: &mut workspace::ui::prelude::ViewContext<Self>,
    ) -> impl gpui::prelude::IntoElement {
        list(self.list_state.clone())
    }
}

enum AssistantMessage {
    User {
        body: View<Editor>,
        contexts: Vec<AssistantContext>,
    },
    Assistant {
        body: SharedString,
    },
}

enum AssistantContext {
    Codebase { results: Vec<SearchResult> },
}
