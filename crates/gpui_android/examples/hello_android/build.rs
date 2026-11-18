fn main() {
    // Android resolves a native library's symbols only when it loads it, so
    // without this a missing system library surfaces as a crash at launch.
    println!("cargo::rustc-link-arg=-Wl,--no-undefined");
}
