// todo!("windows"): remove
#![allow(unused_variables)]

use std::{
    cell::RefCell,
    collections::HashSet,
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
use util::SemanticVersion;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, HWND},
    System::Threading::{CreateEventW, INFINITE},
    UI::WindowsAndMessaging::{
        DispatchMessageW, MsgWaitForMultipleObjects, PeekMessageW, PostQuitMessage,
        TranslateMessage, MSG, PM_REMOVE, QS_ALLINPUT, WM_QUIT,
    },
};

use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, ForegroundExecutor,
    Keymap, Menu, PathPromptOptions, Platform, PlatformDisplay, PlatformInput, PlatformTextSystem,
    PlatformWindow, Task, WindowAppearance, WindowOptions, WindowsDispatcher, WindowsDisplay,
    WindowsTextSystem, WindowsWindow,
};

pub(crate) struct WindowsPlatform {
    inner: Rc<WindowsPlatformInner>,
}

pub(crate) struct WindowsPlatformInner {
    background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
    main_receiver: flume::Receiver<Runnable>,
    text_system: Arc<WindowsTextSystem>,
    callbacks: Mutex<Callbacks>,
    pub(crate) window_handles: RefCell<HashSet<AnyWindowHandle>>,
    pub(crate) event: HANDLE,
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
        let inner = Rc::new(WindowsPlatformInner {
            background_executor,
            foreground_executor,
            main_receiver,
            text_system,
            callbacks,
            window_handles,
            event,
        });
        Self { inner }
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
        'a: loop {
            unsafe {
                MsgWaitForMultipleObjects(Some(&[self.inner.event]), false, INFINITE, QS_ALLINPUT)
            };
            let mut msg = MSG::default();
            while unsafe { PeekMessageW(&mut msg, HWND::default(), 0, 0, PM_REMOVE) }.as_bool() {
                if msg.message == WM_QUIT {
                    break 'a;
                }
                unsafe { TranslateMessage(&msg) };
                unsafe { DispatchMessageW(&msg) };
            }
            while let Ok(runnable) = self.inner.main_receiver.try_recv() {
                runnable.run();
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
        // todo!("windows")
    }

    // todo!("windows")
    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.inner.callbacks.lock().open_urls = Some(callback);
    }

    // todo!("windows")
    fn prompt_for_paths(&self, options: PathPromptOptions) -> Receiver<Option<Vec<PathBuf>>> {
        unimplemented!()
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
}
