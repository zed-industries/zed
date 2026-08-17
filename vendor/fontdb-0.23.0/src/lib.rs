/*!
`fontdb` is a simple, in-memory font database with CSS-like queries.

# Features

- The database can load fonts from files, directories and raw data (`Vec<u8>`).
- The database can match a font using CSS-like queries. See `Database::query`.
- The database can try to load system fonts.
  Currently, this is implemented by scanning predefined directories.
  The library does not interact with the system API.
- Provides a unique ID for each font face.

# Non-goals

- Advanced font properties querying.<br>
  The database provides only storage and matching capabilities.
  For font properties querying you can use [ttf-parser].

- A font fallback mechanism.<br>
  This library can be used to implement a font fallback mechanism, but it doesn't implement one.

- Application's global database.<br>
  The database doesn't use `static`, therefore it's up to the caller where it should be stored.

- Font types support other than TrueType.

# Font vs Face

A font is a collection of font faces. Therefore, a font face is a subset of a font.
A simple font (\*.ttf/\*.otf) usually contains a single font face,
but a font collection (\*.ttc) can contain multiple font faces.

`fontdb` stores and matches font faces, not fonts.
Therefore, after loading a font collection with 5 faces (for example), the database will be populated
with 5 `FaceInfo` objects, all of which will be pointing to the same file or binary data.

# Performance

The database performance is largely limited by the storage itself.
We are using [ttf-parser], so the parsing should not be a bottleneck.

On my machine with Samsung SSD 860 and Gentoo Linux, it takes ~20ms
to load 1906 font faces (most of them are from Google Noto collection)
with a hot disk cache and ~860ms with a cold one.

On Mac Mini M1 it takes just 9ms to load 898 fonts.

# Safety

The library relies on memory-mapped files, which is inherently unsafe.
But since we do not keep the files open it should be perfectly safe.

If you would like to use a persistent memory mapping of the font files,
then you can use the unsafe [`Database::make_shared_face_data`] function.

[ttf-parser]: https://github.com/RazrFalcon/ttf-parser
*/

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

pub use ttf_parser::Language;
pub use ttf_parser::Width as Stretch;

use slotmap::SlotMap;
use tinyvec::TinyVec;

/// A unique per database face ID.
///
/// Since `Database` is not global/unique, we cannot guarantee that a specific ID
/// is actually from the same db instance. This is up to the caller.
///
/// ID overflow will cause a panic, but it's highly unlikely that someone would
/// load more than 4 billion font faces.
///
/// Because the internal representation of ID is private, The `Display` trait
/// implementation for this type only promise that unequal IDs will be displayed
/// as different strings, but does not make any guarantees about format or
/// content of the strings.
///
/// [`KeyData`]: https://docs.rs/slotmap/latest/slotmap/struct.KeyData.html
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Debug, Default)]
pub struct ID(InnerId);

slotmap::new_key_type! {
    /// Internal ID type.
    struct InnerId;
}

impl ID {
    /// Creates a dummy ID.
    ///
    /// Should be used in tandem with [`Database::push_face_info`].
    #[inline]
    pub fn dummy() -> Self {
        Self(InnerId::from(slotmap::KeyData::from_ffi(core::u64::MAX)))
    }
}

impl core::fmt::Display for ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", (self.0).0.as_ffi())
    }
}

/// A list of possible font loading errors.
#[derive(Debug)]
enum LoadError {
    /// A malformed font.
    ///
    /// Typically means that [ttf-parser](https://github.com/RazrFalcon/ttf-parser)
    /// wasn't able to parse it.
    MalformedFont,
    /// A valid TrueType font without a valid *Family Name*.
    UnnamedFont,
    /// A file IO related error.
    #[cfg(feature = "std")]
    IoError(std::io::Error),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for LoadError {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        LoadError::IoError(e)
    }
}

impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoadError::MalformedFont => write!(f, "malformed font"),
            LoadError::UnnamedFont => write!(f, "font doesn't have a family name"),
            #[cfg(feature = "std")]
            LoadError::IoError(ref e) => write!(f, "{}", e),
        }
    }
}

/// A font database.
#[derive(Clone, Debug)]
pub struct Database {
    faces: SlotMap<InnerId, FaceInfo>,
    family_serif: String,
    family_sans_serif: String,
    family_cursive: String,
    family_fantasy: String,
    family_monospace: String,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    /// Create a new, empty `Database`.
    ///
    /// Generic font families would be set to:
    ///
    /// - `serif` - Times New Roman
    /// - `sans-serif` - Arial
    /// - `cursive` - Comic Sans MS
    /// - `fantasy` - Impact (Papyrus on macOS)
    /// - `monospace` - Courier New
    #[inline]
    pub fn new() -> Self {
        Database {
            faces: SlotMap::with_key(),
            family_serif: "Times New Roman".to_string(),
            family_sans_serif: "Arial".to_string(),
            family_cursive: "Comic Sans MS".to_string(),
            #[cfg(not(target_os = "macos"))]
            family_fantasy: "Impact".to_string(),
            #[cfg(target_os = "macos")]
            family_fantasy: "Papyrus".to_string(),
            family_monospace: "Courier New".to_string(),
        }
    }

    /// Loads a font data into the `Database`.
    ///
    /// Will load all font faces in case of a font collection.
    pub fn load_font_data(&mut self, data: Vec<u8>) {
        self.load_font_source(Source::Binary(alloc::sync::Arc::new(data)));
    }

