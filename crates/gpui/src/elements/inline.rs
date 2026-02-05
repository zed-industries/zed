#![allow(missing_docs)]
use crate::{
    AnyElement,
    App,
    Bounds,
    ContentMask,
    Element,
    ElementId,
    GlobalElementId,
    Hitbox,
    InteractiveElement,
    Interactivity,
    LayoutId,
    Pixels,
    Point,
    SharedString,
    Size,
    Style,
    StyleRefinement,
    Styled,
    TextRun,
    TextStyle,
    TextStyleRefinement,
    Window,
    point,
    size,
    taffy::inline_layout::InlineLayoutView, // Concrete types
    taffy::inline_layout::InlineMeasureContext,
    text_system::{
        DecorationSliceSpec, InlineFlowItem, InlineFlowLayout, InlineSegment, LineLayout,
        ShapedInline, TextGlyphRange, paint_inline_background_range, paint_inline_span,
        paint_inline_text_range,
    },
};
use parking_lot::Mutex;
use refineable::Refineable;
use smallvec::SmallVec;
use stacksafe::{StackSafe, stacksafe};
use std::{cell::RefCell, ops::Range, rc::Rc, sync::Arc};

pub fn inline() -> InlineBuilder {
    InlineBuilder::new()
}

enum InlineItem {
    Text {
        text: SharedString,
        runs: Option<Vec<TextRun>>,
        style: Option<crate::TextStyleRefinement>,
    },
    HardBreak,
    Element {
        element: StackSafe<AnyElement>,
        box_index: Option<usize>,
        logical_len: usize,
    },
}

pub struct InlineBuilder {
    items: Vec<InlineItem>,
    layout: InlineLayout,
    interactivity: Interactivity,
}

impl InlineBuilder {
    fn current_text_style(&self) -> Option<TextStyleRefinement> {
        Some(self.interactivity.base_style.text.clone())
    }

    fn new() -> Self {
        Self {
            items: Vec::new(),
            layout: InlineLayout::default(),
            interactivity: Interactivity::new(),
        }
    }

    pub fn runs(mut self, runs: Vec<TextRun>) -> Self {
        if let Some(InlineItem::Text { runs: existing, .. }) = self.items.first_mut() {
            *existing = Some(runs);
        } else {
            self.items.push(InlineItem::Text {
                text: SharedString::default(),
                runs: Some(runs),
                style: self.current_text_style(),
            });
        }
        self
    }

    pub fn font_size(self, font_size: Pixels) -> Self {
        self.text_size(font_size)
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        let text = text.into();
        if text.is_empty() {
            return self;
        }
        let current_style = self.current_text_style();

        let mut lines = text.as_ref().split('\n').peekable();
        while let Some(line) = lines.next() {
            if !line.is_empty() {
                if let Some(InlineItem::Text {
                    text: existing,
                    runs: None,
                    style,
                }) = self.items.last_mut()
                    && *style == current_style
                {
                    *existing = (existing.as_ref().to_owned() + line).into();
                } else {
                    self.items.push(InlineItem::Text {
                        text: SharedString::from(line.to_string()),
                        runs: None,
                        style: current_style.clone(),
                    });
                }
            }

            if lines.peek().is_some() {
                self.items.push(InlineItem::HardBreak);
            }
        }
        self
    }

    pub fn break_line(mut self) -> Self {
        self.items.push(InlineItem::HardBreak);
        self
    }

    pub fn text_runs(mut self, text: SharedString, runs: Vec<TextRun>) -> Self {
        if text.is_empty() {
            return self;
        }

        debug_assert_eq!(
            runs.iter().map(|run| run.len).sum::<usize>(),
            text.as_ref().len(),
            "text_runs length mismatch"
        );

        let mut run_cursor = TextRunCursor::new(runs);
        let mut lines = text.as_ref().split('\n').peekable();
        while let Some(line) = lines.next() {
            if !line.is_empty() {
                let line_runs = run_cursor.take(line.len());
                self.items.push(InlineItem::Text {
                    text: SharedString::from(line.to_string()),
                    runs: Some(line_runs),
                    style: self.current_text_style(),
                });
            }

            if lines.peek().is_some() {
                run_cursor.consume(1);
                self.items.push(InlineItem::HardBreak);
            }
        }
        self
    }

    pub fn child(mut self, child: impl crate::IntoElement) -> Self {
        self.items.push(InlineItem::Element {
            element: StackSafe::new(child.into_any_element()),
            box_index: None,
            logical_len: 1,
        });
        self
    }

    pub fn child_with_logical_len(
        mut self,
        child: impl crate::IntoElement,
        logical_len: usize,
    ) -> Self {
        self.items.push(InlineItem::Element {
            element: StackSafe::new(child.into_any_element()),
            box_index: None,
            logical_len: logical_len.max(1),
        });
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl crate::IntoElement>) -> Self {
        for child in children {
            self.items.push(InlineItem::Element {
                element: StackSafe::new(child.into_any_element()),
                box_index: None,
                logical_len: 1,
            });
        }
        self
    }
}

struct TextRunCursor {
    runs: Vec<TextRun>,
    index: usize,
    offset: usize,
}

impl TextRunCursor {
    fn new(runs: Vec<TextRun>) -> Self {
        Self {
            runs,
            index: 0,
            offset: 0,
        }
    }

    fn take(&mut self, mut len: usize) -> Vec<TextRun> {
        let mut result = Vec::new();
        while len > 0 {
            let Some(run) = self.runs.get(self.index) else {
                break;
            };
            let available = run.len.saturating_sub(self.offset);
            if available == 0 {
                self.index += 1;
                self.offset = 0;
                continue;
            }
            let take = available.min(len);
            let mut clipped = run.clone();
            clipped.len = take;
            result.push(clipped);
            self.offset += take;
            len -= take;
            if self.offset == run.len {
                self.index += 1;
                self.offset = 0;
            }
        }
        result
    }

    fn consume(&mut self, mut len: usize) {
        while len > 0 {
            let Some(run) = self.runs.get(self.index) else {
                break;
            };
            let available = run.len.saturating_sub(self.offset);
            if available == 0 {
                self.index += 1;
                self.offset = 0;
                continue;
            }
            let take = available.min(len);
            self.offset += take;
            len -= take;
            if self.offset == run.len {
                self.index += 1;
                self.offset = 0;
            }
        }
    }
}

fn slice_text_runs(runs: &[TextRun], range: Range<usize>) -> Vec<TextRun> {
    let mut result = Vec::new();
    let mut run_start = 0;
    for run in runs {
        let run_end = run_start + run.len;
        if run_end <= range.start {
            run_start = run_end;
            continue;
        }
        if run_start >= range.end {
            break;
        }
        let overlap_start = range.start.max(run_start);
        let overlap_end = range.end.min(run_end);
        if overlap_end > overlap_start {
            let mut clipped = run.clone();
            clipped.len = overlap_end - overlap_start;
            result.push(clipped);
        }
        run_start = run_end;
    }
    result
}

fn push_text_segment(
    segment_text: &str,
    segment_range: Range<usize>,
    explicit_runs: Option<&[TextRun]>,
    chunk_style: &TextStyle,
    items: &mut Vec<InlineFlowItem>,
) {
    if segment_text.is_empty() {
        return;
    }

    let mut segment_runs = if let Some(explicit) = explicit_runs {
        slice_text_runs(explicit, segment_range)
    } else {
        vec![chunk_style.to_run(segment_text.len())]
    };
    if segment_runs.is_empty() {
        segment_runs.push(chunk_style.to_run(segment_text.len()));
    }

    items.push(InlineFlowItem::Text {
        text: SharedString::from(segment_text.to_string()),
        runs: segment_runs.into(),
    });
}

impl Styled for InlineBuilder {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for InlineBuilder {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl crate::IntoElement for InlineBuilder {
    type Element = Inline;

    fn into_element(self) -> Self::Element {
        Inline {
            items: self.items,
            layout: self.layout.clone(),
            interactivity: self.interactivity,
        }
    }
}

// Redundant, Inline implements IntoElement which provides Into<AnyElement>
// impl From<Inline> for AnyElement {
//     fn from(inline: Inline) -> Self {
//         inline.into_any_element()
//     }
// }

pub struct Inline {
    items: Vec<InlineItem>,
    layout: InlineLayout,
    interactivity: Interactivity,
}

#[derive(Default, Clone)]
pub struct InlineLayout(Rc<RefCell<Option<InlineLayoutView>>>);

impl InlineLayout {
    fn set(&self, handle: InlineLayoutView) {
        *self.0.borrow_mut() = Some(handle);
    }

    pub fn get(&self) -> Option<InlineLayoutView> {
        self.0.borrow().clone()
    }

