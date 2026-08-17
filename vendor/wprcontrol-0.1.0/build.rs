fn main() {
    println!("cargo:rerun-if-changed=.windows/winmd/Microsoft.WPRControl.winmd");
    println!("cargo:rerun-if-changed=build.rs");

    let args = [
        "--in",
        "default",
        ".windows/winmd/Microsoft.WPRControl.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Microsoft.WPRControl",
        "--flat",
        "--reference",
        "windows,skip-root,Windows",
    ];

    let warnings = windows_bindgen::bindgen(args);
    if !warnings.is_empty() {
        eprintln!("windows-bindgen warnings: {warnings}");
    }
}