    /// Loads a font from the given source into the `Database` and returns
    /// the ID of the loaded font.
    ///
    /// Will load all font faces in case of a font collection.
    pub fn load_font_source(&mut self, source: Source) -> TinyVec<[ID; 8]> {
        let ids = source.with_data(|data| {
            let n = ttf_parser::fonts_in_collection(data).unwrap_or(1);
            let mut ids = TinyVec::with_capacity(n as usize);

            for index in 0..n {
                match parse_face_info(source.clone(), data, index) {
                    Ok(mut info) => {
                        let id = self.faces.insert_with_key(|k| {
                            info.id = ID(k);
                            info
                        });
                        ids.push(ID(id));
                    }
                    Err(e) => log::warn!(
                        "Failed to load a font face {} from source cause {}.",
                        index,
                        e
                    ),
                }
            }

            ids
        });

        ids.unwrap_or_default()
    }

    /// Backend function used by load_font_file to load font files.
    #[cfg(feature = "fs")]
    fn load_fonts_from_file(&mut self, path: &std::path::Path, data: &[u8]) {
        let source = Source::File(path.into());

        let n = ttf_parser::fonts_in_collection(data).unwrap_or(1);
        for index in 0..n {
            match parse_face_info(source.clone(), data, index) {
                Ok(info) => {
                    self.push_face_info(info);
                }
                Err(e) => {
                    log::warn!(
                        "Failed to load a font face {} from '{}' cause {}.",
                        index,
                        path.display(),
                        e
                    )
                }
            }
        }
    }

    /// Loads a font file into the `Database`.
    ///
    /// Will load all font faces in case of a font collection.
    #[cfg(all(feature = "fs", feature = "memmap"))]
    pub fn load_font_file<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<(), std::io::Error> {
        self.load_font_file_impl(path.as_ref())
    }

    // A non-generic version.
    #[cfg(all(feature = "fs", feature = "memmap"))]
    fn load_font_file_impl(&mut self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let file = std::fs::File::open(path)?;
        let data: &[u8] = unsafe { &memmap2::MmapOptions::new().map(&file)? };

        self.load_fonts_from_file(path, data);
        Ok(())
    }

    /// Loads a font file into the `Database`.
    ///
    /// Will load all font faces in case of a font collection.
    #[cfg(all(feature = "fs", not(feature = "memmap")))]
    pub fn load_font_file<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<(), std::io::Error> {
        self.load_font_file_impl(path.as_ref())
    }

    // A non-generic version.
    #[cfg(all(feature = "fs", not(feature = "memmap")))]
    fn load_font_file_impl(&mut self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let data = std::fs::read(path)?;

        self.load_fonts_from_file(path, &data);
        Ok(())
    }

    /// Loads font files from the selected directory into the `Database`.
    ///
    /// This method will scan directories recursively.
    ///
    /// Will load `ttf`, `otf`, `ttc` and `otc` fonts.
    ///
    /// Unlike other `load_*` methods, this one doesn't return an error.
    /// It will simply skip malformed fonts and will print a warning into the log for each of them.
    #[cfg(feature = "fs")]
    pub fn load_fonts_dir<P: AsRef<std::path::Path>>(&mut self, dir: P) {
        self.load_fonts_dir_impl(dir.as_ref(), &mut Default::default())
    }

    #[cfg(feature = "fs")]
    fn canonicalize(
        &self,
        path: std::path::PathBuf,
        entry: std::fs::DirEntry,
        seen: &mut std::collections::HashSet<std::path::PathBuf>,
    ) -> Option<(std::path::PathBuf, std::fs::FileType)> {
        let file_type = entry.file_type().ok()?;
        if !file_type.is_symlink() {
            if !seen.is_empty() {
                if seen.contains(&path) {
                    return None;
                }
                seen.insert(path.clone());
            }

            return Some((path, file_type));
        }

        if seen.is_empty() && file_type.is_dir() {
            seen.reserve(8192 / std::mem::size_of::<std::path::PathBuf>());

            for (_, info) in self.faces.iter() {
                let path = match &info.source {
                    Source::Binary(_) => continue,
                    Source::File(path) => path.to_path_buf(),
                    #[cfg(feature = "memmap")]
                    Source::SharedFile(path, _) => path.to_path_buf(),
                };
                seen.insert(path);
            }
        }

        let stat = std::fs::metadata(&path).ok()?;
        if stat.is_symlink() {
            return None;
        }

        let canon = std::fs::canonicalize(path).ok()?;
        if seen.contains(&canon) {
            return None;
        }
        seen.insert(canon.clone());
        Some((canon, stat.file_type()))
    }

    // A non-generic version.
    #[cfg(feature = "fs")]
    fn load_fonts_dir_impl(
        &mut self,
        dir: &std::path::Path,
        seen: &mut std::collections::HashSet<std::path::PathBuf>,
    ) {
        let fonts_dir = match std::fs::read_dir(dir) {
            Ok(dir) => dir,
            Err(_) => return,
        };

        for entry in fonts_dir.flatten() {
            let (path, file_type) = match self.canonicalize(entry.path(), entry, seen) {
                Some(v) => v,
                None => continue,
            };

            if file_type.is_file() {
                match path.extension().and_then(|e| e.to_str()) {
                    #[rustfmt::skip] // keep extensions match as is
                    Some("ttf") | Some("ttc") | Some("TTF") | Some("TTC") |
                    Some("otf") | Some("otc") | Some("OTF") | Some("OTC") => {
                        if let Err(e) = self.load_font_file(&path) {
                            log::warn!("Failed to load '{}' cause {}.", path.display(), e);
                        }
                    },
                    _ => {}
                }
            } else if file_type.is_dir() {
                self.load_fonts_dir_impl(&path, seen);
            }
        }
    }

