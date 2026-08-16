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
        recording: std::rc::Rc<ViewNodeRecording>,
        prepaint_range: Range<PrepaintStateIndex>,
    },
    Render {
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
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
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
        if self.cached_style.is_some()
            && window.node_engine_enabled()
            && let Some(entity_id) = self.entity_id
            && let Some(global_id) = global_id
            && let Some(view) = self.view.as_ref().and_then(View::retained_view)
        {
            let content_mask = window.content_mask();
            let text_style = window.text_style();
            let cache_key = ViewNodeCacheKey {
                bounds,
                content_mask,
                text_style,
            };
            if let Some(decision) =
                window.begin_view_node(global_id.clone(), view, cache_key.clone())
            {
                window.set_view_id(entity_id);
                return window.with_rendered_view(entity_id, |window| match decision {
                    NodeRenderDecision::Graft {
                        node_id,
                        recording,
                        accessed_entities,
                    } => {
                        let prepaint_range = window.graft_view_node_prepaint(&recording);
                        cx.entities.extend_accessed(&accessed_entities);
                        window.finish_view_node_prepaint(node_id, false);
                        ViewElementPrepaintState {
                            element: None,
                            node: Some(ViewNodePrepaintState::Graft {
                                node_id,
                                recording,
                                prepaint_range,
                            }),
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
                        window.finish_view_node_prepaint(node_id, true);
                        ViewElementPrepaintState {
                            element: element,
                            node: Some(ViewNodePrepaintState::Render {
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
            if let Some(entity_id) = self.entity_id {
                window.with_rendered_view(entity_id, |window| match node {
                    ViewNodePrepaintState::Graft {
                        node_id,
                        recording,
                        prepaint_range,
                    } => {
                        let paint_range = window.graft_view_node_paint(&recording);
                        window.store_grafted_view_node(
                            node_id,
                            ViewNodeRecording {
                                scene: recording.scene.clone(),
                                hitboxes: recording.hitboxes.clone(),
                                tooltip_requests: recording.tooltip_requests.clone(),
                                cursor_styles: recording.cursor_styles.clone(),
                                prepaint_range,
                                paint_range,
                            },
                        );
                    }
                    ViewNodePrepaintState::Render {
                        node_id,
                        cache_key,
                        prepaint_range,
                        accessed_entities,
                    } => {
                        let paint_start = window.paint_index();
                        if let Some(element) = element.element.as_mut() {
                            let refreshing = mem::replace(&mut window.refreshing, true);
                            element.paint(window, cx);
                            window.refreshing = refreshing;
                        }
                        let paint_range = paint_start..window.paint_index();
                        let recording =
                            window.capture_view_node_recording(prepaint_range, paint_range);
                        window.store_rendered_view_node(
                            node_id,
                            cache_key,
                            recording,
                            accessed_entities,
                        );
                    }
                });
            }
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
}
