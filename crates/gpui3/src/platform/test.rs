use super::Platform;

pub struct TestPlatform;

impl TestPlatform {
    pub fn new() -> Self {
        TestPlatform
    }
}

impl Platform for TestPlatform {
    fn executor(&self) -> std::rc::Rc<crate::ForegroundExecutor> {
        todo!()
    }

    fn text_system(&self) -> std::sync::Arc<dyn crate::PlatformTextSystem> {
        todo!()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        todo!()
    }

    fn quit(&self) {
        todo!()
    }

    fn restart(&self) {
        todo!()
    }

    fn activate(&self, ignoring_other_apps: bool) {
        todo!()
    }

    fn hide(&self) {
        todo!()
    }

    fn hide_other_apps(&self) {
        todo!()
    }

    fn unhide_other_apps(&self) {
        todo!()
    }

    fn screens(&self) -> Vec<std::rc::Rc<dyn crate::PlatformScreen>> {
        todo!()
    }

    fn screen_by_id(&self, id: uuid::Uuid) -> Option<std::rc::Rc<dyn crate::PlatformScreen>> {
        todo!()
    }

    fn main_window(&self) -> Option<crate::AnyWindowHandle> {
        todo!()
    }

    fn open_window(
        &self,
        handle: crate::AnyWindowHandle,
        options: crate::WindowOptions,
    ) -> Box<dyn crate::PlatformWindow> {
        todo!()
    }

    fn open_url(&self, url: &str) {
        todo!()
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        todo!()
    }

    fn prompt_for_paths(
        &self,
        options: crate::PathPromptOptions,
    ) -> futures::channel::oneshot::Receiver<Option<Vec<std::path::PathBuf>>> {
        todo!()
    }

    fn prompt_for_new_path(
        &self,
        directory: &std::path::Path,
    ) -> futures::channel::oneshot::Receiver<Option<std::path::PathBuf>> {
        todo!()
    }

    fn reveal_path(&self, path: &std::path::Path) {
        todo!()
    }

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_event(&self, callback: Box<dyn FnMut(crate::Event) -> bool>) {
        todo!()
    }

    fn os_name(&self) -> &'static str {
        todo!()
    }

    fn os_version(&self) -> anyhow::Result<crate::SemanticVersion> {
        todo!()
    }

    fn app_version(&self) -> anyhow::Result<crate::SemanticVersion> {
        todo!()
    }

    fn app_path(&self) -> anyhow::Result<std::path::PathBuf> {
        todo!()
    }

    fn local_timezone(&self) -> time::UtcOffset {
        todo!()
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> anyhow::Result<std::path::PathBuf> {
        todo!()
    }

    fn set_cursor_style(&self, style: crate::CursorStyle) {
        todo!()
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        todo!()
    }

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {
        todo!()
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        todo!()
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> anyhow::Result<()> {
        todo!()
    }

    fn read_credentials(&self, url: &str) -> anyhow::Result<Option<(String, Vec<u8>)>> {
        todo!()
    }

    fn delete_credentials(&self, url: &str) -> anyhow::Result<()> {
        todo!()
    }
}
