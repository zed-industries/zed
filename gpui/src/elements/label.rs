use serde_json::json;
use smallvec::{smallvec, SmallVec};

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

pub struct Label {
    text: String,
    family_id: FamilyId,
    font_properties: Properties,
    font_size: f32,
    highlights: Option<Highlights>,
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

    fn compute_runs(
        &self,
        font_cache: &FontCache,
        font_id: FontId,
    ) -> SmallVec<[(usize, FontId, ColorU); 8]> {
        if let Some(highlights) = self.highlights.as_ref() {
            let highlight_font_id = font_cache
                .select_font(self.family_id, &highlights.font_properties)
                .unwrap_or(font_id);

            let mut highlight_indices = highlights.indices.iter().copied().peekable();
            let mut runs = SmallVec::new();

            for (char_ix, c) in self.text.char_indices() {
                let mut font_id = font_id;
                let mut color = ColorU::black();
                if let Some(highlight_ix) = highlight_indices.peek() {
                    if char_ix == *highlight_ix {
                        font_id = highlight_font_id;
                        color = highlights.color;
                        highlight_indices.next();
                    }
                }

                let push_new_run =
                    if let Some((last_len, last_font_id, last_color)) = runs.last_mut() {
                        if font_id == *last_font_id && color == *last_color {
                            *last_len += c.len_utf8();
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    };

                if push_new_run {
                    runs.push((c.len_utf8(), font_id, color));
                }
            }

            runs
        } else {
            smallvec![(self.text.len(), font_id, ColorU::black())]
        }
    }
}

impl Element for Label {
    type LayoutState = Line;
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
        let runs = self.compute_runs(&ctx.font_cache, font_id);
        let line =
            ctx.text_layout_cache
                .layout_str(self.text.as_str(), self.font_size, runs.as_slice());

        let size = vec2f(
            line.width().max(constraint.min.x()).min(constraint.max.x()),
            ctx.font_cache.line_height(font_id, self.font_size).ceil(),
        );

        (size, line)
    }

    fn after_layout(&mut self, _: Vector2F, _: &mut Self::LayoutState, _: &mut AfterLayoutContext) {
    }

    fn paint(
        &mut self,
        bounds: RectF,
        line: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        line.paint(
            bounds.origin(),
            RectF::new(vec2f(0., 0.), bounds.size()),
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

        let runs = label.compute_runs(app.font_cache().as_ref(), menlo_regular);
        assert_eq!(
            runs.as_slice(),
            &[
                (3, menlo_regular, black),
                (1, menlo_bold, red),
                (1, menlo_regular, black),
                (1, menlo_bold, red),
                (3, menlo_regular, black),
                (1, menlo_bold, red),
                (5, menlo_regular, black),
                (1, menlo_bold, red),
                (2, menlo_regular, black),
                (1, menlo_bold, red),
                (15, menlo_regular, black),
            ]
        );
    }
}
