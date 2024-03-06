// todo!("windows"): remove
#![allow(unused_variables)]

use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    ffi::{c_uint, c_void, OsString},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use anyhow::{anyhow, Result};
use async_task::Runnable;
use futures::channel::oneshot::{self, Receiver};
use parking_lot::Mutex;
use time::UtcOffset;
use util::{ResultExt, SemanticVersion};
use windows::{core::{IUnknown, HRESULT, HSTRING}, Win32::{
    Foundation::{CloseHandle, HANDLE, HWND, WAIT_EVENT},
    System::{
        Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE},
        Threading::{CreateEventW, INFINITE}
    },
    UI::{
        Shell::{
            FileOpenDialog, IFileOpenDialog, IShellItem, FILEOPENDIALOGOPTIONS, SIGDN
        },
        WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, MsgWaitForMultipleObjects, PostQuitMessage,
            SystemParametersInfoW, TranslateMessage, MSG, QS_ALLINPUT, SPI_GETWHEELSCROLLCHARS,
            SPI_GETWHEELSCROLLLINES, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, WM_QUIT, WM_SETTINGCHANGE,
        }
    }
}};

use crate::{
    try_get_window_inner, Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle,
    ForegroundExecutor, Keymap, Menu, PathPromptOptions, Platform, PlatformDisplay, PlatformInput,
    PlatformTextSystem, PlatformWindow, Task, WindowAppearance, WindowOptions, WindowsDispatcher,
    WindowsDisplay, WindowsTextSystem, WindowsWindow,
};

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
    pub(crate) window_handles: RefCell<HashSet<AnyWindowHandle>>,
    pub(crate) event: HANDLE,
    pub(crate) settings: RefCell<WindowsPlatformSystemSettings>,
}

