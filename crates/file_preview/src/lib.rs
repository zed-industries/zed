pub mod formats;
mod preview_view;

pub use formats::{FilePreviewFormat, MermaidFormat, SvgFormat};
pub use preview_view::{FilePreviewView, PreviewMode};

use gpui::App;

pub fn init(_cx: &mut App) {
    // TODO: implement in Task 4
}
