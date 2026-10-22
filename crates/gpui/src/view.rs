use crate::{
    AnyElement, AnyEntity, AnyWeakEntity, App, AvailableSpace, Bounds, ContentMask, Context,
    Element, ElementId, Entity, EntityId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, PaintIndex, Pixels, PrepaintStateIndex, Render, RenderOnce, Size, Style,
    StyleRefinement, TextStyle, WeakEntity,
};
use crate::{Empty, Window};
use anyhow::Result;
use collections::FxHashSet;
use refineable::Refineable;
use std::mem;
use std::{
    any::{TypeId, type_name},
    fmt,
    ops::Range,
    rc::Rc,
};

pub(crate) mod retained;
pub(crate) use retained::{ViewOccurrenceId, ViewStore};

/// A dynamically-typed view handle that can be downcast to a specific `Entity<V>`.
///
/// This is the type-erased counterpart to [`ViewElement`]: it holds an entity plus
/// a function pointer to its render, and is itself a [`View`], so embedding it as an
/// element goes through the same [`ViewElement`] machinery as any other view.
#[derive(Clone, Debug)]
pub struct AnyView {
    entity: AnyEntity,
    render: fn(&AnyView, &mut Window, &mut App) -> AnyElement,
    view_type_name: &'static str,
}

impl<V: Render> From<Entity<V>> for AnyView {
    fn from(value: Entity<V>) -> Self {
        AnyView {
            entity: value.into_any(),
            render: any_view::render::<V>,
            view_type_name: type_name::<V>(),
        }
    }
}

impl AnyView {
    /// Embed this view as a cached [`ViewElement`] laid out at `style`.
    ///
    /// The rendered subtree is recycled from the previous frame unless
    /// [Context::notify] was called on the backing entity since it was rendered
    /// (or [Window::refresh] is called, which ignores caching).
    pub fn cached(self, style: StyleRefinement) -> ViewElement<AnyView> {
        self.into_element().cached(style)
    }

    /// Convert this to a weak handle.
    pub fn downgrade(&self) -> AnyWeakView {
        AnyWeakView {
            entity: self.entity.downgrade(),
            render: self.render,
            view_type_name: self.view_type_name,
        }
    }

    /// Convert this to a [Entity] of a specific type.
    /// If this handle does not contain a view of the specified type, returns itself in an `Err` variant.
    pub fn downcast<T: 'static>(self) -> Result<Entity<T>, Self> {
        match self.entity.downcast() {
            Ok(entity) => Ok(entity),
            Err(entity) => Err(Self {
                entity,
                render: self.render,
                view_type_name: self.view_type_name,
            }),
        }
    }

    /// Gets the [TypeId] of the underlying view.
    pub fn entity_type(&self) -> TypeId {
        self.entity.entity_type
    }

    /// The [`EntityId`] of this view.
    pub fn entity_id(&self) -> EntityId {
        self.entity.entity_id()
    }
}

impl PartialEq for AnyView {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl Eq for AnyView {}

/// `AnyView` is the type-erased [`View`]: its `render` is a function pointer rather
/// than a concrete type, but it participates in the reactive graph exactly like any
/// other view via [`ViewElement`].
impl View for AnyView {
    fn entity_id(&self) -> Option<EntityId> {
        Some(self.entity.entity_id())
    }

    #[cfg(not(gpui_hot_reload))]
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        (self.render)(&self, window, cx)
    }

    // The stored function pointer was captured from the original binary, so it must be looked up
    // in the jump table to reach the patched render.
    #[cfg(gpui_hot_reload)]
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        subsecond::HotFn::current(self.render).call((&self, window, cx))
    }

    fn retained_view(&self) -> Option<AnyWeakView> {
        Some(self.downgrade())
    }
}

impl<V: 'static + Render> IntoElement for Entity<V> {
    type Element = ViewElement<Entity<V>>;

    fn into_element(self) -> Self::Element {
        ViewElement::with_type_name(self, type_name::<V>())
    }

    #[inline(never)]
    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }
}

impl IntoElement for AnyView {
    type Element = ViewElement<AnyView>;

    fn into_element(self) -> Self::Element {
        let view_type_name = self.view_type_name;
        ViewElement::with_type_name(self, view_type_name)
    }
}

