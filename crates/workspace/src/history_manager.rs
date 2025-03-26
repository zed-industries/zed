use gpui::{AppContext, Entity, EventEmitter, Global};
use ui::App;
use util::{paths::PathExt, ResultExt};

use crate::WORKSPACE_DB;

pub fn init(cx: &mut App) {
    let manager = cx.new(|_| HistoryManager::new());
    HistoryManager::set_global(manager.clone(), cx);
    cx.subscribe(&manager, |_, event, cx| match event {
        HistoryManagerEvent::Update => perform_update(cx),
    })
    .detach();
    perform_update(cx);
}

fn perform_update(cx: &mut App) {
    cx.spawn(async move |cx| {
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

pub struct HistoryManager;

struct GlobalHistoryManager(Entity<HistoryManager>);

impl Global for GlobalHistoryManager {}

#[derive(Debug, Clone, Copy)]
pub enum HistoryManagerEvent {
    Update,
}

impl EventEmitter<HistoryManagerEvent> for HistoryManager {}

impl HistoryManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalHistoryManager>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(history_manager: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalHistoryManager(history_manager));
    }
}
