mod async_context;
mod entity_map;
mod model_context;
#[cfg(any(test, feature = "test-support"))]
mod test_context;

pub use async_context::*;
pub use entity_map::*;
pub use model_context::*;
use refineable::Refineable;
use smallvec::SmallVec;
#[cfg(any(test, feature = "test-support"))]
pub use test_context::*;
use uuid::Uuid;

use crate::{
    current_platform, image_cache::ImageCache, Action, AnyBox, AnyView, AnyWindowHandle,
    AppMetadata, AssetSource, ClipboardItem, Context, DispatchPhase, DisplayId, Entity, Executor,
    FocusEvent, FocusHandle, FocusId, KeyBinding, Keymap, LayoutId, MainThread, MainThreadOnly,
    Pixels, Platform, PlatformDisplay, Point, Render, SharedString, SubscriberSet, Subscription,
    SvgRenderer, Task, TextStyle, TextStyleRefinement, TextSystem, View, ViewContext, Window,
    WindowContext, WindowHandle, WindowId,
};
use anyhow::{anyhow, Result};
use collections::{HashMap, HashSet, VecDeque};
use futures::{future::BoxFuture, Future};
use parking_lot::Mutex;
use slotmap::SlotMap;
use std::{
    any::{type_name, Any, TypeId},
    borrow::Borrow,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    path::PathBuf,
    rc::Rc,
    sync::{atomic::Ordering::SeqCst, Arc, Weak},
    time::Duration,
};
use util::http::{self, HttpClient};

pub struct App(Arc<Mutex<AppContext>>);

/// Represents an application before it is fully launched. Once your app is
/// configured, you'll start the app with `App::run`.
impl App {
    /// Builds an app with the given asset source.
    pub fn production(asset_source: Arc<dyn AssetSource>) -> Self {
        Self(AppContext::new(
            current_platform(),
            asset_source,
            http::client(),
        ))
    }

    /// Start the application. The provided callback will be called once the
    /// app is fully launched.
    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut MainThread<AppContext>),
    {
        let this = self.0.clone();
        let platform = self.0.lock().platform.clone();
        platform.borrow_on_main_thread().run(Box::new(move || {
            let cx = &mut *this.lock();
            let cx = unsafe { mem::transmute::<&mut AppContext, &mut MainThread<AppContext>>(cx) };
            on_finish_launching(cx);
        }));
    }

    /// Register a handler to be invoked when the platform instructs the application
    /// to open one or more URLs.
    pub fn on_open_urls<F>(&self, mut callback: F) -> &Self
    where
        F: 'static + FnMut(Vec<String>, &mut AppContext),
    {
        let this = Arc::downgrade(&self.0);
        self.0
            .lock()
            .platform
            .borrow_on_main_thread()
            .on_open_urls(Box::new(move |urls| {
                if let Some(app) = this.upgrade() {
                    callback(urls, &mut app.lock());
                }
            }));
        self
    }

    pub fn on_reopen<F>(&self, mut callback: F) -> &Self
    where
        F: 'static + FnMut(&mut AppContext),
    {
        let this = Arc::downgrade(&self.0);
        self.0
            .lock()
            .platform
            .borrow_on_main_thread()
            .on_reopen(Box::new(move || {
                if let Some(app) = this.upgrade() {
                    callback(&mut app.lock());
                }
            }));
        self
    }

    pub fn metadata(&self) -> AppMetadata {
        self.0.lock().app_metadata.clone()
    }

    pub fn executor(&self) -> Executor {
        self.0.lock().executor.clone()
    }

    pub fn text_system(&self) -> Arc<TextSystem> {
        self.0.lock().text_system.clone()
    }
}

type ActionBuilder = fn(json: Option<serde_json::Value>) -> anyhow::Result<Box<dyn Action>>;
type FrameCallback = Box<dyn FnOnce(&mut WindowContext) + Send>;
type Handler = Box<dyn FnMut(&mut AppContext) -> bool + Send + 'static>;
type Listener = Box<dyn FnMut(&dyn Any, &mut AppContext) -> bool + Send + 'static>;
type QuitHandler = Box<dyn FnMut(&mut AppContext) -> BoxFuture<'static, ()> + Send + 'static>;
type ReleaseListener = Box<dyn FnOnce(&mut dyn Any, &mut AppContext) + Send + 'static>;

