#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MIDIOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    pub type MidiOptions;
    #[doc = "Get the `software` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    #[wasm_bindgen(method, getter = "software")]
    pub fn get_software(this: &MidiOptions) -> Option<bool>;
    #[doc = "Change the `software` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    #[wasm_bindgen(method, setter = "software")]
    pub fn set_software(this: &MidiOptions, val: bool);
    #[doc = "Get the `sysex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    #[wasm_bindgen(method, getter = "sysex")]
    pub fn get_sysex(this: &MidiOptions) -> Option<bool>;
    #[doc = "Change the `sysex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    #[wasm_bindgen(method, setter = "sysex")]
    pub fn set_sysex(this: &MidiOptions, val: bool);
}
impl MidiOptions {
    #[doc = "Construct a new `MidiOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_software()` instead."]
    pub fn software(&mut self, val: bool) -> &mut Self {
        self.set_software(val);
        self
    }
    #[deprecated = "Use `set_sysex()` instead."]
    pub fn sysex(&mut self, val: bool) -> &mut Self {
        self.set_sysex(val);
        self
    }
}
impl Default for MidiOptions {
    fn default() -> Self {
        Self::new()
    }
}