    /// Attempts to load system fonts.
    ///
    /// Supports Windows, Linux and macOS.
    ///
    /// System fonts loading is a surprisingly complicated task,
    /// mostly unsolvable without interacting with system libraries.
    /// And since `fontdb` tries to be small and portable, this method
    /// will simply scan some predefined directories.
    /// Which means that fonts that are not in those directories must
    /// be added manually.
    #[cfg(feature = "fs")]
    pub fn load_system_fonts(&mut self) {
        #[cfg(target_os = "windows")]
        {
            let mut seen = Default::default();
            if let Some(ref system_root) = std::env::var_os("SYSTEMROOT") {
                let system_root_path = std::path::Path::new(system_root);
                self.load_fonts_dir_impl(&system_root_path.join("Fonts"), &mut seen);
            } else {
                self.load_fonts_dir_impl("C:\\Windows\\Fonts\\".as_ref(), &mut seen);
            }

            if let Ok(ref home) = std::env::var("USERPROFILE") {
                let home_path = std::path::Path::new(home);
                self.load_fonts_dir_impl(
                    &home_path.join("AppData\\Local\\Microsoft\\Windows\\Fonts"),
                    &mut seen,
                );
                self.load_fonts_dir_impl(
                    &home_path.join("AppData\\Roaming\\Microsoft\\Windows\\Fonts"),
                    &mut seen,
                );
            }
        }

        #[cfg(target_os = "macos")]
        {
            let mut seen = Default::default();
            self.load_fonts_dir_impl("/Library/Fonts".as_ref(), &mut seen);
            self.load_fonts_dir_impl("/System/Library/Fonts".as_ref(), &mut seen);
            // Downloadable fonts, location varies on major macOS releases
            if let Ok(dir) = std::fs::read_dir("/System/Library/AssetsV2") {
                for entry in dir {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(_) => continue,
                    };
                    if entry
                        .file_name()
                        .to_string_lossy()
                        .starts_with("com_apple_MobileAsset_Font")
                    {
                        self.load_fonts_dir_impl(&entry.path(), &mut seen);
                    }
                }
            }
            self.load_fonts_dir_impl("/Network/Library/Fonts".as_ref(), &mut seen);

            if let Ok(ref home) = std::env::var("HOME") {
                let home_path = std::path::Path::new(home);
                self.load_fonts_dir_impl(&home_path.join("Library/Fonts"), &mut seen);
            }
        }

        // Redox OS.
        #[cfg(target_os = "redox")]
        {
            let mut seen = Default::default();
            self.load_fonts_dir_impl("/ui/fonts".as_ref(), &mut seen);
        }

