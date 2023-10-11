mod async_context;
mod entity_map;
mod model_context;

pub use async_context::*;
pub use entity_map::*;
pub use model_context::*;
use refineable::Refineable;

use crate::{
    current_platform, image_cache::ImageCache, AssetSource, Context, DisplayId, Executor, LayoutId,
    MainThread, MainThreadOnly, Platform, PlatformDisplayLinker, RootView, SubscriberSet,
    SvgRenderer, Task, TextStyle, TextStyleRefinement, TextSystem, Window, WindowContext,
    WindowHandle, WindowId,
};
use anyhow::{anyhow, Result};
use collections::{HashMap, VecDeque};
use futures::Future;
use parking_lot::Mutex;
use slotmap::SlotMap;
use std::{
    any::{type_name, Any, TypeId},
    mem,
    sync::{Arc, Weak},
};
use util::http::{self, HttpClient};

#[derive(Clone)]
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
                pending_updates: 0,
                display_linker: platform.display_linker(),
                next_frame_callbacks: Default::default(),
                platform: MainThreadOnly::new(platform, executor.clone()),
                executor,
                svg_renderer: SvgRenderer::new(asset_source),
                image_cache: ImageCache::new(http_client),
                text_style_stack: Vec::new(),
                state_stacks_by_type: HashMap::default(),
                unit_entity,
                entities,
                windows: SlotMap::with_key(),
                pending_effects: Default::default(),
                observers: SubscriberSet::new(),
                event_handlers: SubscriberSet::new(),
                layout_id_buffer: Default::default(),
            })
        }))
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut MainThread<AppContext>),
    {
        let this = self.clone();
        let platform = self.0.lock().platform.clone();
        platform.borrow_on_main_thread().run(Box::new(move || {
            let cx = &mut *this.0.lock();
            let cx = unsafe { mem::transmute::<&mut AppContext, &mut MainThread<AppContext>>(cx) };
            on_finish_launching(cx);
        }));
    }
}

type Handler = Arc<dyn Fn(&mut AppContext) -> bool + Send + Sync + 'static>;
type EventHandler = Arc<dyn Fn(&dyn Any, &mut AppContext) -> bool + Send + Sync + 'static>;
type FrameCallback = Box<dyn FnOnce(&mut WindowContext) + Send>;

pub struct AppContext {
    this: Weak<Mutex<AppContext>>,
    platform: MainThreadOnly<dyn Platform>,
    text_system: Arc<TextSystem>,
    pending_updates: usize,
    pub(crate) display_linker: Arc<dyn PlatformDisplayLinker>,
    pub(crate) next_frame_callbacks: HashMap<DisplayId, Vec<FrameCallback>>,
    pub(crate) executor: Executor,
    pub(crate) svg_renderer: SvgRenderer,
    pub(crate) image_cache: ImageCache,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) state_stacks_by_type: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
    pub(crate) unit_entity: Handle<()>,
    pub(crate) entities: EntityMap,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    pub(crate) pending_effects: VecDeque<Effect>,
    pub(crate) observers: SubscriberSet<EntityId, Handler>,
    pub(crate) event_handlers: SubscriberSet<EntityId, EventHandler>,
    pub(crate) layout_id_buffer: Vec<LayoutId>, // We recycle this memory across layout requests.
}

impl AppContext {
    pub(crate) fn update<R>(&mut self, update: impl FnOnce(&mut Self) -> R) -> R {
        self.pending_updates += 1;
        let result = update(self);
        if self.pending_updates == 1 {
            self.flush_effects();
        }
        self.pending_updates -= 1;
        result
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

    fn flush_effects(&mut self) {
        while let Some(effect) = self.pending_effects.pop_front() {
            match effect {
                Effect::Notify(entity_id) => self.apply_notify_effect(entity_id),
                Effect::Emit { entity_id, event } => self.apply_emit_effect(entity_id, event),
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
            .collect::<Vec<_>>();

        for dirty_window_id in dirty_window_ids {
            self.update_window(dirty_window_id, |cx| cx.draw()).unwrap();
        }
    }

    fn apply_notify_effect(&mut self, updated_entity: EntityId) {
        self.observers
            .clone()
            .retain(&updated_entity, |handler| handler(self));
    }

    fn apply_emit_effect(&mut self, updated_entity: EntityId, event: Box<dyn Any>) {
        self.event_handlers
            .clone()
            .retain(&updated_entity, |handler| handler(&event, self));
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
        F: Future<Output = R> + Send + 'static,
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

    pub fn state<S: 'static>(&self) -> &S {
        self.state_stacks_by_type
            .get(&TypeId::of::<S>())
            .and_then(|stack| stack.last())
            .and_then(|any_state| any_state.downcast_ref::<S>())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<S>()))
            .unwrap()
    }

    pub fn state_mut<S: 'static>(&mut self) -> &mut S {
        self.state_stacks_by_type
            .get_mut(&TypeId::of::<S>())
            .and_then(|stack| stack.last_mut())
            .and_then(|any_state| any_state.downcast_mut::<S>())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<S>()))
            .unwrap()
    }

    pub(crate) fn push_text_style(&mut self, text_style: TextStyleRefinement) {
        self.text_style_stack.push(text_style);
    }

    pub(crate) fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
    }

    pub(crate) fn push_state<T: Send + Sync + 'static>(&mut self, state: T) {
        self.state_stacks_by_type
            .entry(TypeId::of::<T>())
            .or_default()
            .push(Box::new(state));
    }

    pub(crate) fn pop_state<T: 'static>(&mut self) {
        self.state_stacks_by_type
            .get_mut(&TypeId::of::<T>())
            .and_then(|stack| stack.pop())
            .expect("state stack underflow");
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
        build_root_view: impl FnOnce(&mut WindowContext) -> RootView<S> + Send + 'static,
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
    Notify(EntityId),
    Emit {
        entity_id: EntityId,
        event: Box<dyn Any + Send + Sync + 'static>,
    },
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
