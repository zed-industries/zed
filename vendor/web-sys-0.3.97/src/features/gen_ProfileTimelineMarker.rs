#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ProfileTimelineMarker")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProfileTimelineMarker` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub type ProfileTimelineMarker;
    #[doc = "Get the `causeName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "causeName")]
    pub fn get_cause_name(this: &ProfileTimelineMarker) -> Option<::alloc::string::String>;
    #[doc = "Change the `causeName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "causeName")]
    pub fn set_cause_name(this: &ProfileTimelineMarker, val: &str);
    #[doc = "Get the `end` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "end")]
    pub fn get_end(this: &ProfileTimelineMarker) -> Option<f64>;
    #[doc = "Change the `end` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "end")]
    pub fn set_end(this: &ProfileTimelineMarker, val: f64);
    #[doc = "Get the `endStack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "endStack")]
    pub fn get_end_stack(this: &ProfileTimelineMarker) -> Option<::js_sys::Object>;
    #[doc = "Change the `endStack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "endStack")]
    pub fn set_end_stack(this: &ProfileTimelineMarker, val: Option<&::js_sys::Object>);
    #[doc = "Get the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "eventPhase")]
    pub fn get_event_phase(this: &ProfileTimelineMarker) -> Option<u16>;
    #[doc = "Change the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "eventPhase")]
    pub fn set_event_phase(this: &ProfileTimelineMarker, val: u16);
    #[doc = "Get the `isAnimationOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "isAnimationOnly")]
    pub fn get_is_animation_only(this: &ProfileTimelineMarker) -> Option<bool>;
    #[doc = "Change the `isAnimationOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "isAnimationOnly")]
    pub fn set_is_animation_only(this: &ProfileTimelineMarker, val: bool);
    #[doc = "Get the `isOffMainThread` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "isOffMainThread")]
    pub fn get_is_off_main_thread(this: &ProfileTimelineMarker) -> Option<bool>;
    #[doc = "Change the `isOffMainThread` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "isOffMainThread")]
    pub fn set_is_off_main_thread(this: &ProfileTimelineMarker, val: bool);
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[doc = "Get the `messagePortOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineMessagePortOperationType`*"]
    #[wasm_bindgen(method, getter = "messagePortOperation")]
    pub fn get_message_port_operation(
        this: &ProfileTimelineMarker,
    ) -> Option<ProfileTimelineMessagePortOperationType>;
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[doc = "Change the `messagePortOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineMessagePortOperationType`*"]
    #[wasm_bindgen(method, setter = "messagePortOperation")]
    pub fn set_message_port_operation(
        this: &ProfileTimelineMarker,
        val: ProfileTimelineMessagePortOperationType,
    );
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ProfileTimelineMarker) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ProfileTimelineMarker, val: &str);
    #[doc = "Get the `processType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "processType")]
    pub fn get_process_type(this: &ProfileTimelineMarker) -> Option<u16>;
    #[doc = "Change the `processType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "processType")]
    pub fn set_process_type(this: &ProfileTimelineMarker, val: u16);
    #[doc = "Get the `rectangles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "rectangles")]
    pub fn get_rectangles(this: &ProfileTimelineMarker) -> Option<::js_sys::Array>;
    #[doc = "Change the `rectangles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "rectangles")]
    pub fn set_rectangles(this: &ProfileTimelineMarker, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `stack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "stack")]
    pub fn get_stack(this: &ProfileTimelineMarker) -> Option<::js_sys::Object>;
    #[doc = "Change the `stack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "stack")]
    pub fn set_stack(this: &ProfileTimelineMarker, val: Option<&::js_sys::Object>);
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "start")]
    pub fn get_start(this: &ProfileTimelineMarker) -> Option<f64>;
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "start")]
    pub fn set_start(this: &ProfileTimelineMarker, val: f64);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ProfileTimelineMarker) -> Option<::alloc::string::String>;
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ProfileTimelineMarker, val: &str);
    #[doc = "Get the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, getter = "unixTime")]
    pub fn get_unix_time(this: &ProfileTimelineMarker) -> Option<f64>;
    #[doc = "Change the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "unixTime")]
    pub fn set_unix_time(this: &ProfileTimelineMarker, val: f64);
    #[doc = "Change the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "unixTime")]
    pub fn set_unix_time_u32(this: &ProfileTimelineMarker, val: u32);
    #[doc = "Change the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    #[wasm_bindgen(method, setter = "unixTime")]
    pub fn set_unix_time_f64(this: &ProfileTimelineMarker, val: f64);
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[doc = "Get the `workerOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineWorkerOperationType`*"]
    #[wasm_bindgen(method, getter = "workerOperation")]
    pub fn get_worker_operation(
        this: &ProfileTimelineMarker,
    ) -> Option<ProfileTimelineWorkerOperationType>;
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[doc = "Change the `workerOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineWorkerOperationType`*"]
    #[wasm_bindgen(method, setter = "workerOperation")]
    pub fn set_worker_operation(
        this: &ProfileTimelineMarker,
        val: ProfileTimelineWorkerOperationType,
    );
}
impl ProfileTimelineMarker {
    #[doc = "Construct a new `ProfileTimelineMarker`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cause_name()` instead."]
    pub fn cause_name(&mut self, val: &str) -> &mut Self {
        self.set_cause_name(val);
        self
    }
    #[deprecated = "Use `set_end()` instead."]
    pub fn end(&mut self, val: f64) -> &mut Self {
        self.set_end(val);
        self
    }
    #[deprecated = "Use `set_end_stack()` instead."]
    pub fn end_stack(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_end_stack(val);
        self
    }
    #[deprecated = "Use `set_event_phase()` instead."]
    pub fn event_phase(&mut self, val: u16) -> &mut Self {
        self.set_event_phase(val);
        self
    }
    #[deprecated = "Use `set_is_animation_only()` instead."]
    pub fn is_animation_only(&mut self, val: bool) -> &mut Self {
        self.set_is_animation_only(val);
        self
    }
    #[deprecated = "Use `set_is_off_main_thread()` instead."]
    pub fn is_off_main_thread(&mut self, val: bool) -> &mut Self {
        self.set_is_off_main_thread(val);
        self
    }
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[deprecated = "Use `set_message_port_operation()` instead."]
    pub fn message_port_operation(
        &mut self,
        val: ProfileTimelineMessagePortOperationType,
    ) -> &mut Self {
        self.set_message_port_operation(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_process_type()` instead."]
    pub fn process_type(&mut self, val: u16) -> &mut Self {
        self.set_process_type(val);
        self
    }
    #[deprecated = "Use `set_rectangles()` instead."]
    pub fn rectangles(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_rectangles(val);
        self
    }
    #[deprecated = "Use `set_stack()` instead."]
    pub fn stack(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_stack(val);
        self
    }
    #[deprecated = "Use `set_start()` instead."]
    pub fn start(&mut self, val: f64) -> &mut Self {
        self.set_start(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_unix_time()` instead."]
    pub fn unix_time(&mut self, val: f64) -> &mut Self {
        self.set_unix_time(val);
        self
    }
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[deprecated = "Use `set_worker_operation()` instead."]
    pub fn worker_operation(&mut self, val: ProfileTimelineWorkerOperationType) -> &mut Self {
        self.set_worker_operation(val);
        self
    }
}
impl Default for ProfileTimelineMarker {
    fn default() -> Self {
        Self::new()
    }
}
