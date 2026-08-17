// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::{
    effective_target, emit_warning, get_rust_include_path, is_all_bindings, BindingOptions,
    EnvGuard, COPYRIGHT, PRELUDE,
};
use bindgen::callbacks::{ItemInfo, ParseCallbacks};
use std::fmt::Debug;
use std::path::Path;

#[derive(Debug)]
struct StripPrefixCallback {
    remove_prefix: Option<String>,
}

impl StripPrefixCallback {
    fn new(prefix: &str) -> StripPrefixCallback {
        StripPrefixCallback {
            remove_prefix: Some(prefix.to_string()),
        }
    }
}

impl ParseCallbacks for StripPrefixCallback {
    fn generated_name_override(&self, item_info: ItemInfo<'_>) -> Option<String> {
        self.remove_prefix.as_ref().and_then(|s| {
            let prefix = format!("{s}_");
            item_info
                .name
                .strip_prefix(prefix.as_str())
                .map(String::from)
        })
    }
}

const ALLOWED_HEADERS: [&str; 30] = [
    "aead.h",
    "aes.h",
    "base.h",
    "bn.h",
    "boringssl_prefix_symbols.h",
    "boringssl_prefix_symbols_asm.h",
    "boringssl_prefix_symbols_nasm.inc",
    "bytestring.h",
    "chacha.h",
    "cipher.h",
    "cmac.h",
    "crypto.h",
    "curve25519.h",
    "digest.h",
    "ec.h",
    "ec_key.h",
    "ecdh.h",
    "ecdsa.h",
    "err.h",
    "evp.h",
    "hkdf.h",
    "hmac.h",
    "is_awslc.h",
    "kdf.h",
    "mem.h",
    "nid.h",
    "poly1305.h",
    "rand.h",
    "rsa.h",
    "sha.h",
];

// These functions have parameters using a blocked type
const BLOCKED_FUNCTIONS: [&str; 5] = [
    "BN_print_fp",
    "CBS_parse_generalized_time",
    "CBS_parse_utc_time",
    "ERR_print_errors_fp",
    "RSA_print_fp",
];

// These types might vary by platform. Ensure that they do not appear in the universal bindings.
const BLOCKED_TYPES: [&str; 4] = ["FILE", "fpos_t", "tm", "__sFILE"];

fn prepare_bindings_builder(manifest_dir: &Path, options: &BindingOptions) -> bindgen::Builder {
    let clang_args = crate::prepare_clang_args(manifest_dir, options);

    let mut builder = bindgen::Builder::default()
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .allowlist_file(r".*(/|\\)rust_wrapper\.h")
        .rustified_enum(r"point_conversion_form_t")
        .rust_target(bindgen::RustTarget::stable(70, 0).unwrap())
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .generate_comments(true)
        .fit_macro_constants(false)
        .size_t_is_usize(true)
        .layout_tests(true)
        .prepend_enum_name(true)
        .formatter(bindgen::Formatter::Rustfmt)
        .clang_args(clang_args)
        .raw_line(COPYRIGHT)
        .header(
            get_rust_include_path(manifest_dir)
                .join("rust_wrapper.h")
                .display()
                .to_string(),
        );

    if is_all_bindings() {
        builder = builder.allowlist_file(r".*(/|\\)openssl((/|\\)[^/\\]+)+\.h");
    } else {
        for header in ALLOWED_HEADERS {
            emit_warning(format!("Allowed header: {header}").as_str());
            builder = builder.allowlist_file(format!("{}{}", r".*(/|\\)openssl(/|\\)", header));
        }
        for function in BLOCKED_FUNCTIONS {
            emit_warning(format!("Blocked function: {function}").as_str());
            builder = builder.blocklist_function(function);
        }
        for tipe in BLOCKED_TYPES {
            emit_warning(format!("Opaque type: {tipe}").as_str());
            builder = builder.blocklist_type(tipe);
        }
    }

    if !options.disable_prelude {
        builder = builder.raw_line(PRELUDE);
    }

    if options.include_ssl {
        builder = builder.clang_arg("-DAWS_LC_RUST_INCLUDE_SSL");
    }
    if let Some(prefix) = &options.build_prefix {
        let callbacks = StripPrefixCallback::new(prefix.as_str());
        builder = builder.parse_callbacks(Box::new(callbacks));
    }

    builder
}

pub(crate) fn generate_bindings(
    manifest_dir: &Path,
    options: &BindingOptions,
) -> bindgen::Bindings {
    let _guard_target = EnvGuard::new("TARGET", effective_target());
    prepare_bindings_builder(manifest_dir, options)
        .generate()
        .expect("Unable to generate bindings.")
}
