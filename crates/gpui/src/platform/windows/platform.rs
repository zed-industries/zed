use std::{
    cell::RefCell,
    mem::ManuallyDrop,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use ::util::{ResultExt, paths::SanitizedPath};
use anyhow::{Context as _, Result, anyhow};
use async_task::Runnable;
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use parking_lot::RwLock;
use smallvec::SmallVec;
use windows::{
    UI::ViewManagement::UISettings,
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
    core::*,
};

use crate::{platform::blade::BladeContext, *};

pub(crate) struct WindowsPlatform {
    state: RefCell<WindowsPlatformState>,
    raw_window_handles: RwLock<SmallVec<[HWND; 4]>>,
    gpu_context: BladeContext,
    // The below members will never change throughout the entire lifecycle of the app.
    icon: HICON,
    main_receiver: flume::Receiver<Runnable>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<DirectWriteTextSystem>,
    windows_version: WindowsVersion,
    bitmap_factory: ManuallyDrop<IWICImagingFactory>,
    validation_number: usize,
    main_thread_id_win32: u32,
}

pub(crate) struct WindowsPlatformState {
    callbacks: PlatformCallbacks,
    menus: Vec<OwnedMenu>,
    jump_list: JumpList,
    // NOTE: standard cursor handles don't need to close.
    pub(crate) current_cursor: Option<HCURSOR>,
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
        let jump_list = JumpList::new();
        let current_cursor = load_cursor(CursorStyle::Arrow);

        Self {
            callbacks,
            jump_list,
            current_cursor,
            menus: Vec::new(),
        }
    }
}

impl WindowsPlatform {
    pub(crate) fn new() -> Self {
        unsafe {
            OleInitialize(None).expect("unable to initialize Windows OLE");
        }
        let (main_sender, main_receiver) = flume::unbounded::<Runnable>();
        let main_thread_id_win32 = unsafe { GetCurrentThreadId() };
        let validation_number = rand::random::<usize>();
        let dispatcher = Arc::new(WindowsDispatcher::new(
            main_sender,
            main_thread_id_win32,
            validation_number,
        ));
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
        let gpu_context = BladeContext::new().expect("Unable to init GPU context");
        let windows_version = WindowsVersion::new().expect("Error retrieve windows version");

        Self {
            state,
            raw_window_handles,
            gpu_context,
            icon,
            main_receiver,
            background_executor,
            foreground_executor,
            text_system,
            windows_version,
            bitmap_factory,
            validation_number,
            main_thread_id_win32,
        }
    }

