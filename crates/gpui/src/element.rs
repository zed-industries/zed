//! Elements are the workhorses of GPUI. They are responsible for laying out and painting all of
//! the contents of a window. Elements form a tree and are laid out according to the web layout
//! standards as implemented by [taffy](https://github.com/DioxusLabs/taffy). Most of the time,
//! you won't need to interact with this module or these APIs directly. Elements provide their
//! own APIs and GPUI, or other element implementation, uses the APIs in this module to convert
//! that element tree into the pixels you see on the screen.
//!
//! # Element Basics
//!
//! Elements are constructed by calling [`Render::render()`] on the root view of the window,
//! which recursively constructs the element tree from the current state of the application,.
//! These elements are then laid out by Taffy, and painted to the screen according to their own
//! implementation of [`Element::paint()`]. Before the start of the next frame, the entire element
//! tree and any callbacks they have registered with GPUI are dropped and the process repeats.
//!
//! But some state is too simple and voluminous to store in every view that needs it, e.g.
//! whether a hover has been started or not. For this, GPUI provides the [`Element::PrepaintState`], associated type.
//!
//! # Implementing your own elements
//!
//! Elements are intended to be the low level, imperative API to GPUI. They are responsible for upholding,
//! or breaking, GPUI's features as they deem necessary. As an example, most GPUI elements are expected
//! to stay in the bounds that their parent element gives them. But with [`Window::with_content_mask`],
//! you can ignore this restriction and paint anywhere inside of the window's bounds. This is useful for overlays
//! and popups and anything else that shows up 'on top' of other elements.
//! With great power, comes great responsibility.
//!
//! However, most of the time, you won't need to implement your own elements. GPUI provides a number of
//! elements that should cover most common use cases out of the box and it's recommended that you use those
//! to construct `components`, using the [`RenderOnce`] trait and the `#[derive(IntoElement)]` macro. Only implement
//! elements when you need to take manual control of the layout and painting process, such as when using
//! your own custom layout algorithm or rendering a code editor.

#[cfg(any(feature = "inspector", debug_assertions))]
use crate::InspectorElementPath;
use crate::{
    A11ySubtreeBuilder, App, ArenaBox, AvailableSpace, Bounds, Context, DispatchNodeId, ElementId,
    FocusHandle, InspectorElementId, LayoutId, Pixels, Point, Size, Style, Window,
    util::FluentBuilder, window::with_element_arena,
};
use std::{
    any::Any,
    fmt::{self, Debug, Display},
    mem, panic,
    sync::Arc,
};

/// Implemented by types that participate in laying out and painting the contents of a window.
/// Elements form a tree and are laid out according to web-based layout rules, as implemented by Taffy.
/// You can create custom elements by implementing this trait, see the module-level documentation
/// for more details.
pub trait Element: 'static + IntoElement {
    /// The type of state returned from [`Element::request_layout`]. A mutable reference to this state is subsequently
    /// provided to [`Element::prepaint`] and [`Element::paint`].
    type RequestLayoutState: 'static;

    /// The type of state returned from [`Element::prepaint`]. A mutable reference to this state is subsequently
    /// provided to [`Element::paint`].
    type PrepaintState: 'static;

    /// If this element has a unique identifier, return it here. This is used to track elements across frames, and
    /// will cause a GlobalElementId to be passed to the request_layout, prepaint, and paint methods.
    ///
    /// The global id can in turn be used to access state that's connected to an element with the same id across
    /// frames. This id must be unique among children of the first containing element with an id.
    fn id(&self) -> Option<ElementId>;

    /// Source location where this element was constructed, used to disambiguate elements in the
    /// inspector and navigate to their source code.
    fn source_location(&self) -> Option<&'static panic::Location<'static>>;

    /// Before an element can be painted, we need to know where it's going to be and how big it is.
    /// Use this method to request a layout from Taffy and initialize the element's state.
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState);

    /// After laying out an element, we need to commit its bounds to the current frame for hitbox
    /// purposes. The state argument is the same state that was returned from [`Element::request_layout()`].
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState;

    /// Once layout has been completed, this method will be called to paint the element to the screen.
    /// The state argument is the same state that was returned from [`Element::request_layout()`].
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    );

    /// Returns the accessible role for this element, if any.
    /// Elements that return `None` are not included in the accessibility tree.
    ///
    /// Note: inclusion in accessibility tree requires non-`None` [`id`][Element::id].
    ///
    /// See the [accessibility guide](crate::_accessibility) for an overview.
    fn a11y_role(&self) -> Option<accesskit::Role> {
        None
    }

    /// Write accessibility properties to the given node.
    /// Called only when `a11y_role()` returns `Some`.
    ///
    /// See the [accessibility guide](crate::_accessibility) for an overview.
    fn write_a11y_info(&self, _node: &mut accesskit::Node) {}

    /// Add synthetic child nodes to an [`Element`] that has an
    /// [`.id()`][Element::id] and a [`.role()`][Element::a11y_role].
    ///
    /// Some elements may want to inject accessibility nodes that do not
    /// correspond to any GPUI element. For example, a custom text field element
    /// may want to inject synthetic child nodes for the text content.
    ///
    /// See [Synthetic children](crate::_accessibility#synthetic-children) in
    /// the accessibility guide for more detail.
    fn a11y_synthetic_children(
        &mut self,
        _prepaint: &mut Self::PrepaintState,
        _builder: &mut A11ySubtreeBuilder,
    ) {
    }

    /// Convert this element into a dynamically-typed [`AnyElement`].
    fn into_any(self) -> AnyElement {
        AnyElement::new(self)
    }
}

