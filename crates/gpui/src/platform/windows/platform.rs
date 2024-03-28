// todo(windows): remove
#![allow(unused_variables)]

use std::{
    cell::{Cell, RefCell},
    ffi::{c_uint, c_void, OsString},
    iter::once,
    mem::transmute,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, OnceLock},
};

use ::util::{ResultExt, SemanticVersion};
use anyhow::{anyhow, Context, Result};
use async_task::Runnable;
use copypasta::{ClipboardContext, ClipboardProvider};
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use parking_lot::{Mutex, RwLock};
use smallvec::SmallVec;
use time::UtcOffset;
use windows::{
    core::*,
    Wdk::System::SystemServices::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        Media::*,
        Security::Credentials::*,
        Storage::FileSystem::*,
        System::{Com::*, LibraryLoader::*, Ole::*, SystemInformation::*, Threading::*, Time::*},
        UI::{Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
};

use crate::*;

pub(crate) struct WindowsPlatform {
    inner: Rc<WindowsPlatformInner>,
}

/// Windows settings pulled from SystemParametersInfo
/// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow
#[derive(Default, Debug)]
pub(crate) struct WindowsPlatformSystemSettings {
    /// SEE: SPI_GETWHEELSCROLLCHARS
    pub(crate) wheel_scroll_chars: u32,

    /// SEE: SPI_GETWHEELSCROLLLINES
    pub(crate) wheel_scroll_lines: u32,
}

pub(crate) struct WindowsPlatformInner {
    background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
    main_receiver: flume::Receiver<Runnable>,
    text_system: Arc<WindowsTextSystem>,
    callbacks: Mutex<Callbacks>,
    pub raw_window_handles: RwLock<SmallVec<[HWND; 4]>>,
    pub(crate) dispatch_event: OwnedHandle,
    pub(crate) settings: RefCell<WindowsPlatformSystemSettings>,
    pub icon: HICON,
    // NOTE: standard cursor handles don't need to close.
    pub(crate) current_cursor: Cell<HCURSOR>,
}

impl WindowsPlatformInner {
    pub(crate) fn try_get_windows_inner_from_hwnd(
        &self,
        hwnd: HWND,
    ) -> Option<Rc<WindowsWindowInner>> {
        self.raw_window_handles
            .read()
            .iter()
            .find(|entry| *entry == &hwnd)
            .and_then(|hwnd| try_get_window_inner(*hwnd))
    }
}

#[derive(Default)]
struct Callbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    become_active: Option<Box<dyn FnMut()>>,
    resign_active: Option<Box<dyn FnMut()>>,
    quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    event: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
}

enum WindowsMessageWaitResult {
    ForegroundExecution,
    WindowsMessage(MSG),
    Error,
}

impl WindowsPlatformSystemSettings {
    fn new() -> Self {
        let mut settings = Self::default();
        settings.update_all();
        settings
    }

    pub(crate) fn update_all(&mut self) {
        self.update_wheel_scroll_lines();
        self.update_wheel_scroll_chars();
    }

