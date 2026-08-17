#[cfg(feature = "bindgen")]
use bindgen::callbacks::{ItemInfo, MacroParsingBehavior, ParseCallbacks};
#[cfg(feature = "bindgen")]
use bindgen::{MacroTypeVariation, RustTarget};
use std::io::Write;
use std::path::PathBuf;
#[cfg(not(feature = "bindgen"))]
use std::process;
use std::{env, fs};

const INCLUDES: &str = "
#include <openssl/aes.h>
#include <openssl/asn1.h>
#include <openssl/bio.h>
#include <openssl/cmac.h>
#include <openssl/conf.h>
#include <openssl/crypto.h>
#include <openssl/dh.h>
#include <openssl/dsa.h>
#include <openssl/ec.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/hmac.h>
#include <openssl/objects.h>
#include <openssl/opensslv.h>
#include <openssl/pem.h>
#include <openssl/pkcs12.h>
#include <openssl/pkcs7.h>
#include <openssl/rand.h>
#include <openssl/rsa.h>
#include <openssl/safestack.h>
#include <openssl/sha.h>
#include <openssl/ssl.h>
#include <openssl/stack.h>
#include <openssl/x509.h>
#include <openssl/x509_vfy.h>
#include <openssl/x509v3.h>

#if !defined(OPENSSL_IS_AWSLC)
// this must be included after ssl.h for libressl!
#include <openssl/srtp.h>
#endif

#if !(defined(OPENSSL_IS_BORINGSSL) || defined(OPENSSL_IS_AWSLC))
#include <openssl/cms.h>
#endif

#if !defined(OPENSSL_NO_COMP) && !(defined(OPENSSL_IS_BORINGSSL) || defined(OPENSSL_IS_AWSLC))
#include <openssl/comp.h>
#endif

#if !defined(OPENSSL_IS_BORINGSSL)
#include <openssl/ocsp.h>
#endif

#if !defined(LIBRESSL_VERSION_NUMBER) && OPENSSL_VERSION_NUMBER >= 0x10100000
#include <openssl/kdf.h>
#endif

#if OPENSSL_VERSION_NUMBER >= 0x30000000
#include <openssl/decoder.h>
#include <openssl/encoder.h>
#include <openssl/provider.h>
#include <openssl/params.h>
#include <openssl/param_build.h>
#endif

#if OPENSSL_VERSION_NUMBER >= 0x30200000
#include <openssl/quic.h>
#endif

#if defined(LIBRESSL_VERSION_NUMBER) || defined(OPENSSL_IS_BORINGSSL) || defined(OPENSSL_IS_AWSLC)
#include <openssl/poly1305.h>
#endif

#if defined(OPENSSL_IS_BORINGSSL)
#include <openssl/mldsa.h>
#include <openssl/mlkem.h>
#endif

#if OPENSSL_VERSION_NUMBER >= 0x30200000
#include <openssl/thread.h>
#endif
";

/// Attempts to find and use pre-generated AWS-LC Rust bindings from the
/// installation directory. Returns `true` if pre-generated bindings were
/// found and installed, `false` otherwise.
///
/// AWS-LC installations may ship pre-generated bindings at:
///   $PREFIX/share/rust/aws_lc_bindings.rs
///
/// where $PREFIX is the parent of the include directory. These bindings are
/// generated with the same bindgen options used by aws-lc-sys, and the
/// handful of static inline functions that matter (e.g., BIO_get_mem_data)
/// are already implemented in pure Rust in src/lib.rs.
///
/// **Contract:** The pregenerated bindings must not contain `extern "C"`
/// declarations for static inline wrapper functions (i.e., those that would
/// normally be compiled from `awslc_static_wrapper.c`). When this function
/// returns `true`, the caller skips both bindgen and the static wrapper
/// compilation step; the required shims are instead provided as pure Rust
/// in `openssl-sys/src/lib.rs`.
fn try_pregenerated_awslc_bindings(include_dirs: &[PathBuf]) -> bool {
    for include_dir in include_dirs {
        let Some(prefix_dir) = include_dir.parent() else {
            continue;
        };
        let bindings_path = prefix_dir
            .join("share")
            .join("rust")
            .join("aws_lc_bindings.rs");
        if bindings_path.exists() {
            let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
            println!(
                "cargo:warning=Using pre-generated AWS-LC bindings from {}",
                bindings_path.display()
            );
            println!("cargo:rerun-if-changed={}", bindings_path.display());
            println!("cargo:rustc-cfg=awslc_pregenerated");
            fs::copy(&bindings_path, out_dir.join("bindgen.rs"))
                .expect("Failed to copy pre-generated AWS-LC Rust bindings");
            return true;
        }
    }
    false
}

