fn main() {
    // Find WebRTC.framework as a sibling of the executable when running outside of an application bundle.
    // TODO: We shouldn't depend on WebRTC in editor
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");

    #[cfg(target_os = "windows")]
    {
        #[cfg(target_env = "msvc")]
        {
            println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
        }

        let manifest = std::path::Path::new("../zed/resources/windows/manifest.xml");
        println!("cargo:rerun-if-changed={}", manifest.display());
        embed_manifest::embed_manifest(embed_manifest::new_manifest(manifest.to_str().unwrap()))
            .unwrap();
    }
}
