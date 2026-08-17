// This file is based on code from:
//   https://github.com/rustwasm/wasm-bindgen/blob/main/examples/wasm-audio-worklet/src/dependent_module.rs
//
// The original code is licensed under either of:
//   - MIT license (https://opensource.org/licenses/MIT)
//   - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// at your option.
//
// Copyright (c) 2017-2024 The wasm-bindgen Developers
//
// This file incorporates code from the above source under the Apache License, Version 2.0 license.
// Please see the original repository for more details.
//
// See this issue for a further explanation of what this file does: https://github.com/rustwasm/wasm-bindgen/issues/3019

use js_sys::{wasm_bindgen, Array, JsString};
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, Url};

// This is a not-so-clean approach to get the current bindgen ES module URL
// in Rust. This will fail at run time on bindgen targets not using ES modules.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    type ImportMeta;

    #[wasm_bindgen(method, getter)]
    fn url(this: &ImportMeta) -> JsString;

    #[wasm_bindgen(thread_local_v2, js_namespace = import, js_name = meta)]
    static IMPORT_META: ImportMeta;
}

pub fn on_the_fly(code: &str) -> Result<String, JsValue> {
    // Generate the import of the bindgen ES module, assuming `--target web`.
    let header = format!(
        "import init, * as bindgen from '{}';\n\n",
        IMPORT_META.with(ImportMeta::url),
    );

    let options = BlobPropertyBag::new();
    options.set_type("text/javascript");
    Url::create_object_url_with_blob(&Blob::new_with_str_sequence_and_options(
        &Array::of2(&JsValue::from(header.as_str()), &JsValue::from(code)),
        &options,
    )?)
}

// dependent_module! takes a local file name to a JS module as input and
// returns a URL to a slightly modified module in run time. This modified module
// has an additional import statement in the header that imports the current
// bindgen JS module under the `bindgen` alias, and the separate init function.
// How this URL is produced does not matter for the macro user. on_the_fly
// creates a blob URL in run time. A better, more sophisticated solution
// would add wasm_bindgen support to put such a module in pkg/ during build time
// and return a URL to this file instead (described in #3019).
#[macro_export]
macro_rules! dependent_module {
    ($file_name:expr) => {
        $crate::host::audioworklet::dependent_module::on_the_fly(include_str!($file_name))
    };
}
