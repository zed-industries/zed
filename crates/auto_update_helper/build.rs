fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rerun-if-changed=manifest.xml");

        let mut res = winresource::WindowsResource::new();
        res.set_manifest_file("manifest.xml");

        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
