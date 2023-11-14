use crate::{
    div, AnyView, AnyWindowHandle, AppCell, AppContext, AsyncAppContext, BackgroundExecutor,
    Context, Div, EventEmitter, ForegroundExecutor, InputEvent, KeyDownEvent, Keystroke, Model,
    ModelContext, Render, Result, Task, TestDispatcher, TestPlatform, View, ViewContext,
    VisualContext, WindowContext, WindowHandle, WindowOptions,
};
use anyhow::{anyhow, bail};
use futures::{Stream, StreamExt};
use std::{future::Future, ops::Deref, rc::Rc, sync::Arc, time::Duration};

#[derive(Clone)]
pub struct TestAppContext {
    pub app: Rc<AppCell>,
    pub background_executor: BackgroundExecutor,
    pub foreground_executor: ForegroundExecutor,
    pub dispatcher: TestDispatcher,
}

impl Context for TestAppContext {
    type Result<T> = T;

    fn build_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>
    where
        T: 'static,
    {
        let mut app = self.app.borrow_mut();
        app.build_model(build_model)
    }

    fn update_model<T: 'static, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        let mut app = self.app.borrow_mut();
        app.update_model(handle, update)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T,
    {
        let mut lock = self.app.borrow_mut();
        lock.update_window(window, f)
    }

    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        let app = self.app.borrow();
        app.read_model(handle, read)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        let app = self.app.borrow();
        app.read_window(window, read)
    }
}