pub struct AppContext {
    this: Weak<Mutex<AppContext>>,
    pub(crate) platform: MainThreadOnly<dyn Platform>,
    app_metadata: AppMetadata,
    text_system: Arc<TextSystem>,
    flushing_effects: bool,
    pending_updates: usize,
    pub(crate) active_drag: Option<AnyDrag>,
    pub(crate) next_frame_callbacks: HashMap<DisplayId, Vec<FrameCallback>>,
    pub(crate) executor: Executor,
    pub(crate) svg_renderer: SvgRenderer,
    asset_source: Arc<dyn AssetSource>,
    pub(crate) image_cache: ImageCache,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) globals_by_type: HashMap<TypeId, AnyBox>,
    pub(crate) entities: EntityMap,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    pub(crate) keymap: Arc<Mutex<Keymap>>,
    pub(crate) global_action_listeners:
        HashMap<TypeId, Vec<Box<dyn Fn(&dyn Action, DispatchPhase, &mut Self) + Send>>>,
    action_builders: HashMap<SharedString, ActionBuilder>,
    pending_effects: VecDeque<Effect>,
    pub(crate) pending_notifications: HashSet<EntityId>,
    pub(crate) pending_global_notifications: HashSet<TypeId>,
    pub(crate) observers: SubscriberSet<EntityId, Handler>,
    pub(crate) event_listeners: SubscriberSet<EntityId, Listener>,
    pub(crate) release_listeners: SubscriberSet<EntityId, ReleaseListener>,
    pub(crate) global_observers: SubscriberSet<TypeId, Handler>,
    pub(crate) quit_observers: SubscriberSet<(), QuitHandler>,
    pub(crate) layout_id_buffer: Vec<LayoutId>, // We recycle this memory across layout requests.
    pub(crate) propagate_event: bool,
}

impl AppContext {
    pub(crate) fn new(
        platform: Arc<dyn Platform>,
        asset_source: Arc<dyn AssetSource>,
        http_client: Arc<dyn HttpClient>,
    ) -> Arc<Mutex<Self>> {
        let executor = platform.executor();
        assert!(
            executor.is_main_thread(),
            "must construct App on main thread"
        );

        let text_system = Arc::new(TextSystem::new(platform.text_system()));
        let entities = EntityMap::new();

        let app_metadata = AppMetadata {
            os_name: platform.os_name(),
            os_version: platform.os_version().ok(),
            app_version: platform.app_version().ok(),
        };

        Arc::new_cyclic(|this| {
            Mutex::new(AppContext {
                this: this.clone(),
                text_system,
                platform: MainThreadOnly::new(platform, executor.clone()),
                app_metadata,
                flushing_effects: false,
                pending_updates: 0,
                next_frame_callbacks: Default::default(),
                executor,
                svg_renderer: SvgRenderer::new(asset_source.clone()),
                asset_source,
                image_cache: ImageCache::new(http_client),
                text_style_stack: Vec::new(),
                globals_by_type: HashMap::default(),
                entities,
                windows: SlotMap::with_key(),
                keymap: Arc::new(Mutex::new(Keymap::default())),
                global_action_listeners: HashMap::default(),
                action_builders: HashMap::default(),
                pending_effects: VecDeque::new(),
                pending_notifications: HashSet::default(),
                pending_global_notifications: HashSet::default(),
                observers: SubscriberSet::new(),
                event_listeners: SubscriberSet::new(),
                release_listeners: SubscriberSet::new(),
                global_observers: SubscriberSet::new(),
                quit_observers: SubscriberSet::new(),
                layout_id_buffer: Default::default(),
                propagate_event: true,
                active_drag: None,
            })
        })
    }

    /// Quit the application gracefully. Handlers registered with `ModelContext::on_app_quit`
    /// will be given 100ms to complete before exiting.
    pub fn quit(&mut self) {
        let mut futures = Vec::new();

        self.quit_observers.clone().retain(&(), |observer| {
            futures.push(observer(self));
            true
        });

        self.windows.clear();
        self.flush_effects();

        let futures = futures::future::join_all(futures);
        if self
            .executor
            .block_with_timeout(Duration::from_millis(100), futures)
            .is_err()
        {
            log::error!("timed out waiting on app_will_quit");
        }

        self.globals_by_type.clear();
    }