/// A weak, dynamically-typed view handle.
#[derive(Clone)]
pub struct AnyWeakView {
    entity: AnyWeakEntity,
    render: fn(&AnyView, &mut Window, &mut App) -> AnyElement,
    view_type_name: &'static str,
}

impl AnyWeakView {
    /// Upgrade to a strong `AnyView` handle, if the view is still alive.
    pub fn upgrade(&self) -> Option<AnyView> {
        let entity = self.entity.upgrade()?;
        Some(AnyView {
            entity,
            render: self.render,
            view_type_name: self.view_type_name,
        })
    }
}

impl<V: 'static + Render> From<WeakEntity<V>> for AnyWeakView {
    fn from(view: WeakEntity<V>) -> Self {
        AnyWeakView {
            entity: view.into(),
            render: any_view::render::<V>,
            view_type_name: type_name::<V>(),
        }
    }
}

impl PartialEq for AnyWeakView {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl std::fmt::Debug for AnyWeakView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyWeakView")
            .field("entity_id", &self.entity.entity_id)
            .finish_non_exhaustive()
    }
}

mod any_view {
    use crate::{AnyElement, AnyView, App, IntoElement, Render, Window};

    pub(crate) fn render<V: 'static + Render>(
        view: &AnyView,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let view = view.clone().downcast::<V>().unwrap();
        // Record the view's Render type name so the accessibility debug dump can
        // attribute nodes to the view that produced them.
        #[cfg(debug_assertions)]
        window
            .a11y
            .view_type_names
            .insert(view.entity_id(), std::any::type_name::<V>());
        view.update(cx, |view, cx| view.render(window, cx).into_any_element())
    }
}

/// A renderable that participates in GPUI's reactive graph — the unifying model
/// behind [`Render`] and [`RenderOnce`].
///
/// When `entity_id()` returns `Some`, that id becomes the view's identity: it gets
/// a unique element-id space (so internal `use_state` / `.id(..)` never collide
/// across siblings) and `cx.notify()` on that entity re-renders only this view's
/// subtree. `None` behaves like a stateless component.
///
/// You rarely implement `View` directly. `Entity<T: Render>` and any `T: RenderOnce`
/// get a blanket impl below; implement it by hand only when a component needs both
/// parent-supplied props *and* a backing entity for identity.
pub trait View: 'static + Sized {
    /// This view's identity, if it has one. A view typically holds the backing
    /// entity as a field and returns its [`EntityId`] here.
    ///
    /// The id becomes this view's [`ElementId`], so two views keyed on the same
    /// entity must not be rendered at the same position in the element tree
    /// (e.g. as siblings under the same parent): their internal element state
    /// (`use_state`, scroll offsets, etc.) would silently collide. Nesting is
    /// fine — the id is scoped by the parent path.
    fn entity_id(&self) -> Option<EntityId>;

    /// Render this view into an element tree, consuming `self`.
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement;

    #[doc(hidden)]
    fn retained_view(&self) -> Option<AnyWeakView> {
        None
    }
}

/// A stateless component (`RenderOnce`) is a `View` with no identity.
impl<T: RenderOnce> View for T {
    fn entity_id(&self) -> Option<EntityId> {
        None
    }

    #[inline]
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        RenderOnce::render(self, window, cx)
    }
}

/// An entity that renders itself (`Render`) is a `View` keyed on its own id.
impl<T: Render> View for Entity<T> {
    fn entity_id(&self) -> Option<EntityId> {
        Some(Entity::entity_id(self))
    }

    #[inline]
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.update(cx, |this, cx| {
            Render::render(this, window, cx).into_any_element()
        })
    }

    fn retained_view(&self) -> Option<AnyWeakView> {
        Some(self.downgrade().into())
    }
}

impl<T: Render> Entity<T> {
    /// Embed this entity as a cached [`ViewElement`] laid out at `style`.
    ///
    /// The rendered subtree is reused until the entity is notified (or the
    /// cached bounds / text style change). Caching requires a definite size:
    /// a cached view is laid out from `style` and is *not* measured from its
    /// contents. Use [`ViewElement::new`] (or `.child(entity)`) for the
    /// uncached case.
    ///
    /// With experimental view splicing enabled, the composing view must notify when
    /// this external style changes, or derive it from entities it reads through `cx`.
    /// Reads through untracked shared state such as `Rc<RefCell<_>>` are not dependencies.
    #[track_caller]
    pub fn cached(self, style: StyleRefinement) -> ViewElement<Entity<T>> {
        ViewElement::with_type_name(self, type_name::<T>()).cached(style)
    }
}