impl TestAppContext {
    pub fn new(dispatcher: TestDispatcher) -> Self {
        let arc_dispatcher = Arc::new(dispatcher.clone());
        let background_executor = BackgroundExecutor::new(arc_dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(arc_dispatcher);
        let platform = Rc::new(TestPlatform::new(
            background_executor.clone(),
            foreground_executor.clone(),
        ));
        let asset_source = Arc::new(());
        let http_client = util::http::FakeHttpClient::with_404_response();
        Self {
            app: AppContext::new(platform, asset_source, http_client),
            background_executor,
            foreground_executor,
            dispatcher: dispatcher.clone(),
        }
    }

    pub fn new_app(&self) -> TestAppContext {
        Self::new(self.dispatcher.clone())
    }

    pub fn quit(&self) {
        self.app.borrow_mut().quit();
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut app = self.app.borrow_mut();
        app.refresh();
        Ok(())
    }

    pub fn executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    pub fn foreground_executor(&self) -> &ForegroundExecutor {
        &self.foreground_executor
    }

    pub fn update<R>(&self, f: impl FnOnce(&mut AppContext) -> R) -> R {
        let mut cx = self.app.borrow_mut();
        cx.update(f)
    }

    pub fn read<R>(&self, f: impl FnOnce(&AppContext) -> R) -> R {
        let cx = self.app.borrow();
        f(&*cx)
    }

    pub fn add_window<F, V>(&mut self, build_window: F) -> WindowHandle<V>
    where
        F: FnOnce(&mut ViewContext<V>) -> V,
        V: Render,
    {
        let mut cx = self.app.borrow_mut();
        cx.open_window(WindowOptions::default(), |cx| cx.build_view(build_window))
    }

    pub fn add_empty_window(&mut self) -> AnyWindowHandle {
        let mut cx = self.app.borrow_mut();
        cx.open_window(WindowOptions::default(), |cx| {
            cx.build_view(|_| EmptyView {})
        })
        .any_handle
    }

    pub fn add_window_view<F, V>(&mut self, build_window: F) -> (View<V>, VisualTestContext)
    where
        F: FnOnce(&mut ViewContext<V>) -> V,
        V: Render,
    {
        let mut cx = self.app.borrow_mut();
        let window = cx.open_window(WindowOptions::default(), |cx| cx.build_view(build_window));
        drop(cx);
        let view = window.root_view(self).unwrap();
        (view, VisualTestContext::from_window(*window.deref(), self))
    }

    pub fn simulate_new_path_selection(
        &self,
        _select_path: impl FnOnce(&std::path::Path) -> Option<std::path::PathBuf>,
    ) {
        //
    }

    pub fn simulate_prompt_answer(&self, _button_ix: usize) {
        //
    }

    pub fn has_pending_prompt(&self) -> bool {
        false
    }

    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncAppContext) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.foreground_executor.spawn(f(self.to_async()))
    }

    pub fn has_global<G: 'static>(&self) -> bool {
        let app = self.app.borrow();
        app.has_global::<G>()
    }

    pub fn read_global<G: 'static, R>(&self, read: impl FnOnce(&G, &AppContext) -> R) -> R {
        let app = self.app.borrow();
        read(app.global(), &app)
    }

    pub fn try_read_global<G: 'static, R>(
        &self,
        read: impl FnOnce(&G, &AppContext) -> R,
    ) -> Option<R> {
        let lock = self.app.borrow();
        Some(read(lock.try_global()?, &lock))
    }

    pub fn set_global<G: 'static>(&mut self, global: G) {
        let mut lock = self.app.borrow_mut();
        lock.set_global(global);
    }

    pub fn update_global<G: 'static, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut AppContext) -> R,
    ) -> R {
        let mut lock = self.app.borrow_mut();
        lock.update_global(update)
    }

    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext {
            app: Rc::downgrade(&self.app),
            background_executor: self.background_executor.clone(),
            foreground_executor: self.foreground_executor.clone(),
        }
    }

    pub fn dispatch_keystroke(
        &mut self,
        window: AnyWindowHandle,
        keystroke: Keystroke,
        is_held: bool,
    ) {
        let handled = window
            .update(self, |_, cx| {
                cx.dispatch_event(InputEvent::KeyDown(KeyDownEvent { keystroke, is_held }))
            })
            .is_ok_and(|handled| handled);

        if !handled {
            // todo!() simluate input here
        }
    }

    pub fn notifications<T: 'static>(&mut self, entity: &Model<T>) -> impl Stream<Item = ()> {
        let (tx, rx) = futures::channel::mpsc::unbounded();

        entity.update(self, move |_, cx: &mut ModelContext<T>| {
            cx.observe(entity, {
                let tx = tx.clone();
                move |_, _, _| {
                    let _ = tx.unbounded_send(());
                }
            })
            .detach();

            cx.on_release(move |_, _| tx.close_channel()).detach();
        });

        rx
    }

    pub fn events<Evt, T: 'static + EventEmitter<Evt>>(
        &mut self,
        entity: &Model<T>,
    ) -> futures::channel::mpsc::UnboundedReceiver<Evt>
    where
        Evt: 'static + Clone,
    {
        let (tx, rx) = futures::channel::mpsc::unbounded();
        entity
            .update(self, |_, cx: &mut ModelContext<T>| {
                cx.subscribe(entity, move |_model, _handle, event, _cx| {
                    let _ = tx.unbounded_send(event.clone());
                })
            })
            .detach();
        rx
    }

    pub async fn condition<T: 'static>(
        &mut self,
        model: &Model<T>,
        mut predicate: impl FnMut(&mut T, &mut ModelContext<T>) -> bool,
    ) {
        let timer = self.executor().timer(Duration::from_secs(3));
        let mut notifications = self.notifications(model);

        use futures::FutureExt as _;
        use smol::future::FutureExt as _;

        async {
            while notifications.next().await.is_some() {
                if model.update(self, &mut predicate) {
                    return Ok(());
                }
            }
            bail!("model dropped")
        }
        .race(timer.map(|_| Err(anyhow!("condition timed out"))))
        .await
        .unwrap();
    }
}

