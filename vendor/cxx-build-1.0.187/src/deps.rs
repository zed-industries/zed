use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Default)]
pub(crate) struct Crate {
    pub include_prefix: Option<PathBuf>,
    pub links: Option<OsString>,
    pub header_dirs: Vec<HeaderDir>,
}

pub(crate) struct HeaderDir {
    pub exported: bool,
    pub path: PathBuf,
}

impl Crate {
    pub(crate) fn print_to_cargo(&self) {
        if let Some(include_prefix) = &self.include_prefix {
            println!(
                "cargo:CXXBRIDGE_PREFIX={}",
                include_prefix.to_string_lossy(),
            );
        }
        if let Some(links) = &self.links {
            println!("cargo:CXXBRIDGE_LINKS={}", links.to_string_lossy());
        }
        for (i, header_dir) in self.header_dirs.iter().enumerate() {
            if header_dir.exported {
                println!(
                    "cargo:CXXBRIDGE_DIR{}={}",
                    i,
                    header_dir.path.to_string_lossy(),
                );
            }
        }
    }
}

pub(crate) fn direct_dependencies() -> Vec<Crate> {
    let mut crates: BTreeMap<String, Crate> = BTreeMap::new();
    let mut exported_header_dirs: BTreeMap<String, Vec<(usize, PathBuf)>> = BTreeMap::new();

    // Only variables set from a build script of direct dependencies are
    // observable. That's exactly what we want! Your crate needs to declare a
    // direct dependency on the other crate in order to be able to #include its
    // headers.
    //
    // Also, they're only observable if the dependency's manifest contains a
    // `links` key. This is important because Cargo imposes no ordering on the
    // execution of build scripts without a `links` key. When exposing a
    // generated header for the current crate to #include, we need to be sure
    // the dependency's build script has already executed and emitted that
    // generated header.
    //
    // References:
    //   - https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
    //   - https://doc.rust-lang.org/cargo/reference/build-script-examples.html#using-another-sys-crate
    for (k, v) in env::vars_os() {
        let mut k = k.to_string_lossy().into_owned();
        if !k.starts_with("DEP_") {
            continue;
        }

        if k.ends_with("_CXXBRIDGE_PREFIX") {
            k.truncate(k.len() - "_CXXBRIDGE_PREFIX".len());
            crates.entry(k).or_default().include_prefix = Some(PathBuf::from(v));
            continue;
        }

        if k.ends_with("_CXXBRIDGE_LINKS") {
            k.truncate(k.len() - "_CXXBRIDGE_LINKS".len());
            crates.entry(k).or_default().links = Some(v);
            continue;
        }

        let without_counter = k.trim_end_matches(|ch: char| ch.is_ascii_digit());
        let counter_len = k.len() - without_counter.len();
        if counter_len == 0 || !without_counter.ends_with("_CXXBRIDGE_DIR") {
            continue;
        }

        let sort_key = k[k.len() - counter_len..]
            .parse::<usize>()
            .unwrap_or(usize::MAX);
        k.truncate(k.len() - counter_len - "_CXXBRIDGE_DIR".len());
        exported_header_dirs
            .entry(k)
            .or_default()
            .push((sort_key, PathBuf::from(v)));
    }

    for (k, mut dirs) in exported_header_dirs {
        dirs.sort_by_key(|(sort_key, _dir)| *sort_key);
        crates
            .entry(k)
            .or_default()
            .header_dirs
            .extend(dirs.into_iter().map(|(_sort_key, dir)| HeaderDir {
                exported: true,
                path: dir,
            }));
    }

    crates.into_iter().map(|entry| entry.1).collect()
}
