use crate::MenuItem;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use windows::Win32::{
    Foundation::MAX_PATH,
    System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
    UI::Shell::{
        Common::{IObjectArray, IObjectCollection},
        DestinationList, EnumerableObjectCollection, ICustomDestinationList, IShellLinkW,
        PropertiesSystem::{IPropertyStore, PSGetPropertyKeyFromName, PROPERTYKEY},
        ShellLink,
    },
};
use windows_core::{w, Interface, HSTRING, PROPVARIANT, PWSTR};

#[derive(Default)]
struct JumpListState {
    tasks: Vec<MenuItem>,
    recent_paths: Vec<PathBuf>,
}

static JUMP_LIST_STATE: Lazy<Mutex<JumpListState>> =
    Lazy::new(|| Mutex::new(JumpListState::default()));

impl JumpListState {
    fn update_tasks(tasks: Vec<MenuItem>) -> anyhow::Result<()> {
        let mut state = Self::get_instance()
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;
        state.tasks = tasks;
        update_jump_list(&mut state)
    }

    fn update_recent_items(paths: &[PathBuf]) -> anyhow::Result<()> {
        let mut state = Self::get_instance()
            .lock()
            .map_err(|e| anyhow!("Lock error: {}", e))?;
        state.recent_paths = paths.to_vec();
        update_jump_list(&mut state)
    }

    fn get_instance() -> &'static Mutex<JumpListState> {
        &JUMP_LIST_STATE
    }
}

fn update_jump_list(state: &mut JumpListState) -> anyhow::Result<()> {
    let jump_list: ICustomDestinationList =
        unsafe { CoCreateInstance(&DestinationList, None, CLSCTX_INPROC_SERVER)? };

    let mut max_slots = 0u32;
    let user_removed_items: IObjectArray = unsafe { jump_list.BeginList(&mut max_slots)? };

    // Add tasks
    let task_items: IObjectCollection =
        unsafe { CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)? };

    let current_exe = std::env::current_exe()?;

    let mut num_tasks = 0u32;
    for menu_item in &state.tasks {
        if let MenuItem::Action { name, action, .. } = menu_item {
            unsafe {
                let link = create_shell_link(
                    current_exe.as_path(),
                    "",
                    action.name(),
                    name.to_string().as_str(),
                    &current_exe.as_path(),
                    0,
                )?;
                task_items.AddObject(&link)?;
                num_tasks += 1;
            }
        }
    }

    if num_tasks > 0 {
        let task_array: IObjectArray = task_items.cast()?;
        unsafe { jump_list.AddUserTasks(&task_array)? };
    }

    // Add recent items
    let recent_items: IObjectCollection =
        unsafe { CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)? };

    // Honor the removals requested by the user
    unsafe {
        for i in 0..user_removed_items.GetCount()? {
            let removed_link: IShellLinkW = user_removed_items.GetAt(i)?;
            let mut removed_path_vec = vec![0u16; MAX_PATH as usize];
            removed_link.GetArguments(&mut removed_path_vec)?;
            let removed_path_wstr = PWSTR::from_raw(removed_path_vec.as_mut_ptr());
            if !removed_path_wstr.is_null() {
                let removed_path = removed_path_wstr.to_string()?;
                // TODO: this path should also be removed from Zed's recent projects persistent storage.
                state
                    .recent_paths
                    .retain(|x| *x != PathBuf::from(removed_path.as_str()));
            }
        }
    }

    let mut num_recent_items = 0u32;
    for path in &state.recent_paths {
        if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
            let path_str = path.to_string_lossy();

            unsafe {
                let shell_link = create_shell_link(
                    current_exe.as_path(),
                    &path_str,
                    &path_str,
                    dir_name,
                    &Path::new("explorer.exe"), // Simulate explorer folder icon
                    0,
                )?;
                recent_items.AddObject(&shell_link)?;
                num_recent_items += 1;
            }
        }
    }

    if num_recent_items > 0 {
        let recent_array: IObjectArray = recent_items.cast()?;
        // TOOD: i18n for "Recent Folders"
        unsafe { jump_list.AppendCategory(w!("Recent Folders"), &recent_array)? };
    }

    unsafe {
        jump_list.CommitList()?;
    }

    Ok(())
}

pub(crate) fn update_recent_items(paths: &[PathBuf]) -> anyhow::Result<()> {
    JumpListState::update_recent_items(paths)
}

pub(crate) fn add_tasks(menu_items: Vec<MenuItem>) -> anyhow::Result<()> {
    JumpListState::update_tasks(menu_items)
}

unsafe fn create_shell_link(
    program: &Path,
    args: &str,
    desc: &str,
    title: &str,
    icon_path: &Path,
    icon_index: i32,
) -> anyhow::Result<IShellLinkW> {
    let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
    link.SetPath(&HSTRING::from(program))?;
    link.SetIconLocation(&HSTRING::from(icon_path), icon_index)?;
    link.SetArguments(&HSTRING::from(args))?; // path
    link.SetDescription(&HSTRING::from(desc))?; // tooltip

    let title_value = PROPVARIANT::from(title);
    let mut title_key = PROPERTYKEY::default();
    PSGetPropertyKeyFromName(w!("System.Title"), &mut title_key)?;

    let store: IPropertyStore = link.cast()?;
    store.SetValue(&title_key, &title_value)?;
    store.Commit()?;

    Ok(link)
}
