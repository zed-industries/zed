fn main() {
    // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");

    // Register exported Objective-C selectors, protocols, etc
    println!("cargo:rustc-link-arg=-Wl,-ObjC");
}
