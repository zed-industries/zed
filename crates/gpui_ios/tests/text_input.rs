#[cfg(target_os = "ios")]
#[path = "../src/ios/text_input.rs"]
mod text_input;

// Native protocol construction does not need a GPUI window, renderer, or a
// second text model. This fixture deliberately has no registered input handler.
#[cfg(target_os = "ios")]
mod window {
    use gpui::PlatformInputHandler;
    use std::{
        cell::{Cell, RefCell},
        rc::Weak,
    };

    thread_local! {
        static HANDLER_QUERIES: Cell<usize> = const { Cell::new(0) };
    }

    pub(crate) fn handler_queries() -> usize {
        HANDLER_QUERIES.with(Cell::get)
    }

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
            HANDLER_QUERIES.with(|queries| queries.set(queries.get() + 1));
            self.0.borrow().upgrade().map(|window| callback(&window))
        }
    }
}

fn main() {
    #[cfg(target_os = "ios")]
    {
        let main_thread = objc2::MainThreadMarker::new().expect("native test runs on main thread");
        text_input::tests::native_protocol_smoke_test(main_thread);
        text_input::tests::stale_objects_do_not_route_to_handler(
            main_thread,
            window::handler_queries,
        );
        println!("UIKit text input native protocol smoke test passed");
    }
}