/// Implemented by any type that can be converted into an element.
pub trait IntoElement: Sized {
    /// The specific type of element into which the implementing type is converted.
    /// Useful for converting other types into elements automatically, like Strings
    type Element: Element;

    /// Convert self into a type that implements [`Element`].
    fn into_element(self) -> Self::Element;

    /// Convert self into a dynamically-typed [`AnyElement`].
    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }
}

impl<T: IntoElement> FluentBuilder for T {}

/// An object that can be drawn to the screen. This is the trait that distinguishes "views" from
/// other entities. Views are `Entity`'s which `impl Render` and drawn to the screen.
pub trait Render: 'static + Sized {
    /// Render this view into an element tree.
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement;
}

impl Render for Empty {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// You can derive [`IntoElement`] on any type that implements this trait.
/// It is used to construct reusable `components` out of plain data. Think of
/// components as a recipe for a certain pattern of elements. RenderOnce allows
/// you to invoke this pattern, without breaking the fluent builder pattern of
/// the element APIs.
pub trait RenderOnce: 'static {
    /// Render this component into an element tree. Note that this method
    /// takes ownership of self, as compared to [`Render::render()`] method
    /// which takes a mutable reference.
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement;
}

/// This is a helper trait to provide a uniform interface for constructing elements that
/// can accept any number of any kind of child elements
pub trait ParentElement {
    /// Extend this element's children with the given child elements.
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>);

    /// Add a single child element to this element.
    fn child(mut self, child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.extend(std::iter::once(child.into_any_element()));
        self
    }

    /// Add multiple child elements to this element.
    fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.extend(children.into_iter().map(|child| child.into_any_element()));
        self
    }
}

/// A globally unique identifier for an element, used to track state across frames.
///
/// Identity and hashing are path-based, including across windows. The path is
/// owned so an ID remains readable after its element or window is removed.
#[derive(Clone)]
pub struct GlobalElementId {
    handle: InternedHandle,
    ids: Arc<[ElementId]>,
    hash: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub(crate) struct InternedHandle {
    interner_id: u64,
    index: u32,
    generation: u32,
}

#[cfg(test)]
impl InternedHandle {
    pub(crate) fn for_test(index: u32, generation: u32) -> Self {
        Self {
            interner_id: 0,
            index,
            generation,
        }
    }
}

struct InternedPath {
    parent: InternedHandle,
    id: ElementId,
    path: Arc<[ElementId]>,
    hash: u64,
    last_used: u64,
    children: u32,
}

struct PathSlot {
    generation: u32,
    node: Option<InternedPath>,
}

pub(crate) struct ElementIdInterner {
    interner_id: u64,
    epoch: u64,
    nodes: Vec<PathSlot>,
    paths: collections::FxHashMap<(InternedHandle, ElementId), InternedHandle>,
    free: Vec<u32>,
}

impl Default for ElementIdInterner {
    fn default() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_INTERNER_ID: AtomicU64 = AtomicU64::new(1);
        let interner_id = NEXT_INTERNER_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
            .expect("element id interner identities exhausted");
        let root = InternedHandle {
            interner_id,
            index: 0,
            generation: 0,
        };
        Self {
            interner_id,
            epoch: 0,
            nodes: vec![PathSlot {
                generation: 0,
                node: Some(InternedPath {
                    parent: root,
                    id: ElementId::Integer(0),
                    path: Arc::from([]),
                    hash: EMPTY_PATH_HASH,
                    last_used: 0,
                    children: 0,
                }),
            }],
            paths: collections::FxHashMap::default(),
            free: Vec::new(),
        }
    }
}

impl ElementIdInterner {
    pub(crate) fn root(&self) -> InternedHandle {
        InternedHandle {
            interner_id: self.interner_id,
            index: 0,
            generation: 0,
        }
    }

