use std::fmt::{Debug, Formatter};

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use wasm_streams::writable::*;

#[wasm_bindgen(module = "/tests/js/writable_stream.js")]
extern "C" {
    pub fn new_noop_writable_stream() -> sys::WritableStream;
    fn new_recording_writable_stream() -> WritableStreamAndEvents;

    #[derive(Clone, Debug)]
    type WritableStreamAndEvents;

    #[wasm_bindgen(method, getter)]
    fn stream(this: &WritableStreamAndEvents) -> sys::WritableStream;

    #[wasm_bindgen(method, getter)]
    fn events(this: &WritableStreamAndEvents) -> Box<[JsValue]>;

    #[derive(Clone, Debug)]
    type JsRecordedEvent;

    #[wasm_bindgen(method, getter, js_name = "type")]
    fn type_(this: &JsRecordedEvent) -> u8;

    #[wasm_bindgen(method, getter)]
    fn chunk(this: &JsRecordedEvent) -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn reason(this: &JsRecordedEvent) -> JsValue;
}

pub struct RecordingWritableStream {
    raw: WritableStreamAndEvents,
}

impl RecordingWritableStream {
    pub fn new() -> Self {
        Self {
            raw: new_recording_writable_stream(),
        }
    }

    pub fn stream(&self) -> sys::WritableStream {
        self.raw.stream()
    }

    pub fn events(&self) -> Vec<RecordedEvent> {
        self.raw
            .events()
            .iter()
            .map(|x| RecordedEvent::from(x.unchecked_ref::<JsRecordedEvent>()))
            .collect::<Vec<_>>()
    }
}

pub enum RecordedEvent {
    Write(JsValue),
    Close,
    Abort(JsValue),
}

impl From<&JsRecordedEvent> for RecordedEvent {
    fn from(js_event: &JsRecordedEvent) -> Self {
        match js_event.type_() {
            0 => RecordedEvent::Write(js_event.chunk()),
            1 => RecordedEvent::Close,
            2 => RecordedEvent::Abort(js_event.reason()),
            event_type => panic!("unknown event type: {}", event_type),
        }
    }
}

impl PartialEq for RecordedEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RecordedEvent::Write(left_val), RecordedEvent::Write(right_val)) => {
                if left_val.eq(right_val) {
                    true
                } else {
                    equal_uint8_array(left_val, right_val)
                }
            }
            (RecordedEvent::Close, RecordedEvent::Close) => true,
            (RecordedEvent::Abort(left_val), RecordedEvent::Abort(right_val)) => {
                left_val.eq(right_val)
            }
            _ => false,
        }
    }
}

fn equal_uint8_array(left: &JsValue, right: &JsValue) -> bool {
    match (left.dyn_ref::<Uint8Array>(), right.dyn_ref::<Uint8Array>()) {
        (Some(left_array), Some(right_array)) => left_array.to_vec().eq(&right_array.to_vec()),
        _ => false,
    }
}

impl Debug for RecordedEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordedEvent::Write(value) => {
                let mut tuple = f.debug_tuple("Write");
                if let Some(array) = value.dyn_ref::<Uint8Array>() {
                    tuple.field(&array.to_vec())
                } else {
                    tuple.field(value)
                };
                tuple.finish()
            }
            RecordedEvent::Close => f.debug_tuple("Close").finish(),
            RecordedEvent::Abort(value) => f.debug_tuple("Abort").field(value).finish(),
        }
    }
}
