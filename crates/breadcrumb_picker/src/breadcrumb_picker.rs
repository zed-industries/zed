mod directory;
mod symbol;

use std::rc::Rc;

use editor::{
    BREADCRUMB_PICKER_RENDERERS, BreadcrumbPickerRenderers, ErasedBreadcrumbPopoverHandle,
};
use gpui::App;

pub(crate) const MAX_BREADCRUMB_MENU_ENTRIES: usize = 200;

pub fn init(_cx: &mut App) {
    BREADCRUMB_PICKER_RENDERERS
        .set(BreadcrumbPickerRenderers {
            directory: directory::render_breadcrumb_directory_segment,
            symbol: symbol::render_breadcrumb_symbol_segment,
            popover_handle: default_popover_handle,
        })
        .ok();
}

fn default_popover_handle() -> Rc<dyn ErasedBreadcrumbPopoverHandle> {
    Rc::new(directory::DirectoryPopoverHandle(Default::default()))
}
