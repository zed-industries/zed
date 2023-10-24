mod async_context;
mod entity_map;
mod model_context;

pub use async_context::*;
pub use entity_map::*;
pub use model_context::*;
use refineable::Refineable;
use smallvec::SmallVec;

use crate::{
    current_platform, image_cache::ImageCache, Action, AnyBox, AnyView, AppMetadata, AssetSource,
    ClipboardItem, Context, DispatchPhase, DisplayId, Executor, FocusEvent, FocusHandle, FocusId,
    KeyBinding, Keymap, LayoutId, MainThread, MainThreadOnly, Pixels, Platform, Point,
    SharedString, SubscriberSet, Subscription, SvgRenderer, Task, TextStyle, TextStyleRefinement,
    TextSystem, View, Window, WindowContext, WindowHandle, WindowId,
};
use anyhow::{anyhow, Result};
use collections::{HashMap, HashSet, VecDeque};
use futures::{future::BoxFuture, Future};
use parking_lot::{Mutex, RwLock};
use slotmap::SlotMap;
use std::{
    any::{type_name, Any, TypeId},
    borrow::Borrow,
    mem,
    sync::{atomic::Ordering::SeqCst, Arc, Weak},
    time::Duration,
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
        assert!(
            executor.is_main_thread(),
            "must construct App on main thread"
        );

        let text_system = Arc::new(TextSystem::new(platform.text_system()));
        let mut entities = EntityMap::new();
        let unit_entity = entities.insert(entities.reserve(), ());
        let app_metadata = AppMetadata {
            os_name: platform.os_name(),
            os_version: platform.os_version().ok(),
            app_version: platform.app_version().ok(),
        };
        Self(Arc::new_cyclic(|this| {
            Mutex::new(AppContext {
                this: this.clone(),
                text_system,
                platform: MainThreadOnly::new(platform, executor.clone()),
                app_metadata,
                flushing_effects: false,
                pending_updates: 0,
                next_frame_callbacks: Default::default(),
                executor,
                svg_renderer: SvgRenderer::new(asset_source),
                image_cache: ImageCache::new(http_client),
                text_style_stack: Vec::new(),
                globals_by_type: HashMap::default(),
                unit_entity,
                entities,
                windows: SlotMap::with_key(),
                keymap: Arc::new(RwLock::new(Keymap::default())),
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
type Handler = Box<dyn Fn(&mut AppContext) -> bool + Send + Sync + 'static>;
type Listener = Box<dyn Fn(&dyn Any, &mut AppContext) -> bool + Send + Sync + 'static>;
type QuitHandler = Box<dyn Fn(&mut AppContext) -> BoxFuture<'static, ()> + Send + Sync + 'static>;
type ReleaseListener = Box<dyn Fn(&mut dyn Any, &mut AppContext) + Send + Sync + 'static>;

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
    pub(crate) image_cache: ImageCache,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) globals_by_type: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    pub(crate) unit_entity: Handle<()>,
    pub(crate) entities: EntityMap,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    pub(crate) keymap: Arc<RwLock<Keymap>>,
    pub(crate) global_action_listeners:
        HashMap<TypeId, Vec<Box<dyn Fn(&dyn Action, DispatchPhase, &mut Self) + Send + Sync>>>,
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
    }

    pub fn app_metadata(&self) -> AppMetadata {
        self.app_metadata.clone()
    }

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
                    Effect::FocusChanged { window_id, focused } => {
                        self.apply_focus_changed_effect(window_id, focused);
                    }
                    Effect::Refresh => {
                        self.apply_refresh_effect();
                    }
                    Effect::NotifyGlobalObservers { global_type } => {
                        self.apply_notify_global_observers_effect(global_type);
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
                self.event_listeners.remove(&entity_id);
                for release_callback in self.release_listeners.remove(&entity_id) {
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
        self.event_listeners
            .clone()
            .retain(&emitter, |handler| handler(&event, self));
    }

    fn apply_focus_changed_effect(&mut self, window_id: WindowId, focused: Option<FocusId>) {
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

    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext {
            app: unsafe { mem::transmute(self.this.clone()) },
            executor: self.executor.clone(),
        }
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

    pub fn has_global<G: 'static>(&self) -> bool {
        self.globals_by_type.contains_key(&TypeId::of::<G>())
    }

    pub fn global<G: 'static>(&self) -> &G {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    pub fn try_global<G: 'static>(&self) -> Option<&G> {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
    }

    pub fn global_mut<G: 'static>(&mut self) -> &mut G {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type
            .get_mut(&global_type)
            .and_then(|any_state| any_state.downcast_mut::<G>())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    pub fn default_global_mut<G: 'static + Default + Sync + Send>(&mut self) -> &mut G {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type
            .entry(global_type)
            .or_insert_with(|| Box::new(G::default()))
            .downcast_mut::<G>()
            .unwrap()
    }

    pub fn set_global<T: Send + Sync + 'static>(&mut self, global: T) {
        let global_type = TypeId::of::<T>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type.insert(global_type, Box::new(global));
    }

    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static + Send + Sync,
    {
        let mut global = self.lease_global::<G>();
        let result = f(global.as_mut(), self);
        self.restore_global(global);
        result
    }

    pub fn observe_global<G: 'static>(
        &mut self,
        f: impl Fn(&mut Self) + Send + Sync + 'static,
    ) -> Subscription {
        self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| {
                f(cx);
                true
            }),
        )
    }

    pub(crate) fn lease_global<G: 'static + Send + Sync>(&mut self) -> Box<G> {
        self.globals_by_type
            .remove(&TypeId::of::<G>())
            .ok_or_else(|| anyhow!("no global registered of type {}", type_name::<G>()))
            .unwrap()
            .downcast()
            .unwrap()
    }

    pub(crate) fn restore_global<G: 'static + Send + Sync>(&mut self, global: Box<G>) {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type.insert(global_type, global);
    }

    pub(crate) fn push_text_style(&mut self, text_style: TextStyleRefinement) {
        self.text_style_stack.push(text_style);
    }

    pub(crate) fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
    }

    pub fn bind_keys(&mut self, bindings: impl IntoIterator<Item = KeyBinding>) {
        self.keymap.write().add_bindings(bindings);
        self.pending_effects.push_back(Effect::Refresh);
    }

    pub fn on_action<A: Action>(
        &mut self,
        listener: impl Fn(&A, &mut Self) + Send + Sync + 'static,
    ) {
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

    pub fn stop_propagation(&mut self) {
        self.propagate_event = false;
    }
}

