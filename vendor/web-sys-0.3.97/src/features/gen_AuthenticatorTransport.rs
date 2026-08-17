#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AuthenticatorTransport` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AuthenticatorTransport`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatorTransport {
    Usb = "usb",
    Nfc = "nfc",
    Ble = "ble",
    Internal = "internal",
}
