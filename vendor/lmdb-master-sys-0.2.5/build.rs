extern crate cc;

#[cfg(feature = "bindgen")]
extern crate bindgen;

#[cfg(feature = "bindgen")]
#[path = "bindgen.rs"]
mod generate;

use std::env;
use std::path::PathBuf;

#[cfg(all(
    feature = "mdb_idl_logn_8",
    not(any(
        feature = "mdb_idl_logn_9",
        feature = "mdb_idl_logn_10",
        feature = "mdb_idl_logn_11",
        feature = "mdb_idl_logn_12",
        feature = "mdb_idl_logn_13",
        feature = "mdb_idl_logn_14",
        feature = "mdb_idl_logn_15",
        feature = "mdb_idl_logn_16"
    ))
))]
const MDB_IDL_LOGN: u8 = 8;
#[cfg(all(
    feature = "mdb_idl_logn_9",
    not(any(
        feature = "mdb_idl_logn_10",
        feature = "mdb_idl_logn_11",
        feature = "mdb_idl_logn_12",
        feature = "mdb_idl_logn_13",
        feature = "mdb_idl_logn_14",
        feature = "mdb_idl_logn_15",
        feature = "mdb_idl_logn_16"
    ))
))]
const MDB_IDL_LOGN: u8 = 9;
#[cfg(all(
    feature = "mdb_idl_logn_10",
    not(any(
        feature = "mdb_idl_logn_11",
        feature = "mdb_idl_logn_12",
        feature = "mdb_idl_logn_13",
        feature = "mdb_idl_logn_14",
        feature = "mdb_idl_logn_15",
        feature = "mdb_idl_logn_16"
    ))
))]
const MDB_IDL_LOGN: u8 = 10;
#[cfg(all(
    feature = "mdb_idl_logn_11",
    not(any(
        feature = "mdb_idl_logn_12",
        feature = "mdb_idl_logn_13",
        feature = "mdb_idl_logn_14",
        feature = "mdb_idl_logn_15",
        feature = "mdb_idl_logn_16"
    ))
))]
const MDB_IDL_LOGN: u8 = 11;
#[cfg(all(
    feature = "mdb_idl_logn_12",
    not(any(
        feature = "mdb_idl_logn_13",
        feature = "mdb_idl_logn_14",
        feature = "mdb_idl_logn_15",
        feature = "mdb_idl_logn_16"
    ))
))]
const MDB_IDL_LOGN: u8 = 12;
#[cfg(all(
    feature = "mdb_idl_logn_13",
    not(any(
        feature = "mdb_idl_logn_14",
        feature = "mdb_idl_logn_15",
        feature = "mdb_idl_logn_16"
    ))
))]
const MDB_IDL_LOGN: u8 = 13;
#[cfg(all(
    feature = "mdb_idl_logn_14",
    not(any(feature = "mdb_idl_logn_15", feature = "mdb_idl_logn_16"))
))]
const MDB_IDL_LOGN: u8 = 14;
#[cfg(all(feature = "mdb_idl_logn_15", not(any(feature = "mdb_idl_logn_16"))))]
const MDB_IDL_LOGN: u8 = 15;
#[cfg(any(
    feature = "mdb_idl_logn_16",
    not(any(
        feature = "mdb_idl_logn_8",
        feature = "mdb_idl_logn_9",
        feature = "mdb_idl_logn_10",
        feature = "mdb_idl_logn_11",
        feature = "mdb_idl_logn_12",
        feature = "mdb_idl_logn_13",
        feature = "mdb_idl_logn_14",
        feature = "mdb_idl_logn_15",
        feature = "mdb_idl_logn_16",
    ))
))]
const MDB_IDL_LOGN: u8 = 16;

macro_rules! warn {
    ($message:expr) => {
        println!("cargo:warning={}", $message);
    };
}

fn main() {
    #[cfg(feature = "bindgen")]
    generate::generate();

    println!("cargo::rerun-if-changed=lmdb");

    let mut lmdb = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    lmdb.push("lmdb");
    lmdb.push("libraries");
    lmdb.push("liblmdb");

    if cfg!(feature = "fuzzer") && cfg!(feature = "fuzzer-no-link") {
        warn!("Features `fuzzer` and `fuzzer-no-link` are mutually exclusive.");
        warn!("Building with `-fsanitize=fuzzer`.");
    }

    let mut builder = cc::Build::new();

    builder
        .define("MDB_IDL_LOGN", Some(MDB_IDL_LOGN.to_string().as_str()))
        .file(lmdb.join("mdb.c"))
        .file(lmdb.join("midl.c"))
        // https://github.com/mozilla/lmdb/blob/b7df2cac50fb41e8bd16aab4cc5fd167be9e032a/libraries/liblmdb/Makefile#L23
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wbad-function-cast")
        .flag_if_supported("-Wuninitialized");

    if cfg!(feature = "posix-sem") {
        builder.define("MDB_USE_POSIX_SEM", None);
    }

    if cfg!(feature = "use-valgrind") {
        builder.define("USE_VALGRIND", None);
    }

    if cfg!(feature = "asan") {
        builder.flag("-fsanitize=address");
    }

    if cfg!(feature = "fuzzer") {
        builder.flag("-fsanitize=fuzzer");
    } else if cfg!(feature = "fuzzer-no-link") {
        builder.flag("-fsanitize=fuzzer-no-link");
    }

    if cfg!(feature = "longer-keys") {
        builder.define("MDB_MAXKEYSIZE", "0");
    }

    if !cfg!(debug_assertions) {
        builder.define("NDEBUG", None);
    }

    builder.compile("liblmdb.a")
}
