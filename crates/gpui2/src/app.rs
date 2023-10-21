mod async_context;
mod entity_map;
mod model_context;

pub use async_context::*;
pub use entity_map::*;
pub use model_context::*;
use refineable::Refineable;
use smallvec::SmallVec;

use crate::{
    current_platform, image_cache::ImageCache, Action, AssetSource, Context, DisplayId, Executor,
    FocusEvent, FocusHandle, FocusId, KeyBinding, Keymap, LayoutId, MainThread, MainThreadOnly,
    Platform, SemanticVersion, SharedString, SubscriberSet, SvgRenderer, Task, TextStyle,
    TextStyleRefinement, TextSystem, View, Window, WindowContext, WindowHandle, WindowId,
};
use anyhow::{anyhow, Result};
use collections::{HashMap, HashSet, VecDeque};
use futures::Future;
use parking_lot::{Mutex, RwLock};
use slotmap::SlotMap;
use std::{
    any::{type_name, Any, TypeId},
    mem,
    sync::{atomic::Ordering::SeqCst, Arc, Weak},
};
use util::http::{self, HttpClient};

pub struct App(Arc<Mutex<AppContext>>);

impl App {
    pub fn production(asset_source: Arc<dyn AssetSource>) -> Self {
        let http_client = http::client();
        Self::new(current_platform(), asset_source, http_client)
    }

    #[cfg(any(test, feature = "test"))]
    pub fn test() -> Self {
        let platform = Arc::new(super::TestPlatform::new());
        let asset_source = Arc::new(());
        let http_client = util::http::FakeHttpClient::with_404_response();
        Self::new(platform, asset_source, http_client)
    }

    fn new(
        platform: Arc<dyn Platform>,
        asset_source: Arc<dyn AssetSource>,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        let executor = platform.executor();
        let entities = EntityMap::new();
        let unit_entity = entities.insert(entities.reserve(), ());
        Self(Arc::new_cyclic(|this| {
            Mutex::new(AppContext {
                this: this.clone(),
                text_system: Arc::new(TextSystem::new(platform.text_system())),
                platform: MainThreadOnly::new(platform, executor.clone()),
                flushing_effects: false,
                pending_updates: 0,
                next_frame_callbacks: Default::default(),
                executor,
                svg_renderer: SvgRenderer::new(asset_source),
                image_cache: ImageCache::new(http_client),
                text_style_stack: Vec::new(),
                global_stacks_by_type: HashMap::default(),
                unit_entity,
                entities,
                windows: SlotMap::with_key(),
                keymap: Arc::new(RwLock::new(Keymap::default())),
                action_builders: HashMap::default(),
                pending_notifications: Default::default(),
                pending_effects: Default::default(),
                observers: SubscriberSet::new(),
                event_handlers: SubscriberSet::new(),
                release_handlers: SubscriberSet::new(),
                layout_id_buffer: Default::default(),
            })
        }))
    }

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

    pub fn app_version(&self) -> Result<SemanticVersion> {
        self.0.lock().platform.borrow_on_main_thread().app_version()
    }

    pub fn os_name(&self) -> &'static str {
        self.0.lock().platform.borrow_on_main_thread().os_name()
    }

    pub fn os_version(&self) -> Result<SemanticVersion> {
        self.0.lock().platform.borrow_on_main_thread().os_version()
    }

    pub fn executor(&self) -> Executor {
        self.0.lock().executor.clone()
    }

    pub fn text_system(&self) -> Arc<TextSystem> {
        self.0.lock().text_system.clone()
    }
}

type Handler = Box<dyn Fn(&mut AppContext) -> bool + Send + Sync + 'static>;
type EventHandler = Box<dyn Fn(&dyn Any, &mut AppContext) -> bool + Send + Sync + 'static>;
type ReleaseHandler = Box<dyn Fn(&mut dyn Any, &mut AppContext) + Send + Sync + 'static>;
type FrameCallback = Box<dyn FnOnce(&mut WindowContext) + Send>;
type ActionBuilder = fn(json: Option<serde_json::Value>) -> anyhow::Result<Box<dyn Action>>;

pub struct AppContext {
    this: Weak<Mutex<AppContext>>,
    pub(crate) platform: MainThreadOnly<dyn Platform>,
    text_system: Arc<TextSystem>,
    flushing_effects: bool,
    pending_updates: usize,
    pub(crate) next_frame_callbacks: HashMap<DisplayId, Vec<FrameCallback>>,
    pub(crate) executor: Executor,
    pub(crate) svg_renderer: SvgRenderer,
    pub(crate) image_cache: ImageCache,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) global_stacks_by_type: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
    pub(crate) unit_entity: Handle<()>,
    pub(crate) entities: EntityMap,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    pub(crate) keymap: Arc<RwLock<Keymap>>,
    action_builders: HashMap<SharedString, ActionBuilder>,
    pub(crate) pending_notifications: HashSet<EntityId>,
    pending_effects: VecDeque<Effect>,
    pub(crate) observers: SubscriberSet<EntityId, Handler>,
    pub(crate) event_handlers: SubscriberSet<EntityId, EventHandler>,
    pub(crate) release_handlers: SubscriberSet<EntityId, ReleaseHandler>,
    pub(crate) layout_id_buffer: Vec<LayoutId>, // We recycle this memory across layout requests.
}