    pub(crate) fn intern(
        &mut self,
        parent: InternedHandle,
        id: ElementId,
        path: &[ElementId],
    ) -> InternedHandle {
        let key = (parent, id);
        if let Some(&handle) = self.paths.get(&key) {
            self.nodes[handle.index as usize]
                .node
                .as_mut()
                .expect("interned path is live")
                .last_used = self.epoch;
            return handle;
        }

        let parent_node = self.nodes[parent.index as usize]
            .node
            .as_mut()
            .expect("interned parent is live");
        let hash = extend_path_hash(parent_node.hash, &key.1);
        parent_node.children += 1;
        let node = InternedPath {
            parent,
            id: key.1.clone(),
            path: Arc::from(path),
            hash,
            last_used: self.epoch,
            children: 0,
        };
        let index = if let Some(index) = self.free.pop() {
            self.nodes[index as usize].node = Some(node);
            index
        } else {
            let index = u32::try_from(self.nodes.len()).expect("element id interner is full");
            self.nodes.push(PathSlot {
                generation: 0,
                node: Some(node),
            });
            index
        };
        let handle = InternedHandle {
            interner_id: self.interner_id,
            index,
            generation: self.nodes[index as usize].generation,
        };
        self.paths.insert(key, handle);
        handle
    }

    pub(crate) fn global_id(&self, handle: InternedHandle) -> GlobalElementId {
        let slot = &self.nodes[handle.index as usize];
        debug_assert_eq!(handle.interner_id, self.interner_id);
        debug_assert_eq!(slot.generation, handle.generation);
        let node = slot.node.as_ref().expect("interned path is live");
        GlobalElementId {
            handle,
            ids: node.path.clone(),
            hash: node.hash,
        }
    }

    pub(crate) fn resolve(&mut self, id: &GlobalElementId) -> InternedHandle {
        if id.handle.interner_id == self.interner_id
            && self
                .nodes
                .get(id.handle.index as usize)
                .is_some_and(|slot| slot.generation == id.handle.generation && slot.node.is_some())
        {
            self.mark_used(id.handle);
            return id.handle;
        }
        let mut parent = self.root();
        for (index, element_id) in id.ids.iter().enumerate() {
            parent = self.intern(parent, element_id.clone(), &id.ids[..=index]);
        }
        parent
    }

    pub(crate) fn mark_used(&mut self, mut handle: InternedHandle) {
        loop {
            let slot = &mut self.nodes[handle.index as usize];
            debug_assert_eq!(handle.interner_id, self.interner_id);
            debug_assert_eq!(handle.generation, slot.generation);
            let node = slot.node.as_mut().expect("interned path is live");
            if node.last_used == self.epoch {
                break;
            }
            node.last_used = self.epoch;
            if handle.index == 0 {
                break;
            }
            handle = node.parent;
        }
    }

    pub(crate) fn finish_frame(&mut self) {
        // Two frame epochs cover both frame-state buffers, including IDs used only
        // during request_layout. A removed leaf can then release its ancestors in
        // the same sweep, even when recycling has changed their index order.
        for index in 1..self.nodes.len() {
            let mut index = index;
            while index != 0 {
                let slot = &mut self.nodes[index];
                let Some(node) = slot.node.as_ref() else {
                    break;
                };
                if node.children != 0 || self.epoch.saturating_sub(node.last_used) <= 1 {
                    break;
                }
                let node = slot.node.take().expect("reclaimable path is live");
                self.paths.remove(&(node.parent, node.id));
                // Never wrap a generation: an exhausted slot is retired instead.
                if let Some(generation) = slot.generation.checked_add(1) {
                    slot.generation = generation;
                    self.free.push(index as u32);
                }
                index = node.parent.index as usize;
                self.nodes[index]
                    .node
                    .as_mut()
                    .expect("interned parent is live")
                    .children -= 1;
            }
        }
        self.epoch = self
            .epoch
            .checked_add(1)
            .expect("element id epochs exhausted");
    }
}

/// The hash of an empty id path; every path hash is folded from it with
/// [`extend_path_hash`], one id at a time, so the window can keep the hash
/// of each prefix of its id stack and never re-hash a path.
pub(crate) const EMPTY_PATH_HASH: u64 = 0;

/// The hash of the path `prefix` + `id`, given the hash of `prefix`.
pub(crate) fn extend_path_hash(prefix: u64, id: &ElementId) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = collections::FxHasher::with_seed(prefix as usize);
    id.hash(&mut hasher);
    hasher.finish()
}

impl GlobalElementId {
    #[cfg(test)]
    pub(crate) fn new(ids: &[ElementId]) -> Self {
        let hash = ids.iter().fold(EMPTY_PATH_HASH, extend_path_hash);
        Self::with_hash(ids, hash)
    }

