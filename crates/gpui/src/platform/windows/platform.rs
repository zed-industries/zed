// todo(windows): remove
#![allow(unused_variables)]

use std::{
    cell::{Cell, RefCell},
    ffi::{c_void, OsString},
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use ::util::ResultExt;
use anyhow::{anyhow, Context, Result};
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use parking_lot::RwLock;
use smallvec::SmallVec;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Globalization::u_memcpy,
        Graphics::Gdi::*,
        Security::Credentials::*,
        System::{
            Com::*,
            DataExchange::{
                CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard,
                RegisterClipboardFormatW, SetClipboardData,
            },
            LibraryLoader::*,
            Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE},
            Ole::*,
            SystemInformation::*,
            Threading::*,
        },
        UI::{Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
    UI::ViewManagement::UISettings,
};

use crate::*;

pub(crate) struct WindowsPlatform {
    state: RefCell<WindowsPlatformState>,
    raw_window_handles: RwLock<SmallVec<[HWND; 4]>>,
    // The below members will never change throughout the entire lifecycle of the app.
    icon: HICON,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    clipboard_hash_format: u32,
    clipboard_metadata_format: u32,
}

pub(crate) struct WindowsPlatformState {
    callbacks: PlatformCallbacks,
    // NOTE: standard cursor handles don't need to close.
    pub(crate) current_cursor: HCURSOR,
}

#[derive(Default)]
struct PlatformCallbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
}

impl WindowsPlatformState {
    fn new() -> Self {
        let callbacks = PlatformCallbacks::default();
        let current_cursor = load_cursor(CursorStyle::Arrow);

        Self {
            callbacks,
            current_cursor,
        }
    }
}

impl WindowsPlatform {
    pub(crate) fn new() -> Self {
        unsafe {
            OleInitialize(None).expect("unable to initialize Windows OLE");
        }
        let dispatcher = Arc::new(WindowsDispatcher::new());
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);
        let text_system = if let Some(direct_write) = DirectWriteTextSystem::new().log_err() {
            log::info!("Using direct write text system.");
            Arc::new(direct_write) as Arc<dyn PlatformTextSystem>
        } else {
            log::info!("Using cosmic text system.");
            Arc::new(CosmicTextSystem::new()) as Arc<dyn PlatformTextSystem>
        };
        let icon = load_icon().unwrap_or_default();
        let state = RefCell::new(WindowsPlatformState::new());
        let raw_window_handles = RwLock::new(SmallVec::new());
        let clipboard_hash_format = register_clipboard_format(CLIPBOARD_HASH_FORMAT).unwrap();
        let clipboard_metadata_format =
            register_clipboard_format(CLIPBOARD_METADATA_FORMAT).unwrap();

        Self {
            state,
            raw_window_handles,
            icon,
            background_executor,
            foreground_executor,
            text_system,
            clipboard_hash_format,
            clipboard_metadata_format,
        }
    }

    fn redraw_all(&self) {
        for handle in self.raw_window_handles.read().iter() {
            unsafe {
                RedrawWindow(
                    *handle,
                    None,
                    HRGN::default(),
                    RDW_INVALIDATE | RDW_UPDATENOW,
                )
                .ok()
                .log_err();
            }
        }
    }

    pub fn try_get_windows_inner_from_hwnd(&self, hwnd: HWND) -> Option<Rc<WindowsWindowStatePtr>> {
        self.raw_window_handles
            .read()
            .iter()
            .find(|entry| *entry == &hwnd)
            .and_then(|hwnd| try_get_window_inner(*hwnd))
    }

    #[inline]
    fn post_message(&self, message: u32, wparam: WPARAM, lparam: LPARAM) {
        self.raw_window_handles
            .read()
            .iter()
            .for_each(|handle| unsafe {
                PostMessageW(*handle, message, wparam, lparam).log_err();
            });
    }

    fn close_one_window(&self, target_window: HWND) -> bool {
        let mut lock = self.raw_window_handles.write();
        let index = lock
            .iter()
            .position(|handle| *handle == target_window)
            .unwrap();
        lock.remove(index);

        lock.is_empty()
    }
}

