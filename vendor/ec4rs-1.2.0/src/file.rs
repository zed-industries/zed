use std::path::{Path, PathBuf};

use crate::{ConfigParser, Error, ParseError, Properties, PropertiesSource, Section};

/// Convenience wrapper for an [`ConfigParser`] that reads files.
pub struct ConfigFile {
    // TODO: Arc<Path>. It's more important to have cheap clones than mutability.
    /// The path to the open file.
    pub path: PathBuf,
    /// A [`ConfigParser`] that reads from the file.
    pub reader: ConfigParser<std::io::BufReader<std::fs::File>>,
}

impl ConfigFile {
    /// Opens a file for reading and uses it to construct an [`ConfigParser`].
    ///
    /// If the file cannot be opened, wraps the [`std::io::Error`] in a [`ParseError`].
    pub fn open(path: impl Into<PathBuf>) -> Result<ConfigFile, ParseError> {
        let path = path.into();
        let file = std::fs::File::open(&path).map_err(ParseError::Io)?;
        let reader = ConfigParser::new_buffered_with_path(file, Some(path.as_ref()))?;
        Ok(ConfigFile { path, reader })
    }

    /// Wraps a [`ParseError`] in an [`Error::InFile`].
    ///
    /// Uses the path and current line number from this instance.
    pub fn add_error_context(&self, error: ParseError) -> Error {
        Error::InFile(self.path.clone(), self.reader.line_no(), error)
    }
}

impl Iterator for ConfigFile {
    type Item = Result<Section, ParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next()
    }
}

impl std::iter::FusedIterator for ConfigFile {}

impl PropertiesSource for &mut ConfigFile {
    /// Adds properties from the file's sections to the specified [`Properties`] map.
    ///
    /// Uses [`ConfigFile::path`] when determining applicability to stop `**` from going too far.
    /// Returns parse errors wrapped in an [`Error::InFile`].
    fn apply_to(self, props: &mut Properties, path: impl AsRef<Path>) -> Result<(), crate::Error> {
        let get_parent = || self.path.parent();
        let path = if let Some(parent) = get_parent() {
            let path = path.as_ref();
            path.strip_prefix(parent).unwrap_or(path)
        } else {
            path.as_ref()
        };
        match self.reader.apply_to(props, path) {
            Ok(()) => Ok(()),
            Err(crate::Error::Parse(e)) => Err(self.add_error_context(e)),
            Err(e) => panic!("unexpected error variant {:?}", e),
        }
    }
}

/// Directory traverser for finding and opening EditorConfig files.
///
/// All the contained files are open for reading and have not had any sections read.
/// When iterated over, either by using it as an [`Iterator`]
/// or by calling [`ConfigFiles::iter`],
/// returns [`ConfigFile`]s in the order that they would apply to a [`Properties`] map.
pub struct ConfigFiles(Vec<ConfigFile>);

impl ConfigFiles {
    /// Searches for EditorConfig files that might apply to a file at the specified path.
    ///
    /// This function does not canonicalize the path,
    /// but will join relative paths onto the current working directory.
    ///
    /// EditorConfig files are assumed to be named `.editorconfig`
    /// unless an override is supplied as the second argument.
    #[allow(clippy::needless_pass_by_value)]
    pub fn open(
        path: impl AsRef<Path>,
        config_path_override: Option<impl AsRef<std::path::Path>>,
    ) -> Result<ConfigFiles, Error> {
        use std::borrow::Cow;
        let filename = config_path_override
            .as_ref()
            .map_or_else(|| ".editorconfig".as_ref(), |f| f.as_ref());
        Ok(ConfigFiles(if filename.is_relative() {
            let mut abs_path = Cow::from(path.as_ref());
            if abs_path.is_relative() {
                abs_path = std::env::current_dir()
                    .map_err(Error::InvalidCwd)?
                    .join(&path)
                    .into()
            }
            let mut path = abs_path.as_ref();
            let mut vec = Vec::new();
            while let Some(dir) = path.parent() {
                if let Ok(file) = ConfigFile::open(dir.join(filename)) {
                    let should_break = file.reader.is_root;
                    vec.push(file);
                    if should_break {
                        break;
                    }
                }
                path = dir;
            }
            vec
        } else {
            // TODO: Better errors.
            vec![ConfigFile::open(filename).map_err(Error::Parse)?]
        }))
    }

    /// Returns an iterator over the contained [`ConfigFiles`].
    pub fn iter(&self) -> impl Iterator<Item = &ConfigFile> {
        self.0.iter().rev()
    }

    // To maintain the invariant that these files have not had any sections read,
    // there is no `iter_mut` method.
}

impl Iterator for ConfigFiles {
    type Item = ConfigFile;
    fn next(&mut self) -> Option<ConfigFile> {
        self.0.pop()
    }
}

impl std::iter::FusedIterator for ConfigFiles {}

impl PropertiesSource for ConfigFiles {
    /// Adds properties from the files' sections to the specified [`Properties`] map.
    ///
    /// Ignores the files' paths when determining applicability.
    fn apply_to(self, props: &mut Properties, path: impl AsRef<Path>) -> Result<(), crate::Error> {
        let path = path.as_ref();
        for mut file in self {
            file.apply_to(props, path)?;
        }
        Ok(())
    }
}
