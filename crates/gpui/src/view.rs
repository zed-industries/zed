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
}

impl<V: Render> From<Entity<V>> for AnyView {
    fn from(value: Entity<V>) -> Self {
        AnyView {
            entity: value.into_any(),
            render: any_view::render::<V>,
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
        let retained_view = self.downgrade();
        ViewElement::new(self).cached(style, retained_view)
    }

    /// Convert this to a weak handle.
    pub fn downgrade(&self) -> AnyWeakView {
        AnyWeakView {
            entity: self.entity.downgrade(),
            render: self.render,
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

    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        (self.render)(&self, window, cx)
    }
}

impl<V: 'static + Render> IntoElement for Entity<V> {
    type Element = ViewElement<Entity<V>>;

    fn into_element(self) -> Self::Element {
        ViewElement::new(self)
    }

    #[inline(never)]
    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }
}

impl IntoElement for AnyView {
    type Element = ViewElement<AnyView>;

    fn into_element(self) -> Self::Element {
        ViewElement::new(self)
    }
}

/// A weak, dynamically-typed view handle.
#[derive(Clone)]
pub struct AnyWeakView {
    entity: AnyWeakEntity,
    render: fn(&AnyView, &mut Window, &mut App) -> AnyElement,
}

impl AnyWeakView {
    /// Upgrade to a strong `AnyView` handle, if the view is still alive.
    pub fn upgrade(&self) -> Option<AnyView> {
        let entity = self.entity.upgrade()?;
        Some(AnyView {
            entity,
            render: self.render,
        })
    }
}

