//! A set of builders to generate Rust source for PHF data structures at
//! compile time.
//!
//! The provided builders are intended to be used in a Cargo build script to
//! generate a Rust source file that will be included in a library at build
//! time.
//!
//! For more information about `rust-phf` crates, see [the `phf` crate's documentation][phf].
//!
//! [phf]: https://docs.rs/phf
//!
//! ## Examples
//!
//! To use `phf_codegen` on build.rs, you have to add dependencies under `[build-dependencies]`:
//!
//! ```toml
//! [build-dependencies]
//! phf = { version = "0.11.1", default-features = false }
//! phf_codegen = "0.11.1"
//! ```
//!
//! Then put code on build.rs:
//!
//! ```ignore
//! use std::env;
//! use std::fs::File;
//! use std::io::{BufWriter, Write};
//! use std::path::Path;
//!
//! fn main() {
//!     let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
//!     let mut file = BufWriter::new(File::create(&path).unwrap());
//!
//!     write!(
//!         &mut file,
//!         "static KEYWORDS: phf::Map<&'static str, Keyword> = {}",
//!         phf_codegen::Map::new()
//!             .entry("loop", "Keyword::Loop")
//!             .entry("continue", "Keyword::Continue")
//!             .entry("break", "Keyword::Break")
//!             .entry("fn", "Keyword::Fn")
//!             .entry("extern", "Keyword::Extern")
//!             .build()
//!     )
//!     .unwrap();
//!     write!(&mut file, ";\n").unwrap();
//! }
//! ```
//!
//! and lib.rs:
//!
//! ```ignore
//! #[derive(Clone)]
//! enum Keyword {
//!     Loop,
//!     Continue,
//!     Break,
//!     Fn,
//!     Extern,
//! }
//!
//! include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
//!
//! pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
//!     KEYWORDS.get(keyword).cloned()
//! }
//! ```
//!
//! ### Byte-String Keys
//! Byte strings by default produce references to fixed-size arrays; the compiler needs a hint
//! to coerce them to slices:
//!
//! build.rs:
//!
//! ```no_run
//! use std::env;
//! use std::fs::File;
//! use std::io::{BufWriter, Write};
//! use std::path::Path;
//!
//! fn main() {
//!     let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
//!     let mut file = BufWriter::new(File::create(&path).unwrap());
//!
//!     writeln!(
//!         &mut file,
//!          "static KEYWORDS: phf::Map<&'static [u8], Keyword> = \n{};\n",
//!          phf_codegen::Map::<&[u8]>::new()
//!              .entry(b"loop", "Keyword::Loop")
//!              .entry(b"continue", "Keyword::Continue")
//!              .entry(b"break", "Keyword::Break")
//!              .entry(b"fn", "Keyword::Fn")
//!              .entry(b"extern", "Keyword::Extern")
//!              .build()
//!     ).unwrap();
//! }
//! ```
//!
//! lib.rs:
//!
//! ```ignore
//! #[derive(Clone)]
//! enum Keyword {
//!     Loop,
//!     Continue,
//!     Break,
//!     Fn,
//!     Extern,
//! }
//!
//! include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
//!
//! pub fn parse_keyword(keyword: &[u8]) -> Option<Keyword> {
//!     KEYWORDS.get(keyword).cloned()
//! }
//! ```
//!
//! ## Note
//!
//! The compiler's stack will overflow when processing extremely long method
//! chains (500+ calls). When generating large PHF data structures, consider
//! looping over the entries or making each call a separate statement:
//!
//! ```rust
//! let entries = [("hello", "1"), ("world", "2")];
//!
//! let mut builder = phf_codegen::Map::new();
//! for &(key, value) in &entries {
//!     builder.entry(key, value);
//! }
//! // ...
//! ```
//!
//! ```rust
//! let mut builder = phf_codegen::Map::new();
//! builder.entry("hello", "1");
//! builder.entry("world", "2");
//! // ...
//! ```

#![doc(html_root_url = "https://docs.rs/phf_codegen/0.11")]
#![allow(clippy::new_without_default)]

use phf_shared::{FmtConst, PhfHash};
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

use phf_generator::HashState;

struct Delegate<T>(T);

impl<T: FmtConst> fmt::Display for Delegate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_const(f)
    }
}

