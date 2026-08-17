// Copyright © 2016-2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
fn main() {
    use std::env::var;
    use std::path::Path;
    println!("cargo:rerun-if-env-changed=WINAPI_NO_BUNDLED_LIBRARIES");
    if var("WINAPI_NO_BUNDLED_LIBRARIES").is_ok() {
        return;
    }
    if var("TARGET").map(|target| target == "i686-pc-windows-gnu").unwrap_or(false) {
        let dir = var("CARGO_MANIFEST_DIR").unwrap();
        println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());
    }
}
