#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "JsonWebKey")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `JsonWebKey` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub type JsonWebKey;
    #[doc = "Get the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "alg")]
    pub fn get_alg(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "alg")]
    pub fn set_alg(this: &JsonWebKey, val: &str);
    #[doc = "Get the `crv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "crv")]
    pub fn get_crv(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `crv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "crv")]
    pub fn set_crv(this: &JsonWebKey, val: &str);
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "d")]
    pub fn get_d(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "d")]
    pub fn set_d(this: &JsonWebKey, val: &str);
    #[doc = "Get the `dp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "dp")]
    pub fn get_dp(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `dp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "dp")]
    pub fn set_dp(this: &JsonWebKey, val: &str);
    #[doc = "Get the `dq` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "dq")]
    pub fn get_dq(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `dq` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "dq")]
    pub fn set_dq(this: &JsonWebKey, val: &str);
    #[doc = "Get the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "e")]
    pub fn get_e(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "e")]
    pub fn set_e(this: &JsonWebKey, val: &str);
    #[doc = "Get the `ext` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "ext")]
    pub fn get_ext(this: &JsonWebKey) -> Option<bool>;
    #[doc = "Change the `ext` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "ext")]
    pub fn set_ext(this: &JsonWebKey, val: bool);
    #[doc = "Get the `k` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "k")]
    pub fn get_k(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `k` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "k")]
    pub fn set_k(this: &JsonWebKey, val: &str);
    #[doc = "Get the `key_ops` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "key_ops")]
    pub fn get_key_ops(this: &JsonWebKey) -> Option<::js_sys::Array>;
    #[doc = "Change the `key_ops` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "key_ops")]
    pub fn set_key_ops(this: &JsonWebKey, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `kty` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "kty")]
    pub fn get_kty(this: &JsonWebKey) -> ::alloc::string::String;
    #[doc = "Change the `kty` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "kty")]
    pub fn set_kty(this: &JsonWebKey, val: &str);
    #[doc = "Get the `n` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "n")]
    pub fn get_n(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `n` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "n")]
    pub fn set_n(this: &JsonWebKey, val: &str);
    #[doc = "Get the `oth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "oth")]
    pub fn get_oth(this: &JsonWebKey) -> Option<::js_sys::Array>;
    #[doc = "Change the `oth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "oth")]
    pub fn set_oth(this: &JsonWebKey, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `p` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "p")]
    pub fn get_p(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `p` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "p")]
    pub fn set_p(this: &JsonWebKey, val: &str);
    #[doc = "Get the `q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "q")]
    pub fn get_q(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "q")]
    pub fn set_q(this: &JsonWebKey, val: &str);
    #[doc = "Get the `qi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "qi")]
    pub fn get_qi(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `qi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "qi")]
    pub fn set_qi(this: &JsonWebKey, val: &str);
    #[doc = "Get the `use` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "use")]
    pub fn get_use(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `use` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "use")]
    pub fn set_use(this: &JsonWebKey, val: &str);
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "x")]
    pub fn get_x(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "x")]
    pub fn set_x(this: &JsonWebKey, val: &str);
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, getter = "y")]
    pub fn get_y(this: &JsonWebKey) -> Option<::alloc::string::String>;
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    #[wasm_bindgen(method, setter = "y")]
    pub fn set_y(this: &JsonWebKey, val: &str);
}
impl JsonWebKey {
    #[doc = "Construct a new `JsonWebKey`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn new(kty: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_kty(kty);
        ret
    }
    #[deprecated = "Use `set_alg()` instead."]
    pub fn alg(&mut self, val: &str) -> &mut Self {
        self.set_alg(val);
        self
    }
    #[deprecated = "Use `set_crv()` instead."]
    pub fn crv(&mut self, val: &str) -> &mut Self {
        self.set_crv(val);
        self
    }
    #[deprecated = "Use `set_d()` instead."]
    pub fn d(&mut self, val: &str) -> &mut Self {
        self.set_d(val);
        self
    }
    #[deprecated = "Use `set_dp()` instead."]
    pub fn dp(&mut self, val: &str) -> &mut Self {
        self.set_dp(val);
        self
    }
    #[deprecated = "Use `set_dq()` instead."]
    pub fn dq(&mut self, val: &str) -> &mut Self {
        self.set_dq(val);
        self
    }
    #[deprecated = "Use `set_e()` instead."]
    pub fn e(&mut self, val: &str) -> &mut Self {
        self.set_e(val);
        self
    }
    #[deprecated = "Use `set_ext()` instead."]
    pub fn ext(&mut self, val: bool) -> &mut Self {
        self.set_ext(val);
        self
    }
    #[deprecated = "Use `set_k()` instead."]
    pub fn k(&mut self, val: &str) -> &mut Self {
        self.set_k(val);
        self
    }
    #[deprecated = "Use `set_key_ops()` instead."]
    pub fn key_ops(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_key_ops(val);
        self
    }
    #[deprecated = "Use `set_kty()` instead."]
    pub fn kty(&mut self, val: &str) -> &mut Self {
        self.set_kty(val);
        self
    }
    #[deprecated = "Use `set_n()` instead."]
    pub fn n(&mut self, val: &str) -> &mut Self {
        self.set_n(val);
        self
    }
    #[deprecated = "Use `set_oth()` instead."]
    pub fn oth(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_oth(val);
        self
    }
    #[deprecated = "Use `set_p()` instead."]
    pub fn p(&mut self, val: &str) -> &mut Self {
        self.set_p(val);
        self
    }
    #[deprecated = "Use `set_q()` instead."]
    pub fn q(&mut self, val: &str) -> &mut Self {
        self.set_q(val);
        self
    }
    #[deprecated = "Use `set_qi()` instead."]
    pub fn qi(&mut self, val: &str) -> &mut Self {
        self.set_qi(val);
        self
    }
    #[deprecated = "Use `set_use()` instead."]
    pub fn use_(&mut self, val: &str) -> &mut Self {
        self.set_use(val);
        self
    }
    #[deprecated = "Use `set_x()` instead."]
    pub fn x(&mut self, val: &str) -> &mut Self {
        self.set_x(val);
        self
    }
    #[deprecated = "Use `set_y()` instead."]
    pub fn y(&mut self, val: &str) -> &mut Self {
        self.set_y(val);
        self
    }
}
