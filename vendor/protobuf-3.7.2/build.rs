use std::env;
use std::env::VarError;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn cfg_serde() {
    match env::var("CARGO_FEATURE_WITH_SERDE") {
        Ok(_) => {
            println!("cargo:rustc-cfg=serde");
        }
        Err(VarError::NotUnicode(..)) => panic!(),
        Err(VarError::NotPresent) => {}
    }
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"))
}

fn version() -> String {
    env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION")
}

fn write_version() {
    let version = version();
    let version_ident = format!(
        "VERSION_{}",
        version.replace(".", "_").replace("-", "_").to_uppercase()
    );
    let mut file = File::create(Path::join(&out_dir(), "version.rs")).expect("open");
    writeln!(file, "/// protobuf crate version").unwrap();
    writeln!(file, "pub const VERSION: &'static str = \"{}\";", version).unwrap();
    writeln!(file, "/// This symbol is used by codegen").unwrap();
    writeln!(file, "#[doc(hidden)]").unwrap();
    writeln!(
        file,
        "pub const VERSION_IDENT: &'static str = \"{}\";",
        version_ident
    )
    .unwrap();
    writeln!(
        file,
        "/// This symbol can be referenced to assert that proper version of crate is used"
    )
    .unwrap();
    writeln!(file, "pub const {}: () = ();", version_ident).unwrap();
    file.flush().unwrap();
}

fn main() {
    cfg_serde();
    write_version();
}