impl Context for AppContext {
    type EntityContext<'a, 'w, T: Send + Sync + 'static> = ModelContext<'a, T>;
    type Result<T> = T;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        self.update(|cx| {
            let slot = cx.entities.reserve();
            let entity = build_entity(&mut ModelContext::mutable(cx, slot.entity_id));
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
            let result = update(
                &mut entity,
                &mut ModelContext::mutable(cx, handle.entity_id),
            );
            cx.entities.end_lease(entity);
            result
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

    pub fn activate(&self, ignoring_other_apps: bool) {
        self.platform().activate(ignoring_other_apps);
    }

    pub fn write_to_clipboard(&self, item: ClipboardItem) {
        self.platform().write_to_clipboard(item)
    }

    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.platform().read_from_clipboard()
    }

    pub fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Result<()> {
        self.platform().write_credentials(url, username, password)
    }

    pub fn read_credentials(&self, url: &str) -> Result<Option<(String, Vec<u8>)>> {
        self.platform().read_credentials(url)
    }

    pub fn delete_credentials(&self, url: &str) -> Result<()> {
        self.platform().delete_credentials(url)
    }

    pub fn open_url(&self, url: &str) {
        self.platform().open_url(url);
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

    pub fn update_global<G: 'static + Send + Sync, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut MainThread<AppContext>) -> R,
    ) -> R {
        self.0.update_global(|global, cx| {
            let cx = unsafe { mem::transmute::<&mut AppContext, &mut MainThread<AppContext>>(cx) };
            update(global, cx)
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
    NotifyGlobalObservers {
        global_type: TypeId,
    },
}

pub(crate) struct AnyDrag {
    pub drag_handle_view: AnyView,
    pub cursor_offset: Point<Pixels>,
    pub state: AnyBox,
    pub state_type: TypeId,
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
