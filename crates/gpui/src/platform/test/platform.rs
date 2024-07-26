use crate::{
    AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, ForegroundExecutor, Keymap,
    Platform, PlatformDisplay, PlatformTextSystem, Task, TestDisplay, TestWindow, WindowAppearance,
    WindowParams,
};
use anyhow::Result;
use collections::VecDeque;
use futures::channel::oneshot;
use parking_lot::Mutex;
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::Arc,
};
#[cfg(target_os = "windows")]
use windows::Win32::{
    Graphics::Imaging::{CLSID_WICImagingFactory, IWICImagingFactory},
    System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
};

/// TestPlatform implements the Platform trait for use in tests.
pub(crate) struct TestPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,

    pub(crate) active_window: RefCell<Option<TestWindow>>,
    active_display: Rc<dyn PlatformDisplay>,
    active_cursor: Mutex<CursorStyle>,
    current_clipboard_item: Mutex<Option<ClipboardItem>>,
    #[cfg(target_os = "linux")]
    current_primary_item: Mutex<Option<ClipboardItem>>,
    pub(crate) prompts: RefCell<TestPrompts>,
    pub opened_url: RefCell<Option<String>>,
    pub text_system: Arc<dyn PlatformTextSystem>,
    #[cfg(target_os = "windows")]
    bitmap_factory: std::mem::ManuallyDrop<IWICImagingFactory>,
    weak: Weak<Self>,
}

#[derive(Default)]
pub(crate) struct TestPrompts {
    multiple_choice: VecDeque<oneshot::Sender<usize>>,
    new_path: VecDeque<(PathBuf, oneshot::Sender<Result<Option<PathBuf>>>)>,
}

impl TestPlatform {
    pub fn new(executor: BackgroundExecutor, foreground_executor: ForegroundExecutor) -> Rc<Self> {
        #[cfg(target_os = "windows")]
        let bitmap_factory = unsafe {
            windows::Win32::System::Ole::OleInitialize(None)
                .expect("unable to initialize Windows OLE");
            std::mem::ManuallyDrop::new(
                CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)
                    .expect("Error creating bitmap factory."),
            )
        };

        #[cfg(target_os = "macos")]
        let text_system = Arc::new(crate::platform::mac::MacTextSystem::new());

        #[cfg(target_os = "linux")]
        let text_system = Arc::new(crate::platform::linux::CosmicTextSystem::new());

        #[cfg(target_os = "windows")]
        let text_system = Arc::new(
            crate::platform::windows::DirectWriteTextSystem::new(&bitmap_factory)
                .expect("Unable to initialize direct write."),
        );

        Rc::new_cyclic(|weak| TestPlatform {
            background_executor: executor,
            foreground_executor,
            prompts: Default::default(),
            active_cursor: Default::default(),
            active_display: Rc::new(TestDisplay::new()),
            active_window: Default::default(),
            current_clipboard_item: Mutex::new(None),
            #[cfg(target_os = "linux")]
            current_primary_item: Mutex::new(None),
            weak: weak.clone(),
            opened_url: Default::default(),
            #[cfg(target_os = "windows")]
            bitmap_factory,
            text_system,
        })
    }

    pub(crate) fn simulate_new_path_selection(
        &self,
        select_path: impl FnOnce(&std::path::Path) -> Option<std::path::PathBuf>,
    ) {
        let (path, tx) = self
            .prompts
            .borrow_mut()
            .new_path
            .pop_front()
            .expect("no pending new path prompt");
        tx.send(Ok(select_path(&path))).ok();
    }

    pub(crate) fn simulate_prompt_answer(&self, response_ix: usize) {
        let tx = self
            .prompts
            .borrow_mut()
            .multiple_choice
            .pop_front()
            .expect("no pending multiple choice prompt");
        self.background_executor().set_waiting_hint(None);
        tx.send(response_ix).ok();
    }

    pub(crate) fn has_pending_prompt(&self) -> bool {
        !self.prompts.borrow().multiple_choice.is_empty()
    }

    pub(crate) fn prompt(&self, msg: &str, detail: Option<&str>) -> oneshot::Receiver<usize> {
        let (tx, rx) = oneshot::channel();
        self.background_executor()
            .set_waiting_hint(Some(format!("PROMPT: {:?} {:?}", msg, detail)));
        self.prompts.borrow_mut().multiple_choice.push_back(tx);
        rx
    }

    pub(crate) fn set_active_window(&self, window: Option<TestWindow>) {
        let executor = self.foreground_executor().clone();
        let previous_window = self.active_window.borrow_mut().take();
        self.active_window.borrow_mut().clone_from(&window);

        executor
            .spawn(async move {
                if let Some(previous_window) = previous_window {
                    if let Some(window) = window.as_ref() {
                        if Rc::ptr_eq(&previous_window.0, &window.0) {
                            return;
                        }
                    }
                    previous_window.simulate_active_status_change(false);
                }
                if let Some(window) = window {
                    window.simulate_active_status_change(true);
                }
            })
            .detach();
    }

    pub(crate) fn did_prompt_for_new_path(&self) -> bool {
        self.prompts.borrow().new_path.len() > 0
    }
}