    pub fn app_metadata(&self) -> AppMetadata {
        self.app_metadata.clone()
    }

    /// Schedules all windows in the application to be redrawn. This can be called
    /// multiple times in an update cycle and still result in a single redraw.
    pub fn refresh(&mut self) {
        self.pending_effects.push_back(Effect::Refresh);
    }

    pub(crate) fn update<R>(&mut self, update: impl FnOnce(&mut Self) -> R) -> R {
        self.pending_updates += 1;
        let result = update(self);
        if !self.flushing_effects && self.pending_updates == 1 {
            self.flushing_effects = true;
            self.flush_effects();
            self.flushing_effects = false;
        }
        self.pending_updates -= 1;
        result
    }

    pub fn windows(&self) -> Vec<AnyWindowHandle> {
        self.windows
            .values()
            .filter_map(|window| Some(window.as_ref()?.handle.clone()))
            .collect()
    }

    pub fn update_window_root<V, R>(
        &mut self,
        handle: &WindowHandle<V>,
        update: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> Result<R>
    where
        V: 'static + Send,
    {
        self.update_window(handle.any_handle, |cx| {
            let root_view = cx
                .window
                .root_view
                .as_ref()
                .unwrap()
                .clone()
                .downcast()
                .unwrap();
            root_view.update(cx, update)
        })
    }

    pub(crate) fn push_effect(&mut self, effect: Effect) {
        match &effect {
            Effect::Notify { emitter } => {
                if !self.pending_notifications.insert(*emitter) {
                    return;
                }
            }
            Effect::NotifyGlobalObservers { global_type } => {
                if !self.pending_global_notifications.insert(*global_type) {
                    return;
                }
            }
            _ => {}
        };

        self.pending_effects.push_back(effect);
    }

    /// Called at the end of AppContext::update to complete any side effects
    /// such as notifying observers, emitting events, etc. Effects can themselves
    /// cause effects, so we continue looping until all effects are processed.
    fn flush_effects(&mut self) {
        loop {
            self.release_dropped_entities();
            self.release_dropped_focus_handles();
            if let Some(effect) = self.pending_effects.pop_front() {
                match effect {
                    Effect::Notify { emitter } => {
                        self.apply_notify_effect(emitter);
                    }
                    Effect::Emit { emitter, event } => self.apply_emit_effect(emitter, event),
                    Effect::FocusChanged {
                        window_handle,
                        focused,
                    } => {
                        self.apply_focus_changed_effect(window_handle, focused);
                    }
                    Effect::Refresh => {
                        self.apply_refresh_effect();
                    }
                    Effect::NotifyGlobalObservers { global_type } => {
                        self.apply_notify_global_observers_effect(global_type);
                    }
                    Effect::Defer { callback } => {
                        self.apply_defer_effect(callback);
                    }
                }
            } else {
                break;
            }
        }

        let dirty_window_ids = self
            .windows
            .iter()
            .filter_map(|(_, window)| {
                let window = window.as_ref().unwrap();
                if window.dirty {
                    Some(window.handle.clone())
                } else {
                    None
                }
            })
            .collect::<SmallVec<[_; 8]>>();

        for dirty_window_handle in dirty_window_ids {
            self.update_window(dirty_window_handle, |cx| cx.draw())
                .unwrap();
        }
    }

    /// Repeatedly called during `flush_effects` to release any entities whose
    /// reference count has become zero. We invoke any release observers before dropping
    /// each entity.
    fn release_dropped_entities(&mut self) {
        loop {
            let dropped = self.entities.take_dropped();
            if dropped.is_empty() {
                break;
            }

            for (entity_id, mut entity) in dropped {
                self.observers.remove(&entity_id);
                self.event_listeners.remove(&entity_id);
                for release_callback in self.release_listeners.remove(&entity_id) {
                    release_callback(&mut entity, self);
                }
            }
        }
    }

    /// Repeatedly called during `flush_effects` to handle a focused handle being dropped.
    /// For now, we simply blur the window if this happens, but we may want to support invoking
    /// a window blur handler to restore focus to some logical element.
    fn release_dropped_focus_handles(&mut self) {
        for window_handle in self.windows() {
            self.update_window(window_handle, |cx| {
                let mut blur_window = false;
                let focus = cx.window.focus;
                cx.window.focus_handles.write().retain(|handle_id, count| {
                    if count.load(SeqCst) == 0 {
                        if focus == Some(handle_id) {
                            blur_window = true;
                        }
                        false
                    } else {
                        true
                    }
                });

                if blur_window {
                    cx.blur();
                }
            })
            .unwrap();
        }
    }

    fn apply_notify_effect(&mut self, emitter: EntityId) {
        self.pending_notifications.remove(&emitter);
        self.observers
            .clone()
            .retain(&emitter, |handler| handler(self));
    }

    fn apply_emit_effect(&mut self, emitter: EntityId, event: Box<dyn Any>) {
        self.event_listeners
            .clone()
            .retain(&emitter, |handler| handler(event.as_ref(), self));
    }

    fn apply_focus_changed_effect(
        &mut self,
        window_handle: AnyWindowHandle,
        focused: Option<FocusId>,
    ) {
        self.update_window(window_handle, |cx| {
            if cx.window.focus == focused {
                let mut listeners = mem::take(&mut cx.window.focus_listeners);
                let focused =
                    focused.map(|id| FocusHandle::for_id(id, &cx.window.focus_handles).unwrap());
                let blurred = cx
                    .window
                    .last_blur
                    .take()
                    .unwrap()
                    .and_then(|id| FocusHandle::for_id(id, &cx.window.focus_handles));
                if focused.is_some() || blurred.is_some() {
                    let event = FocusEvent { focused, blurred };
                    for listener in &listeners {
                        listener(&event, cx);
                    }
                }

                listeners.extend(cx.window.focus_listeners.drain(..));
                cx.window.focus_listeners = listeners;
            }
        })
        .ok();
    }

    fn apply_refresh_effect(&mut self) {
        for window in self.windows.values_mut() {
            if let Some(window) = window.as_mut() {
                window.dirty = true;
            }
        }
    }

    fn apply_notify_global_observers_effect(&mut self, type_id: TypeId) {
        self.pending_global_notifications.remove(&type_id);
        self.global_observers
            .clone()
            .retain(&type_id, |observer| observer(self));
    }

    fn apply_defer_effect(&mut self, callback: Box<dyn FnOnce(&mut Self) + Send + 'static>) {
        callback(self);
    }

    /// Creates an `AsyncAppContext`, which can be cloned and has a static lifetime
    /// so it can be held across `await` points.
    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext {
            app: unsafe { mem::transmute(self.this.clone()) },
            executor: self.executor.clone(),
        }
    }