/// The element type for [`View`] implementations. Wraps a `View` and hooks it
/// into layout, prepaint, and paint. Constructed via [`ViewElement::new`].
#[doc(hidden)]
pub struct ViewElement<V: View> {
    view: Option<V>,
    entity_id: Option<EntityId>,
    cached_style: Option<StyleRefinement>,
    #[cfg(feature = "profiler")]
    view_type_name: &'static str,
    implicit: bool,
    retained_view: Option<AnyWeakView>,
    retained_occurrence: Option<ViewOccurrenceId>,
    #[cfg(debug_assertions)]
    source: &'static core::panic::Location<'static>,
}

impl<V: View> ViewElement<V> {
    /// Wrap a [`View`] as an element.
    #[track_caller]
    pub fn new(view: V) -> Self {
        Self::with_type_name(view, type_name::<V>())
    }

    /// Wraps a view whose draw time profiling attributes to `view_type_name`,
    /// such as an entity's type rather than [`Entity`]'s.
    #[track_caller]
    #[cfg_attr(not(feature = "profiler"), expect(unused_variables))]
    pub(crate) fn with_type_name(view: V, view_type_name: &'static str) -> Self {
        let entity_id = view.entity_id();
        ViewElement {
            entity_id,
            cached_style: None,
            #[cfg(feature = "profiler")]
            view_type_name,
            implicit: false,
            retained_view: None,
            retained_occurrence: None,
            view: Some(view),
            #[cfg(debug_assertions)]
            source: core::panic::Location::caller(),
        }
    }

    /// Enable caching of this view's rendered subtree, laid out at `style`.
    /// The composer supplies the layout style because caching skips rendering
    /// the contents to measure them.
    ///
    /// Crate-private on purpose: caching is only sound for entity-backed views,
    /// where [`Context::notify`] is the contract that busts the cache. A stateless
    /// view has no such contract, so a frozen subtree could never be invalidated.
    /// Reach this through [`Entity::cached`] or [`AnyView::cached`], which are
    /// entity-backed by construction.
    fn cached(mut self, style: StyleRefinement) -> Self {
        self.cached_style = Some(style);
        self
    }
}

impl<V: View> IntoElement for ViewElement<V> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct ViewElementState {
    prepaint_range: Range<PrepaintStateIndex>,
    paint_range: Range<PaintIndex>,
    cache_key: ViewElementCacheKey,
    accessed_entities: FxHashSet<EntityId>,
}

struct ViewElementHandle(ViewOccurrenceId);

struct ViewElementCacheKey {
    bounds: Bounds<Pixels>,
    content_mask: ContentMask<Pixels>,
    text_style: TextStyle,
}

#[cfg(not(gpui_hot_reload))]
#[inline]
fn render_view<V: View>(view: V, window: &mut Window, cx: &mut App) -> AnyElement {
    view.render(window, cx).into_any_element()
}

/// Every view and component render passes through this hot-patchable boundary, so the
/// patched code for that render and everything it calls runs on the next frame.
#[cfg(gpui_hot_reload)]
fn render_view<V: View>(view: V, window: &mut Window, cx: &mut App) -> AnyElement {
    #[inline(never)]
    fn render_view_impl<V: View>(view: V, window: &mut Window, cx: &mut App) -> AnyElement {
        view.render(window, cx).into_any_element()
    }

    subsecond::HotFn::current(render_view_impl::<V>).call((view, window, cx))
}

struct RetainedViewState {
    prepaint_range: Range<PrepaintStateIndex>,
    paint_range: Range<PaintIndex>,
    cache_key: RetainedViewCacheKey,
}

#[derive(Clone)]
struct RetainedViewCacheKey {
    bounds: Bounds<Pixels>,
    content_mask: ContentMask<Pixels>,
    text_style: Rc<TextStyle>,
}

