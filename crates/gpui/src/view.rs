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
use std::{
    any::{TypeId, type_name},
    fmt,
    ops::Range,
};
use std::{cell::Cell, mem, rc::Rc};

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
    independent_cache: bool,
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
            independent_cache: false,
            view: Some(view),
            #[cfg(debug_assertions)]
            source: core::panic::Location::caller(),
        }
    }

    /// Enable caching of this view's rendered subtree, laid out at `style`.
    /// The composer supplies the layout style because caching skips rendering
    /// the contents to measure them.
    pub(crate) fn cached(mut self, style: StyleRefinement) -> Self {
        self.cached_style = self.entity_id.map(|_| style);
        self
    }

    /// Caches this subtree independently, preventing ancestor view caches from being reused.
    pub fn cached_independently(mut self, style: StyleRefinement) -> Self {
        self.independent_cache = true;
        self.cached(style)
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
    paint_valid: bool,
    contains_independent_cache: Rc<Cell<bool>>,
    cache_key: ViewElementCacheKey,
    accessed_entities: FxHashSet<EntityId>,
}

struct ViewElementCacheKey {
    independent: bool,
    bounds: Bounds<Pixels>,
    content_mask: ContentMask<Pixels>,
    text_style: TextStyle,
    rem_size: Pixels,
    opacity: f32,
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
                self.independent_cache,
                global_id,
                bounds,
                element,
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
    independent: bool,
    global_id: Option<&GlobalElementId>,
    bounds: Bounds<Pixels>,
    element: &mut Option<AnyElement>,
    window: &mut Window,
    cx: &mut App,
    render: &mut dyn FnMut(&mut Window, &mut App) -> AnyElement,
) -> Option<AnyElement> {
    window.set_view_id(entity_id);
    if independent {
        for ancestor in &window.cache_ancestors {
            ancestor.set(true);
        }
    }
    window.with_rendered_view(entity_id, |window| {
        if let Some(mut element) = element.take() {
            element.prepaint(window, cx);
            return Some(element);
        }

        window.with_element_state::<ViewElementState, _>(
            global_id.unwrap(),
            |element_state, window| {
                let content_mask = window.content_mask();
                let text_style = window.text_style();
                let rem_size = window.rem_size();
                let opacity = window.element_opacity();

                if let Some(mut element_state) = element_state
                    && element_state.paint_valid
                    && !element_state.contains_independent_cache.get()
                    && element_state.cache_key.independent == independent
                    && element_state.cache_key.bounds == bounds
                    && element_state.cache_key.content_mask == content_mask
                    && element_state.cache_key.text_style == text_style
                    && element_state.cache_key.rem_size == rem_size
                    && element_state.cache_key.opacity == opacity
                    && !window.dirty_views.contains(&entity_id)
                    && !window.refreshing
                    && (independent || !window.cache_refreshing)
                {
                    let prepaint_start = window.prepaint_index();
                    window.reuse_prepaint(element_state.prepaint_range.clone());
                    cx.entities
                        .extend_accessed(&element_state.accessed_entities);
                    let prepaint_end = window.prepaint_index();
                    element_state.prepaint_range = prepaint_start..prepaint_end;
                    element_state.paint_valid = false;

                    return (None, element_state);
                }

                window.dirty_views.insert(entity_id);
                let refreshing = mem::replace(&mut window.cache_refreshing, true);
                let contains_independent_cache = Rc::new(Cell::new(false));
                window
                    .cache_ancestors
                    .push(contains_independent_cache.clone());
                let prepaint_start = window.prepaint_index();
                let (element, accessed_entities) = cx.detect_accessed_entities(|cx| {
                    let mut element = render(window, cx);
                    element.layout_as_root(Size::<AvailableSpace>::from(bounds.size), window, cx);
                    element.prepaint_at(bounds.origin, window, cx);
                    element
                });

                let prepaint_end = window.prepaint_index();
                window.cache_ancestors.pop();
                window.cache_refreshing = refreshing;

                (
                    Some(element),
                    ViewElementState {
                        accessed_entities,
                        prepaint_range: prepaint_start..prepaint_end,
                        paint_range: PaintIndex::default()..PaintIndex::default(),
                        paint_valid: false,
                        contains_independent_cache,
                        cache_key: ViewElementCacheKey {
                            independent,
                            bounds,
                            content_mask,
                            text_style,
                            rem_size,
                            opacity,
                        },
                    },
                )
            },
        )
    })
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
    window.with_rendered_view(entity_id, |window| {
        let caching_disabled = window.is_inspector_picking(cx);
        if cached && !caching_disabled {
            window.with_element_state::<ViewElementState, _>(
                global_id.unwrap(),
                |element_state, window| {
                    let mut element_state = element_state.unwrap();

                    let paint_start = window.paint_index();

                    if let Some(element) = element {
                        let refreshing = mem::replace(&mut window.cache_refreshing, true);
                        element.paint(window, cx);
                        window.cache_refreshing = refreshing;
                    } else {
                        window.reuse_paint(element_state.paint_range.clone());
                    }

                    let paint_end = window.paint_index();
                    element_state.paint_range = paint_start..paint_end;
                    element_state.cache_key.opacity = window.element_opacity();
                    element_state.paint_valid = true;

                    ((), element_state)
                },
            )
        } else {
            element.as_mut().unwrap().paint(window, cx);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AppContext as _, InteractiveElement as _, ParentElement as _, Styled as _, TestAppContext,
        VisualTestContext, canvas, div, point, px, rems, size, util::FluentBuilder as _,
    };
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
        sync::Arc,
    };

    #[gpui::test]
    fn cached_views_track_parent_context(cx: &mut TestAppContext) {
        let paints = Rc::new(RefCell::new(Vec::new()));
        let (root, cx) = cx.add_window_view(|_, cx| ContextCacheRoot {
            rem_size: px(16.),
            opacity: 1.,
            hover_opacity: false,
            hidden: true,
            content: cx.new(|_| ContextLayer(paints.clone())),
        });
        assert_eq!(paints.borrow().as_slice(), &[]);

        for (rem_size, opacity, hidden, expected_opacity) in [
            (16., 1., false, Some(0.5)),
            (16., 1., true, None),
            (16., 1., false, Some(0.5)),
            (32., 1., false, Some(0.5)),
            (16., 1., false, Some(0.5)),
            (16., 0., false, Some(0.)),
            (16., 1., false, Some(0.5)),
            (16., 0.5, false, Some(0.25)),
        ] {
            paints.borrow_mut().clear();
            root.update(cx, |root, cx| {
                root.rem_size = px(rem_size);
                root.opacity = opacity;
                root.hidden = hidden;
                cx.notify();
            });
            cx.run_until_parked();
            assert_eq!(
                *paints.borrow(),
                expected_opacity
                    .map(|opacity| (size(px(rem_size), px(rem_size)), px(rem_size), opacity))
                    .into_iter()
                    .collect::<Vec<_>>()
            );
            paints.borrow_mut().clear();
            root.update(cx, |_, cx| cx.notify());
            cx.run_until_parked();
            assert_eq!(paints.borrow().as_slice(), &[]);
        }

        cx.update(|window, cx| window.simulate_mouse_move(point(px(10.), px(10.)), cx));
        let drag_view = cx.new(|_| EmptyView);
        for (dragging, expected_opacity) in [(true, 0.125), (false, 0.5)] {
            paints.borrow_mut().clear();
            root.update(cx, |root, cx| {
                root.opacity = 1.;
                cx.active_drag = dragging.then(|| crate::AnyDrag {
                    view: AnyView::from(drag_view.clone()),
                    value: Arc::new(()),
                    cursor_offset: point(px(0.), px(0.)),
                    cursor_style: None,
                    external_payload_source: None,
                });
                cx.notify();
            });
            cx.run_until_parked();
            assert_eq!(
                paints.borrow().as_slice(),
                &[(size(px(16.), px(16.)), px(16.), expected_opacity)]
            );
        }

        root.update(cx, |root, cx| {
            root.opacity = 1.;
            root.hover_opacity = true;
            cx.notify();
        });
        cx.run_until_parked();
        for (position, opacity) in [(200., 0.5), (10., 0.), (200., 0.5)] {
            cx.update(|window, cx| {
                window.simulate_mouse_move(point(px(position), px(position)), cx);
            });
            cx.run_until_parked();
            paints.borrow_mut().clear();
            root.update(cx, |_, cx| cx.notify());
            cx.run_until_parked();
            assert_eq!(
                paints.borrow().as_slice(),
                &[(size(px(16.), px(16.)), px(16.), opacity)]
            );
        }
    }

    #[gpui::test]
    fn independent_cached_views_keep_ancestors_live(cx: &mut TestAppContext) {
        for deferred_depth in 0..=2 {
            let (root, layers, cx) = boundary_cache_window(cx, deferred_depth, Some(true));
            let stateless = root.read_with(cx, |root, _| root.stateless.clone());
            assert_boundary_counts(&layers, [1; 4]);
            assert_counts(std::slice::from_ref(&stateless), 1);
            for (notified, expected) in [
                (None, [2, 1, 2, 1]),
                (Some(2), [3, 1, 3, 1]),
                (Some(1), [4, 2, 4, 1]),
                (Some(0), [5, 2, 5, 1]),
                (None, [6, 2, 6, 1]),
            ] {
                if let Some(index) = notified {
                    layers[index].content.update(cx, |_, cx| cx.notify());
                } else {
                    root.update(cx, |_, cx| cx.notify());
                }
                cx.run_until_parked();
                assert_boundary_counts(&layers, expected);
                assert_counts(std::slice::from_ref(&stateless), expected[0]);
            }
            cx.update(|window, _| window.refresh());
            cx.run_until_parked();
            assert_boundary_counts(&layers, [7, 3, 7, 2]);
        }
    }

    #[gpui::test]
    fn independent_cache_boundary_transitions_preserve_sibling_ranges(cx: &mut TestAppContext) {
        let (root, layers, cx) = boundary_cache_window(cx, 0, Some(false));
        assert_boundary_counts(&layers, [1; 4]);
        for (mode, prefix, notify_parent, expected) in [
            (Some(false), true, false, [1, 1, 1, 1]),
            (Some(true), false, true, [2, 2, 2, 1]),
            (Some(true), true, false, [3, 2, 3, 1]),
            (Some(false), false, true, [4, 3, 4, 1]),
            (Some(false), true, false, [4, 3, 4, 1]),
            (Some(true), false, true, [5, 4, 5, 1]),
            (None, true, true, [6, 4, 6, 1]),
            (None, false, false, [6, 4, 6, 1]),
        ] {
            root.update(cx, |root, cx| {
                root.parent.mode = mode;
                root.prefix = prefix;
                if notify_parent {
                    root.parent.layer.content.update(cx, |_, cx| cx.notify());
                }
                cx.notify();
            });
            cx.run_until_parked();
            assert_boundary_counts(&layers, expected);
            cx.update(|window, _| {
                let mut expected = Vec::new();
                if prefix {
                    expected.push((200., 0., 0xffffff));
                }
                if mode.is_some() {
                    expected.push((0., 0., 0x00ff00));
                }
                expected.extend([
                    (0., if mode.is_some() { 20. } else { 0. }, 0x0000ff),
                    (0., 100., 0xff00ff),
                ]);
                let expected = expected
                    .into_iter()
                    .map(|(x, y, color)| {
                        (
                            Bounds::new(point(px(x), px(y)), size(px(20.), px(20.))),
                            crate::Background::from(crate::rgb(color)),
                        )
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    window
                        .painted_quads()
                        .iter()
                        .map(|quad| (quad.bounds, quad.background))
                        .collect::<Vec<_>>(),
                    expected
                        .iter()
                        .map(|(bounds, color)| (bounds.scale(window.scale_factor()), *color))
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    window
                        .rendered_frame
                        .hitboxes
                        .iter()
                        .filter(|hitbox| hitbox.behavior == crate::HitboxBehavior::BlockMouse)
                        .map(|hitbox| hitbox.bounds)
                        .collect::<Vec<_>>(),
                    expected
                        .iter()
                        .map(|(bounds, _)| *bounds)
                        .collect::<Vec<_>>()
                );
            });
        }
    }

    #[gpui::test]
    fn independent_cached_prepaint_retry_rebuilds_nested_cache(cx: &mut TestAppContext) {
        let counters = [Counter::default(), Counter::default()];
        let actions = Rc::new(Cell::new(0));
        let (root, cx) = cx.add_window_view(|window, cx| {
            let focus = cx.focus_handle();
            window.focus(&focus, cx);
            let nested = CacheSwatch {
                content: cx.new(|_| EmptyView),
                counter: counters[1].clone(),
                color: 0x00ff00,
            };
            RetryCacheRoot {
                content: cx.new(|_| RetryCacheContent {
                    counter: counters[0].clone(),
                    nested,
                    focus,
                    actions: actions.clone(),
                }),
                discard: false,
            }
        });
        assert_counts(&counters, 1);
        for (discard, expected_actions) in [(true, 1), (false, 2)] {
            root.update(cx, |root, cx| {
                root.discard = discard;
                cx.notify();
            });
            cx.run_until_parked();
            assert_counts(&counters, 2);
            cx.dispatch_action(CacheRetryAction);
            assert_eq!(actions.get(), expected_actions);
        }
    }

    #[derive(Clone, Default, IntoElement)]
    struct Counter(Rc<[Cell<usize>; 3]>);

    impl RenderOnce for Counter {
        fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
            self.0[0].set(self.0[0].get() + 1);
            let prepaint = self.0.clone();
            let paint = self.0;
            canvas(
                move |_, _, _| prepaint[1].set(prepaint[1].get() + 1),
                move |_, _, _, _| paint[2].set(paint[2].get() + 1),
            )
            .size(px(10.))
        }
    }

    type ContextPaintLog = Rc<RefCell<Vec<(Size<Pixels>, Pixels, f32)>>>;

    struct ContextCacheRoot {
        rem_size: Pixels,
        opacity: f32,
        hover_opacity: bool,
        hidden: bool,
        content: Entity<ContextLayer>,
    }

    impl Render for ContextCacheRoot {
        fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            window.set_rem_size(self.rem_size);
            div()
                .size(px(100.))
                .opacity(self.opacity)
                .drag_over::<()>(|style, _, _, _| style.opacity(0.25))
                .when(self.hidden, |parent| parent.invisible())
                .when(self.hover_opacity, |parent| {
                    parent.hover(|style| style.opacity(0.))
                })
                .child(
                    div().size_full().opacity(0.5).child(
                        self.content
                            .clone()
                            .cached(StyleRefinement::default().size(px(100.))),
                    ),
                )
        }
    }

    struct ContextLayer(ContextPaintLog);

    impl Render for ContextLayer {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let paints = self.0.clone();
            canvas(
                |_, _, _| {},
                move |bounds, _, window, _| {
                    paints.borrow_mut().push((
                        bounds.size,
                        window.rem_size(),
                        window.element_opacity(),
                    ));
                },
            )
            .size(rems(1.))
        }
    }

    fn assert_counts(counters: &[Counter], expected: usize) {
        for counter in counters {
            assert_eq!(counter.0.each_ref().map(Cell::get), [expected; 3]);
        }
    }

    struct BoundaryCacheRoot {
        parent: BoundaryCacheParent,
        sibling: CacheSwatch,
        prefix: bool,
        stateless: Counter,
    }

    impl Render for BoundaryCacheRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .flex()
                .flex_col()
                .when(self.prefix, |root| {
                    root.child(
                        div()
                            .absolute()
                            .left(px(200.))
                            .top_0()
                            .size(px(20.))
                            .bg(crate::rgb(0xffffff))
                            .occlude(),
                    )
                })
                .child(
                    ViewElement::new(self.parent.clone())
                        .cached(StyleRefinement::default().size(px(100.)).flex_none()),
                )
                .child(
                    ViewElement::new(self.sibling.clone())
                        .cached(StyleRefinement::default().size(px(20.)).flex_none()),
                )
                .child(ViewElement::new(self.stateless.clone()).cached(StyleRefinement::default()))
        }
    }

    #[derive(Clone)]
    struct BoundaryCacheParent {
        layer: CacheSwatch,
        content: CacheSwatch,
        cursor: CacheSwatch,
        mode: Option<bool>,
        deferred_depth: usize,
    }

    impl View for BoundaryCacheParent {
        fn entity_id(&self) -> Option<EntityId> {
            Some(self.layer.content.entity_id())
        }

        fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
            self.layer.content.read(cx);
            let content = self.mode.map(|independent| {
                let content = ViewElement::new(self.content);
                let style = StyleRefinement::default().size(px(20.)).flex_none();
                let mut content = if independent {
                    content.cached_independently(style)
                } else {
                    content.cached(style)
                }
                .into_any_element();
                for _ in 0..self.deferred_depth {
                    content = crate::deferred(content).into_any_element();
                }
                content
            });
            div()
                .size_full()
                .flex()
                .flex_col()
                .child(div().absolute().size_full().child(self.layer.counter))
                .children(content)
                .child(ViewElement::new(self.cursor))
        }
    }

    #[derive(Clone)]
    struct CacheSwatch {
        content: Entity<EmptyView>,
        counter: Counter,
        color: u32,
    }

    impl View for CacheSwatch {
        fn entity_id(&self) -> Option<EntityId> {
            Some(self.content.entity_id())
        }

        fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
            self.content.read(cx);
            div()
                .size(px(20.))
                .flex_none()
                .bg(crate::rgb(self.color))
                .occlude()
                .child(self.counter)
        }
    }

    crate::actions!(view_cache_tests, [CacheRetryAction]);

    struct RetryCacheRoot {
        content: Entity<RetryCacheContent>,
        discard: bool,
    }

    impl Render for RetryCacheRoot {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let content = self.content.clone();
            let counters = [
                content.read(cx).counter.clone(),
                content.read(cx).nested.counter.clone(),
            ];
            let discard = self.discard;
            canvas(
                move |bounds, window, cx| {
                    let mut prepaint = |window: &mut Window| {
                        let mut element = ViewElement::new(content.clone())
                            .cached_independently(StyleRefinement::default().size(px(100.)))
                            .into_any_element();
                        element.layout_as_root(
                            Size::<AvailableSpace>::from(bounds.size),
                            window,
                            cx,
                        );
                        element.prepaint_at(bounds.origin, window, cx);
                        element
                    };
                    if discard {
                        assert_eq!(
                            window.transact(|window| {
                                prepaint(window);
                                assert_counts(&counters, 1);
                                Err::<(), _>(())
                            }),
                            Err(())
                        );
                    }
                    prepaint(window)
                },
                |_, mut element, window, cx| element.paint(window, cx),
            )
            .size(px(100.))
        }
    }

    struct RetryCacheContent {
        counter: Counter,
        nested: CacheSwatch,
        focus: crate::FocusHandle,
        actions: Rc<Cell<usize>>,
    }

    impl Render for RetryCacheContent {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let actions = self.actions.clone();
            div()
                .size_full()
                .track_focus(&self.focus)
                .on_action(move |_: &CacheRetryAction, _, cx| {
                    actions.set(actions.get() + 1);
                    cx.propagate();
                })
                .child(self.counter.clone())
                .child(
                    ViewElement::new(self.nested.clone())
                        .cached(StyleRefinement::default().size(px(20.))),
                )
        }
    }

    fn boundary_cache_window(
        cx: &mut TestAppContext,
        deferred_depth: usize,
        mode: Option<bool>,
    ) -> (
        Entity<BoundaryCacheRoot>,
        [CacheSwatch; 4],
        &mut VisualTestContext,
    ) {
        let layers = [0, 0x00ff00, 0x0000ff, 0xff00ff].map(|color| CacheSwatch {
            content: cx.new(|_| EmptyView),
            counter: Counter::default(),
            color,
        });
        let (root, cx) = cx.add_window_view(|_, _| BoundaryCacheRoot {
            parent: BoundaryCacheParent {
                layer: layers[0].clone(),
                content: layers[1].clone(),
                cursor: layers[2].clone(),
                mode,
                deferred_depth,
            },
            sibling: layers[3].clone(),
            prefix: false,
            stateless: Counter::default(),
        });
        (root, layers, cx)
    }

    fn assert_boundary_counts(layers: &[CacheSwatch; 4], expected: [usize; 4]) {
        for (layer, expected) in layers.iter().zip(expected) {
            assert_counts(std::slice::from_ref(&layer.counter), expected);
        }
    }
}
