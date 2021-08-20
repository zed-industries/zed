use crate::{
    color::Color,
    font_cache::FamilyId,
    fonts::{FontId, TextStyle},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::Line,
    DebugContext, Element, Event, EventContext, FontCache, LayoutContext, PaintContext,
    SizeConstraint,
};
use serde::Deserialize;
use serde_json::json;
use smallvec::{smallvec, SmallVec};

pub struct Label {
    text: String,
    family_id: FamilyId,
    font_size: f32,
    style: LabelStyle,
    highlight_indices: Vec<usize>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct LabelStyle {
    pub text: TextStyle,
    pub highlight_text: Option<TextStyle>,
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

    pub fn with_default_color(mut self, color: Color) -> Self {
        self.style.text.color = color;
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
    ) -> SmallVec<[(usize, FontId, Color); 8]> {
        if self.highlight_indices.is_empty() {
            return smallvec![(self.text.len(), font_id, self.style.text.color)];
        }

        let highlight_font_id = self
            .style
            .highlight_text
            .as_ref()
            .and_then(|style| {
                font_cache
                    .select_font(self.family_id, &style.font_properties)
                    .ok()
            })
            .unwrap_or(font_id);

        let mut highlight_indices = self.highlight_indices.iter().copied().peekable();
        let mut runs = SmallVec::new();

        for (char_ix, c) in self.text.char_indices() {
            let mut font_id = font_id;
            let mut color = self.style.text.color;
            if let Some(highlight_ix) = highlight_indices.peek() {
                if char_ix == *highlight_ix {
                    font_id = highlight_font_id;
                    color = self
                        .style
                        .highlight_text
                        .as_ref()
                        .unwrap_or(&self.style.text)
                        .color;
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
            .select_font(self.family_id, &self.style.text.font_properties)
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
            "text": self.text.to_json(),
            "highlight_text": self.highlight_text
                .as_ref()
                .map_or(serde_json::Value::Null, |style| style.to_json())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fonts::{Properties as FontProperties, Weight};

    #[crate::test(self)]
    fn test_layout_label_with_highlights(cx: &mut crate::MutableAppContext) {
        let menlo = cx.font_cache().load_family(&["Menlo"]).unwrap();
        let menlo_regular = cx
            .font_cache()
            .select_font(menlo, &FontProperties::new())
            .unwrap();
        let menlo_bold = cx
            .font_cache()
            .select_font(menlo, FontProperties::new().weight(Weight::BOLD))
            .unwrap();
        let black = Color::black();
        let red = Color::new(255, 0, 0, 255);

        let label = Label::new(".αβγδε.ⓐⓑⓒⓓⓔ.abcde.".to_string(), menlo, 12.0)
            .with_style(&LabelStyle {
                text: TextStyle {
                    color: black,
                    font_properties: Default::default(),
                },
                highlight_text: Some(TextStyle {
                    color: red,
                    font_properties: *FontProperties::new().weight(Weight::BOLD),
                }),
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
