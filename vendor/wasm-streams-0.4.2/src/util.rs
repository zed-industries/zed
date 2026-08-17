use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub(crate) async fn promise_to_void_future(promise: Promise) -> Result<(), JsValue> {
    let js_value = JsFuture::from(promise).await?;
    debug_assert!(js_value.is_undefined());
    let _ = js_value;
    Ok(())
}

pub(crate) fn clamp_to_u32(value: usize) -> u32 {
    let wrapped = value as u32;
    let overflow = value != (wrapped as usize);
    if overflow {
        u32::MAX
    } else {
        wrapped
    }
}

pub(crate) fn clamp_to_usize(value: u32) -> usize {
    let wrapped = value as usize;
    let overflow = value != (wrapped as u32);
    if overflow {
        usize::MAX
    } else {
        wrapped
    }
}

pub(crate) fn checked_cast_to_u32(value: usize) -> u32 {
    let wrapped = value as u32;
    debug_assert_eq!(value, wrapped as usize);
    wrapped
}

pub(crate) fn checked_cast_to_usize(value: u32) -> usize {
    let wrapped = value as usize;
    debug_assert_eq!(value, wrapped as u32);
    wrapped
}

pub(crate) fn js_to_io_error(js_value: JsValue) -> std::io::Error {
    let message = js_to_string(&js_value).unwrap_or_else(|| "Unknown error".to_string());
    std::io::Error::new(std::io::ErrorKind::Other, message)
}

fn js_to_string(js_value: &JsValue) -> Option<String> {
    js_value.as_string().or_else(|| {
        js_sys::Object::try_from(js_value)
            .map(|js_object| js_object.to_string().as_string().unwrap_throw())
    })
}
