fn main() {
    if std::env::var("ZED_UPDATE_EXPLANATION").is_ok() {
        println!(r#"cargo:rustc-cfg=feature="no-bundled-uninstall""#);
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");
        // Weakly link ScreenCaptureKit to ensure can be used on macOS 10.15+.
        println!("cargo:rustc-link-arg=-Wl,-weak_framework,ScreenCaptureKit");
    }
}
