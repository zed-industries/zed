use std::path::PathBuf;

use gpui::{AppContext, Entity, EventEmitter, Global};
use smallvec::SmallVec;
use ui::App;
use util::{paths::PathExt, ResultExt};

use crate::{SerializedWorkspaceLocation, WorkspaceId, WORKSPACE_DB};

pub fn init(cx: &mut App) {
    let manager = cx.new(|_| HistoryManager::new());
    HistoryManager::set_global(manager.clone(), cx);
    cx.subscribe(&manager, |this, event, cx| match event {
        HistoryManagerEvent::Update => perform_update(this, cx),
    })
    .detach();
    // perform_update(cx);
    HistoryManager::init(manager, cx);
}

fn perform_update(manager: Entity<HistoryManager>, cx: &mut App) {
    cx.spawn(async move |cx| {
        manager
            .update(cx, |this, cx| {
                println!("History: {:#?}", this.history);
            })
            .log_err();
        let recent_folders = WORKSPACE_DB
            .recent_workspaces_on_disk()
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|(id, location)| {
                (
                    location
                        .sorted_paths()
                        .iter()
                        .map(|path| path.compact())
                        .collect::<Vec<_>>(),
                    id,
                )
            })
            .collect::<Vec<_>>();
        let entries = recent_folders
            .iter()
            .map(|(query, _)| query)
            .collect::<Vec<_>>();
        if let Some(user_removed) = cx
            .update(|cx| cx.update_jump_list(entries.as_slice()))
            .log_err()
        {
            let deleted_ids = recent_folders
                .into_iter()
                .filter_map(|(query, id)| {
                    if user_removed.contains(&query) {
                        Some(id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            for id in deleted_ids.iter() {
                WORKSPACE_DB.delete_workspace_by_id(*id).await.log_err();
            }
        }
    })
    .detach()
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

#[derive(Debug, Clone, Copy)]
pub enum HistoryManagerEvent {
    Update,
}

impl EventEmitter<HistoryManagerEvent> for HistoryManager {}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn init(this: Entity<HistoryManager>, cx: &App) {
        cx.spawn(async move |cx| {
            let recent_folders = WORKSPACE_DB
                .recent_workspaces_on_disk()
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|(id, location)| HistoryManagerEntry::new(id, location))
                .collect::<Vec<_>>();
            this.update(cx, |this, _| {
                this.history = recent_folders;
            })
        })
        .detach();
    }

    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalHistoryManager>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(history_manager: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalHistoryManager(history_manager));
    }
}

impl HistoryManagerEntry {
    pub fn new(id: WorkspaceId, location: SerializedWorkspaceLocation) -> Self {
        let path = location
            .sorted_paths()
            .iter()
            .map(|path| path.compact())
            .collect::<SmallVec<[PathBuf; 2]>>();
        Self { id, path }
    }
}
