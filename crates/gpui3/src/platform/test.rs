use super::Platform;

pub struct TestPlatform;

impl TestPlatform {
    pub fn new() -> Self {
        TestPlatform
    }
}

impl Platform for TestPlatform {
    fn font_system(&self) -> std::sync::Arc<dyn crate::PlatformTextSystem> {
        todo!()
    }

    fn open_window(
        &self,
        handle: crate::AnyWindowHandle,
        options: crate::WindowOptions,
    ) -> Box<dyn crate::PlatformWindow> {
        todo!()
    }

    fn dispatcher(&self) -> std::sync::Arc<dyn crate::PlatformDispatcher> {
        todo!()
    }
}
