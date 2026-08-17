fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_err() {
        eprintln!("error: winreg is only supported on Windows platforms");
        eprintln!(
            "help: if your application is multi-platform, use \
            `[target.'cfg(windows)'.dependencies] winreg = \"...\"`"
        );
        eprintln!("help: if your application is only supported on Windows, use `--target x86_64-pc-windows-gnu` or some other windows platform");
        std::process::exit(1);
    }
}
