// todo(windows): remove
#![allow(unused_variables)]

use std::{
    cell::RefCell,
    collections::HashSet,
    ffi::{c_uint, c_void},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use anyhow::{anyhow, Result};
use async_task::Runnable;
use futures::channel::oneshot::Receiver;
use parking_lot::Mutex;
use time::UtcOffset;
use util::{ResultExt, SemanticVersion};
use windows::{
    core::{HSTRING, PCWSTR},
    Wdk::System::SystemServices::RtlGetVersion,
    Win32::{
        Foundation::{CloseHandle, BOOL, HANDLE, HWND, LPARAM, TRUE},
        Graphics::DirectComposition::DCompositionWaitForCompositorClock,
        System::{
            Threading::{CreateEventW, GetCurrentThreadId, INFINITE},
            Time::{GetTimeZoneInformation, TIME_ZONE_ID_INVALID},
        },
        UI::{
            Input::KeyboardAndMouse::GetDoubleClickTime,
            Shell::ShellExecuteW,
            WindowsAndMessaging::{
                DispatchMessageW, EnumThreadWindows, LoadImageW, MsgWaitForMultipleObjects,
                PeekMessageW, PostQuitMessage, SetCursor, SystemParametersInfoW, TranslateMessage,
                HCURSOR, IDC_ARROW, IDC_CROSS, IDC_HAND, IDC_IBEAM, IDC_NO, IDC_SIZENS, IDC_SIZEWE,
                IMAGE_CURSOR, LR_DEFAULTSIZE, LR_SHARED, MSG, PM_REMOVE, QS_ALLINPUT,
                SPI_GETWHEELSCROLLCHARS, SPI_GETWHEELSCROLLLINES, SW_SHOWDEFAULT,
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, WM_QUIT, WM_SETTINGCHANGE,
            },
        },
    },
};
use copypasta::windows_clipboard::WindowsClipboardContext;
use copypasta::ClipboardProvider;

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
    clipboard: Rc<RefCell<WindowsClipboardContext>>,
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
        let clipboard = Rc::new(RefCell::new(WindowsClipboardContext::new().unwrap()));
        let inner = Rc::new(WindowsPlatformInner {
            background_executor,
            foreground_executor,
            main_receiver,
            text_system,
            callbacks,
            window_handles,
            event,
            settings,
            clipboard
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

    fn run_foreground_tasks(&self) {
        for runnable in self.inner.main_receiver.drain() {
            runnable.run();
        }
    }
}

unsafe extern "system" fn invalidate_window_callback(hwnd: HWND, _: LPARAM) -> BOOL {
    if let Some(inner) = try_get_window_inner(hwnd) {
        inner.invalidate_client_area();
    }
    TRUE
}

/// invalidates all windows belonging to a thread causing a paint message to be scheduled
fn invalidate_thread_windows(win32_thread_id: u32) {
    unsafe {
        EnumThreadWindows(
            win32_thread_id,
            Some(invalidate_window_callback),
            LPARAM::default(),
        )
    };
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
        'a: loop {
            let mut msg = MSG::default();
            // will be 0 if woken up by self.inner.event or 1 if the compositor clock ticked
            // SEE: https://learn.microsoft.com/en-us/windows/win32/directcomp/compositor-clock/compositor-clock
            let wait_result =
                unsafe { DCompositionWaitForCompositorClock(Some(&[self.inner.event]), INFINITE) };

            // compositor clock ticked so we should draw a frame
            if wait_result == 1 {
                unsafe { invalidate_thread_windows(GetCurrentThreadId()) };

                while unsafe { PeekMessageW(&mut msg, HWND::default(), 0, 0, PM_REMOVE) }.as_bool()
                {
                    if msg.message == WM_QUIT {
                        break 'a;
                    }

                    if !self.run_immediate_msg_handlers(&msg) {
                        unsafe { TranslateMessage(&msg) };
                        unsafe { DispatchMessageW(&msg) };
                    }
                }
            }

            self.run_foreground_tasks();
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

    // todo(windows)
    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![Rc::new(WindowsDisplay::new())]
    }

    // todo(windows)
    fn display(&self, id: crate::DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(WindowsDisplay::new()))
    }

    // todo(windows)
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

    // todo(windows)
    fn prompt_for_paths(&self, options: PathPromptOptions) -> Receiver<Option<Vec<PathBuf>>> {
        unimplemented!()
    }

    // todo(windows)
    fn prompt_for_new_path(&self, directory: &Path) -> Receiver<Option<PathBuf>> {
        unimplemented!()
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
        Ok(SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        })
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

    fn double_click_interval(&self) -> Duration {
        let millis = unsafe { GetDoubleClickTime() };
        Duration::from_millis(millis as _)
    }

    // todo(windows)
    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        Err(anyhow!("not yet implemented"))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        let handle = match style {
            CursorStyle::IBeam | CursorStyle::IBeamCursorForVerticalLayout => unsafe {
                load_cursor(IDC_IBEAM)
            },
            CursorStyle::Crosshair => unsafe { load_cursor(IDC_CROSS) },
            CursorStyle::PointingHand | CursorStyle::DragLink => unsafe { load_cursor(IDC_HAND) },
            CursorStyle::ResizeLeft | CursorStyle::ResizeRight | CursorStyle::ResizeLeftRight => unsafe {
                load_cursor(IDC_SIZEWE)
            },
            CursorStyle::ResizeUp | CursorStyle::ResizeDown | CursorStyle::ResizeUpDown => unsafe {
                load_cursor(IDC_SIZENS)
            },
            CursorStyle::OperationNotAllowed => unsafe { load_cursor(IDC_NO) },
            _ => unsafe { load_cursor(IDC_ARROW) },
        };
        if handle.is_err() {
            log::error!(
                "Error loading cursor image: {}",
                std::io::Error::last_os_error()
            );
            return;
        }
        let _ = unsafe { SetCursor(HCURSOR(handle.unwrap().0)) };
    }

    // todo(windows)
    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    // todo(windows)
    fn write_to_clipboard(&self, item: ClipboardItem) {
        if item.text.len() > 0{
            self.inner.clipboard.borrow_mut().set_contents(item.text).unwrap();
        }
    }

    // todo(windows)
    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let contents = self.inner.clipboard.borrow_mut().get_contents();
        match contents {
            Ok(text) => Some(ClipboardItem {
                metadata: None,
                text,
            }),
            _ => None,
        }
    }

    // todo(windows)
    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        Task::Ready(Some(Err(anyhow!("not implemented yet."))))
    }

    // todo(windows)
    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::Ready(Some(Err(anyhow!("not implemented yet."))))
    }

    // todo(windows)
    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        Task::Ready(Some(Err(anyhow!("not implemented yet."))))
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        Task::ready(Err(anyhow!("register_url_scheme unimplemented")))
    }
}

unsafe fn load_cursor(name: PCWSTR) -> Result<HANDLE> {
    LoadImageW(None, name, IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE | LR_SHARED).map_err(|e| anyhow!(e))
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
