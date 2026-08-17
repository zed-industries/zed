//! ```cargo
//! [dependencies]
//! glob = "*"
//! ```

use glob::glob;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let parent_dir = env::var("CARGO_MAKE_WORKING_DIRECTORY").expect("CARGO_MAKE_WORKING_DIRECTORY not specified");
    let version_type = env::var("RUST_DECIMAL_BUILD_VERSION").expect("RUST_DECIMAL_BUILD_VERSION not specified");

    // Load in the build number
    let build_number = format!("{parent_dir}/.buildnumber");
    let (major, minor, revision) = parse_build_number(&build_number, &version_type)?;
    println!("{major}.{minor}.{revision}");

    // Process all cargo files
    for entry in glob("./Cargo.toml").expect("Failed to read glob pattern") {
        let path = entry?;
        let toml = fs::read_to_string(&path)?;
        let mut updated_toml = String::new();
        let mut added = false;
        for line in toml.lines() {
            if !added && line.starts_with("version =") {
                added = true;
                updated_toml.push_str(&format!("version = \"{major}.{minor}.{revision}\"\n"));
            } else {
                updated_toml.push_str(line);
                updated_toml.push('\n');
            }
        }
        fs::write(&path, updated_toml)?;
    }

    // Also, update the readme with the build number
    let readme_path = format!("{parent_dir}/README.md");
    let readme = fs::read_to_string(&readme_path)?;
    let mut updated_readme = String::new();
    for line in readme.lines() {
        if line.starts_with("rust_decimal = \"") {
            updated_readme.push_str(&format!("rust_decimal = \"{major}.{minor}\"\n"));
        } else if line.starts_with("rust_decimal_macros = \"") {
            updated_readme.push_str(&format!("rust_decimal_macros = \"{major}.{minor}\"\n"));
        } else {
            updated_readme.push_str(line);
            updated_readme.push('\n');
        }
    }
    fs::write(&readme_path, updated_readme)?;

    // Finally, write the build number back
    fs::write(build_number, format!("{major}\n{minor}\n{revision}\n",))?;
    Ok(())
}

fn parse_build_number(path: &String, version_type: &String) -> Result<(u32, u32, u32), Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let mut lines = io::BufReader::new(file).lines();
    let major: u32 = lines.next().expect("missing major version")?.parse()?;
    let minor: u32 = lines.next().expect("missing minor version")?.parse()?;
    let revision: u32 = lines.next().expect("missing revision version")?.parse()?;
    Ok(match &version_type[..] {
        "major" => (major + 1, 0, 0),
        "minor" => (major, minor + 1, 0),
        "revision" => (major, minor, revision + 1),
        _ => (major, minor, revision),
    })
}
