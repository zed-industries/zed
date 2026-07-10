use crate::{
    ActiveTooltip, AnyView, App, Bounds, DispatchPhase, Element, ElementId, GlobalElementId,
    HighlightStyle, Hitbox, HitboxBehavior, InspectorElementId, IntoElement, LayoutId,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, SharedString, Size, TextOverflow,
    TextRun, TextStyle, TooltipId, TruncateFrom, WhiteSpace, Window, WrappedLine,
    WrappedLineLayout, px, register_tooltip_mouse_handlers, set_tooltip_on_window,
};
use anyhow::Context as _;
use gpui_util::ResultExt;
use itertools::Itertools;
use smallvec::SmallVec;
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    mem,
    ops::{Deref, DerefMut, Range},
    rc::Rc,
    sync::Arc,
};

/// An [`Element`] that renders text.
///
/// In general, [`Text`] objects should be created via the [`text`] macro:
/// ```rust
/// # use gpui::*;
/// # fn render() -> impl IntoElement {
/// div().child(text!("hello"))
/// # }
/// ```
/// ## IDs and Accessibility
///
/// [`Text`] elements have an ID. This ID is primarily used to produce nodes in
/// the accessibility tree, which allows the text to be visible to screen
/// readers and other assistive technologies.
///
/// This ID is stable across frames. If the same text, with the same ID, is
/// present in two consecutive frames, no updates are reported to the screen
/// reader. If the text changes, but the ID stays the same, then the screen
/// reader will be notified that a text node's content has changed. **However**,
/// if the ID changes, then the screen reader will be notified that a node has
/// been removed, and a new node has been added.
///
/// When using the [`text`] macro, each invocation of the macro will get a
/// unique ID, derived from its position in the source code (filename, line, and
/// column). For example:
/// ```rust
/// # use gpui::*;
/// let x = text!("hello");
/// let y = text!("hello");
/// // not equal, because different `text!` invocations produced them
/// assert_ne!(x.id(), y.id());
///
/// fn make_text(s: &str) -> Text { text!(s) }
/// let x = make_text("hello");
/// let y = make_text("hello");
/// // equal, because the same `text!` invocation produced them
/// assert_eq!(x.id(), y.id());
/// ```
/// When the contents of an invocation of [`text`] do not change, this
/// distinction is less relevant (with the caveat that you still need to take
/// care to ensure that duplicate IDs do not appear).
///
/// However, when a [`text`] invocation's argument *does* change, you should
/// consider whether this change should be reported as a node "updating its
/// contents", or an old node being destroyed and a new node being created.
#[derive(Debug, Clone)]
pub struct Text {
    id: Option<ElementId>,
    text: SharedString,
}

impl Text {
    /// Create a new [`Text`] element with a specific ID.
    ///
    /// If you want a unique ID to be assigned automatically, use the [`text`]
    /// macro. The docs for [`Text`] have more detail about choosing IDs.
    #[inline]
    pub const fn new(id: ElementId, text: SharedString) -> Self {
        Self { id: Some(id), text }
    }

    /// Create a new [`Text`] element that is inaccessible to screen readers.
    ///
    /// In order for text to be accessible to screen readers, it must have an ID
    /// provided. If you want text to be accessible, either use [`text`] to have
    /// an ID automatically assigned, or use [`Text::new`] to manually assign an
    /// ID.
    ///
    /// This function is intended for use inside custom UI components, where
    /// accessible properties may be set on parent containers.
    #[inline]
    pub const fn new_inaccessible(text: SharedString) -> Self {
        Self { id: None, text }
    }

    /// The ID of this [`Text`] element.
    #[inline]
    pub const fn id(&self) -> Option<&ElementId> {
        self.id.as_ref()
    }

    /// Produce a new [`Text`] with the given `id`.
    pub fn with_id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// The text that this [`Text`] element will display.
    #[inline]
    pub const fn text(&self) -> &SharedString {
        &self.text
    }
}

impl Deref for Text {
    type Target = SharedString;
    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.text
    }
}

/// Trivial hash function for the location information produced by the [`text`]
/// macro. Not covered by semver guarantees. Performance is not particularly
/// significant because it's only used on small strings in const contexts.
#[doc(hidden)]
pub const fn __hash_text_macro_location_unstable_do_not_use(s: &'static str) -> u64 {
    const BASIS: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    let bytes = s.as_bytes();
    let mut hash = BASIS;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(PRIME);
        i += 1;
    }
    hash
}

/// Create a new [`Text`] element.
///
/// ```rust
/// # use gpui::*;
/// let a = text!("hello");
/// let b = text!(id = "farewell-message", "hello");
///
/// ```
///
/// Text created with this macro is *accessible*. The macro generates an ID
/// based on the source location. See the docs for [`Text`] for a more in-depth
/// explanation of the significance of the ID of a [`Text`] element.
#[macro_export]
macro_rules! text {
    (id = $id:expr, $text:expr) => {{ $crate::Text::new($id.into(), $text.into()) }};
    ($text:expr) => {{
        const ID: &'static str = concat!(file!(), "/", line!(), ":", column!());
        const HASH: u64 = $crate::__hash_text_macro_location_unstable_do_not_use(ID);
        $crate::Text::new($crate::ElementId::Integer(HASH), $text.into())
    }};
}

impl IntoElement for Text {
    type Element = Self;
    #[inline]
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Text {
    type RequestLayoutState = TextLayout;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        self.id.clone()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn a11y_role(&self) -> Option<accesskit::Role> {
        if self.id.is_some() {
            Some(accesskit::Role::Label)
        } else {
            None
        }
    }

    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        node.set_value(self.text.to_string());
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        <SharedString as Element>::request_layout(&mut self.text, id, inspector_id, window, cx)
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        <SharedString as Element>::prepaint(
            &mut self.text,
            id,
            inspector_id,
            bounds,
            request_layout,
            window,
            cx,
        )
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        <SharedString as Element>::paint(
            &mut self.text,
            id,
            inspector_id,
            bounds,
            request_layout,
            prepaint,
            window,
            cx,
        );
    }
}

impl Element for &'static str {
    type RequestLayoutState = TextLayout;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
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
        let mut state = TextLayout::default();
        let layout_id = state.layout(SharedString::from(*self), None, window, cx);
        (layout_id, state)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        text_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        text_layout.prepaint(bounds, self)
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        text_layout: &mut TextLayout,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        text_layout.paint(self, window, cx)
    }
}