    #[cfg(test)]
    /// `ids` with its hash already known, as the window's id stack keeps it.
    pub(crate) fn with_hash(ids: &[ElementId], hash: u64) -> Self {
        debug_assert_eq!(hash, ids.iter().fold(EMPTY_PATH_HASH, extend_path_hash));
        Self {
            handle: InternedHandle::default(),
            ids: Arc::from(ids),
            hash,
        }
    }

    pub(crate) fn accesskit_node_id(&self) -> accesskit::NodeId {
        accesskit::NodeId(self.hash)
    }
}

impl Default for GlobalElementId {
    fn default() -> Self {
        Self {
            handle: InternedHandle::default(),
            ids: Arc::from([]),
            hash: EMPTY_PATH_HASH,
        }
    }
}

impl std::ops::Deref for GlobalElementId {
    type Target = [ElementId];

    fn deref(&self) -> &Self::Target {
        &self.ids
    }
}

impl PartialEq for GlobalElementId {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
            && ((self.handle.interner_id != 0 && self.handle == other.handle)
                || Arc::ptr_eq(&self.ids, &other.ids)
                || self.ids == other.ids)
    }
}

impl Eq for GlobalElementId {}

impl Debug for GlobalElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GlobalElementId")
            .field("ids", &self.ids)
            .field("hash", &self.hash)
            .finish()
    }
}

impl std::hash::Hash for GlobalElementId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl Display for GlobalElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, element_id) in self.ids.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", element_id)?;
        }
        Ok(())
    }
}

trait ElementObject {
    fn inner_element(&mut self) -> &mut dyn Any;

    fn request_layout(&mut self, window: &mut Window, cx: &mut App) -> LayoutId;

    fn prepaint(&mut self, window: &mut Window, cx: &mut App);

    fn paint(&mut self, window: &mut Window, cx: &mut App);

    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels>;
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
pub struct Drawable<E: Element> {
    /// The drawn element.
    pub element: E,
    phase: ElementDrawPhase<E::RequestLayoutState, E::PrepaintState>,
}

#[derive(Default)]
enum ElementDrawPhase<RequestLayoutState, PrepaintState> {
    #[default]
    Start,
    RequestLayout {
        layout_id: LayoutId,
        global_id: Option<GlobalElementId>,
        inspector_id: Option<InspectorElementId>,
        request_layout: RequestLayoutState,
    },
    LayoutComputed {
        layout_id: LayoutId,
        global_id: Option<GlobalElementId>,
        inspector_id: Option<InspectorElementId>,
        available_space: Size<AvailableSpace>,
        request_layout: RequestLayoutState,
    },
    Prepaint {
        node_id: DispatchNodeId,
        global_id: Option<GlobalElementId>,
        inspector_id: Option<InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: RequestLayoutState,
        prepaint: PrepaintState,
    },
    Painted,
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
impl<E: Element> Drawable<E> {
    pub(crate) fn new(element: E) -> Self {
        Drawable {
            element,
            phase: ElementDrawPhase::Start,
        }
    }

    fn request_layout(&mut self, window: &mut Window, cx: &mut App) -> LayoutId {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::Start => {
                let global_id = self
                    .element
                    .id()
                    .map(|element_id| prepare_element_id(element_id, window));

                let inspector_id;
                #[cfg(any(feature = "inspector", debug_assertions))]
                {
                    inspector_id = if window.inspector_enabled() {
                        self.element
                            .source_location()
                            .map(|source| prepare_inspector_id(source, window))
                    } else {
                        None
                    };
                }
                #[cfg(not(any(feature = "inspector", debug_assertions)))]
                {
                    inspector_id = None;
                }

                #[cfg(any(test, feature = "bench-support"))]
                {
                    window.view_reuse_stats.phase_element_visits[0] += 1;
                }
                let (layout_id, request_layout) = self.element.request_layout(
                    global_id.as_ref(),
                    inspector_id.as_ref(),
                    window,
                    cx,
                );

                if global_id.is_some() {
                    window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::RequestLayout {
                    layout_id,
                    global_id,
                    inspector_id,
                    request_layout,
                };
                layout_id
            }
            _ => panic!("must call request_layout only once"),
        }
    }