/// A builder for the `phf::Map` type.
pub struct Map<K> {
    keys: Vec<K>,
    values: Vec<String>,
    path: String,
}

impl<K: Hash + PhfHash + Eq + FmtConst> Map<K> {
    /// Creates a new `phf::Map` builder.
    pub fn new() -> Map<K> {
        // FIXME rust#27438
        //
        // On Windows/MSVC there are major problems with the handling of dllimport.
        // Here, because downstream build scripts only invoke generics from phf_codegen,
        // the linker ends up throwing a way a bunch of static symbols we actually need.
        // This works around the problem, assuming that all clients call `Map::new` by
        // calling a non-generic function.
        fn noop_fix_for_27438() {}
        noop_fix_for_27438();

        Map {
            keys: vec![],
            values: vec![],
            path: String::from("::phf"),
        }
    }

    /// Set the path to the `phf` crate from the global namespace
    pub fn phf_path(&mut self, path: &str) -> &mut Map<K> {
        self.path = path.to_owned();
        self
    }

    /// Adds an entry to the builder.
    ///
    /// `value` will be written exactly as provided in the constructed source.
    pub fn entry(&mut self, key: K, value: &str) -> &mut Map<K> {
        self.keys.push(key);
        self.values.push(value.to_owned());
        self
    }

    /// Calculate the hash parameters and return a struct implementing
    /// [`Display`](::std::fmt::Display) which will print the constructed `phf::Map`.
    ///
    /// # Panics
    ///
    /// Panics if there are any duplicate keys.
    pub fn build(&self) -> DisplayMap<'_, K> {
        let mut set = HashSet::new();
        for key in &self.keys {
            if !set.insert(key) {
                panic!("duplicate key `{}`", Delegate(key));
            }
        }

        let state = phf_generator::generate_hash(&self.keys);

        DisplayMap {
            path: &self.path,
            keys: &self.keys,
            values: &self.values,
            state,
        }
    }
}

/// An adapter for printing a [`Map`](Map).
pub struct DisplayMap<'a, K> {
    path: &'a str,
    state: HashState,
    keys: &'a [K],
    values: &'a [String],
}

impl<'a, K: FmtConst + 'a> fmt::Display for DisplayMap<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // funky formatting here for nice output
        write!(
            f,
            "{}::Map {{
    key: {:?},
    disps: &[",
            self.path, self.state.key
        )?;

        // write map displacements
        for &(d1, d2) in &self.state.disps {
            write!(
                f,
                "
        ({}, {}),",
                d1, d2
            )?;
        }

        write!(
            f,
            "
    ],
    entries: &[",
        )?;

        // write map entries
        for &idx in &self.state.map {
            write!(
                f,
                "
        ({}, {}),",
                Delegate(&self.keys[idx]),
                &self.values[idx]
            )?;
        }

        write!(
            f,
            "
    ],
}}"
        )
    }
}

/// A builder for the `phf::Set` type.
pub struct Set<T> {
    map: Map<T>,
}

impl<T: Hash + PhfHash + Eq + FmtConst> Set<T> {
    /// Constructs a new `phf::Set` builder.
    pub fn new() -> Set<T> {
        Set { map: Map::new() }
    }

    /// Set the path to the `phf` crate from the global namespace
    pub fn phf_path(&mut self, path: &str) -> &mut Set<T> {
        self.map.phf_path(path);
        self
    }

    /// Adds an entry to the builder.
    pub fn entry(&mut self, entry: T) -> &mut Set<T> {
        self.map.entry(entry, "()");
        self
    }

    /// Calculate the hash parameters and return a struct implementing
    /// [`Display`](::std::fmt::Display) which will print the constructed `phf::Set`.
    ///
    /// # Panics
    ///
    /// Panics if there are any duplicate keys.
    pub fn build(&self) -> DisplaySet<'_, T> {
        DisplaySet {
            inner: self.map.build(),
        }
    }
}

/// An adapter for printing a [`Set`](Set).
pub struct DisplaySet<'a, T> {
    inner: DisplayMap<'a, T>,
}

impl<'a, T: FmtConst + 'a> fmt::Display for DisplaySet<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::Set {{ map: {} }}", self.inner.path, self.inner)
    }
}