    /// Obtains a reference to the executor, which can be used to spawn futures.
    pub fn executor(&self) -> &Executor {
        &self.executor
    }

    /// Runs the given closure on the main thread, where interaction with the platform
    /// is possible. The given closure will be invoked with a `MainThread<AppContext>`, which
    /// has platform-specific methods that aren't present on `AppContext`.
    pub fn run_on_main<R>(
        &mut self,
        f: impl FnOnce(&mut MainThread<AppContext>) -> R + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
    {
        if self.executor.is_main_thread() {
            Task::ready(f(unsafe {
                mem::transmute::<&mut AppContext, &mut MainThread<AppContext>>(self)
            }))
        } else {
            let this = self.this.upgrade().unwrap();
            self.executor.run_on_main(move || {
                let cx = &mut *this.lock();
                cx.update(|cx| f(unsafe { mem::transmute::<&mut Self, &mut MainThread<Self>>(cx) }))
            })
        }
    }

    /// Spawns the future returned by the given function on the main thread, where interaction with
    /// the platform is possible. The given closure will be invoked with a `MainThread<AsyncAppContext>`,
    /// which has platform-specific methods that aren't present on `AsyncAppContext`. The future will be
    /// polled exclusively on the main thread.
    // todo!("I think we need somehow to prevent the MainThread<AsyncAppContext> from implementing Send")
    pub fn spawn_on_main<F, R>(
        &self,
        f: impl FnOnce(MainThread<AsyncAppContext>) -> F + Send + 'static,
    ) -> Task<R>
    where
        F: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let cx = self.to_async();
        self.executor.spawn_on_main(move || f(MainThread(cx)))
    }

