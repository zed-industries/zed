use gpui::{AppContext, Entity, EventEmitter, Global};
use ui::{App, Context};
use util::{paths::PathExt, ResultExt};

use crate::{WorkspaceId, WORKSPACE_DB};

pub fn init(cx: &mut App) {
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
                let user_removed = update_jump_list(&recent_folders)
                    .log_err()
                    .unwrap_or_default();
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

#[cfg(target_os = "windows")]
fn update_jump_list(entries: &[(String, WorkspaceId)]) -> anyhow::Result<Vec<String>> {
    use destination_list::{add_dock_menu, add_recent_folders, create_destination_list};

    let (list, removed) = create_destination_list()?;
    add_recent_folders(&list, entries, &removed)?;
    add_dock_menu(&list)?;
    unsafe { list.CommitList() }?;
    Ok(removed)
}

#[cfg(not(target_os = "windows"))]
fn update_jump_list(_: &[(String, WorkspaceId)]) -> anyhow::Result<Vec<String>> {
    Ok(vec![])
}

#[cfg(target_os = "windows")]
mod destination_list {
    use windows::{
        core::{Interface, GUID, HSTRING, PROPVARIANT},
        Win32::{
            System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
            UI::Shell::{
                Common::{IObjectArray, IObjectCollection},
                DestinationList, EnumerableObjectCollection, ICustomDestinationList, IShellLinkW,
                PropertiesSystem::{IPropertyStore, PROPERTYKEY},
                ShellLink,
            },
        },
    };

    use crate::WorkspaceId;

    const PKEY_TITLE: PROPERTYKEY = PROPERTYKEY {
        fmtid: GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9),
        pid: 2,
    };

    const PKEY_LINK_ARGS: PROPERTYKEY = PROPERTYKEY {
        fmtid: GUID::from_u128(0x436f2667_14e2_4feb_b30a_146c53b5b674),
        pid: 100,
    };

    pub(super) fn create_destination_list() -> anyhow::Result<(ICustomDestinationList, Vec<String>)>
    {
        let list: ICustomDestinationList =
            unsafe { CoCreateInstance(&DestinationList, None, CLSCTX_INPROC_SERVER) }?;
        let mut pcminslots = 0;
        let user_removed: IObjectArray = unsafe { list.BeginList(&mut pcminslots) }?;
        let count = unsafe { user_removed.GetCount() }?;
        let mut removed = Vec::with_capacity(count as usize);
        for i in 0..count {
            let shell_link: IShellLinkW = unsafe { user_removed.GetAt(i)? };
            let store: IPropertyStore = shell_link.cast()?;
            let argument = unsafe { store.GetValue(&PKEY_LINK_ARGS)? };
            let args = argument.to_string();
            removed.push(args);
        }
        Ok((list, removed))
    }

    pub(super) fn add_dock_menu(list: &ICustomDestinationList) -> anyhow::Result<()> {
        unsafe {
            let tasks: IObjectCollection =
                CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)?;
            let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
            let exe = HSTRING::from(std::env::current_exe()?.to_string_lossy().to_string());
            link.SetPath(&exe)?;
            link.SetArguments(&HSTRING::from("--dock-action 0"))?;
            link.SetDescription(&HSTRING::from("Opens a new window."))?;
            link.SetIconLocation(&exe, 0)?;
            let store: IPropertyStore = link.cast()?;
            let title = PROPVARIANT::from("New Window");
            store.SetValue(&PKEY_TITLE, &title)?;
            store.Commit()?;
            tasks.AddObject(&link)?;
            list.AddUserTasks(&tasks)?;
            Ok(())
        }
    }

    pub(super) fn add_recent_folders(
        list: &ICustomDestinationList,
        entries: &[(String, WorkspaceId)],
        removed: &[String],
    ) -> anyhow::Result<()> {
        unsafe {
            let tasks: IObjectCollection =
                CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)?;
            for (folder_path, _) in entries.iter() {
                if !is_item_in_array(folder_path, removed) {
                    let link = create_shell_link(folder_path)?;
                    tasks.AddObject(&link)?;
                }
            }
            list.AppendCategory(&HSTRING::from("Recent Folders"), &tasks)?;
            Ok(())
        }
    }

    fn is_item_in_array(item: &str, removed: &[String]) -> bool {
        removed
            .iter()
            .any(|removed_item| removed_item.to_lowercase() == item.to_lowercase())
    }

    fn create_shell_link(folder_path: &str) -> anyhow::Result<IShellLinkW> {
        unsafe {
            let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
            let exe = std::env::current_exe()?.to_string_lossy().to_string();
            link.SetPath(&HSTRING::from(exe))?;
            link.SetArguments(&HSTRING::from(folder_path))?;
            link.SetDescription(&HSTRING::from(folder_path))?;
            link.SetIconLocation(&HSTRING::from("explorer.exe"), 0)?;
            let store: IPropertyStore = link.cast()?;
            let title_string = std::path::Path::new(folder_path)
                .file_name()
                .map(|os_str| os_str.to_string_lossy().to_string())
                .unwrap_or(folder_path.to_string());
            let title = PROPVARIANT::from(title_string.as_str());
            store.SetValue(&PKEY_TITLE, &title)?;
            store.Commit()?;

            Ok(link)
        }
    }
}
