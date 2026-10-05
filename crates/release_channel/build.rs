fn main() {
    println!("cargo::rustc-check-cfg=cfg(__do_not_set_zed_release_channel)");
    println!("cargo::rerun-if-env-changed=ZED_RELEASE_CHANNEL");

    // Enabling this `cfg` will cause a runtime panic if the
    // `ZED_RELEASE_CHANNEL` env var is not also set. TLDR: don't set the `cfg`
    // directly, just set the env var (hence the name).
    if std::env::var("ZED_RELEASE_CHANNEL").is_ok() {
        println!("cargo::rustc-cfg=__do_not_set_zed_release_channel");
    }
}