    pub(crate) fn update_wheel_scroll_lines(&mut self) {
        let mut value = c_uint::default();
        let result = unsafe {
            SystemParametersInfoW(
                SPI_GETWHEELSCROLLLINES,
                0,
                Some((&mut value) as *mut c_uint as *mut c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default(),
            )
        };

        if result.log_err() != None {
            self.wheel_scroll_lines = value;
        }
    }

    pub(crate) fn update_wheel_scroll_chars(&mut self) {
        let mut value = c_uint::default();
        let result = unsafe {
            SystemParametersInfoW(
                SPI_GETWHEELSCROLLCHARS,
                0,
                Some((&mut value) as *mut c_uint as *mut c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default(),
            )
        };

        if result.log_err() != None {
            self.wheel_scroll_chars = value;
        }
    }
}

impl WindowsPlatform {
    pub(crate) fn new() -> Self {
        unsafe {
            OleInitialize(None).expect("unable to initialize Windows OLE");
        }
        let (main_sender, main_receiver) = flume::unbounded::<Runnable>();
        let dispatch_event =
            OwnedHandle::new(unsafe { CreateEventW(None, false, false, None) }.unwrap());
        let dispatcher = Arc::new(WindowsDispatcher::new(main_sender, dispatch_event.to_raw()));
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);
        let text_system = Arc::new(WindowsTextSystem::new());
        let callbacks = Mutex::new(Callbacks::default());
        let raw_window_handles = RwLock::new(SmallVec::new());
        let settings = RefCell::new(WindowsPlatformSystemSettings::new());
        let icon = load_icon().unwrap_or_default();
        let current_cursor = Cell::new(load_cursor(CursorStyle::Arrow));
        let inner = Rc::new(WindowsPlatformInner {
            background_executor,
            foreground_executor,
            main_receiver,
            text_system,
            callbacks,
            raw_window_handles,
            dispatch_event,
            settings,
            icon,
            current_cursor,
        });
        Self { inner }
    }

    fn run_foreground_tasks(&self) {
        for runnable in self.inner.main_receiver.drain() {
            runnable.run();
        }
    }

    fn redraw_all(&self) {
        for handle in self.inner.raw_window_handles.read().iter() {
            unsafe {
                RedrawWindow(
                    *handle,
                    None,
                    HRGN::default(),
                    RDW_INVALIDATE | RDW_UPDATENOW,
                );
            }
        }
    }
}

impl Platform for WindowsPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.inner.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.inner.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.inner.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        on_finish_launching();
        let dispatch_event = self.inner.dispatch_event.to_raw();
        let vsync_event = create_event().unwrap();
        let timer_stop_event = create_event().unwrap();
        let raw_timer_stop_event = timer_stop_event.to_raw();
        begin_vsync_timer(vsync_event.to_raw(), timer_stop_event);
        'a: loop {
            let wait_result = unsafe {
                MsgWaitForMultipleObjects(
                    Some(&[vsync_event.to_raw(), dispatch_event]),
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
                // foreground tasks are dispatched
                WAIT_EVENT(1) => {
                    self.run_foreground_tasks();
                }
                // Windows thread messages are posted
                WAIT_EVENT(2) => {
                    let mut msg = MSG::default();
                    unsafe {
                        while PeekMessageW(&mut msg, HWND::default(), 0, 0, PM_REMOVE).as_bool() {
                            if msg.message == WM_QUIT {
                                break 'a;
                            }
                            if msg.message == WM_SETTINGCHANGE {
                                self.inner.settings.borrow_mut().update_all();
                                continue;
                            }
                            TranslateMessage(&msg);
                            DispatchMessageW(&msg);
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
        end_vsync_timer(raw_timer_stop_event);

        let mut callbacks = self.inner.callbacks.lock();
        if let Some(callback) = callbacks.quit.as_mut() {
            callback()
        }
    }

    fn quit(&self) {
        self.foreground_executor()
            .spawn(async { unsafe { PostQuitMessage(0) } })
            .detach();
    }

    // todo(windows)
    fn restart(&self) {
        unimplemented!()
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

    fn display(&self, id: crate::DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        if let Some(display) = WindowsDisplay::new(id) {
            Some(Rc::new(display) as Rc<dyn PlatformDisplay>)
        } else {
            None
        }
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        if let Some(display) = WindowsDisplay::primary_monitor() {
            Some(Rc::new(display) as Rc<dyn PlatformDisplay>)
        } else {
            None
        }
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        let active_window_hwnd = unsafe { GetActiveWindow() };
        self.inner
            .try_get_windows_inner_from_hwnd(active_window_hwnd)
            .map(|inner| inner.handle)
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        Box::new(WindowsWindow::new(self.inner.clone(), handle, options))
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

    // todo(windows)
    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.inner.callbacks.lock().open_urls = Some(callback);
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
                open_target(path);
            })
            .detach();
    }

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().become_active = Some(callback);
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().resign_active = Some(callback);
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().reopen = Some(callback);
    }

    fn on_event(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.inner.callbacks.lock().event = Some(callback);
    }

    // todo(windows)
    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {}

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.inner.callbacks.lock().app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.lock().will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.inner.callbacks.lock().validate_app_menu_command = Some(callback);
    }

    fn os_name(&self) -> &'static str {
        "Windows"
    }