        // Linux.
        #[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
        {
            #[cfg(feature = "fontconfig")]
            {
                if !self.load_fontconfig() {
                    log::warn!("Fallback to loading from known font dir paths.");
                    self.load_no_fontconfig();
                }
            }

            #[cfg(not(feature = "fontconfig"))]
            {
                self.load_no_fontconfig();
            }
        }
    }


    // Linux.
    #[cfg(all(
        unix,
        feature = "fs",
        not(any(target_os = "macos", target_os = "android"))
    ))]
    fn load_no_fontconfig(&mut self) {
        let mut seen = Default::default();
        self.load_fonts_dir_impl("/usr/share/fonts/".as_ref(), &mut seen);
        self.load_fonts_dir_impl("/usr/local/share/fonts/".as_ref(), &mut seen);

        if let Ok(ref home) = std::env::var("HOME") {
            let home_path = std::path::Path::new(home);
            self.load_fonts_dir_impl(&home_path.join(".fonts"), &mut seen);
            self.load_fonts_dir_impl(&home_path.join(".local/share/fonts"), &mut seen);
        }
    }

    // Linux.
    #[cfg(all(
        unix,
        feature = "fontconfig",
        not(any(target_os = "macos", target_os = "android"))
    ))]
    fn load_fontconfig(&mut self) -> bool {
        use std::path::Path;

        let mut fontconfig = fontconfig_parser::FontConfig::default();
        let home = std::env::var("HOME");

        if let Ok(ref config_file) = std::env::var("FONTCONFIG_FILE") {
            let _ = fontconfig.merge_config(Path::new(config_file));
        } else {
            let xdg_config_home = if let Ok(val) = std::env::var("XDG_CONFIG_HOME") {
                Some(val.into())
            } else if let Ok(ref home) = home {
                // according to https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
                // $XDG_CONFIG_HOME should default to $HOME/.config if not set
                Some(Path::new(home).join(".config"))
            } else {
                None
            };

            let read_global = match xdg_config_home {
                Some(p) => fontconfig
                    .merge_config(&p.join("fontconfig/fonts.conf"))
                    .is_err(),
                None => true,
            };

            if read_global {
                let _ = fontconfig.merge_config(Path::new("/etc/fonts/local.conf"));
            }
            let _ = fontconfig.merge_config(Path::new("/etc/fonts/fonts.conf"));
        }

        for fontconfig_parser::Alias {
            alias,
            default,
            prefer,
            accept,
        } in fontconfig.aliases
        {
            let name = prefer
                .get(0)
                .or_else(|| accept.get(0))
                .or_else(|| default.get(0));

            if let Some(name) = name {
                match alias.to_lowercase().as_str() {
                    "serif" => self.set_serif_family(name),
                    "sans-serif" => self.set_sans_serif_family(name),
                    "sans serif" => self.set_sans_serif_family(name),
                    "monospace" => self.set_monospace_family(name),
                    "cursive" => self.set_cursive_family(name),
                    "fantasy" => self.set_fantasy_family(name),
                    _ => {}
                }
            }
        }

        if fontconfig.dirs.is_empty() {
            return false;
        }

        let mut seen = Default::default();
        for dir in fontconfig.dirs {
            let path = if dir.path.starts_with("~") {
                if let Ok(ref home) = home {
                    Path::new(home).join(dir.path.strip_prefix("~").unwrap())
                } else {
                    continue;
                }
            } else {
                dir.path
            };
            self.load_fonts_dir_impl(&path, &mut seen);
        }

        true
    }

    /// Pushes a user-provided `FaceInfo` to the database.
    ///
    /// In some cases, a caller might want to ignore the font's metadata and provide their own.
    /// This method doesn't parse the `source` font.
    ///
    /// The `id` field should be set to [`ID::dummy()`] and will be then overwritten by this method.
    pub fn push_face_info(&mut self, mut info: FaceInfo) -> ID {
        ID(self.faces.insert_with_key(|k| {
            info.id = ID(k);
            info
        }))
    }

    /// Removes a font face by `id` from the database.
    ///
    /// Returns `false` while attempting to remove a non-existing font face.
    ///
    /// Useful when you want to ignore some specific font face(s)
    /// after loading a large directory with fonts.
    /// Or a specific face from a font.
    pub fn remove_face(&mut self, id: ID) {
        self.faces.remove(id.0);
    }

    /// Returns `true` if the `Database` contains no font faces.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.faces.is_empty()
    }

    /// Returns the number of font faces in the `Database`.
    ///
    /// Note that `Database` stores font faces, not fonts.
    /// For example, if a caller will try to load a font collection (`*.ttc`) that contains 5 faces,
    /// then the `Database` will load 5 font faces and this method will return 5, not 1.
    #[inline]
    pub fn len(&self) -> usize {
        self.faces.len()
    }

    /// Sets the family that will be used by `Family::Serif`.
    pub fn set_serif_family<S: Into<String>>(&mut self, family: S) {
        self.family_serif = family.into();
    }

    /// Sets the family that will be used by `Family::SansSerif`.
    pub fn set_sans_serif_family<S: Into<String>>(&mut self, family: S) {
        self.family_sans_serif = family.into();
    }

    /// Sets the family that will be used by `Family::Cursive`.
    pub fn set_cursive_family<S: Into<String>>(&mut self, family: S) {
        self.family_cursive = family.into();
    }

    /// Sets the family that will be used by `Family::Fantasy`.
    pub fn set_fantasy_family<S: Into<String>>(&mut self, family: S) {
        self.family_fantasy = family.into();
    }

    /// Sets the family that will be used by `Family::Monospace`.
    pub fn set_monospace_family<S: Into<String>>(&mut self, family: S) {
        self.family_monospace = family.into();
    }

    /// Returns the generic family name or the `Family::Name` itself.
    ///
    /// Generic family names should be set via `Database::set_*_family` methods.
    pub fn family_name<'a>(&'a self, family: &'a Family) -> &'a str {
        match family {
            Family::Name(name) => name,
            Family::Serif => self.family_serif.as_str(),
            Family::SansSerif => self.family_sans_serif.as_str(),
            Family::Cursive => self.family_cursive.as_str(),
            Family::Fantasy => self.family_fantasy.as_str(),
            Family::Monospace => self.family_monospace.as_str(),
        }
    }

    /// Performs a CSS-like query and returns the best matched font face.
    pub fn query(&self, query: &Query) -> Option<ID> {
        for family in query.families {
            let name = self.family_name(family);
            let candidates: Vec<_> = self
                .faces
                .iter()
                .filter(|(_, face)| face.families.iter().any(|family| family.0 == name))
                .map(|(_, info)| info)
                .collect();

            if !candidates.is_empty() {
                if let Some(index) = find_best_match(&candidates, query) {
                    return Some(candidates[index].id);
                }
            }
        }

        None
    }

    /// Returns an iterator over the internal storage.
    ///
    /// This can be used for manual font matching.
    #[inline]
    pub fn faces(&self) -> impl Iterator<Item = &FaceInfo> + '_ {
        self.faces.iter().map(|(_, info)| info)
    }

    /// Selects a `FaceInfo` by `id`.
    ///
    /// Returns `None` if a face with such ID was already removed,
    /// or this ID belong to the other `Database`.
    pub fn face(&self, id: ID) -> Option<&FaceInfo> {
        self.faces.get(id.0)
    }

    /// Returns font face storage and the face index by `ID`.
    pub fn face_source(&self, id: ID) -> Option<(Source, u32)> {
        self.face(id).map(|info| (info.source.clone(), info.index))
    }

    /// Executes a closure with a font's data.
    ///
    /// We can't return a reference to a font binary data because of lifetimes.
    /// So instead, you can use this method to process font's data.
    ///
    /// The closure accepts raw font data and font face index.
    ///
    /// In case of `Source::File`, the font file will be memory mapped.
    ///
    /// Returns `None` when font file loading failed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let is_variable = db.with_face_data(id, |font_data, face_index| {
    ///     let font = ttf_parser::Face::from_slice(font_data, face_index).unwrap();
    ///     font.is_variable()
    /// })?;
    /// ```
    pub fn with_face_data<P, T>(&self, id: ID, p: P) -> Option<T>
    where
        P: FnOnce(&[u8], u32) -> T,
    {
        let (src, face_index) = self.face_source(id)?;
        src.with_data(|data| p(data, face_index))
    }

    /// Makes the font data that backs the specified face id shared so that the application can
    /// hold a reference to it.
    ///
    /// # Safety
    ///
    /// If the face originates from a file from disk, then the file is mapped from disk. This is unsafe as
    /// another process may make changes to the file on disk, which may become visible in this process'
    /// mapping and possibly cause crashes.
    ///
    /// If the underlying font provides multiple faces, then all faces are updated to participate in
    /// the data sharing. If the face was previously marked for data sharing, then this function will
    /// return a clone of the existing reference.
    #[cfg(all(feature = "fs", feature = "memmap"))]
    pub unsafe fn make_shared_face_data(
        &mut self,
        id: ID,
    ) -> Option<(std::sync::Arc<dyn AsRef<[u8]> + Send + Sync>, u32)> {
        let face_info = self.faces.get(id.0)?;
        let face_index = face_info.index;

        let old_source = face_info.source.clone();

        let (path, shared_data) = match &old_source {
            Source::Binary(data) => {
                return Some((data.clone(), face_index));
            }
            Source::File(ref path) => {
                let file = std::fs::File::open(path).ok()?;
                let shared_data = std::sync::Arc::new(memmap2::MmapOptions::new().map(&file).ok()?)
                    as std::sync::Arc<dyn AsRef<[u8]> + Send + Sync>;
                (path.clone(), shared_data)
            }
            Source::SharedFile(_, data) => {
                return Some((data.clone(), face_index));
            }
        };

        let shared_source = Source::SharedFile(path.clone(), shared_data.clone());

        self.faces.iter_mut().for_each(|(_, face)| {
            if matches!(&face.source, Source::File(old_path) if old_path == &path) {
                face.source = shared_source.clone();
            }
        });

        Some((shared_data, face_index))
    }

    /// Transfers ownership of shared font data back to the font database. This is the reverse operation
    /// of [`Self::make_shared_face_data`]. If the font data belonging to the specified face is mapped
    /// from a file on disk, then that mapping is closed and the data becomes private to the process again.
    #[cfg(all(feature = "fs", feature = "memmap"))]
    pub fn make_face_data_unshared(&mut self, id: ID) {
        let face_info = match self.faces.get(id.0) {
            Some(face_info) => face_info,
            None => return,
        };

        let old_source = face_info.source.clone();

        let shared_path = match old_source {
            #[cfg(all(feature = "fs", feature = "memmap"))]
            Source::SharedFile(path, _) => path,
            _ => return,
        };

        let new_source = Source::File(shared_path.clone());

        self.faces.iter_mut().for_each(|(_, face)| {
            if matches!(&face.source, Source::SharedFile(path, ..) if path == &shared_path) {
                face.source = new_source.clone();
            }
        });
    }
}

