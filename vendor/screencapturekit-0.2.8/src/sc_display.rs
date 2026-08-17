use screencapturekit_sys::{
    os_types::{geometry::CGRect, rc::ShareId},
    shareable_content::UnsafeSCDisplay,
};

#[derive(Debug, Clone)]
pub struct SCDisplay {
    pub(crate) _unsafe_ref: ShareId<UnsafeSCDisplay>,
    pub display_id: u32,
    pub frame: CGRect,
    pub width: u32,
    pub height: u32,
}

impl From<ShareId<UnsafeSCDisplay>> for SCDisplay {
    fn from(unsafe_ref: ShareId<UnsafeSCDisplay>) -> Self {
        SCDisplay {
            display_id: unsafe_ref.get_display_id(),
            frame: unsafe_ref.get_frame(),
            width: unsafe_ref.get_width(),
            height: unsafe_ref.get_height(),
            _unsafe_ref: unsafe_ref,
        }
    }
}
