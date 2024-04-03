use collections::HashMap;
use gpui::{AppContext, Context, Entity, Global, Model, ModelContext, Task, WeakModel};
use project::{Project, Worktree};
use std::path::Path;

struct SemanticIndex {
    db: lmdb::Database,
    project_indices: HashMap<WeakModel<Project>, Model<ProjectIndex>>,
}

impl Global for SemanticIndex {}

impl SemanticIndex {
    pub fn new(path: &Path) -> Result<Self, lmdb::Error> {
        let env = lmdb::Environment::new().open(path)?;
        let db = env.create_db(None, lmdb::DatabaseFlags::empty())?;

        Ok(SemanticIndex {
            db,
            project_indices: HashMap::default(),
        })
    }

    pub fn project_index(
        &mut self,
        project: Model<Project>,
        cx: &mut AppContext,
    ) -> Model<ProjectIndex> {
        self.project_indices
            .entry(project.downgrade())
            .or_insert_with(|| cx.new_model(|cx| ProjectIndex::new(project, self.db.clone(), cx)))
            .clone()
    }
}

struct ProjectIndex {
    db: lmdb::Database,
    project: WeakModel<Project>,
    worktree_scans: HashMap<WeakModel<Worktree>, Task<()>>,
}

impl ProjectIndex {
    fn new(project: Model<Project>, db: lmdb::Database, cx: &mut ModelContext<Self>) -> Self {
        let mut this = ProjectIndex {
            db,
            project: project.downgrade(),
            worktree_scans: HashMap::default(),
        };

        for worktree in project.read(cx).worktrees().collect::<Vec<_>>() {
            this.add_worktree(worktree, cx);
        }

        this
    }

    fn add_worktree(&mut self, worktree: Model<Worktree>, cx: &mut ModelContext<Self>) {
        if let Some(local_worktree) = worktree.read(cx).as_local() {
            let snapshot = local_worktree.snapshot();
            let scan = cx.spawn(|this, cx| async move {
                snapshot;
            });
            self.worktree_scans.insert(worktree.downgrade(), scan);
        }
    }

    fn remove_worktree(&mut self, worktree: Model<Worktree>, cx: &mut ModelContext<Self>) {
        self.worktree_scans.remove(&worktree.downgrade());
    }
}