    /// Spawns the future returned by the given function on the thread pool. The closure will be invoked
    /// with AsyncAppContext, which allows the application state to be accessed across await points.
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncAppContext) -> Fut + Send + 'static) -> Task<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let cx = self.to_async();
        self.executor.spawn(async move {
            let future = f(cx);
            future.await
        })
    }

    /// Schedules the given function to be run at the end of the current effect cycle, allowing entities
    /// that are currently on the stack to be returned to the app.
    pub fn defer(&mut self, f: impl FnOnce(&mut AppContext) + 'static + Send) {
        self.push_effect(Effect::Defer {
            callback: Box::new(f),
        });
    }

    /// Accessor for the application's asset source, which is provided when constructing the `App`.
    pub fn asset_source(&self) -> &Arc<dyn AssetSource> {
        &self.asset_source
    }

    /// Accessor for the text system.
    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    /// The current text style. Which is composed of all the style refinements provided to `with_text_style`.
    pub fn text_style(&self) -> TextStyle {
        let mut style = TextStyle::default();
        for refinement in &self.text_style_stack {
            style.refine(refinement);
        }
        style
    }

    /// Check whether a global of the given type has been assigned.
    pub fn has_global<G: 'static>(&self) -> bool {
        self.globals_by_type.contains_key(&TypeId::of::<G>())
    }

    /// Access the global of the given type. Panics if a global for that type has not been assigned.
    pub fn global<G: 'static>(&self) -> &G {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    /// Access the global of the given type if a value has been assigned.
    pub fn try_global<G: 'static>(&self) -> Option<&G> {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
    }

    /// Access the global of the given type mutably. Panics if a global for that type has not been assigned.
    pub fn global_mut<G: 'static>(&mut self) -> &mut G {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type
            .get_mut(&global_type)
            .and_then(|any_state| any_state.downcast_mut::<G>())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    /// Access the global of the given type mutably. A default value is assigned if a global of this type has not
    /// yet been assigned.
    pub fn default_global<G: 'static + Default + Send>(&mut self) -> &mut G {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type
            .entry(global_type)
            .or_insert_with(|| Box::new(G::default()))
            .downcast_mut::<G>()
            .unwrap()
    }

    /// Set the value of the global of the given type.
    pub fn set_global<G: Any + Send>(&mut self, global: G) {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type.insert(global_type, Box::new(global));
    }

    /// Update the global of the given type with a closure. Unlike `global_mut`, this method provides
    /// your closure with mutable access to the `AppContext` and the global simultaneously.
    pub fn update_global<G: 'static, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R {
        let mut global = self.lease_global::<G>();
        let result = f(&mut global, self);
        self.end_global_lease(global);
        result
    }

    /// Register a callback to be invoked when a global of the given type is updated.
    pub fn observe_global<G: 'static>(
        &mut self,
        mut f: impl FnMut(&mut Self) + Send + 'static,
    ) -> Subscription {
        self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| {
                f(cx);
                true
            }),
        )
    }

    pub fn all_action_names<'a>(&'a self) -> impl Iterator<Item = SharedString> + 'a {
        self.action_builders.keys().cloned()
    }

    /// Move the global of the given type to the stack.
    pub(crate) fn lease_global<G: 'static>(&mut self) -> GlobalLease<G> {
        GlobalLease::new(
            self.globals_by_type
                .remove(&TypeId::of::<G>())
                .ok_or_else(|| anyhow!("no global registered of type {}", type_name::<G>()))
                .unwrap(),
        )
    }

    /// Restore the global of the given type after it is moved to the stack.
    pub(crate) fn end_global_lease<G: 'static>(&mut self, lease: GlobalLease<G>) {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type.insert(global_type, lease.global);
    }

    pub fn observe_release<E, T>(
        &mut self,
        handle: &E,
        on_release: impl FnOnce(&mut T, &mut AppContext) + Send + 'static,
    ) -> Subscription
    where
        E: Entity<T>,
        T: 'static,
    {
        self.release_listeners.insert(
            handle.entity_id(),
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                on_release(entity, cx)
            }),
        )
    }

    pub(crate) fn push_text_style(&mut self, text_style: TextStyleRefinement) {
        self.text_style_stack.push(text_style);
    }

    pub(crate) fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
    }

    /// Register key bindings.
    pub fn bind_keys(&mut self, bindings: impl IntoIterator<Item = KeyBinding>) {
        self.keymap.lock().add_bindings(bindings);
        self.pending_effects.push_back(Effect::Refresh);
    }

    /// Register a global listener for actions invoked via the keyboard.
    pub fn on_action<A: Action>(&mut self, listener: impl Fn(&A, &mut Self) + Send + 'static) {
        self.global_action_listeners
            .entry(TypeId::of::<A>())
            .or_default()
            .push(Box::new(move |action, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    let action = action.as_any().downcast_ref().unwrap();
                    listener(action, cx)
                }
            }));
    }

    /// Register an action type to allow it to be referenced in keymaps.
    pub fn register_action_type<A: Action>(&mut self) {
        self.action_builders.insert(A::qualified_name(), A::build);
    }

    /// Construct an action based on its name and parameters.
    pub fn build_action(
        &mut self,
        name: &str,
        params: Option<serde_json::Value>,
    ) -> Result<Box<dyn Action>> {
        let build = self
            .action_builders
            .get(name)
            .ok_or_else(|| anyhow!("no action type registered for {}", name))?;
        (build)(params)
    }

    /// Halt propagation of a mouse event, keyboard event, or action. This prevents listeners
    /// that have not yet been invoked from receiving the event.
    pub fn stop_propagation(&mut self) {
        self.propagate_event = false;
    }
}

