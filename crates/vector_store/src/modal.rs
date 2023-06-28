use std::sync::Arc;

use gpui::{
    actions, elements::*, AnyElement, AppContext, ModelHandle, MouseState, Task, ViewContext,
    WeakViewHandle,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use project::Project;
use util::ResultExt;
use workspace::Workspace;

use crate::{SearchResult, VectorStore};

actions!(semantic_search, [Toggle]);

pub type SemanticSearch = Picker<SemanticSearchDelegate>;

pub struct SemanticSearchDelegate {
    workspace: WeakViewHandle<Workspace>,
    project: ModelHandle<Project>,
    vector_store: ModelHandle<VectorStore>,
    selected_match_index: usize,
    matches: Vec<SearchResult>,
}

impl SemanticSearchDelegate {
    // This is currently searching on every keystroke,
    // This is wildly overkill, and has the potential to get expensive
    // We will need to update this to throttle searching
    pub fn new(
        workspace: WeakViewHandle<Workspace>,
        project: ModelHandle<Project>,
        vector_store: ModelHandle<VectorStore>,
    ) -> Self {
        Self {
            workspace,
            project,
            vector_store,
            selected_match_index: 0,
            matches: vec![],
        }
    }
}

impl PickerDelegate for SemanticSearchDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Search repository in natural language...".into()
    }

    fn confirm(&mut self, cx: &mut ViewContext<SemanticSearch>) {
        if let Some(search_result) = self.matches.get(self.selected_match_index) {
            // search_result.file_path
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<SemanticSearch>) {}

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<SemanticSearch>) {
        self.selected_match_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<SemanticSearch>) -> Task<()> {
        let task = self.vector_store.update(cx, |store, cx| {
            store.search(&self.project, query.to_string(), 10, cx)
        });

        cx.spawn(|this, mut cx| async move {
            let results = task.await.log_err();
            this.update(&mut cx, |this, cx| {
                if let Some(results) = results {
                    let delegate = this.delegate_mut();
                    delegate.matches = results;
                }
            });
        })
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> AnyElement<Picker<Self>> {
        let theme = theme::current(cx);
        let style = &theme.picker.item;
        let current_style = style.in_state(selected).style_for(mouse_state);

        let search_result = &self.matches[ix];

        let mut path = search_result.file_path.to_string_lossy();
        let name = search_result.name.clone();

        Flex::column()
            .with_child(Text::new(name, current_style.label.text.clone()).with_soft_wrap(false))
            .with_child(Label::new(
                path.to_string(),
                style.inactive_state().default.label.clone(),
            ))
            .contained()
            .with_style(current_style.container)
            .into_any()
    }
}