impl<T: Send> Model<T> {
    pub fn next_event<Evt>(&self, cx: &mut TestAppContext) -> Evt
    where
        Evt: Send + Clone + 'static,
        T: EventEmitter<Evt>,
    {
        let (tx, mut rx) = futures::channel::mpsc::unbounded();
        let _subscription = self.update(cx, |_, cx| {
            cx.subscribe(self, move |_, _, event, _| {
                tx.unbounded_send(event.clone()).ok();
            })
        });

        cx.executor().run_until_parked();
        rx.try_next()
            .expect("no event received")
            .expect("model was dropped")
    }
}

impl<V> View<V> {
    pub fn condition<Evt>(
        &self,
        cx: &TestAppContext,
        mut predicate: impl FnMut(&V, &AppContext) -> bool,
    ) -> impl Future<Output = ()>
    where
        Evt: 'static,
        V: EventEmitter<Evt>,
    {
        use postage::prelude::{Sink as _, Stream as _};

        let (tx, mut rx) = postage::mpsc::channel(1024);
        let timeout_duration = Duration::from_millis(100); //todo!() cx.condition_duration();

        let mut cx = cx.app.borrow_mut();
        let subscriptions = (
            cx.observe(self, {
                let mut tx = tx.clone();
                move |_, _| {
                    tx.blocking_send(()).ok();
                }
            }),
            cx.subscribe(self, {
                let mut tx = tx.clone();
                move |_, _: &Evt, _| {
                    tx.blocking_send(()).ok();
                }
            }),
        );

        let cx = cx.this.upgrade().unwrap();
        let handle = self.downgrade();

        async move {
            crate::util::timeout(timeout_duration, async move {
                loop {
                    {
                        let cx = cx.borrow();
                        let cx = &*cx;
                        if predicate(
                            handle
                                .upgrade()
                                .expect("view dropped with pending condition")
                                .read(cx),
                            cx,
                        ) {
                            break;
                        }
                    }

                    // todo!(start_waiting)
                    // cx.borrow().foreground_executor().start_waiting();
                    rx.recv()
                        .await
                        .expect("view dropped with pending condition");
                    // cx.borrow().foreground_executor().finish_waiting();
                }
            })
            .await
            .expect("condition timed out");
            drop(subscriptions);
        }
    }
}

use derive_more::{Deref, DerefMut};
#[derive(Deref, DerefMut)]
pub struct VisualTestContext<'a> {
    #[deref]
    #[deref_mut]
    cx: &'a mut TestAppContext,
    window: AnyWindowHandle,
}

impl<'a> VisualTestContext<'a> {
    pub fn from_window(window: AnyWindowHandle, cx: &'a mut TestAppContext) -> Self {
        Self { cx, window }
    }
}

impl<'a> Context for VisualTestContext<'a> {
    type Result<T> = <TestAppContext as Context>::Result<T>;

    fn build_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>> {
        self.cx.build_model(build_model)
    }

    fn update_model<T, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        self.cx.update_model(handle, update)
    }

    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        self.cx.read_model(handle, read)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T,
    {
        self.cx.update_window(window, f)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        self.cx.read_window(window, read)
    }
}

impl<'a> VisualContext for VisualTestContext<'a> {
    fn build_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render,
    {
        self.window
            .update(self.cx, |_, cx| cx.build_view(build_view))
            .unwrap()
    }

    fn update_view<V: 'static, R>(
        &mut self,
        view: &View<V>,
        update: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> Self::Result<R> {
        self.window
            .update(self.cx, |_, cx| cx.update_view(view, update))
            .unwrap()
    }

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: Render,
    {
        self.window
            .update(self.cx, |_, cx| cx.replace_root_view(build_view))
            .unwrap()
    }
}

impl AnyWindowHandle {
    pub fn build_view<V: Render + 'static>(
        &self,
        cx: &mut TestAppContext,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> View<V> {
        self.update(cx, |_, cx| cx.build_view(build_view)).unwrap()
    }
}

pub struct EmptyView {}

impl Render for EmptyView {
    type Element = Div<Self>;

    fn render(&mut self, _cx: &mut crate::ViewContext<Self>) -> Self::Element {
        div()
    }
}
