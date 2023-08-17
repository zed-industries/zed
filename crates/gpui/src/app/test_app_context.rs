use crate::{
    executor,
    geometry::vector::Vector2F,
    keymap_matcher::{Binding, Keystroke},
    platform,
    platform::{Event, InputHandler, KeyDownEvent, Platform},
    Action, AnyWindowHandle, AppContext, BorrowAppContext, BorrowWindowContext, Entity, FontCache,
    Handle, ModelContext, ModelHandle, Subscription, Task, View, ViewContext, ViewHandle,
    WeakHandle, WindowContext, WindowHandle,
};
use collections::BTreeMap;
use futures::Future;
use itertools::Itertools;
use parking_lot::{Mutex, RwLock};
use smallvec::SmallVec;
use smol::stream::StreamExt;
use std::{
    any::Any,
    cell::RefCell,
    mem,
    path::PathBuf,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use super::{
    ref_counts::LeakDetector, window_input_handler::WindowInputHandler, AsyncAppContext, RefCounts,
};

#[derive(Clone)]
pub struct TestAppContext {
    cx: Rc<RefCell<AppContext>>,
    foreground_platform: Rc<platform::test::ForegroundPlatform>,
    condition_duration: Option<Duration>,
    pub function_name: String,
    assertion_context: AssertionContextManager,
}

impl TestAppContext {
    pub fn new(
        foreground_platform: Rc<platform::test::ForegroundPlatform>,
        platform: Arc<dyn Platform>,
        foreground: Rc<executor::Foreground>,
        background: Arc<executor::Background>,
        font_cache: Arc<FontCache>,
        leak_detector: Arc<Mutex<LeakDetector>>,
        first_entity_id: usize,
        function_name: String,
    ) -> Self {
        let mut cx = AppContext::new(
            foreground,
            background,
            platform,
            foreground_platform.clone(),
            font_cache,
            RefCounts::new(leak_detector),
            (),
        );
        cx.next_id = first_entity_id;
        let cx = TestAppContext {
            cx: Rc::new(RefCell::new(cx)),
            foreground_platform,
            condition_duration: None,
            function_name,
            assertion_context: AssertionContextManager::new(),
        };
        cx.cx.borrow_mut().weak_self = Some(Rc::downgrade(&cx.cx));
        cx
    }

    pub fn dispatch_action<A: Action>(&mut self, window: AnyWindowHandle, action: A) {
        self.update_window(window, |window| {
            window.dispatch_action(window.focused_view_id(), &action);
        })
        .expect("window not found");
    }

    pub fn available_actions(
        &self,
        window: AnyWindowHandle,
        view_id: usize,
    ) -> Vec<(&'static str, Box<dyn Action>, SmallVec<[Binding; 1]>)> {
        self.read_window(window, |cx| cx.available_actions(view_id))
            .unwrap_or_default()
    }

    pub fn dispatch_global_action<A: Action>(&mut self, action: A) {
        self.update(|cx| cx.dispatch_global_action_any(&action));
    }

    pub fn dispatch_keystroke(
        &mut self,
        window: AnyWindowHandle,
        keystroke: Keystroke,
        is_held: bool,
    ) {
        let handled = window.update(self, |cx| {
            if cx.dispatch_keystroke(&keystroke) {
                return true;
            }

            if cx.dispatch_event(
                Event::KeyDown(KeyDownEvent {
                    keystroke: keystroke.clone(),
                    is_held,
                }),
                false,
            ) {
                return true;
            }

            false
        });

        if !handled && !keystroke.cmd && !keystroke.ctrl {
            WindowInputHandler {
                app: self.cx.clone(),
                window,
            }
            .replace_text_in_range(None, &keystroke.key)
        }
    }

    pub fn read_window<T, F: FnOnce(&WindowContext) -> T>(
        &self,
        window: AnyWindowHandle,
        callback: F,
    ) -> Option<T> {
        self.cx.borrow().read_window(window, callback)
    }

    pub fn update_window<T, F: FnOnce(&mut WindowContext) -> T>(
        &mut self,
        window: AnyWindowHandle,
        callback: F,
    ) -> Option<T> {
        self.cx.borrow_mut().update_window(window, callback)
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        self.cx.borrow_mut().add_model(build_model)
    }

    pub fn add_window<V, F>(&mut self, build_root_view: F) -> WindowHandle<V>
    where
        V: View,
        F: FnOnce(&mut ViewContext<V>) -> V,
    {
        let window = self
            .cx
            .borrow_mut()
            .add_window(Default::default(), build_root_view);
        window.simulate_activation(self);
        window
    }

    pub fn observe_global<E, F>(&mut self, callback: F) -> Subscription
    where
        E: Any,
        F: 'static + FnMut(&mut AppContext),
    {
        self.cx.borrow_mut().observe_global::<E, F>(callback)
    }

    pub fn set_global<T: 'static>(&mut self, state: T) {
        self.cx.borrow_mut().set_global(state);
    }

    pub fn subscribe_global<E, F>(&mut self, callback: F) -> Subscription
    where
        E: Any,
        F: 'static + FnMut(&E, &mut AppContext),
    {
        self.cx.borrow_mut().subscribe_global(callback)
    }

    pub fn windows(&self) -> Vec<AnyWindowHandle> {
        self.cx.borrow().windows().collect()
    }

    pub fn remove_all_windows(&mut self) {
        self.update(|cx| cx.windows.clear());
    }

    pub fn read<T, F: FnOnce(&AppContext) -> T>(&self, callback: F) -> T {
        callback(&*self.cx.borrow())
    }

    pub fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.cx.borrow_mut();
        // Don't increment pending flushes in order for effects to be flushed before the callback
        // completes, which is helpful in tests.
        let result = callback(&mut *state);
        // Flush effects after the callback just in case there are any. This can happen in edge
        // cases such as the closure dropping handles.
        state.flush_effects();
        result
    }

    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext(self.cx.clone())
    }

    pub fn font_cache(&self) -> Arc<FontCache> {
        self.cx.borrow().font_cache.clone()
    }

    pub fn foreground_platform(&self) -> Rc<platform::test::ForegroundPlatform> {
        self.foreground_platform.clone()
    }

    pub fn platform(&self) -> Arc<dyn platform::Platform> {
        self.cx.borrow().platform.clone()
    }

    pub fn foreground(&self) -> Rc<executor::Foreground> {
        self.cx.borrow().foreground().clone()
    }

    pub fn background(&self) -> Arc<executor::Background> {
        self.cx.borrow().background().clone()
    }

    pub fn spawn<F, Fut, T>(&self, f: F) -> Task<T>
    where
        F: FnOnce(AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = T>,
        T: 'static,
    {
        let foreground = self.foreground();
        let future = f(self.to_async());
        let cx = self.to_async();
        foreground.spawn(async move {
            let result = future.await;
            cx.0.borrow_mut().flush_effects();
            result
        })
    }

    pub fn simulate_new_path_selection(&self, result: impl FnOnce(PathBuf) -> Option<PathBuf>) {
        self.foreground_platform.simulate_new_path_selection(result);
    }

    pub fn did_prompt_for_new_path(&self) -> bool {
        self.foreground_platform.as_ref().did_prompt_for_new_path()
    }

    pub fn leak_detector(&self) -> Arc<Mutex<LeakDetector>> {
        self.cx.borrow().leak_detector()
    }

    pub fn assert_dropped(&self, handle: impl WeakHandle) {
        self.cx
            .borrow()
            .leak_detector()
            .lock()
            .assert_dropped(handle.id())
    }

    /// Drop a handle, assuming it is the last. If it is not the last, panic with debug information about
    /// where the stray handles were created.
    pub fn drop_last<T, W: WeakHandle, H: Handle<T, Weak = W>>(&mut self, handle: H) {
        let weak = handle.downgrade();
        self.update(|_| drop(handle));
        self.assert_dropped(weak);
    }

    pub fn set_condition_duration(&mut self, duration: Option<Duration>) {
        self.condition_duration = duration;
    }

    pub fn condition_duration(&self) -> Duration {
        self.condition_duration.unwrap_or_else(|| {
            if std::env::var("CI").is_ok() {
                Duration::from_secs(2)
            } else {
                Duration::from_millis(500)
            }
        })
    }

    pub fn assert_clipboard_content(&mut self, expected_content: Option<&str>) {
        self.update(|cx| {
            let actual_content = cx.read_from_clipboard().map(|item| item.text().to_owned());
            let expected_content = expected_content.map(|content| content.to_owned());
            assert_eq!(actual_content, expected_content);
        })
    }

    pub fn add_assertion_context(&self, context: String) -> ContextHandle {
        self.assertion_context.add_context(context)
    }

    pub fn assertion_context(&self) -> String {
        self.assertion_context.context()
    }
}

