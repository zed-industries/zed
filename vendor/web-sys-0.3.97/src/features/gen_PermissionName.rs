#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PermissionName` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PermissionName`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionName {
    Geolocation = "geolocation",
    Notifications = "notifications",
    Push = "push",
    PersistentStorage = "persistent-storage",
}
