mod async_context;
mod entity_map;
mod model_context;

pub use async_context::*;
pub use entity_map::*;
pub use model_context::*;
use refineable::Refineable;

use crate::{
    current_platform, run_on_main, spawn_on_main, Context, LayoutId, MainThreadOnly, Platform,
    PlatformDispatcher, Reference, RootView, TextStyle, TextStyleRefinement, TextSystem, Window,
    WindowContext, WindowHandle, WindowId,
};
use anyhow::{anyhow, Result};
use collections::{HashMap, VecDeque};
use futures::{future, Future};
use parking_lot::Mutex;
use slotmap::SlotMap;
use smallvec::SmallVec;
use std::{
    any::{type_name, Any, TypeId},
    ops::{Deref, DerefMut},
    sync::{Arc, Weak},
};
use util::ResultExt;

#[derive(Clone)]
pub struct App(Arc<Mutex<AppContext>>);

impl App {
    pub fn production() -> Self {
        Self::new(current_platform())
    }

    #[cfg(any(test, feature = "test"))]
    pub fn test() -> Self {
        Self::new(Arc::new(super::TestPlatform::new()))
    }

    fn new(platform: Arc<dyn Platform>) -> Self {
        let dispatcher = platform.dispatcher();
        let text_system = Arc::new(TextSystem::new(platform.text_system()));
        let entities = EntityMap::new();
        let unit_entity = entities.redeem(entities.reserve(), ());
        Self(Arc::new_cyclic(|this| {
            Mutex::new(AppContext {
                this: this.clone(),
                platform: MainThreadOnly::new(platform, dispatcher),
                text_system,
                pending_updates: 0,
                text_style_stack: Vec::new(),
                state_stacks_by_type: HashMap::default(),
                unit_entity,
                entities,
                windows: SlotMap::with_key(),
                pending_effects: Default::default(),
                observers: Default::default(),
                layout_id_buffer: Default::default(),
            })
        }))
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut AppContext),
    {
        let this = self.clone();
        let platform = self.0.lock().platform.clone();
        platform.borrow_on_main_thread().run(Box::new(move || {
            let cx = &mut *this.0.lock();
            on_finish_launching(cx);
        }));
    }
}

type Handlers = SmallVec<[Arc<dyn Fn(&mut AppContext) -> bool + Send + Sync + 'static>; 2]>;

pub struct AppContext {
    this: Weak<Mutex<AppContext>>,
    platform: MainThreadOnly<dyn Platform>,
    dispatcher: Arc<dyn PlatformDispatcher>,
    text_system: Arc<TextSystem>,
    pending_updates: usize,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) state_stacks_by_type: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
    pub(crate) unit_entity: Handle<()>,
    pub(crate) entities: EntityMap,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    pub(crate) pending_effects: VecDeque<Effect>,
    pub(crate) observers: HashMap<EntityId, Handlers>,
    pub(crate) layout_id_buffer: Vec<LayoutId>, // We recycle this memory across layout requests.
}

impl AppContext {
    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    pub fn to_async(&self) -> AsyncContext {
        AsyncContext(self.this.clone())
    }

    pub fn run_on_main<R>(
        &self,
        f: impl FnOnce(&mut MainThreadContext) -> R + Send + 'static,
    ) -> impl Future<Output = R>
    where
        R: Send + 'static,
    {
        let this = self.this.upgrade().unwrap();
        run_on_main(self.dispatcher.clone(), move || {
            let cx = &mut *this.lock();
            let platform = cx.platform.borrow_on_main_thread().clone();
            cx.update(|cx| f(&mut MainThreadContext::mutable(cx, platform.as_ref())))
        })
    }

    pub fn spawn_on_main<F, R>(
        &self,
        f: impl FnOnce(&mut MainThreadContext) -> F + Send + 'static,
    ) -> impl Future<Output = R>
    where
        F: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let this = self.this.upgrade().unwrap();
        spawn_on_main(self.dispatcher.clone(), move || {
            let cx = &mut *this.lock();
            let platform = cx.platform.borrow_on_main_thread().clone();
            cx.update(|cx| f(&mut MainThreadContext::mutable(cx, platform.as_ref())))
        })
        // self.platform.read(move |platform| {
    }

    pub fn open_window<S: 'static + Send + Sync>(
        &mut self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut WindowContext) -> RootView<S> + Send + 'static,
    ) -> impl Future<Output = WindowHandle<S>> {
        let id = self.windows.insert(None);
        let handle = WindowHandle::new(id);
        self.spawn_on_main(move |cx| future::ready(cx.open_window(options, build_root_view)))
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
            window.dirty = true;

            cx.windows
                .get_mut(id)
                .ok_or_else(|| anyhow!("window not found"))?
                .replace(window);

            Ok(result)
        })
    }

    fn update<R>(&mut self, update: impl FnOnce(&mut Self) -> R) -> R {
        self.pending_updates += 1;
        let result = update(self);
        self.pending_updates -= 1;
        if self.pending_updates == 0 {
            self.flush_effects();
        }
        result
    }

    fn flush_effects(&mut self) {
        while let Some(effect) = self.pending_effects.pop_front() {
            match effect {
                Effect::Notify(entity_id) => self.apply_notify_effect(entity_id),
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
            self.update_window(dirty_window_id, |cx| cx.draw())
                .unwrap() // We know we have the window.
                .log_err();
        }
    }

    fn apply_notify_effect(&mut self, updated_entity: EntityId) {
        if let Some(mut handlers) = self.observers.remove(&updated_entity) {
            handlers.retain(|handler| handler(self));
            if let Some(new_handlers) = self.observers.remove(&updated_entity) {
                handlers.extend(new_handlers);
            }
            self.observers.insert(updated_entity, handlers);
        }
    }
}

impl Context for AppContext {
    type EntityContext<'a, 'w, T: Send + Sync + 'static> = ModelContext<'a, T>;
    type Result<T> = T;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        let slot = self.entities.reserve();
        let entity = build_entity(&mut ModelContext::mutable(self, slot.id));
        self.entities.redeem(slot, entity)
    }

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self.entities.lease(handle);
        let result = update(&mut *entity, &mut ModelContext::mutable(self, handle.id));
        self.entities.end_lease(entity);
        result
    }
}

pub struct MainThreadContext<'a> {
    app: Reference<'a, AppContext>,
    platform: &'a dyn Platform,
}

impl<'a> MainThreadContext<'a> {
    fn mutable(cx: &'a mut AppContext, platform: &'a dyn Platform) -> Self {
        Self {
            app: Reference::Mutable(cx),
            platform,
        }
    }

    pub fn activate(&mut self, ignoring_other_apps: bool) {
        self.platform.activate(ignoring_other_apps);
    }

    pub fn open_window<S: 'static + Send + Sync>(
        &mut self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut WindowContext) -> RootView<S> + Send + 'static,
    ) -> WindowHandle<S> {
        let id = self.windows.insert(None);
        let handle = WindowHandle::new(id);
        let cx = &mut *self.app;
        let mut window = Window::new(handle.into(), options, self.platform, cx);
        let root_view = build_root_view(&mut WindowContext::mutable(cx, &mut window));
        window.root_view.replace(root_view.into_any());
        cx.windows.get_mut(id).unwrap().replace(window);
        handle
    }
}

impl<'a> Deref for MainThreadContext<'a> {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        &*self.app
    }
}

impl<'a> DerefMut for MainThreadContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.app
    }
}

pub(crate) enum Effect {
    Notify(EntityId),
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