impl BorrowAppContext for TestAppContext {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        self.cx.borrow().read_with(f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        self.cx.borrow_mut().update(f)
    }
}

impl BorrowWindowContext for TestAppContext {
    type Result<T> = T;

    fn read_window<T, F: FnOnce(&WindowContext) -> T>(&self, window: AnyWindowHandle, f: F) -> T {
        self.cx
            .borrow()
            .read_window(window, f)
            .expect("window was closed")
    }

    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>,
    {
        BorrowWindowContext::read_window(self, window, f)
    }

    fn update_window<T, F: FnOnce(&mut WindowContext) -> T>(
        &mut self,
        window: AnyWindowHandle,
        f: F,
    ) -> T {
        self.cx
            .borrow_mut()
            .update_window(window, f)
            .expect("window was closed")
    }

    fn update_window_optional<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>,
    {
        BorrowWindowContext::update_window(self, window, f)
    }
}

impl<T: Entity> ModelHandle<T> {
    pub fn next_notification(&self, cx: &TestAppContext) -> impl Future<Output = ()> {
        let (tx, mut rx) = futures::channel::mpsc::unbounded();
        let mut cx = cx.cx.borrow_mut();
        let subscription = cx.observe(self, move |_, _| {
            tx.unbounded_send(()).ok();
        });

        let duration = if std::env::var("CI").is_ok() {
            Duration::from_secs(5)
        } else {
            Duration::from_secs(1)
        };

        let executor = cx.background().clone();
        async move {
            executor.start_waiting();
            let notification = crate::util::timeout(duration, rx.next())
                .await
                .expect("next notification timed out");
            drop(subscription);
            notification.expect("model dropped while test was waiting for its next notification")
        }
    }

