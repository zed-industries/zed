use libc::c_float;

use crate::{base::boolean_t, display_configuration::CGDisplayConfigRef, error::CGError};

pub type CGDisplayFadeReservationToken = u32;

pub const kCGDisplayFadeReservationInvalidToken: CGDisplayFadeReservationToken = 0;

pub type CGDisplayBlendFraction = c_float;

pub const kCGDisplayBlendNormal: CGDisplayBlendFraction = 0.0;
pub const kCGDisplayBlendSolidColor: CGDisplayBlendFraction = 1.0;

pub type CGDisplayFadeInterval = c_float;

pub type CGDisplayReservationInterval = c_float;

pub const kCGMaxDisplayReservationInterval: CGDisplayReservationInterval = 15.0;

extern "C" {
    pub fn CGConfigureDisplayFadeEffect(
        config: CGDisplayConfigRef,
        fadeOutSeconds: CGDisplayFadeInterval,
        fadeInSeconds: CGDisplayFadeInterval,
        fadeRed: c_float,
        fadeGreen: c_float,
        fadeBlue: c_float,
    ) -> CGError;
    pub fn CGAcquireDisplayFadeReservation(seconds: CGDisplayReservationInterval, token: *mut CGDisplayFadeReservationToken) -> CGError;
    pub fn CGReleaseDisplayFadeReservation(token: CGDisplayFadeReservationToken) -> CGError;
    pub fn CGDisplayFade(
        token: CGDisplayFadeReservationToken,
        duration: CGDisplayFadeInterval,
        startBlend: CGDisplayBlendFraction,
        endBlend: CGDisplayBlendFraction,
        redBlend: c_float,
        greenBlend: c_float,
        blueBlend: c_float,
        synchronous: boolean_t,
    ) -> CGError;
    pub fn CGDisplayFadeOperationInProgress() -> boolean_t;
}
