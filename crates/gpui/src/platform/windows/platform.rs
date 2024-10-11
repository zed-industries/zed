use std::{
    cell::RefCell,
    mem::ManuallyDrop,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use ::util::ResultExt;
use anyhow::{anyhow, Context, Result};
use async_task::Runnable;
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use parking_lot::RwLock;
use smallvec::SmallVec;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::{
            Gdi::*,
            Imaging::{CLSID_WICImagingFactory, IWICImagingFactory},
        },
        Security::Credentials::*,
        System::{Com::*, LibraryLoader::*, Ole::*, SystemInformation::*, Threading::*},
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
    main_receiver: flume::Receiver<Runnable>,
    dispatch_event: HANDLE,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<DirectWriteTextSystem>,
    windows_version: WindowsVersion,
    bitmap_factory: ManuallyDrop<IWICImagingFactory>,
    validation_number: usize,
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
        let (main_sender, main_receiver) = flume::unbounded::<Runnable>();
        let dispatch_event = unsafe { CreateEventW(None, false, false, None) }.unwrap();
        let dispatcher = Arc::new(WindowsDispatcher::new(main_sender, dispatch_event));
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);
        let bitmap_factory = ManuallyDrop::new(unsafe {
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)
                .expect("Error creating bitmap factory.")
        });
        let text_system = Arc::new(
            DirectWriteTextSystem::new(&bitmap_factory)
                .expect("Error creating DirectWriteTextSystem"),
        );
        let icon = load_icon().unwrap_or_default();
        let state = RefCell::new(WindowsPlatformState::new());
        let raw_window_handles = RwLock::new(SmallVec::new());
        let windows_version = WindowsVersion::new().expect("Error retrieve windows version");
        let validation_number = rand::random::<usize>();

        Self {
            state,
            raw_window_handles,
            icon,
            main_receiver,
            dispatch_event,
            background_executor,
            foreground_executor,
            text_system,
            windows_version,
            bitmap_factory,
            validation_number,
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

    fn close_one_window(
        &self,
        target_window: HWND,
        validation_number: usize,
        msg: *const MSG,
    ) -> bool {
        if validation_number != self.validation_number {
            unsafe { DispatchMessageW(msg) };
            return false;
        }
        let mut lock = self.raw_window_handles.write();
        let index = lock
            .iter()
            .position(|handle| *handle == target_window)
            .unwrap();
        lock.remove(index);

        lock.is_empty()
    }

    #[inline]
    fn run_foreground_tasks(&self) {
        for runnable in self.main_receiver.drain() {
            runnable.run();
        }
    }

    fn generate_creation_info(&self) -> WindowCreationInfo {
        WindowCreationInfo {
            icon: self.icon,
            executor: self.foreground_executor.clone(),
            current_cursor: self.state.borrow().current_cursor,
            windows_version: self.windows_version,
            validation_number: self.validation_number,
            main_receiver: self.main_receiver.clone(),
        }
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
                MsgWaitForMultipleObjects(
                    Some(&[*vsync_event, self.dispatch_event]),
                    false,
                    INFINITE,
                    QS_ALLINPUT,
                )
            };

            match wait_result {
                // compositor clock ticked so we should draw a frame
                WAIT_EVENT(0) => self.redraw_all(),
                // foreground tasks are dispatched
                WAIT_EVENT(1) => self.run_foreground_tasks(),
                // Windows thread messages are posted
                WAIT_EVENT(2) => {
                    let mut msg = MSG::default();
                    unsafe {
                        while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                            match msg.message {
                                WM_QUIT => break 'a,
                                CLOSE_ONE_WINDOW => {
                                    if self.close_one_window(
                                        HWND(msg.lParam.0 as _),
                                        msg.wParam.0,
                                        &msg,
                                    ) {
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
                    // foreground tasks may have been queued in the message handlers
                    self.run_foreground_tasks();
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

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

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
        let window = WindowsWindow::new(handle, options, self.generate_creation_info())?;
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
                let _ = tx.send(file_open_dialog(options));
            })
            .detach();

        rx
    }

    fn prompt_for_new_path(&self, directory: &Path) -> Receiver<Result<Option<PathBuf>>> {
        let directory = directory.to_owned();
        let (tx, rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                let _ = tx.send(file_save_dialog(directory));
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

    fn open_with_system(&self, path: &Path) {
        let Ok(full_path) = path.canonicalize() else {
            log::error!("unable to parse file full path: {}", path.display());
            return;
        };
        self.background_executor()
            .spawn(async move {
                let Some(full_path_str) = full_path.to_str() else {
                    return;
                };
                if full_path_str.is_empty() {
                    return;
                };
                open_target(full_path_str);
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
    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}
    fn set_dock_menu(&self, _menus: Vec<MenuItem>, _keymap: &Keymap) {}

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
    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow!("not yet implemented"))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        let hcursor = load_cursor(style);
        let mut lock = self.state.borrow_mut();
        if lock.current_cursor.0 != hcursor.0 {
            self.post_message(CURSOR_STYLE_CHANGED, WPARAM(0), LPARAM(hcursor.0 as isize));
            lock.current_cursor = hcursor;
        }
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        should_auto_hide_scrollbars().log_err().unwrap_or(false)
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        write_to_clipboard(item);
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        read_from_clipboard()
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
                unsafe { CredFree(credentials as *const _ as _) };
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
        unsafe {
            ManuallyDrop::drop(&mut self.bitmap_factory);
            OleUninitialize();
        }
    }
}

pub(crate) struct WindowCreationInfo {
    pub(crate) icon: HICON,
    pub(crate) executor: ForegroundExecutor,
    pub(crate) current_cursor: HCURSOR,
    pub(crate) windows_version: WindowsVersion,
    pub(crate) validation_number: usize,
    pub(crate) main_receiver: flume::Receiver<Runnable>,
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
        if ret.0 as isize <= 32 {
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
        if ret.0 as isize <= 32 {
            log::error!(
                "Unable to open target in explorer: {}",
                std::io::Error::last_os_error()
            );
        }
    }
}

fn file_open_dialog(options: PathPromptOptions) -> Result<Option<Vec<PathBuf>>> {
    let folder_dialog: IFileOpenDialog =
        unsafe { CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL)? };

    let mut dialog_options = FOS_FILEMUSTEXIST;
    if options.multiple {
        dialog_options |= FOS_ALLOWMULTISELECT;
    }
    if options.directories {
        dialog_options |= FOS_PICKFOLDERS;
    }

    unsafe {
        folder_dialog.SetOptions(dialog_options)?;
        if folder_dialog.Show(None).is_err() {
            // User cancelled
            return Ok(None);
        }
    }

    let results = unsafe { folder_dialog.GetResults()? };
    let file_count = unsafe { results.GetCount()? };
    if file_count == 0 {
        return Ok(None);
    }

    let mut paths = Vec::new();
    for i in 0..file_count {
        let item = unsafe { results.GetItemAt(i)? };
        let path = unsafe { item.GetDisplayName(SIGDN_FILESYSPATH)?.to_string()? };
        paths.push(PathBuf::from(path));
    }

    Ok(Some(paths))
}

fn file_save_dialog(directory: PathBuf) -> Result<Option<PathBuf>> {
    let dialog: IFileSaveDialog = unsafe { CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL)? };
    if !directory.to_string_lossy().is_empty() {
        if let Some(full_path) = directory.canonicalize().log_err() {
            let full_path = full_path.to_string_lossy();
            let full_path_str = full_path.trim_start_matches("\\\\?\\");
            if !full_path_str.is_empty() {
                let path_item: IShellItem =
                    unsafe { SHCreateItemFromParsingName(&HSTRING::from(full_path_str), None)? };
                unsafe { dialog.SetFolder(&path_item).log_err() };
            }
        }
    }
    unsafe {
        dialog.SetFileTypes(&[Common::COMDLG_FILTERSPEC {
            pszName: windows::core::w!("All files"),
            pszSpec: windows::core::w!("*.*"),
        }])?;
        if dialog.Show(None).is_err() {
            // User cancelled
            return Ok(None);
        }
    }
    let shell_item = unsafe { dialog.GetResult()? };
    let file_path_string = unsafe {
        let pwstr = shell_item.GetDisplayName(SIGDN_FILESYSPATH)?;
        let string = pwstr.to_string()?;
        CoTaskMemFree(Some(pwstr.0 as _));
        string
    };
    Ok(Some(PathBuf::from(file_path_string)))
}

fn begin_vsync(vsync_event: HANDLE) {
    let event: SafeHandle = vsync_event.into();
    std::thread::spawn(move || unsafe {
        loop {
            windows::Win32::Graphics::Dwm::DwmFlush().log_err();
            SetEvent(*event).log_err();
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

#[cfg(test)]
mod tests {
    use crate::{ClipboardItem, Platform, WindowsPlatform};

    #[test]
    fn test_clipboard() {
        let platform = WindowsPlatform::new();
        let item = ClipboardItem::new_string("你好".to_string());
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));

        let item = ClipboardItem::new_string("12345".to_string());
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));

        let item = ClipboardItem::new_string_with_json_metadata("abcdef".to_string(), vec![3, 4]);
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));
    }
}