impl Platform for TestPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn run(&self, _on_finish_launching: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn quit(&self) {}

    fn restart(&self, _: Option<PathBuf>) {
        unimplemented!()
    }

    fn activate(&self, _ignoring_other_apps: bool) {
        //
    }

    fn hide(&self) {
        unimplemented!()
    }

    fn hide_other_apps(&self) {
        unimplemented!()
    }

    fn unhide_other_apps(&self) {
        unimplemented!()
    }

    fn displays(&self) -> Vec<std::rc::Rc<dyn crate::PlatformDisplay>> {
        vec![self.active_display.clone()]
    }

    fn primary_display(&self) -> Option<std::rc::Rc<dyn crate::PlatformDisplay>> {
        Some(self.active_display.clone())
    }

    fn active_window(&self) -> Option<crate::AnyWindowHandle> {
        self.active_window
            .borrow()
            .as_ref()
            .map(|window| window.0.lock().handle)
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn crate::PlatformWindow>> {
        let window = TestWindow::new(
            handle,
            params,
            self.weak.clone(),
            self.active_display.clone(),
        );
        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn open_url(&self, url: &str) {
        *self.opened_url.borrow_mut() = Some(url.to_string())
    }

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {
        unimplemented!()
    }

    fn prompt_for_paths(
        &self,
        _options: crate::PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<std::path::PathBuf>>>> {
        unimplemented!()
    }

    fn prompt_for_new_path(
        &self,
        directory: &std::path::Path,
    ) -> oneshot::Receiver<Result<Option<std::path::PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        self.prompts
            .borrow_mut()
            .new_path
            .push_back((directory.to_path_buf(), tx));
        rx
    }

    fn reveal_path(&self, _path: &std::path::Path) {
        unimplemented!()
    }

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn set_menus(&self, _menus: Vec<crate::Menu>, _keymap: &Keymap) {}
    fn set_dock_menu(&self, _menu: Vec<crate::MenuItem>, _keymap: &Keymap) {}

    fn add_recent_document(&self, _paths: &Path) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn crate::Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn crate::Action) -> bool>) {}

    fn app_path(&self) -> Result<std::path::PathBuf> {
        unimplemented!()
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<std::path::PathBuf> {
        unimplemented!()
    }

    fn set_cursor_style(&self, style: crate::CursorStyle) {
        *self.active_cursor.lock() = style;
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    #[cfg(target_os = "linux")]
    fn write_to_primary(&self, item: ClipboardItem) {
        *self.current_primary_item.lock() = Some(item);
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        *self.current_clipboard_item.lock() = Some(item);
    }

    #[cfg(target_os = "linux")]
    fn read_from_primary(&self) -> Option<ClipboardItem> {
        self.current_primary_item.lock().clone()
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.current_clipboard_item.lock().clone()
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        unimplemented!()
    }
}

#[cfg(target_os = "windows")]
impl Drop for TestPlatform {
    fn drop(&mut self) {
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.bitmap_factory);
            windows::Win32::System::Ole::OleUninitialize();
        }
    }
}
