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

    fn run(&self, _on_finish_launching: Box<dyn FnOnce()>) {
        todo!()
    }

    fn quit(&self) {
        todo!()
    }

    fn restart(&self) {
        todo!()
    }

    fn activate(&self, _ignoring_other_apps: bool) {
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

    fn screen_by_id(&self, _id: uuid::Uuid) -> Option<std::rc::Rc<dyn crate::PlatformScreen>> {
        todo!()
    }

    fn main_window(&self) -> Option<crate::AnyWindowHandle> {
        todo!()
    }

    fn open_window(
        &self,
        _handle: crate::AnyWindowHandle,
        _options: crate::WindowOptions,
    ) -> Box<dyn crate::PlatformWindow> {
        todo!()
    }

    fn open_url(&self, _url: &str) {
        todo!()
    }

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {
        todo!()
    }

    fn prompt_for_paths(
        &self,
        _options: crate::PathPromptOptions,
    ) -> futures::channel::oneshot::Receiver<Option<Vec<std::path::PathBuf>>> {
        todo!()
    }

    fn prompt_for_new_path(
        &self,
        _directory: &std::path::Path,
    ) -> futures::channel::oneshot::Receiver<Option<std::path::PathBuf>> {
        todo!()
    }

    fn reveal_path(&self, _path: &std::path::Path) {
        todo!()
    }

    fn on_become_active(&self, _callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_resign_active(&self, _callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn on_event(&self, _callback: Box<dyn FnMut(crate::Event) -> bool>) {
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

    fn path_for_auxiliary_executable(&self, _name: &str) -> anyhow::Result<std::path::PathBuf> {
        todo!()
    }

    fn set_cursor_style(&self, _style: crate::CursorStyle) {
        todo!()
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        todo!()
    }

    fn write_to_clipboard(&self, _item: crate::ClipboardItem) {
        todo!()
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        todo!()
    }

    fn write_credentials(
        &self,
        _url: &str,
        _username: &str,
        _password: &[u8],
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn read_credentials(&self, _url: &str) -> anyhow::Result<Option<(String, Vec<u8>)>> {
        todo!()
    }

    fn delete_credentials(&self, _url: &str) -> anyhow::Result<()> {
        todo!()
    }
}