impl<V: View> Element for ViewElement<V> {
    type RequestLayoutState = Option<AnyElement>;
    type PrepaintState = Option<AnyElement>;

    fn id(&self) -> Option<ElementId> {
        self.entity_id.map(ElementId::View)
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        #[cfg(debug_assertions)]
        return Some(self.source);

        #[cfg(not(debug_assertions))]
        return None;
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        if let Some(entity_id) = self.entity_id {
            self.implicit = window.implicit_view_splicing
                && self.cached_style.is_none()
                && window
                    .layout_engine
                    .as_ref()
                    .is_some_and(|engine| engine.cache_enabled())
                && !window.is_inspector_picking(cx);
            if self.implicit {
                window
                    .layout_engine
                    .as_mut()
                    .expect("layout engine")
                    .enable_retention();
                self.retained_view = self.view.as_ref().and_then(View::retained_view);
                if let Some(retained_view) = self.retained_view.as_ref() {
                    return window.request_implicit_view_layout(
                        entity_id,
                        id.expect("entity view id"),
                        retained_view,
                        true,
                        cx,
                        &mut |window, cx| {
                            self.view
                                .take()
                                .expect("unrendered view")
                                .render(window, cx)
                                .into_any_element()
                        },
                    );
                }
                self.implicit = false;
            }
            // Stateful path: create a reactive boundary.
            #[cfg(feature = "profiler")]
            let start = window.window_profiler.begin_view();
            let view = &mut self.view;
            let layout = request_layout_view(
                entity_id,
                self.cached_style.as_ref(),
                window,
                cx,
                &mut |window, cx| render_view(view.take().unwrap(), window, cx),
            );
            #[cfg(feature = "profiler")]
            window.window_profiler.end_view(
                start,
                self.view_type_name,
                crate::profiler::ViewPhase::RequestLayout,
            );
            layout
        } else {
            // Stateless path: isolate subtree via type name (no entity identity).
            request_layout_component(type_name::<V>(), window, cx, &mut |window, cx| {
                render_view(self.view.take().unwrap(), window, cx)
            })
        }
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        if let Some(entity_id) = self.entity_id {
            // Stateful path.
            #[cfg(feature = "profiler")]
            let start = window.window_profiler.begin_view();
            let retained_view =
                if window.view_splicing && (self.cached_style.is_some() || self.implicit) {
                    self.retained_view
                        .clone()
                        .or_else(|| self.view.as_ref().and_then(View::retained_view))
                } else {
                    None
                };
            let prepainted = prepaint_view(
                entity_id,
                global_id,
                bounds,
                element,
                retained_view.as_ref(),
                &mut self.retained_occurrence,
                window,
                cx,
                &mut |window, cx| render_view(self.view.take().unwrap(), window, cx),
            );
            #[cfg(feature = "profiler")]
            window.window_profiler.end_view(
                start,
                self.view_type_name,
                crate::profiler::ViewPhase::Prepaint,
            );
            prepainted
        } else {
            // Stateless path: just prepaint the element.
            prepaint_component(type_name::<V>(), element, window, cx)
        }
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        element: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(entity_id) = self.entity_id {
            // Stateful path.
            #[cfg(feature = "profiler")]
            let start = window.window_profiler.begin_view();
            paint_view(
                entity_id,
                self.cached_style.is_some() || self.implicit,
                global_id,
                element,
                self.retained_occurrence,
                window,
                cx,
            );
            #[cfg(feature = "profiler")]
            window.window_profiler.end_view(
                start,
                self.view_type_name,
                crate::profiler::ViewPhase::Paint,
            );
        } else {
            // Stateless path: just paint the element.
            paint_component(std::any::type_name::<V>(), element, window, cx);
        }
    }
}

/// A view that renders nothing
pub struct EmptyView;