impl IntoElement for &'static str {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl IntoElement for String {
    type Element = SharedString;

    fn into_element(self) -> Self::Element {
        self.into()
    }
}

impl IntoElement for Cow<'static, str> {
    type Element = SharedString;

    fn into_element(self) -> Self::Element {
        self.into()
    }
}

impl Element for SharedString {
    type RequestLayoutState = TextLayout;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
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
        let mut state = TextLayout::default();
        let layout_id = state.layout(self.clone(), None, window, cx);
        (layout_id, state)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        text_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        text_layout.prepaint(bounds, self.as_ref())
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        text_layout: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        text_layout.paint(self.as_ref(), window, cx)
    }
}

impl IntoElement for SharedString {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// Renders text with runs of different styles.
///
/// Callers are responsible for setting the correct style for each run.
/// For text with a uniform style, you can usually avoid calling this constructor
/// and just pass text directly.
pub struct StyledText {
    text: SharedString,
    runs: Option<Vec<TextRun>>,
    delayed_highlights: Option<Vec<(Range<usize>, HighlightStyle)>>,
    delayed_font_family_overrides: Option<Vec<(Range<usize>, SharedString)>>,
    layout: TextLayout,
}

impl StyledText {
    /// Construct a new styled text element from the given string.
    pub fn new(text: impl Into<SharedString>) -> Self {
        StyledText {
            text: text.into(),
            runs: None,
            delayed_highlights: None,
            delayed_font_family_overrides: None,
            layout: TextLayout::default(),
        }
    }

    /// Get the layout for this element. This can be used to map indices to pixels and vice versa.
    pub fn layout(&self) -> &TextLayout {
        &self.layout
    }

    /// Set the styling attributes for the given text, as well as
    /// as any ranges of text that have had their style customized.
    pub fn with_default_highlights(
        mut self,
        default_style: &TextStyle,
        highlights: impl IntoIterator<Item = (Range<usize>, HighlightStyle)>,
    ) -> Self {
        debug_assert!(
            self.delayed_highlights.is_none(),
            "Can't use `with_default_highlights` and `with_highlights`"
        );
        let runs = Self::compute_runs(&self.text, default_style, highlights);
        self.with_runs(runs)
    }

    /// Set the styling attributes for the given text, as well as
    /// as any ranges of text that have had their style customized.
    pub fn with_highlights(
        mut self,
        highlights: impl IntoIterator<Item = (Range<usize>, HighlightStyle)>,
    ) -> Self {
        debug_assert!(
            self.runs.is_none(),
            "Can't use `with_highlights` and `with_default_highlights`"
        );
        self.delayed_highlights = Some(
            highlights
                .into_iter()
                .inspect(|(run, _)| {
                    debug_assert!(self.text.is_char_boundary(run.start));
                    debug_assert!(self.text.is_char_boundary(run.end));
                })
                .collect::<Vec<_>>(),
        );
        self
    }

    fn compute_runs(
        text: &str,
        default_style: &TextStyle,
        highlights: impl IntoIterator<Item = (Range<usize>, HighlightStyle)>,
    ) -> Vec<TextRun> {
        let mut runs = Vec::new();
        let mut ix = 0;
        for (range, highlight) in highlights {
            if ix < range.start {
                debug_assert!(text.is_char_boundary(range.start));
                runs.push(default_style.clone().to_run(range.start - ix));
            }
            debug_assert!(text.is_char_boundary(range.end));
            runs.push(
                default_style
                    .clone()
                    .highlight(highlight)
                    .to_run(range.len()),
            );
            ix = range.end;
        }
        if ix < text.len() {
            runs.push(default_style.to_run(text.len() - ix));
        }
        runs
    }

    /// Override the font family for specific byte ranges of the text.
    ///
    /// This is resolved lazily at layout time, so the overrides are applied
    /// on top of the inherited text style from the parent element.
    /// Can be combined with [`with_highlights`](Self::with_highlights).
    ///
    /// The overrides must be sorted by range start and non-overlapping.
    /// Each override range must fall on character boundaries.
    pub fn with_font_family_overrides(
        mut self,
        overrides: impl IntoIterator<Item = (Range<usize>, SharedString)>,
    ) -> Self {
        self.delayed_font_family_overrides = Some(
            overrides
                .into_iter()
                .inspect(|(range, _)| {
                    debug_assert!(self.text.is_char_boundary(range.start));
                    debug_assert!(self.text.is_char_boundary(range.end));
                })
                .collect(),
        );
        self
    }

    fn apply_font_family_overrides(
        runs: &mut [TextRun],
        overrides: &[(Range<usize>, SharedString)],
    ) {
        let mut byte_offset = 0;
        let mut override_idx = 0;
        for run in runs.iter_mut() {
            let run_end = byte_offset + run.len;
            while override_idx < overrides.len() && overrides[override_idx].0.end <= byte_offset {
                override_idx += 1;
            }
            if override_idx < overrides.len() {
                let (ref range, ref family) = overrides[override_idx];
                if byte_offset >= range.start && run_end <= range.end {
                    run.font.family = family.clone();
                }
            }
            byte_offset = run_end;
        }
    }

    /// Set the text runs for this piece of text.
    pub fn with_runs(mut self, runs: Vec<TextRun>) -> Self {
        let mut text = &*self.text;
        for run in &runs {
            text = text.get(run.len..).unwrap_or_else(|| {
                #[cfg(debug_assertions)]
                panic!("invalid text run. Text: '{text}', run: {run:?}");
                #[cfg(not(debug_assertions))]
                panic!("invalid text run");
            });
        }
        assert!(text.is_empty(), "invalid text run");
        self.runs = Some(runs);
        self
    }
}

impl Element for StyledText {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
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
        let font_family_overrides = self.delayed_font_family_overrides.take();
        let mut runs = self.runs.take().or_else(|| {
            self.delayed_highlights.take().map(|delayed_highlights| {
                Self::compute_runs(&self.text, &window.text_style(), delayed_highlights)
            })
        });

