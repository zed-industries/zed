use anyhow::{Context as _, Result};
use gpui::{App, ClipboardItem, Pixels, RenderImage, Window, img, px};
use std::sync::Arc;
use ui::{IntoElement, Styled, div, prelude::*};

use crate::outputs::OutputContent;

const SVG_SCALE_FACTOR: f32 = 2.0;

pub struct SvgView {
    raw_svg: String,
    width: Pixels,
    height: Pixels,
    image: Arc<RenderImage>,
}

impl SvgView {
    pub fn from(svg_data: &str, cx: &App) -> Result<Self> {
        let renderer = cx.svg_renderer();
        let image = renderer
            .render_single_frame(svg_data.as_bytes(), 1.0, true)
            .context("rendering SVG")?;

        let size = image.size(0);
        let width = px(size.width.0 as f32 / SVG_SCALE_FACTOR);
        let height = px(size.height.0 as f32 / SVG_SCALE_FACTOR);

        Ok(Self {
            raw_svg: svg_data.to_string(),
            width,
            height,
            image,
        })
    }
}

impl Render for SvgView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(self.height)
            .w(self.width)
            .child(img(self.image.clone()))
    }
}

impl OutputContent for SvgView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.raw_svg.clone()))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect width="100" height="100" fill="red"/></svg>"#;

    #[gpui::test]
    fn test_valid_svg(cx: &mut App) {
        let result = SvgView::from(SIMPLE_SVG, cx);
        assert!(result.is_ok());

        let view = result.unwrap();
        assert_eq!(view.raw_svg, SIMPLE_SVG);
        assert!(view.width > Pixels::ZERO);
        assert!(view.height > Pixels::ZERO);
    }

    #[gpui::test]
    fn test_invalid_svg(cx: &mut App) {
        let result = SvgView::from("not valid svg content", cx);
        assert!(result.is_err());
    }
}
