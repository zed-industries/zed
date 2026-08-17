#[cfg(target_os="windows")]
extern crate cc;


#[cfg(not(target_os="windows"))]
fn main() {}

#[cfg(target_os="windows")]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ext/vswhom.cpp");

    let mut cc = cc::Build::new();
    cc.cpp(true);
    cc.file("ext/vswhom.cpp");
    if cc.get_compiler().is_like_msvc() {
        cc.flag("/Zm2000");
    }
    cc.compile("vswhom");

    println!("cargo:rustc-link-lib=dylib=OleAut32");
    println!("cargo:rustc-link-lib=dylib=Ole32");
}
