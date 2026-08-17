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
        js_name = "MIDIOutput",
        typescript_type = "MIDIOutput"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiOutput` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`*"]
    pub type MidiOutput;
    #[wasm_bindgen(method, js_class = "MIDIOutput")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`*"]
    pub fn clear(this: &MidiOutput);
    #[wasm_bindgen(catch, method, js_class = "MIDIOutput")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`*"]
    pub fn send(this: &MidiOutput, data: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MIDIOutput", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`*"]
    pub fn send_with_timestamp(
        this: &MidiOutput,
        data: &::wasm_bindgen::JsValue,
        timestamp: f64,
    ) -> Result<(), JsValue>;
}