impl Context for AppContext {
    type WindowContext<'a> = WindowContext<'a>;
    type ModelContext<'a, T> = ModelContext<'a, T>;
    type Result<T> = T;

    /// Build an entity that is owned by the application. The given function will be invoked with
    /// a `ModelContext` and must return an object representing the entity. A `Model` will be returned
    /// which can be used to access the entity in a context.
    fn build_model<T: 'static + Send>(
        &mut self,
        build_model: impl FnOnce(&mut Self::ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.update(|cx| {
            let slot = cx.entities.reserve();
            let entity = build_model(&mut ModelContext::new(cx, slot.downgrade()));
            cx.entities.insert(slot, entity)
        })
    }

    /// Update the entity referenced by the given model. The function is passed a mutable reference to the
    /// entity along with a `ModelContext` for the entity.
    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut Self::ModelContext<'_, T>) -> R,
    ) -> R {
        self.update(|cx| {
            let mut entity = cx.entities.lease(model);
            let result = update(&mut entity, &mut ModelContext::new(cx, model.downgrade()));
            cx.entities.end_lease(entity);
            result
        })
    }

    fn update_window<T, F>(&mut self, handle: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(&mut Self::WindowContext<'_>) -> T,
    {
        self.update(|cx| {
            let mut window = cx
                .windows
                .get_mut(handle.id)
                .ok_or_else(|| anyhow!("window not found"))?
                .take()
                .unwrap();

            let result = update(&mut WindowContext::new(cx, &mut window));

            cx.windows
                .get_mut(handle.id)
                .ok_or_else(|| anyhow!("window not found"))?
                .replace(window);

            Ok(result)
        })
    }
}

impl<C> MainThread<C>
where
    C: Borrow<AppContext>,
{
    pub(crate) fn platform(&self) -> &dyn Platform {
        self.0.borrow().platform.borrow_on_main_thread()
    }

    /// Instructs the platform to activate the application by bringing it to the foreground.
    pub fn activate(&self, ignoring_other_apps: bool) {
        self.platform().activate(ignoring_other_apps);
    }

    /// Writes data to the platform clipboard.
    pub fn write_to_clipboard(&self, item: ClipboardItem) {
        self.platform().write_to_clipboard(item)
    }

    /// Reads data from the platform clipboard.
    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.platform().read_from_clipboard()
    }

    /// Writes credentials to the platform keychain.
    pub fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Result<()> {
        self.platform().write_credentials(url, username, password)
    }

    /// Reads credentials from the platform keychain.
    pub fn read_credentials(&self, url: &str) -> Result<Option<(String, Vec<u8>)>> {
        self.platform().read_credentials(url)
    }

    /// Deletes credentials from the platform keychain.
    pub fn delete_credentials(&self, url: &str) -> Result<()> {
        self.platform().delete_credentials(url)
    }

    /// Directs the platform's default browser to open the given URL.
    pub fn open_url(&self, url: &str) {
        self.platform().open_url(url);
    }

    pub fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        self.platform().path_for_auxiliary_executable(name)
    }

    pub fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.platform().displays()
    }

    pub fn display_for_uuid(&self, uuid: Uuid) -> Option<Rc<dyn PlatformDisplay>> {
        self.platform()
            .displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    }
}

