use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::path::Path;

/// Build configuration. See [CFG].
pub struct Cfg<'a> {
    /// See [`CFG.include_prefix`][CFG#cfginclude_prefix].
    pub include_prefix: &'a str,
    /// See [`CFG.exported_header_dirs`][CFG#cfgexported_header_dirs].
    pub exported_header_dirs: Vec<&'a Path>,
    /// See [`CFG.exported_header_prefixes`][CFG#cfgexported_header_prefixes].
    pub exported_header_prefixes: Vec<&'a str>,
    /// See [`CFG.exported_header_links`][CFG#cfgexported_header_links].
    pub exported_header_links: Vec<&'a str>,
    /// See [`CFG.doxygen`][CFG#cfgdoxygen].
    pub doxygen: bool,
    marker: PhantomData<*const ()>, // !Send + !Sync
}

/// Global configuration of the current build.
///
/// <br>
///
/// <div style="float:right;margin:22px 50px 0;font-size:1.15em;opacity:.73"><strong>&amp;str</strong></div>
///
/// ## **`CFG.include_prefix`**
///
/// The prefix at which C++ code from your crate as well as directly dependent
/// crates can access the code generated during this build.
///
/// By default, the `include_prefix` is equal to the name of the current crate.
/// That means if your crate is called `demo` and has Rust source files in a
/// *src/* directory and maybe some handwritten C++ header files in an
/// *include/* directory, then the current crate as well as downstream crates
/// might include them as follows:
///
/// ```
/// # const _: &str = stringify! {
///   // include one of the handwritten headers:
/// #include "demo/include/wow.h"
///
///   // include a header generated from Rust cxx::bridge:
/// #include "demo/src/lib.rs.h"
/// # };
/// ```
///
/// By modifying `CFG.include_prefix` we can substitute a prefix that is
/// different from the crate name if desired. Here we'll change it to
/// `"path/to"` which will make import paths take the form
/// `"path/to/include/wow.h"` and `"path/to/src/lib.rs.h"`.
///
/// ```no_run
/// // build.rs
///
/// use cxx_build::CFG;
///
/// fn main() {
///     CFG.include_prefix = "path/to";
///
///     cxx_build::bridge("src/lib.rs")
///         .file("src/demo.cc") // probably contains `#include "path/to/src/lib.rs.h"`
///         /* ... */
///         .compile("demo");
/// }
/// ```
///
/// Note that cross-crate imports are only made available between **direct
/// dependencies**. Another crate must directly depend on your crate in order to
/// #include its headers; a transitive dependency is not sufficient.
/// Additionally, headers from a direct dependency are only importable if the
/// dependency's Cargo.toml manifest contains a `links` key. If not, its headers
/// will not be importable from outside of the same crate.
///
/// <br>
///
/// <div style="float:right;margin:22px 50px 0;font-size:1.15em;opacity:.73"><strong>Vec&lt;&amp;Path&gt;</strong></div>
///
/// ## **`CFG.exported_header_dirs`**
///
/// A vector of absolute paths. The current crate, directly dependent crates,
/// and further crates to which this crate's headers are exported (see below)
/// will be able to `#include` headers from these directories.
///
/// Adding a directory to `exported_header_dirs` is similar to adding it to the
/// current build via the `cc` crate's [`Build::include`][cc::Build::include],
/// but *also* makes the directory available to downstream crates that want to
/// `#include` one of the headers from your crate. If the dir were added only
/// using `Build::include`, the downstream crate including your header would
/// need to manually add the same directory to their own build as well.
///
/// When using `exported_header_dirs`, your crate must also set a `links` key
/// for itself in Cargo.toml. See [*the `links` manifest key*][links]. The
/// reason is that Cargo imposes no ordering on the execution of build scripts
/// without a `links` key, which means the downstream crate's build script might
/// execute before yours decides what to put into `exported_header_dirs`.
///
/// [links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
///
/// ### Example
///
/// One of your crate's headers wants to include a system library, such as
/// `#include "Python.h"`.
///
/// ```no_run
/// // build.rs
///
/// use cxx_build::CFG;
/// use std::path::PathBuf;
///
/// fn main() {
///     let python3 = pkg_config::probe_library("python3").unwrap();
///     let python_include_paths = python3.include_paths.iter().map(PathBuf::as_path);
///     CFG.exported_header_dirs.extend(python_include_paths);
///
///     cxx_build::bridge("src/bridge.rs").compile("demo");
/// }
/// ```
///
/// ### Example
///
/// Your crate wants to rearrange the headers that it exports vs how they're
/// laid out locally inside the crate's source directory.
///
/// Suppose the crate as published contains a file at `./include/myheader.h` but
/// wants it available to downstream crates as `#include "foo/v1/public.h"`.
///
/// ```no_run
/// // build.rs
///
/// use cxx_build::CFG;
/// use std::path::Path;
/// use std::{env, fs};
///
/// fn main() {
///     let out_dir = env::var_os("OUT_DIR").unwrap();
///     let headers = Path::new(&out_dir).join("headers");
///     CFG.exported_header_dirs.push(&headers);
///
///     // We contain `include/myheader.h` locally, but
///     // downstream will use `#include "foo/v1/public.h"`
///     let foo = headers.join("foo").join("v1");
///     fs::create_dir_all(&foo).unwrap();
///     fs::copy("include/myheader.h", foo.join("public.h")).unwrap();
///
///     cxx_build::bridge("src/bridge.rs").compile("demo");
/// }
/// ```
///
/// <p style="margin:0"><br><br></p>
///
/// <div style="float:right;margin:22px 50px 0;font-size:1.15em;opacity:.73"><strong>Vec&lt;&amp;str&gt;</strong></div>
///
/// ## **`CFG.exported_header_prefixes`**
///
/// Vector of strings. These each refer to the `include_prefix` of one of your
/// direct dependencies, or a prefix thereof. They describe which of your
/// dependencies participate in your crate's C++ public API, as opposed to
/// private use by your crate's implementation.
///
/// As a general rule, if one of your headers `#include`s something from one of
/// your dependencies, you need to put that dependency's `include_prefix` into
/// `CFG.exported_header_prefixes` (*or* their `links` key into
/// `CFG.exported_header_links`; see below). On the other hand if only your C++
/// implementation files and *not* your headers are importing from the
/// dependency, you do not export that dependency.
///
/// The significance of exported headers is that if downstream code (crate 𝒜)
/// contains an `#include` of a header from your crate (ℬ) and your header
/// contains an `#include` of something from your dependency (𝒞), the exported
/// dependency 𝒞 becomes available during the downstream crate 𝒜's build.
/// Otherwise the downstream crate 𝒜 doesn't know about 𝒞 and wouldn't be able
/// to find what header your header is referring to, and would fail to build.
///
/// When using `exported_header_prefixes`, your crate must also set a `links`
/// key for itself in Cargo.toml.
///
/// ### Example
///
/// Suppose you have a crate with 5 direct dependencies and the `include_prefix`
/// for each one are:
///
/// - "crate0"
/// - "group/api/crate1"
/// - "group/api/crate2"
/// - "group/api/contrib/crate3"
/// - "detail/crate4"
///
/// Your header involves types from the first four so we re-export those as part
/// of your public API, while crate4 is only used internally by your cc file not
/// your header, so we do not export:
///
/// ```no_run
/// // build.rs
///
/// use cxx_build::CFG;
///
/// fn main() {
///     CFG.exported_header_prefixes = vec!["crate0", "group/api"];
///
///     cxx_build::bridge("src/bridge.rs")
///         .file("src/impl.cc")
///         .compile("demo");
/// }
/// ```
///
/// <p style="margin:0"><br><br></p>
///
/// <div style="float:right;margin:22px 50px 0;font-size:1.15em;opacity:.73"><strong>Vec&lt;&amp;str&gt;</strong></div>
///
/// ## **`CFG.exported_header_links`**
///
/// Vector of strings. These each refer to the `links` attribute ([*the `links`
/// manifest key*][links]) of one of your crate's direct dependencies.
///
/// This achieves an equivalent result to `CFG.exported_header_prefixes` by
/// re-exporting a dependency as part of your crate's public API, except with
/// finer grained control for cases when multiple crates might be sharing the
/// same `include_prefix` and you'd like to export some but not others. Links
/// attributes are guaranteed to be unique identifiers by Cargo.
///
/// When using `exported_header_links`, your crate must also set a `links` key
/// for itself in Cargo.toml.
///
/// ### Example
///
/// ```no_run
/// // build.rs
///
/// use cxx_build::CFG;
///
/// fn main() {
///     CFG.exported_header_links.push("git2");
///
///     cxx_build::bridge("src/bridge.rs").compile("demo");
/// }
/// ```
///
/// <p style="margin:0"><br><br></p>
///
/// <div style="float:right;margin:22px 50px 0;font-size:1.15em;opacity:.73"><strong>bool</strong></div>
///
/// ## **`CFG.doxygen`**
///
/// Boolean. Whether to propagate Rust documentation from inside the cxx::bridge
/// module as Doxygen-style comments in the generated C++ header.
///
/// Documentation on the following are supported:
///
/// - shared structs, and fields of shared structs
/// - shared enums, and their variants
/// - extern "Rust" opaque types
/// - extern "Rust" functions, including methods/member functions
///
/// ### Example
///
/// ```no_run
/// // build.rs
///
/// use cxx_build::CFG;
///
/// fn main() {
///     CFG.doxygen = true;
///
///     cxx_build::bridge("src/bridge.rs").compile("demo");
/// }
/// ```
///
/// ```rust
/// // src/bridge.rs
///
/// #[cxx::bridge]
/// mod ffi {
///     /// documentation of MyStruct
///     pub struct MyStruct {
///         /// documentation of the struct field
///         lol: String,
///     }
///
///     extern "Rust" {
///         /// documentation of MyType
///         type MyType;
///
///         /// function documentation
///         fn asdf() -> bool;
///     }
/// }
/// #
/// # pub struct MyType;
/// # fn asdf() -> bool { true }
/// # fn main() {}
/// ```
///
/// With `CFG.doxygen` enabled, the generated C++ header through which
/// downstream C++ code will be able to access these shared structs and extern
/// "Rust" signatures will have the Rust documentation comments propagated as
/// Doxygen-style comments:
///
/// ```cpp
/// /// documentation of MyStruct
/// struct MyStruct final {
///   /// documentation of the struct field
///   ::rust::String lol;
///   …
/// };
/// ```
///
/// Otherwise by default (without `CFG.doxygen`) they'll just be `//` comments.
#[cfg(doc)]
pub static mut CFG: Cfg = Cfg {
    include_prefix: "",
    exported_header_dirs: Vec::new(),
    exported_header_prefixes: Vec::new(),
    exported_header_links: Vec::new(),
    doxygen: false,
    marker: PhantomData,
};

