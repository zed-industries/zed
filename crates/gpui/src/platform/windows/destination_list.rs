use std::path::PathBuf;

use itertools::Itertools;
use smallvec::SmallVec;
use windows::{
    Win32::{
        Foundation::PROPERTYKEY,
        Globalization::u_strlen,
        System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance, StructuredStorage::PROPVARIANT},
        UI::{
            Controls::INFOTIPSIZE,
            Shell::{
                Common::{IObjectArray, IObjectCollection},
                DestinationList, EnumerableObjectCollection, ICustomDestinationList, IShellLinkW,
                PropertiesSystem::IPropertyStore,
                ShellLink,
            },
        },
    },
    core::{GUID, HSTRING, Interface},
};

use crate::{Action, MenuItem};

pub(crate) struct JumpList {
    pub(crate) dock_menus: Vec<DockMenuItem>,
    pub(crate) recent_workspaces: Vec<SmallVec<[PathBuf; 2]>>,
}

impl JumpList {
    pub(crate) fn new() -> Self {
        Self {
            dock_menus: Vec::new(),
            recent_workspaces: Vec::new(),
        }
    }
}

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
            _ => anyhow::bail!("Only `MenuItem::Action` is supported for dock menu on Windows."),
        }
    }
}

// This code is based on the example from Microsoft:
// https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/Win7Samples/winui/shell/appshellintegration/RecipePropertyHandler/RecipePropertyHandler.cpp
pub(crate) fn update_jump_list(
    jump_list: &JumpList,
) -> anyhow::Result<Vec<SmallVec<[PathBuf; 2]>>> {
    let (list, removed) = create_destination_list()?;
    add_recent_folders(&list, &jump_list.recent_workspaces, removed.as_ref())?;
    add_dock_menu(&list, &jump_list.dock_menus)?;
    unsafe { list.CommitList() }?;
    Ok(removed)
}

// Copied from:
// https://github.com/microsoft/windows-rs/blob/0fc3c2e5a13d4316d242bdeb0a52af611eba8bd4/crates/libs/windows/src/Windows/Win32/Storage/EnhancedStorage/mod.rs#L1881
const PKEY_TITLE: PROPERTYKEY = PROPERTYKEY {
    fmtid: GUID::from_u128(0xf29f85e0_4ff9_1068_ab91_08002b27b3d9),
    pid: 2,
};

fn create_destination_list() -> anyhow::Result<(ICustomDestinationList, Vec<SmallVec<[PathBuf; 2]>>)>
{
    let list: ICustomDestinationList =
        unsafe { CoCreateInstance(&DestinationList, None, CLSCTX_INPROC_SERVER) }?;

    let mut slots = 0;
    let user_removed: IObjectArray = unsafe { list.BeginList(&mut slots) }?;

    let count = unsafe { user_removed.GetCount() }?;
    if count == 0 {
        return Ok((list, Vec::new()));
    }

    let mut removed = Vec::with_capacity(count as usize);
    for i in 0..count {
        let shell_link: IShellLinkW = unsafe { user_removed.GetAt(i)? };
        let description = {
            // INFOTIPSIZE is the maximum size of the buffer
            // see https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getdescription
            let mut buffer = [0u16; INFOTIPSIZE as usize];
            unsafe { shell_link.GetDescription(&mut buffer)? };
            let len = unsafe { u_strlen(buffer.as_ptr()) };
            String::from_utf16_lossy(&buffer[..len as usize])
        };
        let args = description.split('\n').map(PathBuf::from).collect();

        removed.push(args);
    }

    Ok((list, removed))
}

fn add_dock_menu(list: &ICustomDestinationList, dock_menus: &[DockMenuItem]) -> anyhow::Result<()> {
    unsafe {
        let tasks: IObjectCollection =
            CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)?;
        for (idx, dock_menu) in dock_menus.iter().enumerate() {
            let argument = HSTRING::from(format!("--dock-action {}", idx));
            let description = HSTRING::from(dock_menu.description.as_str());
            let display = dock_menu.name.as_str();
            let task = create_shell_link(argument, description, None, display)?;
            tasks.AddObject(&task)?;
        }
        list.AddUserTasks(&tasks)?;
        Ok(())
    }
}

fn add_recent_folders(
    list: &ICustomDestinationList,
    entries: &[SmallVec<[PathBuf; 2]>],
    removed: &Vec<SmallVec<[PathBuf; 2]>>,
) -> anyhow::Result<()> {
    unsafe {
        let tasks: IObjectCollection =
            CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)?;

        for folder_path in entries.iter().filter(|path| !removed.contains(path)) {
            let argument = HSTRING::from(
                folder_path
                    .iter()
                    .map(|path| format!("\"{}\"", path.display()))
                    .join(" "),
            );

            let description = HSTRING::from(
                folder_path
                    .iter()
                    .map(|path| path.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            // simulate folder icon
            // https://github.com/microsoft/vscode/blob/7a5dc239516a8953105da34f84bae152421a8886/src/vs/platform/workspaces/electron-main/workspacesHistoryMainService.ts#L380
            let icon = HSTRING::from("explorer.exe");

            let display = folder_path
                .iter()
                .map(|p| {
                    p.file_name()
                        .map(|name| name.to_string_lossy())
                        .unwrap_or_else(|| p.to_string_lossy())
                })
                .join(", ");

            tasks.AddObject(&create_shell_link(
                argument,
                description,
                Some(icon),
                &display,
            )?)?;
        }

        if tasks.GetCount().unwrap_or(0) > 0 {
            list.AppendCategory(&HSTRING::from("Recent Folders"), &tasks)?;
        }
        Ok(())
    }
}

fn create_shell_link(
    argument: HSTRING,
    description: HSTRING,
    icon: Option<HSTRING>,
    display: &str,
) -> anyhow::Result<IShellLinkW> {
    unsafe {
        let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        let exe_path = HSTRING::from(std::env::current_exe()?.as_os_str());
        link.SetPath(&exe_path)?;
        link.SetArguments(&argument)?;
        link.SetDescription(&description)?;
        if let Some(icon) = icon {
            link.SetIconLocation(&icon, 0)?;
        }
        let store: IPropertyStore = link.cast()?;
        let title = PROPVARIANT::from(display);
        store.SetValue(&PKEY_TITLE, &title)?;
        store.Commit()?;

        Ok(link)
    }
}