    pub fn next_event(&self, cx: &TestAppContext) -> impl Future<Output = T::Event>
    where
        T::Event: Clone,
    {
        let (tx, mut rx) = futures::channel::mpsc::unbounded();
        let mut cx = cx.cx.borrow_mut();
        let subscription = cx.subscribe(self, move |_, event, _| {
            tx.unbounded_send(event.clone()).ok();
        });

        let duration = if std::env::var("CI").is_ok() {
            Duration::from_secs(5)
        } else {
            Duration::from_secs(1)
        };

        cx.foreground.start_waiting();
        async move {
            let event = crate::util::timeout(duration, rx.next())
                .await
                .expect("next event timed out");
            drop(subscription);
            event.expect("model dropped while test was waiting for its next event")
        }
    }

    pub fn condition(
        &self,
        cx: &TestAppContext,
        mut predicate: impl FnMut(&T, &AppContext) -> bool,
    ) -> impl Future<Output = ()> {
        let (tx, mut rx) = futures::channel::mpsc::unbounded();

        let mut cx = cx.cx.borrow_mut();
        let subscriptions = (
            cx.observe(self, {
                let tx = tx.clone();
                move |_, _| {
                    tx.unbounded_send(()).ok();
                }
            }),
            cx.subscribe(self, {
                move |_, _, _| {
                    tx.unbounded_send(()).ok();
                }
            }),
        );

        let cx = cx.weak_self.as_ref().unwrap().upgrade().unwrap();
        let handle = self.downgrade();
        let duration = if std::env::var("CI").is_ok() {
            Duration::from_secs(5)
        } else {
            Duration::from_secs(1)
        };

        async move {
            crate::util::timeout(duration, async move {
                loop {
                    {
                        let cx = cx.borrow();
                        let cx = &*cx;
                        if predicate(
                            handle
                                .upgrade(cx)
                                .expect("model dropped with pending condition")
                                .read(cx),
                            cx,
                        ) {
                            break;
                        }
                    }

                    cx.borrow().foreground().start_waiting();
                    rx.next()
                        .await
                        .expect("model dropped with pending condition");
                    cx.borrow().foreground().finish_waiting();
                }
            })
            .await
            .expect("condition timed out");
            drop(subscriptions);
        }
    }
}

impl AnyWindowHandle {
    pub fn has_pending_prompt(&self, cx: &mut TestAppContext) -> bool {
        let window = self.platform_window_mut(cx);
        let prompts = window.pending_prompts.borrow_mut();
        !prompts.is_empty()
    }

    pub fn current_title(&self, cx: &mut TestAppContext) -> Option<String> {
        self.platform_window_mut(cx).title.clone()
    }

    pub fn simulate_close(&self, cx: &mut TestAppContext) -> bool {
        let handler = self.platform_window_mut(cx).should_close_handler.take();
        if let Some(mut handler) = handler {
            let should_close = handler();
            self.platform_window_mut(cx).should_close_handler = Some(handler);
            should_close
        } else {
            false
        }
    }

    pub fn simulate_resize(&self, size: Vector2F, cx: &mut TestAppContext) {
        let mut window = self.platform_window_mut(cx);
        window.size = size;
        let mut handlers = mem::take(&mut window.resize_handlers);
        drop(window);
        for handler in &mut handlers {
            handler();
        }
        self.platform_window_mut(cx).resize_handlers = handlers;
    }

