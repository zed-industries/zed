//! This module deals with everything related to path handling for Yarn, the package manager for Web ecosystem.
//! Yarn is a bit peculiar, because it references paths within .zip files, which we obviously can't handle.
//! It also uses virtual paths for peer dependencies.
//!
//! Long story short, before we attempt to resolve a path as a "real" path, we try to treat is as a yarn path;
//! for .zip handling, we unpack the contents into the temp directory (yes, this is bad, against the spirit of Yarn and what-not)

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use collections::HashMap;
use fs::Fs;
use gpui::{App, AppContext as _, Context, Entity, Task};
use util::{ResultExt, archive::extract_zip, paths::PathStyle, rel_path::RelPath};

pub(crate) struct YarnPathStore {
    temp_dirs: HashMap<Arc<Path>, tempfile::TempDir>,
    fs: Arc<dyn Fs>,
}

/// Returns `None` when passed path is a malformed virtual path or it's not a virtual path at all.
fn resolve_virtual(path: &Path) -> Option<Arc<Path>> {
    let components: Vec<_> = path.components().collect();
    let mut non_virtual_path = PathBuf::new();

    let mut i = 0;
    let mut is_virtual = false;
    while i < components.len() {
        if let Some(os_str) = components[i].as_os_str().to_str() {
            // Detect the __virtual__ segment
            if os_str == "__virtual__" {
                let pop_count = components
                    .get(i + 2)?
                    .as_os_str()
                    .to_str()?
                    .parse::<usize>()
                    .ok()?;

                // Apply dirname operation pop_count times
                for _ in 0..pop_count {
                    non_virtual_path.pop();
                }
                i += 3; // Skip hash and pop_count components
                is_virtual = true;
                continue;
            }
        }
        non_virtual_path.push(components[i]);
        i += 1;
    }

    is_virtual.then(|| Arc::from(non_virtual_path))
}

impl YarnPathStore {
    pub(crate) fn new(fs: Arc<dyn Fs>, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            temp_dirs: Default::default(),
            fs,
        })
    }

    pub(crate) fn process_path(
        &mut self,
        path: &Path,
        protocol: &str,
        cx: &Context<Self>,
    ) -> Task<Option<(Arc<Path>, Arc<RelPath>)>> {
        let mut is_zip = protocol.eq("zip");

        let path: &Path = if let Some(non_zip_part) = path
            .as_os_str()
            .as_encoded_bytes()
            .strip_prefix("/zip:".as_bytes())
        {
            // typescript-language-server prepends the paths with zip:, which is messy.
            is_zip = true;
            Path::new(OsStr::new(
                std::str::from_utf8(non_zip_part).expect("Invalid UTF-8"),
            ))
        } else {
            path
        };

        let as_virtual = resolve_virtual(path);
        let Some(path) = as_virtual.or_else(|| is_zip.then(|| Arc::from(path))) else {
            return Task::ready(None);
        };
        if let Some(zip_file) = zip_path(&path) {
            let zip_file: Arc<Path> = Arc::from(zip_file);
            cx.spawn(async move |this, cx| {
                let dir = this
                    .read_with(cx, |this, _| {
                        this.temp_dirs
                            .get(&zip_file)
                            .map(|temp| temp.path().to_owned())
                    })
                    .ok()?;
                let zip_root = if let Some(dir) = dir {
                    dir
                } else {
                    let fs = this.update(cx, |this, _| this.fs.clone()).ok()?;
                    let tempdir = dump_zip(zip_file.clone(), fs).await.log_err()?;
                    let new_path = tempdir.path().to_owned();
                    this.update(cx, |this, _| {
                        this.temp_dirs.insert(zip_file.clone(), tempdir);
                    })
                    .ok()?;
                    new_path
                };
                // Rebase zip-path onto new temp path.
                let as_relative =
                    RelPath::new(path.strip_prefix(zip_file).ok()?, PathStyle::local()).ok()?;
                Some((zip_root.into(), as_relative.into_arc()))
            })
        } else {
            Task::ready(None)
        }
    }
}

fn zip_path(path: &Path) -> Option<&Path> {
    let path_str = path.to_str()?;
    let zip_end = path_str.find(".zip/")?;
    let zip_path = &path_str[..zip_end + 4]; // ".zip" is 4 characters long
    Some(Path::new(zip_path))
}

async fn dump_zip(path: Arc<Path>, fs: Arc<dyn Fs>) -> Result<tempfile::TempDir> {
    let dir = tempfile::tempdir()?;
    let contents = fs.load_bytes(&path).await?;
    extract_zip(dir.path(), futures::io::Cursor::new(contents)).await?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_resolve_virtual() {
        let test_cases = vec![
            (
                "/path/to/some/folder/__virtual__/a0b1c2d3/0/subpath/to/file.dat",
                Some(Path::new("/path/to/some/folder/subpath/to/file.dat")),
            ),
            (
                "/path/to/some/folder/__virtual__/e4f5a0b1/0/subpath/to/file.dat",
                Some(Path::new("/path/to/some/folder/subpath/to/file.dat")),
            ),
            (
                "/path/to/some/folder/__virtual__/a0b1c2d3/1/subpath/to/file.dat",
                Some(Path::new("/path/to/some/subpath/to/file.dat")),
            ),
            (
                "/path/to/some/folder/__virtual__/a0b1c2d3/3/subpath/to/file.dat",
                Some(Path::new("/path/subpath/to/file.dat")),
            ),
            ("/path/to/nonvirtual/", None),
            ("/path/to/malformed/__virtual__", None),
            ("/path/to/malformed/__virtual__/a0b1c2d3", None),
            (
                "/path/to/malformed/__virtual__/a0b1c2d3/this-should-be-a-number",
                None,
            ),
        ];

        for (input, expected) in test_cases {
            let input_path = Path::new(input);
            let resolved_path = resolve_virtual(input_path);
            assert_eq!(resolved_path.as_deref(), expected);
        }
    }
}
