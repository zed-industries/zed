use crate::{
    AnyElement, AnyEntity, AnyWeakEntity, App, Bounds, ContentMask, Context, Element, ElementId,
    Entity, EntityId, GlobalElementId, InspectorElementId, IntoElement, LayoutId, PaintIndex,
    Pixels, Point, PrepaintStateIndex, Render, RenderOnce, Style, StyleRefinement, TextStyle,
    WeakEntity,
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
    /// How far the reused prepaint records were moved this frame, for paint
    /// to move the paint records by the same amount.
    reuse_offset: Point<Pixels>,
}

struct ViewElementCacheKey {
    bounds: Bounds<Pixels>,
    content_mask: ContentMask<Pixels>,
    text_style: TextStyle,
    /// The view lay entirely inside its content mask when it was recorded,
    /// so the records hold everything it painted and can be reused at
    /// another position; a clipped recording is missing what fell outside.
    unclipped: bool,
}

/// Whether `bounds` lies entirely inside `content_mask`, edges included —
/// `Bounds::is_contained_within` excludes the far edges, which a view that
/// fills its container's width always touches.
fn unclipped_by(bounds: Bounds<Pixels>, content_mask: &ContentMask<Pixels>) -> bool {
    let mask = content_mask.bounds;
    bounds.origin.x >= mask.origin.x
        && bounds.origin.y >= mask.origin.y
        && bounds.right() <= mask.right()
        && bounds.bottom() <= mask.bottom()
}