    fn redraw_all(&self) {
        for handle in self.raw_window_handles.read().iter() {
            unsafe {
                RedrawWindow(Some(*handle), None, None, RDW_INVALIDATE | RDW_UPDATENOW)
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
                PostMessageW(Some(*handle), message, wparam, lparam).log_err();
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

    #[inline]
    fn run_foreground_task(&self) {
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
            main_thread_id_win32: self.main_thread_id_win32,
        }
    }

    fn handle_dock_action_event(&self, action_idx: usize) {
        let mut lock = self.state.borrow_mut();
        if let Some(mut callback) = lock.callbacks.app_menu_action.take() {
            let Some(action) = lock
                .jump_list
                .dock_menus
                .get(action_idx)
                .map(|dock_menu| dock_menu.action.boxed_clone())
            else {
                lock.callbacks.app_menu_action = Some(callback);
                log::error!("Dock menu for index {action_idx} not found");
                return;
            };
            drop(lock);
            callback(&*action);
            self.state.borrow_mut().callbacks.app_menu_action = Some(callback);
        }
    }

    // Returns true if the app should quit.
    fn handle_events(&self) -> bool {
        let mut msg = MSG::default();
        unsafe {
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                match msg.message {
                    WM_QUIT => return true,
                    WM_GPUI_CLOSE_ONE_WINDOW
                    | WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD
                    | WM_GPUI_DOCK_MENU_ACTION => {
                        if self.handle_gpui_evnets(msg.message, msg.wParam, msg.lParam, &msg) {
                            return true;
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
        false
    }

    // Returns true if the app should quit.
    fn handle_gpui_evnets(
        &self,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        msg: *const MSG,
    ) -> bool {
        if wparam.0 != self.validation_number {
            unsafe { DispatchMessageW(msg) };
            return false;
        }
        match message {
            WM_GPUI_CLOSE_ONE_WINDOW => {
                if self.close_one_window(HWND(lparam.0 as _)) {
                    return true;
                }
            }
            WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD => self.run_foreground_task(),
            WM_GPUI_DOCK_MENU_ACTION => self.handle_dock_action_event(lparam.0 as _),
            _ => unreachable!(),
        }
        false
    }

    fn set_dock_menus(&self, menus: Vec<MenuItem>) {
        let mut actions = Vec::new();
        menus.into_iter().for_each(|menu| {
            if let Some(dock_menu) = DockMenuItem::new(menu).log_err() {
                actions.push(dock_menu);
            }
        });
        let mut lock = self.state.borrow_mut();
        lock.jump_list.dock_menus = actions;
        update_jump_list(&lock.jump_list).log_err();
    }

    fn update_jump_list(
        &self,
        menus: Vec<MenuItem>,
        entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Vec<SmallVec<[PathBuf; 2]>> {
        let mut actions = Vec::new();
        menus.into_iter().for_each(|menu| {
            if let Some(dock_menu) = DockMenuItem::new(menu).log_err() {
                actions.push(dock_menu);
            }
        });
        let mut lock = self.state.borrow_mut();
        lock.jump_list.dock_menus = actions;
        lock.jump_list.recent_workspaces = entries;
        update_jump_list(&lock.jump_list)
            .log_err()
            .unwrap_or_default()
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

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(
            WindowsKeyboardLayout::new()
                .log_err()
                .unwrap_or(WindowsKeyboardLayout::unknown()),
        )
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {
        // todo(windows)
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
                WAIT_EVENT(0) => self.redraw_all(),
                // Windows thread messages are posted
                WAIT_EVENT(1) => {
                    if self.handle_events() {
                        break 'a;
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
        let restart_process = util::command::new_std_command("powershell.exe")
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

    fn is_screen_capture_supported(&self) -> bool {
        false
    }

    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Box<dyn ScreenCaptureSource>>>> {
        let (mut tx, rx) = oneshot::channel();
        tx.send(Err(anyhow!("screen capture not implemented"))).ok();
        rx
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
        let window = WindowsWindow::new(
            handle,
            options,
            self.generate_creation_info(),
            &self.gpu_context,
        )?;
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

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        // The FOS_PICKFOLDERS flag toggles between "only files" and "only folders".
        false
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

    fn set_menus(&self, menus: Vec<Menu>, _keymap: &Keymap) {
        self.state.borrow_mut().menus = menus.into_iter().map(|menu| menu.owned()).collect();
    }

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        Some(self.state.borrow().menus.clone())
    }

    fn set_dock_menu(&self, menus: Vec<MenuItem>, _keymap: &Keymap) {
        self.set_dock_menus(menus);
    }

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
        if lock.current_cursor.map(|c| c.0) != hcursor.map(|c| c.0) {
            self.post_message(
                WM_GPUI_CURSOR_STYLE_CHANGED,
                WPARAM(0),
                LPARAM(hcursor.map_or(0, |c| c.0 as isize)),
            );
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
                    None,
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
            unsafe {
                CredDeleteW(
                    PCWSTR::from_raw(target_name.as_ptr()),
                    CRED_TYPE_GENERIC,
                    None,
                )?
            };
            Ok(())
        })
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        Task::ready(Err(anyhow!("register_url_scheme unimplemented")))
    }

    fn perform_dock_menu_action(&self, action: usize) {
        unsafe {
            PostThreadMessageW(
                self.main_thread_id_win32,
                WM_GPUI_DOCK_MENU_ACTION,
                WPARAM(self.validation_number),
                LPARAM(action as isize),
            )
            .log_err();
        }
    }

    fn update_jump_list(
        &self,
        menus: Vec<MenuItem>,
        entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Vec<SmallVec<[PathBuf; 2]>> {
        self.update_jump_list(menus, entries)
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
    pub(crate) current_cursor: Option<HCURSOR>,
    pub(crate) windows_version: WindowsVersion,
    pub(crate) validation_number: usize,
    pub(crate) main_receiver: flume::Receiver<Runnable>,
    pub(crate) main_thread_id_win32: u32,
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
            let full_path = SanitizedPath::from(full_path);
            let full_path_string = full_path.to_string();
            let path_item: IShellItem =
                unsafe { SHCreateItemFromParsingName(&HSTRING::from(full_path_string), None)? };
            unsafe { dialog.SetFolder(&path_item).log_err() };
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
            Some(module.into()),
            windows::core::PCWSTR(1 as _),
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
    use crate::{ClipboardItem, read_from_clipboard, write_to_clipboard};

    #[test]
    fn test_clipboard() {
        let item = ClipboardItem::new_string("你好，我是张小白".to_string());
        write_to_clipboard(item.clone());
        assert_eq!(read_from_clipboard(), Some(item));

        let item = ClipboardItem::new_string("12345".to_string());
        write_to_clipboard(item.clone());
        assert_eq!(read_from_clipboard(), Some(item));

        let item = ClipboardItem::new_string_with_json_metadata("abcdef".to_string(), vec![3, 4]);
        write_to_clipboard(item.clone());
        assert_eq!(read_from_clipboard(), Some(item));
    }
}