impl Platform for WindowsPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        on_finish_launching();
        let vsync_event = unsafe { Owned::new(CreateEventW(None, false, false, None).unwrap()) };
        begin_vsync(*vsync_event);
        'a: loop {
            let wait_result = unsafe {
                MsgWaitForMultipleObjects(Some(&[*vsync_event]), false, INFINITE, QS_ALLINPUT)
            };

            match wait_result {
                // compositor clock ticked so we should draw a frame
                WAIT_EVENT(0) => {
                    self.redraw_all();
                }
                // Windows thread messages are posted
                WAIT_EVENT(1) => {
                    let mut msg = MSG::default();
                    unsafe {
                        while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                            match msg.message {
                                WM_QUIT => break 'a,
                                CLOSE_ONE_WINDOW => {
                                    if self.close_one_window(HWND(msg.lParam.0)) {
                                        break 'a;
                                    }
                                }
                                _ => {
                                    // todo(windows)
                                    // crate `windows 0.56` reports true as Err
                                    TranslateMessage(&msg).as_bool();
                                    DispatchMessageW(&msg);
                                }
                            }
                        }
                    }
                }
                _ => {
                    log::error!("Something went wrong while waiting {:?}", wait_result);
                    break;
                }
            }
        }

        if let Some(ref mut callback) = self.state.borrow_mut().callbacks.quit {
            callback();
        }
    }

    fn quit(&self) {
        self.foreground_executor()
            .spawn(async { unsafe { PostQuitMessage(0) } })
            .detach();
    }

    fn restart(&self, _: Option<PathBuf>) {
        let pid = std::process::id();
        let Some(app_path) = self.app_path().log_err() else {
            return;
        };
        let script = format!(
            r#"
            $pidToWaitFor = {}
            $exePath = "{}"

            while ($true) {{
                $process = Get-Process -Id $pidToWaitFor -ErrorAction SilentlyContinue
                if (-not $process) {{
                    Start-Process -FilePath $exePath
                    break
                }}
                Start-Sleep -Seconds 0.1
            }}
            "#,
            pid,
            app_path.display(),
        );
        let restart_process = std::process::Command::new("powershell.exe")
            .arg("-command")
            .arg(script)
            .spawn();

        match restart_process {
            Ok(_) => self.quit(),
            Err(e) => log::error!("failed to spawn restart script: {:?}", e),
        }
    }

    // todo(windows)
    fn activate(&self, ignoring_other_apps: bool) {}

    // todo(windows)
    fn hide(&self) {
        unimplemented!()
    }

    // todo(windows)
    fn hide_other_apps(&self) {
        unimplemented!()
    }

    // todo(windows)
    fn unhide_other_apps(&self) {
        unimplemented!()
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        WindowsDisplay::displays()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        WindowsDisplay::primary_monitor().map(|display| Rc::new(display) as Rc<dyn PlatformDisplay>)
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        let active_window_hwnd = unsafe { GetActiveWindow() };
        self.try_get_windows_inner_from_hwnd(active_window_hwnd)
            .map(|inner| inner.handle)
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        let lock = self.state.borrow();
        let window = WindowsWindow::new(
            handle,
            options,
            self.icon,
            self.foreground_executor.clone(),
            lock.current_cursor,
        )?;
        drop(lock);
        let handle = window.get_raw_handle();
        self.raw_window_handles.write().push(handle);

        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        system_appearance().log_err().unwrap_or_default()
    }

    fn open_url(&self, url: &str) {
        let url_string = url.to_string();
        self.background_executor()
            .spawn(async move {
                if url_string.is_empty() {
                    return;
                }
                open_target(url_string.as_str());
            })
            .detach();
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.state.borrow_mut().callbacks.open_urls = Some(callback);
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();

        self.foreground_executor()
            .spawn(async move {
                let tx = Cell::new(Some(tx));

                // create file open dialog
                let folder_dialog: IFileOpenDialog = unsafe {
                    CoCreateInstance::<std::option::Option<&IUnknown>, IFileOpenDialog>(
                        &FileOpenDialog,
                        None,
                        CLSCTX_ALL,
                    )
                    .unwrap()
                };

                // dialog options
                let mut dialog_options: FILEOPENDIALOGOPTIONS = FOS_FILEMUSTEXIST;
                if options.multiple {
                    dialog_options |= FOS_ALLOWMULTISELECT;
                }
                if options.directories {
                    dialog_options |= FOS_PICKFOLDERS;
                }

                unsafe {
                    folder_dialog.SetOptions(dialog_options).unwrap();
                    folder_dialog
                        .SetTitle(&HSTRING::from(OsString::from("Select a folder")))
                        .unwrap();
                }

                let hr = unsafe { folder_dialog.Show(None) };

                if hr.is_err() {
                    if hr.unwrap_err().code() == HRESULT(0x800704C7u32 as i32) {
                        // user canceled error
                        if let Some(tx) = tx.take() {
                            tx.send(Ok(None)).unwrap();
                        }
                        return;
                    }
                }

                let mut results = unsafe { folder_dialog.GetResults().unwrap() };

                let mut paths: Vec<PathBuf> = Vec::new();
                for i in 0..unsafe { results.GetCount().unwrap() } {
                    let mut item: IShellItem = unsafe { results.GetItemAt(i).unwrap() };
                    let mut path: PWSTR =
                        unsafe { item.GetDisplayName(SIGDN_FILESYSPATH).unwrap() };
                    let mut path_os_string = OsString::from_wide(unsafe { path.as_wide() });

                    paths.push(PathBuf::from(path_os_string));
                }

                if let Some(tx) = tx.take() {
                    if paths.is_empty() {
                        tx.send(Ok(None)).unwrap();
                    } else {
                        tx.send(Ok(Some(paths))).unwrap();
                    }
                }
            })
            .detach();

        rx
    }

    fn prompt_for_new_path(&self, directory: &Path) -> Receiver<Result<Option<PathBuf>>> {
        let directory = directory.to_owned();
        let (tx, rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                unsafe {
                    let Ok(dialog) = show_savefile_dialog(directory) else {
                        let _ = tx.send(Ok(None));
                        return;
                    };
                    let Ok(_) = dialog.Show(None) else {
                        let _ = tx.send(Ok(None)); // user cancel
                        return;
                    };
                    if let Ok(shell_item) = dialog.GetResult() {
                        if let Ok(file) = shell_item.GetDisplayName(SIGDN_FILESYSPATH) {
                            let _ = tx.send(Ok(Some(PathBuf::from(file.to_string().unwrap()))));
                            return;
                        }
                    }
                    let _ = tx.send(Ok(None));
                }
            })
            .detach();

        rx
    }

    fn reveal_path(&self, path: &Path) {
        let Ok(file_full_path) = path.canonicalize() else {
            log::error!("unable to parse file path");
            return;
        };
        self.background_executor()
            .spawn(async move {
                let Some(path) = file_full_path.to_str() else {
                    return;
                };
                if path.is_empty() {
                    return;
                }
                open_target_in_explorer(path);
            })
            .detach();
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().callbacks.quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().callbacks.reopen = Some(callback);
    }

    // todo(windows)
    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {}
    fn set_dock_menu(&self, menus: Vec<MenuItem>, keymap: &Keymap) {}

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.state.borrow_mut().callbacks.app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().callbacks.will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.state.borrow_mut().callbacks.validate_app_menu_command = Some(callback);
    }

    fn app_path(&self) -> Result<PathBuf> {
        Ok(std::env::current_exe()?)
    }

    // todo(windows)
    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        Err(anyhow!("not yet implemented"))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        let hcursor = load_cursor(style);
        let mut lock = self.state.borrow_mut();
        if lock.current_cursor.0 != hcursor.0 {
            self.post_message(CURSOR_STYLE_CHANGED, WPARAM(0), LPARAM(hcursor.0));
            lock.current_cursor = hcursor;
        }
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        should_auto_hide_scrollbars().log_err().unwrap_or(false)
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        write_to_clipboard(
            item,
            self.clipboard_hash_format,
            self.clipboard_metadata_format,
        );
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        read_from_clipboard(self.clipboard_hash_format, self.clipboard_metadata_format)
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        let mut password = password.to_vec();
        let mut username = username.encode_utf16().chain(Some(0)).collect_vec();
        let mut target_name = windows_credentials_target_name(url)
            .encode_utf16()
            .chain(Some(0))
            .collect_vec();
        self.foreground_executor().spawn(async move {
            let credentials = CREDENTIALW {
                LastWritten: unsafe { GetSystemTimeAsFileTime() },
                Flags: CRED_FLAGS(0),
                Type: CRED_TYPE_GENERIC,
                TargetName: PWSTR::from_raw(target_name.as_mut_ptr()),
                CredentialBlobSize: password.len() as u32,
                CredentialBlob: password.as_ptr() as *mut _,
                Persist: CRED_PERSIST_LOCAL_MACHINE,
                UserName: PWSTR::from_raw(username.as_mut_ptr()),
                ..CREDENTIALW::default()
            };
            unsafe { CredWriteW(&credentials, 0) }?;
            Ok(())
        })
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        let mut target_name = windows_credentials_target_name(url)
            .encode_utf16()
            .chain(Some(0))
            .collect_vec();
        self.foreground_executor().spawn(async move {
            let mut credentials: *mut CREDENTIALW = std::ptr::null_mut();
            unsafe {
                CredReadW(
                    PCWSTR::from_raw(target_name.as_ptr()),
                    CRED_TYPE_GENERIC,
                    0,
                    &mut credentials,
                )?
            };

            if credentials.is_null() {
                Ok(None)
            } else {
                let username: String = unsafe { (*credentials).UserName.to_string()? };
                let credential_blob = unsafe {
                    std::slice::from_raw_parts(
                        (*credentials).CredentialBlob,
                        (*credentials).CredentialBlobSize as usize,
                    )
                };
                let password = credential_blob.to_vec();
                unsafe { CredFree(credentials as *const c_void) };
                Ok(Some((username, password)))
            }
        })
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        let mut target_name = windows_credentials_target_name(url)
            .encode_utf16()
            .chain(Some(0))
            .collect_vec();
        self.foreground_executor().spawn(async move {
            unsafe { CredDeleteW(PCWSTR::from_raw(target_name.as_ptr()), CRED_TYPE_GENERIC, 0)? };
            Ok(())
        })
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        Task::ready(Err(anyhow!("register_url_scheme unimplemented")))
    }
}