/// A single font face info.
///
/// A font can have multiple faces.
///
/// A single item of the `Database`.
#[derive(Clone, Debug)]
pub struct FaceInfo {
    /// An unique ID.
    pub id: ID,

    /// A font source.
    ///
    /// Note that multiple `FaceInfo` objects can reference the same data in case of
    /// font collections, which means that they'll use the same Source.
    pub source: Source,

    /// A face index in the `source`.
    pub index: u32,

    /// A list of family names.
    ///
    /// Contains pairs of Name + Language. Where the first family is always English US,
    /// unless it's missing from the font.
    ///
    /// Corresponds to a *Typographic Family* (ID 16) or a *Font Family* (ID 1) [name ID]
    /// in a TrueType font.
    ///
    /// This is not an *Extended Typographic Family* or a *Full Name*.
    /// Meaning it will contain _Arial_ and not _Arial Bold_.
    ///
    /// [name ID]: https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids
    pub families: Vec<(String, Language)>,

    /// A PostScript name.
    ///
    /// Corresponds to a *PostScript name* (6) [name ID] in a TrueType font.
    ///
    /// [name ID]: https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids
    pub post_script_name: String,

    /// A font face style.
    pub style: Style,

    /// A font face weight.
    pub weight: Weight,

    /// A font face stretch.
    pub stretch: Stretch,

    /// Indicates that the font face is monospaced.
    pub monospaced: bool,
}

/// A font source.
///
/// Either a raw binary data or a file path.
///
/// Stores the whole font and not just a single face.
#[derive(Clone)]
pub enum Source {
    /// A font's raw data, typically backed by a Vec<u8>.
    Binary(alloc::sync::Arc<dyn AsRef<[u8]> + Sync + Send>),

    /// A font's path.
    #[cfg(feature = "fs")]
    File(std::path::PathBuf),

    /// A font's raw data originating from a shared file mapping.
    #[cfg(all(feature = "fs", feature = "memmap"))]
    SharedFile(
        std::path::PathBuf,
        std::sync::Arc<dyn AsRef<[u8]> + Sync + Send>,
    ),
}

impl core::fmt::Debug for Source {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Binary(arg0) => f
                .debug_tuple("SharedBinary")
                .field(&arg0.as_ref().as_ref())
                .finish(),
            #[cfg(feature = "fs")]
            Self::File(arg0) => f.debug_tuple("File").field(arg0).finish(),
            #[cfg(all(feature = "fs", feature = "memmap"))]
            Self::SharedFile(arg0, arg1) => f
                .debug_tuple("SharedFile")
                .field(arg0)
                .field(&arg1.as_ref().as_ref())
                .finish(),
        }
    }
}

impl Source {
    fn with_data<P, T>(&self, p: P) -> Option<T>
    where
        P: FnOnce(&[u8]) -> T,
    {
        match &self {
            #[cfg(all(feature = "fs", not(feature = "memmap")))]
            Source::File(ref path) => {
                let data = std::fs::read(path).ok()?;

                Some(p(&data))
            }
            #[cfg(all(feature = "fs", feature = "memmap"))]
            Source::File(ref path) => {
                let file = std::fs::File::open(path).ok()?;
                let data = unsafe { &memmap2::MmapOptions::new().map(&file).ok()? };

                Some(p(data))
            }
            Source::Binary(ref data) => Some(p(data.as_ref().as_ref())),
            #[cfg(all(feature = "fs", feature = "memmap"))]
            Source::SharedFile(_, ref data) => Some(p(data.as_ref().as_ref())),
        }
    }
}

