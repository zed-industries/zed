#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::env;

include!("no_atomic.rs");

// The rustc-cfg strings below are *not* public API. Please let us know by
// opening a GitHub issue if your build environment requires some way to enable
// these cfgs other than by executing our build script.
fn main() {
    println!("cargo:rerun-if-changed=no_atomic.rs");
    println!("cargo:rustc-check-cfg=cfg(valuable_no_atomic_cas,valuable_no_atomic,valuable_no_atomic_64)");

    let target = match env::var("TARGET") {
        Ok(target) => target,
        Err(e) => {
            println!(
                "cargo:warning=valuable: unable to get TARGET environment variable: {}",
                e
            );
            return;
        }
    };

    // Note that this is `no_*`, not `has_*`. This allows treating
    // `cfg(target_has_atomic = "ptr")` as true when the build script doesn't
    // run. This is needed for compatibility with non-cargo build systems that
    // don't run the build script.
    if NO_ATOMIC_CAS.contains(&&*target) {
        println!("cargo:rustc-cfg=valuable_no_atomic_cas");
    }
    if NO_ATOMIC.contains(&&*target) {
        println!("cargo:rustc-cfg=valuable_no_atomic");
        println!("cargo:rustc-cfg=valuable_no_atomic_64");
    } else if NO_ATOMIC_64.contains(&&*target) {
        println!("cargo:rustc-cfg=valuable_no_atomic_64");
    } else {
        // Otherwise, assuming `"max-atomic-width" == 64`.
    }
}