        if let Some(ref overrides) = font_family_overrides {
            let runs =
                runs.get_or_insert_with(|| vec![window.text_style().to_run(self.text.len())]);
            Self::apply_font_family_overrides(runs, overrides);
        }

        let layout_id = self.layout.layout(self.text.clone(), runs, window, cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        self.layout.prepaint(bounds, &self.text)
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.layout.paint(&self.text, window, cx)
    }
}

impl IntoElement for StyledText {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// The Layout for TextElement. This can be used to map indices to pixels and vice versa.
#[derive(Default, Clone)]
pub struct TextLayout(Rc<RefCell<Option<TextLayoutInner>>>);

struct TextLayoutInner {
    len: usize,
    lines: SmallVec<[WrappedLine; 1]>,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    size: Option<Size<Pixels>>,
    /// Cached answer to a min-content sizing probe: the size of the text
    /// wrapped at the width of its widest unbreakable segment. Cached
    /// separately from `size`/`lines` so sizing probes never clobber the
    /// layout that painting uses.
    min_content_size: Option<Size<Pixels>>,
    /// Cached answer to a max-content sizing probe: the size of the text
    /// with no wrapping at all.
    max_content_size: Option<Size<Pixels>>,
    bounds: Option<Bounds<Pixels>>,
}

/// The content-size probe slots of an existing layout, so that installing a
/// new wrapped layout preserves previously answered sizing probes.
fn content_size_slots(
    text_layout: Option<&TextLayoutInner>,
) -> (Option<Size<Pixels>>, Option<Size<Pixels>>) {
    text_layout.map_or((None, None), |layout| {
        (layout.min_content_size, layout.max_content_size)
    })
}

impl TextLayout {
    fn layout(
        &self,
        text: SharedString,
        runs: Option<Vec<TextRun>>,
        window: &mut Window,
        _: &mut App,
    ) -> LayoutId {
        let text_style = window.text_style();
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let line_height = window.pixel_snap(
            text_style
                .line_height
                .to_pixels(font_size.into(), window.rem_size()),
        );

        let runs = if let Some(runs) = runs {
            runs
        } else {
            vec![text_style.to_run(text.len())]
        };
        window.request_measured_layout(Default::default(), {
            let element_state = self.clone();

            move |known_dimensions, available_space, window, cx| {
                element_state.measure(
                    known_dimensions,
                    available_space,
                    &text,
                    &runs,
                    &text_style,
                    font_size,
                    line_height,
                    window,
                    cx,
                )
            }
        })
    }

    /// Answer a single measurement request from the layout engine, caching
    /// the result in the element state keyed by the constraints it was
    /// computed under. The layout stored for painting always corresponds to
    /// the most recent non-probe measurement.
    #[allow(clippy::too_many_arguments)]
    fn measure(
        &self,
        known_dimensions: Size<Option<Pixels>>,
        available_space: Size<crate::AvailableSpace>,
        text: &SharedString,
        runs: &[TextRun],
        text_style: &TextStyle,
        font_size: Pixels,
        line_height: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        let wrap_width = if text_style.white_space == WhiteSpace::Normal {
            known_dimensions.width.or(match available_space.width {
                crate::AvailableSpace::Definite(x) => Some(x),
                _ => None,
            })
        } else {
            None
        };

        let (truncate_width, truncation_affix, truncate_from) =
            if let Some(text_overflow) = text_style.text_overflow.clone() {
                let width = known_dimensions.width.or(match available_space.width {
                    crate::AvailableSpace::Definite(x) => match text_style.line_clamp {
                        Some(max_lines) => Some(x * max_lines),
                        None => Some(x),
                    },
                    _ => None,
                });

                match text_overflow {
                    TextOverflow::Truncate(s) => (width, s, TruncateFrom::End),
                    TextOverflow::TruncateStart(s) => (width, s, TruncateFrom::Start),
                    TextOverflow::TruncateMiddle(s) => (width, s, TruncateFrom::Middle),
                }
            } else {
                (None, "".into(), TruncateFrom::End)
            };

        // Min-content and max-content sizing probes: questions the
        // layout engine asks before the final width is resolved.
        // Answered from dedicated cache slots so they never clobber
        // `lines` — painting uses the layout from the final measure
        // call, which arrives with a definite width once the parent
        // has resolved one.
        if text_style.white_space == WhiteSpace::Normal
            && wrap_width.is_none()
            && truncate_width.is_none()
        {
            let is_min_content = matches!(available_space.width, crate::AvailableSpace::MinContent);

            if let Some(text_layout) = self.0.borrow().as_ref() {
                let cached_size = if is_min_content {
                    text_layout.min_content_size
                } else {
                    text_layout.max_content_size
                };
                if let Some(size) = cached_size {
                    return size;
                }
            }

            // The min-content width is the widest unbreakable segment
            // across all hard lines; re-shaping at that width yields
            // the matching height. The max-content size is simply the
            // unwrapped shape. Both shape calls hit the line layout
            // cache when repeated within or across frames.
            let probe_wrap_width = if is_min_content {
                let min_content_width = window
                    .text_system()
                    .shape_text(text.clone(), font_size, &runs, None, text_style.line_clamp)
                    .log_err()
                    .map_or(px(0.), |lines| {
                        lines.iter().fold(px(0.), |width, line| {
                            width.max(line.unwrapped_layout.min_content_width(&line.text))
                        })
                    });
                Some(min_content_width)
            } else {
                None
            };

            let lines = window
                .text_system()
                .shape_text(
                    text.clone(),
                    font_size,
                    &runs,
                    probe_wrap_width,
                    text_style.line_clamp,
                )
                .log_err()
                .unwrap_or_default();
            let mut size: Size<Pixels> = Size::default();
            for line in &lines {
                let line_size = line.size(line_height);
                size.height += line_size.height;
                size.width = size.width.max(line_size.width).ceil();
            }

            let mut element_state = self.0.borrow_mut();
            let text_layout = element_state.get_or_insert_with(|| TextLayoutInner {
                lines: Default::default(),
                len: 0,
                line_height,
                wrap_width: None,
                truncate_width: None,
                size: None,
                min_content_size: None,
                max_content_size: None,
                bounds: None,
            });
            if is_min_content {
                text_layout.min_content_size = Some(size);
            } else {
                text_layout.max_content_size = Some(size);
            }
            // In unconstrained contexts the final layout call is
            // itself a max-content measure, so no definite-width call
            // will ever store lines; keep these as the layout to
            // paint in that case.
            if text_layout.size.is_none() {
                text_layout.lines = lines;
                text_layout.len = text.len();
                text_layout.wrap_width = probe_wrap_width;
                text_layout.size = Some(size);
            }
            return size;
        }

        // Only use cached layout if:
        // 1. We have a cached size
        // 2. wrap_width matches exactly (a probe-installed fallback
        //    layout also matches here when the final width resolves
        //    to exactly the min-content width)
        // 3. truncate_width is None (if truncate_width is Some, we need to re-layout
        //    because the previous layout may have been computed without truncation)
        // 4. the cached layout was not truncated (a truncated layout answers an
        //    unconstrained probe with the truncated size, which poisons intrinsic
        //    sizing with whatever width some earlier measure pass happened to use)
        if let Some(text_layout) = self.0.borrow().as_ref()
            && let Some(size) = text_layout.size
            && wrap_width == text_layout.wrap_width
            && truncate_width.is_none()
            && text_layout.truncate_width.is_none()
        {
            return size;
        }

        let mut line_wrapper = cx.text_system().line_wrapper(text_style.font(), font_size);
        let (text, runs) = if let Some(truncate_width) = truncate_width {
            if let Some(max_lines) = text_style.line_clamp
                && let Some(wrap_width) = wrap_width
            {
                line_wrapper.truncate_wrapped_line(
                    text.clone(),
                    wrap_width,
                    max_lines,
                    &truncation_affix,
                    &runs,
                    truncate_from,
                )
            } else if let Some(unclipped) = window
                .text_system()
                .shape_text(text.clone(), font_size, &runs, None, None)
                .log_err()
                && unclipped
                    .iter()
                    .all(|line| line.size(line_height).width <= truncate_width)
            {
                // The truncation decision below sums per-character advances,
                // which overestimates the shaped width (no kerning), truncating
                // text that fits exactly in its measured width. Skip truncation
                // whenever the honestly-shaped text fits; the shaping result
                // comes from the line layout cache when the same text was
                // already measured untruncated this frame.
                (text.clone(), Cow::Borrowed(runs))
            } else {
                line_wrapper.truncate_line(
                    text.clone(),
                    truncate_width,
                    &truncation_affix,
                    &runs,
                    truncate_from,
                )
            }
        } else {
            (text.clone(), Cow::Borrowed(runs))
        };
        let len = text.len();

        let Some(lines) = window
            .text_system()
            .shape_text(
                text,
                font_size,
                &runs,
                wrap_width,            // Wrap if we know the width.
                text_style.line_clamp, // Limit the number of lines if line_clamp is set.
            )
            .log_err()
        else {
            let mut element_state = self.0.borrow_mut();
            let probe_sizes = content_size_slots(element_state.as_ref());
            element_state.replace(TextLayoutInner {
                lines: Default::default(),
                len: 0,
                line_height,
                wrap_width,
                truncate_width,
                size: Some(Size::default()),
                min_content_size: probe_sizes.0,
                max_content_size: probe_sizes.1,
                bounds: None,
            });
            return Size::default();
        };

        let mut size: Size<Pixels> = Size::default();
        for line in &lines {
            let line_size = line.size(line_height);
            size.height += line_size.height;
            size.width = size.width.max(line_size.width).ceil();
        }

        let mut element_state = self.0.borrow_mut();
        let probe_sizes = content_size_slots(element_state.as_ref());
        element_state.replace(TextLayoutInner {
            lines,
            len,
            line_height,
            wrap_width,
            truncate_width,
            size: Some(size),
            min_content_size: probe_sizes.0,
            max_content_size: probe_sizes.1,
            bounds: None,
        });

        size
    }