impl Drop for WindowsPlatform {
    fn drop(&mut self) {
        self.text_system.destroy();
        unsafe { OleUninitialize() };
    }
}

fn open_target(target: &str) {
    unsafe {
        let ret = ShellExecuteW(
            None,
            windows::core::w!("open"),
            &HSTRING::from(target),
            None,
            None,
            SW_SHOWDEFAULT,
        );
        if ret.0 <= 32 {
            log::error!("Unable to open target: {}", std::io::Error::last_os_error());
        }
    }
}

fn open_target_in_explorer(target: &str) {
    unsafe {
        let ret = ShellExecuteW(
            None,
            windows::core::w!("open"),
            windows::core::w!("explorer.exe"),
            &HSTRING::from(format!("/select,{}", target).as_str()),
            None,
            SW_SHOWDEFAULT,
        );
        if ret.0 <= 32 {
            log::error!(
                "Unable to open target in explorer: {}",
                std::io::Error::last_os_error()
            );
        }
    }
}

unsafe fn show_savefile_dialog(directory: PathBuf) -> Result<IFileSaveDialog> {
    let dialog: IFileSaveDialog = CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL)?;
    let bind_context = CreateBindCtx(0)?;
    let Ok(full_path) = directory.canonicalize() else {
        return Ok(dialog);
    };
    let dir_str = full_path.into_os_string();
    if dir_str.is_empty() {
        return Ok(dialog);
    }
    let dir_vec = dir_str.encode_wide().collect_vec();
    let ret = SHCreateItemFromParsingName(PCWSTR::from_raw(dir_vec.as_ptr()), &bind_context)
        .inspect_err(|e| log::error!("unable to create IShellItem: {}", e));
    if ret.is_ok() {
        let dir_shell_item: IShellItem = ret.unwrap();
        let _ = dialog
            .SetFolder(&dir_shell_item)
            .inspect_err(|e| log::error!("unable to set folder for save file dialog: {}", e));
    }

    Ok(dialog)
}