impl Render for EmptyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[inline(never)]
fn request_layout_view(
    entity_id: EntityId,
    cached_style: Option<&StyleRefinement>,
    window: &mut Window,
    cx: &mut App,
    render: &mut dyn FnMut(&mut Window, &mut App) -> AnyElement,
) -> (LayoutId, Option<AnyElement>) {
    window.with_rendered_view(entity_id, |window| {
        let caching_disabled = window.is_inspector_picking(cx);
        match cached_style {
            Some(style) if !caching_disabled => {
                let mut root_style = Style::default();
                root_style.refine(style);
                let layout_id = window.request_layout(root_style, None, cx);
                (layout_id, None)
            }
            _ => {
                #[cfg(any(test, feature = "bench-support"))]
                {
                    window.record_view_render(
                        entity_id,
                        caching_disabled,
                        cached_style.is_some(),
                        false,
                    );
                    if cached_style.is_some() {
                        window.view_reuse_stats.cached_misses += 1;
                        window.view_reuse_stats.miss_inspector += 1;
                    }
                }
                let mut element = render(window, cx);
                let layout_id = element.request_layout(window, cx);
                (layout_id, Some(element))
            }
        }
    })
}

#[inline(never)]
fn request_layout_component(
    name: &'static str,
    window: &mut Window,
    cx: &mut App,
    render: &mut dyn FnMut(&mut Window, &mut App) -> AnyElement,
) -> (LayoutId, Option<AnyElement>) {
    window.with_id(ElementId::from(name), |window| {
        let mut element = render(window, cx);
        let layout_id = element.request_layout(window, cx);
        (layout_id, Some(element))
    })
}

