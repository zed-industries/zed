use gpui::AppContext;

#[derive(Default)]
pub struct FontSizeDelta(pub f32);

pub fn adjust_font_size_delta(cx: &mut AppContext, f: fn(&mut f32, cx: &mut AppContext)) {
    cx.update_default_global::<FontSizeDelta, _, _>(|size, cx| {
        f(&mut size.0, cx);
    });
    cx.refresh_windows();
}

pub fn font_size_for_setting(size: f32, cx: &AppContext) -> f32 {
    if cx.has_global::<FontSizeDelta>() {
        size + cx.global::<FontSizeDelta>().0
    } else {
        size
    }
}
