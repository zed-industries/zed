use gpui::geometry::rect::RectF;

pub fn paint_layer<F>(cx: &mut gpui::PaintContext, clip_bounds: Option<RectF>, f: F)
where
    F: FnOnce(&mut gpui::PaintContext) -> (),
{
    cx.scene.push_layer(clip_bounds);
    f(cx);
    cx.scene.pop_layer()
}
