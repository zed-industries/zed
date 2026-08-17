use screencapturekit_sys::{
    content_filter::{UnsafeContentFilter, UnsafeInitParams::*},
    os_types::rc::{Id, ShareId},
    shareable_content::{UnsafeSCRunningApplication, UnsafeSCWindow},
};

use crate::{
    sc_display::SCDisplay, sc_running_application::SCRunningApplication, sc_window::SCWindow,
};

#[derive(Debug)]
pub struct SCContentFilter {
    pub(crate) _unsafe_ref: Id<UnsafeContentFilter>,
}

pub enum InitParams {
    DesktopIndependentWindow(SCWindow),
    Display(SCDisplay),
    DisplayIncludingWindows(SCDisplay, Vec<SCWindow>),
    DisplayExcludingWindows(SCDisplay, Vec<SCWindow>),
    DisplayIncludingApplicationsExceptingWindows(
        SCDisplay,
        Vec<SCRunningApplication>,
        Vec<SCWindow>,
    ),
    DisplayExcludingApplicationsExceptingWindows(
        SCDisplay,
        Vec<SCRunningApplication>,
        Vec<SCWindow>,
    ),
}

fn windows_to_unsafe(w: Vec<SCWindow>) -> Vec<ShareId<UnsafeSCWindow>> {
    w.into_iter().map(|w| w._unsafe_ref).collect()
}

fn applications_to_unsafe(
    a: Vec<SCRunningApplication>,
) -> Vec<ShareId<UnsafeSCRunningApplication>> {
    a.into_iter().map(|a| a._unsafe_ref).collect()
}

impl From<InitParams> for screencapturekit_sys::content_filter::UnsafeInitParams {
    fn from(value: InitParams) -> Self {
        match value {
            InitParams::DesktopIndependentWindow(w) => DesktopIndependentWindow(w._unsafe_ref),
            InitParams::Display(d) => Display(d._unsafe_ref),
            InitParams::DisplayIncludingWindows(d, w) => {
                DisplayIncludingWindows(d._unsafe_ref, windows_to_unsafe(w))
            }
            InitParams::DisplayExcludingWindows(d, w) => {
                DisplayExcludingWindows(d._unsafe_ref, windows_to_unsafe(w))
            }
            InitParams::DisplayIncludingApplicationsExceptingWindows(d, a, w) => {
                DisplayIncludingApplicationsExceptingWindows(
                    d._unsafe_ref,
                    applications_to_unsafe(a),
                    windows_to_unsafe(w),
                )
            }

            InitParams::DisplayExcludingApplicationsExceptingWindows(d, a, w) => {
                DisplayExcludingApplicationsExceptingWindows(
                    d._unsafe_ref,
                    applications_to_unsafe(a),
                    windows_to_unsafe(w),
                )
            }
        }
    }
}
impl SCContentFilter {
    pub fn new(params: InitParams) -> Self {
        Self {
            _unsafe_ref: UnsafeContentFilter::init(params.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sc_shareable_content::SCShareableContent;

    use super::InitParams::Display;
    use super::*;
    #[cfg_attr(feature = "ci", ignore)]
    #[test]
    fn test_sc_filter() {
        let display = SCShareableContent::current().displays.pop().unwrap();
        SCContentFilter::new(Display(display));
    }
}
