fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rerun-if-changed=manifest.xml");

        let mut res = winresource::WindowsResource::new();
        res.set_manifest_file("manifest.xml");
        res.set_icon("app-icon.ico");

        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
