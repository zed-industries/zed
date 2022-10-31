fn main() {
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");

    if let Ok(value) = std::env::var("ZED_MIXPANEL_TOKEN") {
        println!("cargo:rustc-env=ZED_MIXPANEL_TOKEN={value}");
    }
    if let Ok(value) = std::env::var("ZED_AMPLITUDE_API_KEY") {
        println!("cargo:rustc-env=ZED_AMPLITUDE_API_KEY={value}");
    }
    if let Ok(value) = std::env::var("ZED_PREVIEW_CHANNEL") {
        println!("cargo:rustc-env=ZED_PREVIEW_CHANNEL={value}");
    }

    if std::env::var("ZED_BUNDLE").ok().as_deref() == Some("true") {
        // Find WebRTC.framework in the Frameworks folder when running as part of an application bundle.
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Frameworks");
    } else {
        // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle.
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
    }

    // Seems to be required to enable Swift concurrency
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    // Register exported Objective-C selectors, protocols, etc
    println!("cargo:rustc-link-arg=-Wl,-ObjC");
}
