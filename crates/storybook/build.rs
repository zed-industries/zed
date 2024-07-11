fn main() {
    // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle.
    // TODO: We shouldn't depend on WebRTC in editor
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(target_env = "msvc")]
        {
            println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
        }
    }
}
