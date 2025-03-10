use std::path::Path;

use db::smol;
use gpui::{AppContext, Entity, EventEmitter, Global};
use ui::{App, Context};
use util::{
    paths::{PathExt, SanitizedPath},
    ResultExt,
};

use crate::{WorkspaceId, WORKSPACE_DB};

pub fn init(cx: &mut App) -> Entity<HistoryManager> {
    let manager = cx.new(|_| HistoryManager::new());
    HistoryManager::set_global(manager.clone(), cx);
    cx.subscribe(&manager, |manager, event, cx| match event {
        HistoryManagerEvent::Update => cx
            .spawn(|mut cx| async move {
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
                                .map(|path| path.compact().to_string_lossy().into_owned())
                                .collect::<Vec<_>>()
                                .join(""),
                            id,
                        )
                    })
                    .collect::<Vec<_>>();
                let user_removed = update_jump_list(&recent_folders);
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
                manager
                    .update(&mut cx, |_, cx| {
                        cx.emit(HistoryManagerEvent::Delete(DeleteSource::System))
                    })
                    .log_err();
            })
            .detach(),
        HistoryManagerEvent::Delete(DeleteSource::User) => {
            manager.update(cx, |_, cx| cx.emit(HistoryManagerEvent::Update));
        }
        _ => {}
    })
    .detach();
    // ret.update(cx, update)
    // let x = ret.downgrade();
    // ret
    manager
}

pub struct HistoryManager;

struct GlobalHistoryManager(Entity<HistoryManager>);

impl Global for GlobalHistoryManager {}

#[derive(Debug, Clone, Copy)]
pub enum HistoryManagerEvent {
    Update,
    Delete(DeleteSource),
}

#[derive(Debug, Clone, Copy)]
pub enum DeleteSource {
    System,
    User,
}

impl EventEmitter<HistoryManagerEvent> for HistoryManager {}

impl HistoryManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn global<'a, T>(cx: &Context<'a, T>) -> Option<Entity<Self>> {
        cx.try_global::<GlobalHistoryManager>()
            .map(|model| model.0.clone())
    }

    pub fn set_global(history_manager: Entity<Self>, cx: &mut App) {
        cx.set_global(GlobalHistoryManager(history_manager));
    }
}

fn update_jump_list(entries: &[(String, WorkspaceId)]) -> Vec<String> {
    println!("Updating jump list: {:?}", entries);
    vec![]
}
