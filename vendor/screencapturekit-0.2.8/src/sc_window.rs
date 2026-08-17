use screencapturekit_sys::{os_types::rc::ShareId, shareable_content::UnsafeSCWindow};

use crate::sc_running_application::SCRunningApplication;

#[derive(Debug)]
pub struct SCWindow {
    pub(crate) _unsafe_ref: ShareId<UnsafeSCWindow>,
    pub width: u32,
    pub height: u32,
    pub title: Option<String>,
    pub owning_application: Option<SCRunningApplication>,
    pub window_id: u32,
    pub window_layer: u32,
    pub is_active: bool,
    pub is_on_screen: bool,
}

impl From<ShareId<UnsafeSCWindow>> for SCWindow {
    fn from(unsafe_ref: ShareId<UnsafeSCWindow>) -> Self {
        let frame = unsafe_ref.get_frame();
        SCWindow {
            title: unsafe_ref.get_title(),
            width: frame.size.width as u32,
            height: frame.size.height as u32,
            window_id: unsafe_ref.get_window_id(),
            window_layer: unsafe_ref.get_window_layer(),
            is_active: unsafe_ref.get_is_active() == 1,
            is_on_screen: unsafe_ref.get_is_on_screen() == 1,
            owning_application: unsafe_ref
                .get_owning_application()
                .map(SCRunningApplication::from),
            _unsafe_ref: unsafe_ref,
        }
    }
}
