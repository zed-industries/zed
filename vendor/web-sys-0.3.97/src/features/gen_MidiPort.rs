#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MIDIPort",
        typescript_type = "MIDIPort"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiPort` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub type MidiPort;
    #[wasm_bindgen(method, getter, js_class = "MIDIPort", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn id(this: &MidiPort) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MIDIPort", js_name = "manufacturer")]
    #[doc = "Getter for the `manufacturer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/manufacturer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn manufacturer(this: &MidiPort) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "MIDIPort", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn name(this: &MidiPort) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "MIDIPort", js_name = "version")]
    #[doc = "Getter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn version(this: &MidiPort) -> Option<::alloc::string::String>;
    #[cfg(feature = "MidiPortType")]
    #[wasm_bindgen(method, getter, js_class = "MIDIPort", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`, `MidiPortType`*"]
    pub fn type_(this: &MidiPort) -> MidiPortType;
    #[cfg(feature = "MidiPortDeviceState")]
    #[wasm_bindgen(method, getter, js_class = "MIDIPort", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`, `MidiPortDeviceState`*"]
    pub fn state(this: &MidiPort) -> MidiPortDeviceState;
    #[cfg(feature = "MidiPortConnectionState")]
    #[wasm_bindgen(method, getter, js_class = "MIDIPort", js_name = "connection")]
    #[doc = "Getter for the `connection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/connection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`, `MidiPortConnectionState`*"]
    pub fn connection(this: &MidiPort) -> MidiPortConnectionState;
    #[wasm_bindgen(method, getter, js_class = "MIDIPort", js_name = "onstatechange")]
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn onstatechange(this: &MidiPort) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MIDIPort", js_name = "onstatechange")]
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn set_onstatechange(this: &MidiPort, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, js_class = "MIDIPort")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn close(this: &MidiPort) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "MIDIPort")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn open(this: &MidiPort) -> ::js_sys::Promise;
}