impl ViewElementCacheKey {
    /// Whether records made under this key can be reused at `bounds` under
    /// `content_mask`: exactly as they are, or moved by the returned offset.
    fn reuse_offset(
        &self,
        bounds: Bounds<Pixels>,
        content_mask: &ContentMask<Pixels>,
    ) -> Option<Point<Pixels>> {
        if self.bounds == bounds && self.content_mask == *content_mask {
            Some(Point::default())
        } else if self.unclipped && self.bounds.size == bounds.size {
            Some(bounds.origin - self.bounds.origin)
        } else {
            None
        }
    }
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
    ) -> Option<AnyElement> {
        if let Some(entity_id) = self.entity_id {
            // Stateful path.
            window.set_view_id(entity_id);
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

                        if let Some(mut element_state) = element_state
                            && element_state.cache_key.text_style == text_style
                            && !window.dirty_views.contains(&entity_id)
                            && !window.refreshing
                            && let Some(offset) =
                                element_state.cache_key.reuse_offset(bounds, &content_mask)
                        {
                            let prepaint_start = window.prepaint_index();
                            window.reuse_prepaint_at(element_state.prepaint_range.clone(), offset);
                            cx.entities
                                .extend_accessed(&element_state.accessed_entities);
                            let prepaint_end = window.prepaint_index();
                            element_state.prepaint_range = prepaint_start..prepaint_end;
                            element_state.reuse_offset = offset;
                            // The records now describe the view here, clipped
                            // by what clips it here.
                            element_state.cache_key.bounds = bounds;
                            element_state.cache_key.unclipped = unclipped_by(bounds, &content_mask);
                            element_state.cache_key.content_mask = content_mask;

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
                                    unclipped: unclipped_by(bounds, &content_mask),
                                    content_mask,
                                    text_style,
                                },
                                reuse_offset: Point::default(),
                            },
                        )
                    },
                )
            })
        } else {
            // Stateless path: just prepaint the element.
            window.with_id(
                ElementId::Name(std::any::type_name::<V>().into()),
                |window| {
                    element.as_mut().unwrap().prepaint(window, cx);
                },
            );
            Some(element.take().unwrap())
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
                        let refreshing = mem::replace(&mut window.refreshing, true);
                        element.paint(window, cx);
                        window.refreshing = refreshing;
                    } else {
                        window.reuse_paint_at(
                            element_state.paint_range.clone(),
                            element_state.reuse_offset,
                        );
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
mod cached_view_tests {
    use super::*;
    use crate::{
        AnyWindowHandle, AppContext as _, Hsla, InteractiveElement as _, ParentElement as _,
        ScaledPixels, Styled as _, TestAppContext, div, px,
    };
    use std::{cell::Cell, rc::Rc};

    /// A fixed-size card whose renders are counted, with a hover style so a
    /// hitbox gets recorded and solid backgrounds so quads do.
    struct Card {
        renders: Rc<Cell<usize>>,
    }

    impl Render for Card {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div()
                .id("card")
                .size(px(100.))
                .bg(Hsla::red())
                .hover(|style| style.bg(Hsla::green()))
                .child(div().id("inner").size(px(20.)).bg(Hsla::blue()))
        }
    }

    /// A 200px-tall clipping viewport with the card placed `scroll` pixels
    /// from its top, like an item in a scrolling list.
    struct Viewport {
        card: Entity<Card>,
        scroll: Rc<Cell<f32>>,
    }

    impl Render for Viewport {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let mut style = StyleRefinement::default();
            style.size.width = Some(px(100.).into());
            style.size.height = Some(px(100.).into());
            div().size(px(200.)).overflow_hidden().child(
                div()
                    .mt(px(self.scroll.get()))
                    .child(self.card.clone().cached(style)),
            )
        }
    }

    fn quads(cx: &mut TestAppContext, window: AnyWindowHandle) -> Vec<(f32, f32, f32, f32)> {
        cx.update_window(window, |_, window, _| {
            let scale = window.scale_factor();
            let unscale = |v: ScaledPixels| v.0 / scale;
            let mut quads: Vec<_> = window
                .rendered_frame
                .scene
                .quads
                .iter()
                .map(|quad| {
                    let clipped = quad.bounds.intersect(&quad.content_mask.bounds);
                    (
                        unscale(clipped.origin.x),
                        unscale(clipped.origin.y),
                        unscale(clipped.size.width),
                        unscale(clipped.size.height),
                    )
                })
                .collect();
            quads.sort_by(|a, b| a.partial_cmp(b).unwrap());
            quads
        })
        .unwrap()
    }

    fn hitboxes(cx: &mut TestAppContext, window: AnyWindowHandle) -> Vec<(f32, f32)> {
        cx.update_window(window, |_, window, _| {
            let mut hitboxes: Vec<_> = window
                .rendered_frame
                .hitboxes
                .iter()
                .map(|hitbox| {
                    (
                        f32::from(hitbox.bounds.origin.y),
                        f32::from(hitbox.size.height),
                    )
                })
                .collect();
            hitboxes.sort_by(|a, b| a.partial_cmp(b).unwrap());
            hitboxes
        })
        .unwrap()
    }

    /// A card of `items` columns of `depth` nested stateful divs.
    struct DeepCard {
        items: usize,
        depth: usize,
        renders: Rc<Cell<usize>>,
    }

    impl Render for DeepCard {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            let depth = self.depth;
            div()
                .flex()
                .flex_wrap()
                .size_full()
                .children((0..self.items).map(move |item| {
                    let mut element = div().id(("leaf", item as u64)).size(px(2.));
                    for level in (0..depth).rev() {
                        element = div()
                            .id(("some-container-element", level as u64))
                            .child(element);
                    }
                    element.id(("item", item as u64)).size(px(4.))
                }))
        }
    }

    /// A list of `CARD_HEIGHT`-tall cached cards in an 800px viewport,
    /// rendering only the cards that intersect it, scrolled by `scroll`.
    struct ScrollingList {
        cards: Vec<Entity<DeepCard>>,
        scroll: Rc<Cell<f32>>,
    }

    const CARD_HEIGHT: f32 = 100.;
    const VIEWPORT_HEIGHT: f32 = 800.;

    impl Render for ScrollingList {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let scroll = self.scroll.get();
            div()
                .w(px(400.))
                .h(px(VIEWPORT_HEIGHT))
                .overflow_hidden()
                .children(self.cards.iter().enumerate().filter_map(|(ix, card)| {
                    let top = ix as f32 * CARD_HEIGHT - scroll;
                    (top + CARD_HEIGHT > 0. && top < VIEWPORT_HEIGHT).then(|| {
                        let mut style = StyleRefinement::default();
                        style.size.width = Some(px(400.).into());
                        style.size.height = Some(px(CARD_HEIGHT).into());
                        style.position = Some(crate::Position::Absolute);
                        style.inset.top = Some(px(top).into());
                        card.clone().cached(style)
                    })
                }))
        }
    }

    /// Frame cost of scrolling a list of cached views by a few pixels per
    /// frame: on `main` every card misses its cache on every frame, since the
    /// key includes where it was.
    ///
    /// `cargo test -p gpui --release --lib cached_view_tests::scrolling_frame_cost -- --ignored --nocapture`
    #[crate::test]
    #[ignore = "prints timings; run by hand"]
    fn scrolling_frame_cost(cx: &mut TestAppContext) {
        let scroll = Rc::new(Cell::new(0.));
        let renders = Rc::new(Cell::new(0));
        let window = cx.add_window({
            let scroll = scroll.clone();
            let renders = renders.clone();
            move |_, cx| ScrollingList {
                cards: (0..100)
                    .map(|_| {
                        cx.new(|_| DeepCard {
                            items: 40,
                            depth: 8,
                            renders: renders.clone(),
                        })
                    })
                    .collect(),
                scroll,
            }
        });
        let window = AnyWindowHandle::from(window);
        let mut frame = |cx: &mut TestAppContext| {
            scroll.set(scroll.get() + 3.);
            cx.update_window(window, |root, window, cx| {
                root.downcast::<ScrollingList>()
                    .unwrap()
                    .update(cx, |_, cx| cx.notify());
                let started = std::time::Instant::now();
                window.draw(cx).clear(cx);
                started.elapsed()
            })
            .unwrap()
        };
        for _ in 0..5 {
            frame(cx);
        }
        renders.set(0);
        let mut samples: Vec<_> = (0..60).map(|_| frame(cx)).collect();
        samples.sort();
        let median = samples[samples.len() / 2];
        eprintln!(
            "8 visible cards x (40 items x 8 deep) scrolled 3px per frame: median frame {:.2} ms, {} card renders over 60 frames",
            median.as_secs_f64() * 1e3,
            renders.get(),
        );
    }

    #[crate::test]
    fn cached_view_is_reused_where_it_moves_to(cx: &mut TestAppContext) {
        let renders = Rc::new(Cell::new(0));
        let scroll = Rc::new(Cell::new(0.));
        let window = cx.add_window({
            let renders = renders.clone();
            let scroll = scroll.clone();
            move |_, cx| Viewport {
                card: cx.new(|_| Card { renders }),
                scroll,
            }
        });
        let window = AnyWindowHandle::from(window);
        // Opening the window drew it once.
        assert_eq!(renders.get(), 1);
        // The viewport re-renders (it moved the card); the card itself is
        // only re-rendered when it is notified.
        let draw = |cx: &mut TestAppContext| {
            cx.update_window(window, |root, window, cx| {
                root.downcast::<Viewport>()
                    .unwrap()
                    .update(cx, |_, cx| cx.notify());
                window.draw(cx).clear(cx)
            })
            .unwrap();
        };

        draw(cx);
        assert_eq!(renders.get(), 1);
        let at_top = quads(cx, window);
        assert_eq!(
            at_top,
            vec![(0., 0., 20., 20.), (0., 0., 100., 100.)],
            "the card and its inner square, at the top"
        );
        let hitboxes_at_top = hitboxes(cx, window);
        assert!(!hitboxes_at_top.is_empty());

        // Scrolling moves the card; the view is not dirty, so the previous
        // frame's records are reused, moved down.
        scroll.set(50.);
        draw(cx);
        assert_eq!(renders.get(), 1, "the card was not rendered again");
        assert_eq!(
            quads(cx, window),
            vec![(0., 50., 20., 20.), (0., 50., 100., 100.)],
            "the reused quads moved with the card"
        );
        assert_eq!(
            hitboxes(cx, window),
            hitboxes_at_top
                .iter()
                .map(|(y, h)| (y + 50., *h))
                .collect::<Vec<_>>(),
            "the reused hitboxes moved with the card"
        );

        // Partly out of the viewport: still reused, and clipped by the
        // viewport at the new position.
        scroll.set(150.);
        draw(cx);
        assert_eq!(renders.get(), 1);
        assert_eq!(
            quads(cx, window),
            vec![(0., 150., 20., 20.), (0., 150., 100., 50.)],
            "the reused quads are clipped by the viewport where it now cuts them"
        );

        // A recording made while clipped is missing what fell outside, so it
        // is not reused anywhere else: moving back renders the card again.
        scroll.set(0.);
        draw(cx);
        assert_eq!(renders.get(), 2, "a clipped recording is not moved");
        assert_eq!(quads(cx, window), at_top);

        // Notifying the entity always renders it again, wherever it is.
        cx.update_window(window, |root, _, cx| {
            let card = root.downcast::<Viewport>().unwrap().read(cx).card.clone();
            card.update(cx, |_, cx| cx.notify());
        })
        .unwrap();
        draw(cx);
        assert_eq!(renders.get(), 3);
    }
}
