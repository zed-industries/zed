use gpui::{Context, Hsla};
use theme::ActiveTheme;

use crate::constants::DIFF_HIGHLIGHT_ALPHA;

pub fn get_diff_colors<T>(cx: &Context<T>) -> (Hsla, Hsla, Hsla) {
    let theme = cx.theme();
    let mut deleted_bg = theme.status().deleted_background;
    deleted_bg.a = DIFF_HIGHLIGHT_ALPHA;
    let mut created_bg = theme.status().created_background;
    created_bg.a = DIFF_HIGHLIGHT_ALPHA;
    let mut modified_bg = theme.status().modified_background;
    modified_bg.a = DIFF_HIGHLIGHT_ALPHA;
    (deleted_bg, created_bg, modified_bg)
}