    fn prepaint(&self, bounds: Bounds<Pixels>, text: &str) {
        let mut element_state = self.0.borrow_mut();
        let element_state = element_state
            .as_mut()
            .with_context(|| format!("measurement has not been performed on {text}"))
            .unwrap();
        // The final measure call is expected to arrive with the resolved
        // width, so the stored lines should fit the bounds we're given here.
        // A wider layout means a measure/layout protocol violation and would
        // paint text overflowing its bounds.
        if let Some(size) = element_state.size
            && size.width > bounds.size.width + px(1.)
            && element_state.wrap_width != Some(bounds.size.width)
        {
            log::warn!(
                "text layed out at width {:?} is being painted into narrower bounds {:?}: {text:?}",
                size.width,
                bounds.size.width
            );
        }
        element_state.bounds = Some(bounds);
    }

    fn paint(&self, text: &str, window: &mut Window, cx: &mut App) {
        let element_state = self.0.borrow();
        let element_state = element_state
            .as_ref()
            .with_context(|| format!("measurement has not been performed on {text}"))
            .unwrap();
        let bounds = element_state
            .bounds
            .with_context(|| format!("prepaint has not been performed on {text}"))
            .unwrap();

        let line_height = element_state.line_height;
        let mut line_origin = bounds.origin;
        let text_style = window.text_style();
        for line in &element_state.lines {
            line.paint_background(
                line_origin,
                line_height,
                text_style.text_align,
                Some(bounds),
                window,
                cx,
            )
            .log_err();
            line.paint(
                line_origin,
                line_height,
                text_style.text_align,
                Some(bounds),
                window,
                cx,
            )
            .log_err();
            line_origin.y += line.size(line_height).height;
        }
    }

    /// Get the byte index into the input of the pixel position.
    pub fn index_for_position(&self, mut position: Point<Pixels>) -> Result<usize, usize> {
        let element_state = self.0.borrow();
        let element_state = element_state
            .as_ref()
            .expect("measurement has not been performed");
        let bounds = element_state
            .bounds
            .expect("prepaint has not been performed");

        if position.y < bounds.top() {
            return Err(0);
        }

        let line_height = element_state.line_height;
        let mut line_origin = bounds.origin;
        let mut line_start_ix = 0;
        for line in &element_state.lines {
            let line_bottom = line_origin.y + line.size(line_height).height;
            if position.y > line_bottom {
                line_origin.y = line_bottom;
                line_start_ix += line.len() + 1;
            } else {
                let position_within_line = position - line_origin;
                match line.index_for_position(position_within_line, line_height) {
                    Ok(index_within_line) => return Ok(line_start_ix + index_within_line),
                    Err(index_within_line) => return Err(line_start_ix + index_within_line),
                }
            }
        }

        Err(line_start_ix.saturating_sub(1))
    }

