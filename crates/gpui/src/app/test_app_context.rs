use std::{
    cell::RefCell,
    marker::PhantomData,
    mem,
    path::PathBuf,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use futures::Future;
use itertools::Itertools;
use parking_lot::{Mutex, RwLock};
use smol::stream::StreamExt;

use crate::{
    executor, geometry::vector::Vector2F, keymap_matcher::Keystroke, platform, Action,
    AnyViewHandle, AppContext, Appearance, Entity, Event, FontCache, InputHandler, KeyDownEvent,
    LeakDetector, ModelContext, ModelHandle, MutableAppContext, Platform, ReadModelWith,
    ReadViewWith, RenderContext, Task, UpdateModel, UpdateView, View, ViewContext, ViewHandle,
    WeakHandle, WindowInputHandler,
};
use collections::BTreeMap;

use super::{AsyncAppContext, RefCounts};

pub struct TestAppContext {
    cx: Rc<RefCell<MutableAppContext>>,
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
        let mut cx = MutableAppContext::new(
            foreground,
            background,
            platform,
            foreground_platform.clone(),
            font_cache,
            RefCounts {
                #[cfg(any(test, feature = "test-support"))]
                leak_detector,
                ..Default::default()
            },
            (),
        );
        cx.next_entity_id = first_entity_id;
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

    pub fn dispatch_action<A: Action>(&self, window_id: usize, action: A) {
        let mut cx = self.cx.borrow_mut();
        if let Some(view_id) = cx.focused_view_id(window_id) {
            cx.handle_dispatch_action_from_effect(window_id, Some(view_id), &action);
        }
    }

    pub fn dispatch_global_action<A: Action>(&self, action: A) {
        self.cx.borrow_mut().dispatch_global_action(action);
    }

    pub fn dispatch_keystroke(&mut self, window_id: usize, keystroke: Keystroke, is_held: bool) {
        let handled = self.cx.borrow_mut().update(|cx| {
            let presenter = cx
                .presenters_and_platform_windows
                .get(&window_id)
                .unwrap()
                .0
                .clone();

            if cx.dispatch_keystroke(window_id, &keystroke) {
                return true;
            }

            if presenter.borrow_mut().dispatch_event(
                Event::KeyDown(KeyDownEvent {
                    keystroke: keystroke.clone(),
                    is_held,
                }),
                false,
                cx,
            ) {
                return true;
            }

            false
        });

        if !handled && !keystroke.cmd && !keystroke.ctrl {
            WindowInputHandler {
                app: self.cx.clone(),
                window_id,
            }
            .replace_text_in_range(None, &keystroke.key)
        }
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        self.cx.borrow_mut().add_model(build_model)
    }

    pub fn add_window<T, F>(&mut self, build_root_view: F) -> (usize, ViewHandle<T>)
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        let (window_id, view) = self
            .cx
            .borrow_mut()
            .add_window(Default::default(), build_root_view);
        self.simulate_window_activation(Some(window_id));
        (window_id, view)
    }

    pub fn add_view<T, F>(
        &mut self,
        parent_handle: impl Into<AnyViewHandle>,
        build_view: F,
    ) -> ViewHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.cx.borrow_mut().add_view(parent_handle, build_view)
    }

    pub fn window_ids(&self) -> Vec<usize> {
        self.cx.borrow().window_ids().collect()
    }

    pub fn root_view<T: View>(&self, window_id: usize) -> Option<ViewHandle<T>> {
        self.cx.borrow().root_view(window_id)
    }

    pub fn read<T, F: FnOnce(&AppContext) -> T>(&self, callback: F) -> T {
        callback(self.cx.borrow().as_ref())
    }

    pub fn update<T, F: FnOnce(&mut MutableAppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.cx.borrow_mut();
        // Don't increment pending flushes in order for effects to be flushed before the callback
        // completes, which is helpful in tests.
        let result = callback(&mut *state);
        // Flush effects after the callback just in case there are any. This can happen in edge
        // cases such as the closure dropping handles.
        state.flush_effects();
        result
    }

    pub fn render<F, V, T>(&mut self, handle: &ViewHandle<V>, f: F) -> T
    where
        F: FnOnce(&mut V, &mut RenderContext<V>) -> T,
        V: View,
    {
        handle.update(&mut *self.cx.borrow_mut(), |view, cx| {
            let mut render_cx = RenderContext {
                app: cx,
                window_id: handle.window_id(),
                view_id: handle.id(),
                view_type: PhantomData,
                titlebar_height: 0.,
                hovered_region_ids: Default::default(),
                clicked_region_ids: None,
                refreshing: false,
                appearance: Appearance::Light,
            };
            f(view, &mut render_cx)
        })
    }

    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext(self.cx.clone())
    }

    pub fn font_cache(&self) -> Arc<FontCache> {
        self.cx.borrow().cx.font_cache.clone()
    }

    pub fn foreground_platform(&self) -> Rc<platform::test::ForegroundPlatform> {
        self.foreground_platform.clone()
    }

    pub fn platform(&self) -> Arc<dyn platform::Platform> {
        self.cx.borrow().cx.platform.clone()
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

    pub fn simulate_prompt_answer(&self, window_id: usize, answer: usize) {
        use postage::prelude::Sink as _;

        let mut done_tx = self
            .window_mut(window_id)
            .pending_prompts
            .borrow_mut()
            .pop_front()
            .expect("prompt was not called");
        let _ = done_tx.try_send(answer);
    }

    pub fn has_pending_prompt(&self, window_id: usize) -> bool {
        let window = self.window_mut(window_id);
        let prompts = window.pending_prompts.borrow_mut();
        !prompts.is_empty()
    }

    pub fn current_window_title(&self, window_id: usize) -> Option<String> {
        self.window_mut(window_id).title.clone()
    }

    pub fn simulate_window_close(&self, window_id: usize) -> bool {
        let handler = self.window_mut(window_id).should_close_handler.take();
        if let Some(mut handler) = handler {
            let should_close = handler();
            self.window_mut(window_id).should_close_handler = Some(handler);
            should_close
        } else {
            false
        }
    }

    pub fn simulate_window_resize(&self, window_id: usize, size: Vector2F) {
        let mut window = self.window_mut(window_id);
        window.size = size;
        let mut handlers = mem::take(&mut window.resize_handlers);
        drop(window);
        for handler in &mut handlers {
            handler();
        }
        self.window_mut(window_id).resize_handlers = handlers;
    }

    pub fn simulate_window_activation(&self, to_activate: Option<usize>) {
        let mut handlers = BTreeMap::new();
        {
            let mut cx = self.cx.borrow_mut();
            for (window_id, (_, window)) in &mut cx.presenters_and_platform_windows {
                let window = window
                    .as_any_mut()
                    .downcast_mut::<platform::test::Window>()
                    .unwrap();
                handlers.insert(
                    *window_id,
                    mem::take(&mut window.active_status_change_handlers),
                );
            }
        };
        let mut handlers = handlers.into_iter().collect::<Vec<_>>();
        handlers.sort_unstable_by_key(|(window_id, _)| Some(*window_id) == to_activate);

        for (window_id, mut window_handlers) in handlers {
            for window_handler in &mut window_handlers {
                window_handler(Some(window_id) == to_activate);
            }

            self.window_mut(window_id)
                .active_status_change_handlers
                .extend(window_handlers);
        }
    }

    pub fn is_window_edited(&self, window_id: usize) -> bool {
        self.window_mut(window_id).edited
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

    fn window_mut(&self, window_id: usize) -> std::cell::RefMut<platform::test::Window> {
        std::cell::RefMut::map(self.cx.borrow_mut(), |state| {
            let (_, window) = state
                .presenters_and_platform_windows
                .get_mut(&window_id)
                .unwrap();
            let test_window = window
                .as_any_mut()
                .downcast_mut::<platform::test::Window>()
                .unwrap();
            test_window
        })
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

impl UpdateModel for TestAppContext {
    fn update_model<T: Entity, O>(
        &mut self,
        handle: &ModelHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ModelContext<T>) -> O,
    ) -> O {
        self.cx.borrow_mut().update_model(handle, update)
    }
}

impl ReadModelWith for TestAppContext {
    fn read_model_with<E: Entity, T>(
        &self,
        handle: &ModelHandle<E>,
        read: &mut dyn FnMut(&E, &AppContext) -> T,
    ) -> T {
        let cx = self.cx.borrow();
        let cx = cx.as_ref();
        read(handle.read(cx), cx)
    }
}

impl UpdateView for TestAppContext {
    fn update_view<T, S>(
        &mut self,
        handle: &ViewHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ViewContext<T>) -> S,
    ) -> S
    where
        T: View,
    {
        self.cx.borrow_mut().update_view(handle, update)
    }
}

impl ReadViewWith for TestAppContext {
    fn read_view_with<V, T>(
        &self,
        handle: &ViewHandle<V>,
        read: &mut dyn FnMut(&V, &AppContext) -> T,
    ) -> T
    where
        V: View,
    {
        let cx = self.cx.borrow();
        let cx = cx.as_ref();
        read(handle.read(cx), cx)
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

        async move {
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
                        let cx = cx.as_ref();
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
        let subscriptions = self.update(&mut *cx, |_, cx| {
            (
                cx.observe(self, {
                    let mut tx = tx.clone();
                    move |_, _, _| {
                        tx.blocking_send(()).ok();
                    }
                }),
                cx.subscribe(self, {
                    let mut tx = tx.clone();
                    move |_, _, _, _| {
                        tx.blocking_send(()).ok();
                    }
                }),
            )
        });

        let cx = cx.weak_self.as_ref().unwrap().upgrade().unwrap();
        let handle = self.downgrade();

        async move {
            crate::util::timeout(timeout_duration, async move {
                loop {
                    {
                        let cx = cx.borrow();
                        let cx = cx.as_ref();
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