    fn os_version(&self) -> Result<SemanticVersion> {
        let mut info = unsafe { std::mem::zeroed() };
        let status = unsafe { RtlGetVersion(&mut info) };
        if status.is_ok() {
            Ok(SemanticVersion {
                major: info.dwMajorVersion as _,
                minor: info.dwMinorVersion as _,
                patch: info.dwBuildNumber as _,
            })
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
            return Ok(SemanticVersion {
                major: ((version_info.dwProductVersionMS >> 16) & 0xFFFF) as usize,
                minor: (version_info.dwProductVersionMS & 0xFFFF) as usize,
                patch: ((version_info.dwProductVersionLS >> 16) & 0xFFFF) as usize,
            });
        } else {
            log::error!(
                "no version info present: {}",
                std::io::Error::last_os_error()
            );
            return Err(anyhow!("no version info present"));
        }
    }

    // todo(windows)
    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow!("not yet implemented"))
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
        self.inner.current_cursor.set(load_cursor(style));
    }

    // todo(windows)
    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(item.text().to_owned()).unwrap();
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let mut ctx = ClipboardContext::new().unwrap();
        let content = ctx.get_contents().unwrap();
        Some(ClipboardItem {
            text: content,
            metadata: None,
        })
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        let mut password = password.to_vec();
        let mut username = username.encode_utf16().chain(once(0)).collect_vec();
        let mut target_name = windows_credentials_target_name(url)
            .encode_utf16()
            .chain(once(0))
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
            .chain(once(0))
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
            .chain(once(0))
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

fn begin_vsync_timer(vsync_event: HANDLE, timer_stop_event: OwnedHandle) {
    let vsync_fn = select_vsync_fn();
    std::thread::spawn(move || {
        while vsync_fn(timer_stop_event.to_raw()) {
            if unsafe { SetEvent(vsync_event) }.log_err().is_none() {
                break;
            }
        }
    });
}

fn end_vsync_timer(timer_stop_event: HANDLE) {
    unsafe { SetEvent(timer_stop_event) }.log_err();
}

fn select_vsync_fn() -> Box<dyn Fn(HANDLE) -> bool + Send> {
    if let Some(dcomp_fn) = load_dcomp_vsync_fn() {
        log::info!("use DCompositionWaitForCompositorClock for vsync");
        return Box::new(move |timer_stop_event| {
            // will be 0 if woken up by timer_stop_event or 1 if the compositor clock ticked
            // SEE: https://learn.microsoft.com/en-us/windows/win32/directcomp/compositor-clock/compositor-clock
            (unsafe { dcomp_fn(1, &timer_stop_event, INFINITE) }) == 1
        });
    }
    log::info!("use fallback vsync function");
    Box::new(fallback_vsync_fn())
}

fn load_dcomp_vsync_fn() -> Option<unsafe extern "system" fn(u32, *const HANDLE, u32) -> u32> {
    static FN: OnceLock<Option<unsafe extern "system" fn(u32, *const HANDLE, u32) -> u32>> =
        OnceLock::new();
    *FN.get_or_init(|| {
        let hmodule = unsafe { LoadLibraryW(windows::core::w!("dcomp.dll")) }.ok()?;
        let address = unsafe {
            GetProcAddress(
                hmodule,
                windows::core::s!("DCompositionWaitForCompositorClock"),
            )
        }?;
        Some(unsafe { transmute(address) })
    })
}

fn fallback_vsync_fn() -> impl Fn(HANDLE) -> bool + Send {
    let freq = WindowsDisplay::primary_monitor()
        .and_then(|monitor| monitor.frequency())
        .unwrap_or(60);
    log::info!("primaly refresh rate is {freq}Hz");

    let interval = (1000 / freq).max(1);
    log::info!("expected interval is {interval}ms");

    unsafe { timeBeginPeriod(1) };

    struct TimePeriod;
    impl Drop for TimePeriod {
        fn drop(&mut self) {
            unsafe { timeEndPeriod(1) };
        }
    }
    let period = TimePeriod;

    move |timer_stop_event| {
        let _ = (&period,);
        (unsafe { WaitForSingleObject(timer_stop_event, interval) }) == WAIT_TIMEOUT
    }
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
