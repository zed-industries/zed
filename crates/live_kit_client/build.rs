fn main() {
    println!("cargo:rustc-link-lib=objc");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=Foundation");
}