    pub fn len(&self) -> usize {
        self.0.borrow().as_ref().map_or(0, |h| h.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn text(&self) -> SharedString {
        self.0
            .borrow()
            .as_ref()
            .map_or(SharedString::default(), |h| h.text())
    }

    pub fn plain_text(&self) -> SharedString {
        self.0
            .borrow()
            .as_ref()
            .map_or(SharedString::default(), |h| h.plain_text())
    }

    pub fn plain_text_range(&self, range: Range<usize>) -> SharedString {
        self.0
            .borrow()
            .as_ref()
            .map_or(SharedString::default(), |h| h.plain_text_range(range))
    }

    pub fn surrounding_word_range(&self, logical_index: usize) -> Range<usize> {
        self.0
            .borrow()
            .as_ref()
            .map_or(logical_index..logical_index, |h| {
                h.surrounding_word_range(logical_index)
            })
    }

    pub fn bounds(&self) -> Bounds<Pixels> {
        self.0
            .borrow()
            .as_ref()
            .map_or(Bounds::default(), |h| h.bounds())
    }

    /// Returns the byte index in the source text for a given position.
    pub fn index_for_position(&self, position: Point<Pixels>) -> Result<usize, usize> {
        let handle = self.0.borrow();
        let Some(h) = handle.as_ref() else {
            return Err(0);
        };

        let layout = &h.layout;
        let truncation = layout.truncation.as_ref();
        for (line_ix, line) in layout.lines.iter().enumerate() {
            let truncation = truncation.filter(|truncation| truncation.line_ix == line_ix);
            let line_top = h.origin.y + line.y;
            let line_bottom = line_top + line.height;
            let line_origin = h.origin + point(Pixels::ZERO, line.y);
            let line_start = line
                .segments
                .first()
                .map(|segment| match segment {
                    InlineSegment::Text { logical_range, .. } => logical_range.start,
                    InlineSegment::InlineBox { logical_range, .. } => logical_range.start,
                    InlineSegment::HardBreak { logical_range, .. } => logical_range.start,
                })
                .unwrap_or(0);

            if position.y < line_top {
                return Err(line_start);
            }

            if position.y >= line_top && position.y < line_bottom {
                if line.segments.len() == 1 {
                    if let InlineSegment::HardBreak { logical_range, .. } = &line.segments[0] {
                        return Ok(logical_range.start);
                    }
                }

                let line_end_x = line_origin.x
                    + truncation
                        .map(|truncation| truncation.visible_width)
                        .unwrap_or(line.width);
                let clip_x = truncation.map(|truncation| truncation.clip_x);
                let visible_logical_end =
                    truncation.map(|truncation| truncation.visible_logical_end);

                if position.x >= line_end_x {
                    let line_end = visible_logical_end.unwrap_or_else(|| {
                        line.segments
                            .last()
                            .map(|segment| match segment {
                                InlineSegment::HardBreak { logical_range, .. } => {
                                    logical_range.start
                                }
                                InlineSegment::Text { logical_range, .. } => logical_range.end,
                                InlineSegment::InlineBox { logical_range, .. } => logical_range.end,
                            })
                            .unwrap_or(0)
                    });
                    return Ok(line_end.min(layout.logical_len));
                }

                for segment in line.segments.iter() {
                    match segment {
                        InlineSegment::Text {
                            island_ix,
                            text_range,
                            logical_range,
                            layout_start_x,
                            x,
                            width,
                        } => {
                            let seg_x = line_origin.x + *x;
                            if let Some(clip_x) = clip_x
                                && *x >= clip_x
                            {
                                break;
                            }
                            if position.x < seg_x {
                                return Ok(logical_range.start);
                            }
                            if position.x >= seg_x && position.x < seg_x + *width {
                                let local_x = position.x - seg_x;
                                let layout_x = *layout_start_x + local_x;
                                let island_layout = &layout.islands[*island_ix].layout;
                                let text_ix = island_layout
                                    .index_for_x(layout_x)
                                    .unwrap_or_else(|| island_layout.closest_index_for_x(layout_x));
                                let clamped = text_ix.clamp(text_range.start, text_range.end);
                                let logical_ix = logical_range.start + (clamped - text_range.start);
                                let logical_ix = logical_ix.min(layout.logical_len);
                                let logical_ix = visible_logical_end
                                    .map(|end| logical_ix.min(end))
                                    .unwrap_or(logical_ix);
                                return Ok(logical_ix);
                            }
                        }
                        InlineSegment::InlineBox {
                            logical_range,
                            x,
                            width,
                            ..
                        } => {
                            let seg_x = line_origin.x + *x;
                            if let Some(clip_x) = clip_x
                                && *x >= clip_x
                            {
                                break;
                            }
                            if position.x < seg_x {
                                return Ok(logical_range.start);
                            }
                            if position.x >= seg_x && position.x < seg_x + *width {
                                let midpoint = seg_x + (*width * 0.5);
                                if position.x < midpoint {
                                    return Ok(logical_range.start);
                                }
                                return Ok(logical_range.end);
                            }
                        }
                        InlineSegment::HardBreak { logical_range, .. } => {
                            if position.x >= line_end_x {
                                return Ok(logical_range.start);
                            }
                        }
                    }
                }

                let line_end = visible_logical_end.unwrap_or_else(|| {
                    line.segments
                        .last()
                        .map(|segment| match segment {
                            InlineSegment::HardBreak { logical_range, .. } => logical_range.start,
                            InlineSegment::Text { logical_range, .. } => logical_range.end,
                            InlineSegment::InlineBox { logical_range, .. } => logical_range.end,
                        })
                        .unwrap_or(0)
                });
                return Ok(line_end.min(layout.logical_len));
            }
        }
        Err(truncation
            .map(|truncation| truncation.visible_logical_end)
            .unwrap_or(layout.logical_len))
    }

    /// Returns the position for a given byte index in the source text.
    pub fn position_for_index(&self, index: usize) -> Option<Point<Pixels>> {
        let handle = self.0.borrow();
        let h = handle.as_ref()?;
        let layout = &h.layout;
        let truncation = layout.truncation.as_ref();
        let mut index = if let Some(truncation) = truncation {
            index.min(truncation.visible_logical_end)
        } else {
            index
        };

        for (line_ix, line) in layout.lines.iter().enumerate() {
            let truncation = truncation.filter(|truncation| truncation.line_ix == line_ix);
            let line_origin = h.origin + point(Pixels::ZERO, line.y);
            for segment in &line.segments {
                match segment {
                    InlineSegment::Text {
                        island_ix,
                        text_range,
                        logical_range,
                        layout_start_x,
                        x,
                        ..
                    } => {
                        if let Some(truncation) = truncation
                            && *x >= truncation.clip_x
                        {
                            break;
                        }
                        if index >= logical_range.start && index <= logical_range.end {
                            let offset_in_range = index.saturating_sub(logical_range.start);
                            let text_ix = (text_range.start + offset_in_range).min(text_range.end);
                            let island_layout = &layout.islands[*island_ix].layout;
                            let layout_x = island_layout.x_for_index(text_ix);
                            let x_offset = *x + (layout_x - *layout_start_x);
                            return Some(line_origin + point(x_offset, Pixels::ZERO));
                        }
                    }
                    InlineSegment::InlineBox {
                        logical_range,
                        x,
                        width,
                        ..
                    } => {
                        if let Some(truncation) = truncation
                            && *x >= truncation.clip_x
                        {
                            break;
                        }
                        if index <= logical_range.start {
                            return Some(line_origin + point(*x, Pixels::ZERO));
                        }
                        if index <= logical_range.end {
                            return Some(line_origin + point(*x + *width, Pixels::ZERO));
                        }
                    }
                    InlineSegment::HardBreak { logical_range, x } => {
                        if index == logical_range.start {
                            return Some(line_origin + point(*x, Pixels::ZERO));
                        }
                    }
                }
            }
        }

        if index >= layout.logical_len {
            if let Some(line) = layout.lines.last() {
                let line_origin = h.origin + point(Pixels::ZERO, line.y);
                if let Some(segment) = line.segments.last() {
                    let x = match segment {
                        InlineSegment::Text { x, width, .. } => *x + *width,
                        InlineSegment::InlineBox { x, width, .. } => *x + *width,
                        InlineSegment::HardBreak { x, .. } => *x,
                    };
                    return Some(line_origin + point(x, Pixels::ZERO));
                }
            }
        }

        None
    }

    /// Returns the line height for the line containing the given byte index.
    pub fn line_height_for_index(&self, index: usize) -> Pixels {
        let handle = self.0.borrow();
        let Some(h) = handle.as_ref() else {
            return Pixels::ZERO;
        };
        let layout = &h.layout;
        let truncation = layout.truncation.as_ref();

        for (line_ix, line) in layout.lines.iter().enumerate() {
            let truncation = truncation.filter(|truncation| truncation.line_ix == line_ix);
            for segment in &line.segments {
                match segment {
                    InlineSegment::Text { logical_range, .. } => {
                        if index >= logical_range.start && index <= logical_range.end {
                            return truncation
                                .map(|truncation| truncation.visible_text_height)
                                .unwrap_or(line.text_height);
                        }
                    }
                    InlineSegment::InlineBox { logical_range, .. }
                    | InlineSegment::HardBreak { logical_range, .. } => {
                        if index >= logical_range.start && index <= logical_range.end {
                            return truncation
                                .map(|truncation| truncation.visible_height)
                                .unwrap_or(line.height);
                        }
                    }
                }
            }
        }

        layout
            .lines
            .first()
            .map_or(Pixels::ZERO, |line| line.text_height)
    }
}

pub struct InlineFrameState {
    pub layout_id: LayoutId,
    pub style: Style,
    items: Vec<InlineFlowItem>,
    layout_result: Arc<Mutex<Option<Arc<InlineFlowLayout>>>>,
}

#[derive(Clone)]
pub struct InlineInspectorState {
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub base_style: Box<StyleRefinement>,
    pub bounds: Bounds<Pixels>,
    pub content_size: Size<Pixels>,
    pub logical_len: usize,
    pub line_count: usize,
    pub box_count: usize,
    pub truncation: Option<InlineInspectorTruncation>,
    pub text_preview: SharedString,
}

#[derive(Clone)]
pub struct InlineInspectorTruncation {
    pub line_ix: usize,
    pub clip_x: Pixels,
    pub visible_width: Pixels,
    pub ellipsis: SharedString,
}

#[derive(Clone, Copy)]
struct RunEndInfo {
    run_ix: usize,
    end_index: usize,
}

fn build_run_end_infos(layout: &LineLayout) -> Vec<RunEndInfo> {
    let mut infos = Vec::new();
    for (run_ix, run) in layout.runs.iter().enumerate() {
        if let Some(last) = run.glyphs.last() {
            infos.push(RunEndInfo {
                run_ix,
                end_index: last.index,
            });
        }
    }
    infos
}

fn text_glyph_range(
    layout: &LineLayout,
    run_infos: &[RunEndInfo],
    text_start: usize,
    text_end: usize,
) -> Option<TextGlyphRange> {
    if text_start >= text_end || layout.runs.is_empty() || run_infos.is_empty() {
        return None;
    }

    let start_info = run_infos.partition_point(|info| info.end_index < text_start);
    if start_info >= run_infos.len() {
        return None;
    }
    let start_run = run_infos[start_info].run_ix;

    let end_search = text_end.saturating_sub(1);
    let mut end_info = run_infos.partition_point(|info| info.end_index < end_search);
    if end_info >= run_infos.len() {
        end_info = run_infos.len() - 1;
    }
    let end_run = run_infos[end_info].run_ix;
    if start_run > end_run {
        return None;
    }

    let start_glyph = layout.runs[start_run]
        .glyphs
        .partition_point(|g| g.index < text_start);
    if start_glyph >= layout.runs[start_run].glyphs.len() {
        return None;
    }

    let end_glyph = layout.runs[end_run]
        .glyphs
        .partition_point(|g| g.index < text_end);
    if start_run == end_run && end_glyph <= start_glyph {
        return None;
    }

    let start_position = layout.runs[start_run].glyphs[start_glyph].position;
    Some(TextGlyphRange {
        start_run,
        start_glyph,
        end_run,
        end_glyph,
        start_position,
    })
}

struct InlinePaintPlan {
    line_origin: Point<Pixels>,
    line_bounds: Bounds<Pixels>,
    line_height: Pixels,
    clip_x: Option<Pixels>,
    ops: Vec<InlinePaintOp>,
    ellipsis_bounds: Option<Bounds<Pixels>>,
}

impl InlinePaintPlan {
    fn clip_bounds(&self) -> Bounds<Pixels> {
        if let Some(clip_x) = self.clip_x {
            let clip_width = clip_x.min(self.line_bounds.size.width);
            Bounds::new(
                self.line_bounds.origin,
                size(clip_width, self.line_bounds.size.height),
            )
        } else {
            self.line_bounds
        }
    }

    fn box_clips(plans: &[InlinePaintPlan], box_count: usize) -> Vec<Option<Bounds<Pixels>>> {
        let mut clips = vec![None; box_count];
        for line in plans {
            for op in &line.ops {
                if let InlinePaintOp::Box {
                    index,
                    clip_bounds: Some(clip_bounds),
                    ..
                } = op
                {
                    clips[*index] = Some(clip_bounds.clone());
                }
            }
        }
        clips
    }

    fn build(shaped: &ShapedInline, origin: Point<Pixels>) -> Vec<InlinePaintPlan> {
        let truncation = shaped.truncation.as_ref();
        let mut lines = Vec::with_capacity(shaped.lines.len());
        let mut run_infos_by_island: Vec<Option<Vec<RunEndInfo>>> =
            vec![None; shaped.islands.len()];

        for (line_ix, line) in shaped.lines.iter().enumerate() {
            let truncation = truncation.filter(|truncation| truncation.line_ix == line_ix);
            let clip_x = truncation.map(|truncation| truncation.clip_x);
            let line_height = truncation.map_or(line.text_height, |truncation| {
                truncation.visible_text_height
            });
            let line_origin = origin + point(Pixels::ZERO, line.y);
            let line_bounds = Bounds::new(line_origin, size(line.width, line_height));
            let ellipsis_bounds = truncation.and_then(|truncation| {
                if truncation.ellipsis_style.ellipsis_text.is_empty() {
                    return None;
                }
                if truncation.clip_first_item && truncation.truncate_text_end.is_none() {
                    return None;
                }
                Some(Bounds::new(
                    line_origin,
                    size(truncation.visible_width, truncation.visible_text_height),
                ))
            });
            let mut ops = Vec::with_capacity(line.segments.len());
            for (segment_ix, segment) in line.segments.iter().enumerate() {
                match segment {
                    InlineSegment::Text {
                        island_ix,
                        text_range,
                        layout_start_x,
                        x,
                        ..
                    } => {
                        if let Some(truncation) = truncation {
                            if *x >= truncation.clip_x {
                                break;
                            }
                            if let Some(truncate_ix) = truncation.truncate_segment_ix
                                && segment_ix > truncate_ix
                            {
                                break;
                            }
                        }

                        let mut segment_range = text_range.clone();
                        if let Some(truncation) = truncation
                            && truncation.truncate_segment_ix == Some(segment_ix)
                            && let Some(truncate_end) = truncation.truncate_text_end
                        {
                            if truncate_end <= text_range.start && !truncation.clip_first_item {
                                break;
                            }
                            segment_range = text_range.start..truncate_end;
                        }

                        if segment_range.is_empty() {
                            continue;
                        }

                        let island = shaped.island(*island_ix);
                        let run_infos = run_infos_by_island[*island_ix]
                            .get_or_insert_with(|| build_run_end_infos(island.layout));
                        let glyph_range = match text_glyph_range(
                            island.layout,
                            run_infos,
                            segment_range.start,
                            segment_range.end,
                        ) {
                            Some(range) => range,
                            None => continue,
                        };
                        let decoration = island.decoration_slice_spec(segment_range.clone());
                        let x_offset = *x - *layout_start_x;
                        ops.push(InlinePaintOp::Text {
                            island_ix: *island_ix,
                            text_start: segment_range.start,
                            text_end: segment_range.end,
                            decoration,
                            glyphs: glyph_range,
                            x_offset,
                        });
                    }
                    InlineSegment::InlineBox {
                        index, x, width, ..
                    } => {
                        if *index >= shaped.boxes.len() {
                            continue;
                        }

                        if let Some(truncation) = truncation {
                            if *x >= truncation.clip_x {
                                break;
                            }

                            let segment_end = *x + *width;
                            if segment_end > truncation.clip_x {
                                if !(truncation.clip_first_item && segment_ix == 0) {
                                    break;
                                }
                            }
                        }

                        let placement = &shaped.boxes[*index];
                        let bounds = Bounds {
                            origin: origin + placement.relative_bounds.origin,
                            size: placement.relative_bounds.size,
                        };
                        let clip_bounds = if let Some(truncation) = truncation {
                            let segment_end = *x + *width;
                            if segment_end <= truncation.clip_x {
                                Some(bounds)
                            } else if truncation.clip_first_item && segment_ix == 0 {
                                let line_clip = Bounds::new(
                                    line_origin,
                                    size(truncation.clip_x, truncation.visible_height),
                                );
                                let clipped = bounds.intersect(&line_clip);
                                if clipped.size.width.0 <= 0.0 || clipped.size.height.0 <= 0.0 {
                                    None
                                } else {
                                    Some(clipped)
                                }
                            } else {
                                None
                            }
                        } else {
                            Some(bounds)
                        };
                        if let Some(clip_bounds) = clip_bounds {
                            ops.push(InlinePaintOp::Box {
                                index: *index,
                                clip_bounds: Some(clip_bounds),
                            });
                        }
                    }
                    InlineSegment::HardBreak { .. } => {}
                }
            }

            lines.push(InlinePaintPlan {
                line_origin,
                line_bounds,
                line_height,
                clip_x,
                ops,
                ellipsis_bounds,
            });
        }

        lines
    }
}

enum InlinePaintOp {
    Text {
        island_ix: usize,
        text_start: usize,
        text_end: usize,
        decoration: DecorationSliceSpec,
        glyphs: TextGlyphRange,
        x_offset: Pixels,
    },
    Box {
        index: usize,
        clip_bounds: Option<Bounds<Pixels>>,
    },
}

pub struct InlinePaintState {
    pub hitbox: Option<Hitbox>,
    paint_bundle: Option<InlinePaintBundle>,
}

struct InlinePaintBundle {
    shaped: ShapedInline,
    paint_lines: Vec<InlinePaintPlan>,
    box_clips: Vec<Option<Bounds<Pixels>>>,
}

impl InlinePaintBundle {
    fn from_layout(
        layout: Option<&Arc<InlineFlowLayout>>,
        items: &[InlineFlowItem],
        origin: Point<Pixels>,
        window: &Window,
    ) -> Option<Self> {
        layout.map(|layout| {
            let shaped = window.text_system().shape_inline(items, layout.clone());
            let paint_lines = InlinePaintPlan::build(&shaped, origin);
            let box_clips = InlinePaintPlan::box_clips(&paint_lines, shaped.boxes.len());
            Self {
                shaped,
                paint_lines,
                box_clips,
            }
        })
    }
}

fn content_origin(bounds: Bounds<Pixels>, style: &Style, window: &Window) -> Point<Pixels> {
    let padding = style.padding.to_pixels(
        bounds.size.map(crate::AbsoluteLength::Pixels),
        window.rem_size(),
    );
    let border = style.border_widths.to_pixels(window.rem_size());
    Point {
        x: bounds.origin.x + padding.left + border.left,
        y: bounds.origin.y + padding.top + border.top,
    }
}

impl Inline {
    pub fn font_size(self, font_size: Pixels) -> Self {
        self.text_size(font_size)
    }

    pub fn layout(&self) -> InlineLayout {
        self.layout.clone()
    }
}

impl Styled for Inline {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for Inline {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl crate::ParentElement for Inline {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.items
            .extend(elements.into_iter().map(|el| InlineItem::Element {
                element: StackSafe::new(el),
                box_index: None,
                logical_len: 1,
            }));
    }
}

impl Element for Inline {
    type RequestLayoutState = InlineFrameState;
    type PrepaintState = InlinePaintState;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        self.interactivity.source_location()
    }

    #[stacksafe]
    fn request_layout(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&crate::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        #[cfg(any(feature = "inspector", debug_assertions))]
        window.with_inspector_state(
            _inspector_id,
            cx,
            |inspector_state: &mut Option<InlineInspectorState>, _window| {
                if let Some(inspector_state) = inspector_state {
                    self.interactivity.base_style = inspector_state.base_style.clone();
                } else {
                    *inspector_state = Some(InlineInspectorState {
                        base_style: self.interactivity.base_style.clone(),
                        bounds: Bounds::default(),
                        content_size: Size::default(),
                        logical_len: 0,
                        line_count: 0,
                        box_count: 0,
                        truncation: None,
                        text_preview: SharedString::default(),
                    });
                }
            },
        );

        let mut style = Style::default();
        style.refine(&self.interactivity.base_style);

        let mut text_style = window.text_style();
        text_style.refine(&self.interactivity.base_style.text);
        let mut items = Vec::new();
        let mut child_layout_ids = SmallVec::<[LayoutId; 2]>::new();

        let mut next_box_index = 0usize;
        for item in &mut self.items {
            match item {
                InlineItem::Text {
                    text,
                    runs,
                    style: item_style,
                } => {
                    if text.is_empty() {
                        continue;
                    }

                    let mut chunk_style = text_style.clone();
                    if let Some(refinement) = item_style {
                        chunk_style.refine(refinement);
                    }

                    let explicit_runs = runs.as_deref();
                    let text_str = text.as_ref();
                    let mut segment_start = 0;

                    for (ix, ch) in text_str.char_indices() {
                        if ch == '\n' {
                            let range = segment_start..ix;
                            push_text_segment(
                                &text_str[range.clone()],
                                range,
                                explicit_runs,
                                &chunk_style,
                                &mut items,
                            );

                            items.push(InlineFlowItem::HardBreak);

                            segment_start = ix + 1;
                        }
                    }

                    if segment_start < text_str.len() {
                        let range = segment_start..text_str.len();
                        push_text_segment(
                            &text_str[range.clone()],
                            range,
                            explicit_runs,
                            &chunk_style,
                            &mut items,
                        );
                    }
                }
                InlineItem::HardBreak => {
                    items.push(InlineFlowItem::HardBreak);
                }
                InlineItem::Element {
                    element,
                    box_index,
                    logical_len,
                } => {
                    let id = window.with_text_style(style.text_style().cloned(), |window| {
                        element.request_layout(window, cx)
                    });

                    // We don't need `mark_inline_leaf` anymore because the measure context handles it.

                    child_layout_ids.push(id);
                    *box_index = Some(next_box_index);
                    next_box_index += 1;
                    items.push(InlineFlowItem::InlineBox {
                        layout_id: id,
                        metrics: None,
                        logical_len: *logical_len,
                    });
                }
            }
        }

        let layout_result = Arc::new(Mutex::new(None));
        let measure_context = InlineMeasureContext {
            items: items.clone(),
            text_style: text_style.clone(),
            result: layout_result.clone(),
        };

        let layout_id = window.request_layout_with_context(
            style.clone(),
            measure_context,
            child_layout_ids,
            cx,
        );

        (
            layout_id,
            InlineFrameState {
                layout_id,
                style,
                items,
                layout_result,
            },
        )
    }

    #[stacksafe]
    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&crate::InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let layout = request_layout.layout_result.lock().take();

        // 3. Persist to element_states
        // We store the materialized layout in the window's element state associated with
        // this element's global ID. This ensures that the results computed during the
        // Layout phase are preserved and accessible during the Paint phase.
        if let Some(id) = global_id {
            if let Some(layout) = &layout {
                window.with_element_state(id, |_, _| ((), Some(layout.clone())));
            }
        }

        let content_origin = content_origin(bounds, &request_layout.style, window);

        let handle = layout.as_ref().map(|layout| InlineLayoutView {
            layout: layout.clone(),
            origin: content_origin,
        });

        if let Some(ref handle) = handle {
            self.layout.set(handle.clone());
        }

        let paint_bundle = InlinePaintBundle::from_layout(
            layout.as_ref(),
            &request_layout.items,
            content_origin,
            window,
        );

        if let Some(bundle) = &paint_bundle {
            for item in &mut self.items {
                if let InlineItem::Element {
                    element, box_index, ..
                } = item
                {
                    let Some(box_index) = *box_index else {
                        continue;
                    };
                    let Some(clip_bounds) = bundle.box_clips.get(box_index).and_then(|b| b.clone())
                    else {
                        continue;
                    };
                    window.with_content_mask(
                        Some(ContentMask {
                            bounds: clip_bounds,
                        }),
                        |window| element.prepaint(window, cx),
                    );
                }
            }
        }

        let content_size = handle
            .as_ref()
            .map(|h| h.content_size())
            .unwrap_or(bounds.size);

        #[cfg(any(feature = "inspector", debug_assertions))]
        window.with_inspector_state(
            inspector_id,
            cx,
            |inspector_state: &mut Option<InlineInspectorState>, _window| {
                if let (Some(inspector_state), Some(layout)) = (inspector_state, layout.as_ref()) {
                    inspector_state.bounds = bounds;
                    inspector_state.content_size = content_size;
                    inspector_state.logical_len = layout.logical_len;
                    inspector_state.line_count = layout.lines.len();
                    inspector_state.box_count = layout.boxes.len();
                    inspector_state.text_preview = layout.logical_text.clone();
                    inspector_state.truncation =
                        layout
                            .truncation
                            .as_ref()
                            .map(|truncation| InlineInspectorTruncation {
                                line_ix: truncation.line_ix,
                                clip_x: truncation.clip_x,
                                visible_width: truncation.visible_width,
                                ellipsis: truncation.ellipsis_style.ellipsis_text.clone(),
                            });
                }
            },
        );

        let hitbox = self.interactivity.prepaint(
            global_id,
            inspector_id,
            bounds,
            content_size,
            window,
            cx,
            |_style, _scroll_offset, hitbox, _window, _cx| hitbox,
        );

        InlinePaintState {
            hitbox,
            paint_bundle,
        }
    }

    #[stacksafe]
    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&crate::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        state: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.interactivity.paint(
            global_id,
            inspector_id,
            bounds,
            state.hitbox.as_ref(),
            window,
            cx,
            |_style, window, cx| {
                if let Some(bundle) = &state.paint_bundle {
                    let shaped = &bundle.shaped;
                    for plan in &bundle.paint_lines {
                        let clip_bounds = plan.clip_bounds();
                        let _ = window.paint_layer(clip_bounds, |window| {
                            for op in &plan.ops {
                                let InlinePaintOp::Text {
                                    island_ix,
                                    text_start,
                                    text_end,
                                    decoration,
                                    glyphs,
                                    x_offset,
                                } = op
                                else {
                                    continue;
                                };
                                let island = shaped.island(*island_ix);
                                let decorations = island.decorations;
                                let origin = plan.line_origin + point(*x_offset, Pixels::ZERO);
                                let _ = paint_inline_span(
                                    &island,
                                    decorations,
                                    *decoration,
                                    *text_start..*text_end,
                                    *glyphs,
                                    origin,
                                    plan.line_height,
                                    window,
                                    cx,
                                );
                            }
                        });

                        if let Some(ellipsis_bounds) = plan.ellipsis_bounds {
                            let Some(truncation) = shaped.truncation.as_ref() else {
                                continue;
                            };
                            let ellipsis_len = truncation.ellipsis_style.ellipsis_text.len();
                            if ellipsis_len == 0 {
                                continue;
                            }
                            let ellipsis_layout = window
                                .text_system()
                                .ellipsis_layout_for_style(&truncation.ellipsis_style);
                            let origin =
                                plan.line_origin + point(truncation.ellipsis_x, Pixels::ZERO);
                            let span = DecorationSliceSpec {
                                run_start: 0,
                                run_end: ellipsis_layout.runs.len().saturating_sub(1),
                                start_offset: 0,
                                end_offset: ellipsis_len,
                            };
                            let _ = window.paint_layer(ellipsis_bounds, |window| {
                                let _ = paint_inline_background_range(
                                    &ellipsis_layout.layout,
                                    &ellipsis_layout.runs,
                                    span,
                                    0..ellipsis_len,
                                    origin,
                                    truncation.visible_text_height,
                                    window,
                                    cx,
                                );
                                let _ = paint_inline_text_range(
                                    &ellipsis_layout.layout,
                                    &ellipsis_layout.runs,
                                    span,
                                    0..ellipsis_len,
                                    origin,
                                    truncation.visible_text_height,
                                    window,
                                    cx,
                                );
                            });
                        }
                    }

                    for item in &mut self.items {
                        if let InlineItem::Element {
                            element, box_index, ..
                        } = item
                        {
                            let Some(box_index) = *box_index else {
                                continue;
                            };
                            let Some(clip_bounds) =
                                bundle.box_clips.get(box_index).and_then(|b| b.clone())
                            else {
                                continue;
                            };
                            window.paint_layer(clip_bounds, |window| element.paint(window, cx));
                        }
                    }
                }
            },
        );
    }
}

impl crate::IntoElement for Inline {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnyElement, App, AvailableSpace, Bounds, Context, Element, EmptyView, GlobalElementId,
        InlineSegment, InspectorElementId, IntoElement, LayoutId, Pixels, Point, Render,
        TestAppContext, TextOverflow, TextStyle, Window, black, blue, div, element::ParentElement,
        elements::div::StatefulInteractiveElement, green, point, px, red, rems, size,
        text_system::INLINE_BOX_PLACEHOLDER, white,
    };
    use std::{cell::RefCell, rc::Rc, sync::Arc};