#[inline(never)]
fn prepaint_view(
    entity_id: EntityId,
    global_id: Option<&GlobalElementId>,
    bounds: Bounds<Pixels>,
    element: &mut Option<AnyElement>,
    retained_view: Option<&AnyWeakView>,
    retained_occurrence: &mut Option<ViewOccurrenceId>,
    window: &mut Window,
    cx: &mut App,
    render: &mut dyn FnMut(&mut Window, &mut App) -> AnyElement,
) -> Option<AnyElement> {
    if !window.view_splicing {
        return prepaint_legacy_view(entity_id, global_id, bounds, element, window, cx, render);
    }
    window.set_view_id(entity_id);
    let occurrence = window.begin_view_occurrence(entity_id, global_id, retained_view);
    *retained_occurrence = occurrence;
    let parent_occurrence = mem::replace(&mut window.current_view_occurrence, occurrence);
    let mut scope = retained_view.is_some().then(|| {
        let mut scope = window.capture_view_scope();
        scope.parent_occurrence = parent_occurrence;
        scope
    });
    let result = window.with_rendered_view(entity_id, |window| {
        if retained_view.is_none()
            && let Some(mut element) = element.take()
        {
            if let Some(occurrence) = occurrence {
                window.view_store.invalidate_state(occurrence);
            }
            element.prepaint(window, cx);
            return Some(element);
        }

        let occurrence = occurrence.expect("cached views have an occurrence");
        let implicit = window.is_implicit_view(occurrence);
        let mut prepaint = |window: &mut Window| {
            let constraints_unchanged = window.implicit_prepaint_layout(occurrence);
            let content_mask = window.content_mask();
            let text_style = window.retained_text_style_handle();

            let element_state = window.view_store.state(occurrence);
            #[cfg(any(test, feature = "bench-support"))]
            let force_reconstruction = window.force_view_reconstruction;
            #[cfg(not(any(test, feature = "bench-support")))]
            let force_reconstruction = false;

            #[cfg(any(test, feature = "bench-support"))]
            let cache_changes = element_state.map(|state| {
                (
                    state.cache_key.bounds != bounds,
                    state.cache_key.bounds.size != bounds.size,
                    state.cache_key.content_mask != content_mask,
                    state.cache_key.text_style != text_style,
                )
            });
            if element.is_none()
                && let Some(element_state) = element_state
                && (!window.is_implicit_view(occurrence) || window.view_store.has_paint(occurrence))
                && element_state.cache_key.bounds == bounds
                && element_state.cache_key.content_mask == content_mask
                && element_state.cache_key.text_style == text_style
                && !window.view_store.is_own_dirty(occurrence)
                && (!window.dirty_views.contains(&entity_id)
                    || (window.view_splicing && window.view_store.is_descendant_dirty(occurrence)))
                && window.retained_scope_fallback != Some(retained::SpliceFallback::Transaction)
                && !window.refreshing
                && !force_reconstruction
                && constraints_unchanged
            {
                let prepaint_range = element_state.prepaint_range.clone();
                let can_reuse = !window.view_store.is_descendant_dirty(occurrence)
                    || window.prepare_view_splice(occurrence).is_ok();
                if can_reuse {
                    #[cfg(any(test, feature = "bench-support"))]
                    {
                        window.view_reuse_stats.cached_hits += 1;
                        window.view_reuse_stats.reused_view_boundary_visits[0] += 1;
                        window.view_reuse_stats.reused_view_boundary_visits[1] += 1;
                    }
                    let prepaint_start = window.prepaint_index();
                    window.register_retained_access(occurrence, cx);
                    if window.view_store.is_descendant_dirty(occurrence) {
                        window.splice_view_prepaint(occurrence, cx);
                    } else {
                        window.reuse_retained_prepaint(prepaint_range, cx);
                    }
                    let prepaint_end = window.prepaint_index();
                    window.view_store.reused_view_prepaint(
                        occurrence,
                        prepaint_start..prepaint_end,
                        scope.take().expect("cached views have a scope"),
                    );
                    return (None, ViewElementHandle(occurrence));
                }
            }

            let implicit = window.is_implicit_view(occurrence);
            #[cfg(any(test, feature = "bench-support"))]
            let synthetic_refresh = !implicit && !window.refreshing;
            #[cfg(any(test, feature = "bench-support"))]
            {
                let stats = &mut window.view_reuse_stats;
                stats.cached_misses += 1;
                if let Some((bounds_changed, size_changed, mask_changed, text_changed)) =
                    cache_changes
                {
                    stats.miss_bounds += usize::from(bounds_changed);
                    stats.miss_content_mask += usize::from(mask_changed);
                    stats.miss_text_style += usize::from(text_changed);
                    stats.miss_dirty += usize::from(
                        window.dirty_views.contains(&entity_id)
                            || window.view_store.is_own_dirty(occurrence),
                    );
                    stats.miss_refreshing += usize::from(window.refreshing);
                    stats.miss_forced_reconstruction += usize::from(force_reconstruction);
                    stats.bounds_size_changed += usize::from(size_changed);
                    stats.bounds_origin_only_changed +=
                        usize::from(bounds_changed && !size_changed);
                } else {
                    stats.miss_first_mount += 1;
                }
                if element.is_none() && !window.is_implicit_view(occurrence) {
                    window.record_view_render(entity_id, false, true, cache_changes.is_none());
                }
                if synthetic_refresh {
                    window.cached_reconstruction_depth += 1;
                }
            }
            // Ordinary entity ancestors do not invalidate explicit cached children
            // on reconstruction. Only an explicit cached miss implies a refresh.
            let refreshing = window.refreshing;
            window.refreshing |= !implicit;
            let previous_access = window.begin_view_entity_access(cx);
            window
                .view_store
                .begin_reconstruction(occurrence, scope.as_ref().expect("cached view scope"));
            let prepaint_start = window.prepaint_index();
            let mut element = if let Some(element) = element.take() {
                element
            } else if implicit {
                window.reconstruct_implicit_view(occurrence, cx, render)
            } else {
                let mut element = render(window, cx);
                element.layout_as_root(Size::<AvailableSpace>::from(bounds.size), window, cx);
                element
            };
            if implicit {
                element.prepaint(window, cx);
            } else {
                element.prepaint_at(bounds.origin, window, cx);
            }

            let prepaint_end = window.prepaint_index();
            let (own, _) = window.end_cached_entity_access(previous_access, cx);
            window.refreshing = refreshing;
            #[cfg(any(test, feature = "bench-support"))]
            {
                if synthetic_refresh {
                    window.cached_reconstruction_depth -= 1;
                }
            }

            window.view_store.set_state(
                occurrence,
                RetainedViewState {
                    prepaint_range: prepaint_start..prepaint_end,
                    paint_range: PaintIndex::default()..PaintIndex::default(),
                    cache_key: RetainedViewCacheKey {
                        bounds,
                        content_mask,
                        text_style,
                    },
                },
                scope.take().expect("cached views have a scope"),
            );
            window.view_store.set_own_access(occurrence, own, false);
            (Some(element), ViewElementHandle(occurrence))
        };
        if implicit {
            prepaint(window).0
        } else {
            window.with_element_state::<ViewElementHandle, _>(
                global_id.unwrap(),
                |_handle, window| prepaint(window),
            )
        }
    });
    window.current_view_occurrence = parent_occurrence;
    result
}

