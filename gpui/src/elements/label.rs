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
    font_size: f32,
    style: LabelStyle,
    highlight_indices: Vec<usize>,
}

#[derive(Clone, Debug, Default)]
pub struct LabelStyle {
    pub default_color: ColorU,
    pub highlight_color: ColorU,
    pub font_properties: Properties,
    pub highlight_font_properties: Properties,
}

impl Label {
    pub fn new(text: String, family_id: FamilyId, font_size: f32) -> Self {
        Self {
            text,
            family_id,
            font_size,
            highlight_indices: Default::default(),
            style: Default::default(),
        }
    }

    pub fn with_style(mut self, style: &LabelStyle) -> Self {
        self.style = style.clone();
        self
    }

    pub fn with_default_color(mut self, color: ColorU) -> Self {
        self.style.default_color = color;
        self
    }

    pub fn with_highlights(mut self, indices: Vec<usize>) -> Self {
        self.highlight_indices = indices;
        self
    }

    fn compute_runs(
        &self,
        font_cache: &FontCache,
        font_id: FontId,
    ) -> SmallVec<[(usize, FontId, ColorU); 8]> {
        if self.highlight_indices.is_empty() {
            return smallvec![(self.text.len(), font_id, self.style.default_color)];
        }

        let highlight_font_id = font_cache
            .select_font(self.family_id, &self.style.highlight_font_properties)
            .unwrap_or(font_id);

        let mut highlight_indices = self.highlight_indices.iter().copied().peekable();
        let mut runs = SmallVec::new();

        for (char_ix, c) in self.text.char_indices() {
            let mut font_id = font_id;
            let mut color = self.style.default_color;
            if let Some(highlight_ix) = highlight_indices.peek() {
                if char_ix == *highlight_ix {
                    font_id = highlight_font_id;
                    color = self.style.highlight_color;
                    highlight_indices.next();
                }
            }

            let push_new_run = if let Some((last_len, last_font_id, last_color)) = runs.last_mut() {
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
    }
}

impl Element for Label {
    type LayoutState = Line;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let font_id = cx
            .font_cache
            .select_font(self.family_id, &self.style.font_properties)
            .unwrap();
        let runs = self.compute_runs(&cx.font_cache, font_id);
        let line =
            cx.text_layout_cache
                .layout_str(self.text.as_str(), self.font_size, runs.as_slice());

        let size = vec2f(
            line.width().max(constraint.min.x()).min(constraint.max.x()),
            cx.font_cache.line_height(font_id, self.font_size).ceil(),
        );

        (size, line)
    }

    fn after_layout(&mut self, _: Vector2F, _: &mut Self::LayoutState, _: &mut AfterLayoutContext) {
    }

    fn paint(
        &mut self,
        bounds: RectF,
        line: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        line.paint(
            bounds.origin(),
            RectF::new(vec2f(0., 0.), bounds.size()),
            cx,
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
        cx: &DebugContext,
    ) -> Value {
        json!({
            "type": "Label",
            "bounds": bounds.to_json(),
            "text": &self.text,
            "highlight_indices": self.highlight_indices,
            "font_family": cx.font_cache.family_name(self.family_id).unwrap(),
            "font_size": self.font_size,
            "style": self.style.to_json(),
        })
    }
}

impl ToJson for LabelStyle {
    fn to_json(&self) -> Value {
        json!({
            "default_color": self.default_color.to_json(),
            "default_font_properties": self.font_properties.to_json(),
            "highlight_color": self.highlight_color.to_json(),
            "highlight_font_properties": self.highlight_font_properties.to_json(),
        })
    }
}

#[cfg(test)]
mod tests {
    use font_kit::properties::Weight;

    use super::*;

    #[crate::test(self)]
    fn test_layout_label_with_highlights(cx: &mut crate::MutableAppContext) {
        let menlo = cx.font_cache().load_family(&["Menlo"]).unwrap();
        let menlo_regular = cx
            .font_cache()
            .select_font(menlo, &Properties::new())
            .unwrap();
        let menlo_bold = cx
            .font_cache()
            .select_font(menlo, Properties::new().weight(Weight::BOLD))
            .unwrap();
        let black = ColorU::black();
        let red = ColorU::new(255, 0, 0, 255);

        let label = Label::new(".αβγδε.ⓐⓑⓒⓓⓔ.abcde.".to_string(), menlo, 12.0)
            .with_style(&LabelStyle {
                default_color: black,
                highlight_color: red,
                highlight_font_properties: *Properties::new().weight(Weight::BOLD),
                ..Default::default()
            })
            .with_highlights(vec![
                ".α".len(),
                ".αβ".len(),
                ".αβγδ".len(),
                ".αβγδε.ⓐ".len(),
                ".αβγδε.ⓐⓑ".len(),
            ]);

        let runs = label.compute_runs(cx.font_cache().as_ref(), menlo_regular);
        assert_eq!(
            runs.as_slice(),
            &[
                (".α".len(), menlo_regular, black),
                ("βγ".len(), menlo_bold, red),
                ("δ".len(), menlo_regular, black),
                ("ε".len(), menlo_bold, red),
                (".ⓐ".len(), menlo_regular, black),
                ("ⓑⓒ".len(), menlo_bold, red),
                ("ⓓⓔ.abcde.".len(), menlo_regular, black),
            ]
        );
    }
}
