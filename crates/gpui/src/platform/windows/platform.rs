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
use copypasta::{ClipboardContext, ClipboardProvider};
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use parking_lot::RwLock;
use semantic_version::SemanticVersion;
use smallvec::SmallVec;
use time::UtcOffset;
use windows::{
    core::*,
    Wdk::System::SystemServices::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        Security::Credentials::*,
        Storage::FileSystem::*,
        System::{Com::*, LibraryLoader::*, Ole::*, SystemInformation::*, Threading::*, Time::*},
        UI::{Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
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

        Self {
            state,
            raw_window_handles,
            icon,
            background_executor,
            foreground_executor,
            text_system,
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
        let vsync_event = create_event().unwrap();
        begin_vsync(vsync_event.to_raw());
        'a: loop {
            let wait_result = unsafe {
                MsgWaitForMultipleObjects(
                    Some(&[vsync_event.to_raw()]),
                    false,
                    INFINITE,
                    QS_ALLINPUT,
                )
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
    ) -> Box<dyn PlatformWindow> {
        let lock = self.state.borrow();
        let window = WindowsWindow::new(
            handle,
            options,
            self.icon,
            self.foreground_executor.clone(),
            lock.current_cursor,
        );
        drop(lock);
        let handle = window.get_raw_handle();
        self.raw_window_handles.write().push(handle);

        Box::new(window)
    }

    // todo(windows)
    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
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

    fn prompt_for_paths(&self, options: PathPromptOptions) -> Receiver<Option<Vec<PathBuf>>> {
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
                            tx.send(None).unwrap();
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
                    if paths.len() == 0 {
                        tx.send(None).unwrap();
                    } else {
                        tx.send(Some(paths)).unwrap();
                    }
                }
            })
            .detach();

        rx
    }

    fn prompt_for_new_path(&self, directory: &Path) -> Receiver<Option<PathBuf>> {
        let directory = directory.to_owned();
        let (tx, rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                unsafe {
                    let Ok(dialog) = show_savefile_dialog(directory) else {
                        let _ = tx.send(None);
                        return;
                    };
                    let Ok(_) = dialog.Show(None) else {
                        let _ = tx.send(None); // user cancel
                        return;
                    };
                    if let Ok(shell_item) = dialog.GetResult() {
                        if let Ok(file) = shell_item.GetDisplayName(SIGDN_FILESYSPATH) {
                            let _ = tx.send(Some(PathBuf::from(file.to_string().unwrap())));
                            return;
                        }
                    }
                    let _ = tx.send(None);
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

    fn os_name(&self) -> &'static str {
        "Windows"
    }

    fn os_version(&self) -> Result<SemanticVersion> {
        let mut info = unsafe { std::mem::zeroed() };
        let status = unsafe { RtlGetVersion(&mut info) };
        if status.is_ok() {
            Ok(SemanticVersion::new(
                info.dwMajorVersion as _,
                info.dwMinorVersion as _,
                info.dwBuildNumber as _,
            ))
        } else {
            Err(anyhow::anyhow!(
                "unable to get Windows version: {}",
                std::io::Error::last_os_error()
            ))
        }
    }

    fn app_version(&self) -> Result<SemanticVersion> {
        let mut file_name_buffer = vec![0u16; MAX_PATH as usize];
        let file_name = {
            let mut file_name_buffer_capacity = MAX_PATH as usize;
            let mut file_name_length;
            loop {
                file_name_length =
                    unsafe { GetModuleFileNameW(None, &mut file_name_buffer) } as usize;
                if file_name_length < file_name_buffer_capacity {
                    break;
                }
                // buffer too small
                file_name_buffer_capacity *= 2;
                file_name_buffer = vec![0u16; file_name_buffer_capacity];
            }
            PCWSTR::from_raw(file_name_buffer[0..(file_name_length + 1)].as_ptr())
        };

        let version_info_block = {
            let mut version_handle = 0;
            let version_info_size =
                unsafe { GetFileVersionInfoSizeW(file_name, Some(&mut version_handle)) } as usize;
            if version_info_size == 0 {
                log::error!(
                    "unable to get version info size: {}",
                    std::io::Error::last_os_error()
                );
                return Err(anyhow!("unable to get version info size"));
            }
            let mut version_data = vec![0u8; version_info_size + 2];
            unsafe {
                GetFileVersionInfoW(
                    file_name,
                    version_handle,
                    version_info_size as u32,
                    version_data.as_mut_ptr() as _,
                )
            }
            .inspect_err(|_| {
                log::error!(
                    "unable to retrieve version info: {}",
                    std::io::Error::last_os_error()
                )
            })?;
            version_data
        };

        let version_info_raw = {
            let mut buffer = unsafe { std::mem::zeroed() };
            let mut size = 0;
            let entry = "\\".encode_utf16().chain(Some(0)).collect_vec();
            if !unsafe {
                VerQueryValueW(
                    version_info_block.as_ptr() as _,
                    PCWSTR::from_raw(entry.as_ptr()),
                    &mut buffer,
                    &mut size,
                )
            }
            .as_bool()
            {
                log::error!(
                    "unable to query version info data: {}",
                    std::io::Error::last_os_error()
                );
                return Err(anyhow!("the specified resource is not valid"));
            }
            if size == 0 {
                log::error!(
                    "unable to query version info data: {}",
                    std::io::Error::last_os_error()
                );
                return Err(anyhow!("no value is available for the specified name"));
            }
            buffer
        };

        let version_info = unsafe { &*(version_info_raw as *mut VS_FIXEDFILEINFO) };
        // https://learn.microsoft.com/en-us/windows/win32/api/verrsrc/ns-verrsrc-vs_fixedfileinfo
        if version_info.dwSignature == 0xFEEF04BD {
            return Ok(SemanticVersion::new(
                ((version_info.dwProductVersionMS >> 16) & 0xFFFF) as usize,
                (version_info.dwProductVersionMS & 0xFFFF) as usize,
                ((version_info.dwProductVersionLS >> 16) & 0xFFFF) as usize,
            ));
        } else {
            log::error!(
                "no version info present: {}",
                std::io::Error::last_os_error()
            );
            return Err(anyhow!("no version info present"));
        }
    }

    fn app_path(&self) -> Result<PathBuf> {
        Ok(std::env::current_exe()?)
    }

    fn local_timezone(&self) -> UtcOffset {
        let mut info = unsafe { std::mem::zeroed() };
        let ret = unsafe { GetTimeZoneInformation(&mut info) };
        if ret == TIME_ZONE_ID_INVALID {
            log::error!(
                "Unable to get local timezone: {}",
                std::io::Error::last_os_error()
            );
            return UtcOffset::UTC;
        }
        // Windows treat offset as:
        // UTC = localtime + offset
        // so we add a minus here
        let hours = -info.Bias / 60;
        let minutes = -info.Bias % 60;

        UtcOffset::from_hms(hours as _, minutes as _, 0).unwrap()
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

    // todo(windows)
    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        if item.text.len() > 0 {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(item.text().to_owned()).unwrap();
        }
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let mut ctx = ClipboardContext::new().unwrap();
        let content = ctx.get_contents().ok()?;
        Some(ClipboardItem {
            text: content,
            metadata: None,
        })
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
        unsafe {
            OleUninitialize();
        }
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
