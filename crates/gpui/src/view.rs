use crate::{
    AnyElement, AnyEntity, AnyWeakEntity, App, Bounds, Context, Element, ElementId, Entity,
    EntityId, GlobalElementId, InspectorElementId, IntoElement, LayoutId, Pixels, Render,
    RenderOnce, Style, StyleRefinement, ViewNodeCacheKey, ViewNodeId, WeakEntity,
};
use crate::{AppContext as _, Empty, Window};
use anyhow::Result;
use collections::FxHashSet;
use refineable::Refineable;
use std::{any::TypeId, fmt};

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
    fn element_id(&self) -> Option<ElementId> {
        Some(ElementId::View(self.entity.entity_id()))
    }

    fn entity(
        &mut self,
        _: &mut Option<AnyEntity>,
        _: &mut Window,
        _: &mut App,
    ) -> Option<EntityId> {
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

/// What [`ViewElement`] draws: the one shape behind [`Render`], [`Component`], and
/// [`RenderOnce`], so the node and inline element code exists once. Sealed; the public
/// ways in are those three traits.
///
/// A view with an [`element_id`](View::element_id) is mounted as a node: its output is
/// reused until the entity backing it is notified, and `cx.notify()` on that entity
/// re-renders only this view's subtree. A view without one renders inline as part of
/// its parent, in an element-id scope of its own (its type and its order among inline
/// views of that type in the enclosing node) so its internal `use_state` / `.id(..)`
/// never collide with its siblings'.
#[doc(hidden)]
pub trait View: 'static + Sized + sealed::View {
    /// Identifies where this view mounts as a node. Two node views with the same id must
    /// not be rendered at the same position in the element tree (e.g. as siblings under the
    /// same parent); nesting is fine, since the id is scoped by the parent path. `None`
    /// renders inline, without a node.
    fn element_id(&self) -> Option<ElementId>;

    /// The entity backing this view's node, given the entity the node created on a
    /// previous mount (`owned`), which the view may replace. Called before
    /// [`render`](View::render) for views with an element id.
    fn entity(
        &mut self,
        owned: &mut Option<AnyEntity>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<EntityId>;

    /// Render this view into an element tree, consuming `self`.
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement;
}

mod sealed {
    pub trait View {}
    impl<T: crate::RenderOnce> View for T {}
    impl<T: crate::Render> View for crate::Entity<T> {}
    impl View for crate::AnyView {}
    impl<C: crate::Component> View for super::ComponentView<C> {}
}

/// A stateless component (`RenderOnce`) is a `View` with no identity.
impl<T: RenderOnce> View for T {
    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn entity(
        &mut self,
        _: &mut Option<AnyEntity>,
        _: &mut Window,
        _: &mut App,
    ) -> Option<EntityId> {
        None
    }

    #[inline]
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        RenderOnce::render(self, window, cx)
    }
}

/// An entity that renders itself (`Render`) is a `View` keyed on its own id.
impl<T: Render> View for Entity<T> {
    fn element_id(&self) -> Option<ElementId> {
        Some(ElementId::View(Entity::entity_id(self)))
    }

    fn entity(
        &mut self,
        _: &mut Option<AnyEntity>,
        _: &mut Window,
        _: &mut App,
    ) -> Option<EntityId> {
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
        ViewElement::new(self).cached(style)
    }
}

/// The element type for [`View`] implementations. Wraps a `View` and hooks it
/// into layout, prepaint, and paint. Constructed via [`ViewElement::new`].
#[doc(hidden)]
pub struct ViewElement<V: View> {
    view: Option<V>,
    element_id: Option<ElementId>,
    /// The entity backing the node, resolved at layout.
    entity_id: Option<EntityId>,
    /// This view's order among inline views of its type in the enclosing node, assigned
    /// when it renders inline.
    inline_occurrence: u64,
    cached_style: Option<StyleRefinement>,
    node_layout: Option<NodeViewLayout>,
    #[cfg(debug_assertions)]
    source: &'static core::panic::Location<'static>,
}

