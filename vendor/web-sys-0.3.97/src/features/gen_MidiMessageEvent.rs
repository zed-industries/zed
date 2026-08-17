#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MIDIMessageEvent",
        typescript_type = "MIDIMessageEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiMessageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
    pub type MidiMessageEvent;
    #[wasm_bindgen(catch, method, getter, js_class = "MIDIMessageEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
    pub fn data(this: &MidiMessageEvent) -> Result<::alloc::vec::Vec<u8>, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "MIDIMessageEvent")]
    #[doc = "The `new MidiMessageEvent(..)` constructor, creating a new instance of `MidiMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/MIDIMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
    pub fn new(type_: &str) -> Result<MidiMessageEvent, JsValue>;
    #[cfg(feature = "MidiMessageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MIDIMessageEvent")]
    #[doc = "The `new MidiMessageEvent(..)` constructor, creating a new instance of `MidiMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/MIDIMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiMessageEvent`, `MidiMessageEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MidiMessageEventInit,
    ) -> Result<MidiMessageEvent, JsValue>;
}
