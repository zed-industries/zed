#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `OrientationLockType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `OrientationLockType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrientationLockType {
    Any = "any",
    Natural = "natural",
    Landscape = "landscape",
    Portrait = "portrait",
    PortraitPrimary = "portrait-primary",
    PortraitSecondary = "portrait-secondary",
    LandscapePrimary = "landscape-primary",
    LandscapeSecondary = "landscape-secondary",
}