fn begin_vsync(vsync_evnet: HANDLE) {
    std::thread::spawn(move || unsafe {
        loop {
            windows::Win32::Graphics::Dwm::DwmFlush().log_err();
            SetEvent(vsync_evnet).log_err();
        }
    });
}

fn load_icon() -> Result<HICON> {
    let module = unsafe { GetModuleHandleW(None).context("unable to get module handle")? };
    let handle = unsafe {
        LoadImageW(
            module,
            IDI_APPLICATION,
            IMAGE_ICON,
            0,
            0,
            LR_DEFAULTSIZE | LR_SHARED,
        )
        .context("unable to load icon file")?
    };
    Ok(HICON(handle.0))
}

#[inline]
fn should_auto_hide_scrollbars() -> Result<bool> {
    let ui_settings = UISettings::new()?;
    Ok(ui_settings.AutoHideScrollBars()?)
}

fn register_clipboard_format(format: PCWSTR) -> Result<u32> {
    let ret = unsafe { RegisterClipboardFormatW(format) };
    if ret == 0 {
        Err(anyhow::anyhow!(
            "Error when registering clipboard format: {}",
            std::io::Error::last_os_error()
        ))
    } else {
        Ok(ret)
    }
}

fn write_to_clipboard(item: ClipboardItem, hash_format: u32, metadata_format: u32) {
    write_to_clipboard_inner(item, hash_format, metadata_format).log_err();
    unsafe { CloseClipboard().log_err() };
}