/// A database query.
///
/// Mainly used by `Database::query()`.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash)]
pub struct Query<'a> {
    /// A prioritized list of font family names or generic family names.
    ///
    /// [font-family](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#propdef-font-family) in CSS.
    pub families: &'a [Family<'a>],

    /// Specifies the weight of glyphs in the font, their degree of blackness or stroke thickness.
    ///
    /// [font-weight](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-weight-prop) in CSS.
    pub weight: Weight,

    /// Selects a normal, condensed, or expanded face from a font family.
    ///
    /// [font-stretch](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-stretch-prop) in CSS.
    pub stretch: Stretch,

    /// Allows italic or oblique faces to be selected.
    ///
    /// [font-style](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-style-prop) in CSS.
    pub style: Style,
}

// Enum value descriptions are from the CSS spec.
/// A [font family](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#propdef-font-family).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Family<'a> {
    /// The name of a font family of choice.
    ///
    /// This must be a *Typographic Family* (ID 16) or a *Family Name* (ID 1) in terms of TrueType.
    /// Meaning you have to pass a family without any additional suffixes like _Bold_, _Italic_,
    /// _Regular_, etc.
    ///
    /// Localized names are allowed.
    Name(&'a str),

    /// Serif fonts represent the formal text style for a script.
    Serif,

    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low contrast
    /// and have stroke endings that are plain — without any flaring, cross stroke,
    /// or other ornamentation.
    SansSerif,

    /// Glyphs in cursive fonts generally use a more informal script style,
    /// and the result looks more like handwritten pen or brush writing than printed letterwork.
    Cursive,

    /// Fantasy fonts are primarily decorative or expressive fonts that
    /// contain decorative or expressive representations of characters.
    Fantasy,

    /// The sole criterion of a monospace font is that all glyphs have the same fixed width.
    Monospace,
}

/// Specifies the weight of glyphs in the font, their degree of blackness or stroke thickness.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
pub struct Weight(pub u16);

impl Default for Weight {
    #[inline]
    fn default() -> Weight {
        Weight::NORMAL
    }
}

impl Weight {
    /// Thin weight (100), the thinnest value.
    pub const THIN: Weight = Weight(100);
    /// Extra light weight (200).
    pub const EXTRA_LIGHT: Weight = Weight(200);
    /// Light weight (300).
    pub const LIGHT: Weight = Weight(300);
    /// Normal (400).
    pub const NORMAL: Weight = Weight(400);
    /// Medium weight (500, higher than normal).
    pub const MEDIUM: Weight = Weight(500);
    /// Semibold weight (600).
    pub const SEMIBOLD: Weight = Weight(600);
    /// Bold weight (700).
    pub const BOLD: Weight = Weight(700);
    /// Extra-bold weight (800).
    pub const EXTRA_BOLD: Weight = Weight(800);
    /// Black weight (900), the thickest value.
    pub const BLACK: Weight = Weight(900);
}

/// Allows italic or oblique faces to be selected.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Style {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Default for Style {
    #[inline]
    fn default() -> Style {
        Style::Normal
    }
}

fn parse_face_info(source: Source, data: &[u8], index: u32) -> Result<FaceInfo, LoadError> {
    let raw_face = ttf_parser::RawFace::parse(data, index).map_err(|_| LoadError::MalformedFont)?;
    let (families, post_script_name) = parse_names(&raw_face).ok_or(LoadError::UnnamedFont)?;
    let (mut style, weight, stretch) = parse_os2(&raw_face);
    let (monospaced, italic) = parse_post(&raw_face);

    if style == Style::Normal && italic {
        style = Style::Italic;
    }

    Ok(FaceInfo {
        id: ID::dummy(),
        source,
        index,
        families,
        post_script_name,
        style,
        weight,
        stretch,
        monospaced,
    })
}

fn parse_names(raw_face: &ttf_parser::RawFace) -> Option<(Vec<(String, Language)>, String)> {
    const NAME_TAG: ttf_parser::Tag = ttf_parser::Tag::from_bytes(b"name");
    let name_data = raw_face.table(NAME_TAG)?;
    let name_table = ttf_parser::name::Table::parse(name_data)?;

    let mut families = collect_families(ttf_parser::name_id::TYPOGRAPHIC_FAMILY, &name_table.names);

    // We have to fallback to Family Name when no Typographic Family Name was set.
    if families.is_empty() {
        families = collect_families(ttf_parser::name_id::FAMILY, &name_table.names);
    }

    // Make English US the first one.
    if families.len() > 1 {
        if let Some(index) = families
            .iter()
            .position(|f| f.1 == Language::English_UnitedStates)
        {
            if index != 0 {
                families.swap(0, index);
            }
        }
    }

    if families.is_empty() {
        return None;
    }

    let post_script_name = name_table
        .names
        .into_iter()
        .find(|name| {
            name.name_id == ttf_parser::name_id::POST_SCRIPT_NAME && name.is_supported_encoding()
        })
        .and_then(|name| name_to_unicode(&name))?;

    Some((families, post_script_name))
}

fn collect_families(name_id: u16, names: &ttf_parser::name::Names) -> Vec<(String, Language)> {
    let mut families = Vec::new();
    for name in names.into_iter() {
        if name.name_id == name_id && name.is_unicode() {
            if let Some(family) = name_to_unicode(&name) {
                families.push((family, name.language()));
            }
        }
    }

    // If no Unicode English US family name was found then look for English MacRoman as well.
    if !families
        .iter()
        .any(|f| f.1 == Language::English_UnitedStates)
    {
        for name in names.into_iter() {
            if name.name_id == name_id && name.is_mac_roman() {
                if let Some(family) = name_to_unicode(&name) {
                    families.push((family, name.language()));
                    break;
                }
            }
        }
    }

    families
}