    /// Get the pixel position for the given byte index.
    pub fn position_for_index(&self, index: usize) -> Option<Point<Pixels>> {
        let element_state = self.0.borrow();
        let element_state = element_state
            .as_ref()
            .expect("measurement has not been performed");
        let bounds = element_state
            .bounds
            .expect("prepaint has not been performed");
        let line_height = element_state.line_height;

        let mut line_origin = bounds.origin;
        let mut line_start_ix = 0;

        for line in &element_state.lines {
            let line_end_ix = line_start_ix + line.len();
            if index < line_start_ix {
                break;
            } else if index > line_end_ix {
                line_origin.y += line.size(line_height).height;
                line_start_ix = line_end_ix + 1;
                continue;
            } else {
                let ix_within_line = index - line_start_ix;
                return Some(line_origin + line.position_for_index(ix_within_line, line_height)?);
            }
        }

        None
    }

    /// Retrieve the layout for the line containing the given byte index.
    pub fn line_layout_for_index(&self, index: usize) -> Option<Arc<WrappedLineLayout>> {
        let element_state = self.0.borrow();
        let element_state = element_state
            .as_ref()
            .expect("measurement has not been performed");
        let mut line_start_ix = 0;

        for line in &element_state.lines {
            let line_end_ix = line_start_ix + line.len();
            if index < line_start_ix {
                break;
            } else if index > line_end_ix {
                line_start_ix = line_end_ix + 1;
                continue;
            } else {
                return Some(line.layout.clone());
            }
        }

        None
    }

    /// Retrieve all line layouts in source order.
    pub fn line_layouts(&self) -> SmallVec<[Arc<WrappedLineLayout>; 1]> {
        self.0
            .borrow()
            .as_ref()
            .expect("measurement has not been performed")
            .lines
            .iter()
            .map(|line| line.layout.clone())
            .collect()
    }

    /// The bounds of this layout.
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.0.borrow().as_ref().unwrap().bounds.unwrap()
    }

    /// The line height for this layout.
    pub fn line_height(&self) -> Pixels {
        self.0.borrow().as_ref().unwrap().line_height
    }

    /// The UTF-8 length of the underlying text.
    pub fn len(&self) -> usize {
        self.0.borrow().as_ref().unwrap().len
    }

    /// The text for this layout.
    pub fn text(&self) -> String {
        self.0
            .borrow()
            .as_ref()
            .unwrap()
            .lines
            .iter()
            .map(|s| &s.text)
            .join("\n")
    }

    /// The text for this layout (with soft-wraps as newlines)
    pub fn wrapped_text(&self) -> String {
        let mut accumulator = String::new();

        for wrapped in self.0.borrow().as_ref().unwrap().lines.iter() {
            let mut seen = 0;
            for boundary in wrapped.layout.wrap_boundaries.iter() {
                let index = wrapped.layout.unwrapped_layout.runs[boundary.run_ix].glyphs
                    [boundary.glyph_ix]
                    .index;

                accumulator.push_str(&wrapped.text[seen..index]);
                accumulator.push('\n');
                seen = index;
            }
            accumulator.push_str(&wrapped.text[seen..]);
            accumulator.push('\n');
        }
        // Remove trailing newline
        accumulator.pop();
        accumulator
    }
}

/// A text element that can be interacted with.
pub struct InteractiveText {
    element_id: ElementId,
    text: StyledText,
    click_listener:
        Option<Box<dyn Fn(&[Range<usize>], InteractiveTextClickEvent, &mut Window, &mut App)>>,
    hover_listener: Option<Box<dyn Fn(Option<usize>, MouseMoveEvent, &mut Window, &mut App)>>,
    tooltip_builder: Option<Rc<dyn Fn(usize, &mut Window, &mut App) -> Option<AnyView>>>,
    tooltip_id: Option<TooltipId>,
    clickable_ranges: Vec<Range<usize>>,
}

struct InteractiveTextClickEvent {
    mouse_down_index: usize,
    mouse_up_index: usize,
}

#[doc(hidden)]
#[derive(Default)]
pub struct InteractiveTextState {
    mouse_down_index: Rc<Cell<Option<usize>>>,
    hovered_index: Rc<Cell<Option<usize>>>,
    active_tooltip: Rc<RefCell<Option<ActiveTooltip>>>,
}

/// InteractiveTest is a wrapper around StyledText that adds mouse interactions.
impl InteractiveText {
    /// Creates a new InteractiveText from the given text.
    pub fn new(id: impl Into<ElementId>, text: StyledText) -> Self {
        Self {
            element_id: id.into(),
            text,
            click_listener: None,
            hover_listener: None,
            tooltip_builder: None,
            tooltip_id: None,
            clickable_ranges: Vec::new(),
        }
    }

    /// on_click is called when the user clicks on one of the given ranges, passing the index of
    /// the clicked range.
    pub fn on_click(
        mut self,
        ranges: Vec<Range<usize>>,
        listener: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.click_listener = Some(Box::new(move |ranges, event, window, cx| {
            for (range_ix, range) in ranges.iter().enumerate() {
                if range.contains(&event.mouse_down_index) && range.contains(&event.mouse_up_index)
                {
                    listener(range_ix, window, cx);
                }
            }
        }));
        self.clickable_ranges = ranges;
        self
    }

