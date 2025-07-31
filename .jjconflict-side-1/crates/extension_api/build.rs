fn main() {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut parts = version.split(|c: char| !c.is_ascii_digit());
    let major = parts.next().unwrap().parse::<u16>().unwrap().to_be_bytes();
    let minor = parts.next().unwrap().parse::<u16>().unwrap().to_be_bytes();
    let patch = parts.next().unwrap().parse::<u16>().unwrap().to_be_bytes();

    std::fs::write(
        std::path::Path::new(&out_dir).join("version_bytes"),
        [major[0], major[1], minor[0], minor[1], patch[0], patch[1]],
    )
    .unwrap();
}