impl<'a> Debug for Cfg<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let Self {
            include_prefix,
            exported_header_dirs,
            exported_header_prefixes,
            exported_header_links,
            doxygen,
            marker: _,
        } = self;
        formatter
            .debug_struct("Cfg")
            .field("include_prefix", include_prefix)
            .field("exported_header_dirs", exported_header_dirs)
            .field("exported_header_prefixes", exported_header_prefixes)
            .field("exported_header_links", exported_header_links)
            .field("doxygen", doxygen)
            .finish()
    }
}

#[cfg(not(doc))]
pub use self::r#impl::Cfg::CFG;

#[cfg(not(doc))]
mod r#impl {
    use crate::intern::{intern, InternedString};
    use crate::syntax::map::UnorderedMap as Map;
    use crate::vec::{self, InternedVec as _};
    use std::cell::RefCell;
    use std::fmt::{self, Debug};
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};
    use std::sync::{OnceLock, PoisonError, RwLock};

    struct CurrentCfg {
        include_prefix: InternedString,
        exported_header_dirs: Vec<InternedString>,
        exported_header_prefixes: Vec<InternedString>,
        exported_header_links: Vec<InternedString>,
        doxygen: bool,
    }

    impl CurrentCfg {
        fn default() -> Self {
            let include_prefix = crate::env_os("CARGO_PKG_NAME")
                .map(|pkg| intern(&pkg.to_string_lossy()))
                .unwrap_or_default();
            let exported_header_dirs = Vec::new();
            let exported_header_prefixes = Vec::new();
            let exported_header_links = Vec::new();
            let doxygen = false;
            CurrentCfg {
                include_prefix,
                exported_header_dirs,
                exported_header_prefixes,
                exported_header_links,
                doxygen,
            }
        }
    }

    fn current() -> &'static RwLock<CurrentCfg> {
        static CURRENT: OnceLock<RwLock<CurrentCfg>> = OnceLock::new();
        CURRENT.get_or_init(|| RwLock::new(CurrentCfg::default()))
    }

    thread_local! {
        // FIXME: If https://github.com/rust-lang/rust/issues/77425 is resolved,
        // we can delete this thread local side table and instead make each CFG
        // instance directly own the associated super::Cfg.
        //
        //     #[allow(const_item_mutation)]
        //     pub const CFG: Cfg = Cfg {
        //         cfg: AtomicPtr::new(ptr::null_mut()),
        //     };
        //     pub struct Cfg {
        //         cfg: AtomicPtr<super::Cfg>,
        //     }
        //
        static CONST_DEREFS: RefCell<Map<Handle, Box<super::Cfg<'static>>>> = RefCell::default();
    }

    #[derive(Eq, PartialEq, Hash)]
    struct Handle(*const Cfg<'static>);

    impl<'a> Cfg<'a> {
        fn current() -> super::Cfg<'a> {
            let current = current().read().unwrap_or_else(PoisonError::into_inner);
            let include_prefix = current.include_prefix.str();
            let exported_header_dirs = current.exported_header_dirs.vec();
            let exported_header_prefixes = current.exported_header_prefixes.vec();
            let exported_header_links = current.exported_header_links.vec();
            let doxygen = current.doxygen;
            super::Cfg {
                include_prefix,
                exported_header_dirs,
                exported_header_prefixes,
                exported_header_links,
                doxygen,
                marker: PhantomData,
            }
        }

        const fn handle(self: &Cfg<'a>) -> Handle {
            Handle(<*const Cfg>::cast(self))
        }
    }

    // Since super::Cfg is !Send and !Sync, all Cfg are thread local and will
    // drop on the same thread where they were created.
    pub enum Cfg<'a> {
        Mut(super::Cfg<'a>),
        CFG,
    }

    impl<'a> Debug for Cfg<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            if let Cfg::Mut(cfg) = self {
                Debug::fmt(cfg, formatter)
            } else {
                Debug::fmt(&Cfg::current(), formatter)
            }
        }
    }

    impl<'a> Deref for Cfg<'a> {
        type Target = super::Cfg<'a>;

        fn deref(&self) -> &Self::Target {
            if let Cfg::Mut(cfg) = self {
                cfg
            } else {
                let cfg = CONST_DEREFS.with(|derefs| -> *mut super::Cfg {
                    &raw mut **derefs
                        .borrow_mut()
                        .entry(self.handle())
                        .or_insert_with(|| Box::new(Cfg::current()))
                });
                unsafe { &mut *cfg }
            }
        }
    }

    impl<'a> DerefMut for Cfg<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            if let Cfg::CFG = self {
                CONST_DEREFS.with(|derefs| derefs.borrow_mut().remove(&self.handle()));
                *self = Cfg::Mut(Cfg::current());
            }
            match self {
                Cfg::Mut(cfg) => cfg,
                Cfg::CFG => unreachable!(),
            }
        }
    }

    impl<'a> Drop for Cfg<'a> {
        fn drop(&mut self) {
            if let Cfg::Mut(cfg) = self {
                let super::Cfg {
                    include_prefix,
                    exported_header_dirs,
                    exported_header_prefixes,
                    exported_header_links,
                    doxygen,
                    marker: _,
                } = cfg;
                let mut current = current().write().unwrap_or_else(PoisonError::into_inner);
                current.include_prefix = intern(include_prefix);
                current.exported_header_dirs = vec::intern(exported_header_dirs);
                current.exported_header_prefixes = vec::intern(exported_header_prefixes);
                current.exported_header_links = vec::intern(exported_header_links);
                current.doxygen = *doxygen;
            } else {
                CONST_DEREFS.with(|derefs| derefs.borrow_mut().remove(&self.handle()));
            }
        }
    }
}
