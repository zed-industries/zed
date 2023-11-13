use crate::{
    AnyWindowHandle, BackgroundExecutor, CursorStyle, DisplayId, ForegroundExecutor, Platform,
    PlatformDisplay, PlatformTextSystem, TestDisplay, TestWindow, WindowOptions,
};
use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use std::{rc::Rc, sync::Arc};

pub struct TestPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,

    active_window: Arc<Mutex<Option<AnyWindowHandle>>>,
    active_display: Rc<dyn PlatformDisplay>,
    active_cursor: Mutex<CursorStyle>,
}

impl TestPlatform {
    pub fn new(executor: BackgroundExecutor, foreground_executor: ForegroundExecutor) -> Self {
        TestPlatform {
            background_executor: executor,
            foreground_executor,

            active_cursor: Default::default(),
            active_display: Rc::new(TestDisplay::new()),
            active_window: Default::default(),
        }
    }
}

// todo!("implement out what our tests needed in GPUI 1")
impl Platform for TestPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        Arc::new(crate::platform::mac::MacTextSystem::new())
    }

    fn run(&self, _on_finish_launching: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn quit(&self) {
        unimplemented!()
    }

    fn restart(&self) {
        unimplemented!()
    }

    fn activate(&self, _ignoring_other_apps: bool) {
        unimplemented!()
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

    fn display(&self, id: DisplayId) -> Option<std::rc::Rc<dyn crate::PlatformDisplay>> {
        self.displays().iter().find(|d| d.id() == id).cloned()
    }

    fn main_window(&self) -> Option<crate::AnyWindowHandle> {
        unimplemented!()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn crate::PlatformWindow> {
        *self.active_window.lock() = Some(handle);
        Box::new(TestWindow::new(options, self.active_display.clone()))
    }

    fn set_display_link_output_callback(
        &self,
        _display_id: DisplayId,
        _callback: Box<dyn FnMut(&crate::VideoTimestamp, &crate::VideoTimestamp) + Send>,
    ) {
        unimplemented!()
    }

    fn start_display_link(&self, _display_id: DisplayId) {
        unimplemented!()
    }

    fn stop_display_link(&self, _display_id: DisplayId) {
        unimplemented!()
    }

    fn open_url(&self, _url: &str) {
        unimplemented!()
    }

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {
        unimplemented!()
    }

    fn prompt_for_paths(
        &self,
        _options: crate::PathPromptOptions,
    ) -> futures::channel::oneshot::Receiver<Option<Vec<std::path::PathBuf>>> {
        unimplemented!()
    }

    fn prompt_for_new_path(
        &self,
        _directory: &std::path::Path,
    ) -> futures::channel::oneshot::Receiver<Option<std::path::PathBuf>> {
        unimplemented!()
    }

    fn reveal_path(&self, _path: &std::path::Path) {
        unimplemented!()
    }

    fn on_become_active(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_resign_active(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_event(&self, _callback: Box<dyn FnMut(crate::InputEvent) -> bool>) {
        unimplemented!()
    }

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
        unimplemented!()
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<std::path::PathBuf> {
        unimplemented!()
    }

    fn set_cursor_style(&self, style: crate::CursorStyle) {
        *self.active_cursor.lock() = style;
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        // todo()
        true
    }

    fn write_to_clipboard(&self, _item: crate::ClipboardItem) {
        unimplemented!()
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        unimplemented!()
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Result<()> {
        Ok(())
    }

    fn read_credentials(&self, _url: &str) -> Result<Option<(String, Vec<u8>)>> {
        Ok(None)
    }

    fn delete_credentials(&self, _url: &str) -> Result<()> {
        Ok(())
    }
}