impl<V: View> ViewElement<V> {
    /// Wrap a [`View`] as an element.
    #[track_caller]
    pub fn new(view: V) -> Self {
        let element_id = view.element_id();
        ViewElement {
            element_id,
            entity_id: None,
            inline_occurrence: 0,
            cached_style: None,
            node_layout: None,
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

impl<V: View> ViewElement<V> {
    /// Renders the view as part of its parent, in an element-id scope of its type and its
    /// order among inline views of that type, so its internal ids do not collide with its
    /// siblings'.
    fn render_inline(
        &mut self,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Option<AnyElement>) {
        let view = self.view.take().expect("view is rendered once per frame");
        self.inline_occurrence = window
            .node_engine
            .next_inline_occurrence(std::any::type_name::<V>());
        self.with_inline_scope(window, |window| {
            let mut element = view.render(window, cx).into_any_element();
            let layout_id = element.request_layout(window, cx);
            (layout_id, Some(element))
        })
    }

    fn with_inline_scope<R>(&self, window: &mut Window, f: impl FnOnce(&mut Window) -> R) -> R {
        window.with_id(
            ElementId::NamedInteger(std::any::type_name::<V>().into(), self.inline_occurrence),
            f,
        )
    }
}

/// Carried from `request_layout` to `prepaint` for a view mounted as a node.
struct NodeViewLayout {
    layout: LayoutId,
    node_id: ViewNodeId,
    /// Whether the layout was reused rather than rendered.
    grafted: bool,
    /// Entities read while rendering at layout time. Empty when layout was grafted, since
    /// the node's stored dependencies already cover it.
    accessed_entities: FxHashSet<EntityId>,
}

#[doc(hidden)]
pub struct ViewElementPrepaintState {
    element: Option<AnyElement>,
    node: Option<ViewNodePrepaintState>,
}

enum ViewNodePrepaintState {
    Graft {
        node_id: ViewNodeId,
    },
    Render {
        node_id: ViewNodeId,
        cache_key: ViewNodeCacheKey,
        accessed_entities: FxHashSet<EntityId>,
    },
}

impl<V: View> Element for ViewElement<V> {
    type RequestLayoutState = Option<AnyElement>;
    type PrepaintState = ViewElementPrepaintState;

    fn id(&self) -> Option<ElementId> {
        self.element_id.clone()
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
        if let Some(id) = id
            && let Some(view) = self.view.as_mut()
        {
            let cache_key = window.view_node_key(Bounds::default());
            let node_id = window.begin_node_occurrence(id.clone(), &cache_key);
            let mut owned = window.node_engine.take_owned_entity(node_id);
            let entity_id = view.entity(&mut owned, window, cx);
            window.node_engine.store_owned_entity(node_id, owned);
            let Some(entity_id) = entity_id else {
                // A view with an id but no entity cannot be reused, since nothing could
                // notify it; it renders inline like a stateless one.
                window.finish_node_phase(node_id, false);
                window.node_engine.abandon_occurrence(node_id);
                return self.render_inline(window, cx);
            };
            self.entity_id = Some(entity_id);
            window.node_engine.set_view_id(node_id, entity_id);
            let (layout, element) =
                if let Some(layout) = window.node_engine.reuse_layout(node_id, &cache_key) {
                    cx.entities
                        .extend_accessed(&window.node_engine.node(node_id).accessed_entities);
                    window.finish_node_phase(node_id, false);
                    self.node_layout = Some(NodeViewLayout {
                        layout,
                        node_id,
                        grafted: true,
                        accessed_entities: window.node_engine.take_dependency_set(),
                    });
                    (layout, None)
                } else {
                    window.restart_node_render(node_id);
                    let mut accessed_entities = window.node_engine.take_dependency_set();
                    let view = self.view.take().expect("view is rendered once per frame");
                    let (layout, element) = cx.track_reads(&mut accessed_entities, |cx| {
                        window.with_rendered_view(entity_id, |window| {
                            let mut element = view.render(window, cx).into_any_element();
                            let layout = element.request_layout(window, cx);
                            (layout, element)
                        })
                    });
                    window.node_engine.store_layout(node_id, layout);
                    window.finish_node_phase(node_id, true);
                    self.node_layout = Some(NodeViewLayout {
                        layout,
                        node_id,
                        grafted: false,
                        accessed_entities,
                    });
                    (layout, Some(element))
                };
            // `.cached(style)` predates node memoization; the style it supplies now simply
            // becomes the box the view is laid out in.
            let layout = match &self.cached_style {
                Some(style) => {
                    let mut root_style = Style::default();
                    root_style.refine(style);
                    window.request_layout(root_style, [layout], cx)
                }
                None => layout,
            };
            return (layout, element);
        }
        self.render_inline(window, cx)
    }

    fn prepaint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> ViewElementPrepaintState {
        if let Some(node_layout) = self.node_layout.take() {
            let NodeViewLayout {
                layout,
                node_id,
                grafted,
                accessed_entities,
            } = node_layout;
            // Ambient inputs such as content masks and image caches are pushed during
            // prepaint, so the key is built here rather than carried over from layout.
            let cache_key = window.view_node_key(bounds);
            let entity_id = self.entity_id.expect("node views have an entity");
            window.set_view_id(entity_id);
            window.enter_node_prepaint(node_id);
            return window.with_rendered_view(entity_id, |window| {
                let mut accessed_entities = accessed_entities;
                if grafted {
                    if window
                        .node_engine
                        .node(node_id)
                        .cache_key
                        .matches(&cache_key, false)
                        && window.retained_layout_unchanged(layout)
                    {
                        window.graft_view_node_prepaint(node_id);
                        window.node_engine.recycle_dependency_set(accessed_entities);
                        window.finish_node_phase(node_id, false);
                        return ViewElementPrepaintState {
                            element: None,
                            node: Some(ViewNodePrepaintState::Graft { node_id }),
                        };
                    }
                    // The grafted layout is being replaced, so the reused recording and the
                    // dependencies it implied no longer describe this node.
                    window.restart_node_render(node_id);
                    accessed_entities.clear();
                }
                let element = cx.track_reads(&mut accessed_entities, |cx| {
                    if let Some(mut element) = element.take() {
                        element.prepaint(window, cx);
                        element
                    } else {
                        // Layout was grafted, so the view has not rendered this frame.
                        let view = self.view.take().expect("view is rendered once per frame");
                        let mut element = view.render(window, cx).into_any_element();
                        let new_layout = element.request_layout(window, cx);
                        window.replace_retained_layout(layout, new_layout, cx);
                        window.node_engine.store_layout(node_id, new_layout);
                        element.prepaint(window, cx);
                        element
                    }
                });
                window.finish_node_phase(node_id, true);
                ViewElementPrepaintState {
                    element: Some(element),
                    node: Some(ViewNodePrepaintState::Render {
                        node_id,
                        cache_key,
                        accessed_entities,
                    }),
                }
            });
        }
        self.with_inline_scope(window, |window| {
            if let Some(element) = element.as_mut() {
                element.prepaint(window, cx);
            }
        });
        ViewElementPrepaintState {
            element: element.take(),
            node: None,
        }
    }

    fn paint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
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
            let rendered = matches!(node, ViewNodePrepaintState::Render { .. });
            window.enter_node_paint(node_id);
            if let Some(entity_id) = self.entity_id {
                window.with_rendered_view(entity_id, |window| match node {
                    ViewNodePrepaintState::Graft { node_id } => {
                        window.graft_view_node_paint(node_id);
                        window.node_engine.store_graft();
                    }
                    ViewNodePrepaintState::Render {
                        node_id,
                        cache_key,
                        mut accessed_entities,
                    } => {
                        window.begin_view_node_paint(node_id);
                        if let Some(element) = element.element.as_mut() {
                            cx.track_reads(&mut accessed_entities, |cx| element.paint(window, cx));
                        }
                        window.finish_view_node_paint(node_id);
                        window
                            .node_engine
                            .store_render(node_id, cache_key, accessed_entities);
                    }
                });
            }
            window.finish_node_phase(node_id, rendered);
            return;
        }

        self.with_inline_scope(window, |window| {
            if let Some(element) = element.element.as_mut() {
                element.paint(window, cx);
            }
        });
    }
}

/// A component: a value built by its parent's render that renders itself from `&self`.
/// Implementing it is enough to use the value as an element (`Input::new("…")` as a
/// child). Components are owned by the framework rather than referred to by the program,
/// which is what makes them cheap to write: there is no handle to manage.
///
/// By default a component renders inline, as part of the node that rendered it. It still
/// gets its own element-id scope, so `use_state` inside it does not collide with a
/// sibling of the same type. A component that implements `PartialEq` can instead be
/// [`cached`](Component::cached): it is then mounted as a node of its own, and when the
/// parent renders again it is only re-rendered if the new value differs from the one it
/// last rendered (or something it read was notified).
///
/// This is what [`RenderOnce`] wanted to be; `RenderOnce` consumes `self`, so a
/// `RenderOnce` value can never be rendered again and cannot be cached.
pub trait Component: 'static {
    /// Builds the component's elements from its current inputs and local state.
    fn render(&self, window: &mut Window, cx: &mut App) -> impl IntoElement;