    struct StandardTestView<T = ()> {
        state: T,
        render: Box<dyn FnMut(&mut T, &mut Window, &mut Context<Self>) -> AnyElement>,
    }

    impl<T: 'static> StandardTestView<T> {
        fn new(
            state: T,
            render: Box<dyn FnMut(&mut T, &mut Window, &mut Context<Self>) -> AnyElement>,
        ) -> Self {
            Self { state, render }
        }
    }

    impl StandardTestView<Option<InlineLayout>> {
        fn layout(&self) -> Arc<crate::text_system::InlineFlowLayout> {
            self.state
                .as_ref()
                .and_then(|s| s.get())
                .map(|h| h.layout.clone())
                .expect("layout not captured")
        }
    }

    fn strip_placeholders(text: &str) -> String {
        text.chars()
            .filter(|ch| *ch != INLINE_BOX_PLACEHOLDER)
            .collect()
    }

    impl<T: 'static> Render for StandardTestView<T> {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            (self.render)(&mut self.state, window, cx)
        }
    }

    struct BoundsCapture {
        child: AnyElement,
        captured: Rc<RefCell<Option<Bounds<Pixels>>>>,
    }

    impl BoundsCapture {
        fn new(child: impl IntoElement, captured: Rc<RefCell<Option<Bounds<Pixels>>>>) -> Self {
            Self {
                child: child.into_any_element(),
                captured,
            }
        }
    }

    impl Element for BoundsCapture {
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
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            window: &mut Window,
            cx: &mut App,
        ) -> (LayoutId, Self::RequestLayoutState) {
            let layout_id = self.child.request_layout(window, cx);
            (layout_id, ())
        }

        fn prepaint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            window: &mut Window,
            cx: &mut App,
        ) -> Self::PrepaintState {
            *self.captured.borrow_mut() = Some(bounds);
            self.child.prepaint(window, cx);
        }

        fn paint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            _bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _prepaint: &mut Self::PrepaintState,
            window: &mut Window,
            cx: &mut App,
        ) {
            self.child.paint(window, cx);
        }
    }

    impl IntoElement for BoundsCapture {
        type Element = Self;

        fn into_element(self) -> Self::Element {
            self
        }
    }

    fn assert_pixels_close(actual: Pixels, expected: Pixels, label: &str) {
        let diff = (actual.0 - expected.0).abs();
        let tolerance = 0.5;
        assert!(
            diff <= tolerance,
            "{}: expected {:.3}, got {:.3} (diff {:.3})",
            label,
            expected.0,
            actual.0,
            diff
        );
    }

    fn assert_bounds_close(actual: Bounds<Pixels>, expected: Bounds<Pixels>) {
        assert_pixels_close(actual.origin.x, expected.origin.x, "origin.x");
        assert_pixels_close(actual.origin.y, expected.origin.y, "origin.y");
        assert_pixels_close(actual.size.width, expected.size.width, "size.width");
        assert_pixels_close(actual.size.height, expected.size.height, "size.height");
    }

    fn text_width(window: &Window, text: &str, font_size: Pixels) -> Pixels {
        let run = TextStyle::default().to_run(text.len());
        window
            .text_system()
            .shape_line(text.to_string().into(), font_size, &[run], None)
            .layout
            .width
    }

    /// Test 1: Minimal box model - inline with border, inner box 80×40
    /// Expected output:
    /// - content_size: ~183 × 40
    /// - 1 line, height=40
    /// - 3 segments (text + box + text)
    /// - box at x≈41.2, y=0
    #[gpui::test]
    fn full_layout_test1_minimal_box_model(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .border_1()
            .border_color(black())
            .font_size(px(16.0))
            .text("Hello ")
            .child(
                div()
                    .w(px(80.0))
                    .h(px(40.0))
                    .border_1()
                    .border_color(red())
                    .child("box"),
            )
            .text(" tail text.")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(800.0), px(600.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert_eq!(layout.boxes.len(), 1, "should have 1 box");
        let box_bounds = &layout.boxes[0].relative_bounds;
        assert_eq!(
            box_bounds.size.width.0, 80.0,
            "box width={} expected 80",
            box_bounds.size.width.0
        );
        assert_eq!(
            box_bounds.size.height.0, 40.0,
            "box height={} expected 40",
            box_bounds.size.height.0
        );

        assert_eq!(layout.lines.len(), 1, "expected 1 line");
        let line0 = &layout.lines[0];
        assert_eq!(
            line0.height.0, 40.0,
            "line height={} expected 40",
            line0.height.0
        );
        assert_eq!(
            line0.segments.len(),
            3,
            "expected 3 segments (text+box+text)"
        );

        assert!(
            layout.content_size.width.0 > 150.0,
            "content_size.width should be > 150"
        );
        assert_eq!(
            layout.content_size.height.0, 40.0,
            "content_size.height={} expected 40",
            layout.content_size.height.0
        );
    }

    /// Test 2: Inline width + padding, box simple
    /// Expected output:
    /// - inline w=200, p=16 causes wrapping
    /// - Multiple lines due to constrained width
    #[gpui::test]
    fn full_layout_test2_inline_width_padding(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .w(px(200.0))
            .p(px(16.0))
            .border_1()
            .border_color(black())
            .font_size(px(16.0))
            .text("Hello ")
            .child(
                div()
                    .w(px(80.0))
                    .h(px(40.0))
                    .border_1()
                    .border_color(red())
                    .child("inline box"),
            )
            .text(" tail text inside padded inline().")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(800.0), px(600.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert_eq!(layout.boxes.len(), 1, "should have 1 box");
        let box_bounds = &layout.boxes[0].relative_bounds;
        assert_eq!(
            box_bounds.size.width.0, 80.0,
            "box width={} expected 80",
            box_bounds.size.width.0
        );
        assert_eq!(
            box_bounds.size.height.0, 40.0,
            "box height={} expected 40",
            box_bounds.size.height.0
        );

        assert!(
            layout.lines.len() >= 2,
            "expected multiple lines due to narrow width, got {}",
            layout.lines.len()
        );
    }

    /// Test 3: Inline margin, box padding
    /// Expected ATOMS output:
    /// - inline m=12, box 80×40 with p=6
    /// - content_size: ~380 × 40
    /// - 1 line, height=40 (margin is on inline, not affecting line height)
    #[gpui::test]
    fn full_layout_test3_inline_margin_box_padding(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .m(px(12.0))
            .border_1()
            .border_color(black())
            .font_size(px(16.0))
            .text("Hello ")
            .child(
                div()
                    .w(px(80.0))
                    .h(px(40.0))
                    .p(px(6.0))
                    .border_1()
                    .border_color(red())
                    .child("padded box"),
            )
            .text(" tail text with margin around inline().")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(800.0), px(600.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert_eq!(layout.boxes.len(), 1, "should have 1 box");
        let box_bounds = &layout.boxes[0].relative_bounds;
        assert_eq!(
            box_bounds.size.width.0, 80.0,
            "box width={} expected 80",
            box_bounds.size.width.0
        );
        assert_eq!(
            box_bounds.size.height.0, 40.0,
            "box height={} expected 40",
            box_bounds.size.height.0
        );

        assert_eq!(layout.lines.len(), 1, "expected 1 line");
        let line0 = &layout.lines[0];
        assert_eq!(
            line0.height.0, 40.0,
            "line height={} expected 40",
            line0.height.0
        );
        assert_eq!(
            line0.segments.len(),
            3,
            "expected 3 segments (text+box+text)"
        );

        assert_eq!(
            layout.content_size.height.0, 40.0,
            "content_size.height={} expected 40",
            layout.content_size.height.0
        );
    }

    /// Test 4: Narrow width with wrapping
    /// Expected ATOMS output:
    /// - Multiple lines due to 160px width constraint
    /// - Box wraps to line 1 (after "Hello " on line 0)
    /// - ~5 lines total with wrapped text
    #[gpui::test]
    fn full_layout_test4_narrow_width_wrapping(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .w(px(160.0))
            .border_1()
            .border_color(black())
            .font_size(px(16.0))
            .text("Hello ")
            .child(
                div()
                    .w(px(140.0))
                    .h(px(40.0))
                    .border_1()
                    .border_color(red())
                    .child("box"),
            )
            .text(" tail text that should wrap.")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(800.0), px(600.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert!(
            layout.lines.len() >= 3,
            "expected at least 3 lines (wrapping), got {}",
            layout.lines.len()
        );

        assert_eq!(layout.boxes.len(), 1, "should have 1 box");
        let box_bounds = &layout.boxes[0].relative_bounds;
        assert_eq!(
            box_bounds.size.width.0, 140.0,
            "box width={} expected 140",
            box_bounds.size.width.0
        );
        assert_eq!(
            box_bounds.size.height.0, 40.0,
            "box height={} expected 40",
            box_bounds.size.height.0
        );

        let line0 = &layout.lines[0];
        assert!(
            line0.height.0 < 30.0,
            "line 0 height={} should be < 30 (text only)",
            line0.height.0
        );

        if layout.lines.len() > 1 {
            let line1 = &layout.lines[1];
            assert!(
                line1.height.0 >= 40.0,
                "line 1 height={} should be >= 40 (contains box)",
                line1.height.0
            );
        }
    }

    /// Test 5: Explicit inline height vs tall box
    /// Expected output:
    /// - inline h=40, box 80×40
    /// - 1 line, height determined by box
    #[gpui::test]
    fn full_layout_test5_explicit_height(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .h(px(40.0))
            .border_1()
            .border_color(black())
            .font_size(px(16.0))
            .text("Short ")
            .child(
                div()
                    .w(px(80.0))
                    .h(px(40.0))
                    .border_1()
                    .border_color(red())
                    .child("tall box"),
            )
            .text(" text.")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(800.0), px(600.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert_eq!(layout.boxes.len(), 1, "should have 1 box");
        let box_bounds = &layout.boxes[0].relative_bounds;
        assert_eq!(
            box_bounds.size.width.0, 80.0,
            "box width={} expected 80",
            box_bounds.size.width.0
        );
        assert_eq!(
            box_bounds.size.height.0, 40.0,
            "box height={} expected 40",
            box_bounds.size.height.0
        );

        assert_eq!(layout.lines.len(), 1, "expected 1 line");
        let line0 = &layout.lines[0];
        assert_eq!(
            line0.height.0, 40.0,
            "line height={} expected 40",
            line0.height.0
        );
        assert_eq!(
            line0.segments.len(),
            3,
            "expected 3 segments (text+box+text)"
        );
    }

    /// Test 6: Inline padding + box margin
    /// Expected output:
    /// - inline p=12, box m=10
    /// - 1 line, height=60 (box 40 + 10+10 margin)
    #[gpui::test]
    fn full_layout_test6_inline_padding_box_margin(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .p(px(12.0))
            .border_1()
            .border_color(black())
            .font_size(px(16.0))
            .text("Hello ")
            .child(
                div()
                    .w(px(80.0))
                    .h(px(40.0))
                    .m(px(10.0))
                    .border_1()
                    .border_color(red())
                    .child("box margin"),
            )
            .text(" tail text with margins around the inner box.")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(800.0), px(600.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert_eq!(layout.boxes.len(), 1, "should have 1 box");
        let box_bounds = &layout.boxes[0].relative_bounds;
        assert_eq!(
            box_bounds.size.width.0, 80.0,
            "box width={} expected 80",
            box_bounds.size.width.0
        );
        assert_eq!(
            box_bounds.size.height.0, 40.0,
            "box height={} expected 40",
            box_bounds.size.height.0
        );

        assert_eq!(layout.lines.len(), 1, "expected 1 line");
        let line0 = &layout.lines[0];
        assert_eq!(
            line0.height.0, 60.0,
            "line height={} expected 60 (box 40 + margins 10+10)",
            line0.height.0
        );

        assert_eq!(
            layout.content_size.height.0, 60.0,
            "content_size.height={} expected 60",
            layout.content_size.height.0
        );
    }

    /// Test 7: Full stress test with all box model properties
    /// Expected output:
    /// - inline w=360, p=10, border_1
    /// - box 140×40, p=10, m=10
    /// - 2 lines: line 0 has box (height=60), line 1 is wrapped text (~26px)
    /// - content_size: ~306 × 86
    #[gpui::test]
    fn full_layout_test7_stress(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .w(px(360.0))
            .p(px(10.0))
            .border_1()
            .border_color(black())
            .font_size(px(16.0))
            .text("Hello ")
            .child(
                div()
                    .w(px(140.0))
                    .h(px(40.0))
                    .p(px(10.0))
                    .m(px(10.0))
                    .border_1()
                    .border_color(red())
                    .child("heavy box"),
            )
            .text(" tail text.")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(800.0), px(600.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert_eq!(layout.boxes.len(), 1, "should have 1 box");
        let box_bounds = &layout.boxes[0].relative_bounds;
        assert_eq!(
            box_bounds.size.width.0, 140.0,
            "box width={} expected 140",
            box_bounds.size.width.0
        );
        assert_eq!(
            box_bounds.size.height.0, 40.0,
            "box height={} expected 40",
            box_bounds.size.height.0
        );

        assert!(
            !layout.lines.is_empty(),
            "expected at least 1 line, got {}",
            layout.lines.len()
        );

        let line0 = &layout.lines[0];
        assert_eq!(
            line0.height.0, 60.0,
            "line 0 height={} expected 60 (box 40 + margins 10+10)",
            line0.height.0
        );

        assert_eq!(
            line0.segments.len(),
            3,
            "line 0 should have 3 segments, got {}",
            line0.segments.len()
        );
    }

    /// Test 8: Narrow container with nested inline containing truncated link
    /// Layout structure:
    /// - Container: 120px wide
    /// - Content: "Here is " + [truncated link 40px high] + " after."
    /// - Expected: Multiple lines due to narrow width
    /// - Link should be truncated with ellipsis if text is too long
    #[gpui::test]
    fn full_layout_test8_narrow_with_nested_truncating_link(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .w(px(120.0))
            .border_1()
            .border_color(black())
            .font_size(px(14.0))
            .text("Here is some text with a ")
            .child(
                inline()
                    .h(px(40.0))
                    .border_1()
                    .border_color(blue())
                    .text_color(blue())
                    .text("clickable link with very long text that should truncate")
                    .truncate()
                    .into_element(),
            )
            .text(" embedded in the flow. Try hovering!")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(120.0), px(600.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert!(
            layout.lines.len() >= 2,
            "expected at least 2 lines due to narrow width (120px), got {}",
            layout.lines.len()
        );

        assert_eq!(
            layout.boxes.len(),
            1,
            "expected 1 box (the nested inline), got {}",
            layout.boxes.len()
        );

        assert!(
            layout.content_size.width.0 <= 120.0,
            "content_size.width={} should be <= 120 (container constraint)",
            layout.content_size.width.0
        );

        assert!(
            layout.content_size.height.0 >= 40.0,
            "content_size.height={} should be at least 40 (box height)",
            layout.content_size.height.0
        );

        let nested_box = &layout.boxes[0];
        assert_eq!(
            nested_box.relative_bounds.size.height.0, 40.0,
            "nested box height={} expected 40 (explicitly set h(px(40.0)))",
            nested_box.relative_bounds.size.height.0
        );

        assert!(
            nested_box.relative_bounds.size.width.0 <= 120.0,
            "nested box width={} should be <= 120 (container width)",
            nested_box.relative_bounds.size.width.0
        );

        let line_with_box = layout.lines.iter().find(|line| {
            line.segments
                .iter()
                .any(|seg| matches!(seg, InlineSegment::InlineBox { .. }))
        });

        assert!(
            line_with_box.is_some(),
            "expected to find a line containing the inline box segment"
        );

        let line_with_box = line_with_box.unwrap();

        assert!(
            line_with_box.height.0 >= 38.0, // Allow small tolerance
            "line with box height={} should be >= 38 (box is 40px tall)",
            line_with_box.height.0
        );

        let box_segment_count = line_with_box
            .segments
            .iter()
            .filter(|s| matches!(s, InlineSegment::InlineBox { .. }))
            .count();

        assert_eq!(
            box_segment_count, 1,
            "line with box should have exactly 1 inline box segment, got {}",
            box_segment_count
        );

        if let Some(InlineSegment::InlineBox { x, .. }) = line_with_box
            .segments
            .iter()
            .find(|s| matches!(s, InlineSegment::InlineBox { .. }))
        {
            assert!(x.0 >= 0.0, "box x position {} should be >= 0", x.0);
        }
    }

    /// Test 9: Window resize triggers relayout and content doesn't overflow bounds.
    /// Expected behavior:
    /// 1. First layout at wide width (800px) - text fits on fewer lines
    /// 2. Resize to narrow width (300px) - text should re-wrap to multiple lines
    /// 3. content_size.width must be <= container width (no overflow)
    #[gpui::test]
    fn test_resize_triggers_relayout_no_overflow(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(move |captured_store, _, _| {
                    let inner_inline = inline()
                        .h(px(25.0))
                        .border_1()
                        .border_color(blue())
                        .text("nested link");

                    let inline_elem = inline()
                        .border_1()
                        .border_color(black())
                        .font_size(px(16.0))
                        .text("Here is some long text with a ")
                        .child(inner_inline)
                        .text(" that should wrap when the window becomes narrow enough.")
                        .into_element();

                    *captured_store = Some(inline_elem.layout());

                    div()
                        .flex()
                        .flex_col()
                        .size_full()
                        .id("scroll-container")
                        .overflow_y_scroll()
                        .child(div().flex().flex_col().child(inline_elem))
                        .into_any_element()
                }),
            )
        });

        cx.simulate_resize(size(px(800.0), px(600.0)));
        cx.run_until_parked();

        let layout_wide = view.read(&*cx.app.borrow()).layout();

        let lines_at_wide = layout_wide.lines.len();
        let content_width_wide = layout_wide.content_size.width.0;
        cx.simulate_resize(size(px(300.0), px(600.0)));
        cx.run_until_parked();

        let layout_narrow = view.read(&*cx.app.borrow()).layout();

        let lines_at_narrow = layout_narrow.lines.len();
        let content_width_narrow = layout_narrow.content_size.width.0;
        assert!(
            lines_at_narrow > lines_at_wide,
            "After resize from 800px to 300px, line count should increase. \
             Wide: {} lines, Narrow: {} lines",
            lines_at_wide,
            lines_at_narrow
        );

        let max_expected_width = 300.0 - 2.0; // container width minus borders
        assert!(
            content_width_narrow <= max_expected_width,
            "CONTENT OVERFLOW: content_size.width ({}) > container width ({}).",
            content_width_narrow,
            max_expected_width
        );

        assert!(
            content_width_narrow < content_width_wide,
            "Content width should decrease after resize: wide={} narrow={}",
            content_width_wide,
            content_width_narrow
        );
    }

    #[gpui::test]
    fn test_markdown_list_item_wrapping(cx: &mut TestAppContext) {
        let test_size = 14.0;

        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(move |captured_store, _, _| {
                    let inline_elem = inline()
                        .text(" item one")
                        .text_color(black())
                        .font_size(px(test_size))
                        .into_element();

                    *captured_store = Some(inline_elem.layout());

                    div()
                        .flex()
                        .flex_col()
                        .bg(white())
                        .p(px(16.0))
                        .border_1()
                        .border_color(red())
                        .child("Some long text")
                        .child(
                            div().flex().flex_col().child(
                                div().flex_1().flex_basis(px(0.0)).child(
                                    div()
                                        .mb_2()
                                        .line_height(rems(1.3))
                                        .items_center()
                                        .flex()
                                        .flex_row()
                                        .child(div().w(px(36.)).h(px(20.)).bg(green()))
                                        .child(inline_elem),
                                ),
                            ),
                        )
                        .into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();

        assert_eq!(
            layout.lines.len(),
            1,
            "Expected NO premature wrapping - text should stay on 1 line when it fits, got {} lines",
            layout.lines.len()
        );
    }

    /// Test 11: Text truncation across multiple fragments.
    /// This test verifies that truncation doesn't aggressively drop all fragments
    /// when it encounters an overflow in a later fragment.
    #[gpui::test]
    fn test_text_truncation_multi_fragment(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);

        let inline_elem = inline()
            .w(px(100.0)) // Narrow width
            .whitespace_nowrap()
            .text_overflow(TextOverflow::Truncate("...".into()))
            .text("First")
            .text(" part that overflows")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(100.0), px(600.0)), |_, _| {
            inline_elem
        });

        let layout_handle = layout_store.get().expect("layout captured");
        let layout = layout_handle.layout.clone();
        let layout_text = strip_placeholders(layout.logical_text.as_ref());

        assert_eq!(
            layout_text, "First part that overflows",
            "Expected logical text to remain unmodified, got '{}'",
            layout_text
        );

        let Some(truncation) = layout.truncation.as_ref() else {
            panic!("Expected truncation plan to be present");
        };
        assert!(
            truncation.visible_logical_end < layout.logical_len,
            "Expected truncation to shorten visible logical range"
        );
        assert_eq!(
            truncation.ellipsis_style.ellipsis_text.as_ref(),
            "...",
            "Expected ellipsis text to be cached in truncation plan"
        );
    }

    #[gpui::test]
    fn test_truncation_single_line_inline_box_kept(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(|captured_store, window, _cx| {
                    let font_size = px(16.0);
                    let ellipsis_text = "...";
                    let prefix = "Prefix ";
                    let suffix = "tail that should truncate";
                    let box_width = px(30.0);
                    let width = text_width(window, prefix, font_size)
                        + box_width
                        + text_width(window, ellipsis_text, font_size)
                        + px(1.0);

                    let inline_elem = inline()
                        .w(width)
                        .font_size(font_size)
                        .whitespace_nowrap()
                        .text_overflow(TextOverflow::Truncate(ellipsis_text.into()))
                        .text(prefix)
                        .child(div().w(box_width).h(px(10.0)).bg(red()))
                        .text(suffix)
                        .into_element();
                    *captured_store = Some(inline_elem.layout());
                    inline_elem.into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();
        let truncation = layout
            .truncation
            .as_ref()
            .expect("expected truncation plan");
        assert_eq!(layout.lines.len(), 1, "expected single-line layout");

        let line = &layout.lines[0];
        let box_range = line
            .segments
            .iter()
            .find_map(|segment| match segment {
                InlineSegment::InlineBox { logical_range, .. } => Some(logical_range.clone()),
                _ => None,
            })
            .expect("expected inline box segment");

        assert!(
            truncation.visible_logical_end >= box_range.end,
            "expected box to remain visible (visible_end={}, box_end={})",
            truncation.visible_logical_end,
            box_range.end
        );
    }

    #[gpui::test]
    fn test_truncation_inline_box_skipped(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(|captured_store, window, _cx| {
                    let font_size = px(16.0);
                    let ellipsis_text = "...";
                    let prefix = "Prefix ";
                    let suffix = "tail that should truncate";
                    let box_width = px(40.0);
                    let width = text_width(window, prefix, font_size)
                        + text_width(window, ellipsis_text, font_size);

                    let inline_elem = inline()
                        .w(width)
                        .font_size(font_size)
                        .whitespace_nowrap()
                        .text_overflow(TextOverflow::Truncate(ellipsis_text.into()))
                        .text(prefix)
                        .child(div().w(box_width).h(px(10.0)).bg(red()))
                        .text(suffix)
                        .into_element();
                    *captured_store = Some(inline_elem.layout());
                    inline_elem.into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();
        let truncation = layout
            .truncation
            .as_ref()
            .expect("expected truncation plan");
        let line = &layout.lines[0];
        let box_range = line
            .segments
            .iter()
            .find_map(|segment| match segment {
                InlineSegment::InlineBox { logical_range, .. } => Some(logical_range.clone()),
                _ => None,
            })
            .expect("expected inline box segment");

        assert!(
            truncation.visible_logical_end <= box_range.start,
            "expected box to be skipped (visible_end={}, box_start={})",
            truncation.visible_logical_end,
            box_range.start
        );
    }

    #[gpui::test]
    fn test_line_clamp_ellipsis_last_visible_line(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(|captured_store, _window, _cx| {
                    let inline_elem = inline()
                        .w(px(200.0))
                        .line_clamp(2)
                        .text_overflow(TextOverflow::Truncate("...".into()))
                        .text("Short")
                        .break_line()
                        .text("Tiny")
                        .break_line()
                        .text("This line should be clamped")
                        .into_element();
                    *captured_store = Some(inline_elem.layout());
                    inline_elem.into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();
        let truncation = layout
            .truncation
            .as_ref()
            .expect("expected truncation plan");

        assert_eq!(
            layout.lines.len(),
            2,
            "expected line clamp to keep two lines"
        );
        assert_eq!(
            truncation.line_ix, 1,
            "ellipsis should be on the last visible line"
        );

        let last_line = &layout.lines[1];
        assert_pixels_close(truncation.clip_x, last_line.width, "line-clamp clip_x");
    }

    #[gpui::test]
    fn test_ellipsis_wider_than_line_clipped(cx: &mut TestAppContext) {
        let ellipsis_width = Rc::new(RefCell::new(None));
        let inline_width = Rc::new(RefCell::new(None));
        let ellipsis_width_handle = ellipsis_width.clone();
        let inline_width_handle = inline_width.clone();

        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(move |captured_store, window, _cx| {
                    let font_size = px(16.0);
                    let ellipsis_text = "WIDE-ELLIPSIS";
                    let ellipsis_px = text_width(window, ellipsis_text, font_size);
                    let width = ellipsis_px * 0.5;
                    *ellipsis_width_handle.borrow_mut() = Some(ellipsis_px);
                    *inline_width_handle.borrow_mut() = Some(width);

                    let inline_elem = inline()
                        .w(width)
                        .font_size(font_size)
                        .whitespace_nowrap()
                        .text_overflow(TextOverflow::Truncate(ellipsis_text.into()))
                        .text("This text will truncate")
                        .into_element();
                    *captured_store = Some(inline_elem.layout());
                    inline_elem.into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();
        let truncation = layout
            .truncation
            .as_ref()
            .expect("expected truncation plan");
        let ellipsis_width = ellipsis_width
            .borrow()
            .expect("ellipsis width should be captured");
        let inline_width = inline_width
            .borrow()
            .expect("inline width should be captured");

        assert!(
            ellipsis_width > inline_width,
            "expected ellipsis width to exceed line width"
        );
        assert!(
            truncation.clip_first_item,
            "expected clip-first-item rule when ellipsis is wider than line"
        );
        assert!(
            truncation.truncate_text_end.is_some(),
            "expected truncation to keep at least one glyph"
        );
        assert!(
            truncation.visible_width.0 <= inline_width.0 + 0.5,
            "expected visible width to be clipped to line width"
        );
    }

    #[gpui::test]
    fn test_truncation_long_ellipsis_overflow_hidden(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(|captured_store, _window, _cx| {
                    let ellipsis_text = "................................................";
                    let inline_elem = inline()
                        .border_1()
                        .border_color(blue())
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_overflow(TextOverflow::Truncate(ellipsis_text.into()))
                        .text("Very long text that can be tested for elipis")
                        .into_element();

                    *captured_store = Some(inline_elem.layout());

                    div()
                        .mt(px(20.0))
                        .w(px(240.0))
                        .p(px(8.0))
                        .border_1()
                        .border_color(black())
                        .child("Test 15: ellipsis wider than line (clipped)")
                        .child(inline_elem)
                        .into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();
        let truncation = layout
            .truncation
            .as_ref()
            .expect("expected truncation plan");

        assert_eq!(
            layout.lines.len(),
            1,
            "expected nowrap truncation to stay on one line"
        );
        assert!(
            truncation.clip_first_item,
            "expected clip-first-item rule when ellipsis is wider than line"
        );
        assert!(
            truncation.truncate_text_end.is_some(),
            "expected truncation to keep at least one glyph"
        );
        assert!(
            truncation.visible_logical_end < layout.logical_len,
            "expected truncation to reduce visible logical range"
        );
    }

    #[gpui::test]
    fn test_truncation_clip_first_item_keeps_glyph(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(|captured_store, window, _cx| {
                    let font_size = px(16.0);
                    let ellipsis_text = "................................................";
                    let ellipsis_width = text_width(window, ellipsis_text, font_size);
                    let first_glyph_width = text_width(window, "W", font_size);
                    let width = ellipsis_width + (first_glyph_width / 2.0);
                    let glyph_count = ((width / first_glyph_width).ceil() as usize) + 2;
                    let text = "W".repeat(glyph_count);

                    let inline_elem = inline()
                        .w(width)
                        .font_size(font_size)
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_overflow(TextOverflow::Truncate(ellipsis_text.into()))
                        .text(text)
                        .into_element();

                    *captured_store = Some(inline_elem.layout());
                    inline_elem.into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();
        let truncation = layout
            .truncation
            .as_ref()
            .expect("expected truncation plan");

        assert!(
            truncation.clip_first_item,
            "expected clip-first-item rule to trigger"
        );
        assert!(
            truncation.visible_logical_end > 0,
            "expected at least one glyph to remain visible"
        );
        assert_eq!(
            truncation.truncate_segment_ix,
            Some(0),
            "expected truncation to target the first segment"
        );
        assert!(
            truncation.truncate_text_end.is_some_and(|end| end > 0),
            "expected truncate_text_end to include the first glyph"
        );
    }

    #[gpui::test]
    fn test_ellipsis_style_run_boundary_bias(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(|captured_store, window, _cx| {
                    let font_size = px(16.0);
                    let ellipsis_text = "...";
                    let first = "Red";
                    let second = "Blue";
                    let boundary_x = text_width(window, first, font_size);
                    let ellipsis_width = text_width(window, ellipsis_text, font_size);
                    let width = boundary_x + ellipsis_width;

                    let inline_elem = inline()
                        .w(width)
                        .font_size(font_size)
                        .whitespace_nowrap()
                        .text_overflow(TextOverflow::Truncate(ellipsis_text.into()))
                        .text_color(red())
                        .text(first)
                        .text_color(blue())
                        .text(second)
                        .into_element();
                    *captured_store = Some(inline_elem.layout());
                    inline_elem.into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();
        let truncation = layout
            .truncation
            .as_ref()
            .expect("expected truncation plan");

        assert_eq!(
            truncation.ellipsis_style.decoration.color,
            red(),
            "ellipsis should inherit the previous run's style"
        );
    }

    #[gpui::test]
    fn test_flex_basis_zero_min_content_correctness(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| {
            StandardTestView::new(
                None,
                Box::new(move |captured_store, _, _| {
                    let inline_elem = inline()
                        .text(" item one") // Short text should wrap if bug is present
                        .text_color(gpui::black())
                        .into_element();

                    *captured_store = Some(inline_elem.layout());

                    div()
                        .flex()
                        .flex_col()
                        .size_full()
                        .bg(gpui::white())
                        .p(px(16.0))
                        .border_1()
                        .border_color(gpui::red())
                        .child("Reproduction: Markdown List Item (flex_1 + w_0)")
                        .child(
                            // List Item container
                            div().border_1().border_color(gpui::blue()).child(
                                // Content container (The Culprit!)
                                div().child(
                                    // Node 2 (Flex Row) - The Paragraph modified by Image
                                    div()
                                        .mb_2()
                                        .line_height(rems(1.3))
                                        .items_center()
                                        .flex()
                                        .flex_row()
                                        .child(
                                            // Node 0 (Image)
                                            div().w(px(36.)).h(px(20.)).bg(gpui::green()),
                                        )
                                        .child(inline_elem),
                                ),
                            ),
                        )
                        .into_any_element()
                }),
            )
        });

        cx.run_until_parked();

        let layout = view.read(&*cx.app.borrow()).layout();

        // 1. Check Content Width
        // If the bug exists, the parent w_min() collapsed to 0.0 because it thought
        // the child could shrink to 0. The layout engine then forced the text into 0px.
        assert!(
            layout.content_size.width.0 > 10.0,
            "Layout collapsed to near-zero width ({:?})! \
             The engine incorrectly reported 0.0 as MinContent size for a w(0) element.",
            layout.content_size.width
        );

        // 2. Check Line Wrapping
        // "ItemOne" is a single word. It should never wrap unless forced into a tiny width.
        assert_eq!(
            layout.lines.len(),
            1,
            "Text wrapped unexpectedly! This indicates the container was crushed to 0 width."
        );
    }

    #[gpui::test]
    fn test_inline_min_content_respects_parent_max_w(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let captured_size = Rc::new(RefCell::new(None));
        let captured_size_for_draw = captured_size.clone();

        cx.draw(
            Point::default(),
            size(px(800.0), px(600.0)),
            move |window, cx| {
                let mut element = div()
                    .max_w(px(400.0))
                    .child(inline().text("word ".repeat(120)))
                    .into_any_element();
                let size = element.layout_as_root(AvailableSpace::min_size(), window, cx);
                *captured_size_for_draw.borrow_mut() = Some(size);
                div()
            },
        );

        let size = captured_size
            .borrow()
            .clone()
            .expect("layout size should be captured");
        assert!(
            size.width.0 >= 380.0 && size.width.0 <= 401.0,
            "Expected width near max_w (400), got {:?}",
            size.width
        );
    }

    #[gpui::test]
    fn test_inline_min_content_auto_width_matches_child(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline().text("Short line of text.").into_element();
        let layout_store = inline_elem.layout();
        let captured_size = Rc::new(RefCell::new(None));
        let captured_size_for_draw = captured_size.clone();

        cx.draw(
            Point::default(),
            size(px(800.0), px(600.0)),
            move |window, cx| {
                let mut measure_element = div()
                    .child(inline().text("Short line of text."))
                    .into_any_element();
                let size = measure_element.layout_as_root(AvailableSpace::min_size(), window, cx);
                *captured_size_for_draw.borrow_mut() = Some(size);

                div().child(inline_elem)
            },
        );

        let size = captured_size
            .borrow()
            .clone()
            .expect("layout size should be captured");
        let handle = layout_store.get().expect("layout should be captured");
        let inline_width = handle.layout.content_size.width.0;
        let delta = (size.width.0 - inline_width).abs();
        assert!(
            delta <= 1.0,
            "Expected parent width ~= child width (delta={}), parent={:?}, child={}",
            delta,
            size.width,
            inline_width
        );
    }

    #[gpui::test]
    fn test_inline_box_bounds_match_child_layout_hidpi(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let captured_bounds = Rc::new(RefCell::new(None));

        let inline_elem = inline()
            .p(px(12.0))
            .border_1()
            .border_color(black())
            .font_size(px(16.0))
            .text("Hi ")
            .child(BoundsCapture::new(
                div().w(px(60.0)).h(px(24.0)).bg(green()),
                captured_bounds.clone(),
            ))
            .text(" there")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(400.0), px(200.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let child_bounds = captured_bounds
            .borrow()
            .clone()
            .expect("child bounds should be captured");

        assert_eq!(handle.layout.boxes.len(), 1, "expected 1 inline box");
        let box_bounds = handle.layout.boxes[0].relative_bounds;
        let expected = Bounds {
            origin: handle.origin + box_bounds.origin,
            size: box_bounds.size,
        };

        assert_bounds_close(child_bounds, expected);
    }

    #[gpui::test]
    fn test_hard_break_index_mapping(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .font_size(px(16.0))
            .text("Hello")
            .break_line()
            .text("World")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(400.0), px(200.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert_eq!(layout.logical_text.as_ref(), "Hello\nWorld");

        let line0 = &layout.lines[0];
        let (logical_range, break_x) = line0
            .segments
            .iter()
            .find_map(|segment| match segment {
                InlineSegment::HardBreak { logical_range, x } => Some((logical_range.clone(), *x)),
                _ => None,
            })
            .expect("hard break segment");

        assert_eq!(logical_range.start, "Hello".len());
        let expected_origin = handle.origin + point(Pixels::ZERO, line0.y);
        let position = layout_store
            .position_for_index(logical_range.start)
            .expect("position for hard break");

        assert_pixels_close(position.x, expected_origin.x + break_x, "hard break x");
        assert_pixels_close(position.y, expected_origin.y, "hard break y");
        assert_eq!(
            layout_store.line_height_for_index(logical_range.start),
            line0.height
        );
    }

    #[gpui::test]
    fn test_text_runs_newline_inserts_hard_break(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let text = "Hello\nWorld";
        let runs = vec![
            TextStyle::default().to_run("Hello\n".len()),
            TextStyle::default().to_run("World".len()),
        ];
        let inline_elem = inline()
            .font_size(px(16.0))
            .text_runs(text.into(), runs)
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(400.0), px(200.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let layout = &handle.layout;

        assert_eq!(layout.logical_text.as_ref(), text);
        assert_eq!(layout.lines.len(), 2, "expected two lines from newline");

        let line0 = &layout.lines[0];
        let logical_range = line0
            .segments
            .iter()
            .find_map(|segment| match segment {
                InlineSegment::HardBreak { logical_range, .. } => Some(logical_range.clone()),
                _ => None,
            })
            .expect("hard break segment");
        assert_eq!(logical_range.start, "Hello".len());
    }

    #[gpui::test]
    fn test_plain_text_strips_inline_box_placeholders(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .text("Hi ")
            .child(div().w(px(10.0)).h(px(10.0)))
            .text("there")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(400.0), px(200.0)), |_, _| {
            inline_elem
        });

        let handle = layout_store.get().expect("layout should be captured");
        let logical = handle.text();
        assert!(
            logical.as_ref().contains(INLINE_BOX_PLACEHOLDER),
            "expected inline box placeholder in logical text"
        );

        assert_eq!(layout_store.plain_text().as_ref(), "Hi there");

        let placeholder_index = "Hi ".len();
        let range_plain = layout_store.plain_text_range(placeholder_index..logical.len());
        assert_eq!(range_plain.as_ref(), "there");
    }

    #[gpui::test]
    fn test_surrounding_word_range_boundaries(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| EmptyView);
        let inline_elem = inline()
            .text("Hi")
            .child(div().w(px(10.0)).h(px(10.0)))
            .text("there")
            .into_element();
        let layout_store = inline_elem.layout();

        cx.draw(Point::default(), size(px(400.0), px(200.0)), |_, _| {
            inline_elem
        });

        let logical = layout_store.text();
        let placeholder_index = logical
            .as_ref()
            .find(INLINE_BOX_PLACEHOLDER)
            .expect("placeholder should exist");

        let range_on_box = layout_store.surrounding_word_range(placeholder_index);
        assert_eq!(layout_store.plain_text_range(range_on_box).as_ref(), "Hi");

        let inline_elem = inline().text("Hi there").into_element();
        let layout_store = inline_elem.layout();
        cx.draw(Point::default(), size(px(400.0), px(200.0)), |_, _| {
            inline_elem
        });

        let space_index = "Hi".len();
        let range_on_space = layout_store.surrounding_word_range(space_index);
        assert_eq!(layout_store.plain_text_range(range_on_space).as_ref(), "Hi");

        let end_index = layout_store.len();
        let range_at_end = layout_store.surrounding_word_range(end_index);
        assert_eq!(
            layout_store.plain_text_range(range_at_end).as_ref(),
            "there"
        );
    }
}
