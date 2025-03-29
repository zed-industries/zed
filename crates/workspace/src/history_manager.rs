use std::path::PathBuf;

use gpui::{AppContext, Entity, Global};
use smallvec::SmallVec;
use ui::App;
use util::{paths::PathExt, ResultExt};

use crate::{SerializedWorkspaceLocation, WorkspaceId, WORKSPACE_DB};

pub fn init(cx: &mut App) {
    let manager = cx.new(|_| HistoryManager::new());
    HistoryManager::set_global(manager.clone(), cx);
    HistoryManager::init(manager, cx);
}

pub struct HistoryManager {
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

    fn init(this: Entity<HistoryManager>, cx: &App) {
        cx.spawn(async move |cx| {
            let recent_folders = WORKSPACE_DB
                .recent_workspaces_on_disk()
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|(id, location)| HistoryManagerEntry::new(id, &location))
                .collect::<Vec<_>>();
            this.update(cx, |this, cx| {
                this.history = recent_folders;
                this.update_jump_list(cx);
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

    pub fn update_history(&mut self, id: WorkspaceId, entry: HistoryManagerEntry, cx: &App) {
        if let Some(pos) = self.history.iter().position(|e| e.id == id) {
            if pos == 0 {
                return;
            }
            self.history.remove(pos);
        }
        self.history.insert(0, entry);
        self.update_jump_list(cx);
    }

    pub fn delete_history(&mut self, id: WorkspaceId, cx: &App) {
        let Some(pos) = self.history.iter().position(|e| e.id == id) else {
            return;
        };
        self.history.remove(pos);
        self.update_jump_list(cx);
    }

    fn update_jump_list(&mut self, cx: &App) {
        let entries = self
            .history
            .iter()
            .map(|entry| &entry.path)
            .collect::<Vec<_>>();
        let user_removed = cx.update_jump_list(entries.as_slice());
        let mut deleted_ids = Vec::new();
        for idx in (0..self.history.len()).rev() {
            if let Some(entry) = self.history.get(idx) {
                if user_removed.contains(&entry.path) {
                    deleted_ids.push(entry.id);
                    self.history.remove(idx);
                }
            }
        }
        cx.spawn(async move |_| {
            for id in deleted_ids.iter() {
                WORKSPACE_DB.delete_workspace_by_id(*id).await.log_err();
            }
        })
        .detach();
    }
}

impl HistoryManagerEntry {
    pub fn new(id: WorkspaceId, location: &SerializedWorkspaceLocation) -> Self {
        let path = location
            .sorted_paths()
            .iter()
            .map(|path| path.compact())
            .collect::<SmallVec<[PathBuf; 2]>>();
        Self { id, path }
    }
}