    /// Mounts this component as a node of its own, identified by its type and its order
    /// among cached siblings of that type. Its output is reused across frames while the
    /// values the parent supplies compare equal.
    ///
    /// Callbacks cannot be compared, so a component with callbacks implements `PartialEq`
    /// by hand over its other fields. That is sound as long as the callbacks capture
    /// handles (entities, focus handles) and read state when they run, rather than values
    /// computed by the parent's render, which would go stale.
    fn cached(self) -> Cached<Self>
    where
        Self: Sized + PartialEq,
    {
        Cached(self)
    }
}

impl<C: Component> IntoElement for C {
    type Element = ViewElement<ComponentView<C>>;

    #[track_caller]
    fn into_element(self) -> Self::Element {
        ViewElement::new(ComponentView {
            value: Some(self),
            instance: None,
            cached: None,
        })
    }
}

/// A [`Component`] to be mounted as a node; see [`Component::cached`].
pub struct Cached<C: Component + PartialEq>(C);

impl<C: Component + PartialEq> IntoElement for Cached<C> {
    type Element = ViewElement<ComponentView<C>>;

    #[track_caller]
    fn into_element(self) -> Self::Element {
        ViewElement::new(ComponentView {
            value: Some(self.0),
            instance: None,
            cached: Some(C::eq),
        })
    }
}

/// The [`View`] of a [`Component`]. Inline unless `cached`, in which case it mounts as a
/// node backed by an entity holding the value.
#[doc(hidden)]
pub struct ComponentView<C: Component> {
    value: Option<C>,
    instance: Option<Entity<ComponentInstance<C>>>,
    /// Compares the value last rendered with the one the parent supplied now, for a
    /// cached component.
    cached: Option<fn(&C, &C) -> bool>,
}

struct ComponentInstance<C: Component> {
    value: C,
}

impl<C: Component> View for ComponentView<C> {
    fn element_id(&self) -> Option<ElementId> {
        self.cached
            .is_some()
            .then(|| ElementId::Name(std::any::type_name::<C>().into()))
    }

