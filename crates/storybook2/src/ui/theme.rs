use std::sync::Arc;

use gpui3::WindowContext;

use crate::theme::Theme;
use crate::themes::rose_pine_dawn;

pub fn theme(cx: &WindowContext) -> Arc<Theme> {
    Arc::new(rose_pine_dawn())
}
