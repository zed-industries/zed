use crate::{Error, Manifest, Value};
use std::collections::HashSet;
use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};

/// This crate supports reading `Cargo.toml` not only from a real directory, but also directly from other sources, like tarballs or bare git repos (BYO directory reader).
///
/// The implementation must have a concept of the current directory, which is set to the crate's manifest dir.
pub trait AbstractFilesystem {
    /// List all files and directories at the given relative path (no leading `/`).
    fn file_names_in(&self, rel_path: &str) -> io::Result<HashSet<Box<str>>>;

    /// `parse_root_workspace` is preferred.
    ///
    /// The `rel_path_hint` may be specified explicitly by `package.workspace` (it may be relative like `"../"`, without `Cargo.toml`) or `None`,
    /// which means you have to search for workspace's `Cargo.toml` in parent directories.
    ///
    /// Read bytes of the root workspace manifest TOML file and return the path it's been read from.
    /// The path needs to be an absolute path, because it will be used as the base path for inherited readmes, and would be ambiguous otherwise.
    #[deprecated(note = "implement parse_root_workspace instead")]
    #[doc(hidden)]
    fn read_root_workspace(&self, _rel_path_hint: Option<&str>) -> io::Result<(Vec<u8>, PathBuf)> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "AbstractFilesystem::read_root_workspace unimplemented"))
    }

    /// The `rel_path_hint` may be specified explicitly by `package.workspace` (it may be relative like `"../"`, without `Cargo.toml`) or `None`,
    /// which means you have to search for workspace's `Cargo.toml` in parent directories.
    ///
    /// Read and parse the root workspace manifest TOML file and return the path it's been read from.
    /// The path needs to be an absolute path, because it will be used as the base path for inherited readmes, and would be ambiguous otherwise.
    #[allow(deprecated)]
    fn parse_root_workspace(&self, rel_path_hint: Option<&str>) -> Result<(Manifest<Value>, PathBuf), Error> {
        let (data, path) = self.read_root_workspace(rel_path_hint).map_err(|e| Error::Workspace(Box::new(e.into())))?;
        let manifest = Manifest::from_slice(&data).map_err(|e| Error::Workspace(Box::new(e)))?;
        if manifest.workspace.is_none() {
            return Err(Error::WorkspaceIntegrity(format!("Manifest at {} was expected to be a workspace.\nUse package.workspace to select a differnt path, or implement cargo_toml::AbstractFilesystem::parse_root_workspace", path.display())));
        }
        Ok((manifest, path))
    }
}

impl<T> AbstractFilesystem for &T
where
    T: AbstractFilesystem + ?Sized,
{
    fn file_names_in(&self, rel_path: &str) -> io::Result<HashSet<Box<str>>> {
        <T as AbstractFilesystem>::file_names_in(*self, rel_path)
    }

    #[allow(deprecated)]
    fn read_root_workspace(&self, rel_path_hint: Option<&str>) -> io::Result<(Vec<u8>, PathBuf)> {
        <T as AbstractFilesystem>::read_root_workspace(*self, rel_path_hint)
    }

    fn parse_root_workspace(&self, rel_path_hint: Option<&str>) -> Result<(Manifest<Value>, PathBuf), Error> {
        <T as AbstractFilesystem>::parse_root_workspace(*self, rel_path_hint)
    }
}

/// [`AbstractFilesystem`] implementation for real files.
pub struct Filesystem<'a> {
    path: &'a Path,
}

impl<'a> Filesystem<'a> {
    #[must_use]
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
}

impl<'a> AbstractFilesystem for Filesystem<'a> {
    fn file_names_in(&self, rel_path: &str) -> io::Result<HashSet<Box<str>>> {
        Ok(read_dir(self.path.join(rel_path))?.filter_map(|entry| {
            entry.ok().map(|e| {
                e.file_name().to_string_lossy().into_owned().into()
            })
        })
        .collect())
    }

    fn parse_root_workspace(&self, path: Option<&str>) -> Result<(Manifest<Value>, PathBuf), Error> {
        match path {
            Some(path) => {
                let ws = self.path.join(path);
                let toml_path = ws.join("Cargo.toml");
                let data = std::fs::read(&toml_path)
                    .map_err(|e| Error::Workspace(Box::new(Error::Io(e))))?;
                Ok((parse_workspace(&data, &toml_path)?, ws))
            },
            None => {
                // Try relative path first
                match find_workspace(self.path) {
                    Ok(found) => Ok(found),
                    Err(err) if self.path.is_absolute() => Err(err),
                    Err(_) => find_workspace(&self.path.ancestors().last().unwrap().canonicalize()?),
                }
            },
        }
    }
}

#[inline(never)]
fn find_workspace(path: &Path) -> Result<(Manifest<Value>, PathBuf), Error> {
    if path.parent().is_none() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Can't find workspace in '{}', because it has no parent directories", path.display())).into())
    }
    let mut last_error = None;
    path.ancestors().skip(1)
        .map(|parent| parent.join("Cargo.toml"))
        .find_map(|p| {
            let data = std::fs::read(&p).ok()?;
            match parse_workspace(&data, &p) {
                Ok(manifest) => Some((manifest, p)),
                Err(e) => {
                    last_error = Some(e);
                    None
                },
            }
        })
        .ok_or(last_error.unwrap_or_else(|| {
            let has_slash = path.to_str().is_some_and(|s| s.ends_with('/'));
            io::Error::new(io::ErrorKind::NotFound, format!("Can't find workspace in '{}{}..'", path.display(), if has_slash {""} else {"/"})).into()
        }))
}

#[inline(never)]
fn parse_workspace(data: &[u8], path: &Path) -> Result<Manifest<Value>, Error> {
    let manifest = Manifest::from_slice(data)?;
    if manifest.workspace.is_none() {
        return Err(Error::WorkspaceIntegrity(format!("Manifest at {} was expected to be a workspace.", path.display())));
    }
    Ok(manifest)
}
