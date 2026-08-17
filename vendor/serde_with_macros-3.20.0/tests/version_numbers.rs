//! Ensure version numbers in various files are up to date.

#[track_caller]
fn check_contains_regex(path: &str, template: &str) {
    #[track_caller]
    fn inner(path: &str, template: &str) -> Result<(), String> {
        // Expand the placeholders in the template.
        let pattern = template
            .replace("{name}", &regex::escape(env!("CARGO_PKG_NAME")))
            .replace("{version}", &regex::escape(env!("CARGO_PKG_VERSION")));
        let re = regex::Regex::new(&pattern)
            .map_err(|err| format!("could not parse template: {}", err))?;
        let text = std::fs::read_to_string(path)
            .map_err(|err| format!("could not read {}: {}", path, err))?;

        println!("Searching for \"{pattern}\" in {path}...");
        match re.find(&text) {
            Some(m) => {
                let line_no = text[..m.start()].lines().count();
                println!("{} (line {}) ... ok", path, line_no + 1);
                Ok(())
            }
            None => Err(format!("could not find \"{pattern}\" in {path}")),
        }
    }

    if let Err(e) = inner(path, template) {
        panic!("{e}");
    }
}

#[test]
fn test_changelog() {
    check_contains_regex("CHANGELOG.md", r"## \[{version}\]");
}

#[test]
fn test_serde_with_dependency() {
    check_contains_regex(
        "../serde_with/Cargo.toml",
        r#"(?m)^serde_with_macros = .*? version = "={version}""#,
    );
}

/// Check that all docs.rs links point to the current version
///
/// Parse all docs.rs links in `*.rs` and `*.md` files and check that they point to the current version.
/// If a link should point to latest version this can be done by using `latest` in the version.
/// The `*` version specifier is not allowed.
#[test]
fn test_docs_rs_url_point_to_current_version() -> Result<(), Box<dyn std::error::Error>> {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");

    let re = regex::Regex::new(&format!(
        "https?://docs.rs/{pkg_name}/((\\d[^/]+|\\*|latest))/"
    ))?;
    let mut error = false;

    for entry in glob::glob("**/*.rs")?.chain(glob::glob("**/README.md")?) {
        let entry = entry?;
        let content = std::fs::read_to_string(&entry)?;
        for (line_number, line) in content.split('\n').enumerate() {
            for capture in re.captures_iter(line) {
                match capture
                    .get(1)
                    .expect("Will exist if regex matches")
                    .as_str()
                {
                    "latest" => {}
                    version if version != pkg_version => {
                        error = true;
                        println!(
                            "{}:{} pkg_version is {} but found URL {}",
                            entry.display(),
                            line_number + 1,
                            pkg_version,
                            capture.get(0).expect("Group 0 always exists").as_str()
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    if error {
        panic!("Found wrong URLs in file(s)");
    } else {
        Ok(())
    }
}