    pub(crate) fn prepaint(&mut self, window: &mut Window, cx: &mut App) {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::RequestLayout {
                layout_id,
                global_id,
                inspector_id,
                mut request_layout,
            }
            | ElementDrawPhase::LayoutComputed {
                layout_id,
                global_id,
                inspector_id,
                mut request_layout,
                ..
            } => {
                if let Some(element_id) = self.element.id() {
                    window
                        .element_id_stack
                        .push(element_id, &mut window.element_id_interner);
                    debug_assert_eq!(&**global_id.as_ref().unwrap(), &*window.element_id_stack);
                }

                let bounds = window.layout_bounds(layout_id);
                let mut pushed_a11y_node = false;
                if window.a11y.is_active() {
                    if let Some(global_id) = global_id.as_ref() {
                        if let Some(role) = self.element.a11y_role() {
                            let node_id = global_id.accesskit_node_id();
                            let mut node = accesskit::Node::new(role);
                            let scale = window.scale_factor();
                            node.set_bounds(accesskit::Rect {
                                x0: (bounds.origin.x.0 * scale) as f64,
                                y0: (bounds.origin.y.0 * scale) as f64,
                                x1: ((bounds.origin.x.0 + bounds.size.width.0) * scale) as f64,
                                y1: ((bounds.origin.y.0 + bounds.size.height.0) * scale) as f64,
                            });
                            self.element.write_a11y_info(&mut node);
                            window.a11y.node_bounds.insert(node_id, bounds);
                            pushed_a11y_node = window.a11y.nodes.push(node_id, node);
                            #[cfg(debug_assertions)]
                            if pushed_a11y_node {
                                let view = window
                                    .a11y
                                    .view_type_names
                                    .get(&window.current_view())
                                    .copied();
                                let source_location = self.element.source_location();
                                window.a11y.nodes.record_node_info(
                                    node_id,
                                    crate::window::a11y::debug::NodeDebugInfo {
                                        synthetic: false,
                                        view,
                                        element_id: global_id.last().map(|id| format!("{id:?}")),
                                        source_location,
                                    },
                                );
                            }
                        }
                    }
                }

                let node_id = window.next_frame.dispatch_tree.push_node();
                #[cfg(any(test, feature = "bench-support"))]
                {
                    window.view_reuse_stats.phase_element_visits[1] += 1;
                }
                let mut prepaint = self.element.prepaint(
                    global_id.as_ref(),
                    inspector_id.as_ref(),
                    bounds,
                    &mut request_layout,
                    window,
                    cx,
                );
                window.next_frame.dispatch_tree.pop_node();

                if pushed_a11y_node {
                    if let Some(global_id) = global_id.as_ref() {
                        #[cfg(debug_assertions)]
                        let creator = crate::window::a11y::debug::NodeCreator {
                            view: window
                                .a11y
                                .view_type_names
                                .get(&window.current_view())
                                .copied(),
                            element_id: global_id.last().map(|id| format!("{id:?}")),
                            source_location: self.element.source_location(),
                        };
                        let mut builder = A11ySubtreeBuilder::new(
                            global_id.accesskit_node_id(),
                            &mut window.a11y.nodes,
                        );
                        #[cfg(debug_assertions)]
                        {
                            builder = builder.with_creator(creator);
                        }
                        self.element
                            .a11y_synthetic_children(&mut prepaint, &mut builder);
                    }
                    window.a11y.nodes.pop();
                }

                if global_id.is_some() {
                    window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::Prepaint {
                    node_id,
                    global_id,
                    inspector_id,
                    bounds,
                    request_layout,
                    prepaint,
                };
            }
            _ => panic!("must call request_layout before prepaint"),
        }
    }

    pub(crate) fn paint(
        &mut self,
        window: &mut Window,
        cx: &mut App,
    ) -> (E::RequestLayoutState, E::PrepaintState) {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::Prepaint {
                node_id,
                global_id,
                inspector_id,
                bounds,
                mut request_layout,
                mut prepaint,
                ..
            } => {
                if let Some(element_id) = self.element.id() {
                    window
                        .element_id_stack
                        .push(element_id, &mut window.element_id_interner);
                    debug_assert_eq!(&**global_id.as_ref().unwrap(), &*window.element_id_stack);
                }

                window.next_frame.dispatch_tree.set_active_node(node_id);
                #[cfg(any(test, feature = "bench-support"))]
                {
                    window.view_reuse_stats.phase_element_visits[2] += 1;
                }
                self.element.paint(
                    global_id.as_ref(),
                    inspector_id.as_ref(),
                    bounds,
                    &mut request_layout,
                    &mut prepaint,
                    window,
                    cx,
                );

                if global_id.is_some() {
                    window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::Painted;
                (request_layout, prepaint)
            }
            _ => panic!("must call prepaint before paint"),
        }
    }

    pub(crate) fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementDrawPhase::Start) {
            self.request_layout(window, cx);
        }

