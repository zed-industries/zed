use crate::{CVReturn, CVTime};
use core_foundation_sys::base::CFTypeID;

// avoid depending on core-graphics
pub type CGDirectDisplayID = u32;

#[derive(Debug, Copy, Clone)]
pub enum __CVDisplayLink {}
pub type CVDisplayLinkRef = *mut __CVDisplayLink;

extern "C" {
    pub fn CVDisplayLinkGetTypeID() -> CFTypeID;
    pub fn CVDisplayLinkCreateWithCGDisplay(
        displayID: CGDirectDisplayID,
        displayLinkOut: *mut CVDisplayLinkRef,
    ) -> CVReturn;
    pub fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(displayLink: CVDisplayLinkRef)
        -> CVTime;
    pub fn CVDisplayLinkRelease(displayLink: CVDisplayLinkRef);
}
