use crate::{
    AnyElement, AnyEntity, AnyWeakEntity, App, Bounds, ContentMask, Context, Element, ElementId,
    Entity, EntityId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    NodeRenderDecision, PaintIndex, Pixels, PrepaintStateIndex, Render, RenderOnce, Style,
    StyleRefinement, TextStyle, ViewNodeCacheKey, ViewNodeId, ViewNodeRecording, WeakEntity,
};
use crate::{Empty, Window};
use anyhow::Result;
use collections::FxHashSet;
use refineable::Refineable;
use std::mem;
use std::{any::TypeId, fmt, ops::Range};

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
        ViewElement::new(self).cached(style)
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

    fn retained_view(&self) -> Option<AnyView> {
        Some(self.clone())
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
}

impl IntoElement for AnyView {
    type Element = ViewElement<AnyView>;

    fn into_element(self) -> Self::Element {
        ViewElement::new(self)
    }
}

/// A weak, dynamically-typed view handle.
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

    #[doc(hidden)]
    fn retained_view(&self) -> Option<AnyView> {
        None
    }

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

    fn retained_view(&self) -> Option<AnyView> {
        Some(self.clone().into())
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
        ViewElement::new(self).cached(style)
    }
}

/// The element type for [`View`] implementations. Wraps a `View` and hooks it
/// into layout, prepaint, and paint. Constructed via [`ViewElement::new`].
#[doc(hidden)]
pub struct ViewElement<V: View> {
    view: Option<V>,
    entity_id: Option<EntityId>,
    cached_style: Option<StyleRefinement>,
    retained_layout: Option<RetainedViewLayout>,
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
            retained_layout: None,
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
    pub(crate) fn cached(mut self, style: StyleRefinement) -> Self {
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

struct RetainedViewLayout {
    layout: LayoutId,
    layout_range: Option<Range<PrepaintStateIndex>>,
    node_id: ViewNodeId,
    recording: Option<ViewNodeRecording>,
    accessed_entities: FxHashSet<EntityId>,
}

struct ViewElementState {
    prepaint_range: Range<PrepaintStateIndex>,
    paint_range: Range<PaintIndex>,
    cache_key: ViewElementCacheKey,
    accessed_entities: FxHashSet<EntityId>,
}

struct ViewElementCacheKey {
    bounds: Bounds<Pixels>,
    content_mask: ContentMask<Pixels>,
    text_style: TextStyle,
}

#[doc(hidden)]
pub struct ViewElementPrepaintState {
    element: Option<AnyElement>,
    node: Option<ViewNodePrepaintState>,
}

enum ViewNodePrepaintState {
    Graft {
        node_id: ViewNodeId,
        recording: ViewNodeRecording,
    },
    Render {
        layout_range: Option<Range<PrepaintStateIndex>>,
        node_id: ViewNodeId,
        cache_key: ViewNodeCacheKey,
        prepaint_range: Range<PrepaintStateIndex>,
        accessed_entities: FxHashSet<EntityId>,
    },
}

impl<V: View> Element for ViewElement<V> {
    type RequestLayoutState = Option<AnyElement>;
    type PrepaintState = ViewElementPrepaintState;

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
        if self.cached_style.is_none()
            && window.node_engine_enabled()
            && let Some(entity_id) = self.entity_id
            && let Some(id) = id
            && let Some(view) = self.view.as_ref().and_then(View::retained_view)
        {
            let cache_key = window.view_node_key(Bounds::default());
            let (decision, previous_layout) =
                window.begin_view_node_layout(id.clone(), view.clone(), cache_key, cx);
            let node_id = match &decision {
                NodeRenderDecision::Graft { node_id, .. }
                | NodeRenderDecision::Render { node_id } => *node_id,
            };
            if let NodeRenderDecision::Graft {
                recording,
                accessed_entities,
                ..
            } = decision
                && let Some(layout) = previous_layout
            {
                cx.entities.extend_accessed(&accessed_entities);
                window.finish_view_node_prepaint(node_id, false, cx);
                let layout_range = window.graft_view_node_layout(&recording);
                self.retained_layout = Some(RetainedViewLayout {
                    layout,
                    layout_range,
                    node_id,
                    recording: Some(recording),
                    accessed_entities,
                });
                return (layout, None);
            }
            let layout_start = window.prepaint_index();
            let ((layout, element), accessed_entities) = cx.collect_accessed_entities(|cx| {
                window.with_rendered_view(entity_id, |window| {
                    let mut element = view.render(window, cx).into_any_element();
                    let layout = element.request_layout(window, cx);
                    (layout, element)
                })
            });
            window.store_view_node_layout(node_id, layout, cx);
            window.finish_view_node_prepaint(node_id, false, cx);
            self.retained_layout = Some(RetainedViewLayout {
                layout,
                layout_range: Some(layout_start..window.prepaint_index()),
                node_id,
                recording: None,
                accessed_entities,
            });
            return (layout, Some(element));
        }
        if let Some(entity_id) = self.entity_id {
            // Stateful path: create a reactive boundary.
            window.with_rendered_view(entity_id, |window| {
                let caching_disabled = window.is_inspector_picking(cx);
                match self.cached_style.as_ref() {
                    Some(style) if !caching_disabled => {
                        let mut root_style = Style::default();
                        root_style.refine(style);
                        let layout_id = window.request_layout(root_style, None, cx);
                        (layout_id, None)
                    }
                    _ => {
                        let mut element = self
                            .view
                            .take()
                            .unwrap()
                            .render(window, cx)
                            .into_any_element();
                        let layout_id = element.request_layout(window, cx);
                        (layout_id, Some(element))
                    }
                }
            })
        } else {
            // Stateless path: isolate subtree via type name (no entity identity).
            window.with_id(
                ElementId::Name(std::any::type_name::<V>().into()),
                |window| {
                    let mut element = self
                        .view
                        .take()
                        .unwrap()
                        .render(window, cx)
                        .into_any_element();
                    let layout_id = element.request_layout(window, cx);
                    (layout_id, Some(element))
                },
            )
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
    ) -> ViewElementPrepaintState {
        if let Some(retained) = self.retained_layout.take() {
            let cache_key = window.view_node_key(bounds);
            let node_id = retained.node_id;
            let entity_id = self.entity_id.expect("retained views have an entity");
            window.set_view_id(entity_id);
            window.enter_view_node_prepaint(node_id);
            return window.with_rendered_view(entity_id, |window| {
                if let Some(recording) = retained.recording
                    && window.view_node_cache_key(node_id, cx).as_ref() == Some(&cache_key)
                    && window.retained_layout_unchanged(retained.layout)
                {
                    window.graft_view_node_prepaint(&recording);
                    cx.entities.extend_accessed(&retained.accessed_entities);
                    cx.entities.recycle_access_scope(retained.accessed_entities);
                    window.finish_view_node_prepaint(node_id, false, cx);
                    return ViewElementPrepaintState {
                        element: None,
                        node: Some(ViewNodePrepaintState::Graft { node_id, recording }),
                    };
                }
                let rebuilding_layout = element.is_none();
                if rebuilding_layout {
                    window.restart_view_node_render(node_id, cx);
                }
                let prepaint_start = window.prepaint_index();
                let (element, mut prepaint_dependencies) = cx.collect_accessed_entities(|cx| {
                    if let Some(mut element) = element.take() {
                        element.prepaint(window, cx);
                        element
                    } else {
                        let view = self
                            .view
                            .as_ref()
                            .and_then(View::retained_view)
                            .expect("retained views can be rendered again");
                        let mut element = view.render(window, cx).into_any_element();
                        let layout = element.request_layout(window, cx);
                        window.replace_retained_layout(retained.layout, layout, cx);
                        window.store_view_node_layout(node_id, layout, cx);
                        element.prepaint(window, cx);
                        element
                    }
                });
                let mut accessed_entities = retained.accessed_entities;
                if rebuilding_layout {
                    accessed_entities.clear();
                }
                accessed_entities.extend(prepaint_dependencies.drain());
                cx.entities.recycle_access_scope(prepaint_dependencies);
                window.finish_view_node_prepaint(node_id, true, cx);
                ViewElementPrepaintState {
                    element: Some(element),
                    node: Some(ViewNodePrepaintState::Render {
                        layout_range: retained.layout_range,
                        node_id,
                        cache_key,
                        prepaint_range: prepaint_start..window.prepaint_index(),
                        accessed_entities,
                    }),
                }
            });
        }
        if self.cached_style.is_some()
            && window.node_engine_enabled()
            && let Some(entity_id) = self.entity_id
            && let Some(global_id) = global_id
            && let Some(view) = self.view.as_ref().and_then(View::retained_view)
        {
            let cache_key = window.view_node_key(bounds);
            if let Some(decision) =
                window.begin_view_node(global_id.clone(), view, cache_key.clone(), cx)
            {
                window.set_view_id(entity_id);
                return window.with_rendered_view(entity_id, |window| match decision {
                    NodeRenderDecision::Graft {
                        node_id,
                        recording,
                        accessed_entities,
                    } => {
                        window.graft_view_node_prepaint(&recording);
                        cx.entities.extend_accessed(&accessed_entities);
                        cx.entities.recycle_access_scope(accessed_entities);
                        window.finish_view_node_prepaint(node_id, false, cx);
                        ViewElementPrepaintState {
                            element: None,
                            node: Some(ViewNodePrepaintState::Graft { node_id, recording }),
                        }
                    }
                    NodeRenderDecision::Render { node_id } => {
                        let refreshing = mem::replace(&mut window.refreshing, true);
                        let prepaint_start = window.prepaint_index();
                        let (element, accessed_entities) = cx.collect_accessed_entities(|cx| {
                            let Some(view) = self.view.take() else {
                                return None;
                            };
                            let mut element = view.render(window, cx).into_any_element();
                            element.layout_as_root(bounds.size.into(), window, cx);
                            element.prepaint_at(bounds.origin, window, cx);
                            Some(element)
                        });
                        let prepaint_range = prepaint_start..window.prepaint_index();
                        window.refreshing = refreshing;
                        window.finish_view_node_prepaint(node_id, true, cx);
                        ViewElementPrepaintState {
                            element: element,
                            node: Some(ViewNodePrepaintState::Render {
                                layout_range: None,
                                node_id,
                                cache_key,
                                prepaint_range,
                                accessed_entities,
                            }),
                        }
                    }
                });
            }
        }

        if let Some(entity_id) = self.entity_id {
            // Stateful path.
            window.set_view_id(entity_id);
            window.with_rendered_view(entity_id, |window| {
                if let Some(mut element) = element.take() {
                    element.prepaint(window, cx);
                    return ViewElementPrepaintState {
                        element: Some(element),
                        node: None,
                    };
                }

                let element = window.with_element_state::<ViewElementState, _>(
                    global_id.unwrap(),
                    |element_state, window| {
                        let content_mask = window.content_mask();
                        let text_style = window.text_style();

                        if let Some(mut element_state) = element_state
                            && element_state.cache_key.bounds == bounds
                            && element_state.cache_key.content_mask == content_mask
                            && element_state.cache_key.text_style == text_style
                            && !window.dirty_views.contains(&entity_id)
                            && !window.refreshing
                        {
                            let prepaint_start = window.prepaint_index();
                            window.reuse_prepaint(element_state.prepaint_range.clone());
                            cx.entities
                                .extend_accessed(&element_state.accessed_entities);
                            let prepaint_end = window.prepaint_index();
                            element_state.prepaint_range = prepaint_start..prepaint_end;

                            return (None, element_state);
                        }

                        let refreshing = mem::replace(&mut window.refreshing, true);
                        let prepaint_start = window.prepaint_index();
                        let (mut element, accessed_entities) = cx.detect_accessed_entities(|cx| {
                            let mut element = self
                                .view
                                .take()
                                .unwrap()
                                .render(window, cx)
                                .into_any_element();
                            element.layout_as_root(bounds.size.into(), window, cx);
                            element.prepaint_at(bounds.origin, window, cx);
                            element
                        });

                        let prepaint_end = window.prepaint_index();
                        window.refreshing = refreshing;

                        (
                            Some(element),
                            ViewElementState {
                                accessed_entities,
                                prepaint_range: prepaint_start..prepaint_end,
                                paint_range: PaintIndex::default()..PaintIndex::default(),
                                cache_key: ViewElementCacheKey {
                                    bounds,
                                    content_mask,
                                    text_style,
                                },
                            },
                        )
                    },
                );
                ViewElementPrepaintState {
                    element,
                    node: None,
                }
            })
        } else {
            // Stateless path: just prepaint the element.
            window.with_id(
                ElementId::Name(std::any::type_name::<V>().into()),
                |window| {
                    element.as_mut().unwrap().prepaint(window, cx);
                },
            );
            ViewElementPrepaintState {
                element: element.take(),
                node: None,
            }
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
        if let Some(node) = element.node.take() {
            let node_id = match &node {
                ViewNodePrepaintState::Graft { node_id, .. }
                | ViewNodePrepaintState::Render { node_id, .. } => *node_id,
            };
            window.enter_view_node_prepaint(node_id);
            if let Some(entity_id) = self.entity_id {
                window.with_rendered_view(entity_id, |window| match node {
                    ViewNodePrepaintState::Graft { node_id, recording } => {
                        window.graft_view_node_paint(node_id, &recording, cx);
                        window.store_grafted_view_node(node_id, recording, cx);
                    }
                    ViewNodePrepaintState::Render {
                        layout_range,
                        node_id,
                        cache_key,
                        prepaint_range,
                        mut accessed_entities,
                    } => {
                        let recording = window.begin_view_node_paint(node_id, cx);
                        let paint_start = window.paint_index();
                        if let Some(element) = element.element.as_mut() {
                            let refreshing = mem::replace(&mut window.refreshing, true);
                            let (_, mut dependencies) =
                                cx.collect_accessed_entities(|cx| element.paint(window, cx));
                            accessed_entities.extend(dependencies.drain());
                            cx.entities.recycle_access_scope(dependencies);
                            window.refreshing = refreshing;
                        }
                        let paint_range = paint_start..window.paint_index();
                        let recording = window.capture_view_node_recording(
                            node_id,
                            recording,
                            layout_range,
                            prepaint_range,
                            paint_range,
                        );
                        window.store_rendered_view_node(
                            node_id,
                            cache_key,
                            recording,
                            accessed_entities,
                            cx,
                        );
                    }
                });
            }
            window.finish_view_node_prepaint(node_id, false, cx);
            return;
        }

        let element = &mut element.element;
        if let Some(entity_id) = self.entity_id {
            // Stateful path.
            window.with_rendered_view(entity_id, |window| {
                let caching_disabled = window.is_inspector_picking(cx);
                if self.cached_style.is_some() && !caching_disabled {
                    window.with_element_state::<ViewElementState, _>(
                        global_id.unwrap(),
                        |element_state, window| {
                            let mut element_state = element_state.unwrap();

                            let paint_start = window.paint_index();

                            if let Some(element) = element {
                                let refreshing = mem::replace(&mut window.refreshing, true);
                                element.paint(window, cx);
                                window.refreshing = refreshing;
                            } else {
                                window.reuse_paint(element_state.paint_range.clone());
                            }

                            let paint_end = window.paint_index();
                            element_state.paint_range = paint_start..paint_end;

                            ((), element_state)
                        },
                    )
                } else {
                    element.as_mut().unwrap().paint(window, cx);
                }
            });
        } else {
            // Stateless path: just paint the element.
            window.with_id(
                ElementId::Name(std::any::type_name::<V>().into()),
                |window| {
                    element.as_mut().unwrap().paint(window, cx);
                },
            );
        }
    }
}

/// A component whose inputs can be retained and rendered again after local state changes.
/// Parent renders supply fresh inputs through [`component`].
pub trait Component: 'static {
    /// Builds the component's elements from its current inputs and local state.
    fn render(&self, window: &mut Window, cx: &mut App) -> impl IntoElement;
}

/// Mounts a repeatable component under a key that is unique in its containing element scope.
/// Reusing the key preserves local state; supplying a new value replaces its inputs.
pub fn component<C: Component>(key: impl Into<ElementId>, value: C) -> impl IntoElement {
    ComponentView {
        key: key.into(),
        value,
    }
}

#[derive(IntoElement)]
struct ComponentView<C: Component> {
    key: ElementId,
    value: C,
}

struct ComponentInstance<C: Component> {
    value: C,
}

impl<C: Component> Render for ComponentInstance<C> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.value.render(window, cx)
    }
}

