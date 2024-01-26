#![allow(unused)]

use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DisplayId,
    ForegroundExecutor, Keymap, LinuxDispatcher, LinuxTextSystem, Menu, PathPromptOptions,
    Platform, PlatformDisplay, PlatformInput, PlatformTextSystem, PlatformWindow, Result,
    SemanticVersion, Task, WindowOptions,
};

use futures::channel::oneshot;
use parking_lot::Mutex;

use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use time::UtcOffset;

pub(crate) struct LinuxPlatform(Mutex<LinuxPlatformState>);

pub(crate) struct LinuxPlatformState {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<LinuxTextSystem>,
}

impl Default for LinuxPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxPlatform {
    pub(crate) fn new() -> Self {
        let dispatcher = Arc::new(LinuxDispatcher::new());
        Self(Mutex::new(LinuxPlatformState {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system: Arc::new(LinuxTextSystem::new()),
        }))
    }
}

impl Platform for LinuxPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.0.lock().background_executor.clone()
    }

    fn foreground_executor(&self) -> crate::ForegroundExecutor {
        self.0.lock().foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.0.lock().text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn quit(&self) {
        unimplemented!()
    }

    fn restart(&self) {
        unimplemented!()
    }

    fn activate(&self, ignoring_other_apps: bool) {
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

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        unimplemented!()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        unimplemented!()
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        unimplemented!()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        unimplemented!()
    }

    fn set_display_link_output_callback(
        &self,
        display_id: DisplayId,
        callback: Box<dyn FnMut() + Send>,
    ) {
        unimplemented!()
    }

    fn start_display_link(&self, display_id: DisplayId) {
        unimplemented!()
    }

    fn stop_display_link(&self, display_id: DisplayId) {
        unimplemented!()
    }

    fn open_url(&self, url: &str) {
        unimplemented!()
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        unimplemented!()
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        unimplemented!()
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        unimplemented!()
    }

    fn reveal_path(&self, path: &Path) {
        unimplemented!()
    }

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_event(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        unimplemented!()
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        unimplemented!()
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        unimplemented!()
    }

    fn os_name(&self) -> &'static str {
        "Linux"
    }

    fn double_click_interval(&self) -> Duration {
        unimplemented!()
    }

    fn os_version(&self) -> Result<SemanticVersion> {
        unimplemented!()
    }

    fn app_version(&self) -> Result<SemanticVersion> {
        unimplemented!()
    }

    fn app_path(&self) -> Result<PathBuf> {
        unimplemented!()
    }

    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {
        unimplemented!()
    }

    fn local_timezone(&self) -> UtcOffset {
        unimplemented!()
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        unimplemented!()
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        unimplemented!()
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        unimplemented!()
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        unimplemented!()
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        unimplemented!()
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        unimplemented!()
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        unimplemented!()
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::ClipboardItem;

    use super::*;

    fn build_platform() -> LinuxPlatform {
        let platform = LinuxPlatform::new();
        platform
    }
}
