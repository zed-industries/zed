fn main() {
    use rustc_version::version;
    let version = version().expect("Can't get the rustc version");
    println!(
        "cargo:rustc-env=RUSTC_SEMVER={}.{}",
        version.major, version.minor
    );
}
