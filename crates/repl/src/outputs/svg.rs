use anyhow::Result;
use gpui::{App, ClipboardItem, RenderImage, Window, img};
use std::sync::Arc;
use ui::{IntoElement, Styled, div, prelude::*};

use crate::outputs::OutputContent;

const SVG_SCALE_FACTOR: f32 = 2.0; //needed because svg_renderer has a smooth svg scale factor that would affect the div dimensions otherwise

pub struct SvgView {
    raw_svg: String,
    height: u32,
    width: u32,
    image: Arc<RenderImage>,
}

impl SvgView {
    pub fn from(svg_data: &str, cx: &App) -> Result<Self> {
        let renderer = cx.svg_renderer();
        let image = renderer.render_single_frame(svg_data.as_bytes(), 1.0, true)?;

        let size = image.size(0);
        let width = (size.width.0 as f32 / SVG_SCALE_FACTOR) as u32;
        let height = (size.height.0 as f32 / SVG_SCALE_FACTOR) as u32;

        Ok(SvgView {
            raw_svg: svg_data.to_string(),
            height,
            width,
            image,
        })
    }
}

impl Render for SvgView {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let line_height = window.line_height();

        let (height, width) = if self.height as f32 / f32::from(line_height) == u8::MAX as f32 {
            let height = u8::MAX as f32 * line_height;
            let width = self.width as f32 * height / self.height as f32;
            (height, width)
        } else {
            (self.height.into(), self.width.into())
        };

        let image = self.image.clone();

        div().h(height).w(width).child(img(image))
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
    fn test_svg_view_from_valid_svg(cx: &mut App) {
        let result = SvgView::from(SIMPLE_SVG, cx);

        assert!(result.is_ok());
        let view = result.unwrap();
        assert_eq!(view.raw_svg, SIMPLE_SVG);
        assert!(view.width > 0);
        assert!(view.height > 0);
    }

    #[gpui::test]
    fn test_svg_view_from_invalid_svg(cx: &mut App) {
        let result = SvgView::from("not valid svg content", cx);
        assert!(result.is_err());
    }

    #[gpui::test]
    fn test_svg_view_clipboard_contains_raw_svg(cx: &mut App) {
        let view = SvgView::from(SIMPLE_SVG, cx).unwrap();
        assert_eq!(view.raw_svg, SIMPLE_SVG);
    }
}
