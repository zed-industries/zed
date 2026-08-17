fn main() {
    let is_nightly = version_check::is_feature_flaggable() == Some(true);
    let is_at_least_1_49 = version_check::is_min_version("1.49.0").unwrap_or(false);
    let is_at_least_1_51 = version_check::is_min_version("1.51.0").unwrap_or(false);

    if !is_at_least_1_49 {
        println!("cargo:warning=slotmap requires rustc => 1.49.0");
    }

    if is_at_least_1_51 || is_nightly {
        println!("cargo:rustc-cfg=has_min_const_generics");
    }

    if is_nightly {
        println!("cargo:rustc-cfg=nightly");
    }
}
