use crate::{SearchResult, VectorStore};
use editor::{scroll::autoscroll::Autoscroll, Editor};
use gpui::{
    actions, elements::*, AnyElement, AppContext, ModelHandle, MouseState, Task, ViewContext,
    WeakViewHandle,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use project::{Project, ProjectPath};
use std::{collections::HashMap, sync::Arc, time::Duration};
use util::ResultExt;
use workspace::Workspace;

const MIN_QUERY_LEN: usize = 5;
const EMBEDDING_DEBOUNCE_INTERVAL: Duration = Duration::from_millis(500);

actions!(semantic_search, [Toggle]);

pub type SemanticSearch = Picker<SemanticSearchDelegate>;

pub struct SemanticSearchDelegate {
    workspace: WeakViewHandle<Workspace>,
    project: ModelHandle<Project>,
    vector_store: ModelHandle<VectorStore>,
    selected_match_index: usize,
    matches: Vec<SearchResult>,
    history: HashMap<String, Vec<SearchResult>>,
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
            history: HashMap::new(),
        }
    }
}

impl PickerDelegate for SemanticSearchDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Search repository in natural language...".into()
    }

    fn confirm(&mut self, cx: &mut ViewContext<SemanticSearch>) {
        if let Some(search_result) = self.matches.get(self.selected_match_index) {
            // Open Buffer
            let search_result = search_result.clone();
            let buffer = self.project.update(cx, |project, cx| {
                project.open_buffer(
                    ProjectPath {
                        worktree_id: search_result.worktree_id,
                        path: search_result.file_path.clone().into(),
                    },
                    cx,
                )
            });

            let workspace = self.workspace.clone();
            let position = search_result.clone().offset;
            cx.spawn(|_, mut cx| async move {
                let buffer = buffer.await?;
                workspace.update(&mut cx, |workspace, cx| {
                    let editor = workspace.open_project_item::<Editor>(buffer, cx);
                    editor.update(cx, |editor, cx| {
                        editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                            s.select_ranges([position..position])
                        });
                    });
                })?;
                Ok::<_, anyhow::Error>(())
            })
            .detach_and_log_err(cx);
            cx.emit(PickerEvent::Dismiss);
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
        log::info!("Searching for {:?}...", query);
        if query.len() < MIN_QUERY_LEN {
            log::info!("Query below minimum length");
            return Task::ready(());
        }

        let vector_store = self.vector_store.clone();
        let project = self.project.clone();
        cx.spawn(|this, mut cx| async move {
            cx.background().timer(EMBEDDING_DEBOUNCE_INTERVAL).await;

            let retrieved_cached = this.update(&mut cx, |this, _| {
                let delegate = this.delegate_mut();
                if delegate.history.contains_key(&query) {
                    let historic_results = delegate.history.get(&query).unwrap().to_owned();
                    delegate.matches = historic_results.clone();
                    true
                } else {
                    false
                }
            });

            if let Some(retrieved) = retrieved_cached.log_err() {
                if !retrieved {
                    let task = vector_store.update(&mut cx, |store, cx| {
                        store.search(project.clone(), query.to_string(), 10, cx)
                    });

                    if let Some(results) = task.await.log_err() {
                        log::info!("Not queried previously, searching...");
                        this.update(&mut cx, |this, _| {
                            let delegate = this.delegate_mut();
                            delegate.matches = results.clone();
                            delegate.history.insert(query, results);
                        })
                        .ok();
                    }
                } else {
                    log::info!("Already queried, retrieved directly from cached history");
                }
            }
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

        let path = search_result.file_path.to_string_lossy();
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