        let layout_id = match mem::take(&mut self.phase) {
            ElementDrawPhase::RequestLayout {
                layout_id,
                global_id,
                inspector_id,
                request_layout,
            } => {
                window.compute_layout(layout_id, available_space, cx);
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    global_id,
                    inspector_id,
                    available_space,
                    request_layout,
                };
                layout_id
            }
            ElementDrawPhase::LayoutComputed {
                layout_id,
                global_id,
                inspector_id,
                available_space: prev_available_space,
                request_layout,
            } => {
                if available_space != prev_available_space {
                    window.compute_layout(layout_id, available_space, cx);
                }
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    global_id,
                    inspector_id,
                    available_space,
                    request_layout,
                };
                layout_id
            }
            _ => panic!("cannot measure after painting"),
        };

        window.layout_bounds(layout_id).size
    }
}

impl<E> ElementObject for Drawable<E>
where
    E: Element,
    E::RequestLayoutState: 'static,
{
    fn inner_element(&mut self) -> &mut dyn Any {
        &mut self.element
    }

    #[inline]
    fn request_layout(&mut self, window: &mut Window, cx: &mut App) -> LayoutId {
        Drawable::request_layout(self, window, cx)
    }

    #[inline]
    fn prepaint(&mut self, window: &mut Window, cx: &mut App) {
        Drawable::prepaint(self, window, cx);
    }

    #[inline]
    fn paint(&mut self, window: &mut Window, cx: &mut App) {
        Drawable::paint(self, window, cx);
    }

    #[inline]
    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        Drawable::layout_as_root(self, available_space, window, cx)
    }
}

/// A dynamically typed element that can be used to store any element type.
pub struct AnyElement(ArenaBox<dyn ElementObject>);

impl AnyElement {
    pub(crate) fn new<E>(element: E) -> Self
    where
        E: 'static + Element,
        E::RequestLayoutState: Any,
    {
        let element = with_element_arena(|arena| arena.alloc(|| Drawable::new(element)))
            .map(|element| element as &mut dyn ElementObject);
        AnyElement(element)
    }

    /// Attempt to downcast a reference to the boxed element to a specific type.
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.inner_element().downcast_mut::<T>()
    }

    /// Request the layout ID of the element stored in this `AnyElement`.
    /// Used for laying out child elements in a parent element.
    pub fn request_layout(&mut self, window: &mut Window, cx: &mut App) -> LayoutId {
        self.0.request_layout(window, cx)
    }

    /// Prepares the element to be painted by storing its bounds, giving it a chance to draw hitboxes and
    /// request autoscroll before the final paint pass is confirmed.
    pub fn prepaint(&mut self, window: &mut Window, cx: &mut App) -> Option<FocusHandle> {
        let focus_assigned = window.next_frame.focus.is_some();

        self.0.prepaint(window, cx);

        if !focus_assigned && let Some(focus_id) = window.next_frame.focus {
            return FocusHandle::for_id(focus_id, &cx.focus_handles);
        }

        None
    }

    /// Paints the element stored in this `AnyElement`.
    pub fn paint(&mut self, window: &mut Window, cx: &mut App) {
        self.0.paint(window, cx);
    }

    /// Performs layout for this element within the given available space and returns its size.
    pub fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        self.0.layout_as_root(available_space, window, cx)
    }

    /// Prepaints this element at the given absolute origin.
    /// If any element in the subtree beneath this element is focused, its FocusHandle is returned.
    pub fn prepaint_at(
        &mut self,
        origin: Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<FocusHandle> {
        window.with_absolute_element_offset(origin, |window| self.prepaint(window, cx))
    }

    /// Performs layout on this element in the available space, then prepaints it at the given absolute origin.
    /// If any element in the subtree beneath this element is focused, its FocusHandle is returned.
    pub fn prepaint_as_root(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<FocusHandle> {
        self.layout_as_root(available_space, window, cx);
        window.with_absolute_element_offset(origin, |window| self.prepaint(window, cx))
    }
}

impl Element for AnyElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = self.request_layout(window, cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.paint(window, cx);
    }
}

impl IntoElement for AnyElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }

    fn into_any_element(self) -> AnyElement {
        self
    }
}

/// The empty element, which renders nothing.
pub struct Empty;

impl IntoElement for Empty {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Empty {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (
            window.request_layout(
                Style {
                    display: crate::Display::None,
                    ..Default::default()
                },
                None,
                cx,
            ),
            (),
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _state: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }
}

#[inline(never)]
fn prepare_element_id(element_id: ElementId, window: &mut Window) -> GlobalElementId {
    window
        .element_id_stack
        .push(element_id, &mut window.element_id_interner);
    window
        .element_id_stack
        .global_id(&window.element_id_interner)
}

#[cfg(any(feature = "inspector", debug_assertions))]
#[inline(never)]
fn prepare_inspector_id(
    source: &'static panic::Location<'static>,
    window: &mut Window,
) -> InspectorElementId {
    let path = InspectorElementPath {
        global_id: window
            .element_id_stack
            .global_id(&window.element_id_interner),
        source_location: source,
    };
    window.build_inspector_element_id(path)
}

#[cfg(test)]
mod global_element_id_tests {
    use super::*;
    use std::hash::{Hash, Hasher};