impl AppContext {
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

    pub(crate) fn read_window<R>(
        &mut self,
        id: WindowId,
        read: impl FnOnce(&WindowContext) -> R,
    ) -> Result<R> {
        let window = self
            .windows
            .get(id)
            .ok_or_else(|| anyhow!("window not found"))?
            .as_ref()
            .unwrap();
        Ok(read(&WindowContext::immutable(self, &window)))
    }

    pub(crate) fn update_window<R>(
        &mut self,
        id: WindowId,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        self.update(|cx| {
            let mut window = cx
                .windows
                .get_mut(id)
                .ok_or_else(|| anyhow!("window not found"))?
                .take()
                .unwrap();

            let result = update(&mut WindowContext::mutable(cx, &mut window));

            cx.windows
                .get_mut(id)
                .ok_or_else(|| anyhow!("window not found"))?
                .replace(window);

            Ok(result)
        })
    }

    pub(crate) fn push_effect(&mut self, effect: Effect) {
        match &effect {
            Effect::Notify { emitter } => {
                if self.pending_notifications.insert(*emitter) {
                    self.pending_effects.push_back(effect);
                }
            }
            Effect::Emit { .. } => self.pending_effects.push_back(effect),
            Effect::FocusChanged { .. } => self.pending_effects.push_back(effect),
            Effect::Refresh => self.pending_effects.push_back(effect),
        }
    }

    fn flush_effects(&mut self) {
        loop {
            self.release_dropped_entities();
            self.release_dropped_focus_handles();
            if let Some(effect) = self.pending_effects.pop_front() {
                match effect {
                    Effect::Notify { emitter } => self.apply_notify_effect(emitter),
                    Effect::Emit { emitter, event } => self.apply_emit_effect(emitter, event),
                    Effect::FocusChanged { window_id, focused } => {
                        self.apply_focus_changed(window_id, focused)
                    }
                    Effect::Refresh => {
                        self.apply_refresh();
                    }
                }
            } else {
                break;
            }
        }

        let dirty_window_ids = self
            .windows
            .iter()
            .filter_map(|(window_id, window)| {
                let window = window.as_ref().unwrap();
                if window.dirty {
                    Some(window_id)
                } else {
                    None
                }
            })
            .collect::<SmallVec<[_; 8]>>();

        for dirty_window_id in dirty_window_ids {
            self.update_window(dirty_window_id, |cx| cx.draw()).unwrap();
        }
    }

    fn release_dropped_entities(&mut self) {
        loop {
            let dropped = self.entities.take_dropped();
            if dropped.is_empty() {
                break;
            }

            for (entity_id, mut entity) in dropped {
                self.observers.remove(&entity_id);
                self.event_handlers.remove(&entity_id);
                for release_callback in self.release_handlers.remove(&entity_id) {
                    release_callback(&mut entity, self);
                }
            }
        }
    }

    fn release_dropped_focus_handles(&mut self) {
        let window_ids = self.windows.keys().collect::<SmallVec<[_; 8]>>();
        for window_id in window_ids {
            self.update_window(window_id, |cx| {
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
        self.event_handlers
            .clone()
            .retain(&emitter, |handler| handler(&event, self));
    }

    fn apply_focus_changed(&mut self, window_id: WindowId, focused: Option<FocusId>) {
        self.update_window(window_id, |cx| {
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

    fn apply_refresh(&mut self) {
        for window in self.windows.values_mut() {
            if let Some(window) = window.as_mut() {
                window.dirty = true;
            }
        }
    }

    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext(unsafe { mem::transmute(self.this.clone()) })
    }

    pub fn executor(&self) -> &Executor {
        &self.executor
    }

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

    pub fn spawn_on_main<F, R>(
        &self,
        f: impl FnOnce(&mut MainThread<AppContext>) -> F + Send + 'static,
    ) -> Task<R>
    where
        F: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let this = self.this.upgrade().unwrap();
        self.executor.spawn_on_main(move || {
            let cx = &mut *this.lock();
            cx.update(|cx| {
                f(unsafe { mem::transmute::<&mut AppContext, &mut MainThread<AppContext>>(cx) })
            })
        })
    }

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

    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    pub fn text_style(&self) -> TextStyle {
        let mut style = TextStyle::default();
        for refinement in &self.text_style_stack {
            style.refine(refinement);
        }
        style
    }

    pub fn global<G: 'static>(&self) -> &G {
        self.global_stacks_by_type
            .get(&TypeId::of::<G>())
            .and_then(|stack| stack.last())
            .and_then(|any_state| any_state.downcast_ref::<G>())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    pub fn global_mut<G: 'static>(&mut self) -> &mut G {
        self.global_stacks_by_type
            .get_mut(&TypeId::of::<G>())
            .and_then(|stack| stack.last_mut())
            .and_then(|any_state| any_state.downcast_mut::<G>())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    pub fn default_global<G: 'static + Default + Sync + Send>(&mut self) -> &mut G {
        let stack = self
            .global_stacks_by_type
            .entry(TypeId::of::<G>())
            .or_default();
        if stack.is_empty() {
            stack.push(Box::new(G::default()));
        }
        stack.last_mut().unwrap().downcast_mut::<G>().unwrap()
    }

    pub fn set_global<T: Send + Sync + 'static>(&mut self, global: T) {
        let global = Box::new(global);
        let stack = self
            .global_stacks_by_type
            .entry(TypeId::of::<T>())
            .or_default();
        if let Some(last) = stack.last_mut() {
            *last = global;
        } else {
            stack.push(global)
        }
    }

    pub(crate) fn push_global<T: Send + Sync + 'static>(&mut self, global: T) {
        self.global_stacks_by_type
            .entry(TypeId::of::<T>())
            .or_default()
            .push(Box::new(global));
    }

    pub(crate) fn pop_global<T: 'static>(&mut self) -> Box<T> {
        self.global_stacks_by_type
            .get_mut(&TypeId::of::<T>())
            .and_then(|stack| stack.pop())
            .expect("state stack underflow")
            .downcast()
            .unwrap()
    }