impl MainThread<AppContext> {
    fn update<R>(&mut self, update: impl FnOnce(&mut Self) -> R) -> R {
        self.0.update(|cx| {
            update(unsafe {
                std::mem::transmute::<&mut AppContext, &mut MainThread<AppContext>>(cx)
            })
        })
    }

    pub fn update_window<R>(
        &mut self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&mut MainThread<WindowContext>) -> R,
    ) -> Result<R> {
        self.0.update_window(handle, |cx| {
            update(unsafe {
                std::mem::transmute::<&mut WindowContext, &mut MainThread<WindowContext>>(cx)
            })
        })
    }

    pub fn update_window_root<V, R>(
        &mut self,
        handle: &WindowHandle<V>,
        update: impl FnOnce(&mut V, &mut MainThread<ViewContext<'_, V>>) -> R,
    ) -> Result<R>
    where
        V: 'static + Send,
    {
        self.update_window(handle.any_handle, |cx| {
            let root_view = cx
                .window
                .root_view
                .as_ref()
                .unwrap()
                .clone()
                .downcast()
                .unwrap();
            root_view.update(cx, update)
        })
    }

    /// Opens a new window with the given option and the root view returned by the given function.
    /// The function is invoked with a `WindowContext`, which can be used to interact with window-specific
    /// functionality.
    pub fn open_window<V: Render>(
        &mut self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut MainThread<WindowContext>) -> View<V> + Send + 'static,
    ) -> WindowHandle<V> {
        self.update(|cx| {
            let id = cx.windows.insert(None);
            let handle = WindowHandle::new(id);
            let mut window = Window::new(handle.into(), options, cx);
            let mut window_context = MainThread(WindowContext::new(cx, &mut window));
            let root_view = build_root_view(&mut window_context);
            window.root_view.replace(root_view.into());
            cx.windows.get_mut(id).unwrap().replace(window);
            handle
        })
    }

    /// Update the global of the given type with a closure. Unlike `global_mut`, this method provides
    /// your closure with mutable access to the `MainThread<AppContext>` and the global simultaneously.
    pub fn update_global<G: 'static + Send, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut MainThread<AppContext>) -> R,
    ) -> R {
        self.0.update_global(|global, cx| {
            let cx = unsafe { mem::transmute::<&mut AppContext, &mut MainThread<AppContext>>(cx) };
            update(global, cx)
        })
    }
}

/// These effects are processed at the end of each application update cycle.
pub(crate) enum Effect {
    Notify {
        emitter: EntityId,
    },
    Emit {
        emitter: EntityId,
        event: Box<dyn Any + Send + 'static>,
    },
    FocusChanged {
        window_handle: AnyWindowHandle,
        focused: Option<FocusId>,
    },
    Refresh,
    NotifyGlobalObservers {
        global_type: TypeId,
    },
    Defer {
        callback: Box<dyn FnOnce(&mut AppContext) + Send + 'static>,
    },
}

/// Wraps a global variable value during `update_global` while the value has been moved to the stack.
pub(crate) struct GlobalLease<G: 'static> {
    global: AnyBox,
    global_type: PhantomData<G>,
}

impl<G: 'static> GlobalLease<G> {
    fn new(global: AnyBox) -> Self {
        GlobalLease {
            global,
            global_type: PhantomData,
        }
    }
}

impl<G: 'static> Deref for GlobalLease<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        self.global.downcast_ref().unwrap()
    }
}

impl<G: 'static> DerefMut for GlobalLease<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.global.downcast_mut().unwrap()
    }
}

/// Contains state associated with an active drag operation, started by dragging an element
/// within the window or by dragging into the app from the underlying platform.
pub(crate) struct AnyDrag {
    pub view: AnyView,
    pub cursor_offset: Point<Pixels>,
}

#[cfg(test)]
mod tests {
    use super::AppContext;

    #[test]
    fn test_app_context_send_sync() {
        // This will not compile if `AppContext` does not implement `Send`
        fn assert_send<T: Send>() {}
        assert_send::<AppContext>();
    }
}
