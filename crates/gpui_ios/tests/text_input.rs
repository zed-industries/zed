#[cfg(target_os = "ios")]
#[path = "../src/ios/text_input.rs"]
mod text_input;

// Native protocol construction does not need a GPUI window, renderer, or a
// second text model. This fixture deliberately has no registered input handler.
#[cfg(target_os = "ios")]
mod window {
    use gpui::PlatformInputHandler;
    use objc2::runtime::Sel;
    use std::{cell::RefCell, rc::Weak};

    pub(crate) struct IosWindowState;

    impl IosWindowState {
        pub(crate) fn with_input_handler<R>(
            &self,
            _callback: impl FnOnce(&mut PlatformInputHandler) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[derive(Default)]
    pub(crate) struct WindowReference(pub(crate) RefCell<Weak<IosWindowState>>);

    impl WindowReference {
        pub(crate) fn with_window<R>(
            &self,
            callback: impl FnOnce(&IosWindowState) -> R,
        ) -> Option<R> {
            self.0.borrow().upgrade().map(|window| callback(&window))
        }

        pub(crate) fn can_perform_action(&self, _action: Sel) -> bool {
            false
        }

        pub(crate) fn dispatch_edit_menu_shortcut(&self, _key: &str) {
            panic!("the smoke test must not dispatch edit-menu shortcuts");
        }
    }
}

fn main() {
    #[cfg(target_os = "ios")]
    {
        let main_thread = objc2::MainThreadMarker::new().expect("native test runs on main thread");
        text_input::tests::native_protocol_smoke_test(main_thread);
        println!("UIKit text input native protocol smoke test passed");
    }
}
