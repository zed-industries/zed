#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "MidiPort",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MIDIInput",
        typescript_type = "MIDIInput"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiInput` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInput`*"]
    pub type MidiInput;
    #[wasm_bindgen(method, getter, js_class = "MIDIInput", js_name = "onmidimessage")]
    #[doc = "Getter for the `onmidimessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInput`*"]
    pub fn onmidimessage(this: &MidiInput) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MIDIInput", js_name = "onmidimessage")]
    #[doc = "Setter for the `onmidimessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInput`*"]
    pub fn set_onmidimessage(this: &MidiInput, value: Option<&::js_sys::Function>);
}
