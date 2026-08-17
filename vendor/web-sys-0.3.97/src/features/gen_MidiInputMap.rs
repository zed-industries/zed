#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MIDIInputMap",
        typescript_type = "MIDIInputMap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiInputMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
    pub type MidiInputMap;
    #[wasm_bindgen(method, getter, js_class = "MIDIInputMap", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
    pub fn size(this: &MidiInputMap) -> u32;
    #[wasm_bindgen(catch, method, js_class = "MIDIInputMap", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
    pub fn for_each(this: &MidiInputMap, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[cfg(feature = "MidiInput")]
    #[wasm_bindgen(method, js_class = "MIDIInputMap")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInput`, `MidiInputMap`*"]
    pub fn get(this: &MidiInputMap, key: &str) -> Option<MidiInput>;
    #[wasm_bindgen(method, js_class = "MIDIInputMap")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
    pub fn has(this: &MidiInputMap, key: &str) -> bool;
    #[wasm_bindgen(method, js_class = "MIDIInputMap")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
    pub fn entries(this: &MidiInputMap) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "MIDIInputMap")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
    pub fn keys(this: &MidiInputMap) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "MIDIInputMap")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
    pub fn values(this: &MidiInputMap) -> ::js_sys::Iterator;
}
