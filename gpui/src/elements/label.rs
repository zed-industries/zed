use serde_json::json;

use crate::{
    color::ColorU,
    font_cache::FamilyId,
    fonts::{FontId, Properties},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::Line,
    AfterLayoutContext, DebugContext, Element, Event, EventContext, FontCache, LayoutContext,
    PaintContext, SizeConstraint,
};
use std::{ops::Range, sync::Arc};

pub struct Label {
    text: String,
    family_id: FamilyId,
    font_properties: Properties,
    font_size: f32,
    highlights: Option<Highlights>,
}

pub struct LayoutState {
    line: Arc<Line>,
    colors: Vec<(Range<usize>, ColorU)>,
}

pub struct Highlights {
    color: ColorU,
    indices: Vec<usize>,
    font_properties: Properties,
}

impl Label {
    pub fn new(text: String, family_id: FamilyId, font_size: f32) -> Self {
        Self {
            text,
            family_id,
            font_properties: Properties::new(),
            font_size,
            highlights: None,
        }
    }

    pub fn with_highlights(
        mut self,
        color: ColorU,
        font_properties: Properties,
        indices: Vec<usize>,
    ) -> Self {
        self.highlights = Some(Highlights {
            color,
            font_properties,
            indices,
        });
        self
    }

    fn layout_text(
        &self,
        font_cache: &FontCache,
        font_id: FontId,
    ) -> (Vec<(Range<usize>, FontId)>, Vec<(Range<usize>, ColorU)>) {
        let text_len = self.text.len();
        let mut styles;
        let mut colors;
        if let Some(highlights) = self.highlights.as_ref() {
            styles = Vec::new();
            colors = Vec::new();
            let highlight_font_id = font_cache
                .select_font(self.family_id, &highlights.font_properties)
                .unwrap_or(font_id);
            let mut pending_highlight: Option<Range<usize>> = None;
            for ix in &highlights.indices {
                if let Some(pending_highlight) = pending_highlight.as_mut() {
                    if *ix == pending_highlight.end {
                        pending_highlight.end += 1;
                    } else {
                        styles.push((pending_highlight.clone(), highlight_font_id));
                        colors.push((pending_highlight.clone(), highlights.color));
                        styles.push((pending_highlight.end..*ix, font_id));
                        colors.push((pending_highlight.end..*ix, ColorU::black()));
                        *pending_highlight = *ix..*ix + 1;
                    }
                } else {
                    styles.push((0..*ix, font_id));
                    colors.push((0..*ix, ColorU::black()));
                    pending_highlight = Some(*ix..*ix + 1);
                }
            }
            if let Some(pending_highlight) = pending_highlight.as_mut() {
                styles.push((pending_highlight.clone(), highlight_font_id));
                colors.push((pending_highlight.clone(), highlights.color));
                if text_len > pending_highlight.end {
                    styles.push((pending_highlight.end..text_len, font_id));
                    colors.push((pending_highlight.end..text_len, ColorU::black()));
                }
            } else {
                styles.push((0..text_len, font_id));
                colors.push((0..text_len, ColorU::black()));
            }
        } else {
            styles = vec![(0..text_len, font_id)];
            colors = vec![(0..text_len, ColorU::black())];
        }

        (styles, colors)
    }
}

impl Element for Label {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let font_id = ctx
            .font_cache
            .select_font(self.family_id, &self.font_properties)
            .unwrap();
        let (styles, colors) = self.layout_text(&ctx.font_cache, font_id);
        let line =
            ctx.text_layout_cache
                .layout_str(self.text.as_str(), self.font_size, styles.as_slice());

        let size = vec2f(
            line.width.max(constraint.min.x()).min(constraint.max.x()),
            ctx.font_cache.line_height(font_id, self.font_size).ceil(),
        );

        (size, LayoutState { line, colors })
    }

    fn after_layout(&mut self, _: Vector2F, _: &mut Self::LayoutState, _: &mut AfterLayoutContext) {
    }

    fn paint(
        &mut self,
        bounds: RectF,
        layout: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        layout.line.paint(
            bounds.origin(),
            RectF::new(vec2f(0., 0.), bounds.size()),
            layout.colors.as_slice(),
            ctx,
        )
    }

    fn dispatch_event(
        &mut self,
        _: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut EventContext,
    ) -> bool {
        false
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        ctx: &DebugContext,
    ) -> Value {
        json!({
            "type": "Label",
            "bounds": bounds.to_json(),
            "font_family": ctx.font_cache.family_name(self.family_id).unwrap(),
            "font_size": self.font_size,
            "font_properties": self.font_properties.to_json(),
            "text": &self.text,
            "highlights": self.highlights.to_json(),
        })
    }
}

impl ToJson for Highlights {
    fn to_json(&self) -> Value {
        json!({
            "color": self.color.to_json(),
            "indices": self.indices,
            "font_properties": self.font_properties.to_json(),
        })
    }
}

#[cfg(test)]
mod tests {
    use font_kit::properties::Weight;

    use super::*;

    #[crate::test(self)]
    fn test_layout_label_with_highlights(app: &mut crate::MutableAppContext) {
        let menlo = app.font_cache().load_family(&["Menlo"]).unwrap();
        let menlo_regular = app
            .font_cache()
            .select_font(menlo, &Properties::new())
            .unwrap();
        let menlo_bold = app
            .font_cache()
            .select_font(menlo, Properties::new().weight(Weight::BOLD))
            .unwrap();
        let black = ColorU::black();
        let red = ColorU::new(255, 0, 0, 255);

        let label = Label::new(".αβγδε.ⓐⓑⓒⓓⓔ.abcde.".to_string(), menlo, 12.0).with_highlights(
            red,
            *Properties::new().weight(Weight::BOLD),
            vec![
                ".α".len(),
                ".αβ".len(),
                ".αβγδ".len(),
                ".αβγδε.ⓐ".len(),
                ".αβγδε.ⓐⓑ".len(),
            ],
        );

        let (styles, colors) = label.layout_text(app.font_cache().as_ref(), menlo_regular);
        assert_eq!(styles.len(), colors.len());

        let mut spans = Vec::new();
        for ((style_range, font_id), (color_range, color)) in styles.into_iter().zip(colors) {
            assert_eq!(style_range, color_range);
            spans.push((style_range, font_id, color));
        }
        assert_eq!(
            spans,
            &[
                (0..3, menlo_regular, black),
                (3..4, menlo_bold, red),
                (4..5, menlo_regular, black),
                (5..6, menlo_bold, red),
                (6..9, menlo_regular, black),
                (9..10, menlo_bold, red),
                (10..15, menlo_regular, black),
                (15..16, menlo_bold, red),
                (16..18, menlo_regular, black),
                (18..19, menlo_bold, red),
                (19..34, menlo_regular, black)
            ]
        );
    }
}