#[cfg(feature = "bindgen")]
pub fn run(include_dirs: &[PathBuf]) {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut builder = bindgen::builder()
        .parse_callbacks(Box::new(OpensslCallbacks))
        .rust_target(RustTarget::stable(70, 0).unwrap())
        .ctypes_prefix("::libc")
        .raw_line("use libc::*;")
        .raw_line("#[cfg(windows)] use std::os::windows::raw::HANDLE;")
        .raw_line("type evp_pkey_st = EVP_PKEY;")
        .allowlist_file(".*[/\\\\]openssl/[^/\\\\]+\\.h")
        .allowlist_recursively(false)
        // libc is missing pthread_once_t on macOS
        .blocklist_type("CRYPTO_ONCE")
        .blocklist_function("CRYPTO_THREAD_run_once")
        // we don't want to mess with va_list
        .blocklist_function("BIO_vprintf")
        .blocklist_function("BIO_vsnprintf")
        .blocklist_function("ERR_vset_error")
        .blocklist_function("ERR_add_error_vdata")
        .blocklist_function("EVP_KDF_vctrl")
        .blocklist_type("OSSL_FUNC_core_vset_error_fn")
        .blocklist_type("OSSL_FUNC_BIO_vprintf_fn")
        .blocklist_type("OSSL_FUNC_BIO_vsnprintf_fn")
        // struct hostent * does not exist on Windows
        .blocklist_function("BIO_gethostbyname")
        // Maintain compatibility for existing enum definitions
        .rustified_enum("point_conversion_form_t")
        // Maintain compatibility for pre-union definitions
        .blocklist_type("GENERAL_NAME")
        .blocklist_type("GENERAL_NAME_st")
        .blocklist_type("EVP_PKEY")
        .blocklist_type("evp_pkey_st")
        .layout_tests(false)
        .header_contents("includes.h", INCLUDES);

    for include_dir in include_dirs {
        builder = builder
            .clang_arg("-I")
            .clang_arg(include_dir.display().to_string());
    }

    builder
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindgen.rs"))
        .unwrap();
}

#[cfg(feature = "bindgen")]
pub fn run_boringssl(include_dirs: &[PathBuf]) {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::File::create(out_dir.join("boring_static_wrapper.h"))
        .expect("Failed to create boring_static_wrapper.h")
        .write_all(INCLUDES.as_bytes())
        .expect("Failed to write contents to boring_static_wrapper.h");

    let mut builder = bindgen::builder()
        .rust_target(RustTarget::stable(70, 0).unwrap())
        .ctypes_prefix("::libc")
        .raw_line("use libc::*;")
        .derive_default(false)
        .enable_function_attribute_detection()
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .rustified_enum("point_conversion_form_t")
        .allowlist_file(".*[/\\\\]openssl/[^/]+\\.h")
        .allowlist_recursively(false)
        .blocklist_function("BIO_vsnprintf")
        .blocklist_function("OPENSSL_vasprintf")
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("boring_static_wrapper").display().to_string())
        .layout_tests(false)
        .header(
            out_dir
                .join("boring_static_wrapper.h")
                .display()
                .to_string(),
        );

    for include_dir in include_dirs {
        builder = builder
            .clang_arg("-I")
            .clang_arg(include_dir.display().to_string());
    }

    builder
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindgen.rs"))
        .unwrap();

    cc::Build::new()
        .file(out_dir.join("boring_static_wrapper.c"))
        .includes(include_dirs)
        .compile("boring_static_wrapper");
}

