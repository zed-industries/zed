fn main() {
    // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle.
    // TODO: We shouldn't depend on WebRTC in editor
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
}
