#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MIDIConnectionEvent",
        typescript_type = "MIDIConnectionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiConnectionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEvent`*"]
    pub type MidiConnectionEvent;
    #[cfg(feature = "MidiPort")]
    #[wasm_bindgen(method, getter, js_class = "MIDIConnectionEvent", js_name = "port")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiPort`*"]
    pub fn port(this: &MidiConnectionEvent) -> Option<MidiPort>;
    #[wasm_bindgen(catch, constructor, js_class = "MIDIConnectionEvent")]
    #[doc = "The `new MidiConnectionEvent(..)` constructor, creating a new instance of `MidiConnectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEvent`*"]
    pub fn new(type_: &str) -> Result<MidiConnectionEvent, JsValue>;
    #[cfg(feature = "MidiConnectionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MIDIConnectionEvent")]
    #[doc = "The `new MidiConnectionEvent(..)` constructor, creating a new instance of `MidiConnectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiConnectionEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MidiConnectionEventInit,
    ) -> Result<MidiConnectionEvent, JsValue>;
}