#[inline(never)]
fn prepaint_component(
    name: &'static str,
    element: &mut Option<AnyElement>,
    window: &mut Window,
    cx: &mut App,
) -> Option<AnyElement> {
    window.with_id(ElementId::from(name), |window| {
        element.as_mut().unwrap().prepaint(window, cx);
    });
    Some(element.take().unwrap())
}

#[inline(never)]
fn paint_view(
    entity_id: EntityId,
    cached: bool,
    global_id: Option<&GlobalElementId>,
    element: &mut Option<AnyElement>,
    retained_occurrence: Option<ViewOccurrenceId>,
    window: &mut Window,
    cx: &mut App,
) {
    if !window.view_splicing {
        paint_legacy_view(entity_id, cached, global_id, element, window, cx);
        return;
    }
    let caching_disabled = window.is_inspector_picking(cx);
    let mut scope = (cached && !caching_disabled).then(|| window.capture_view_scope());
    window.with_rendered_view(entity_id, |window| {
        if cached && !caching_disabled {
            let mut paint = |handle: ViewElementHandle, window: &mut Window| {
                let occurrence = handle.0;

                window.next_frame.paint_views.push(occurrence);
                let paint_start = window.paint_index();

                if let Some(element) = element {
                    let parent_occurrence = window.current_view_occurrence.replace(occurrence);
                    let previous_access = window.begin_view_entity_access(cx);
                    let refreshing = window.refreshing;
                    window.refreshing |= !window.is_implicit_view(occurrence);
                    element.paint(window, cx);
                    window.refreshing = refreshing;
                    let (own, _) = window.end_cached_entity_access(previous_access, cx);
                    window.view_store.set_own_access(occurrence, own, true);
                    window.current_view_occurrence = parent_occurrence;
                } else {
                    #[cfg(any(test, feature = "bench-support"))]
                    {
                        window.view_reuse_stats.reused_view_boundary_visits[2] += 1;
                    }
                    window.splice_view_paint(occurrence, cx);
                }

                let paint_end = window.paint_index();
                window.view_store.painted_view(
                    occurrence,
                    paint_start..paint_end,
                    scope.take().expect("cached views have a paint scope"),
                );

                ((), handle)
            };
            if let Some(occurrence) = retained_occurrence.filter(|id| window.is_implicit_view(*id))
            {
                paint(ViewElementHandle(occurrence), window);
            } else {
                window.with_element_state::<ViewElementHandle, _>(
                    global_id.unwrap(),
                    |element_state, window| paint(element_state.expect("view handle"), window),
                )
            }
        } else {
            element.as_mut().unwrap().paint(window, cx);
        }
    });
}

