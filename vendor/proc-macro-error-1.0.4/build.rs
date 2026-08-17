fn main() {
    if !version_check::is_feature_flaggable().unwrap_or(false) {
        println!("cargo:rustc-cfg=use_fallback");
    }

    if version_check::is_max_version("1.38.0").unwrap_or(false)
        || !version_check::Channel::read().unwrap().is_stable()
    {
        println!("cargo:rustc-cfg=skip_ui_tests");
    }
}
