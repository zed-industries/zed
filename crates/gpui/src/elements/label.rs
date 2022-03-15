use crate::{
    fonts::TextStyle,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{ToJson, Value},
    text_layout::{Line, RunStyle},
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

#[derive(Clone, Debug, Deserialize, Default)]
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

impl LabelStyle {
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.text.font_size = font_size;
        self
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

    fn compute_runs(&self) -> SmallVec<[(usize, RunStyle); 8]> {
        let font_id = self.style.text.font_id;
        if self.highlight_indices.is_empty() {
            return smallvec![(
                self.text.len(),
                RunStyle {
                    font_id,
                    color: self.style.text.color,
                    underline: self.style.text.underline,
                }
            )];
        }

        let highlight_font_id = self
            .style
            .highlight_text
            .as_ref()
            .map_or(font_id, |style| style.font_id);

        let mut highlight_indices = self.highlight_indices.iter().copied().peekable();
        let mut runs = SmallVec::new();
        let highlight_style = self
            .style
            .highlight_text
            .as_ref()
            .unwrap_or(&self.style.text);

        for (char_ix, c) in self.text.char_indices() {
            let mut font_id = font_id;
            let mut color = self.style.text.color;
            let mut underline = self.style.text.underline;
            if let Some(highlight_ix) = highlight_indices.peek() {
                if char_ix == *highlight_ix {
                    font_id = highlight_font_id;
                    color = highlight_style.color;
                    underline = highlight_style.underline;
                    highlight_indices.next();
                }
            }

            let last_run: Option<&mut (usize, RunStyle)> = runs.last_mut();
            let push_new_run = if let Some((last_len, last_style)) = last_run {
                if font_id == last_style.font_id
                    && color == last_style.color
                    && underline == last_style.underline
                {
                    *last_len += c.len_utf8();
                    false
                } else {
                    true
                }
            } else {
                true
            };

            if push_new_run {
                runs.push((
                    c.len_utf8(),
                    RunStyle {
                        font_id,
                        color,
                        underline,
                    },
                ));
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
            line.width()
                .ceil()
                .max(constraint.min.x())
                .min(constraint.max.x()),
            cx.font_cache
                .line_height(self.style.text.font_id, self.style.text.font_size),
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
        line.paint(bounds.origin(), visible_bounds, bounds.size().y(), cx)
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
    use crate::color::Color;
    use crate::fonts::{Properties as FontProperties, Weight};

    #[crate::test(self)]
    fn test_layout_label_with_highlights(cx: &mut crate::MutableAppContext) {
        let default_style = TextStyle::new(
            "Menlo",
            12.,
            Default::default(),
            Default::default(),
            Color::black(),
            cx.font_cache(),
        )
        .unwrap();
        let highlight_style = TextStyle::new(
            "Menlo",
            12.,
            *FontProperties::new().weight(Weight::BOLD),
            Default::default(),
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

        let default_run_style = RunStyle {
            font_id: default_style.font_id,
            color: default_style.color,
            underline: default_style.underline,
        };
        let highlight_run_style = RunStyle {
            font_id: highlight_style.font_id,
            color: highlight_style.color,
            underline: highlight_style.underline,
        };
        let runs = label.compute_runs();
        assert_eq!(
            runs.as_slice(),
            &[
                (".α".len(), default_run_style),
                ("βγ".len(), highlight_run_style),
                ("δ".len(), default_run_style),
                ("ε".len(), highlight_run_style),
                (".ⓐ".len(), default_run_style),
                ("ⓑⓒ".len(), highlight_run_style),
                ("ⓓⓔ.abcde.".len(), default_run_style),
            ]
        );
    }
}