impl<V: 'static + Render> From<WeakEntity<V>> for AnyWeakView {
    fn from(view: WeakEntity<V>) -> Self {
        AnyWeakView {
            entity: view.into(),
            render: any_view::render::<V>,
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
}

impl<T: Render> Entity<T> {
    /// Embed this entity as a cached [`ViewElement`] laid out at `style`.
    ///
    /// The rendered subtree is reused until the entity is notified (or the
    /// cached bounds / text style change). Caching requires a definite size:
    /// a cached view is laid out from `style` and is *not* measured from its
    /// contents. Use [`ViewElement::new`] (or `.child(entity)`) for the
    /// uncached case.
    #[track_caller]
    pub fn cached(self, style: StyleRefinement) -> ViewElement<Entity<T>> {
        let retained_view = self.downgrade().into();
        ViewElement::new(self).cached(style, retained_view)
    }
}

/// The element type for [`View`] implementations. Wraps a `View` and hooks it
/// into layout, prepaint, and paint. Constructed via [`ViewElement::new`].
#[doc(hidden)]
pub struct ViewElement<V: View> {
    view: Option<V>,
    entity_id: Option<EntityId>,
    cached_style: Option<StyleRefinement>,
    retained_view: Option<AnyWeakView>,
    #[cfg(debug_assertions)]
    source: &'static core::panic::Location<'static>,
}

impl<V: View> ViewElement<V> {
    /// Wrap a [`View`] as an element.
    #[track_caller]
    pub fn new(view: V) -> Self {
        let entity_id = view.entity_id();
        ViewElement {
            entity_id,
            cached_style: None,
            retained_view: None,
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
    fn cached(mut self, style: StyleRefinement, retained_view: AnyWeakView) -> Self {
        self.cached_style = Some(style);
        self.retained_view = Some(retained_view);
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

#[derive(Clone)]
struct ViewElementCacheKey {
    bounds: Bounds<Pixels>,
    content_mask: ContentMask<Pixels>,
    text_style: TextStyle,
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
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        if let Some(entity_id) = self.entity_id {
            // Stateful path: create a reactive boundary.
            let view = &mut self.view;
            request_layout_view(
                entity_id,
                self.cached_style.as_ref(),
                window,
                cx,
                &mut |window, cx| view.take().unwrap().render(window, cx).into_any_element(),
            )
        } else {
            // Stateless path: isolate subtree via type name (no entity identity).
            request_layout_component(type_name::<V>(), window, cx, &mut |window, cx| {
                self.view
                    .take()
                    .unwrap()
                    .render(window, cx)
                    .into_any_element()
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
            prepaint_view(
                entity_id,
                global_id,
                bounds,
                element,
                self.retained_view.as_ref(),
                window,
                cx,
                &mut |window, cx| {
                    self.view
                        .take()
                        .unwrap()
                        .render(window, cx)
                        .into_any_element()
                },
            )
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
            paint_view(
                entity_id,
                self.cached_style.is_some(),
                global_id,
                element,
                window,
                cx,
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
                    window.record_view_render(entity_id, caching_disabled);
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
    window: &mut Window,
    cx: &mut App,
    render: &mut dyn FnMut(&mut Window, &mut App) -> AnyElement,
) -> Option<AnyElement> {
    window.set_view_id(entity_id);
    let occurrence = window.begin_view_occurrence(entity_id, global_id, retained_view);
    let parent_occurrence = mem::replace(&mut window.current_view_occurrence, occurrence);
    let mut scope = (retained_view.is_some() && element.is_none()).then(|| {
        let mut scope = window.capture_view_scope();
        scope.parent_occurrence = parent_occurrence;
        scope
    });
    let previous_access = scope
        .as_ref()
        .map(|_| window.begin_cached_entity_access(cx));
    let result = window.with_rendered_view(entity_id, |window| {
        if let Some(mut element) = element.take() {
            element.prepaint(window, cx);
            return Some(element);
        }

        window.with_element_state::<ViewElementHandle, _>(global_id.unwrap(), |_handle, window| {
            let occurrence = occurrence.expect("cached views have an occurrence");
            let content_mask = window.content_mask();
            let text_style = window.text_style();

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
            let mut reused = false;
            if let Some(element_state) = element_state
                && element_state.cache_key.bounds == bounds
                && element_state.cache_key.content_mask == content_mask
                && element_state.cache_key.text_style == text_style
                && !window.view_store.is_own_dirty(occurrence)
                && (!window.dirty_views.contains(&entity_id)
                    || window.view_store.is_descendant_dirty(occurrence))
                && window.retained_scope_fallback != Some(retained::SpliceFallback::Transaction)
                && !window.refreshing
                && !force_reconstruction
            {
                let prepaint_range = element_state.prepaint_range.clone();
                let can_reuse = !window.view_store.is_descendant_dirty(occurrence)
                    || window.prepare_view_splice(occurrence).is_ok();
                if can_reuse {
                    reused = true;
                }
                if reused {
                    #[cfg(any(test, feature = "bench-support"))]
                    {
                        window.view_reuse_stats.cached_hits += 1;
                        window.view_reuse_stats.reused_view_boundary_visits[0] += 1;
                        window.view_reuse_stats.reused_view_boundary_visits[1] += 1;
                    }
                    let prepaint_start = window.prepaint_index();
                    if window.view_store.is_descendant_dirty(occurrence) {
                        window.splice_view_prepaint(occurrence, cx);
                    } else {
                        window.reuse_prepaint(prepaint_range);
                    }
                    let prepaint_end = window.prepaint_index();
                    window.view_store.reused_view_prepaint(
                        occurrence,
                        prepaint_start..prepaint_end,
                        scope.take().expect("cached views have a scope"),
                    );
                    cx.entities.extend_accessed(
                        &window
                            .view_store
                            .state(occurrence)
                            .expect("cached state")
                            .accessed_entities,
                    );
                    return (None, ViewElementHandle(occurrence));
                }
            }

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
                window.record_view_render(entity_id, false);
                window.cached_reconstruction_depth += 1;
            }
            let refreshing = mem::replace(&mut window.refreshing, true);
            let prepaint_start = window.prepaint_index();
            let mut element = render(window, cx);
            element.layout_as_root(Size::<AvailableSpace>::from(bounds.size), window, cx);
            element.prepaint_at(bounds.origin, window, cx);

            let prepaint_end = window.prepaint_index();
            window.refreshing = refreshing;
            #[cfg(any(test, feature = "bench-support"))]
            {
                window.cached_reconstruction_depth -= 1;
            }

            window.view_store.set_state(
                occurrence,
                ViewElementState {
                    accessed_entities: FxHashSet::default(),
                    prepaint_range: prepaint_start..prepaint_end,
                    paint_range: PaintIndex::default()..PaintIndex::default(),
                    cache_key: ViewElementCacheKey {
                        bounds,
                        content_mask,
                        text_style,
                    },
                },
                scope.take().expect("cached views have a scope"),
            );
            (Some(element), ViewElementHandle(occurrence))
        })
    });
    if let Some(previous_access) = previous_access {
        let (own, all) = window.end_cached_entity_access(previous_access, cx);
        if result.is_some() {
            window.view_store.set_own_access(
                occurrence.expect("cached occurrence"),
                own,
                all,
                false,
            );
        }
    }
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
    window: &mut Window,
    cx: &mut App,
) {
    let caching_disabled = window.is_inspector_picking(cx);
    let mut scope = (cached && !caching_disabled).then(|| window.capture_view_scope());
    let previous_access = scope
        .as_ref()
        .map(|_| window.begin_cached_entity_access(cx));
    let rendered = element.is_some();
    let mut painted_occurrence = None;
    window.with_rendered_view(entity_id, |window| {
        if cached && !caching_disabled {
            window.with_element_state::<ViewElementHandle, _>(
                global_id.unwrap(),
                |element_state, window| {
                    let handle = element_state.unwrap();
                    let occurrence = handle.0;
                    painted_occurrence = Some(occurrence);

                    window.next_frame.paint_views.push(occurrence);
                    let paint_start = window.paint_index();

                    if let Some(element) = element {
                        let refreshing = mem::replace(&mut window.refreshing, true);
                        element.paint(window, cx);
                        window.refreshing = refreshing;
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
                },
            )
        } else {
            element.as_mut().unwrap().paint(window, cx);
        }
    });
    if let Some(previous_access) = previous_access {
        let (own, all) = window.end_cached_entity_access(previous_access, cx);
        if rendered {
            window.view_store.set_own_access(
                painted_occurrence.expect("painted cached occurrence"),
                own,
                all,
                true,
            );
        }
    }
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
