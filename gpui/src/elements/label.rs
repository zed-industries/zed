use crate::{
    color::Color,
    fonts::{FontId, TextStyle},
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::Line,
    DebugContext, Element, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};
use serde::Deserialize;
use serde_json::json;
use smallvec::{smallvec, SmallVec};

pub struct Label {
    text: String,
    style: LabelStyle,
    highlight_indices: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LabelStyle {
    pub text: TextStyle,
    pub highlight_text: Option<TextStyle>,
}

impl From<TextStyle> for LabelStyle {
    fn from(text: TextStyle) -> Self {
        LabelStyle {
            text,
            highlight_text: None,
        }
    }
}

impl Label {
    pub fn new(text: String, style: impl Into<LabelStyle>) -> Self {
        Self {
            text,
            highlight_indices: Default::default(),
            style: style.into(),
        }
    }

    pub fn with_highlights(mut self, indices: Vec<usize>) -> Self {
        self.highlight_indices = indices;
        self
    }

    fn compute_runs(&self) -> SmallVec<[(usize, FontId, Color); 8]> {
        let font_id = self.style.text.font_id;
        if self.highlight_indices.is_empty() {
            return smallvec![(self.text.len(), font_id, self.style.text.color)];
        }

        let highlight_font_id = self
            .style
            .highlight_text
            .as_ref()
            .map_or(font_id, |style| style.font_id);

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
        let runs = self.compute_runs();
        let line = cx.text_layout_cache.layout_str(
            self.text.as_str(),
            self.style.text.font_size,
            runs.as_slice(),
        );

        let size = vec2f(
            line.width().max(constraint.min.x()).min(constraint.max.x()),
            cx.font_cache
                .line_height(self.style.text.font_id, self.style.text.font_size)
                .ceil(),
        );

        (size, line)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
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
        _: &DebugContext,
    ) -> Value {
        json!({
            "type": "Label",
            "bounds": bounds.to_json(),
            "text": &self.text,
            "highlight_indices": self.highlight_indices,
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
        let default_style = TextStyle::new(
            "Menlo",
            12.,
            Default::default(),
            Color::black(),
            cx.font_cache(),
        )
        .unwrap();
        let highlight_style = TextStyle::new(
            "Menlo",
            12.,
            *FontProperties::new().weight(Weight::BOLD),
            Color::new(255, 0, 0, 255),
            cx.font_cache(),
        )
        .unwrap();
        let label = Label::new(
            ".αβγδε.ⓐⓑⓒⓓⓔ.abcde.".to_string(),
            LabelStyle {
                text: default_style.clone(),
                highlight_text: Some(highlight_style.clone()),
            },
        )
        .with_highlights(vec![
            ".α".len(),
            ".αβ".len(),
            ".αβγδ".len(),
            ".αβγδε.ⓐ".len(),
            ".αβγδε.ⓐⓑ".len(),
        ]);

        let runs = label.compute_runs();
        assert_eq!(
            runs.as_slice(),
            &[
                (".α".len(), default_style.font_id, default_style.color),
                ("βγ".len(), highlight_style.font_id, highlight_style.color),
                ("δ".len(), default_style.font_id, default_style.color),
                ("ε".len(), highlight_style.font_id, highlight_style.color),
                (".ⓐ".len(), default_style.font_id, default_style.color),
                ("ⓑⓒ".len(), highlight_style.font_id, highlight_style.color),
                (
                    "ⓓⓔ.abcde.".len(),
                    default_style.font_id,
                    default_style.color
                ),
            ]
        );
    }
}