fn name_to_unicode(name: &ttf_parser::name::Name) -> Option<String> {
    if name.is_unicode() {
        let mut raw_data: Vec<u16> = Vec::new();
        for c in ttf_parser::LazyArray16::<u16>::new(name.name) {
            raw_data.push(c);
        }

        String::from_utf16(&raw_data).ok()
    } else if name.is_mac_roman() {
        // We support only MacRoman encoding here, which should be enough in most cases.
        let mut raw_data = Vec::with_capacity(name.name.len());
        for b in name.name {
            raw_data.push(MAC_ROMAN[*b as usize]);
        }

        String::from_utf16(&raw_data).ok()
    } else {
        None
    }
}

fn parse_os2(raw_face: &ttf_parser::RawFace) -> (Style, Weight, Stretch) {
    const OS2_TAG: ttf_parser::Tag = ttf_parser::Tag::from_bytes(b"OS/2");
    let table = match raw_face
        .table(OS2_TAG)
        .and_then(ttf_parser::os2::Table::parse)
    {
        Some(table) => table,
        None => return (Style::Normal, Weight::NORMAL, Stretch::Normal),
    };

    let style = match table.style() {
        ttf_parser::Style::Normal => Style::Normal,
        ttf_parser::Style::Italic => Style::Italic,
        ttf_parser::Style::Oblique => Style::Oblique,
    };

    let weight = table.weight();
    let stretch = table.width();

    (style, Weight(weight.to_number()), stretch)
}

fn parse_post(raw_face: &ttf_parser::RawFace) -> (bool, bool) {
    // We need just a single value from the `post` table, while ttf-parser will parse all.
    // Therefore we have a custom parser.

    const POST_TAG: ttf_parser::Tag = ttf_parser::Tag::from_bytes(b"post");
    let data = match raw_face.table(POST_TAG) {
        Some(v) => v,
        None => return (false, false),
    };

    // All we care about, it that u32 at offset 12 is non-zero.
    let monospaced = data.get(12..16) != Some(&[0, 0, 0, 0]);

    // Italic angle as f16.16.
    let italic = data.get(4..8) != Some(&[0, 0, 0, 0]);

    (monospaced, italic)
}

trait NameExt {
    fn is_mac_roman(&self) -> bool;
    fn is_supported_encoding(&self) -> bool;
}

impl NameExt for ttf_parser::name::Name<'_> {
    #[inline]
    fn is_mac_roman(&self) -> bool {
        use ttf_parser::PlatformId::Macintosh;
        // https://docs.microsoft.com/en-us/typography/opentype/spec/name#macintosh-encoding-ids-script-manager-codes
        const MACINTOSH_ROMAN_ENCODING_ID: u16 = 0;

        self.platform_id == Macintosh && self.encoding_id == MACINTOSH_ROMAN_ENCODING_ID
    }

    #[inline]
    fn is_supported_encoding(&self) -> bool {
        self.is_unicode() || self.is_mac_roman()
    }
}

// https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-style-matching
// Based on https://github.com/servo/font-kit
#[inline(never)]
fn find_best_match(candidates: &[&FaceInfo], query: &Query) -> Option<usize> {
    debug_assert!(!candidates.is_empty());

    // Step 4.
    let mut matching_set: Vec<usize> = (0..candidates.len()).collect();

    // Step 4a (`font-stretch`).
    let matches = matching_set
        .iter()
        .any(|&index| candidates[index].stretch == query.stretch);
    let matching_stretch = if matches {
        // Exact match.
        query.stretch
    } else if query.stretch <= Stretch::Normal {
        // Closest stretch, first checking narrower values and then wider values.
        let stretch = matching_set
            .iter()
            .filter(|&&index| candidates[index].stretch < query.stretch)
            .min_by_key(|&&index| {
                query.stretch.to_number() - candidates[index].stretch.to_number()
            });

        match stretch {
            Some(&matching_index) => candidates[matching_index].stretch,
            None => {
                let matching_index = *matching_set.iter().min_by_key(|&&index| {
                    candidates[index].stretch.to_number() - query.stretch.to_number()
                })?;

                candidates[matching_index].stretch
            }
        }
    } else {
        // Closest stretch, first checking wider values and then narrower values.
        let stretch = matching_set
            .iter()
            .filter(|&&index| candidates[index].stretch > query.stretch)
            .min_by_key(|&&index| {
                candidates[index].stretch.to_number() - query.stretch.to_number()
            });

        match stretch {
            Some(&matching_index) => candidates[matching_index].stretch,
            None => {
                let matching_index = *matching_set.iter().min_by_key(|&&index| {
                    query.stretch.to_number() - candidates[index].stretch.to_number()
                })?;

                candidates[matching_index].stretch
            }
        }
    };
    matching_set.retain(|&index| candidates[index].stretch == matching_stretch);

    // Step 4b (`font-style`).
    let style_preference = match query.style {
        Style::Italic => [Style::Italic, Style::Oblique, Style::Normal],
        Style::Oblique => [Style::Oblique, Style::Italic, Style::Normal],
        Style::Normal => [Style::Normal, Style::Oblique, Style::Italic],
    };
    let matching_style = *style_preference.iter().find(|&query_style| {
        matching_set
            .iter()
            .any(|&index| candidates[index].style == *query_style)
    })?;

    matching_set.retain(|&index| candidates[index].style == matching_style);

    // Step 4c (`font-weight`).
    //
    // The spec doesn't say what to do if the weight is between 400 and 500 exclusive, so we
    // just use 450 as the cutoff.
    let weight = query.weight.0;

    let matching_weight = if matching_set
        .iter()
        .any(|&index| candidates[index].weight.0 == weight)
    {
        Weight(weight)
    } else if (400..450).contains(&weight)
        && matching_set
            .iter()
            .any(|&index| candidates[index].weight.0 == 500)
    {
        // Check 500 first.
        Weight::MEDIUM
    } else if (450..=500).contains(&weight)
        && matching_set
            .iter()
            .any(|&index| candidates[index].weight.0 == 400)
    {
        // Check 400 first.
        Weight::NORMAL
    } else if weight <= 500 {
        // Closest weight, first checking thinner values and then fatter ones.
        let idx = matching_set
            .iter()
            .filter(|&&index| candidates[index].weight.0 <= weight)
            .min_by_key(|&&index| weight - candidates[index].weight.0);

        match idx {
            Some(&matching_index) => candidates[matching_index].weight,
            None => {
                let matching_index = *matching_set
                    .iter()
                    .min_by_key(|&&index| candidates[index].weight.0 - weight)?;
                candidates[matching_index].weight
            }
        }
    } else {
        // Closest weight, first checking fatter values and then thinner ones.
        let idx = matching_set
            .iter()
            .filter(|&&index| candidates[index].weight.0 >= weight)
            .min_by_key(|&&index| candidates[index].weight.0 - weight);

        match idx {
            Some(&matching_index) => candidates[matching_index].weight,
            None => {
                let matching_index = *matching_set
                    .iter()
                    .min_by_key(|&&index| weight - candidates[index].weight.0)?;
                candidates[matching_index].weight
            }
        }
    };
    matching_set.retain(|&index| candidates[index].weight == matching_weight);

    // Ignore step 4d (`font-size`).

    // Return the result.
    matching_set.into_iter().next()
}