#[inline(never)]
fn prepaint_legacy_view(
    entity_id: EntityId,
    global_id: Option<&GlobalElementId>,
    bounds: Bounds<Pixels>,
    element: &mut Option<AnyElement>,
    window: &mut Window,
    cx: &mut App,
    render: &mut dyn FnMut(&mut Window, &mut App) -> AnyElement,
) -> Option<AnyElement> {
    window.set_view_id(entity_id);
    window.with_rendered_view(entity_id, |window| {
        if let Some(mut element) = element.take() {
            element.prepaint(window, cx);
            return Some(element);
        }
        window.with_element_state::<ViewElementState, _>(global_id.unwrap(), |state, window| {
            let content_mask = window.content_mask();
            let text_style = window.text_style();
            #[cfg(any(test, feature = "bench-support"))]
            let first_mount = state.is_none();
            #[cfg(any(test, feature = "bench-support"))]
            let forced = window.force_view_reconstruction;
            #[cfg(not(any(test, feature = "bench-support")))]
            let forced = false;
            #[cfg(any(test, feature = "bench-support"))]
            {
                let stats = &mut window.view_reuse_stats;
                if let Some(state) = state.as_ref() {
                    let bounds_changed = state.cache_key.bounds != bounds;
                    let mask_changed = state.cache_key.content_mask != content_mask;
                    let text_changed = state.cache_key.text_style != text_style;
                    let dirty = window.dirty_views.contains(&entity_id);
                    if bounds_changed
                        || mask_changed
                        || text_changed
                        || dirty
                        || window.refreshing
                        || forced
                    {
                        stats.cached_misses += 1;
                        stats.miss_bounds += usize::from(bounds_changed);
                        stats.miss_content_mask += usize::from(mask_changed);
                        stats.miss_text_style += usize::from(text_changed);
                        stats.miss_dirty += usize::from(dirty);
                        stats.miss_refreshing += usize::from(window.refreshing);
                        stats.miss_forced_reconstruction += usize::from(forced);
                    } else {
                        stats.cached_hits += 1;
                        stats.reused_view_boundary_visits[0] += 1;
                        stats.reused_view_boundary_visits[1] += 1;
                    }
                    if state.cache_key.bounds.size != bounds.size {
                        stats.bounds_size_changed += 1;
                    } else if bounds_changed {
                        stats.bounds_origin_only_changed += 1;
                    }
                } else {
                    stats.cached_misses += 1;
                    stats.miss_first_mount += 1;
                }
            }
            if let Some(mut state) = state
                && state.cache_key.bounds == bounds
                && state.cache_key.content_mask == content_mask
                && state.cache_key.text_style == text_style
                && !window.dirty_views.contains(&entity_id)
                && !window.refreshing
                && !forced
            {
                let start = window.prepaint_index();
                window.reuse_prepaint(state.prepaint_range.clone());
                cx.entities.extend_accessed(&state.accessed_entities);
                state.prepaint_range = start..window.prepaint_index();
                return (None, state);
            }
            #[cfg(any(test, feature = "bench-support"))]
            let synthetic_refresh = !window.refreshing;
            #[cfg(any(test, feature = "bench-support"))]
            {
                window.record_view_render(entity_id, false, true, first_mount);
                if synthetic_refresh {
                    window.cached_reconstruction_depth += 1;
                }
            }
            let refreshing = mem::replace(&mut window.refreshing, true);
            let start = window.prepaint_index();
            let (element, accessed_entities) = cx.detect_accessed_entities(|cx| {
                let mut element = render(window, cx);
                element.layout_as_root(bounds.size.into(), window, cx);
                element.prepaint_at(bounds.origin, window, cx);
                element
            });
            let end = window.prepaint_index();
            window.refreshing = refreshing;
            #[cfg(any(test, feature = "bench-support"))]
            {
                if synthetic_refresh {
                    window.cached_reconstruction_depth -= 1;
                }
            }
            (
                Some(element),
                ViewElementState {
                    prepaint_range: start..end,
                    paint_range: PaintIndex::default()..PaintIndex::default(),
                    cache_key: ViewElementCacheKey {
                        bounds,
                        content_mask,
                        text_style,
                    },
                    accessed_entities,
                },
            )
        })
    })
}

#[inline(never)]
fn paint_legacy_view(
    entity_id: EntityId,
    cached: bool,
    global_id: Option<&GlobalElementId>,
    element: &mut Option<AnyElement>,
    window: &mut Window,
    cx: &mut App,
) {
    window.with_rendered_view(entity_id, |window| {
        if cached && !window.is_inspector_picking(cx) {
            window.with_element_state::<ViewElementState, _>(global_id.unwrap(), |state, window| {
                let mut state = state.expect("cached legacy state");
                let start = window.paint_index();
                if let Some(element) = element {
                    let refreshing = mem::replace(&mut window.refreshing, true);
                    element.paint(window, cx);
                    window.refreshing = refreshing;
                } else {
                    #[cfg(any(test, feature = "bench-support"))]
                    {
                        window.view_reuse_stats.reused_view_boundary_visits[2] += 1;
                    }
                    window.reuse_paint(state.paint_range.clone());
                }
                state.paint_range = start..window.paint_index();
                ((), state)
            })
        } else {
            element
                .as_mut()
                .expect("uncached element")
                .paint(window, cx);
        }
    });
}

#[inline(never)]
fn paint_component(
    name: &'static str,
    element: &mut Option<AnyElement>,
    window: &mut Window,
    cx: &mut App,
) {
    window.with_id(ElementId::Name(name.into()), |window| {
        element.as_mut().unwrap().paint(window, cx);
    });
}