    fn path(depth: usize, leaf: u64) -> Vec<ElementId> {
        let mut ids: Vec<ElementId> = (0..depth as u64)
            .map(|level| ElementId::NamedInteger("some-container-element".into(), level))
            .collect();
        ids.push(ElementId::NamedInteger("leaf".into(), leaf));
        ids
    }

    fn fx_hash(value: &impl Hash) -> u64 {
        let mut hasher = collections::FxHasher::default();
        value.hash(&mut hasher);
        hasher.finish()
    }

    fn intern_path(interner: &mut ElementIdInterner, path: &[ElementId]) -> GlobalElementId {
        let mut handle = interner.root();
        for (index, id) in path.iter().enumerate() {
            handle = interner.intern(handle, id.clone(), &path[..=index]);
        }
        interner.global_id(handle)
    }

    #[test]
    fn interned_paths_are_stable_across_frames_and_windows() {
        let mut interner = ElementIdInterner::default();
        let ids = path(8, 1);
        let first = intern_path(&mut interner, &ids);
        for _ in 0..10 {
            interner.finish_frame();
            let next = intern_path(&mut interner, &ids);
            assert_eq!(first.handle, next.handle);
            assert!(Arc::ptr_eq(&first.ids, &next.ids));
        }
        let different = intern_path(&mut interner, &path(8, 2));
        assert_ne!(first.handle, different.handle);
        assert_ne!(first, different);
        // Even a path-hash collision must not equate distinct paths.
        let mut collision = different;
        collision.hash = first.hash;
        assert_ne!(first, collision);

        let mut other_window = ElementIdInterner::default();
        let other = intern_path(&mut other_window, &ids);
        assert_ne!(first.handle, other.handle);
        assert_eq!(first, other);
        assert_eq!(fx_hash(&first), fx_hash(&other));
        assert_eq!(first.accesskit_node_id(), other.accesskit_node_id());
        assert_eq!(other_window.resolve(&first), other.handle);
        assert_ne!(first, intern_path(&mut other_window, &path(8, 2)));

        let expected = GlobalElementId::new(&ids);
        assert_eq!(first.to_string(), expected.to_string());
        assert_eq!(
            format!("{first:?}"),
            format!(
                "GlobalElementId {{ ids: {ids:?}, hash: {} }}",
                expected.hash
            )
        );
        assert_eq!(
            interner.global_id(interner.root()),
            GlobalElementId::default()
        );
    }

    #[test]
    fn recycled_handles_do_not_match_and_owned_paths_survive_reclamation() {
        let mut interner = ElementIdInterner::default();
        let original = intern_path(&mut interner, &[1_u64.into()]);
        let retained = original.clone();
        interner.finish_frame();
        interner.finish_frame();
        assert_eq!(interner.paths.len(), 1);
        interner.finish_frame();
        assert!(interner.paths.is_empty());
        assert_eq!(Arc::strong_count(&retained.ids), 2);
        assert_eq!(&*retained, &[1_u64.into()]);
        assert_eq!(retained.to_string(), "1");
        assert_eq!(original, retained);

        let next = intern_path(&mut interner, &[2_u64.into()]);
        assert_eq!(retained.handle.index, next.handle.index);
        assert_ne!(retained.handle.generation, next.handle.generation);
        assert_ne!(retained.handle, next.handle);
        assert_ne!(retained, next);
        let revived = interner.resolve(&retained);
        assert_ne!(revived, retained.handle);
        assert_eq!(interner.global_id(revived), retained);
    }

    #[test]
    fn interner_is_bounded_after_one_thousand_churn_frames() {
        let mut interner = ElementIdInterner::default();
        for frame in 0..1000_u64 {
            for item in 0..64_u64 {
                let ids = [
                    ElementId::Integer(frame * 64 + item),
                    ElementId::Integer(1),
                    ElementId::Integer(2),
                    ElementId::Integer(3),
                ];
                intern_path(&mut interner, &ids);
            }
            interner.finish_frame();
            assert!(interner.paths.len() <= 2 * 64 * 4);
            assert!(interner.nodes.len() <= 1 + 3 * 64 * 4);
            assert!(interner.paths.capacity() <= 2048);
        }
        interner.finish_frame();
        interner.finish_frame();
        assert!(interner.paths.is_empty());
        assert_eq!(interner.free.len(), interner.nodes.len() - 1);
    }