    fn entity(
        &mut self,
        owned: &mut Option<AnyEntity>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<EntityId> {
        let value = self.value.take()?;
        let unchanged = self.cached?;
        let instance = match owned
            .take()
            .and_then(|entity| entity.downcast::<ComponentInstance<C>>().ok())
        {
            Some(instance) => {
                if !unchanged(&instance.read(cx).value, &value) {
                    instance.update(cx, |instance, _| instance.value = value);
                    window.invalidate_component(instance.entity_id());
                }
                instance
            }
            None => cx.new(|_| ComponentInstance { value }),
        };
        *owned = Some(instance.clone().into_any());
        let id = instance.entity_id();
        self.instance = Some(instance);
        Some(id)
    }

    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        match (self.instance, self.value) {
            (Some(instance), _) => instance.update(cx, |instance, cx| {
                instance.value.render(window, cx).into_any_element()
            }),
            (None, Some(value)) => value.render(window, cx).into_any_element(),
            (None, None) => Empty.into_any_element(),
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

#[cfg(test)]
mod tests {
    #[gpui::test]
    fn global_reads_outside_rendering_do_not_retain_dependency_entities(cx: &mut TestAppContext) {
        struct Value;
        impl crate::Global for Value {}
        cx.update(|cx| {
            let snapshot = cx.leak_detector_snapshot();
            assert!(!cx.has_global::<Value>());
            assert!(cx.try_global::<Value>().is_none());
            cx.set_global(Value);
            cx.global::<Value>();
            cx.assert_no_new_leaks(&snapshot);
        });
    }

    #[gpui::test]
    fn global_changes_only_invalidate_reading_scopes(cx: &mut TestAppContext) {
        struct LeftColor(u32);
        impl crate::Global for LeftColor {}
        struct RightColor(u32);
        impl crate::Global for RightColor {}
        struct UnusedGlobal;
        impl crate::Global for UnusedGlobal {}
        struct Leaf {
            left: bool,
            renders: Rc<Cell<usize>>,
        }
        impl Render for Leaf {
            fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
                self.renders.set(self.renders.get() + 1);
                let color = if self.left {
                    cx.try_global::<LeftColor>().map_or(0, |color| color.0)
                } else {
                    cx.try_global::<RightColor>().map_or(0, |color| color.0)
                };
                div().size(px(40.)).bg(rgb(color))
            }
        }
        struct Host(Vec<Entity<Leaf>>);
        impl Render for Host {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div().children(self.0.iter().cloned())
            }
        }
        let left = Rc::new(Cell::new(0));
        let right = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(100.), px(100.)), |_, cx| {
            Host(vec![
                cx.new(|_| Leaf {
                    left: true,
                    renders: left.clone(),
                }),
                cx.new(|_| Leaf {
                    left: false,
                    renders: right.clone(),
                }),
            ])
        });
        cx.run_until_parked();
        assert_eq!((left.get(), right.get()), (1, 1));
        let draw = |cx: &mut TestAppContext| {
            window
                .update(cx, |_, _, cx| cx.notify())
                .expect("window open");
            cx.run_until_parked();
        };
        cx.update(|cx| cx.set_global(LeftColor(0xff0000)));
        cx.run_until_parked();
        assert_eq!(
            (left.get(), right.get()),
            (1, 1),
            "global writes preserve frame demand"
        );
        draw(cx);
        assert_eq!((left.get(), right.get()), (2, 1));
        cx.update(|cx| cx.global_mut::<LeftColor>().0 = 0x00ff00);
        draw(cx);
        assert_eq!((left.get(), right.get()), (3, 1));
        cx.update(|cx| {
            cx.remove_global::<LeftColor>();
        });
        draw(cx);
        assert_eq!((left.get(), right.get()), (4, 1));
        cx.update(|cx| cx.set_global(RightColor(0x0000ff)));
        draw(cx);
        assert_eq!((left.get(), right.get()), (4, 2));
        cx.update(|cx| cx.set_global(UnusedGlobal));
        draw(cx);
        assert_eq!((left.get(), right.get()), (4, 2));
    }

    #[gpui::test]
    fn shared_image_completion_invalidates_every_consumer(cx: &mut TestAppContext) {
        use futures::FutureExt as _;
        let mut encoded = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            2,
            2,
            image::Rgba([255, 0, 0, 255]),
        ))
        .write_to(&mut encoded, image::ImageFormat::Png)
        .expect("encode image");
        let (complete, pending) = futures::channel::oneshot::channel::<()>();
        let pending = pending.shared();
        let bytes = encoded.into_inner();
        cx.update(|cx| {
            cx.set_http_client(http_client::FakeHttpClient::create(move |_| {
                let pending = pending.clone();
                let bytes = bytes.clone();
                async move {
                    pending.await?;
                    Ok(http_client::Response::builder()
                        .status(200)
                        .body(bytes.into())?)
                }
            }))
        });
        struct ImageView;
        impl Render for ImageView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                crate::img("https://test.example/shared.png")
                    .w(px(40.))
                    .h(px(40.))
            }
        }
        struct Host(Vec<Entity<ImageView>>, bool);
        impl Render for Host {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let children = div().flex().children(self.0.iter().cloned());
                if self.1 {
                    crate::image_cache(crate::retain_all("images"))
                        .child(children)
                        .into_any_element()
                } else {
                    children.into_any_element()
                }
            }
        }
        struct PassiveView(Rc<Cell<usize>>);
        impl Render for PassiveView {
            fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
                self.0.set(self.0.get() + 1);
                let _image = window.get_asset::<crate::ImgResourceLoader>(
                    &crate::Resource::Uri("https://test.example/shared.png".into()),
                    cx,
                );
                div()
            }
        }
        let passive_renders = Rc::new(Cell::new(0));
        let _passive = cx.open_window(size(px(200.), px(100.)), |_, _| {
            PassiveView(passive_renders.clone())
        });
        let handles = [false, false, true].map(|cached| {
            cx.open_window(size(px(200.), px(100.)), |_, cx| {
                Host(vec![cx.new(|_| ImageView), cx.new(|_| ImageView)], cached)
            })
        });
        cx.run_until_parked();
        for handle in handles {
            handle
                .update(cx, |_, window, _| {
                    assert!(window.rendered_frame.scene.polychrome_sprites.is_empty())
                })
                .expect("window open");
        }
        complete.send(()).expect("image request pending");
        cx.run_until_parked();
        for handle in handles {
            handle
                .update(cx, |_, window, cx| {
                    window.simulate_next_frame(cx);
                })
                .expect("window open");
        }
        cx.run_until_parked();
        for handle in handles {
            handle
                .update(cx, |host, window, _| {
                    assert_eq!(
                        window.rendered_frame.scene.polychrome_sprites.len(),
                        2,
                        "image cache: {}",
                        host.1
                    )
                })
                .expect("window open");
        }
        assert_eq!(
            passive_renders.get(),
            1,
            "get_asset must not subscribe to completion"
        );
    }

    use crate::{
        Component, Context, Entity, Render, StyleRefinement, TestAppContext, Window, div,
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
        for memoized in [false, true] {
            for cached in [false, true] {
                let window = cx.open_window(size(px(300.), px(100.)), |window, cx| {
                    window.node_engine = if memoized {
                        crate::NodeEngine::new()
                    } else {
                        crate::NodeEngine::new_eager()
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
                            if memoized && step == 0 {
                                assert!(window.node_stats().reused_subtrees > 0);
                            }
                            window.all_debug_bounds()
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
                        .update(cx, |_, window, _| window.all_debug_bounds())
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

        let memoized_scene = window
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

        assert_eq!(memoized_scene, cold_scene);
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
                .child("Memoized text")
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
    fn node_engine_automatic_views_match_eager_across_layout_and_mount_changes(
        cx: &mut TestAppContext,
    ) {
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<IntrinsicRoot>| {
                window.node_engine = engine;
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
        let eager = cx.open_window(
            size(px(400.), px(100.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(400.), px(100.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        for step in 0..10 {
            for window in [eager, memoized] {
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
                            // A parent notify alone does not rebuild a child: the leaf must
                            // notify for its own change.
                            root.leaves
                                .first()
                                .expect("first leaf")
                                .update(cx, |leaf, cx| {
                                    leaf.color = 0x9900ff;
                                    cx.notify();
                                });
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
                snapshot(eager, cx),
                snapshot(memoized, cx),
                "frame after step {step}"
            );
            if step == 1 {
                memoized
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
                        window.node_engine = engine;
                        PercentageHost {
                            width: 200.,
                            layout_mode,
                            leaf: cx.new(|_| PercentageLeaf { relative_width }),
                        }
                    }
                };
                let eager = cx.open_window(
                    size(px(400.), px(200.)),
                    build(crate::NodeEngine::new_eager()),
                );
                let memoized =
                    cx.open_window(size(px(400.), px(200.)), build(crate::NodeEngine::new()));
                for width in [200., 300., 160., 320., 320.] {
                    for window in [eager, memoized] {
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
                        snapshot(eager, cx),
                        snapshot(memoized, cx),
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
                window.node_engine = engine;
                let leaf = cx.new(|cx| InteractiveLeaf {
                    focus: cx.focus_handle(),
                    clicks: 0,
                    keys: 0,
                });
                leaf.read(cx).focus.clone().focus(window, cx);
                InteractiveHost { offset: 0., leaf }
            }
        };
        let eager = cx.open_window(
            size(px(300.), px(100.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(300.), px(100.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        for step in 0..8 {
            for window in [eager, memoized] {
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
                snapshot(eager, cx),
                snapshot(memoized, cx),
                "interaction {step}"
            );
        }
        memoized
            .update(cx, |host, _, cx| {
                let leaf = host.leaf.read(cx);
                assert_eq!(leaf.clicks, 1, "old geometry must not retain a hit target");
                assert_eq!(leaf.keys, 2, "focus and key listeners must survive reuse");
            })
            .expect("window open");
    }

    struct MetadataLeaf {
        focus: crate::FocusHandle,
        events: Rc<std::cell::RefCell<Vec<&'static str>>>,
    }

    impl Render for MetadataLeaf {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("leaf")
                .track_focus(&self.focus)
                .key_context("MetadataLeaf")
                .w(px(100.))
                .h(px(40.))
                .child("memoized text")
                .on_key_down(cx.listener(|this, _, _, _| this.events.borrow_mut().push("leaf")))
        }
    }

    #[gpui::test]
    fn repeated_view_mounts_keep_separate_recordings(cx: &mut TestAppContext) {
        struct Repeated {
            leaf: Entity<MetadataLeaf>,
            count: usize,
        }
        impl Render for Repeated {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .flex()
                    .children((0..self.count).map(|_| div().child(self.leaf.clone())))
            }
        }
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<Repeated>| {
                window.node_engine = engine;
                Repeated {
                    leaf: cx.new(|cx| MetadataLeaf {
                        focus: cx.focus_handle(),
                        events: Rc::default(),
                    }),
                    count: 2,
                }
            }
        };
        let eager = cx.open_window(
            size(px(400.), px(100.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(400.), px(100.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        let mut reused = 0;
        for count in [2, 2, 1, 2, 3, 1, 0, 2] {
            for window in [eager, memoized] {
                window
                    .update(cx, |root, _, cx| {
                        root.count = count;
                        cx.notify();
                    })
                    .expect("window open");
            }
            cx.run_until_parked();
            let mut snapshot = |handle: crate::WindowHandle<Repeated>, cx: &mut TestAppContext| {
                handle
                    .update(cx, |_, window, _| {
                        reused += window.node_stats().reused_subtrees;
                        (
                            window.rendered_frame.scene.snapshot_for_test(),
                            window
                                .tab_stops(crate::node_engine::FrameOutput::Rendered)
                                .operation_count(),
                        )
                    })
                    .expect("window open")
            };
            assert_eq!(snapshot(eager, cx), snapshot(memoized, cx));
        }
        assert!(reused > 0);
    }

    struct MetadataBranch {
        focus: crate::FocusHandle,
        leaf: Entity<MetadataLeaf>,
        events: Rc<std::cell::RefCell<Vec<&'static str>>>,
    }

    impl Render for MetadataBranch {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("branch")
                .track_focus(&self.focus)
                .key_context("MetadataBranch")
                .child(self.leaf.clone())
                .on_key_down(cx.listener(|this, _, _, _| this.events.borrow_mut().push("branch")))
        }
    }

    struct MetadataRoot {
        prefix_count: usize,
        branch: Entity<MetadataBranch>,
        events: Rc<std::cell::RefCell<Vec<&'static str>>>,
    }

    impl Render for MetadataRoot {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("root")
                .key_context("MetadataRoot")
                .children(
                    (0..self.prefix_count).map(|index| div().id(index).absolute().child("prefix")),
                )
                .child(self.branch.clone())
                .on_key_down(cx.listener(|this, _, _, _| this.events.borrow_mut().push("root")))
        }
    }

    #[gpui::test]
    fn node_engine_replays_nested_metadata_from_different_frames(cx: &mut TestAppContext) {
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<MetadataRoot>| {
                window.node_engine = engine;
                let events = Rc::new(std::cell::RefCell::new(Vec::new()));
                let leaf = cx.new(|cx| MetadataLeaf {
                    focus: cx.focus_handle(),
                    events: events.clone(),
                });
                leaf.read(cx).focus.clone().focus(window, cx);
                let branch = cx.new(|cx| MetadataBranch {
                    focus: cx.focus_handle(),
                    leaf,
                    events: events.clone(),
                });
                MetadataRoot {
                    prefix_count: 0,
                    branch,
                    events,
                }
            }
        };
        let eager = cx.open_window(
            size(px(300.), px(100.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(300.), px(100.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        for step in 0..12 {
            for handle in [eager, memoized] {
                handle
                    .update(cx, |root, _, cx| {
                        root.events.borrow_mut().clear();
                        if step % 3 == 1 {
                            root.branch.update(cx, |_, cx| cx.notify());
                        } else {
                            root.prefix_count = (step * 7) % 5;
                            cx.notify();
                        }
                    })
                    .expect("window open");
            }
            cx.run_until_parked();
            for handle in [eager, memoized] {
                crate::VisualTestContext::from_window(handle.into(), cx)
                    .simulate_keystrokes("enter");
            }
            cx.run_until_parked();
            let snapshot = |handle: crate::WindowHandle<MetadataRoot>, cx: &mut TestAppContext| {
                handle
                    .update(cx, |root, window, cx| {
                        let branch = root.branch.read(cx);
                        assert!(branch.focus.contains_focused(window, cx));
                        assert!(branch.leaf.read(cx).focus.is_focused(window));
                        assert_eq!(&*root.events.borrow(), &["leaf", "branch", "root"]);
                        window.rendered_frame.scene.snapshot_for_test()
                    })
                    .expect("window open")
            };
            assert_eq!(
                snapshot(eager, cx),
                snapshot(memoized, cx),
                "metadata frame {step}"
            );
        }
    }

    struct OptionalPaint {
        child: crate::AnyElement,
        paint_child: bool,
    }

    impl IntoElement for OptionalPaint {
        type Element = Self;
        fn into_element(self) -> Self {
            self
        }
    }

    impl crate::Element for OptionalPaint {
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
            cx: &mut crate::App,
        ) -> (crate::LayoutId, ()) {
            (self.child.request_layout(window, cx), ())
        }
        fn prepaint(
            &mut self,
            _: Option<&crate::GlobalElementId>,
            _: Option<&crate::InspectorElementId>,
            _: crate::Bounds<crate::Pixels>,
            _: &mut (),
            window: &mut Window,
            cx: &mut crate::App,
        ) {
            self.child.prepaint(window, cx);
        }
        fn paint(
            &mut self,
            _: Option<&crate::GlobalElementId>,
            _: Option<&crate::InspectorElementId>,
            _: crate::Bounds<crate::Pixels>,
            _: &mut (),
            _: &mut (),
            window: &mut Window,
            cx: &mut crate::App,
        ) {
            if self.paint_child {
                self.child.paint(window, cx);
            }
        }
    }

    struct OptionalPaintRoot {
        leaf: Entity<InteractiveLeaf>,
        paint_child: bool,
    }

    impl Render for OptionalPaintRoot {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().child(OptionalPaint {
                child: self.leaf.clone().into_any_element(),
                paint_child: self.paint_child,
            })
        }
    }

    #[gpui::test]
    fn node_engine_captures_children_that_prepaint_without_paint(cx: &mut TestAppContext) {
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<OptionalPaintRoot>| {
                window.node_engine = engine;
                OptionalPaintRoot {
                    leaf: cx.new(|cx| InteractiveLeaf {
                        focus: cx.focus_handle(),
                        clicks: 0,
                        keys: 0,
                    }),
                    paint_child: false,
                }
            }
        };
        let eager = cx.open_window(
            size(px(300.), px(100.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(300.), px(100.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        for step in 0..8 {
            for handle in [eager, memoized] {
                handle
                    .update(cx, |root, _, cx| {
                        root.paint_child = step % 2 == 0;
                        cx.notify();
                    })
                    .expect("window open");
            }
            cx.run_until_parked();
            let snapshot = |handle: crate::WindowHandle<OptionalPaintRoot>,
                            cx: &mut TestAppContext| {
                handle
                    .update(cx, |_, window, _| {
                        window.rendered_frame.scene.snapshot_for_test()
                    })
                    .expect("window open")
            };
            assert_eq!(
                snapshot(eager, cx),
                snapshot(memoized, cx),
                "optional paint {step}"
            );
        }
    }

    struct AmbientStyle(u32);
    impl crate::Global for AmbientStyle {}

    struct AmbientLeaf {
        image: std::sync::Arc<crate::RenderImage>,
    }

    impl Render for AmbientLeaf {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("ambient-leaf")
                .w(px(100.))
                .h(px(60.))
                .bg(rgb(cx
                    .try_global::<AmbientStyle>()
                    .map_or(0x112233, |style| style.0)))
                .child(format!("active {}", window.is_window_active()))
                .child(crate::img(self.image.clone()).size(px(20.)))
        }
    }

    struct AmbientHost {
        leaf: Entity<AmbientLeaf>,
        deferred: bool,
    }

    impl Render for AmbientHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().child(if self.deferred {
                crate::deferred(self.leaf.clone()).into_any_element()
            } else {
                self.leaf.clone().into_any_element()
            })
        }
    }

    #[gpui::test]
    fn node_engine_invalidates_ambient_inputs_and_evicted_images(cx: &mut TestAppContext) {
        let image = std::sync::Arc::new(crate::RenderImage::new(smallvec::smallvec![
            image::Frame::new(image::ImageBuffer::from_pixel(
                2,
                2,
                image::Rgba([255, 0, 0, 255])
            ))
        ]));
        let build = |engine| {
            let image = image.clone();
            move |window: &mut Window, cx: &mut Context<AmbientHost>| {
                window.node_engine = engine;
                AmbientHost {
                    leaf: cx.new(|_| AmbientLeaf { image }),
                    deferred: false,
                }
            }
        };
        let eager = cx.open_window(
            size(px(300.), px(100.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(300.), px(100.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        for step in 0..9 {
            if step == 1 {
                cx.update(|cx| cx.set_global(AmbientStyle(0x335577)));
            }
            if step == 2 {
                cx.update(|cx| cx.global_mut::<AmbientStyle>().0 = 0x7799bb);
            }
            if step == 3 {
                cx.update(|cx| {
                    cx.remove_global::<AmbientStyle>();
                });
            }
            for handle in [eager, memoized] {
                handle
                    .update(cx, |host, window, cx| {
                        if step == 4 {
                            window.drop_image(image.clone()).expect("evict image");
                        }
                        if step == 5 {
                            window.set_rem_size(px(20.));
                        }
                        host.deferred = step == 6 || step == 7;
                        cx.notify();
                    })
                    .expect("window open");
            }
            cx.run_until_parked();
            let snapshot = |handle: crate::WindowHandle<AmbientHost>, cx: &mut TestAppContext| {
                handle
                    .update(cx, |_, window, _| {
                        assert!(
                            window.has_image_atlas_entry(&image),
                            "evicted image must be uploaded again"
                        );
                        window.rendered_frame.scene.snapshot_for_test()
                    })
                    .expect("window open")
            };
            assert_eq!(
                snapshot(eager, cx),
                snapshot(memoized, cx),
                "ambient update {step}"
            );
        }
    }

    struct OverlapHost {
        front: Option<Entity<InteractiveLeaf>>,
        back: Entity<InteractiveLeaf>,
        reversed: bool,
    }

    impl Render for OverlapHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let mut children = vec![self.back.clone()];
            children.extend(self.front.clone());
            if self.reversed {
                children.reverse();
            }
            div()
                .id("clip")
                .relative()
                .w(px(60.))
                .h(px(30.))
                .overflow_hidden()
                .children(
                    children
                        .into_iter()
                        .map(|child| div().absolute().child(child)),
                )
        }
    }

    #[gpui::test]
    fn node_engine_preserves_clipping_overlap_and_focused_removal(cx: &mut TestAppContext) {
        let build = |engine| {
            move |window: &mut Window, cx: &mut Context<OverlapHost>| {
                window.node_engine = engine;
                let front = cx.new(|cx| InteractiveLeaf {
                    focus: cx.focus_handle(),
                    clicks: 0,
                    keys: 0,
                });
                front.read(cx).focus.clone().focus(window, cx);
                OverlapHost {
                    front: Some(front),
                    back: cx.new(|cx| InteractiveLeaf {
                        focus: cx.focus_handle(),
                        clicks: 0,
                        keys: 0,
                    }),
                    reversed: false,
                }
            }
        };
        let eager = cx.open_window(
            size(px(300.), px(100.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(300.), px(100.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        for step in 0..6 {
            for handle in [eager, memoized] {
                handle
                    .update(cx, |host, _, cx| {
                        if step == 3 {
                            host.reversed = true;
                        }
                        if step == 4 {
                            host.front.take();
                        }
                        cx.notify();
                    })
                    .expect("window open");
            }
            cx.run_until_parked();
            for handle in [eager, memoized] {
                let mut visual = crate::VisualTestContext::from_window(handle.into(), cx);
                visual.simulate_click(
                    crate::point(px(if step == 2 { 70. } else { 10. }), px(10.)),
                    crate::Modifiers::default(),
                );
                visual.simulate_keystrokes("enter");
            }
            cx.run_until_parked();
            let snapshot = |handle: crate::WindowHandle<OverlapHost>, cx: &mut TestAppContext| {
                handle
                    .update(cx, |host, window, cx| {
                        let front = host
                            .front
                            .as_ref()
                            .map(|leaf| (leaf.read(cx).clicks, leaf.read(cx).keys));
                        let back = host.back.read(cx);
                        if step == 2 {
                            assert_eq!(front.map(|counts| counts.0), Some(2));
                        }
                        (
                            window.rendered_frame.scene.snapshot_for_test(),
                            front,
                            back.clicks,
                            back.keys,
                            window.focused(cx).is_some(),
                        )
                    })
                    .expect("window open")
            };
            assert_eq!(
                snapshot(eager, cx),
                snapshot(memoized, cx),
                "overlap step {step}"
            );
        }
    }

    #[gpui::test]
    fn node_engine_caches_components_over_an_entity(cx: &mut TestAppContext) {
        struct Custom {
            source: Entity<usize>,
            renders: Rc<Cell<usize>>,
        }
        impl PartialEq for Custom {
            fn eq(&self, other: &Self) -> bool {
                self.source == other.source
            }
        }
        impl Component for Custom {
            fn render(&self, _: &mut Window, cx: &mut crate::App) -> impl IntoElement {
                self.renders.set(self.renders.get() + 1);
                div().size_full().child(self.source.read(cx).to_string())
            }
        }
        struct Host {
            source: Entity<usize>,
            renders: Rc<Cell<usize>>,
        }
        impl Render for Host {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div().w(px(100.)).h(px(50.)).child(
                    Custom {
                        source: self.source.clone(),
                        renders: self.renders.clone(),
                    }
                    .cached(),
                )
            }
        }
        let renders = Rc::new(Cell::new(0));
        let handle = cx.open_window(size(px(300.), px(100.)), |window, cx| {
            window.node_engine = crate::NodeEngine::new();
            Host {
                source: cx.new(|_| 0),
                renders: renders.clone(),
            }
        });
        cx.run_until_parked();
        for _ in 0..3 {
            handle
                .update(cx, |_, _, cx| cx.notify())
                .expect("window open");
            cx.run_until_parked();
        }
        assert_eq!(renders.get(), 1);
        handle
            .update(cx, |host, _, cx| {
                host.source.update(cx, |value, cx| {
                    *value += 1;
                    cx.notify();
                })
            })
            .expect("window open");
        cx.run_until_parked();
        assert_eq!(renders.get(), 2);
        let memoized = handle
            .update(cx, |_, window, _| {
                let scene = window.rendered_frame.scene.snapshot_for_test();
                window.refresh();
                scene
            })
            .expect("window open");
        cx.run_until_parked();
        handle
            .update(cx, |_, window, _| {
                assert_eq!(memoized, window.rendered_frame.scene.snapshot_for_test());
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
            window.node_engine = crate::NodeEngine::new();
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

    struct SiblingLeaf {
        revision: usize,
    }

    impl Render for SiblingLeaf {
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

    struct SiblingHost {
        leaves: Vec<Entity<SiblingLeaf>>,
    }

    impl Render for SiblingHost {
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
            move |window: &mut Window, cx: &mut Context<SiblingHost>| {
                window.node_engine = engine;
                SiblingHost {
                    leaves: (0..3)
                        .map(|_| cx.new(|_| SiblingLeaf { revision: 0 }))
                        .collect(),
                }
            }
        };
        let eager = cx.open_window(
            size(px(500.), px(300.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(500.), px(300.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        let snapshot = |window: crate::WindowHandle<SiblingHost>, cx: &mut TestAppContext| {
            window
                .update(cx, |_, window, _| {
                    window.rendered_frame.scene.snapshot_for_test()
                })
                .expect("window open")
        };
        for step in 0..6 {
            let dirty_all = step % 2 == 0;
            for window in [eager, memoized] {
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
            assert_eq!(snapshot(eager, cx), snapshot(memoized, cx));
            memoized
                .update(cx, |_, window, _| {
                    let stats = window.node_stats();
                    assert_eq!(stats.rebuilt_scopes, if dirty_all { 4 } else { 2 });
                    assert_eq!(stats.reused_subtrees, if dirty_all { 0 } else { 2 });
                })
                .expect("window open");
        }
    }

    struct StatefulComponent {
        state: Rc<std::cell::RefCell<Option<Entity<usize>>>>,
        seen_revision: Rc<Cell<usize>>,
        renders: Rc<Cell<usize>>,
        revision: usize,
    }

    /// Only the input the component renders from takes part; the shared cells are
    /// test plumbing, standing in for the callbacks a real component would skip.
    impl PartialEq for StatefulComponent {
        fn eq(&self, other: &Self) -> bool {
            self.revision == other.revision
        }
    }

    impl Component for StatefulComponent {
        fn render(&self, window: &mut Window, cx: &mut crate::App) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
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
        component_renders: Rc<Cell<usize>>,
        revision: usize,
        show: bool,
        cached: bool,
        renders: usize,
    }

    impl Render for ComponentHost {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders += 1;
            div().when(self.show, |element| {
                let component = StatefulComponent {
                    state: self.state.clone(),
                    seen_revision: self.seen_revision.clone(),
                    renders: self.component_renders.clone(),
                    revision: self.revision,
                };
                if self.cached {
                    element.child(component.cached())
                } else {
                    element.child(component)
                }
            })
        }
    }

    #[gpui::test]
    fn node_engine_component_state_callbacks_and_unmount(cx: &mut TestAppContext) {
        for cached in [false, true] {
            component_state_callbacks_and_unmount(cached, cx);
        }
    }

    fn component_state_callbacks_and_unmount(cached: bool, cx: &mut TestAppContext) {
        let state = Rc::new(std::cell::RefCell::new(None));
        let seen = Rc::new(Cell::new(0));
        let component_renders = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(200.), px(100.)), |window, _| {
            window.node_engine = crate::NodeEngine::new();
            ComponentHost {
                state: state.clone(),
                seen_revision: seen.clone(),
                component_renders: component_renders.clone(),
                revision: 1,
                show: true,
                cached,
                renders: 0,
            }
        });
        cx.run_until_parked();
        let mut visual = crate::VisualTestContext::from_window(window.into(), cx);
        visual.simulate_click(crate::point(px(10.), px(10.)), crate::Modifiers::default());
        assert_eq!(seen.get(), 1);
        let original = state.borrow().clone().expect("component state initialized");
        cx.update(|cx| assert_eq!(*original.read(cx), 1));
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
                // A dirty node rebuilds its ancestors with it, so the host renders again
                // whether the state belongs to the host's node or to a cached component's.
                assert_eq!(
                    host.renders,
                    renders + 1,
                    "a component's state change re-renders the enclosing view"
                );
            })
            .expect("window open");
        original.update(cx, |value, _| *value -= 1);
        let renders = component_renders.get();
        window
            .update(cx, |_, _, cx| cx.notify())
            .expect("window open");
        cx.run_until_parked();
        if cached {
            assert_eq!(
                component_renders.get(),
                renders,
                "a cached component with equal inputs is reused when its host re-renders"
            );
        } else {
            assert_eq!(
                component_renders.get(),
                renders + 1,
                "an inline component renders with its host"
            );
        }
        window
            .update(cx, |host, _, cx| {
                host.revision = 9;
                cx.notify();
            })
            .expect("window open");
        cx.run_until_parked();
        assert_eq!(
            component_renders.get(),
            renders + if cached { 1 } else { 2 },
            "changed inputs re-render the component"
        );
        visual.simulate_click(crate::point(px(10.), px(10.)), crate::Modifiers::default());
        assert_eq!(seen.get(), 9, "recorded handlers must receive fresh inputs");
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
        for engine in [crate::NodeEngine::new_eager(), crate::NodeEngine::new()] {
            let renders = Rc::new(Cell::new(0));
            let window = cx.open_window(size(px(100.), px(100.)), |window, _| {
                window.node_engine = engine;
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
                window.node_engine = engine;
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
        let eager = cx.open_window(
            size(px(400.), px(200.)),
            build(crate::NodeEngine::new_eager()),
        );
        let memoized = cx.open_window(size(px(400.), px(200.)), build(crate::NodeEngine::new()));
        cx.run_until_parked();
        let baseline = memoized
            .update(cx, |_, window, _| window.node_stats().layout_nodes)
            .expect("window open");
        for step in 0..30 {
            for window in [eager, memoized] {
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
                snapshot(eager, cx),
                snapshot(memoized, cx),
                "nested frame {step}"
            );
            memoized
                .update(cx, |_, window, _| {
                    let stats = window.node_stats();
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
    fn node_engine_reuses_siblings_while_a_deferred_draw_is_open(cx: &mut TestAppContext) {
        struct PopoverOwner {
            open: bool,
            renders: Rc<Cell<usize>>,
        }
        impl Render for PopoverOwner {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                self.renders.set(self.renders.get() + 1);
                div()
                    .size(px(40.))
                    .bg(rgb(0x336699))
                    .when(self.open, |element| {
                        element.child(crate::deferred(
                            div()
                                .absolute()
                                .left(px(10.))
                                .top(px(10.))
                                .size(px(60.))
                                .bg(rgb(0xff0000)),
                        ))
                    })
            }
        }
        struct Host {
            owner: Entity<PopoverOwner>,
            changing: Entity<CountingLeaf>,
            clean: Entity<CountingLeaf>,
        }
        impl Render for Host {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .flex()
                    .child(self.owner.clone())
                    .child(div().size(px(40.)).child(self.changing.clone()))
                    .child(div().size(px(40.)).child(self.clean.clone()))
            }
        }
        let owner_renders = Rc::new(Cell::new(0));
        let changing_renders = Rc::new(Cell::new(0));
        let clean_renders = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(200.), px(100.)), |_, cx| Host {
            owner: cx.new(|_| PopoverOwner {
                open: true,
                renders: owner_renders.clone(),
            }),
            changing: cx.new(|_| CountingLeaf {
                render_count: changing_renders.clone(),
                dependency: None,
                color: 0x00ff00,
            }),
            clean: cx.new(|_| CountingLeaf {
                render_count: clean_renders.clone(),
                dependency: None,
                color: 0x0000ff,
            }),
        });
        cx.run_until_parked();
        assert_eq!(
            (
                owner_renders.get(),
                changing_renders.get(),
                clean_renders.get()
            ),
            (1, 1, 1)
        );

        let live_nodes_with_popover = window
            .update(cx, |_, window, _| window.node_stats().live_nodes)
            .expect("window open");

        // A frame caused by a sibling: the popover owner is reused, and with it the root its
        // recording attached, which is replayed rather than drawn again.
        window
            .update(cx, |host, _, cx| {
                host.changing.update(cx, |leaf, cx| {
                    leaf.color = 0x00aa00;
                    cx.notify();
                })
            })
            .expect("window open");
        cx.run_until_parked();
        assert_eq!(
            (
                owner_renders.get(),
                changing_renders.get(),
                clean_renders.get()
            ),
            (1, 2, 1)
        );
        let mut visual = crate::VisualTestContext::from_window(window.into(), cx);
        let stats = visual.assert_incremental_matches_full_refresh("sibling change");
        assert_eq!(stats.full_refresh_reason, None);
        assert!(
            stats.reused_subtrees > 0,
            "the clean siblings must be reused"
        );

        // Closing the popover renders the owner without attaching the root, which drops it.
        window
            .update(cx, |host, _, cx| {
                host.owner.update(cx, |owner, cx| {
                    owner.open = false;
                    cx.notify();
                })
            })
            .expect("window open");
        cx.run_until_parked();
        let stats = visual.assert_incremental_matches_full_refresh("popover closed");
        assert_eq!(stats.live_nodes, live_nodes_with_popover - 1);

        window
            .update(cx, |host, _, cx| {
                host.owner.update(cx, |owner, cx| {
                    owner.open = true;
                    cx.notify();
                })
            })
            .expect("window open");
        cx.run_until_parked();
        let stats = visual.assert_incremental_matches_full_refresh("popover reopened");
        assert_eq!(stats.live_nodes, live_nodes_with_popover);
    }

    #[gpui::test]
    fn node_engine_dependency_changes_dirty_views_without_notifying_them(cx: &mut TestAppContext) {
        let dependency = cx.new(|_| Dependency);
        let renders = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(100.), px(100.)), |window, _| {
            window.node_engine = crate::NodeEngine::new();
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
            window.node_engine = crate::NodeEngine::new();
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