    pub fn is_edited(&self, cx: &mut TestAppContext) -> bool {
        self.platform_window_mut(cx).edited
    }

    pub fn simulate_prompt_answer(&self, answer: usize, cx: &mut TestAppContext) {
        use postage::prelude::Sink as _;

        let mut done_tx = self
            .platform_window_mut(cx)
            .pending_prompts
            .borrow_mut()
            .pop_front()
            .expect("prompt was not called");
        done_tx.try_send(answer).ok();
    }

    fn platform_window_mut<'a>(
        &self,
        cx: &'a mut TestAppContext,
    ) -> std::cell::RefMut<'a, platform::test::Window> {
        std::cell::RefMut::map(cx.cx.borrow_mut(), |state| {
            let window = state.windows.get_mut(&self).unwrap();
            let test_window = window
                .platform_window
                .as_any_mut()
                .downcast_mut::<platform::test::Window>()
                .unwrap();
            test_window
        })
    }
}

impl<T: View> ViewHandle<T> {
    pub fn next_notification(&self, cx: &TestAppContext) -> impl Future<Output = ()> {
        use postage::prelude::{Sink as _, Stream as _};

        let (mut tx, mut rx) = postage::mpsc::channel(1);
        let mut cx = cx.cx.borrow_mut();
        let subscription = cx.observe(self, move |_, _| {
            tx.try_send(()).ok();
        });

        let duration = if std::env::var("CI").is_ok() {
            Duration::from_secs(5)
        } else {
            Duration::from_secs(1)
        };

        async move {
            let notification = crate::util::timeout(duration, rx.recv())
                .await
                .expect("next notification timed out");
            drop(subscription);
            notification.expect("model dropped while test was waiting for its next notification")
        }
    }

    pub fn condition(
        &self,
        cx: &TestAppContext,
        mut predicate: impl FnMut(&T, &AppContext) -> bool,
    ) -> impl Future<Output = ()> {
        use postage::prelude::{Sink as _, Stream as _};

        let (tx, mut rx) = postage::mpsc::channel(1024);
        let timeout_duration = cx.condition_duration();

        let mut cx = cx.cx.borrow_mut();
        let subscriptions = (
            cx.observe(self, {
                let mut tx = tx.clone();
                move |_, _| {
                    tx.blocking_send(()).ok();
                }
            }),
            cx.subscribe(self, {
                let mut tx = tx.clone();
                move |_, _, _| {
                    tx.blocking_send(()).ok();
                }
            }),
        );

        let cx = cx.weak_self.as_ref().unwrap().upgrade().unwrap();
        let handle = self.downgrade();

        async move {
            crate::util::timeout(timeout_duration, async move {
                loop {
                    {
                        let cx = cx.borrow();
                        let cx = &*cx;
                        if predicate(
                            handle
                                .upgrade(cx)
                                .expect("view dropped with pending condition")
                                .read(cx),
                            cx,
                        ) {
                            break;
                        }
                    }

                    cx.borrow().foreground().start_waiting();
                    rx.recv()
                        .await
                        .expect("view dropped with pending condition");
                    cx.borrow().foreground().finish_waiting();
                }
            })
            .await
            .expect("condition timed out");
            drop(subscriptions);
        }
    }
}

/// Tracks string context to be printed when assertions fail.
/// Often this is done by storing a context string in the manager and returning the handle.
#[derive(Clone)]
pub struct AssertionContextManager {
    id: Arc<AtomicUsize>,
    contexts: Arc<RwLock<BTreeMap<usize, String>>>,
}

impl AssertionContextManager {
    pub fn new() -> Self {
        Self {
            id: Arc::new(AtomicUsize::new(0)),
            contexts: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn add_context(&self, context: String) -> ContextHandle {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let mut contexts = self.contexts.write();
        contexts.insert(id, context);
        ContextHandle {
            id,
            manager: self.clone(),
        }
    }

    pub fn context(&self) -> String {
        let contexts = self.contexts.read();
        format!("\n{}\n", contexts.values().join("\n"))
    }
}

/// Used to track the lifetime of a piece of context so that it can be provided when an assertion fails.
/// For example, in the EditorTestContext, `set_state` returns a context handle so that if an assertion fails,
/// the state that was set initially for the failure can be printed in the error message
pub struct ContextHandle {
    id: usize,
    manager: AssertionContextManager,
}

impl Drop for ContextHandle {
    fn drop(&mut self) {
        let mut contexts = self.manager.contexts.write();
        contexts.remove(&self.id);
    }
}
