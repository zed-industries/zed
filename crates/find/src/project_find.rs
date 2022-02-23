use crate::SearchMode;
use editor::MultiBuffer;
use gpui::{Entity, ModelContext, ModelHandle, Task};
use project::Project;

struct ProjectFind {
    last_search: SearchParams,
    project: ModelHandle<Project>,
    excerpts: ModelHandle<MultiBuffer>,
    pending_search: Task<Option<()>>,
}

#[derive(Default)]
struct SearchParams {
    query: String,
    regex: bool,
    whole_word: bool,
    case_sensitive: bool,
}

struct ProjectFindView {
    model: ModelHandle<ProjectFind>,
}

impl Entity for ProjectFind {
    type Event = ();
}

impl ProjectFind {
    fn new(project: ModelHandle<Project>, cx: &mut ModelContext<Self>) -> Self {
        let replica_id = project.read(cx).replica_id();
        Self {
            project,
            last_search: Default::default(),
            excerpts: cx.add_model(|_| MultiBuffer::new(replica_id)),
            pending_search: Task::ready(None),
        }
    }

    fn search(&mut self, params: SearchParams, cx: &mut ModelContext<Self>) {
        self.pending_search = cx.spawn_weak(|this, cx| async move {
            //
            None
        });
    }
}
