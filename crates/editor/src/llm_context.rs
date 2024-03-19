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
    fn render(self, _cx: &mut ui::prelude::WindowContext) -> impl ui::prelude::IntoElement {
        let file_name = self.path.map_or("Untitled".to_string(), |path| {
            path.to_string_lossy().to_string()
        });
        let file_name_text_color = if self.focused {
            Color::Default
        } else {
            Color::Muted
        };

        div()
            .h_flex()
            .child(Icon::from_path(self.icon_path.clone()))
            .child(
                div()
                    .h_6()
                    .child(Label::new(file_name).color(file_name_text_color))
                    .ml_1(),
            )
    }
}

// Questions for later:
// What does it look like to have the assistant panel have a close button to let go of this context, etc.
// Does this just show a tiny little element that gets used by something else?
//