    #[test]
    fn reused_state_keeps_ancestors_live_without_reinterning() {
        let mut interner = ElementIdInterner::default();
        let original = intern_path(&mut interner, &path(32, 1));
        for _ in 0..1000 {
            interner.mark_used(original.handle);
            interner.finish_frame();
        }
        let next = intern_path(&mut interner, &path(32, 1));
        assert_eq!(original.handle, next.handle);
        assert!(Arc::ptr_eq(&original.ids, &next.ids));
        assert_eq!(interner.paths.len(), 33);
    }

    #[test]
    fn exhausted_generation_is_retired() {
        let mut interner = ElementIdInterner::default();
        let original = intern_path(&mut interner, &[1_u64.into()]);
        interner.nodes[original.handle.index as usize].generation = u32::MAX;
        interner.finish_frame();
        interner.finish_frame();
        interner.finish_frame();
        let next = intern_path(&mut interner, &[2_u64.into()]);
        assert_ne!(original.handle.index, next.handle.index);
    }

    #[test]
    fn equal_paths_are_equal_and_hash_alike() {
        let a = GlobalElementId::new(&path(8, 1));
        let b = GlobalElementId::new(&path(8, 1));
        let c = GlobalElementId::new(&path(8, 2));
        let shallower = GlobalElementId::new(&path(7, 1));
        assert_eq!(a, b);
        assert_eq!(fx_hash(&a), fx_hash(&b));
        assert_eq!(a.accesskit_node_id(), b.accesskit_node_id());
        assert_ne!(a, c);
        assert_ne!(a, shallower);
        assert_ne!(fx_hash(&a), fx_hash(&c));
        assert_eq!(&*a, path(8, 1).as_slice());
        assert_eq!(a.to_string(), b.to_string());
        assert_eq!(GlobalElementId::default(), GlobalElementId::new(&[]));
    }

    /// A window of `items` columns of `depth` nested stateful divs — the
    /// shape of a list of cards — drawn from scratch on every frame.
    struct DeepStatefulTree {
        items: usize,
        depth: usize,
    }

    impl crate::Render for DeepStatefulTree {
        fn render(&mut self, _: &mut Window, _: &mut crate::Context<Self>) -> impl IntoElement {
            use crate::{InteractiveElement as _, ParentElement as _, Styled as _, div, px};
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

    /// Frame cost of a tree of stateful elements: every one of them builds a
    /// `GlobalElementId` and looks its state up in two frames' maps.
    ///
    /// `cargo test -p gpui --release --lib global_element_id_tests::frame_cost -- --ignored --nocapture`
    #[crate::test]
    #[ignore = "prints timings; run by hand"]
    fn frame_cost(cx: &mut crate::TestAppContext) {
        use crate::AppContext as _;
        for (items, depth) in [(500, 4), (500, 12), (2000, 12)] {
            let window = cx.add_window(move |_, _| DeepStatefulTree { items, depth });
            let window = crate::AnyWindowHandle::from(window);
            let mut frame = |cx: &mut crate::TestAppContext| {
                cx.update_window(window, |_, window, cx| {
                    window.refresh();
                    let started = std::time::Instant::now();
                    window.draw(cx).clear(cx);
                    started.elapsed()
                })
                .unwrap()
            };
            for _ in 0..5 {
                frame(cx);
            }
            let mut samples: Vec<_> = (0..40).map(|_| frame(cx)).collect();
            samples.sort();
            let median = samples[samples.len() / 2];
            let stateful = items * (depth + 2);
            eprintln!(
                "{items} items x {depth} deep ({stateful} stateful elements): median frame {:.2} ms, {:.0} ns per stateful element",
                median.as_secs_f64() * 1e3,
                median.as_nanos() as f64 / stateful as f64,
            );
        }
    }

    /// `cargo test -p gpui --release --lib global_element_id_tests::hash_cost -- --ignored --nocapture`
    #[test]
    #[ignore = "prints timings; run by hand"]
    fn hash_cost() {
        let ids = path(11, 7);
        let id = GlobalElementId::new(&ids);
        let n = 1_000_000u64;
        let mut acc = 0u64;
        let started = std::time::Instant::now();
        for _ in 0..n {
            acc ^= fx_hash(&std::hint::black_box(ids.as_slice()));
        }
        let walk = started.elapsed();
        let started = std::time::Instant::now();
        for _ in 0..n {
            acc ^= fx_hash(std::hint::black_box(&id));
        }
        let cached = started.elapsed();
        let started = std::time::Instant::now();
        for _ in 0..n {
            acc ^= GlobalElementId::new(std::hint::black_box(&ids)).hash;
        }
        let build = started.elapsed();
        eprintln!(
            "12-deep path: hash by walking ids {:.1} ns, cached {:.1} ns, build (clone + hash once) {:.1} ns [{acc}]",
            walk.as_nanos() as f64 / n as f64,
            cached.as_nanos() as f64 / n as f64,
            build.as_nanos() as f64 / n as f64,
        );
    }
}
