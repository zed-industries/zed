fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");

        println!("cargo:rerun-if-env-changed=ZED_BUNDLE");
        if std::env::var("ZED_BUNDLE").ok().as_deref() == Some("true") {
            // Find WebRTC.framework in the Frameworks folder when running as part of an application bundle.
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Frameworks");
        } else {
            // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle.
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        }
    }
}