    /// on_hover is called when the mouse moves over a character within the text, passing the
    /// index of the hovered character, or None if the mouse leaves the text.
    pub fn on_hover(
        mut self,
        listener: impl Fn(Option<usize>, MouseMoveEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.hover_listener = Some(Box::new(listener));
        self
    }

    /// tooltip lets you specify a tooltip for a given character index in the string.
    pub fn tooltip(
        mut self,
        builder: impl Fn(usize, &mut Window, &mut App) -> Option<AnyView> + 'static,
    ) -> Self {
        self.tooltip_builder = Some(Rc::new(builder));
        self
    }
}

impl Element for InteractiveText {
    type RequestLayoutState = ();
    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        Some(self.element_id.clone())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn a11y_role(&self) -> Option<accesskit::Role> {
        Some(accesskit::Role::Label)
    }

    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        node.set_value(self.text.text.to_string());
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.text.request_layout(None, inspector_id, window, cx)
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Hitbox {
        window.with_optional_element_state::<InteractiveTextState, _>(
            global_id,
            |interactive_state, window| {
                let mut interactive_state = interactive_state
                    .map(|interactive_state| interactive_state.unwrap_or_default());

                if let Some(interactive_state) = interactive_state.as_mut() {
                    if self.tooltip_builder.is_some() {
                        self.tooltip_id =
                            set_tooltip_on_window(&interactive_state.active_tooltip, window);
                    } else {
                        // If there is no longer a tooltip builder, remove the active tooltip.
                        interactive_state.active_tooltip.take();
                    }
                }

                self.text
                    .prepaint(None, inspector_id, bounds, state, window, cx);
                let hitbox = window.insert_hitbox(bounds, HitboxBehavior::Normal);
                (hitbox, interactive_state)
            },
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        hitbox: &mut Hitbox,
        window: &mut Window,
        cx: &mut App,
    ) {
        let current_view = window.current_view();
        let text_layout = self.text.layout().clone();
        window.with_element_state::<InteractiveTextState, _>(
            global_id.unwrap(),
            |interactive_state, window| {
                let mut interactive_state = interactive_state.unwrap_or_default();
                if let Some(click_listener) = self.click_listener.take() {
                    let mouse_position = window.mouse_position();
                    if let Ok(ix) = text_layout.index_for_position(mouse_position)
                        && self
                            .clickable_ranges
                            .iter()
                            .any(|range| range.contains(&ix))
                    {
                        window.set_cursor_style(crate::CursorStyle::PointingHand, hitbox)
                    }

                    let text_layout = text_layout.clone();
                    let mouse_down = interactive_state.mouse_down_index.clone();
                    if let Some(mouse_down_index) = mouse_down.get() {
                        let hitbox = hitbox.clone();
                        let clickable_ranges = mem::take(&mut self.clickable_ranges);
                        window.on_mouse_event(
                            move |event: &MouseUpEvent, phase, window: &mut Window, cx| {
                                if phase == DispatchPhase::Bubble && hitbox.is_hovered(window) {
                                    if let Ok(mouse_up_index) =
                                        text_layout.index_for_position(event.position)
                                    {
                                        click_listener(
                                            &clickable_ranges,
                                            InteractiveTextClickEvent {
                                                mouse_down_index,
                                                mouse_up_index,
                                            },
                                            window,
                                            cx,
                                        )
                                    }

                                    mouse_down.take();
                                    window.refresh();
                                }
                            },
                        );
                    } else {
                        let hitbox = hitbox.clone();
                        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, _| {
                            if phase == DispatchPhase::Bubble
                                && hitbox.is_hovered(window)
                                && let Ok(mouse_down_index) =
                                    text_layout.index_for_position(event.position)
                            {
                                mouse_down.set(Some(mouse_down_index));
                                window.refresh();
                            }
                        });
                    }
                }

                window.on_mouse_event({
                    let mut hover_listener = self.hover_listener.take();
                    let hitbox = hitbox.clone();
                    let text_layout = text_layout.clone();
                    let hovered_index = interactive_state.hovered_index.clone();
                    move |event: &MouseMoveEvent, phase, window, cx| {
                        if phase == DispatchPhase::Bubble && hitbox.is_hovered(window) {
                            let current = hovered_index.get();
                            let updated = text_layout.index_for_position(event.position).ok();
                            if current != updated {
                                hovered_index.set(updated);
                                if let Some(hover_listener) = hover_listener.as_ref() {
                                    hover_listener(updated, event.clone(), window, cx);
                                }
                                cx.notify(current_view);
                            }
                        }
                    }
                });

                if let Some(tooltip_builder) = self.tooltip_builder.clone() {
                    let active_tooltip = interactive_state.active_tooltip.clone();
                    let build_tooltip = Rc::new({
                        let tooltip_is_hoverable = false;
                        let text_layout = text_layout.clone();
                        move |window: &mut Window, cx: &mut App| {
                            text_layout
                                .index_for_position(window.mouse_position())
                                .ok()
                                .and_then(|position| tooltip_builder(position, window, cx))
                                .map(|view| (view, tooltip_is_hoverable))
                        }
                    });

                    // Use bounds instead of testing hitbox since this is called during prepaint.
                    let check_is_hovered_during_prepaint = Rc::new({
                        let source_bounds = hitbox.bounds;
                        let text_layout = text_layout.clone();
                        let pending_mouse_down = interactive_state.mouse_down_index.clone();
                        move |window: &Window| {
                            text_layout
                                .index_for_position(window.mouse_position())
                                .is_ok()
                                && source_bounds.contains(&window.mouse_position())
                                && pending_mouse_down.get().is_none()
                        }
                    });

                    let check_is_hovered = Rc::new({
                        let hitbox = hitbox.clone();
                        let text_layout = text_layout.clone();
                        let pending_mouse_down = interactive_state.mouse_down_index.clone();
                        move |window: &Window| {
                            text_layout
                                .index_for_position(window.mouse_position())
                                .is_ok()
                                && hitbox.is_hovered(window)
                                && pending_mouse_down.get().is_none()
                        }
                    });

                    register_tooltip_mouse_handlers(
                        &active_tooltip,
                        self.tooltip_id,
                        build_tooltip,
                        check_is_hovered,
                        check_is_hovered_during_prepaint,
                        None,
                        window,
                    );
                }

                self.text
                    .paint(None, inspector_id, bounds, &mut (), &mut (), window, cx);

                ((), interactive_state)
            },
        );
    }
}

impl IntoElement for InteractiveText {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_element_for() {
        use crate::{ParentElement as _, SharedString, div};
        use std::borrow::Cow;

        let _ = div().child("static str");
        let _ = div().child("String".to_string());
        let _ = div().child(Cow::Borrowed("Cow"));
        let _ = div().child(SharedString::from("SharedString"));
    }