/// Macintosh Roman to UTF-16 encoding table.
///
/// https://en.wikipedia.org/wiki/Mac_OS_Roman
#[rustfmt::skip]
const MAC_ROMAN: &[u16; 256] = &[
    0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007,
    0x0008, 0x0009, 0x000A, 0x000B, 0x000C, 0x000D, 0x000E, 0x000F,
    0x0010, 0x2318, 0x21E7, 0x2325, 0x2303, 0x0015, 0x0016, 0x0017,
    0x0018, 0x0019, 0x001A, 0x001B, 0x001C, 0x001D, 0x001E, 0x001F,
    0x0020, 0x0021, 0x0022, 0x0023, 0x0024, 0x0025, 0x0026, 0x0027,
    0x0028, 0x0029, 0x002A, 0x002B, 0x002C, 0x002D, 0x002E, 0x002F,
    0x0030, 0x0031, 0x0032, 0x0033, 0x0034, 0x0035, 0x0036, 0x0037,
    0x0038, 0x0039, 0x003A, 0x003B, 0x003C, 0x003D, 0x003E, 0x003F,
    0x0040, 0x0041, 0x0042, 0x0043, 0x0044, 0x0045, 0x0046, 0x0047,
    0x0048, 0x0049, 0x004A, 0x004B, 0x004C, 0x004D, 0x004E, 0x004F,
    0x0050, 0x0051, 0x0052, 0x0053, 0x0054, 0x0055, 0x0056, 0x0057,
    0x0058, 0x0059, 0x005A, 0x005B, 0x005C, 0x005D, 0x005E, 0x005F,
    0x0060, 0x0061, 0x0062, 0x0063, 0x0064, 0x0065, 0x0066, 0x0067,
    0x0068, 0x0069, 0x006A, 0x006B, 0x006C, 0x006D, 0x006E, 0x006F,
    0x0070, 0x0071, 0x0072, 0x0073, 0x0074, 0x0075, 0x0076, 0x0077,
    0x0078, 0x0079, 0x007A, 0x007B, 0x007C, 0x007D, 0x007E, 0x007F,
    0x00C4, 0x00C5, 0x00C7, 0x00C9, 0x00D1, 0x00D6, 0x00DC, 0x00E1,
    0x00E0, 0x00E2, 0x00E4, 0x00E3, 0x00E5, 0x00E7, 0x00E9, 0x00E8,
    0x00EA, 0x00EB, 0x00ED, 0x00EC, 0x00EE, 0x00EF, 0x00F1, 0x00F3,
    0x00F2, 0x00F4, 0x00F6, 0x00F5, 0x00FA, 0x00F9, 0x00FB, 0x00FC,
    0x2020, 0x00B0, 0x00A2, 0x00A3, 0x00A7, 0x2022, 0x00B6, 0x00DF,
    0x00AE, 0x00A9, 0x2122, 0x00B4, 0x00A8, 0x2260, 0x00C6, 0x00D8,
    0x221E, 0x00B1, 0x2264, 0x2265, 0x00A5, 0x00B5, 0x2202, 0x2211,
    0x220F, 0x03C0, 0x222B, 0x00AA, 0x00BA, 0x03A9, 0x00E6, 0x00F8,
    0x00BF, 0x00A1, 0x00AC, 0x221A, 0x0192, 0x2248, 0x2206, 0x00AB,
    0x00BB, 0x2026, 0x00A0, 0x00C0, 0x00C3, 0x00D5, 0x0152, 0x0153,
    0x2013, 0x2014, 0x201C, 0x201D, 0x2018, 0x2019, 0x00F7, 0x25CA,
    0x00FF, 0x0178, 0x2044, 0x20AC, 0x2039, 0x203A, 0xFB01, 0xFB02,
    0x2021, 0x00B7, 0x201A, 0x201E, 0x2030, 0x00C2, 0x00CA, 0x00C1,
    0x00CB, 0x00C8, 0x00CD, 0x00CE, 0x00CF, 0x00CC, 0x00D3, 0x00D4,
    0xF8FF, 0x00D2, 0x00DA, 0x00DB, 0x00D9, 0x0131, 0x02C6, 0x02DC,
    0x00AF, 0x02D8, 0x02D9, 0x02DA, 0x00B8, 0x02DD, 0x02DB, 0x02C7,
];