fn write_to_clipboard_inner(
    item: ClipboardItem,
    hash_format: u32,
    metadata_format: u32,
) -> Result<()> {
    unsafe {
        OpenClipboard(None)?;
        EmptyClipboard()?;
        let encode_wide = item.text.encode_utf16().chain(Some(0)).collect_vec();
        set_data_to_clipboard(&encode_wide, CF_UNICODETEXT.0 as u32)?;

        if let Some(ref metadata) = item.metadata {
            let hash_result = {
                let hash = ClipboardItem::text_hash(&item.text);
                hash.to_ne_bytes()
            };
            let encode_wide = std::slice::from_raw_parts(hash_result.as_ptr().cast::<u16>(), 4);
            set_data_to_clipboard(encode_wide, hash_format)?;

            let metadata_wide = metadata.encode_utf16().chain(Some(0)).collect_vec();
            set_data_to_clipboard(&metadata_wide, metadata_format)?;
        }
    }
    Ok(())
}

fn set_data_to_clipboard(data: &[u16], format: u32) -> Result<()> {
    unsafe {
        let global = GlobalAlloc(GMEM_MOVEABLE, data.len() * 2)?;
        let handle = GlobalLock(global);
        u_memcpy(handle as _, data.as_ptr(), data.len() as _);
        let _ = GlobalUnlock(global);
        SetClipboardData(format, HANDLE(global.0 as isize))?;
    }
    Ok(())
}

fn read_from_clipboard(hash_format: u32, metadata_format: u32) -> Option<ClipboardItem> {
    let result = read_from_clipboard_inner(hash_format, metadata_format).log_err();
    unsafe { CloseClipboard().log_err() };
    result
}

fn read_from_clipboard_inner(hash_format: u32, metadata_format: u32) -> Result<ClipboardItem> {
    unsafe {
        OpenClipboard(None)?;
        let text = {
            let handle = GetClipboardData(CF_UNICODETEXT.0 as u32)?;
            let text = PCWSTR(handle.0 as *const u16);
            String::from_utf16_lossy(text.as_wide())
        };
        let mut item = ClipboardItem {
            text,
            metadata: None,
        };
        let Some(hash) = read_hash_from_clipboard(hash_format) else {
            return Ok(item);
        };
        let Some(metadata) = read_metadata_from_clipboard(metadata_format) else {
            return Ok(item);
        };
        if hash == ClipboardItem::text_hash(&item.text) {
            item.metadata = Some(metadata);
        }
        Ok(item)
    }
}

fn read_hash_from_clipboard(hash_format: u32) -> Option<u64> {
    unsafe {
        let handle = GetClipboardData(hash_format).log_err()?;
        let raw_ptr = handle.0 as *const u16;
        let hash_bytes: [u8; 8] = std::slice::from_raw_parts(raw_ptr.cast::<u8>(), 8)
            .to_vec()
            .try_into()
            .log_err()?;
        Some(u64::from_ne_bytes(hash_bytes))
    }
}

fn read_metadata_from_clipboard(metadata_format: u32) -> Option<String> {
    unsafe {
        let handle = GetClipboardData(metadata_format).log_err()?;
        let text = PCWSTR(handle.0 as *const u16);
        Some(String::from_utf16_lossy(text.as_wide()))
    }
}

// clipboard
pub const CLIPBOARD_HASH_FORMAT: PCWSTR = windows::core::w!("zed-text-hash");
pub const CLIPBOARD_METADATA_FORMAT: PCWSTR = windows::core::w!("zed-metadata");

#[cfg(test)]
mod tests {
    use crate::{ClipboardItem, Platform, WindowsPlatform};

    #[test]
    fn test_clipboard() {
        let platform = WindowsPlatform::new();
        let item = ClipboardItem::new("你好".to_string());
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));

        let item = ClipboardItem::new("12345".to_string());
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));

        let item = ClipboardItem::new("abcdef".to_string()).with_metadata(vec![3, 4]);
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));
    }
}