    #[test]
    fn text_macro_id() {
        // one call to `text!` = one id
        fn make_text_stable_id(happy: bool) -> Text {
            text!(if happy { "happy" } else { "sad" })
        }

        // two calls to `text!` = two ids
        fn make_text_unstable_id(happy: bool) -> Text {
            if happy { text!("happy") } else { text!("sad") }
        }

        assert_eq!(make_text_stable_id(false).id, make_text_stable_id(true).id);
        assert_ne!(
            make_text_unstable_id(false).id,
            make_text_unstable_id(true).id
        );
    }
}

/// Characterization tests for how text sizes itself under flex and grid
/// layout, which depends on text elements answering min-content sizing
/// probes with the width of their widest unbreakable word rather than their
/// full unwrapped width.
#[cfg(target_os = "macos")]
#[cfg(test)]
mod content_sizing_tests {
    use super::TextLayout;
    use crate::{
        AnyElement, App, AvailableSpace, InteractiveElement, IntoElement, ParentElement, Pixels,
        SharedString, Size, Styled, TestAppContext, TextOverflow, TextStyle, VisualTestContext,
        WhiteSpace, Window, div, point, px, size,
    };

    const IBM_PLEX_SANS_REGULAR: &[u8] =
        include_bytes!("../../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");

    const LONG_TEXT: &str = "the quick brown fox jumps over the lazy dog and keeps on running";

    const LINE_HEIGHT: crate::Pixels = px(20.);

    #[gpui::test]
    fn text_in_flex_row_shrinks_and_wraps_instead_of_overflowing(cx: &mut TestAppContext) {
        let cx = harness(cx);

        // A flex item's automatic minimum size is its min-content size, so
        // without any explicit `min_w_0` the item must be able to shrink to
        // the container and wrap its text, not overflow at full unwrapped
        // width.
        draw_fixture(
            cx,
            div()
                .flex()
                .flex_row()
                .w(px(200.))
                .child(div().debug_selector(|| "TEXT".into()).child(LONG_TEXT))
                .into_any_element(),
        );
        let wrapped_bounds = cx.debug_bounds("TEXT").unwrap();

        draw_fixture(
            cx,
            div()
                .flex()
                .flex_row()
                .w(px(200.))
                .child(div().debug_selector(|| "TEXT".into()).child("word"))
                .into_any_element(),
        );
        let single_line_bounds = cx.debug_bounds("TEXT").unwrap();

        assert!(
            wrapped_bounds.size.width <= px(200.5),
            "text in a flex row should shrink to the container instead of \
             overflowing at its unwrapped width (got {:?})",
            wrapped_bounds.size.width
        );
        assert!(
            wrapped_bounds.size.height >= single_line_bounds.size.height * 2.,
            "shrunk text should wrap onto multiple lines \
             (wrapped height {:?}, single-line height {:?})",
            wrapped_bounds.size.height,
            single_line_bounds.size.height
        );
    }

    #[gpui::test]
    fn grid_min_content_column_floors_at_longest_word(cx: &mut TestAppContext) {
        let cx = harness(cx);

        draw_fixture(
            cx,
            div()
                .grid()
                .grid_cols_min_content(2)
                .w(px(240.))
                .child(div().debug_selector(|| "LONG".into()).child(LONG_TEXT))
                .child(div().debug_selector(|| "SHORT".into()).child("short"))
                .into_any_element(),
        );

        let long_bounds = cx.debug_bounds("LONG").unwrap();
        let short_bounds = cx.debug_bounds("SHORT").unwrap();

        assert!(
            long_bounds.size.width <= px(240.5),
            "a min-content grid track should not overflow the grid at the \
             cell's unwrapped width (got {:?})",
            long_bounds.size.width
        );
        // Grid stretches both cells to the row height, so wrapping is
        // visible on both bounds; compare against the explicit line height.
        assert!(
            long_bounds.size.height >= LINE_HEIGHT * 2.,
            "the constrained cell should wrap its text \
             (cell height {:?}, line height {LINE_HEIGHT:?})",
            long_bounds.size.height,
        );
        assert!(
            short_bounds.size.width >= px(1.),
            "the short cell should still get a non-degenerate track \
             (got {:?})",
            short_bounds.size.width
        );
    }

    #[gpui::test]
    fn grid_auto_columns_size_like_a_table(cx: &mut TestAppContext) {
        let cx = harness(cx);

        // A two-column auto grid behaves like `table-layout: auto`: the
        // short column hugs its content instead of taking an equal share,
        // and the long column absorbs the remaining space and wraps.
        draw_fixture(
            cx,
            div()
                .grid()
                .grid_cols_auto(2)
                .w(px(300.))
                .child(div().debug_selector(|| "LABEL".into()).child("label:"))
                .child(div().debug_selector(|| "PROSE".into()).child(LONG_TEXT))
                .into_any_element(),
        );

        let label_bounds = cx.debug_bounds("LABEL").unwrap();
        let prose_bounds = cx.debug_bounds("PROSE").unwrap();

        assert!(
            label_bounds.size.width < px(100.),
            "the short column should hug its content rather than split the \
             grid evenly (got {:?})",
            label_bounds.size.width
        );
        assert!(
            label_bounds.size.width > px(10.),
            "the short column should still fit its content (got {:?})",
            label_bounds.size.width
        );
        assert!(
            label_bounds.size.width + prose_bounds.size.width <= px(300.5),
            "auto columns should not overflow the grid \
             (label {:?} + prose {:?})",
            label_bounds.size.width,
            prose_bounds.size.width
        );
        assert!(
            prose_bounds.size.height >= LINE_HEIGHT * 2.,
            "the long column should wrap its text (got {:?})",
            prose_bounds.size.height
        );
        assert!(
            label_bounds.size.height <= prose_bounds.size.height,
            "the label's row participation should not exceed the prose cell"
        );
    }

    #[gpui::test]
    fn truncated_text_stays_on_a_single_line(cx: &mut TestAppContext) {
        let cx = harness(cx);

        draw_fixture(
            cx,
            div()
                .flex()
                .flex_row()
                .w(px(80.))
                .child(
                    div()
                        .debug_selector(|| "TRUNCATED".into())
                        .max_w_full()
                        .truncate()
                        .child(LONG_TEXT),
                )
                .into_any_element(),
        );

        let truncated_bounds = cx.debug_bounds("TRUNCATED").unwrap();
        assert!(
            truncated_bounds.size.width <= px(80.5),
            "truncated text should not overflow its container (got {:?})",
            truncated_bounds.size.width
        );
        assert!(
            truncated_bounds.size.height <= px(30.),
            "truncated text should stay on a single line (got {:?})",
            truncated_bounds.size.height
        );
    }