impl Drop for WindowsPlatformInner {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.event) }.ok();
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
        let (main_sender, main_receiver) = flume::unbounded::<Runnable>();
        let event = unsafe { CreateEventW(None, false, false, None) }.unwrap();
        let dispatcher = Arc::new(WindowsDispatcher::new(main_sender, event));
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);
        let text_system = Arc::new(WindowsTextSystem::new());
        let callbacks = Mutex::new(Callbacks::default());
        let window_handles = RefCell::new(HashSet::new());
        let settings = RefCell::new(WindowsPlatformSystemSettings::new());
        let inner = Rc::new(WindowsPlatformInner {
            background_executor,
            foreground_executor,
            main_receiver,
            text_system,
            callbacks,
            window_handles,
            event,
            settings,
        });
        Self { inner }
    }

    /// runs message handlers that should be processed before dispatching to prevent translating unnecessary messages
    /// returns true if message is handled and should not dispatch
    fn run_immediate_msg_handlers(&self, msg: &MSG) -> bool {
        if msg.message == WM_SETTINGCHANGE {
            self.inner.settings.borrow_mut().update_all();
            return true;
        }

        if let Some(inner) = try_get_window_inner(msg.hwnd) {
            inner.handle_immediate_msg(msg.message, msg.wParam, msg.lParam)
        } else {
            false
        }
    }

    fn wait_message(&self) -> WindowsMessageWaitResult {
        let wait_result = unsafe {
            MsgWaitForMultipleObjects(Some(&[self.inner.event]), false, INFINITE, QS_ALLINPUT)
        };

        match wait_result {
            WAIT_EVENT(0) => WindowsMessageWaitResult::ForegroundExecution,
            WAIT_EVENT(1) => {
                let mut msg = MSG::default();
                unsafe { GetMessageW(&mut msg, HWND::default(), 0, 0) };
                WindowsMessageWaitResult::WindowsMessage(msg)
            }
            _ => {
                log::error!("unhandled windows wait message: {}", wait_result.0);
                WindowsMessageWaitResult::Error
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
        loop {
            match self.wait_message() {
                WindowsMessageWaitResult::ForegroundExecution => {
                    for runnable in self.inner.main_receiver.drain() {
                        runnable.run();
                    }
                }
                WindowsMessageWaitResult::WindowsMessage(msg) => {
                    if msg.message == WM_QUIT {
                        break;
                    }

                    if !self.run_immediate_msg_handlers(&msg) {
                        unsafe { TranslateMessage(&msg) };
                        unsafe { DispatchMessageW(&msg) };
                    }
                }
                WindowsMessageWaitResult::Error => {}
            }
        }

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

    // todo!("windows")
    fn restart(&self) {
        unimplemented!()
    }

    // todo!("windows")
    fn activate(&self, ignoring_other_apps: bool) {}

    // todo!("windows")
    fn hide(&self) {
        unimplemented!()
    }

    // todo!("windows")
    fn hide_other_apps(&self) {
        unimplemented!()
    }

    // todo!("windows")
    fn unhide_other_apps(&self) {
        unimplemented!()
    }

    // todo!("windows")
    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![Rc::new(WindowsDisplay::new())]
    }

    // todo!("windows")
    fn display(&self, id: crate::DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(WindowsDisplay::new()))
    }

    // todo!("windows")
    fn active_window(&self) -> Option<AnyWindowHandle> {
        unimplemented!()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        Box::new(WindowsWindow::new(self.inner.clone(), handle, options))
    }

    // todo!("windows")
    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    // todo!("windows")
    fn open_url(&self, url: &str) {
        unimplemented!()
    }

    // todo!("windows")
    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.inner.callbacks.lock().open_urls = Some(callback);
    }

    #[allow(overflowing_literals)]
    fn prompt_for_paths(&self, options: PathPromptOptions) -> Receiver<Option<Vec<PathBuf>>> {
        println!("prompt_for_paths {:?}", options);

        let (tx, rx) = oneshot::channel();

        self.foreground_executor().spawn(async move {
            // initialize COM
            let hr = unsafe { CoInitializeEx(Some(std::ptr::null_mut()), COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE) };
            if hr.is_err() { panic!("CoInitializeEx failed: {:?}", hr); }

            let tx = Cell::new(Some(tx));

            // create file open dialog
            let folder_dialog: IFileOpenDialog = unsafe {
                CoCreateInstance::<std::option::Option<&IUnknown>, IFileOpenDialog>(
                    &FileOpenDialog,
                    None,
                    CLSCTX_ALL
                ).unwrap()
            };

            // dialog options (https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-_fileopendialogoptions)
            let mut dialog_options: u32 = 0x1000; // FOS_FILEMUSTEXIST

            if options.multiple {
                dialog_options |= 0x200; // FOS_ALLOWMULTISELECT
            }

            if options.directories {
                dialog_options |= 0x20; // FOS_PICKFOLDERS
            }

            unsafe {
                folder_dialog.SetOptions(FILEOPENDIALOGOPTIONS(dialog_options)).unwrap(); // FOS_PICKFOLDERS
                folder_dialog.SetTitle(&HSTRING::from(OsString::from("Select a folder"))).unwrap();
            }

            let hr = unsafe { folder_dialog.Show(None) };

            if hr.is_err() {
                if hr.unwrap_err().code() == HRESULT(0x800704C7) { // user canceled error (we don't want to crash)
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
                let mut path = unsafe { item.GetDisplayName(SIGDN(0x80058000)).unwrap() }; // SIGDN_FILESYSPATH

                paths.push(PathBuf::from(unsafe { path.to_string().unwrap() }));
            }


            if let Some(tx) = tx.take() {
                if paths.len() == 0 {
                    tx.send(None).unwrap();
                } else {
                    println!("paths: {:?}", paths);
                    tx.send(Some(paths)).unwrap();
                }
            }

            // uninitialize COM
            unsafe {
                CoUninitialize();
            }

        }).detach();

        rx
    }

    // todo!("windows")
    fn prompt_for_new_path(&self, directory: &Path) -> Receiver<Option<PathBuf>> {
        unimplemented!()
    }

    // todo!("windows")
    fn reveal_path(&self, path: &Path) {
        unimplemented!()
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

    // todo!("windows")
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
        Ok(SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        })
    }

    fn app_version(&self) -> Result<SemanticVersion> {
        Ok(SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        })
    }

    // todo!("windows")
    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow!("not yet implemented"))
    }

    // todo!("windows")
    fn local_timezone(&self) -> UtcOffset {
        UtcOffset::from_hms(9, 0, 0).unwrap()
    }

    // todo!("windows")
    fn double_click_interval(&self) -> Duration {
        Duration::from_millis(100)
    }

    // todo!("windows")
    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        Err(anyhow!("not yet implemented"))
    }

    // todo!("windows")
    fn set_cursor_style(&self, style: CursorStyle) {}

    // todo!("windows")
    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    // todo!("windows")
    fn write_to_clipboard(&self, item: ClipboardItem) {
        unimplemented!()
    }

    // todo!("windows")
    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        unimplemented!()
    }

    // todo!("windows")
    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        Task::Ready(Some(Err(anyhow!("not implemented yet."))))
    }

    // todo!("windows")
    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::Ready(Some(Err(anyhow!("not implemented yet."))))
    }

    // todo!("windows")
    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        Task::Ready(Some(Err(anyhow!("not implemented yet."))))
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        Task::ready(Err(anyhow!("register_url_scheme unimplemented")))
    }
}