#[cfg(not(feature = "bindgen"))]
pub fn run_boringssl(include_dirs: &[PathBuf]) {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::File::create(out_dir.join("boring_static_wrapper.h"))
        .expect("Failed to create boring_static_wrapper.h")
        .write_all(INCLUDES.as_bytes())
        .expect("Failed to write contents to boring_static_wrapper.h");

    let mut bindgen_cmd = process::Command::new("bindgen");
    bindgen_cmd
        .arg("-o")
        .arg(out_dir.join("bindgen.rs"))
        .arg("--rust-target=1.80")
        .arg("--ctypes-prefix=::libc")
        .arg("--raw-line=use libc::*;")
        .arg("--no-derive-default")
        .arg("--enable-function-attribute-detection")
        .arg("--default-macro-constant-type=signed")
        .arg("--rustified-enum=point_conversion_form_t")
        .arg("--allowlist-file=.*[/\\\\]openssl/[^/]+\\.h")
        .arg("--no-recursive-allowlist")
        .arg("--blocklist-function=BIO_vsnprintf")
        .arg("--blocklist-function=OPENSSL_vasprintf")
        .arg("--experimental")
        .arg("--wrap-static-fns")
        .arg("--wrap-static-fns-path")
        .arg(out_dir.join("boring_static_wrapper").display().to_string())
        .arg("--no-layout-tests")
        .arg(out_dir.join("boring_static_wrapper.h"))
        .arg("--")
        .arg(format!("--target={}", env::var("TARGET").unwrap()));

    for include_dir in include_dirs {
        bindgen_cmd.arg("-I").arg(include_dir.display().to_string());
    }

    let result = bindgen_cmd.status().expect("bindgen failed to execute");
    assert!(result.success());

    cc::Build::new()
        .file(out_dir.join("boring_static_wrapper.c"))
        .includes(include_dirs)
        .compile("boring_static_wrapper");
}

#[cfg(feature = "bindgen")]
mod bindgen_options {
    use bindgen::callbacks::{ItemInfo, ParseCallbacks};

    #[derive(Debug)]
    pub struct StripPrefixCallback {
        remove_prefix: Option<String>,
    }

    impl StripPrefixCallback {
        pub fn new(prefix: &str) -> StripPrefixCallback {
            StripPrefixCallback {
                remove_prefix: Some(prefix.to_string()),
            }
        }
    }

    impl ParseCallbacks for StripPrefixCallback {
        fn generated_name_override(&self, item_info: ItemInfo<'_>) -> Option<String> {
            self.remove_prefix
                .as_ref()
                .and_then(|s| item_info.name.strip_prefix(s.as_str()).map(String::from))
        }
    }
}

#[cfg(feature = "bindgen")]
pub fn run_awslc(include_dirs: &[PathBuf], symbol_prefix: Option<String>) {
    // If the AWS-LC installation ships pre-generated Rust bindings, use them
    // directly. These bindings already handle symbol prefix stripping.
    if try_pregenerated_awslc_bindings(include_dirs) {
        return;
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::File::create(out_dir.join("awslc_static_wrapper.h"))
        .expect("Failed to create awslc_static_wrapper.h")
        .write_all(INCLUDES.as_bytes())
        .expect("Failed to write contents to awslc_static_wrapper.h");

    let mut builder = bindgen::builder()
        .rust_target(RustTarget::stable(70, 0).unwrap())
        .ctypes_prefix("::libc")
        .raw_line("use libc::*;")
        .derive_default(false)
        .enable_function_attribute_detection()
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .rustified_enum("point_conversion_form_t")
        .allowlist_file(r".*(/|\\)openssl((/|\\)[^/\\]+)+\.h")
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("awslc_static_wrapper").display().to_string())
        .layout_tests(false)
        .header(out_dir.join("awslc_static_wrapper.h").display().to_string());

    if let Some(prefix) = symbol_prefix {
        use bindgen_options::StripPrefixCallback;
        let callback = StripPrefixCallback::new(prefix.as_str());
        builder = builder.parse_callbacks(Box::from(callback));
    }

    for include_dir in include_dirs {
        builder = builder
            .clang_arg("-I")
            .clang_arg(include_dir.display().to_string());
    }

    builder
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindgen.rs"))
        .unwrap();

    cc::Build::new()
        .file(out_dir.join("awslc_static_wrapper.c"))
        .includes(include_dirs)
        .compile("awslc_static_wrapper");
}