    /// The invariant behind constraint-keyed measurement caching: the answer
    /// to a measurement request depends only on that request's constraints,
    /// never on what happened to be measured before it.
    #[gpui::test]
    fn measure_answers_are_independent_of_measurement_history(cx: &mut TestAppContext) {
        cx.text_system()
            .add_fonts(vec![IBM_PLEX_SANS_REGULAR.into()])
            .unwrap();
        let cx = cx.add_empty_window();
        cx.update(|window, cx| {
            let text: SharedString = LONG_TEXT.into();
            let constraints = [
                AvailableSpace::MinContent,
                AvailableSpace::MaxContent,
                AvailableSpace::Definite(px(60.)),
                AvailableSpace::Definite(px(240.)),
            ];
            for truncate in [false, true] {
                let text_style = test_text_style(truncate);
                for second in constraints {
                    let fresh = measure_sequence(&[second], &text, &text_style, window, cx)[0];
                    for first in constraints {
                        let with_history =
                            measure_sequence(&[first, second], &text, &text_style, window, cx);
                        assert_eq!(
                            with_history[1], fresh,
                            "measuring {second:?} after {first:?} (truncate: {truncate}) \
                             changed the answer"
                        );
                    }
                }
            }
        });
    }

    /// Regression test for a truncated layout (from a measurement pass that
    /// happened to resolve a tiny width) being served to a later
    /// intrinsic-size probe, permanently collapsing the element.
    #[gpui::test]
    fn truncated_measure_does_not_poison_intrinsic_size(cx: &mut TestAppContext) {
        cx.text_system()
            .add_fonts(vec![IBM_PLEX_SANS_REGULAR.into()])
            .unwrap();
        let cx = cx.add_empty_window();
        cx.update(|window, cx| {
            let text: SharedString = LONG_TEXT.into();
            let text_style = test_text_style(true);

            let fresh = measure_sequence(
                &[AvailableSpace::MaxContent],
                &text,
                &text_style,
                window,
                cx,
            )[0];
            let poisoned = measure_sequence(
                &[
                    AvailableSpace::Definite(px(40.)),
                    AvailableSpace::MaxContent,
                ],
                &text,
                &text_style,
                window,
                cx,
            );

            assert!(
                poisoned[0].width <= px(41.),
                "the tiny definite measure should truncate (got {:?})",
                poisoned[0].width
            );
            assert_eq!(
                poisoned[1], fresh,
                "an unconstrained probe after a truncated measure must return \
                 the full intrinsic size"
            );
        });
    }

    /// Truncation must only activate on actual overflow: text laid out in a
    /// box of exactly its own measured width keeps its full content, even
    /// though the truncation decision's per-character width estimate (which
    /// ignores kerning) may overestimate the shaped width.
    #[gpui::test]
    fn text_that_fits_exactly_is_not_truncated(cx: &mut TestAppContext) {
        cx.text_system()
            .add_fonts(vec![IBM_PLEX_SANS_REGULAR.into()])
            .unwrap();
        let cx = cx.add_empty_window();
        cx.update(|window, cx| {
            // Kerning-heavy text maximizes the gap between the shaped width
            // and the truncation decision's per-character estimate.
            let text: SharedString = "AV To Wa VAVAV Cargo.toml".into();
            let text_style = test_text_style(true);

            let layout = TextLayout::default();
            let runs = vec![text_style.to_run(text.len())];
            let font_size = text_style.font_size.to_pixels(window.rem_size());
            // The exact shaped width, unrounded: the tightest box that still
            // fits the text, with no slack to absorb estimation error.
            let shaped_width = window
                .text_system()
                .shape_text(text.clone(), font_size, &runs, None, None)
                .unwrap()[0]
                .size(px(20.))
                .width;
            layout.measure(
                Size::default(),
                size(
                    AvailableSpace::Definite(shaped_width),
                    AvailableSpace::MaxContent,
                ),
                &text,
                &runs,
                &text_style,
                font_size,
                px(20.),
                window,
                cx,
            );

            let state = layout.0.borrow();
            let lines = &state.as_ref().unwrap().lines;
            assert_eq!(lines.len(), 1);
            assert_eq!(
                lines[0].text, text,
                "text that fits exactly in its measured width must not be truncated"
            );
        });
    }

    fn harness(cx: &mut TestAppContext) -> &mut VisualTestContext {
        cx.text_system()
            .add_fonts(vec![IBM_PLEX_SANS_REGULAR.into()])
            .unwrap();
        cx.add_empty_window()
    }

    fn draw_fixture(cx: &mut VisualTestContext, fixture: AnyElement) {
        let mut fixture = Some(fixture);
        cx.draw(point(px(0.), px(0.)), size(px(400.), px(400.)), |_, _| {
            div()
                .font_family("IBM Plex Sans")
                .text_size(px(16.))
                .line_height(LINE_HEIGHT)
                .child(fixture.take().unwrap())
                .into_any_element()
        });
    }

    fn test_text_style(truncate: bool) -> TextStyle {
        TextStyle {
            font_family: "IBM Plex Sans".into(),
            font_size: px(16.).into(),
            white_space: if truncate {
                WhiteSpace::Nowrap
            } else {
                WhiteSpace::Normal
            },
            text_overflow: truncate.then(|| TextOverflow::Truncate("\u{2026}".into())),
            ..Default::default()
        }
    }

    /// Feed `constraints` one at a time into a single [`TextLayout`],
    /// returning each measurement's answer.
    fn measure_sequence(
        constraints: &[AvailableSpace],
        text: &SharedString,
        text_style: &TextStyle,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<Size<Pixels>> {
        let layout = TextLayout::default();
        let runs = vec![text_style.to_run(text.len())];
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        constraints
            .iter()
            .map(|constraint| {
                layout.measure(
                    Size::default(),
                    size(*constraint, AvailableSpace::MaxContent),
                    text,
                    &runs,
                    text_style,
                    font_size,
                    px(20.),
                    window,
                    cx,
                )
            })
            .collect()
    }
}
