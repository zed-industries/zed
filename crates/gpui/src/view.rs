use crate::{
    AnyElement, AnyEntity, AnyWeakEntity, App, Bounds, ContentMask, Context, Element, ElementId,
    Entity, EntityId, GlobalElementId, InspectorElementId, IntoElement, LayoutId, PaintIndex,
    Pixels, PrepaintStateIndex, Render, Style, StyleRefinement, TextStyle, WeakEntity, relative,
};
use crate::{Empty, Window};
use anyhow::Result;
use collections::FxHashSet;
use refineable::Refineable;
use std::mem;
use std::rc::Rc;
use std::{any::TypeId, fmt, hash::Hash, ops::Range};

struct AnyViewState {
    prepaint_range: Range<PrepaintStateIndex>,
    paint_range: Range<PaintIndex>,
    cache_key: ViewCacheKey,
    accessed_entities: FxHashSet<EntityId>,
}

#[derive(Default)]
struct ViewCacheKey {
    bounds: Bounds<Pixels>,
    content_mask: ContentMask<Pixels>,
    text_style: TextStyle,
}

/// A dynamically-typed handle to a view, which can be downcast to a [Entity] for a specific type.
#[derive(Clone, Debug)]
pub struct AnyView {
    entity: AnyEntity,
    render: fn(&AnyView, &mut Window, &mut App) -> AnyElement,
    cached_style: Option<Rc<StyleRefinement>>,
}

impl<V: Render> From<Entity<V>> for AnyView {
    fn from(value: Entity<V>) -> Self {
        AnyView {
            entity: value.into_any(),
            render: any_view::render::<V>,
            cached_style: None,
        }
    }
}

impl AnyView {
    /// Indicate that this view should be cached when using it as an element.
    /// When using this method, the view's previous layout and paint will be recycled from the previous frame if [Context::notify] has not been called since it was rendered.
    /// The one exception is when [Window::refresh] is called, in which case caching is ignored.
    pub fn cached(mut self, style: StyleRefinement) -> Self {
        self.cached_style = Some(style.into());
        self
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
                cached_style: self.cached_style,
            }),
        }
    }

    /// Gets the [TypeId] of the underlying view.
    pub fn entity_type(&self) -> TypeId {
        self.entity.entity_type
    }

    /// Gets the entity id of this handle.
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

impl Element for AnyView {
    type RequestLayoutState = Option<AnyElement>;
    type PrepaintState = Option<AnyElement>;

