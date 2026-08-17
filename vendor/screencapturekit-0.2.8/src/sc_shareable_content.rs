use screencapturekit_sys::{os_types::rc::Id, shareable_content::UnsafeSCShareableContent};

use crate::{
    sc_display::SCDisplay, sc_running_application::SCRunningApplication, sc_window::SCWindow,
};

#[derive(Debug)]
pub struct SCShareableContent {
    _unsafe_ref: Id<UnsafeSCShareableContent>,
    pub windows: Vec<SCWindow>,
    pub applications: Vec<SCRunningApplication>,
    pub displays: Vec<SCDisplay>,
}

impl SCShareableContent {
    pub fn current() -> Self {
        SCShareableContent::try_current().unwrap()
    }

    pub fn try_current() -> Result<Self, String> {
        let unsafe_ref = UnsafeSCShareableContent::get()?;

        let windows: Vec<SCWindow> = unsafe_ref
            .windows()
            .into_iter()
            .map(SCWindow::from)
            .collect();

        let applications = unsafe_ref
            .applications()
            .into_iter()
            .map(SCRunningApplication::from)
            .collect();

        let displays = unsafe_ref
            .displays()
            .into_iter()
            .map(SCDisplay::from)
            .collect();

        Ok(SCShareableContent {
            windows,
            applications,
            displays,
            _unsafe_ref: unsafe_ref,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sc_shareable_content() {
        SCShareableContent::current();
    }
}
