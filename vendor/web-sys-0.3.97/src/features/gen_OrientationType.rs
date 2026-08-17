#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `OrientationType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `OrientationType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrientationType {
    PortraitPrimary = "portrait-primary",
    PortraitSecondary = "portrait-secondary",
    LandscapePrimary = "landscape-primary",
    LandscapeSecondary = "landscape-secondary",
}
