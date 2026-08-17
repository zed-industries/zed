#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DeviceMotionEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceMotionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub type DeviceMotionEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &DeviceMotionEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &DeviceMotionEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &DeviceMotionEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &DeviceMotionEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &DeviceMotionEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &DeviceMotionEventInit, val: bool);
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Get the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "acceleration")]
    pub fn get_acceleration(this: &DeviceMotionEventInit) -> Option<DeviceAccelerationInit>;
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Change the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, setter = "acceleration")]
    pub fn set_acceleration(this: &DeviceMotionEventInit, val: &DeviceAccelerationInit);
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Get the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "accelerationIncludingGravity")]
    pub fn get_acceleration_including_gravity(
        this: &DeviceMotionEventInit,
    ) -> Option<DeviceAccelerationInit>;
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Change the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, setter = "accelerationIncludingGravity")]
    pub fn set_acceleration_including_gravity(
        this: &DeviceMotionEventInit,
        val: &DeviceAccelerationInit,
    );
    #[doc = "Get the `interval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, getter = "interval")]
    pub fn get_interval(this: &DeviceMotionEventInit) -> Option<f64>;
    #[doc = "Change the `interval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    #[wasm_bindgen(method, setter = "interval")]
    pub fn set_interval(this: &DeviceMotionEventInit, val: Option<f64>);
    #[cfg(feature = "DeviceRotationRateInit")]
    #[doc = "Get the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, getter = "rotationRate")]
    pub fn get_rotation_rate(this: &DeviceMotionEventInit) -> Option<DeviceRotationRateInit>;
    #[cfg(feature = "DeviceRotationRateInit")]
    #[doc = "Change the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, setter = "rotationRate")]
    pub fn set_rotation_rate(this: &DeviceMotionEventInit, val: &DeviceRotationRateInit);
}
impl DeviceMotionEventInit {
    #[doc = "Construct a new `DeviceMotionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[deprecated = "Use `set_acceleration()` instead."]
    pub fn acceleration(&mut self, val: &DeviceAccelerationInit) -> &mut Self {
        self.set_acceleration(val);
        self
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[deprecated = "Use `set_acceleration_including_gravity()` instead."]
    pub fn acceleration_including_gravity(&mut self, val: &DeviceAccelerationInit) -> &mut Self {
        self.set_acceleration_including_gravity(val);
        self
    }
    #[deprecated = "Use `set_interval()` instead."]
    pub fn interval(&mut self, val: Option<f64>) -> &mut Self {
        self.set_interval(val);
        self
    }
    #[cfg(feature = "DeviceRotationRateInit")]
    #[deprecated = "Use `set_rotation_rate()` instead."]
    pub fn rotation_rate(&mut self, val: &DeviceRotationRateInit) -> &mut Self {
        self.set_rotation_rate(val);
        self
    }
}
impl Default for DeviceMotionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
