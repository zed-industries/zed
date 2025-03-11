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

use crate::{Action, MenuItem};

pub(crate) struct DockMenuItem {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) action: Box<dyn Action>,
}

impl DockMenuItem {
    pub(crate) fn new(item: MenuItem) -> anyhow::Result<Self> {
        match item {
            MenuItem::Action { name, action, .. } => Ok(Self {
                name: name.clone().into(),
                description: if name == "New Window" {
                    "Opens a new window".to_string()
                } else {
                    name.into()
                },
                action,
            }),
            _ => Err(anyhow::anyhow!(
                "Only `MenuItem::Action` is supported for dock menu on Windows."
            )),
        }
    }
}

pub(crate) fn update_jump_list(
    entries: &[&str],
    dock_menus: &[DockMenuItem],
) -> anyhow::Result<Vec<String>> {
    let (list, removed) = create_destination_list()?;
    add_recent_folders(&list, entries, &removed)?;
    add_dock_menu(&list, dock_menus)?;
    unsafe { list.CommitList() }?;
    Ok(removed)
}

const PKEY_TITLE: PROPERTYKEY = PROPERTYKEY {
    fmtid: GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9),
    pid: 2,
};

const PKEY_LINK_ARGS: PROPERTYKEY = PROPERTYKEY {
    fmtid: GUID::from_u128(0x436f2667_14e2_4feb_b30a_146c53b5b674),
    pid: 100,
};

fn create_destination_list() -> anyhow::Result<(ICustomDestinationList, Vec<String>)> {
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

fn add_dock_menu(list: &ICustomDestinationList, dock_menus: &[DockMenuItem]) -> anyhow::Result<()> {
    unsafe {
        let tasks: IObjectCollection =
            CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)?;
        let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        let exe_path = std::env::current_exe()?;
        let exe = HSTRING::from(exe_path.as_os_str());
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

fn add_recent_folders(
    list: &ICustomDestinationList,
    entries: &[&str],
    removed: &[String],
) -> anyhow::Result<()> {
    unsafe {
        let tasks: IObjectCollection =
            CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)?;
        for folder_path in entries.iter() {
            if !is_item_in_array(folder_path, removed) {
                let link = create_shell_link(folder_path)?;
                tasks.AddObject(&link)?;
            }
        }
        list.AppendCategory(&HSTRING::from("Recent Folders"), &tasks)?;
        Ok(())
    }
}

#[inline]
fn is_item_in_array(item: &str, removed: &[String]) -> bool {
    removed
        .iter()
        .any(|removed_item| removed_item.to_lowercase() == item.to_lowercase())
}

fn create_shell_link(folder_path: &str) -> anyhow::Result<IShellLinkW> {
    unsafe {
        let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        let exe_path = std::env::current_exe()?;
        let exe = HSTRING::from(exe_path.as_os_str());
        link.SetPath(&exe)?;
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