/// A builder for the `phf::OrderedMap` type.
pub struct OrderedMap<K> {
    keys: Vec<K>,
    values: Vec<String>,
    path: String,
}

impl<K: Hash + PhfHash + Eq + FmtConst> OrderedMap<K> {
    /// Constructs a enw `phf::OrderedMap` builder.
    pub fn new() -> OrderedMap<K> {
        OrderedMap {
            keys: vec![],
            values: vec![],
            path: String::from("::phf"),
        }
    }

    /// Set the path to the `phf` crate from the global namespace
    pub fn phf_path(&mut self, path: &str) -> &mut OrderedMap<K> {
        self.path = path.to_owned();
        self
    }

    /// Adds an entry to the builder.
    ///
    /// `value` will be written exactly as provided in the constructed source.
    pub fn entry(&mut self, key: K, value: &str) -> &mut OrderedMap<K> {
        self.keys.push(key);
        self.values.push(value.to_owned());
        self
    }

    /// Calculate the hash parameters and return a struct implementing
    /// [`Display`](::std::fmt::Display) which will print the constructed
    /// `phf::OrderedMap`.
    ///
    /// # Panics
    ///
    /// Panics if there are any duplicate keys.
    pub fn build(&self) -> DisplayOrderedMap<'_, K> {
        let mut set = HashSet::new();
        for key in &self.keys {
            if !set.insert(key) {
                panic!("duplicate key `{}`", Delegate(key));
            }
        }

        let state = phf_generator::generate_hash(&self.keys);

        DisplayOrderedMap {
            path: &self.path,
            state,
            keys: &self.keys,
            values: &self.values,
        }
    }
}

/// An adapter for printing a [`OrderedMap`](OrderedMap).
pub struct DisplayOrderedMap<'a, K> {
    path: &'a str,
    state: HashState,
    keys: &'a [K],
    values: &'a [String],
}

impl<'a, K: FmtConst + 'a> fmt::Display for DisplayOrderedMap<'a, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}::OrderedMap {{
    key: {:?},
    disps: &[",
            self.path, self.state.key
        )?;
        for &(d1, d2) in &self.state.disps {
            write!(
                f,
                "
        ({}, {}),",
                d1, d2
            )?;
        }
        write!(
            f,
            "
    ],
    idxs: &[",
        )?;
        for &idx in &self.state.map {
            write!(
                f,
                "
        {},",
                idx
            )?;
        }
        write!(
            f,
            "
    ],
    entries: &[",
        )?;
        for (key, value) in self.keys.iter().zip(self.values.iter()) {
            write!(
                f,
                "
        ({}, {}),",
                Delegate(key),
                value
            )?;
        }
        write!(
            f,
            "
    ],
}}"
        )
    }
}

/// A builder for the `phf::OrderedSet` type.
pub struct OrderedSet<T> {
    map: OrderedMap<T>,
}

impl<T: Hash + PhfHash + Eq + FmtConst> OrderedSet<T> {
    /// Constructs a new `phf::OrderedSet` builder.
    pub fn new() -> OrderedSet<T> {
        OrderedSet {
            map: OrderedMap::new(),
        }
    }

    /// Set the path to the `phf` crate from the global namespace
    pub fn phf_path(&mut self, path: &str) -> &mut OrderedSet<T> {
        self.map.phf_path(path);
        self
    }

    /// Adds an entry to the builder.
    pub fn entry(&mut self, entry: T) -> &mut OrderedSet<T> {
        self.map.entry(entry, "()");
        self
    }

    /// Calculate the hash parameters and return a struct implementing
    /// [`Display`](::std::fmt::Display) which will print the constructed
    /// `phf::OrderedSet`.
    ///
    /// # Panics
    ///
    /// Panics if there are any duplicate keys.
    pub fn build(&self) -> DisplayOrderedSet<'_, T> {
        DisplayOrderedSet {
            inner: self.map.build(),
        }
    }
}

/// An adapter for printing a [`OrderedSet`](OrderedSet).
pub struct DisplayOrderedSet<'a, T> {
    inner: DisplayOrderedMap<'a, T>,
}

impl<'a, T: FmtConst + 'a> fmt::Display for DisplayOrderedSet<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}::OrderedSet {{ map: {} }}",
            self.inner.path, self.inner
        )
    }
}
