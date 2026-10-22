use crate::{
    ActiveTooltip, AnyView, App, Bounds, DispatchPhase, Element, ElementId, GlobalElementId,
    HighlightStyle, Hitbox, HitboxBehavior, InspectorElementId, IntoElement, LayoutId,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, SharedString, Size, TextOverflow,
    TextRun, TextStyle, TooltipId, TruncateFrom, WhiteSpace, Window, WrappedLine,
    WrappedLineLayout, register_tooltip_mouse_handlers, set_tooltip_on_window,
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

#[derive(Clone, Copy)]
enum TextPaintPass {
    All,
    Background,
    Foreground,
}

struct TextLayoutInner {
    len: usize,
    lines: SmallVec<[WrappedLine; 1]>,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    truncate_width: Option<Pixels>,
    size: Option<Size<Pixels>>,
    bounds: Option<Bounds<Pixels>>,
}

pub(crate) struct TextMeasure {
    text: SharedString,
    runs: TextRuns,
    text_style: TextStyle,
    font_size: Pixels,
    line_height: Pixels,
    rem_size: Pixels,
    scale_factor: f32,
    font_generation: usize,
    element_state: TextLayout,
}

enum TextRuns {
    FromStyle,
    Explicit(Vec<TextRun>),
}

impl TextMeasure {
    fn runs_equal(&self, previous: &Self) -> bool {
        match (&self.runs, &previous.runs) {
            (TextRuns::FromStyle, TextRuns::FromStyle) => true,
            (TextRuns::Explicit(runs), TextRuns::Explicit(previous_runs)) => runs == previous_runs,
            (TextRuns::FromStyle, TextRuns::Explicit(runs)) => {
                runs.as_slice() == [self.text_style.to_run(self.text.len())]
            }
            (TextRuns::Explicit(runs), TextRuns::FromStyle) => {
                runs.as_slice() == [previous.text_style.to_run(previous.text.len())]
            }
        }
    }

    pub(crate) fn carry_from(&self, previous: &Self) -> bool {
        if self.text != previous.text
            || self.text_style != previous.text_style
            || !self.runs_equal(previous)
            || self.font_size != previous.font_size
            || self.line_height != previous.line_height
            || self.rem_size != previous.rem_size
            || self.scale_factor != previous.scale_factor
            || self.font_generation != previous.font_generation
        {
            return false;
        }
        {
            let state = previous.element_state.0.borrow();
            let Some(state) = state.as_ref() else {
                return false;
            };
            if state.lines.is_empty() && !self.text.is_empty() {
                return false;
            }
        }
        let mut measured = if Rc::strong_count(&previous.element_state.0) == 1
            || Rc::ptr_eq(&self.element_state.0, &previous.element_state.0)
        {
            previous
                .element_state
                .0
                .borrow_mut()
                .take()
                .expect("validated text state")
        } else {
            // Public TextLayout handles may outlive their frame. Preserve those
            // handles, but move the state when only the old descriptor owns it.
            let state = previous.element_state.0.borrow();
            let state = state.as_ref().expect("validated text state");
            TextLayoutInner {
                len: state.len,
                lines: state
                    .lines
                    .iter()
                    .map(|line| WrappedLine {
                        layout: line.layout.clone(),
                        text: line.text.clone(),
                        decoration_runs: line.decoration_runs.clone(),
                    })
                    .collect(),
                line_height: state.line_height,
                wrap_width: state.wrap_width,
                truncate_width: state.truncate_width,
                size: state.size,
                bounds: None,
            }
        };
        measured.bounds = None;
        self.element_state.0.borrow_mut().replace(measured);
        true
    }

    pub(crate) fn refresh_fonts(&mut self, generation: usize) -> bool {
        if self.font_generation == generation {
            return false;
        }
        self.font_generation = generation;
        self.reset();
        true
    }

    pub(crate) fn reset(&self) {
        self.element_state.0.borrow_mut().take();
    }
}

impl TextLayoutInner {
    fn cached_size(
        &self,
        wrap_width: Option<Pixels>,
        truncate_width: Option<Pixels>,
    ) -> Option<Size<Pixels>> {
        let size = self.size?;
        // A truncated layout must not answer intrinsic sizing with the width
        // imposed by an earlier measurement pass.
        if truncate_width.is_none()
            && self.truncate_width.is_none()
            && (wrap_width.is_none()
                || wrap_width == self.wrap_width
                || (self.wrap_width.is_none()
                    && wrap_width.is_some_and(|width| size.width <= width)))
        {
            Some(size)
        } else {
            None
        }
    }
}

impl TextLayout {
    fn layout(
        &self,
        text: SharedString,
        runs: Option<Vec<TextRun>>,
        window: &mut Window,
        _: &mut App,
    ) -> LayoutId {
        window.request_text_layout(self.measurement(text, runs, window))
    }

    fn measurement(
        &self,
        text: SharedString,
        runs: Option<Vec<TextRun>>,
        window: &Window,
    ) -> TextMeasure {
        let text_style = window.text_style();
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let line_height = window.pixel_snap(
            text_style
                .line_height
                .to_pixels(font_size.into(), window.rem_size()),
        );

        let runs = runs.map_or(TextRuns::FromStyle, TextRuns::Explicit);
        TextMeasure {
            text,
            runs,
            text_style,
            font_size,
            line_height,
            rem_size: window.rem_size(),
            scale_factor: window.scale_factor(),
            font_generation: window.text_system().font_generation(),
            element_state: self.clone(),
        }
    }
}

impl TextMeasure {
    pub(crate) fn measure(
        &self,
        known_dimensions: Size<Option<Pixels>>,
        available_space: Size<crate::AvailableSpace>,
        run_mode: Option<taffy::RunMode>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        let Self {
            text,
            runs,
            text_style,
            element_state,
            ..
        } = self;
        let font_size = self.font_size;
        let line_height = self.line_height;
        // Taffy keeps preliminary size entries separately from its final layout
        // entry. A probe must not overwrite paintable lines: final layout may hit
        // that entry without calling us again. Taffy 0.14 always measures a leaf
        // on a PerformLayout miss, even when both dimensions are known. Before
        // any lines exist, keep the first probe so a fitting final measurement
        // can reuse it without allocating another set of shaped-line wrappers.
        let publish =
            run_mode != Some(taffy::RunMode::ComputeSize) || element_state.0.borrow().is_none();
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

        if let Some(text_layout) = element_state.0.borrow().as_ref()
            && (run_mode.is_none() || wrap_width.is_some() || text_layout.wrap_width.is_none())
            && let Some(size) = text_layout.cached_size(wrap_width, truncate_width)
        {
            return size;
        }

        let default_run;
        let runs = match runs {
            TextRuns::FromStyle => {
                default_run = text_style.to_run(text.len());
                std::slice::from_ref(&default_run)
            }
            TextRuns::Explicit(runs) => runs.as_slice(),
        };
        let (text, runs) = if let Some(truncate_width) = truncate_width {
            if let Some(max_lines) = text_style.line_clamp
                && let Some(wrap_width) = wrap_width
            {
                let mut line_wrapper = cx.text_system().line_wrapper(text_style.font(), font_size);
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
                let mut line_wrapper = cx.text_system().line_wrapper(text_style.font(), font_size);
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
            if publish {
                element_state.0.borrow_mut().replace(TextLayoutInner {
                    lines: Default::default(),
                    len: 0,
                    line_height,
                    wrap_width,
                    truncate_width,
                    size: Some(Size::default()),
                    bounds: None,
                });
            }
            return Size::default();
        };

        let mut size: Size<Pixels> = Size::default();
        for line in &lines {
            let line_size = line.size(line_height);
            size.height += line_size.height;
            size.width = size.width.max(line_size.width).ceil();
        }

        if publish {
            element_state.0.borrow_mut().replace(TextLayoutInner {
                lines,
                len,
                line_height,
                wrap_width,
                truncate_width,
                size: Some(size),
                bounds: None,
            });
        }

        size
    }
}

impl TextLayout {
    fn prepaint(&self, bounds: Bounds<Pixels>, text: &str) {
        let mut element_state = self.0.borrow_mut();
        let element_state = element_state
            .as_mut()
            .with_context(|| format!("measurement has not been performed on {text}"))
            .unwrap();
        element_state.bounds = Some(bounds);
    }

    fn paint(&self, text: &str, window: &mut Window, cx: &mut App) {
        self.paint_lines(TextPaintPass::All, window, cx)
            .with_context(|| format!("failed to paint {text}"))
            .unwrap();
    }

    /// Paints the backgrounds of the text runs without the glyphs, so that callers can paint
    /// content, such as highlights, between the backgrounds and the text. Follow it with
    /// [`Self::paint_foreground`] to paint the glyphs.
    pub fn paint_background(&self, window: &mut Window, cx: &mut App) -> anyhow::Result<()> {
        self.paint_lines(TextPaintPass::Background, window, cx)
    }

    /// Paints the glyphs and decorations of the text without the run backgrounds.
    pub fn paint_foreground(&self, window: &mut Window, cx: &mut App) -> anyhow::Result<()> {
        self.paint_lines(TextPaintPass::Foreground, window, cx)
    }

    fn paint_lines(
        &self,
        pass: TextPaintPass,
        window: &mut Window,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        let element_state = self.0.borrow();
        let element_state = element_state
            .as_ref()
            .context("measurement has not been performed")?;
        let bounds = element_state
            .bounds
            .context("prepaint has not been performed")?;

        let line_height = element_state.line_height;
        let mut line_origin = bounds.origin;
        let text_style = window.text_style();
        for line in &element_state.lines {
            if matches!(pass, TextPaintPass::All | TextPaintPass::Background) {
                line.paint_background(
                    line_origin,
                    line_height,
                    text_style.text_align,
                    Some(bounds),
                    window,
                    cx,
                )
                .log_err();
            }
            if matches!(pass, TextPaintPass::All | TextPaintPass::Foreground) {
                line.paint(
                    line_origin,
                    line_height,
                    text_style.text_align,
                    Some(bounds),
                    window,
                    cx,
                )
                .log_err();
            }
            line_origin.y += line.size(line_height).height;
        }
        Ok(())
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
    long_press_tooltip_active: Rc<Cell<bool>>,
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
                        interactive_state.long_press_tooltip_active.set(false);
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
                        interactive_state.long_press_tooltip_active.clone(),
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
    use crate::AppContext as _;

    #[gpui::test]
    fn text_run_storage_preserves_layout_and_descriptor_carry(cx: &mut crate::TestAppContext) {
        eprintln!(
            "TextMeasure={} StyledText={}",
            std::mem::size_of::<TextMeasure>(),
            std::mem::size_of::<StyledText>()
        );
        cx.add_empty_window().update(|window, cx| {
            let text: SharedString = "one two three four".into();
            let default_run = window.text_style().to_run(text.len());
            let mut changed_run = default_run.clone();
            changed_run.color = crate::red();
            let mut first_run = default_run.clone();
            first_run.len = 4;
            let mut second_run = changed_run.clone();
            second_run.len = text.len() - first_run.len;
            let variants = [
                None,
                Some(vec![default_run.clone()]),
                Some(vec![changed_run]),
                Some(vec![first_run, second_run]),
            ];
            let available = Size {
                width: crate::AvailableSpace::Definite(crate::px(50.0)),
                height: crate::AvailableSpace::MaxContent,
            };
            let scope = crate::element::InternedHandle::for_test(0, 0);
            for previous_runs in &variants {
                for next_runs in &variants {
                    let mut engine = crate::TaffyLayoutEngine::new();
                    engine.enable_cache();
                    let previous = TextLayout::default();
                    engine.set_request_scope(&scope);
                    let id = engine.request_text_layout(previous.measurement(
                        text.clone(),
                        previous_runs.clone(),
                        window,
                    ));
                    engine.compute_layout(id, available, window, cx);
                    let (identity, dirty) = engine.text_context_identity_and_dirty(id);
                    assert!(!dirty);
                    engine.clear();
                    let next = TextLayout::default();
                    let measure = next.measurement(text.clone(), next_runs.clone(), window);
                    let previous_effective = previous_runs
                        .clone()
                        .unwrap_or_else(|| vec![default_run.clone()]);
                    let next_effective = next_runs
                        .clone()
                        .unwrap_or_else(|| vec![default_run.clone()]);
                    let changed = previous_effective != next_effective;
                    let reference = TextLayout::default();
                    let expected_size = reference
                        .measurement(text.clone(), Some(next_effective), window)
                        .measure(
                            Size::default(),
                            available,
                            Some(taffy::RunMode::PerformLayout),
                            window,
                            cx,
                        );
                    engine.set_request_scope(&scope);
                    let next_id = engine.request_text_layout(measure);
                    assert_eq!(id, next_id);
                    assert_eq!(
                        engine.text_context_identity_and_dirty(next_id),
                        (identity, changed)
                    );
                    assert_eq!(next.0.borrow().is_none(), changed);
                    engine.compute_layout(next_id, available, window, cx);
                    assert_eq!(next.wrapped_text(), reference.wrapped_text());
                    assert_eq!(
                        next.0.borrow().as_ref().expect("measured state").size,
                        Some(expected_size)
                    );
                }
            }
        });
    }

    #[gpui::test]
    fn text_context_reuses_allocation_and_invalidates_changed_descriptors(
        cx: &mut crate::TestAppContext,
    ) {
        cx.add_empty_window().update(|window, cx| {
            let text: SharedString = "one two three four".into();
            let available = Size {
                width: crate::AvailableSpace::Definite(crate::px(50.0)),
                height: crate::AvailableSpace::MaxContent,
            };
            let scope = crate::element::InternedHandle::for_test(0, 0);
            for shared in [false, true] {
                for change in 0..12 {
                    let mut engine = crate::TaffyLayoutEngine::new();
                    engine.enable_cache();
                    let previous = TextLayout::default();
                    engine.set_request_scope(&scope);
                    let id = engine.request_text_layout(previous.measurement(
                        text.clone(),
                        None,
                        window,
                    ));
                    engine.compute_layout(id, available, window, cx);
                    let old_text = previous.wrapped_text();
                    let old_bounds = Bounds::default();
                    previous.prepaint(old_bounds, &text);
                    let previous = shared.then_some(previous);
                    let (identity, dirty) = engine.text_context_identity_and_dirty(id);
                    assert!(identity.is_some());
                    assert!(!dirty);
                    engine.clear();

                    let fresh = TextLayout::default();
                    let mut measure = fresh.measurement(text.clone(), None, window);
                    // Seed a populated destination to prove rejection resets it,
                    // rather than merely relying on fresh elements being empty.
                    measure.measure(
                        Size::default(),
                        available,
                        Some(taffy::RunMode::PerformLayout),
                        window,
                        cx,
                    );
                    match change {
                        0 => {}
                        1 => measure.text = "ONE two three four".into(),
                        2 => {
                            let mut run = measure.text_style.to_run(measure.text.len());
                            run.len -= 1;
                            measure.runs = TextRuns::Explicit(vec![run]);
                        }
                        3 => measure.text_style.font_family = "different font".into(),
                        4 => measure.text_style.white_space = WhiteSpace::Nowrap,
                        5 => measure.text_style.line_clamp = Some(1),
                        6 => {
                            measure.text_style.text_overflow =
                                Some(TextOverflow::Truncate("…".into()))
                        }
                        7 => measure.font_size += crate::px(1.0),
                        8 => measure.line_height += crate::px(1.0),
                        9 => measure.rem_size += crate::px(1.0),
                        10 => measure.scale_factor += 0.25,
                        11 => measure.font_generation += 1,
                        _ => unreachable!(),
                    }
                    engine.set_request_scope(&scope);
                    let next = engine.request_text_layout(measure);
                    assert_eq!(next, id);
                    assert_eq!(
                        engine.text_context_identity_and_dirty(next),
                        (identity, change != 0),
                        "change {change}, shared {shared}"
                    );
                    if change == 0 {
                        assert_eq!(fresh.wrapped_text(), old_text);
                        assert_eq!(
                            fresh.0.borrow().as_ref().expect("carried state").bounds,
                            None
                        );
                        engine.compute_layout(next, available, window, cx);
                        assert_eq!(fresh.wrapped_text(), old_text);
                    } else {
                        assert!(fresh.0.borrow().is_none());
                    }
                    if let Some(previous) = previous {
                        assert_eq!(previous.wrapped_text(), old_text);
                        assert_eq!(
                            previous.0.borrow().as_ref().expect("old state").bounds,
                            Some(old_bounds)
                        );
                    }
                }
            }
        });
    }

    #[gpui::test]
    fn text_context_transitions_retire_custom_captures(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            let mut engine = crate::TaffyLayoutEngine::new();
            engine.enable_cache();
            let scope = crate::element::InternedHandle::for_test(0, 0);
            let available = Size {
                width: crate::AvailableSpace::MaxContent,
                height: crate::AvailableSpace::MaxContent,
            };
            let text: SharedString = "text".into();
            let layout = TextLayout::default();
            engine.set_request_scope(&scope);
            let id = engine.request_text_layout(layout.measurement(text.clone(), None, window));
            engine.compute_layout(id, available, window, cx);
            let old_text = layout.wrapped_text();
            engine.clear();

            let capture = Rc::new(());
            let weak = Rc::downgrade(&capture);
            engine.set_request_scope(&scope);
            let custom = engine.request_measured_layout(
                crate::Style::default(),
                crate::px(16.0),
                1.0,
                move |_, _, _, _| {
                    assert_eq!(Rc::strong_count(&capture), 1);
                    crate::size(crate::px(37.0), crate::px(19.0))
                },
            );
            assert_eq!(custom, id);
            assert_eq!(engine.text_context_identity_and_dirty(id), (None, true));
            engine.compute_layout(id, available, window, cx);
            assert!(weak.upgrade().is_some());
            engine.clear();
            assert!(weak.upgrade().is_some());

            let fresh = TextLayout::default();
            engine.set_request_scope(&scope);
            assert_eq!(
                engine.request_text_layout(fresh.measurement(text, None, window)),
                id
            );
            assert!(weak.upgrade().is_none());
            assert!(engine.text_context_identity_and_dirty(id).0.is_some());
            assert!(engine.text_context_identity_and_dirty(id).1);
            assert!(fresh.0.borrow().is_none());
            engine.compute_layout(id, available, window, cx);
            assert_eq!(fresh.wrapped_text(), old_text);
            engine.clear();

            engine.set_request_scope(&scope);
            assert_eq!(
                engine.request_layout(crate::Style::default(), crate::px(16.0), 1.0, &[]),
                id
            );
            assert_eq!(engine.text_context_identity_and_dirty(id), (None, true));
            engine.compute_layout(id, available, window, cx);
            assert_eq!(engine.layout_bounds(id, 1.0).size, Size::default());
            assert_eq!(layout.wrapped_text(), old_text);
        });
    }

    #[test]
    fn text_measurement_cache_guard() {
        let size = crate::size(crate::px(100.), crate::px(40.));
        let mut layout = TextLayoutInner {
            len: 0,
            lines: Default::default(),
            line_height: crate::px(20.),
            wrap_width: None,
            truncate_width: None,
            size: Some(size),
            bounds: None,
        };
        for width in [None, Some(crate::px(100.)), Some(crate::px(101.))] {
            assert_eq!(layout.cached_size(width, None), Some(size));
            assert_eq!(layout.cached_size(width, Some(crate::px(200.))), None);
        }
        assert_eq!(layout.cached_size(Some(crate::px(99.)), None), None);

        layout.wrap_width = Some(crate::px(100.));
        assert_eq!(layout.cached_size(None, None), Some(size));
        assert_eq!(layout.cached_size(Some(crate::px(100.)), None), Some(size));
        assert_eq!(layout.cached_size(Some(crate::px(99.)), None), None);
        assert_eq!(layout.cached_size(Some(crate::px(101.)), None), None);

        for cached_wrap_width in [None, Some(crate::px(100.))] {
            layout.wrap_width = cached_wrap_width;
            layout.truncate_width = Some(crate::px(100.));
            for width in [None, Some(crate::px(100.)), Some(crate::px(101.))] {
                assert_eq!(layout.cached_size(width, None), None);
            }
        }
        layout.truncate_width = None;
        layout.size = None;
        assert_eq!(layout.cached_size(None, None), None);
        assert_eq!(layout.cached_size(Some(crate::px(100.)), None), None);
    }

    #[gpui::test]
    fn text_cache_transfers_state_preserves_old_handles_and_recovers_empty_state(
        cx: &mut crate::TestAppContext,
    ) {
        cx.add_empty_window().update(|window, cx| {
            let text: SharedString = "one two three four".into();
            let available = Size {
                width: crate::AvailableSpace::Definite(crate::px(50.0)),
                height: crate::AvailableSpace::MaxContent,
            };
            let previous = TextLayout::default();
            let previous_measure = previous.measurement(text.clone(), None, window);
            let measured_size = previous_measure.measure(
                Size::default(),
                available,
                Some(taffy::RunMode::PerformLayout),
                window,
                cx,
            );
            let old_bounds = Bounds::new(
                crate::point(crate::px(13.0), crate::px(17.0)),
                measured_size,
            );
            previous.prepaint(old_bounds, &text);
            let old_text = previous.wrapped_text();
            let fresh = TextLayout::default();
            let fresh_measure = fresh.measurement(text.clone(), None, window);
            assert!(fresh_measure.carry_from(&previous_measure));
            assert_eq!(
                previous.0.borrow().as_ref().expect("old state").bounds,
                Some(old_bounds)
            );
            assert_eq!(previous.wrapped_text(), old_text);
            assert_eq!(fresh.wrapped_text(), old_text);
            assert_eq!(fresh.0.borrow().as_ref().expect("fresh state").bounds, None);
            fresh_measure.measure(
                Size::default(),
                Size {
                    width: crate::AvailableSpace::Definite(crate::px(15.0)),
                    ..available
                },
                Some(taffy::RunMode::PerformLayout),
                window,
                cx,
            );
            assert_eq!(previous.wrapped_text(), old_text);
            assert_eq!(
                previous.0.borrow().as_ref().expect("old state").bounds,
                Some(old_bounds)
            );

            drop(previous);
            assert_eq!(Rc::strong_count(&previous_measure.element_state.0), 1);
            let moved = TextLayout::default();
            let moved_measure = moved.measurement(text.clone(), None, window);
            assert!(moved_measure.carry_from(&previous_measure));
            assert!(previous_measure.element_state.0.borrow().is_none());
            assert_eq!(moved.wrapped_text(), old_text);
            assert!(moved_measure.carry_from(&moved_measure));
            assert_eq!(moved.wrapped_text(), old_text);

            // shape_text currently has no Err producer. Seed exactly the empty
            // failure result to exercise its rejection and next-frame recovery.
            let mut engine = crate::TaffyLayoutEngine::new();
            engine.enable_cache();
            let scope = crate::element::InternedHandle::for_test(0, 0);
            engine.set_request_scope(&scope);
            let id = engine.request_text_layout(moved_measure);
            engine.compute_layout(id, available, window, cx);
            {
                let mut state = moved.0.borrow_mut();
                let state = state.as_mut().expect("measured state");
                state.lines.clear();
                state.len = 0;
                state.size = Some(Size::default());
                state.bounds = None;
            }
            engine.clear();
            let recovered = TextLayout::default();
            engine.set_request_scope(&scope);
            let recovered_id =
                engine.request_text_layout(recovered.measurement(text, None, window));
            assert_eq!(recovered_id, id);
            assert!(
                recovered.0.borrow().is_none(),
                "failed state must not carry"
            );
            engine.compute_layout(recovered_id, available, window, cx);
            assert_eq!(recovered.wrapped_text(), old_text);
            assert_eq!(
                recovered.0.borrow().as_ref().expect("recovered").size,
                Some(measured_size)
            );
        });
    }

    #[gpui::test]
    fn text_cache_first_probe_reuses_lines_without_overwriting_final(
        cx: &mut crate::TestAppContext,
    ) {
        cx.add_empty_window().update(|window, cx| {
            for text in ["one two three four five six", "one two three\nfour five"] {
                let layout = TextLayout::default();
                let measure = layout.measurement(text.into(), None, window);
                let intrinsic = Size {
                    width: crate::AvailableSpace::MaxContent,
                    height: crate::AvailableSpace::MaxContent,
                };
                let size = measure.measure(
                    Size::default(),
                    intrinsic,
                    Some(taffy::RunMode::ComputeSize),
                    window,
                    cx,
                );
                assert!(size.width > crate::px(0.));
                let first_lines = layout.wrapped_text();
                let bounds = Bounds::new(Point::default(), size);
                layout.prepaint(bounds, text);
                let fitting = Size {
                    width: crate::AvailableSpace::Definite(size.width),
                    ..intrinsic
                };
                assert_eq!(
                    measure.measure(
                        Size::default(),
                        fitting,
                        Some(taffy::RunMode::PerformLayout),
                        window,
                        cx,
                    ),
                    size
                );
                assert_eq!(layout.wrapped_text(), first_lines);
                assert_eq!(
                    layout.0.borrow().as_ref().expect("reused lines").bounds,
                    Some(bounds)
                );

                let narrow = Size {
                    width: crate::AvailableSpace::Definite(size.width / 2.),
                    ..intrinsic
                };
                measure.measure(
                    Size::default(),
                    narrow,
                    Some(taffy::RunMode::ComputeSize),
                    window,
                    cx,
                );
                assert_eq!(layout.wrapped_text(), first_lines);
                assert_eq!(
                    layout.0.borrow().as_ref().expect("final lines").bounds,
                    Some(bounds)
                );
                let final_size = measure.measure(
                    Size::default(),
                    narrow,
                    Some(taffy::RunMode::PerformLayout),
                    window,
                    cx,
                );
                let reference = TextLayout::default();
                let reference_size = reference.measurement(text.into(), None, window).measure(
                    Size::default(),
                    narrow,
                    None,
                    window,
                    cx,
                );
                assert_eq!(final_size, reference_size);
                assert_eq!(layout.wrapped_text(), reference.wrapped_text());
                assert!(
                    layout
                        .0
                        .borrow()
                        .as_ref()
                        .expect("new final lines")
                        .bounds
                        .is_none()
                );
            }
        });
    }

    #[gpui::test]
    fn text_cache_probe_miss_preserves_cached_final_lines(cx: &mut crate::TestAppContext) {
        use taffy::{LayoutInput, RunMode};

        fn compute(
            tree: &mut taffy::TaffyTree<()>,
            id: taffy::NodeId,
            input: LayoutInput,
            measure: &TextMeasure,
            calls: &mut usize,
            window: &mut Window,
            cx: &mut App,
        ) -> taffy::LayoutOutput {
            taffy::compute_cached_layout(tree, id, input, |tree, id, input| {
                *calls += 1;
                taffy::compute_leaf_layout(
                    input,
                    tree.style(id).expect("text style"),
                    |_, _| 0.0,
                    |known, available| {
                        let measured = measure.measure(
                            Size {
                                width: known.width.map(Pixels),
                                height: known.height.map(Pixels),
                            },
                            available.into(),
                            Some(input.run_mode),
                            window,
                            cx,
                        );
                        taffy::geometry::Size {
                            width: measured.width.0,
                            height: measured.height.0,
                        }
                    },
                )
            })
        }

        cx.add_empty_window().update(|window, cx| {
            window
                .invalidator
                .set_phase(crate::window::DrawPhase::Prepaint);
            window.set_scale_factor(1.0);
            for white_space in [WhiteSpace::Normal, WhiteSpace::Nowrap] {
                for overflow in [
                    None,
                    Some(TextOverflow::Truncate("…".into())),
                    Some(TextOverflow::TruncateStart("…".into())),
                    Some(TextOverflow::TruncateMiddle("…".into())),
                ] {
                    for line_clamp in [None, Some(1), Some(2)] {
                        window.with_text_style(
                            Some(crate::TextStyleRefinement {
                                white_space: Some(white_space),
                                text_overflow: overflow.clone(),
                                line_clamp,
                                ..Default::default()
                            }),
                            |window| {
                                let text: SharedString =
                                    "one two three four five six seven eight".into();
                                let previous = TextLayout::default();
                                let previous_measure =
                                    previous.measurement(text.clone(), None, window);
                                let mut tree = taffy::TaffyTree::<()>::new();
                                let id = tree.new_leaf(Default::default()).expect("text leaf");
                                let final_input = LayoutInput {
                                    run_mode: RunMode::PerformLayout,
                                    known_dimensions: taffy::geometry::Size {
                                        width: Some(100.0),
                                        height: Some(50.0),
                                    },
                                    ..LayoutInput::HIDDEN
                                };
                                let mut calls = 0;
                                compute(
                                    &mut tree,
                                    id,
                                    final_input,
                                    &previous_measure,
                                    &mut calls,
                                    window,
                                    cx,
                                );
                                assert!(previous.0.borrow().is_some());
                                let expected = previous.wrapped_text();
                                let fresh = TextLayout::default();
                                let fresh_measure = fresh.measurement(text.clone(), None, window);
                                assert!(fresh_measure.carry_from(&previous_measure));
                                for (width, height, available_width) in [
                                    (None, None, taffy::AvailableSpace::MinContent),
                                    (None, None, taffy::AvailableSpace::MaxContent),
                                    (None, None, taffy::AvailableSpace::Definite(15.0)),
                                    (Some(30.0), None, taffy::AvailableSpace::MaxContent),
                                    (None, Some(20.0), taffy::AvailableSpace::MaxContent),
                                    (Some(40.0), Some(20.0), taffy::AvailableSpace::MaxContent),
                                ] {
                                    let probe = LayoutInput {
                                        run_mode: RunMode::ComputeSize,
                                        known_dimensions: taffy::geometry::Size { width, height },
                                        available_space: taffy::geometry::Size {
                                            width: available_width,
                                            height: taffy::AvailableSpace::MaxContent,
                                        },
                                        ..LayoutInput::HIDDEN
                                    };
                                    let before_probe = calls;
                                    let output = compute(
                                        &mut tree,
                                        id,
                                        probe,
                                        &fresh_measure,
                                        &mut calls,
                                        window,
                                        cx,
                                    );
                                    let reference = TextLayout::default();
                                    let reference_measure =
                                        reference.measurement(text.clone(), None, window);
                                    let mut reference_tree = taffy::TaffyTree::<()>::new();
                                    let reference_id = reference_tree
                                        .new_leaf(Default::default())
                                        .expect("reference leaf");
                                    let mut reference_calls = 0;
                                    let reference_output = compute(
                                        &mut reference_tree,
                                        reference_id,
                                        probe,
                                        &reference_measure,
                                        &mut reference_calls,
                                        window,
                                        cx,
                                    );
                                    assert_eq!(output, reference_output);
                                    assert_eq!(calls, before_probe + 1);
                                    assert_eq!(fresh.wrapped_text(), expected);
                                    compute(
                                        &mut tree,
                                        id,
                                        final_input,
                                        &fresh_measure,
                                        &mut calls,
                                        window,
                                        cx,
                                    );
                                    assert_eq!(calls, before_probe + 1, "final layout must hit");
                                    assert_eq!(fresh.wrapped_text(), expected);
                                }
                            },
                        );
                    }
                }
            }
        });
    }

    #[gpui::test]
    fn text_cache_rejects_changed_inputs_and_wrapped_intrinsic_sizes(
        cx: &mut crate::TestAppContext,
    ) {
        cx.add_empty_window().update(|window, cx| {
            let text: SharedString = "one two three four five six seven eight".into();
            let previous = TextLayout::default();
            let previous_measure = previous.measurement(text.clone(), None, window);
            previous_measure.measure(
                Size::default(),
                Size {
                    width: crate::AvailableSpace::Definite(crate::px(30.0)),
                    height: crate::AvailableSpace::MaxContent,
                },
                Some(taffy::RunMode::PerformLayout),
                window,
                cx,
            );
            for change in 0..11 {
                let fresh = TextLayout::default();
                let mut measure = fresh.measurement(text.clone(), None, window);
                match change {
                    0 => measure.text = "ONE two three four five six seven eight".into(),
                    1 => {
                        let mut run = measure.text_style.to_run(measure.text.len());
                        run.len -= 1;
                        measure.runs = TextRuns::Explicit(vec![run]);
                    }
                    2 => measure.text_style.font_family = "different font".into(),
                    3 => measure.text_style.white_space = WhiteSpace::Nowrap,
                    4 => measure.text_style.line_clamp = Some(1),
                    5 => {
                        measure.text_style.text_overflow = Some(TextOverflow::Truncate("…".into()))
                    }
                    6 => measure.font_size += crate::px(1.0),
                    7 => measure.line_height += crate::px(1.0),
                    8 => measure.rem_size += crate::px(1.0),
                    9 => measure.scale_factor += 0.25,
                    10 => measure.font_generation += 1,
                    _ => unreachable!(),
                }
                assert!(!measure.carry_from(&previous_measure), "change {change}");
                assert!(fresh.0.borrow().is_none());
            }

            let fresh = TextLayout::default();
            let measure = fresh.measurement(text.clone(), None, window);
            assert!(measure.carry_from(&previous_measure));
            let expected_lines = fresh.wrapped_text();
            let intrinsic = measure.measure(
                Size::default(),
                Size {
                    width: crate::AvailableSpace::MaxContent,
                    height: crate::AvailableSpace::MaxContent,
                },
                Some(taffy::RunMode::ComputeSize),
                window,
                cx,
            );
            let reference = TextLayout::default()
                .measurement(text, None, window)
                .measure(
                    Size::default(),
                    Size {
                        width: crate::AvailableSpace::MaxContent,
                        height: crate::AvailableSpace::MaxContent,
                    },
                    None,
                    window,
                    cx,
                );
            assert_eq!(intrinsic, reference);
            assert_eq!(fresh.wrapped_text(), expected_lines);
        });
    }

    #[gpui::test]
    fn text_layout_cache_matches_fresh_frames_and_font_installation(
        cx: &mut crate::TestAppContext,
    ) {
        cx.add_empty_window().update(|window, cx| {
            window
                .invalidator
                .set_phase(crate::window::DrawPhase::Prepaint);
            for overflow in [
                None,
                Some(TextOverflow::Truncate("…".into())),
                Some(TextOverflow::TruncateStart("…".into())),
                Some(TextOverflow::TruncateMiddle("…".into())),
            ] {
                window.with_text_style(
                    Some(crate::TextStyleRefinement {
                        white_space: Some(WhiteSpace::Normal),
                        text_overflow: overflow,
                        line_clamp: Some(2),
                        ..Default::default()
                    }),
                    |window| {
                        let mut reference = crate::TaffyLayoutEngine::new();
                        let mut cached = crate::TaffyLayoutEngine::new();
                        cached.enable_cache();
                        let scope = crate::element::InternedHandle::for_test(0, 0);
                        let mut previous_id = None;
                        for frame in 0..8 {
                            let scale = if frame < 4 { 1.0 } else { 1.25 };
                            window.set_scale_factor(scale);
                            let width = if frame % 4 < 2 { 100.0 } else { 140.0 };
                            let text: SharedString = if frame < 6 {
                                "one two three four five six seven eight".into()
                            } else {
                                "ONE two three four five six seven eight".into()
                            };
                            let mut results = Vec::new();
                            for (index, engine) in
                                [&mut reference, &mut cached].into_iter().enumerate()
                            {
                                let layout = TextLayout::default();
                                engine.set_request_scope(&scope);
                                let id = engine.request_text_layout(layout.measurement(
                                    text.clone(),
                                    None,
                                    window,
                                ));
                                if index == 1 {
                                    if let Some(previous_id) = previous_id {
                                        assert_eq!(id, previous_id);
                                    }
                                    previous_id = Some(id);
                                    if frame == 1 {
                                        assert!(
                                            layout.0.borrow().is_some(),
                                            "carry before cache hit"
                                        );
                                        layout.prepaint(Bounds::default(), &text);
                                        window
                                            .text_system()
                                            .add_fonts(Vec::new())
                                            .expect("font installation");
                                    }
                                }
                                engine.compute_layout(
                                    id,
                                    Size {
                                        width: crate::AvailableSpace::Definite(crate::px(width)),
                                        height: crate::AvailableSpace::MaxContent,
                                    },
                                    window,
                                    cx,
                                );
                                let inner = layout.0.borrow();
                                let inner = inner.as_ref().expect("final text state");
                                if frame == 1 && index == 1 {
                                    assert!(
                                        inner.bounds.is_none(),
                                        "font generation must invalidate cached final state"
                                    );
                                }
                                results.push((
                                    engine.layout_bounds(id, scale),
                                    inner.size,
                                    layout.wrapped_text(),
                                ));
                                engine.clear();
                            }
                            assert_eq!(results[0], results[1], "frame {frame}");
                        }
                    },
                );
            }
        });
    }

    #[gpui::test]
    fn layout_cache_churn_bounds_nodes_and_releases_text_state(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            window
                .invalidator
                .set_phase(crate::window::DrawPhase::Prepaint);
            let mut engine = crate::TaffyLayoutEngine::new();
            engine.enable_cache();
            let mut previous_states = Vec::new();
            for frame in 0..1000_u64 {
                let scope = crate::element::InternedHandle::for_test((frame / 2) as u32, 0);
                // Alternate replacement, ordinal shrinkage, and complete removal.
                let count = [16, 3, 0, 7][frame as usize % 4];
                let mut states = Vec::new();
                for ordinal in 0..count {
                    let layout = TextLayout::default();
                    states.push(Rc::downgrade(&layout.0));
                    engine.set_request_scope(&scope);
                    let id = engine.request_text_layout(layout.measurement(
                        format!("frame {frame}, label {ordinal}").into(),
                        None,
                        window,
                    ));
                    engine.compute_layout(id, crate::AvailableSpace::min_size(), window, cx);
                    assert!(layout.0.borrow().is_some());
                }
                engine.clear();
                assert_eq!(
                    engine.cache_counts(),
                    (count, count, usize::from(count > 0)),
                    "frame {frame}"
                );
                assert!(
                    previous_states
                        .iter()
                        .all(|state: &std::rc::Weak<_>| state.upgrade().is_none()),
                    "old descriptors must release their TextLayout handles"
                );
                previous_states = states;
            }
            engine.clear();
            assert_eq!(engine.cache_counts(), (0, 0, 0));
            assert!(
                previous_states
                    .iter()
                    .all(|state| state.upgrade().is_none())
            );
        });
    }

    #[gpui::test]
    fn layout_cache_reused_removed_id_matches_fresh_text_and_style(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            window
                .invalidator
                .set_phase(crate::window::DrawPhase::Prepaint);
            let mut cached = crate::TaffyLayoutEngine::new();
            cached.enable_cache();
            let mut fresh = crate::TaffyLayoutEngine::new();
            let scope = crate::element::InternedHandle::for_test(0, 0);
            let mut removed_state = None;
            for frame in 0..3 {
                let scale = [1.0, 1.25, 2.0][frame];
                window.set_scale_factor(scale);
                let text: SharedString = if frame == 0 {
                    "short".into()
                } else {
                    "changed text that should wrap onto several lines".into()
                };
                let mut results = Vec::new();
                window.with_text_style(
                    Some(crate::TextStyleRefinement {
                        font_size: Some(crate::px(12.0 + frame as f32 * 8.0).into()),
                        white_space: Some(WhiteSpace::Normal),
                        ..Default::default()
                    }),
                    |window| {
                        for engine in [&mut cached, &mut fresh] {
                            let layout = TextLayout::default();
                            let state = Rc::downgrade(&layout.0);
                            engine.set_request_scope(&scope);
                            let id = engine.request_text_layout(layout.measurement(
                                text.clone(),
                                None,
                                window,
                            ));
                            let mut style = crate::Style::default();
                            style.size.width = crate::px(80.0 + frame as f32 * 20.0).into();
                            style.padding = crate::Edges::all(crate::px(frame as f32 * 3.0).into());
                            engine.set_request_scope(&scope);
                            let root = engine.request_layout(style, crate::px(16.0), scale, &[id]);
                            engine.compute_layout(
                                root,
                                crate::AvailableSpace::min_size(),
                                window,
                                cx,
                            );
                            results.push((
                                engine.layout_bounds(root, scale),
                                engine.layout_bounds(id, scale),
                                layout.wrapped_text(),
                            ));
                            engine.clear();
                            if engine.cache_enabled() {
                                removed_state = Some(state);
                            }
                        }
                    },
                );
                assert_eq!(results[0], results[1], "frame {frame}");
                // An absent frame must evict both the scoped box and its text state.
                cached.clear();
                assert_eq!(cached.cache_counts(), (0, 0, 0));
                assert!(
                    removed_state
                        .as_ref()
                        .expect("cached text state")
                        .upgrade()
                        .is_none()
                );
            }
        });
    }

    #[gpui::test]
    fn text_measurement_reuses_fitting_unwrapped_lines(cx: &mut crate::TestAppContext) {
        let window = cx.add_window(|_, _| crate::EmptyView);
        cx.update_window(window.into(), |_, window, cx| {
            window
                .invalidator
                .set_phase(crate::window::DrawPhase::Prepaint);
            for text in ["one two three four five six", "one two three\nfour five"] {
                for (white_space, line_clamp) in [
                    (WhiteSpace::Normal, None),
                    (WhiteSpace::Nowrap, None),
                    (WhiteSpace::Normal, Some(1)),
                ] {
                    window.with_text_style(
                        Some(crate::TextStyleRefinement {
                            white_space: Some(white_space),
                            line_clamp,
                            ..Default::default()
                        }),
                        |window| {
                            let layout = TextLayout::default();
                            let layout_id = layout.layout(text.into(), None, window, cx);
                            window.compute_layout(
                                layout_id,
                                crate::size(
                                    crate::AvailableSpace::MaxContent,
                                    crate::AvailableSpace::MaxContent,
                                ),
                                cx,
                            );
                            let size = layout
                                .0
                                .borrow()
                                .as_ref()
                                .expect("measured")
                                .size
                                .expect("measured size");
                            assert!(size.width > crate::px(0.));
                            let bounds = Bounds::new(Point::default(), size);
                            layout.prepaint(bounds, text);
                            let lines: Vec<_> = layout
                                .0
                                .borrow()
                                .as_ref()
                                .expect("measured")
                                .lines
                                .iter()
                                .map(|line| line.layout.clone())
                                .collect();
                            for width in [size.width + crate::px(10.), size.width] {
                                let available = crate::size(
                                    crate::AvailableSpace::Definite(width),
                                    crate::AvailableSpace::MinContent,
                                );
                                window.compute_layout(layout_id, available, cx);
                                let cached = layout.0.borrow();
                                let cached = cached.as_ref().expect("measured");
                                assert_eq!(cached.size, Some(size));
                                assert_eq!(cached.wrap_width, None);
                                assert_eq!(cached.bounds, Some(bounds));
                                assert_eq!(cached.lines.len(), lines.len());
                                for (cached, original) in cached.lines.iter().zip(&lines) {
                                    assert!(Arc::ptr_eq(&cached.layout, original));
                                }
                                let fresh = TextLayout::default();
                                let fresh_id = fresh.layout(text.into(), None, window, cx);
                                window.compute_layout(fresh_id, available, cx);
                                assert_eq!(layout.wrapped_text(), fresh.wrapped_text());
                                assert_eq!(
                                    cached.size,
                                    fresh.0.borrow().as_ref().expect("measured").size
                                );
                            }

                            let narrow_width = size.width / 2.;
                            let available = crate::size(
                                crate::AvailableSpace::Definite(narrow_width),
                                crate::AvailableSpace::MinContent,
                            );
                            window.compute_layout(layout_id, available, cx);
                            let cached = layout.0.borrow();
                            let cached = cached.as_ref().expect("measured");
                            if white_space == WhiteSpace::Normal {
                                assert_eq!(cached.wrap_width, Some(narrow_width));
                                assert_eq!(cached.bounds, None);
                            } else {
                                assert_eq!(cached.wrap_width, None);
                                assert_eq!(cached.bounds, Some(bounds));
                            }
                            let fresh = TextLayout::default();
                            let fresh_id = fresh.layout(text.into(), None, window, cx);
                            window.compute_layout(fresh_id, available, cx);
                            assert_eq!(layout.wrapped_text(), fresh.wrapped_text());
                            assert_eq!(
                                cached.size,
                                fresh.0.borrow().as_ref().expect("measured").size
                            );
                        },
                    );
                }
            }
            window.invalidator.set_phase(crate::window::DrawPhase::None);
        })
        .expect("update window");
    }

    #[gpui::test]
    fn text_measurement_truncation_does_not_answer_intrinsic_sizing(
        cx: &mut crate::TestAppContext,
    ) {
        let window = cx.add_window(|_, _| crate::EmptyView);
        cx.update_window(window.into(), |_, window, cx| {
            window
                .invalidator
                .set_phase(crate::window::DrawPhase::Prepaint);
            let text = "one two three four five six";
            for overflow in [
                TextOverflow::Truncate("…".into()),
                TextOverflow::TruncateStart("…".into()),
                TextOverflow::TruncateMiddle("…".into()),
            ] {
                for (white_space, line_clamp) in [
                    (WhiteSpace::Normal, None),
                    (WhiteSpace::Nowrap, None),
                    (WhiteSpace::Normal, Some(2)),
                ] {
                    window.with_text_style(
                        Some(crate::TextStyleRefinement {
                            text_overflow: Some(overflow.clone()),
                            white_space: Some(white_space),
                            line_clamp,
                            ..Default::default()
                        }),
                        |window| {
                            for intrinsic in [
                                crate::AvailableSpace::MinContent,
                                crate::AvailableSpace::MaxContent,
                            ] {
                                let layout = TextLayout::default();
                                let layout_id = layout.layout(text.into(), None, window, cx);
                                window.compute_layout(
                                    layout_id,
                                    crate::size(
                                        crate::AvailableSpace::Definite(crate::px(20.)),
                                        crate::AvailableSpace::MinContent,
                                    ),
                                    cx,
                                );
                                assert!(
                                    layout
                                        .0
                                        .borrow()
                                        .as_ref()
                                        .expect("measured")
                                        .truncate_width
                                        .is_some()
                                );
                                let available =
                                    crate::size(intrinsic, crate::AvailableSpace::MinContent);
                                window.compute_layout(layout_id, available, cx);
                                let cached = layout.0.borrow();
                                let cached = cached.as_ref().expect("measured");
                                assert_eq!(cached.truncate_width, None);
                                assert_eq!(cached.len, text.len());
                                let fresh = TextLayout::default();
                                let fresh_id = fresh.layout(text.into(), None, window, cx);
                                window.compute_layout(fresh_id, available, cx);
                                assert_eq!(layout.wrapped_text(), fresh.wrapped_text());
                                assert_eq!(
                                    cached.size,
                                    fresh.0.borrow().as_ref().expect("measured").size
                                );
                            }
                        },
                    );
                }
            }
            window.invalidator.set_phase(crate::window::DrawPhase::None);
        })
        .expect("update window");
    }

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
