use std::{ops::Range, path::PathBuf};
use text::Point;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct EditorLanguageModelContext {
    pub icon_path: SharedString,
    pub path: Option<PathBuf>,
    // TODO: render the ranges as well
    pub selection_ranges: Vec<Range<Point>>,
    pub focused: bool,
}

// [ 🐍  my_file.py ]
// [ 🐍  my_file.py (1-5, 9-12) ]
impl RenderOnce for EditorLanguageModelContext {
    fn render(self, cx: &mut ui::prelude::WindowContext) -> impl ui::prelude::IntoElement {
        div()
            .h_flex()
            .child(Icon::from_path(self.icon_path.clone()))
            .child(self.path.map_or("Untitled".to_string(), |path| {
                path.to_string_lossy().to_string()
            }))
    }
}

// Questions for later:
// What does it look like to have the assistant panel have a close button to let go of this context, etc.
// Does this just show a tiny little element that gets used by something else?
//