    fn id(&self) -> Option<ElementId> {
        Some(ElementId::View(self.entity_id()))
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        window.with_rendered_view(self.entity_id(), |window| {
            // Disable caching when inspecting so that mouse_hit_test has all hitboxes.
            let caching_disabled = window.is_inspector_picking(cx);
            match self.cached_style.as_ref() {
                Some(style) if !caching_disabled => {
                    let mut root_style = Style::default();
                    root_style.refine(style);
                    let layout_id = window.request_layout(root_style, None, cx);
                    (layout_id, None)
                }
                _ => {
                    let mut element = (self.render)(self, window, cx);
                    let layout_id = element.request_layout(window, cx);
                    (layout_id, Some(element))
                }
            }
        })
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
        window.set_view_id(self.entity_id());
        window.with_rendered_view(self.entity_id(), |window| {
            if let Some(mut element) = element.take() {
                element.prepaint(window, cx);
                return Some(element);
            }

            window.with_element_state::<AnyViewState, _>(
                global_id.unwrap(),
                |element_state, window| {
                    let content_mask = window.content_mask();
                    let text_style = window.text_style();

                    if let Some(mut element_state) = element_state
                        && element_state.cache_key.bounds == bounds
                        && element_state.cache_key.content_mask == content_mask
                        && element_state.cache_key.text_style == text_style
                        && !window.dirty_views.contains(&self.entity_id())
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
                        let mut element = (self.render)(self, window, cx);
                        element.layout_as_root(bounds.size.into(), window, cx);
                        element.prepaint_at(bounds.origin, window, cx);
                        element
                    });

                    let prepaint_end = window.prepaint_index();
                    window.refreshing = refreshing;

                    (
                        Some(element),
                        AnyViewState {
                            accessed_entities,
                            prepaint_range: prepaint_start..prepaint_end,
                            paint_range: PaintIndex::default()..PaintIndex::default(),
                            cache_key: ViewCacheKey {
                                bounds,
                                content_mask,
                                text_style,
                            },
                        },
                    )
                },
            )
        })
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        element: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_rendered_view(self.entity_id(), |window| {
            let caching_disabled = window.is_inspector_picking(cx);
            if self.cached_style.is_some() && !caching_disabled {
                window.with_element_state::<AnyViewState, _>(
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
    }
}

impl<V: 'static + Render> IntoElement for Entity<V> {
    type Element = AnyView;

    fn into_element(self) -> Self::Element {
        self.into()
    }
}

impl IntoElement for AnyView {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// A weak, dynamically-typed view handle that does not prevent the view from being released.
pub struct AnyWeakView {
    entity: AnyWeakEntity,
    render: fn(&AnyView, &mut Window, &mut App) -> AnyElement,
}

impl AnyWeakView {
    /// Convert to a strongly-typed handle if the referenced view has not yet been released.
    pub fn upgrade(&self) -> Option<AnyView> {
        let entity = self.entity.upgrade()?;
        Some(AnyView {
            entity,
            render: self.render,
            cached_style: None,
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
        view.update(cx, |view, cx| view.render(window, cx).into_any_element())
    }
}

/// A component backed by an entity that participates in GPUI's reactive graph.
///
/// Views combine the ergonomic builder-pattern API of components ([`RenderOnce`](crate::RenderOnce))
/// with the push-pull reactivity of entities. When a view's entity calls
/// `cx.notify()`, only this view (and its dirty ancestors/children) need to
/// re-render.
///
/// Unlike [`Render`], which puts the rendering trait on the entity's state type,
/// `View` goes on the *component* type — the struct that holds both the entity
/// handle and any display props. This means the consumer controls styling
/// through the builder pattern, while the entity provides reactive state.
///
/// # Example
///
/// ```ignore
/// struct CounterState {
///     count: usize,
/// }
///
/// struct Counter {
///     state: Entity<CounterState>,
///     label: SharedString,
/// }
///
/// impl Counter {
///     fn new(state: Entity<CounterState>) -> Self {
///         Self { state, label: "Count".into() }
///     }
///
///     fn label(mut self, label: impl Into<SharedString>) -> Self {
///         self.label = label.into();
///         self
///     }
/// }
///
/// impl View for Counter {
///     type State = CounterState;
///
///     fn entity(&self) -> &Entity<CounterState> {
///         &self.state
///     }
///
///
///     fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
///         let count = self.state.read(cx).count;
///         div().child(format!("{}: {}", self.label, count))
///     }
/// }
///
/// // Usage in a parent's render:
/// // Counter::new(my_counter_entity).label("Total")
/// ```
pub trait View: 'static + Sized + Hash {
    /// The entity type that backs this view's state.
    type State: 'static;

    /// Returns a reference to the entity that backs this view.
    fn entity(&self) -> &Entity<Self::State>;

    /// Render this view into an element tree. Takes ownership of self,
    /// consuming the component props. The entity state persists across frames.
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement;

    /// Returns the style to use for caching this view.
    /// When `Some`, the view element will be cached using the given style for its outer layout.
    /// The default returns a full-size style refinement (`width: 100%, height: 100%`).
    /// Return `None` to disable caching.
    fn style(&self) -> Option<StyleRefinement> {
        let mut style = StyleRefinement::default();
        style.size.width = Some(relative(1.).into());
        style.size.height = Some(relative(1.).into());
        Some(style)
    }
}

/// An element that wraps a [`View`], creating a reactive boundary in the element tree.
///
/// This is the stateful counterpart to [`Component`](crate::Component) — where `Component<C>`
/// wraps a stateless [`RenderOnce`](crate::RenderOnce) type, `ViewElement<V>` wraps a stateful
/// [`View`] type and hooks its entity into GPUI's push-pull reactive graph.
///
/// You don't construct this directly. Instead, implement [`IntoElement`] for your
/// [`View`] type using [`ViewElement::new`]:
///
/// ```ignore
/// impl IntoElement for Counter {
///     type Element = ViewElement<Self>;
///     fn into_element(self) -> Self::Element {
///         ViewElement::new(self)
///     }
/// }
/// ```
#[doc(hidden)]
pub struct ViewElement<V: View> {
    view: Option<V>,
    entity_id: EntityId,
    props_hash: u64,
    cached_style: Option<StyleRefinement>,
    #[cfg(debug_assertions)]
    source: &'static core::panic::Location<'static>,
}

impl<V: View> ViewElement<V> {
    /// Create a new `ViewElement` wrapping the given [`View`].
    ///
    /// Use this in your [`IntoElement`] implementation.
    #[track_caller]
    pub fn new(view: V) -> Self {
        use std::hash::Hasher;
        let entity_id = view.entity().entity_id();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        view.hash(&mut hasher);
        let props_hash = hasher.finish();
        ViewElement {
            entity_id,
            props_hash,
            cached_style: None,
            view: Some(view),
            #[cfg(debug_assertions)]
            source: core::panic::Location::caller(),
        }
    }

    /// Enable caching for this view element. When cached, the view's render,
    /// prepaint, and paint will all be reused from the previous frame if the
    /// entity hasn't been notified and the props hash hasn't changed.
    ///
    /// The provided style defines the outer layout of this view in its parent
    /// (since the actual content render is deferred until we know if caching
    /// can be used).
    pub fn cached(mut self, style: StyleRefinement) -> Self {
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
    props_hash: u64,
}

impl<V: View> Element for ViewElement<V> {
    type RequestLayoutState = Option<AnyElement>;
    type PrepaintState = Option<AnyElement>;

    fn id(&self) -> Option<ElementId> {
        Some(ElementId::View(self.entity_id))
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
        window.with_rendered_view(self.entity_id, |window| {
            let caching_disabled = window.is_inspector_picking(cx);
            match self.cached_style.as_ref() {
                // Cached path: defer render, use style for layout.
                // Render will happen in prepaint only on cache miss.
                Some(style) if !caching_disabled => {
                    let mut root_style = Style::default();
                    root_style.refine(style);
                    let layout_id = window.request_layout(root_style, None, cx);
                    (layout_id, None)
                }
                // Non-cached path: render eagerly.
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
        window.set_view_id(self.entity_id);
        window.with_rendered_view(self.entity_id, |window| {
            // Non-cached path: element was rendered in request_layout, just prepaint it.
            if let Some(mut element) = element.take() {
                element.prepaint(window, cx);
                return Some(element);
            }

            // Cached path: check cache, render only on miss.
            window.with_element_state::<ViewElementState, _>(
                global_id.unwrap(),
                |element_state, window| {
                    let content_mask = window.content_mask();
                    let text_style = window.text_style();

                    // Cache hit: entity clean, props unchanged, bounds stable.
                    // Skip render, prepaint, and paint entirely.
                    if let Some(mut element_state) = element_state
                        && element_state.cache_key.bounds == bounds
                        && element_state.cache_key.content_mask == content_mask
                        && element_state.cache_key.text_style == text_style
                        && element_state.cache_key.props_hash == self.props_hash
                        && !window.dirty_views.contains(&self.entity_id)
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

                    // Cache miss: render now, layout as root, prepaint.
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
                                props_hash: self.props_hash,
                            },
                        },
                    )
                },
            )
        })
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
        window.with_rendered_view(self.entity_id, |window| {
            let caching_disabled = window.is_inspector_picking(cx);
            if self.cached_style.is_some() && !caching_disabled {
                // Cached path: check if we rendered or reused.
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
                // Non-cached path: just paint the element.
                element.as_mut().unwrap().paint(window, cx);
            }
        });
    }
}

/// A view that renders nothing
pub struct EmptyView;

impl Render for EmptyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}
