use crate::{
    AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DisplayId, ForegroundExecutor,
    Keymap, Platform, PlatformDisplay, PlatformTextSystem, Task, TestDisplay, TestWindow,
    WindowAppearance, WindowParams,
};
use anyhow::{anyhow, Result};
use collections::VecDeque;
use futures::channel::oneshot;
use parking_lot::Mutex;
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::{Rc, Weak},
    sync::Arc,
};

/// TestPlatform implements the Platform trait for use in tests.
pub(crate) struct TestPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,

    pub(crate) active_window: RefCell<Option<TestWindow>>,
    active_display: Rc<dyn PlatformDisplay>,
    active_cursor: Mutex<CursorStyle>,
    current_clipboard_item: Mutex<Option<ClipboardItem>>,
    pub(crate) prompts: RefCell<TestPrompts>,
    pub opened_url: RefCell<Option<String>>,
    weak: Weak<Self>,
}

#[derive(Default)]
pub(crate) struct TestPrompts {
    multiple_choice: VecDeque<oneshot::Sender<usize>>,
    new_path: VecDeque<(PathBuf, oneshot::Sender<Option<PathBuf>>)>,
}

impl TestPlatform {
    pub fn new(executor: BackgroundExecutor, foreground_executor: ForegroundExecutor) -> Rc<Self> {
        Rc::new_cyclic(|weak| TestPlatform {
            background_executor: executor,
            foreground_executor,
            prompts: Default::default(),
            active_cursor: Default::default(),
            active_display: Rc::new(TestDisplay::new()),
            active_window: Default::default(),
            current_clipboard_item: Mutex::new(None),
            weak: weak.clone(),
            opened_url: Default::default(),
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
        tx.send(select_path(&path)).ok();
    }

    pub(crate) fn simulate_prompt_answer(&self, response_ix: usize) {
        let tx = self
            .prompts
            .borrow_mut()
            .multiple_choice
            .pop_front()
            .expect("no pending multiple choice prompt");
        tx.send(response_ix).ok();
    }

    pub(crate) fn has_pending_prompt(&self) -> bool {
        !self.prompts.borrow().multiple_choice.is_empty()
    }

    pub(crate) fn prompt(&self) -> oneshot::Receiver<usize> {
        let (tx, rx) = oneshot::channel();
        self.prompts.borrow_mut().multiple_choice.push_back(tx);
        rx
    }

    pub(crate) fn set_active_window(&self, window: Option<TestWindow>) {
        let executor = self.foreground_executor().clone();
        let previous_window = self.active_window.borrow_mut().take();
        *self.active_window.borrow_mut() = window.clone();

        executor
            .spawn(async move {
                if let Some(previous_window) = previous_window {
                    if let Some(window) = window.as_ref() {
                        if Arc::ptr_eq(&previous_window.0, &window.0) {
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
        #[cfg(target_os = "linux")]
        return Arc::new(crate::platform::linux::LinuxTextSystem::new());

        #[cfg(target_os = "macos")]
        return Arc::new(crate::platform::mac::MacTextSystem::new());

        #[cfg(target_os = "windows")]
        return Arc::new(crate::platform::windows::WindowsTextSystem::new());
    }

    fn run(&self, _on_finish_launching: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn quit(&self) {}

    fn restart(&self) {
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

    fn display(&self, id: DisplayId) -> Option<std::rc::Rc<dyn crate::PlatformDisplay>> {
        self.displays().iter().find(|d| d.id() == id).cloned()
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
    ) -> Box<dyn crate::PlatformWindow> {
        let window = TestWindow::new(
            handle,
            params,
            self.weak.clone(),
            self.active_display.clone(),
        );
        Box::new(window)
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
    ) -> oneshot::Receiver<Option<Vec<std::path::PathBuf>>> {
        unimplemented!()
    }

    fn prompt_for_new_path(
        &self,
        directory: &std::path::Path,
    ) -> oneshot::Receiver<Option<std::path::PathBuf>> {
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

    fn on_become_active(&self, _callback: Box<dyn FnMut()>) {}

    fn on_resign_active(&self, _callback: Box<dyn FnMut()>) {}

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_event(&self, _callback: Box<dyn FnMut(crate::PlatformInput) -> bool>) {
        unimplemented!()
    }

    fn set_menus(&self, _menus: Vec<crate::Menu>, _keymap: &Keymap) {}

    fn add_recent_documents(&self, _paths: &[PathBuf]) {}

    fn clear_recent_documents(&self) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn crate::Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn crate::Action) -> bool>) {}

    fn os_name(&self) -> &'static str {
        "test"
    }

    fn os_version(&self) -> Result<crate::SemanticVersion> {
        Err(anyhow!("os_version called on TestPlatform"))
    }

    fn app_version(&self) -> Result<crate::SemanticVersion> {
        Err(anyhow!("app_version called on TestPlatform"))
    }

    fn app_path(&self) -> Result<std::path::PathBuf> {
        unimplemented!()
    }

    fn local_timezone(&self) -> time::UtcOffset {
        time::UtcOffset::UTC
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

    fn write_to_clipboard(&self, item: ClipboardItem) {
        *self.current_clipboard_item.lock() = Some(item);
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