#[cfg(not(feature = "bindgen"))]
pub fn run_awslc(include_dirs: &[PathBuf], symbol_prefix: Option<String>) {
    // If the AWS-LC installation ships pre-generated Rust bindings, use them
    // directly. These bindings already handle symbol prefix stripping.
    if try_pregenerated_awslc_bindings(include_dirs) {
        return;
    }

    if symbol_prefix.is_some() {
        panic!("aws-lc installation has prefixed symbols, but bindgen-cli does not support removing prefixes. \
        Enable the bindgen crate feature to support this installation.")
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fs::File::create(out_dir.join("awslc_static_wrapper.h"))
        .expect("Failed to create awslc_static_wrapper.h")
        .write_all(INCLUDES.as_bytes())
        .expect("Failed to write contents to awslc_static_wrapper.h");

    let mut bindgen_cmd = process::Command::new("bindgen");
    bindgen_cmd
        .arg("-o")
        .arg(out_dir.join("bindgen.rs"))
        .arg("--rust-target=1.80")
        .arg("--ctypes-prefix=::libc")
        .arg("--raw-line=use libc::*;")
        .arg("--no-derive-default")
        .arg("--enable-function-attribute-detection")
        .arg("--default-macro-constant-type=signed")
        .arg("--rustified-enum=point_conversion_form_t")
        .arg(r"--allowlist-file=.*(/|\\)openssl((/|\\)[^/\\]+)+\.h")
        .arg("--experimental")
        .arg("--wrap-static-fns")
        .arg("--wrap-static-fns-path")
        .arg(out_dir.join("awslc_static_wrapper").display().to_string())
        .arg(out_dir.join("awslc_static_wrapper.h"))
        .arg("--")
        .arg(format!("--target={}", env::var("TARGET").unwrap()));

    for include_dir in include_dirs {
        bindgen_cmd.arg("-I").arg(include_dir.display().to_string());
    }

    let result = bindgen_cmd.status().expect("bindgen failed to execute");
    assert!(result.success());

    cc::Build::new()
        .file(out_dir.join("awslc_static_wrapper.c"))
        .includes(include_dirs)
        .compile("awslc_static_wrapper");
}

#[cfg(feature = "bindgen")]
#[derive(Debug)]
struct OpensslCallbacks;

#[cfg(feature = "bindgen")]
impl ParseCallbacks for OpensslCallbacks {
    // for now we'll continue hand-writing constants
    fn will_parse_macro(&self, _name: &str) -> MacroParsingBehavior {
        MacroParsingBehavior::Ignore
    }

    fn item_name(&self, item_info: ItemInfo) -> Option<String> {
        match item_info.name {
            // Our original definitions of these are wrong, so rename to avoid breakage
            "CRYPTO_EX_new"
            | "CRYPTO_EX_dup"
            | "CRYPTO_EX_free"
            | "BIO_meth_set_write"
            | "BIO_meth_set_read"
            | "BIO_meth_set_puts"
            | "BIO_meth_set_ctrl"
            | "BIO_meth_set_create"
            | "BIO_meth_set_destroy"
            | "CRYPTO_set_locking_callback"
            | "CRYPTO_set_id_callback"
            | "SSL_CTX_set_tmp_dh_callback"
            | "SSL_set_tmp_dh_callback"
            | "SSL_CTX_set_tmp_ecdh_callback"
            | "SSL_set_tmp_ecdh_callback"
            | "SSL_CTX_callback_ctrl"
            | "SSL_CTX_set_alpn_select_cb" => Some(format!("{}__fixed_rust", item_info.name)),
            // On NetBSD, "off_t" is generated as "__off_t".
            "__off_t" => Some("off_t".to_string()),
            _ => None,
        }
    }
}