    pub(crate) fn push_text_style(&mut self, text_style: TextStyleRefinement) {
        self.text_style_stack.push(text_style);
    }

    pub(crate) fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
    }

    pub fn bind_keys(&mut self, bindings: impl IntoIterator<Item = KeyBinding>) {
        self.keymap.write().add_bindings(bindings);
        self.push_effect(Effect::Refresh);
    }

    pub fn register_action_type<A: Action>(&mut self) {
        self.action_builders.insert(A::qualified_name(), A::build);
    }

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
}

impl Context for AppContext {
    type BorrowedContext<'a, 'w> = Self;
    type EntityContext<'a, 'w, T: Send + Sync + 'static> = ModelContext<'a, T>;
    type Result<T> = T;

    fn refresh(&mut self) {
        self.push_effect(Effect::Refresh);
    }

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        self.update(|cx| {
            let slot = cx.entities.reserve();
            let entity = build_entity(&mut ModelContext::mutable(cx, slot.id));
            cx.entities.insert(slot, entity)
        })
    }

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        self.update(|cx| {
            let mut entity = cx.entities.lease(handle);
            let result = update(&mut entity, &mut ModelContext::mutable(cx, handle.id));
            cx.entities.end_lease(entity);
            result
        })
    }

    fn read_global<G: 'static + Send + Sync, R>(&self, read: impl FnOnce(&G, &Self) -> R) -> R {
        read(self.global(), self)
    }

    fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static + Send + Sync,
    {
        let mut global = self
            .global_stacks_by_type
            .get_mut(&TypeId::of::<G>())
            .and_then(|stack| stack.pop())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap();
        let result = f(global.downcast_mut().unwrap(), self);
        self.global_stacks_by_type
            .get_mut(&TypeId::of::<G>())
            .unwrap()
            .push(global);
        result
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

    pub(crate) fn update_window<R>(
        &mut self,
        id: WindowId,
        update: impl FnOnce(&mut MainThread<WindowContext>) -> R,
    ) -> Result<R> {
        self.0.update_window(id, |cx| {
            update(unsafe {
                std::mem::transmute::<&mut WindowContext, &mut MainThread<WindowContext>>(cx)
            })
        })
    }

    pub(crate) fn platform(&self) -> &dyn Platform {
        self.platform.borrow_on_main_thread()
    }

    pub fn activate(&mut self, ignoring_other_apps: bool) {
        self.platform().activate(ignoring_other_apps);
    }

    pub fn open_window<S: 'static + Send + Sync>(
        &mut self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut WindowContext) -> View<S> + Send + 'static,
    ) -> WindowHandle<S> {
        self.update(|cx| {
            let id = cx.windows.insert(None);
            let handle = WindowHandle::new(id);
            let mut window = Window::new(handle.into(), options, cx);
            let root_view = build_root_view(&mut WindowContext::mutable(cx, &mut window));
            window.root_view.replace(root_view.into_any());
            cx.windows.get_mut(id).unwrap().replace(window);
            handle
        })
    }
}

pub(crate) enum Effect {
    Notify {
        emitter: EntityId,
    },
    Emit {
        emitter: EntityId,
        event: Box<dyn Any + Send + Sync + 'static>,
    },
    FocusChanged {
        window_id: WindowId,
        focused: Option<FocusId>,
    },
    Refresh,
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
