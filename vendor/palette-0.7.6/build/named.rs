use std::fs::File;

pub fn build() {
    use std::path::Path;

    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("named.rs");
    let mut writer = File::create(dest_path).expect("couldn't create named.rs");
    build_colors(&mut writer);
}

#[cfg(feature = "named")]
pub fn build_colors(writer: &mut File) {
    use std::io::{BufRead, BufReader, Write};

    let reader =
        BufReader::new(File::open("build/svg_colors.txt").expect("could not open svg_colors.txt"));
    let mut entries = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split('\t');
        let name = parts.next().expect("couldn't get the color name");
        let mut rgb = parts
            .next()
            .unwrap_or_else(|| panic!("couldn't get color for {}", name))
            .split(", ");
        let red: u8 = rgb
            .next()
            .and_then(|r| r.trim().parse().ok())
            .unwrap_or_else(|| panic!("couldn't get red for {}", name));
        let green: u8 = rgb
            .next()
            .and_then(|r| r.trim().parse().ok())
            .unwrap_or_else(|| panic!("couldn't get green for {}", name));
        let blue: u8 = rgb
            .next()
            .and_then(|r| r.trim().parse().ok())
            .unwrap_or_else(|| panic!("couldn't get blue for {}", name));

        writeln!(writer, "\n///<div style=\"display: inline-block; width: 3em; height: 1em; border: 1px solid black; background: {0};\"></div>", name).unwrap();
        writeln!(
            writer,
            "pub const {}: crate::rgb::Srgb<u8> = crate::rgb::Srgb::new({}, {}, {});",
            name.to_uppercase(),
            red,
            green,
            blue
        )
        .unwrap();

        entries.push((name.to_owned(), name.to_uppercase()));
    }

    gen_from_str(writer, &entries)
}

#[cfg(feature = "named_from_str")]
fn gen_from_str(writer: &mut File, entries: &[(String, String)]) {
    use std::io::Write;

    writer
        .write_all(
            "static COLORS: ::phf::Map<&'static str, crate::rgb::Srgb<u8>> = phf::phf_map! {\n"
                .as_bytes(),
        )
        .unwrap();

    for (key, value) in entries {
        writeln!(writer, "    \"{}\" => {},", key, value).unwrap();
    }

    writer.write_all("};\n".as_bytes()).unwrap();
}

#[cfg(not(feature = "named"))]
pub fn build_colors(_writer: &mut File) {}

#[allow(unused)]
#[cfg(not(feature = "named_from_str"))]
fn gen_from_str(_writer: &mut File, _entries: &[(String, String)]) {}
