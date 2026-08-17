#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MIDIOutputMap",
        typescript_type = "MIDIOutputMap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiOutputMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
    pub type MidiOutputMap;
    #[wasm_bindgen(method, getter, js_class = "MIDIOutputMap", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
    pub fn size(this: &MidiOutputMap) -> u32;
    #[wasm_bindgen(catch, method, js_class = "MIDIOutputMap", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
    pub fn for_each(this: &MidiOutputMap, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[cfg(feature = "MidiOutput")]
    #[wasm_bindgen(method, js_class = "MIDIOutputMap")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`, `MidiOutputMap`*"]
    pub fn get(this: &MidiOutputMap, key: &str) -> Option<MidiOutput>;
    #[wasm_bindgen(method, js_class = "MIDIOutputMap")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
    pub fn has(this: &MidiOutputMap, key: &str) -> bool;
    #[wasm_bindgen(method, js_class = "MIDIOutputMap")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
    pub fn entries(this: &MidiOutputMap) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "MIDIOutputMap")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
    pub fn keys(this: &MidiOutputMap) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "MIDIOutputMap")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
    pub fn values(this: &MidiOutputMap) -> ::js_sys::Iterator;
}
