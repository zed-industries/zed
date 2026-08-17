fn main() {
    if version_check::is_max_version("1.36.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=always_assert_unwind");
    }
}