impl<C: Component> RenderOnce for ComponentView<C> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut value = Some(self.value);
        let instance = window.use_keyed_state(self.key, cx, |_, _| ComponentInstance {
            value: value.take().expect("component inputs are consumed once"),
        });
        if let Some(value) = value {
            instance.update(cx, |instance, _| instance.value = value);
            window.invalidate_component(instance.entity_id());
        }
        instance
    }
}

/// A view that renders nothing
pub struct EmptyView;

impl Render for EmptyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Context, DrawEngine, Entity, Render, StyleRefinement, TestAppContext, Window, div,
        prelude::*, px, rgb, size,
    };
    use std::{cell::Cell, rc::Rc};

    #[gpui::test]
    fn node_engine_replays_debug_bounds_in_paint_order(cx: &mut TestAppContext) {
        struct Leaf(&'static str);
        impl Render for Leaf {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let label = self.0;
                div()
                    .w(px(100.))
                    .h(px(100.))
                    .debug_selector(|| "shared".into())
                    .child(div().size_full().debug_selector(move || label.into()))
            }
        }
        struct Root {
            children: Vec<Entity<Leaf>>,
            cached: bool,
        }
        impl Render for Root {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .flex()
                    .size_full()
                    .children(self.children.iter().map(|child| {
                        if self.cached {
                            child
                                .clone()
                                .cached(StyleRefinement::default().w(px(100.)).h(px(100.)))
                                .into_any_element()
                        } else {
                            child.clone().into_any_element()
                        }
                    }))
            }
        }
        for retained in [false, true] {
            for cached in [false, true] {
                let window = cx.open_window(size(px(300.), px(100.)), |window, cx| {
                    window.draw_engine = if retained {
                        DrawEngine::Node(crate::NodeEngine::new())
                    } else {
                        DrawEngine::Legacy
                    };
                    Root {
                        children: ["a", "b", "c"]
                            .into_iter()
                            .map(|label| cx.new(|_| Leaf(label)))
                            .collect(),
                        cached,
                    }
                });
                cx.run_until_parked();
                for step in 0..3 {
                    window
                        .update(cx, |root, _, cx| match step {
                            0 => root
                                .children
                                .first()
                                .expect("first leaf")
                                .update(cx, |_, cx| cx.notify()),
                            1 => {
                                root.children.swap(0, 2);
                                cx.notify();
                            }
                            _ => {
                                root.children.pop();
                                cx.notify();
                            }
                        })
                        .expect("window");
                    cx.run_until_parked();
                    let actual = window
                        .update(cx, |_, window, _| {
                            if retained && step == 0 {
                                assert!(
                                    window
                                        .retained_node_stats()
                                        .expect("retained stats")
                                        .reused_subtrees
                                        > 0
                                );
                            }
                            window.rendered_frame.debug_bounds.clone()
                        })
                        .expect("window");
                    if step == 2 {
                        assert!(!actual.contains_key("a"));
                    }
                    assert_eq!(
                        actual.get("shared"),
                        actual.get(if step == 0 {
                            "c"
                        } else if step == 1 {
                            "a"
                        } else {
                            "b"
                        })
                    );
                    window
                        .update(cx, |_, window, _| window.refresh())
                        .expect("window");
                    cx.run_until_parked();
                    let expected = window
                        .update(cx, |_, window, _| {
                            window.rendered_frame.debug_bounds.clone()
                        })
                        .expect("window");
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    struct Dependency;

    struct CountingLeaf {
        render_count: Rc<Cell<usize>>,
        dependency: Option<Entity<Dependency>>,
        color: u32,
    }

    impl Render for CountingLeaf {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            self.render_count.set(self.render_count.get() + 1);
            if let Some(dependency) = &self.dependency {
                dependency.read(cx);
            }
            div().size_full().bg(rgb(self.color))
        }
    }

    struct NodeEngineRoot {
        left: Entity<CountingLeaf>,
        middle: Entity<CountingLeaf>,
        right: Entity<CountingLeaf>,
    }

    impl Render for NodeEngineRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let leaf_style = || StyleRefinement::default().w(px(100.)).h(px(100.));
            div()
                .flex()
                .flex_row()
                .size_full()
                .child(self.left.clone().cached(leaf_style()))
                .child(self.middle.clone().cached(leaf_style()))
                .child(self.right.clone().cached(leaf_style()))
        }
    }

    #[gpui::test]
    fn node_engine_grafts_clean_siblings_and_cold_rebuilds_the_same_scene(cx: &mut TestAppContext) {
        let left_render_count = Rc::new(Cell::new(0));
        let middle_render_count = Rc::new(Cell::new(0));
        let right_render_count = Rc::new(Cell::new(0));
        let dependency = cx.new(|_| Dependency);
        let _node_engine_guard = DrawEngine::force_node_engine_for_test();
        let window = cx.open_window(size(px(300.), px(100.)), |_, cx| NodeEngineRoot {
            left: cx.new({
                let left_render_count = left_render_count.clone();
                |_| CountingLeaf {
                    render_count: left_render_count,
                    dependency: None,
                    color: 0xff0000,
                }
            }),
            middle: cx.new({
                let middle_render_count = middle_render_count.clone();
                let dependency = dependency.clone();
                |_| CountingLeaf {
                    render_count: middle_render_count,
                    dependency: Some(dependency),
                    color: 0x00ff00,
                }
            }),
            right: cx.new({
                let right_render_count = right_render_count.clone();
                |_| CountingLeaf {
                    render_count: right_render_count,
                    dependency: None,
                    color: 0x0000ff,
                }
            }),
        });
        cx.run_until_parked();

        assert_eq!(
            (
                left_render_count.get(),
                middle_render_count.get(),
                right_render_count.get(),
            ),
            (1, 1, 1)
        );

        window
            .update(cx, |root, _, cx| {
                root.middle.update(cx, |_, cx| cx.notify());
            })
            .expect("test window should remain open");
        cx.run_until_parked();
        assert_eq!(
            (
                left_render_count.get(),
                middle_render_count.get(),
                right_render_count.get(),
            ),
            (1, 2, 1)
        );

        dependency.update(cx, |_, cx| cx.notify());
        cx.run_until_parked();
        assert_eq!(
            (
                left_render_count.get(),
                middle_render_count.get(),
                right_render_count.get(),
            ),
            (1, 3, 1)
        );

        let retained_scene = window
            .update(cx, |_, window, _| {
                window.rendered_frame.scene.snapshot_for_test()
            })
            .expect("test window should remain open");
        window
            .update(cx, |_, window, _| {
                window.clear_view_nodes_for_test();
                window.refresh();
            })
            .expect("test window should remain open");
        cx.run_until_parked();
        let cold_scene = window
            .update(cx, |_, window, _| {
                window.rendered_frame.scene.snapshot_for_test()
            })
            .expect("test window should remain open");

        assert_eq!(retained_scene, cold_scene);
        assert_eq!(
            (
                left_render_count.get(),
                middle_render_count.get(),
                right_render_count.get(),
            ),
            (2, 4, 2)
        );
    }

    #[gpui::test]
    fn node_engine_does_not_reinvalidate_previously_notified_views(cx: &mut TestAppContext) {
        let left_render_count = Rc::new(Cell::new(0));
        let middle_render_count = Rc::new(Cell::new(0));
        let right_render_count = Rc::new(Cell::new(0));
        let _node_engine_guard = DrawEngine::force_node_engine_for_test();
        let window = cx.open_window(size(px(300.), px(100.)), |_, cx| NodeEngineRoot {
            left: cx.new({
                let left_render_count = left_render_count.clone();
                |_| CountingLeaf {
                    render_count: left_render_count,
                    dependency: None,
                    color: 0xff0000,
                }
            }),
            middle: cx.new({
                let middle_render_count = middle_render_count.clone();
                |_| CountingLeaf {
                    render_count: middle_render_count,
                    dependency: None,
                    color: 0x00ff00,
                }
            }),
            right: cx.new({
                let right_render_count = right_render_count.clone();
                |_| CountingLeaf {
                    render_count: right_render_count,
                    dependency: None,
                    color: 0x0000ff,
                }
            }),
        });
        cx.run_until_parked();
        assert_eq!(
            (
                left_render_count.get(),
                middle_render_count.get(),
                right_render_count.get(),
            ),
            (1, 1, 1)
        );

        window
            .update(cx, |root, _, cx| {
                root.left.update(cx, |_, cx| cx.notify());
            })
            .expect("test window should remain open");
        cx.run_until_parked();
        assert_eq!(
            (
                left_render_count.get(),
                middle_render_count.get(),
                right_render_count.get(),
            ),
            (2, 1, 1)
        );

        // A later frame triggered by a different view must not re-render views
        // that were notified on earlier frames.
        window
            .update(cx, |root, _, cx| {
                root.right.update(cx, |_, cx| cx.notify());
            })
            .expect("test window should remain open");
        cx.run_until_parked();
        assert_eq!(
            (
                left_render_count.get(),
                middle_render_count.get(),
                right_render_count.get(),
            ),
            (2, 1, 2)
        );
    }
    struct IntrinsicLeaf {
        renders: Rc<Cell<usize>>,
        width: f32,
        color: u32,
    }

    impl Render for IntrinsicLeaf {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div()
                .w(px(self.width))
                .h(px(50.))
                .bg(rgb(self.color))
                .child("Retained text")
        }
    }

    struct IntrinsicRoot {
        leaves: Vec<Entity<IntrinsicLeaf>>,
        show_first: bool,
        reverse: bool,
        opacity: f32,
    }

    impl Render for IntrinsicRoot {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let mut leaves = self
                .leaves
                .iter()
                .enumerate()
                .filter(|(index, _)| self.show_first || *index != 0)
                .map(|(_, leaf)| leaf.clone())
                .collect::<Vec<_>>();
            if self.reverse {
                leaves.reverse();
            }
            div().flex().gap_2().opacity(self.opacity).children(leaves)
        }
    }

    #[gpui::test]
    fn node_engine_automatic_views_match_legacy_across_layout_and_mount_changes(
        cx: &mut TestAppContext,
    ) {
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<IntrinsicRoot>| {
                window.draw_engine = engine;
                IntrinsicRoot {
                    leaves: (0..3)
                        .map(|_| {
                            cx.new(|_| IntrinsicLeaf {
                                renders: Rc::new(Cell::new(0)),
                                width: 80.,
                                color: 0x225599,
                            })
                        })
                        .collect(),
                    show_first: true,
                    reverse: false,
                    opacity: 1.,
                }
            }
        };
        let legacy = cx.open_window(size(px(400.), px(100.)), build(DrawEngine::Legacy));
        let retained = cx.open_window(
            size(px(400.), px(100.)),
            build(DrawEngine::Node(crate::NodeEngine::new())),
        );
        cx.run_until_parked();
        for step in 0..10 {
            for window in [legacy, retained] {
                window
                    .update(cx, |root, _, cx| match step {
                        0 => {}
                        1 => root
                            .leaves
                            .first()
                            .expect("first leaf")
                            .update(cx, |leaf, cx| {
                                leaf.color = 0xff0000;
                                cx.notify();
                            }),
                        2 => root
                            .leaves
                            .first()
                            .expect("first leaf")
                            .update(cx, |leaf, cx| {
                                leaf.width = 130.;
                                cx.notify();
                            }),
                        3 => {
                            root.reverse = true;
                            cx.notify();
                        }
                        4 => {
                            root.show_first = false;
                            cx.notify();
                        }
                        5 => {
                            root.show_first = true;
                            cx.notify();
                        }
                        6 => cx.notify(),
                        7 => {
                            root.opacity = 0.5;
                            cx.notify();
                        }
                        8 => {
                            root.leaves
                                .first()
                                .expect("first leaf")
                                .update(cx, |leaf, _| leaf.color = 0x9900ff);
                            cx.notify();
                        }
                        _ => root
                            .leaves
                            .last()
                            .expect("last leaf")
                            .update(cx, |leaf, cx| {
                                leaf.color = 0x00ff00;
                                cx.notify();
                            }),
                    })
                    .expect("window remains open");
            }
            cx.run_until_parked();
            let snapshot = |window: crate::WindowHandle<IntrinsicRoot>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, _| {
                        window.rendered_frame.scene.snapshot_for_test()
                    })
                    .expect("window remains open")
            };
            assert_eq!(
                snapshot(legacy, cx),
                snapshot(retained, cx),
                "frame after step {step}"
            );
            if step == 1 {
                retained
                    .update(cx, |root, _, cx| {
                        assert_eq!(
                            root.leaves
                                .get(1)
                                .expect("middle leaf")
                                .read(cx)
                                .renders
                                .get(),
                            1,
                            "an ordinary clean sibling should not render again"
                        );
                    })
                    .expect("window remains open");
            }
        }
    }

    struct PercentageLeaf {
        relative_width: bool,
    }

    impl Render for PercentageLeaf {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .w(px(100.))
                .when(self.relative_width, |element| {
                    element.w(crate::relative(0.5))
                })
                .h(px(100.))
                .p(crate::relative(0.1))
                .bg(rgb(0x336699))
                .child(div().size_full().bg(rgb(0xff0000)))
        }
    }

    struct PercentageHost {
        width: f32,
        layout_mode: usize,
        leaf: Entity<PercentageLeaf>,
    }

    impl Render for PercentageHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .w(px(self.width))
                .h(px(120.))
                .when(self.layout_mode == 1, |element| element.flex())
                .when(self.layout_mode == 2, |element| element.grid().grid_cols(2))
                .child(self.leaf.clone())
                .child(div().w(px(180.)).h(px(40.)).bg(rgb(0xabcdef)))
        }
    }

    #[gpui::test]
    fn node_engine_preserves_percentage_layout_after_parent_resize(cx: &mut TestAppContext) {
        for relative_width in [false, true] {
            for layout_mode in 0..3 {
                let build = |engine| {
                    move |window: &mut Window, cx: &mut Context<PercentageHost>| {
                        window.draw_engine = engine;
                        PercentageHost {
                            width: 200.,
                            layout_mode,
                            leaf: cx.new(|_| PercentageLeaf { relative_width }),
                        }
                    }
                };
                let legacy = cx.open_window(size(px(400.), px(200.)), build(DrawEngine::Legacy));
                let retained = cx.open_window(
                    size(px(400.), px(200.)),
                    build(DrawEngine::Node(crate::NodeEngine::new())),
                );
                for width in [200., 300., 160., 320., 320.] {
                    for window in [legacy, retained] {
                        window
                            .update(cx, |host, _, cx| {
                                host.width = width;
                                cx.notify();
                            })
                            .expect("window open");
                    }
                    cx.run_until_parked();
                    let snapshot = |window: crate::WindowHandle<PercentageHost>,
                                    cx: &mut TestAppContext| {
                        window
                            .update(cx, |_, window, _| {
                                window.rendered_frame.scene.snapshot_for_test()
                            })
                            .expect("window open")
                    };
                    assert_eq!(
                        snapshot(legacy, cx),
                        snapshot(retained, cx),
                        "parent width {width}, relative {relative_width}, layout mode {layout_mode}"
                    );
                }
            }
        }
    }

    struct InteractiveLeaf {
        focus: crate::FocusHandle,
        clicks: usize,
        keys: usize,
    }

    impl Render for InteractiveLeaf {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("interactive")
                .track_focus(&self.focus)
                .w(px(80.))
                .h(px(40.))
                .bg(rgb(0x336699))
                .hover(|style| style.bg(rgb(0x993366)))
                .child(format!("{}/{}", self.clicks, self.keys))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.clicks += 1;
                    cx.notify();
                }))
                .on_key_down(cx.listener(|this, event: &crate::KeyDownEvent, _, cx| {
                    if event.keystroke.key == "enter" {
                        this.keys += 1;
                        cx.notify();
                    }
                }))
        }
    }

    struct InteractiveHost {
        offset: f32,
        leaf: Entity<InteractiveLeaf>,
    }

    impl Render for InteractiveHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .child(div().w(px(self.offset)).h(px(40.)))
                .child(self.leaf.clone())
        }
    }

    #[gpui::test]
    fn node_engine_preserves_focus_hover_and_moved_hit_targets(cx: &mut TestAppContext) {
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<InteractiveHost>| {
                window.draw_engine = engine;
                let leaf = cx.new(|cx| InteractiveLeaf {
                    focus: cx.focus_handle(),
                    clicks: 0,
                    keys: 0,
                });
                leaf.read(cx).focus.clone().focus(window, cx);
                InteractiveHost { offset: 0., leaf }
            }
        };
        let legacy = cx.open_window(size(px(300.), px(100.)), build(DrawEngine::Legacy));
        let retained = cx.open_window(
            size(px(300.), px(100.)),
            build(DrawEngine::Node(crate::NodeEngine::new())),
        );
        cx.run_until_parked();
        for step in 0..8 {
            for window in [legacy, retained] {
                let mut visual = crate::VisualTestContext::from_window(window.into(), cx);
                match step {
                    0 | 1 => {
                        window
                            .update(cx, |_, _, cx| cx.notify())
                            .expect("window open");
                    }
                    2 => visual.simulate_keystrokes("enter"),
                    3 => visual.simulate_mouse_move(
                        crate::point(px(10.), px(10.)),
                        None,
                        crate::Modifiers::default(),
                    ),
                    4 => {
                        window
                            .update(cx, |host, _, cx| {
                                host.offset = 120.;
                                cx.notify();
                            })
                            .expect("window open");
                    }
                    5 => visual.simulate_click(
                        crate::point(px(10.), px(10.)),
                        crate::Modifiers::default(),
                    ),
                    6 => visual.simulate_click(
                        crate::point(px(130.), px(10.)),
                        crate::Modifiers::default(),
                    ),
                    _ => visual.simulate_keystrokes("enter"),
                }
            }
            cx.run_until_parked();
            let snapshot = |window: crate::WindowHandle<InteractiveHost>,
                            cx: &mut TestAppContext| {
                window
                    .update(cx, |host, window, cx| {
                        let leaf = host.leaf.read(cx);
                        (
                            window.rendered_frame.scene.snapshot_for_test(),
                            leaf.clicks,
                            leaf.keys,
                            leaf.focus.is_focused(window),
                        )
                    })
                    .expect("window open")
            };
            assert_eq!(
                snapshot(legacy, cx),
                snapshot(retained, cx),
                "interaction {step}"
            );
        }
        retained
            .update(cx, |host, _, cx| {
                let leaf = host.leaf.read(cx);
                assert_eq!(leaf.clicks, 1, "old geometry must not retain a hit target");
                assert_eq!(leaf.keys, 2, "focus and key listeners must survive reuse");
            })
            .expect("window open");
    }

    struct ArenaMeasuredElement {
        lifetime: Rc<()>,
    }

    impl IntoElement for ArenaMeasuredElement {
        type Element = Self;
        fn into_element(self) -> Self {
            self
        }
    }

    impl crate::Element for ArenaMeasuredElement {
        type RequestLayoutState = ();
        type PrepaintState = ();
        fn id(&self) -> Option<crate::ElementId> {
            None
        }
        fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
            None
        }
        fn request_layout(
            &mut self,
            _: Option<&crate::GlobalElementId>,
            _: Option<&crate::InspectorElementId>,
            window: &mut Window,
            _: &mut crate::App,
        ) -> (crate::LayoutId, ()) {
            let child = std::cell::RefCell::new(div().into_any_element());
            let lifetime = self.lifetime.clone();
            (
                window.request_measured_layout(crate::Style::default(), move |_, _, _, _| {
                    std::hint::black_box(&lifetime);
                    assert!(child.borrow_mut().downcast_mut::<crate::Div>().is_some());
                    size(px(80.), px(40.))
                }),
                (),
            )
        }
        fn prepaint(
            &mut self,
            _: Option<&crate::GlobalElementId>,
            _: Option<&crate::InspectorElementId>,
            _: crate::Bounds<crate::Pixels>,
            _: &mut (),
            _: &mut Window,
            _: &mut crate::App,
        ) {
        }
        fn paint(
            &mut self,
            _: Option<&crate::GlobalElementId>,
            _: Option<&crate::InspectorElementId>,
            _: crate::Bounds<crate::Pixels>,
            _: &mut (),
            _: &mut (),
            _: &mut Window,
            _: &mut crate::App,
        ) {
        }
    }

    struct ArenaMeasuredView {
        lifetime: Rc<()>,
        renders: Rc<Cell<usize>>,
    }

    impl Render for ArenaMeasuredView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            ArenaMeasuredElement {
                lifetime: self.lifetime.clone(),
            }
        }
    }

    struct ArenaMeasuredHost(Entity<ArenaMeasuredView>);

    impl Render for ArenaMeasuredHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().child(self.0.clone())
        }
    }

    #[gpui::test]
    fn node_engine_releases_frame_bound_measurement_callbacks(cx: &mut TestAppContext) {
        let lifetime = Rc::new(());
        let renders = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(200.), px(100.)), |window, cx| {
            window.draw_engine = DrawEngine::Node(crate::NodeEngine::new());
            ArenaMeasuredHost(cx.new(|_| ArenaMeasuredView {
                lifetime: lifetime.clone(),
                renders: renders.clone(),
            }))
        });
        cx.run_until_parked();
        for _ in 0..4 {
            let before = renders.get();
            window
                .update(cx, |_, _, cx| cx.notify())
                .expect("window open");
            cx.run_until_parked();
            assert!(
                renders.get() > before,
                "an arena capture cannot be reused on a later frame"
            );
            assert_eq!(
                Rc::strong_count(&lifetime),
                2,
                "the measurement closure must be released before the next frame"
            );
        }
    }

    struct BenchmarkLeaf {
        revision: usize,
    }

    impl Render for BenchmarkLeaf {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .w(px(120.))
                .h(px(160.))
                .flex()
                .flex_col()
                .children((0..8).map(|row| {
                    div()
                        .h(px(20.))
                        .child(format!("Row {row}: {}", self.revision))
                }))
        }
    }

    struct BenchmarkHost {
        leaves: Vec<Entity<BenchmarkLeaf>>,
    }

    impl Render for BenchmarkHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .flex()
                .flex_wrap()
                .children(self.leaves.iter().cloned())
        }
    }

    #[gpui::test]
    fn node_engine_reuses_after_fully_dirty_frames(cx: &mut TestAppContext) {
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<BenchmarkHost>| {
                window.draw_engine = engine;
                BenchmarkHost {
                    leaves: (0..3)
                        .map(|_| cx.new(|_| BenchmarkLeaf { revision: 0 }))
                        .collect(),
                }
            }
        };
        let legacy = cx.open_window(size(px(500.), px(300.)), build(DrawEngine::Legacy));
        let retained = cx.open_window(
            size(px(500.), px(300.)),
            build(DrawEngine::Node(crate::NodeEngine::new())),
        );
        cx.run_until_parked();
        let snapshot = |window: crate::WindowHandle<BenchmarkHost>, cx: &mut TestAppContext| {
            window
                .update(cx, |_, window, _| {
                    window.rendered_frame.scene.snapshot_for_test()
                })
                .expect("window open")
        };
        for step in 0..6 {
            let dirty_all = step % 2 == 0;
            for window in [legacy, retained] {
                window
                    .update(cx, |host, window, cx| {
                        for leaf in host.leaves.iter().take(if dirty_all { 3 } else { 1 }) {
                            leaf.update(cx, |leaf, cx| {
                                leaf.revision += 1;
                                cx.notify();
                            });
                        }
                        if step == 4 {
                            window.refresh();
                        }
                    })
                    .expect("window open");
            }
            cx.run_until_parked();
            assert_eq!(snapshot(legacy, cx), snapshot(retained, cx));
            retained
                .update(cx, |_, window, _| {
                    let stats = window.retained_node_stats().expect("retained engine");
                    assert_eq!(stats.rebuilt_scopes, if dirty_all { 4 } else { 2 });
                    assert_eq!(stats.reused_subtrees, if dirty_all { 0 } else { 2 });
                    assert_eq!(window.rendered_frame.scene.paint_operations.capacity(), 0);
                    assert_eq!(window.next_frame.scene.paint_operations.capacity(), 0);
                })
                .expect("window open");
        }
    }

    #[gpui::test]
    #[ignore = "manual CPU benchmark; run with --release --ignored --nocapture"]
    fn node_engine_update_benchmark(cx: &mut TestAppContext) {
        #[cfg(feature = "test-memory")]
        eprintln!(
            "test-memory counts live Rust allocations; timing includes allocator instrumentation"
        );
        #[cfg(not(target_os = "macos"))]
        assert!(
            std::env::var_os("GPUI_BENCH_MEMORY").is_none(),
            "malloc-zone memory sampling requires macOS"
        );
        let cycles = std::env::var("GPUI_BENCH_MEMORY_CYCLES").map_or(1, |value| {
            value.parse::<usize>().expect("memory cycle count")
        });
        assert!(cycles > 0);
        assert!(cycles == 1 || std::env::var_os("GPUI_BENCH_MEMORY").is_some());
        for cycle in 0..cycles {
            eprintln!("mount cycle {cycle}");
            run_node_engine_update_benchmark(cx);
        }
    }

    fn run_node_engine_update_benchmark(cx: &mut TestAppContext) {
        #[cfg(target_os = "macos")]
        fn heap_bytes() -> usize {
            #[repr(C)]
            struct MallocStatistics {
                blocks_in_use: u32,
                size_in_use: usize,
                max_size_in_use: usize,
                size_allocated: usize,
            }
            unsafe extern "C" {
                fn malloc_zone_statistics(
                    zone: *mut std::ffi::c_void,
                    statistics: *mut MallocStatistics,
                );
            }
            let mut statistics = MallocStatistics {
                blocks_in_use: 0,
                size_in_use: 0,
                max_size_in_use: 0,
                size_allocated: 0,
            };
            // A null zone sums all malloc zones, including native framework allocations.
            unsafe { malloc_zone_statistics(std::ptr::null_mut(), &mut statistics) };
            #[cfg(feature = "test-memory")]
            eprintln!(
                "Rust live requested bytes: {}",
                crate::test::memory::live_bytes()
            );
            statistics.size_in_use
        }
        #[cfg(target_os = "macos")]
        let memory_baseline = std::env::var_os("GPUI_BENCH_MEMORY").map(|_| heap_bytes());
        let dirty_all = std::env::var_os("GPUI_BENCH_ALL_DIRTY").is_some();
        eprintln!(
            "workload: {}",
            if dirty_all {
                "all leaves dirty"
            } else {
                "one leaf dirty"
            }
        );
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<BenchmarkHost>| {
                window.draw_engine = engine;
                BenchmarkHost {
                    leaves: (0..64)
                        .map(|_| cx.new(|_| BenchmarkLeaf { revision: 0 }))
                        .collect(),
                }
            }
        };
        let selected = std::env::var("GPUI_BENCH_ENGINE").ok();
        #[cfg(target_os = "macos")]
        assert!(
            memory_baseline.is_none() || selected.is_some(),
            "memory measurements require a single engine per process"
        );
        let engines: Vec<_> = ["legacy", "retained"]
            .into_iter()
            .filter(|name| selected.as_deref().is_none_or(|selected| selected == *name))
            .collect();
        assert!(
            !engines.is_empty(),
            "GPUI_BENCH_ENGINE must be legacy or retained"
        );
        let windows: Vec<_> = engines
            .iter()
            .map(|name| {
                let engine = if *name == "legacy" {
                    DrawEngine::Legacy
                } else {
                    DrawEngine::Node(crate::NodeEngine::new())
                };
                cx.open_window(size(px(1600.), px(1000.)), build(engine))
            })
            .collect();
        cx.run_until_parked();
        #[cfg(target_os = "macos")]
        if let Some(baseline) = memory_baseline {
            let live = heap_bytes();
            eprintln!(
                "memory after mount: baseline={baseline} live={live} delta={}",
                live as i128 - baseline as i128
            );
        }
        let mut samples = vec![Vec::new(); windows.len()];
        let mut previous_layout_count = None;
        for round in 0..5 {
            let mut order: Vec<_> = (0..windows.len()).collect();
            if round % 2 != 0 {
                order.reverse();
            }
            for index in order {
                let window = windows[index];
                let started = std::time::Instant::now();
                for step in 0..100 {
                    window
                        .update(cx, |host, _, cx| {
                            for (index, leaf) in host.leaves.iter().enumerate() {
                                if dirty_all || index == step % 64 {
                                    leaf.update(cx, |leaf, cx| {
                                        leaf.revision += 1;
                                        cx.notify();
                                    });
                                }
                            }
                        })
                        .expect("window open");
                    cx.run_until_parked();
                }
                samples[index].push(started.elapsed().as_secs_f64() * 10_000.);
            }
            let snapshot = |window: crate::WindowHandle<BenchmarkHost>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, _| {
                        window.rendered_frame.scene.snapshot_for_test()
                    })
                    .expect("window open")
            };
            if let [legacy, retained] = windows.as_slice() {
                assert_eq!(snapshot(*legacy, cx), snapshot(*retained, cx));
            }
            for window in &windows {
                window
                    .update(cx, |_, window, _| {
                        let Some(stats) = window.retained_node_stats() else {
                            return;
                        };
                        if let Some(previous) = previous_layout_count {
                            assert_eq!(stats.layout_nodes, previous);
                        }
                        previous_layout_count = Some(stats.layout_nodes);
                        eprintln!("round {round}: {stats:?}");
                    })
                    .expect("window open");
            }
            #[cfg(target_os = "macos")]
            if let Some(baseline) = memory_baseline {
                let live = heap_bytes();
                eprintln!(
                    "memory round {round}: live={live} delta={}",
                    live as i128 - baseline as i128
                );
                for window in &windows {
                    window.update(cx, |_, window, cx| {
                        let frame_operations = (window.rendered_frame.scene.paint_operations.capacity()
                            + window.next_frame.scene.paint_operations.capacity())
                            * std::mem::size_of::<crate::scene::PaintOperation>();
                        let recorded_operations = match &window.draw_engine {
                            DrawEngine::Legacy => 0,
                            DrawEngine::Node(engine) => engine.recorded_operation_buffer_bytes(cx),
                        };
                        let arena_bytes = cx.element_arena.borrow().capacity();
                        eprintln!("paint-operation buffer bytes: frames={frame_operations} recordings={recorded_operations}; element arena bytes={arena_bytes}");
                    }).expect("window open");
                }
            }
        }
        for (name, mut samples) in engines.into_iter().zip(samples) {
            samples.sort_by(f64::total_cmp);
            eprintln!("{name}: microseconds/update {samples:?}");
        }
        #[cfg(target_os = "macos")]
        if let Some(baseline) = memory_baseline {
            for window in windows {
                window
                    .update(cx, |_, window, _| window.remove_window())
                    .expect("window open");
            }
            cx.run_until_parked();
            let live = heap_bytes();
            eprintln!(
                "memory after unmount: live={live} delta={}",
                live as i128 - baseline as i128
            );
        }
    }

    struct StatefulComponent {
        state: Rc<std::cell::RefCell<Option<Entity<usize>>>>,
        seen_revision: Rc<Cell<usize>>,
        revision: usize,
    }

    impl super::Component for StatefulComponent {
        fn render(&self, window: &mut Window, cx: &mut crate::App) -> impl IntoElement {
            let count = window.use_state(cx, |_, _| 0usize);
            *self.state.borrow_mut() = Some(count.clone());
            let value = *count.read(cx);
            let seen = self.seen_revision.clone();
            let revision = self.revision;
            div()
                .id("click")
                .w(px(100.))
                .h(px(40.))
                .bg(rgb(0x225599))
                .child(format!("{value}/{revision}"))
                .on_click(move |_, _, cx| {
                    seen.set(revision);
                    count.update(cx, |count, cx| {
                        *count += 1;
                        cx.notify();
                    });
                })
        }
    }

    struct ComponentHost {
        state: Rc<std::cell::RefCell<Option<Entity<usize>>>>,
        seen_revision: Rc<Cell<usize>>,
        revision: usize,
        show: bool,
        renders: usize,
    }

    impl Render for ComponentHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders += 1;
            div().when(self.show, |element| {
                element.child(super::component(
                    "counter",
                    StatefulComponent {
                        state: self.state.clone(),
                        seen_revision: self.seen_revision.clone(),
                        revision: self.revision,
                    },
                ))
            })
        }
    }

    #[gpui::test]
    fn node_engine_component_state_callbacks_and_unmount(cx: &mut TestAppContext) {
        let state = Rc::new(std::cell::RefCell::new(None));
        let seen = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(200.), px(100.)), |window, _| {
            window.draw_engine = DrawEngine::Node(crate::NodeEngine::new());
            ComponentHost {
                state: state.clone(),
                seen_revision: seen.clone(),
                revision: 1,
                show: true,
                renders: 0,
            }
        });
        cx.run_until_parked();
        let mut visual = crate::VisualTestContext::from_window(window.into(), cx);
        visual.simulate_click(crate::point(px(10.), px(10.)), crate::Modifiers::default());
        assert_eq!(seen.get(), 1);
        let original = state.borrow().clone().expect("component state initialized");
        cx.update(|cx| assert_eq!(*original.read(cx), 1));
        window
            .update(cx, |host, _, cx| {
                host.revision = 9;
                cx.notify();
            })
            .expect("window open");
        cx.run_until_parked();
        visual.simulate_click(crate::point(px(10.), px(10.)), crate::Modifiers::default());
        assert_eq!(seen.get(), 9, "retained handlers must receive fresh inputs");
        cx.update(|cx| assert_eq!(*original.read(cx), 2));
        window
            .update(cx, |host, _, cx| {
                host.show = false;
                cx.notify();
            })
            .expect("window open");
        cx.run_until_parked();
        let renders = window
            .update(cx, |host, _, _| host.renders)
            .expect("window open");
        original.update(cx, |value, cx| {
            *value += 1;
            cx.notify();
        });
        cx.run_until_parked();
        window
            .update(cx, |host, _, _| {
                assert_eq!(host.renders, renders, "unmounted state must unsubscribe")
            })
            .expect("window open");
        window
            .update(cx, |host, _, cx| {
                host.show = true;
                cx.notify();
            })
            .expect("window open");
        cx.run_until_parked();
        let remounted = state.borrow().clone().expect("component state initialized");
        assert_ne!(remounted.entity_id(), original.entity_id());
        cx.update(|cx| assert_eq!(*remounted.read(cx), 0));
    }

    struct NotifyDuringRender {
        renders: Rc<Cell<usize>>,
    }
    impl Render for NotifyDuringRender {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            assert!(
                self.renders.get() <= 3,
                "render notifications must not schedule an endless frame loop"
            );
            cx.notify();
            div().size_full().bg(rgb(0x225599))
        }
    }

    #[gpui::test]
    fn node_engine_defers_render_notifications_until_next_requested_frame(cx: &mut TestAppContext) {
        for engine in [
            DrawEngine::Legacy,
            DrawEngine::Node(crate::NodeEngine::new()),
        ] {
            let renders = Rc::new(Cell::new(0));
            let window = cx.open_window(size(px(100.), px(100.)), |window, _| {
                window.draw_engine = engine;
                NotifyDuringRender {
                    renders: renders.clone(),
                }
            });
            cx.run_until_parked();
            assert_eq!(renders.get(), 1);
            window
                .update(cx, |_, _, cx| cx.notify())
                .expect("window open");
            cx.run_until_parked();
            assert_eq!(renders.get(), 2);
        }
    }
    struct NestedRoot {
        prefix: Entity<IntrinsicLeaf>,
        branch: Entity<IntrinsicRoot>,
    }

    impl Render for NestedRoot {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .child(self.prefix.clone())
                .child(self.branch.clone())
        }
    }

    #[gpui::test]
    fn node_engine_nested_reuse_keeps_layout_storage_bounded(cx: &mut TestAppContext) {
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<NestedRoot>| {
                window.draw_engine = engine;
                NestedRoot {
                    prefix: cx.new(|_| IntrinsicLeaf {
                        renders: Rc::new(Cell::new(0)),
                        width: 80.,
                        color: 0x112233,
                    }),
                    branch: cx.new(|cx| IntrinsicRoot {
                        leaves: (0..3)
                            .map(|_| {
                                cx.new(|_| IntrinsicLeaf {
                                    renders: Rc::new(Cell::new(0)),
                                    width: 80.,
                                    color: 0x225599,
                                })
                            })
                            .collect(),
                        show_first: true,
                        reverse: false,
                        opacity: 1.,
                    }),
                }
            }
        };
        let legacy = cx.open_window(size(px(400.), px(200.)), build(DrawEngine::Legacy));
        let retained = cx.open_window(
            size(px(400.), px(200.)),
            build(DrawEngine::Node(crate::NodeEngine::new())),
        );
        cx.run_until_parked();
        let baseline = retained
            .update(cx, |_, window, _| {
                window
                    .retained_node_stats()
                    .expect("retained engine")
                    .layout_nodes
            })
            .expect("window open");
        for step in 0..30 {
            for window in [legacy, retained] {
                window
                    .update(cx, |root, _, cx| {
                        if step % 3 == 2 {
                            let leaf = root
                                .branch
                                .read(cx)
                                .leaves
                                .first()
                                .expect("first leaf")
                                .clone();
                            leaf.update(cx, |leaf, cx| {
                                leaf.color ^= 0xffffff;
                                cx.notify();
                            });
                        } else {
                            root.prefix.update(cx, |leaf, cx| {
                                leaf.color ^= 0xffffff;
                                cx.notify();
                            });
                        }
                    })
                    .expect("window open");
            }
            cx.run_until_parked();
            let snapshot = |window: crate::WindowHandle<NestedRoot>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, _| {
                        window.rendered_frame.scene.snapshot_for_test()
                    })
                    .expect("window open")
            };
            assert_eq!(
                snapshot(legacy, cx),
                snapshot(retained, cx),
                "nested frame {step}"
            );
            retained
                .update(cx, |_, window, _| {
                    let stats = window.retained_node_stats().expect("retained engine");
                    assert_eq!(
                        stats.layout_nodes, baseline,
                        "obsolete layout trees must be collected"
                    );
                    assert_eq!(stats.live_nodes, 6);
                    assert!(stats.reused_subtrees > 0);
                })
                .expect("window open");
        }
    }

    #[gpui::test]
    fn node_engine_dependency_changes_dirty_views_without_notifying_them(cx: &mut TestAppContext) {
        let dependency = cx.new(|_| Dependency);
        let renders = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(100.), px(100.)), |window, _| {
            window.draw_engine = DrawEngine::Node(crate::NodeEngine::new());
            CountingLeaf {
                render_count: renders.clone(),
                dependency: Some(dependency.clone()),
                color: 0x225599,
            }
        });
        cx.run_until_parked();
        let view = window
            .update(cx, |_, _, cx| cx.entity())
            .expect("window open");
        let observer_calls = Rc::new(Cell::new(0));
        let _subscription = cx.update({
            let observer_calls = observer_calls.clone();
            |cx| {
                cx.observe(&view, move |_, _| {
                    observer_calls.set(observer_calls.get() + 1)
                })
            }
        });

        dependency.update(cx, |_, cx| cx.notify());
        cx.run_until_parked();
        assert_eq!(
            renders.get(),
            2,
            "a changed dependency must rebuild the view's output"
        );
        assert_eq!(
            observer_calls.get(),
            0,
            "reading an entity during render must not make the view's observers run when it changes"
        );

        view.update(cx, |_, cx| cx.notify());
        cx.run_until_parked();
        assert_eq!(renders.get(), 3);
        assert_eq!(
            observer_calls.get(),
            1,
            "notifying the view itself still reaches its observers"
        );
    }

    #[gpui::test]
    fn node_engine_replaces_dependencies_after_a_render(cx: &mut TestAppContext) {
        let dependency = cx.new(|_| Dependency);
        let renders = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(100.), px(100.)), |window, _| {
            window.draw_engine = DrawEngine::Node(crate::NodeEngine::new());
            CountingLeaf {
                render_count: renders.clone(),
                dependency: Some(dependency.clone()),
                color: 0x225599,
            }
        });
        cx.run_until_parked();
        window
            .update(cx, |leaf, _, cx| {
                leaf.dependency = None;
                cx.notify();
            })
            .expect("window open");
        cx.run_until_parked();
        let before = renders.get();
        dependency.update(cx, |_, cx| cx.notify());
        cx.run_until_parked();
        assert_eq!(
            renders.get(),
            before,
            "old dependencies must stop waking the window"
        );
    }
}
