use std::{path::PathBuf, sync::Arc};

use fs::Fs;
use gpui::{AppContext, Entity, Global, MenuItem};
use smallvec::SmallVec;
use ui::{App, Context};
use util::{ResultExt, paths::PathExt};

use crate::{
    NewWindow, SerializedWorkspaceLocation, WorkspaceId, path_list::PathList,
    persistence::WorkspaceDb,
};

pub fn init(fs: Arc<dyn Fs>, cx: &mut App) {
    let manager = cx.new(|_| HistoryManager::new());
    HistoryManager::set_global(manager.clone(), cx);
    HistoryManager::init(manager, fs, cx);
}

pub struct HistoryManager {
    /// The history of workspaces that have been opened in the past, in reverse order.
    /// The most recent workspace is at the end of the vector.
    history: Vec<HistoryManagerEntry>,
}

#[derive(Debug)]
pub struct HistoryManagerEntry {
    pub id: WorkspaceId,
    pub path: SmallVec<[PathBuf; 2]>,
}

struct GlobalHistoryManager(Entity<HistoryManager>);

impl Global for GlobalHistoryManager {}

impl HistoryManager {
    fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    fn init(this: Entity<HistoryManager>, fs: Arc<dyn Fs>, cx: &App) {
        let db = WorkspaceDb::global(cx);
        cx.spawn(async move |cx| {
            let recent_folders = db
                .recent_project_workspaces(fs.as_ref())
                .await
                .log_err()
                .unwrap_or_default()
                .into_iter()
                .rev()
                .filter_map(|workspace| {
                    if matches!(workspace.location, SerializedWorkspaceLocation::Local) {
                        Some(HistoryManagerEntry::new(
                            workspace.workspace_id,
                            &workspace.paths,
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            this.update(cx, |this, cx| {
                this.restore_history(recent_folders, cx);
            })
        })
        .detach();
    }

    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalHistoryManager>()
            .map(|model| model.0.clone())
    }

    fn set_global(history_manager: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalHistoryManager(history_manager));
    }

    fn restore_history(&mut self, mut history: Vec<HistoryManagerEntry>, cx: &mut Context<Self>) {
        // Projects can open while the database query is running. Keep those entries
        // newer than the persisted snapshot instead of overwriting them.
        history.retain(|entry| !self.history.iter().any(|opened| opened.id == entry.id));
        history.append(&mut self.history);
        self.history = history;
        for entry in &self.history {
            for path in &entry.path {
                cx.add_recent_document(path);
            }
        }
        self.update_jump_list(cx);
    }

    pub fn update_history(
        &mut self,
        id: WorkspaceId,
        entry: HistoryManagerEntry,
        cx: &mut Context<'_, HistoryManager>,
    ) {
        if let Some(pos) = self.history.iter().position(|e| e.id == id) {
            self.history.remove(pos);
        }
        for path in &entry.path {
            cx.add_recent_document(path);
        }
        self.history.push(entry);
        self.update_jump_list(cx);
    }

    pub fn delete_history(&mut self, id: WorkspaceId, cx: &mut Context<'_, HistoryManager>) {
        let Some(pos) = self.history.iter().position(|e| e.id == id) else {
            return;
        };
        self.history.remove(pos);
        self.update_jump_list(cx);
    }

    fn update_jump_list(&mut self, cx: &mut Context<'_, HistoryManager>) {
        let menus = vec![MenuItem::action("New Window", NewWindow)];
        let entries = self
            .history
            .iter()
            .rev()
            .map(|entry| entry.path.iter().map(|path| path.compact()).collect())
            .collect::<Vec<_>>();
        let user_removed = cx.update_jump_list(menus, entries);
        let db = WorkspaceDb::global(cx);
        cx.spawn(async move |this, cx| {
            let user_removed = user_removed.await;
            if user_removed.is_empty() {
                return;
            }
            let mut deleted_ids = Vec::new();
            if let Ok(()) = this.update(cx, |this, _| {
                for idx in (0..this.history.len()).rev() {
                    if let Some(entry) = this.history.get(idx)
                        && user_removed.contains(&entry.path)
                    {
                        deleted_ids.push(entry.id);
                        this.history.remove(idx);
                    }
                }
            }) {
                for id in deleted_ids.iter() {
                    db.delete_workspace_by_id(*id).await.log_err();
                }
            }
        })
        .detach();
    }
}

impl HistoryManagerEntry {
    pub fn new(id: WorkspaceId, paths: &PathList) -> Self {
        let path = paths
            .ordered_paths()
            .cloned()
            .collect::<SmallVec<[PathBuf; 2]>>();
        Self { id, path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use util::paths::home_dir;

    fn entry(id: i64, path: &std::path::Path) -> HistoryManagerEntry {
        HistoryManagerEntry::new(WorkspaceId(id), &PathList::new(&[path]))
    }

    #[gpui::test]
    fn test_recent_documents_include_every_opened_project(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
        let backend = home_dir().join("projects/backend");
        let frontend = home_dir().join("projects/frontend");
        let manager = cx.new(|_| HistoryManager::new());

        manager.update(cx, |manager, cx| {
            manager.update_history(WorkspaceId(1), entry(1, &backend), cx);
            manager.update_history(WorkspaceId(2), entry(2, &frontend), cx);
        });
        assert_eq!(
            cx.recent_documents(),
            vec![backend.clone(), frontend.clone()]
        );

        manager.update(cx, |manager, cx| {
            manager.update_history(WorkspaceId(1), entry(1, &backend), cx);
        });
        assert_eq!(cx.recent_documents(), vec![frontend, backend]);
    }

    #[gpui::test]
    fn test_restore_recent_documents_preserves_projects_opened_during_load(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
        let backend = home_dir().join("projects/backend");
        let frontend = home_dir().join("projects/frontend");
        let other = home_dir().join("projects/other");
        let manager = cx.new(|_| HistoryManager::new());

        manager.update(cx, |manager, cx| {
            manager.update_history(WorkspaceId(1), entry(1, &backend), cx);
            manager.update_history(WorkspaceId(2), entry(2, &frontend), cx);
            manager.restore_history(vec![entry(1, &backend), entry(3, &other)], cx);
            assert_eq!(
                manager
                    .history
                    .iter()
                    .map(|entry| entry.id)
                    .collect::<Vec<_>>(),
                vec![WorkspaceId(3), WorkspaceId(1), WorkspaceId(2)]
            );
        });

        assert_eq!(cx.recent_documents(), vec![other, backend, frontend]);
    }

    #[gpui::test]
    async fn test_recent_documents_are_loaded_from_persisted_projects(cx: &mut TestAppContext) {
        use crate::persistence::model::SerializedWorkspace;

        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
        let fs = fs::FakeFs::new(cx.executor());
        let directory = home_dir().join("projects");
        fs.insert_tree(
            &directory,
            serde_json::json!({ "backend": {}, "frontend": {} }),
        )
        .await;
        let paths = [directory.join("backend"), directory.join("frontend")];
        let db = cx.update(|cx| WorkspaceDb::global(cx));
        for path in &paths {
            let id = db.next_id().await.expect("allocate workspace id");
            db.save_workspace(SerializedWorkspace {
                id,
                location: SerializedWorkspaceLocation::Local,
                paths: PathList::new(&[path]),
                identity_paths: None,
                center_group: Default::default(),
                window_bounds: None,
                centered_layout: false,
                display: None,
                docks: Default::default(),
                session_id: None,
                bookmarks: Default::default(),
                numbered_bookmarks: Default::default(),
                breakpoints: Default::default(),
                user_toolchains: Default::default(),
                recent_navigation_history: Vec::new(),
                window_id: None,
                path_style: util::paths::PathStyle::local(),
            })
            .await;
        }

        cx.update(|cx| init(fs, cx));
        cx.run_until_parked();

        let mut recent_documents = cx.recent_documents();
        recent_documents.sort();
        assert_eq!(recent_documents, paths);
    }
}
