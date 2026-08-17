// SPDX-License-Identifier: Apache-2.0

//================================================
// Macros
//================================================

#[cfg(feature = "runtime")]
macro_rules! link {
    (
        @LOAD:
        $(#[doc=$doc:expr])*
        #[cfg($cfg:meta)]
        fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*
    ) => (
        $(#[doc=$doc])*
        #[cfg($cfg)]
        pub fn $name(library: &mut super::SharedLibrary) {
            let symbol = unsafe { library.library.get(stringify!($name).as_bytes()) }.ok();
            library.functions.$name = match symbol {
                Some(s) => *s,
                None => None,
            };
        }

        #[cfg(not($cfg))]
        pub fn $name(_: &mut super::SharedLibrary) {}
    );

    (
        @LOAD:
        fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*
    ) => (
        link!(@LOAD: #[cfg(feature = "runtime")] fn $name($($pname: $pty), *) $(-> $ret)*);
    );

    (
        $(
            $(#[doc=$doc:expr] #[cfg($cfg:meta)])*
            pub fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*;
        )+
    ) => (
        use std::cell::{RefCell};
        use std::fmt;
        use std::sync::{Arc};
        use std::path::{Path, PathBuf};

        /// The (minimum) version of a `libclang` shared library.
        #[allow(missing_docs)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Version {
            V3_5 = 35,
            V3_6 = 36,
            V3_7 = 37,
            V3_8 = 38,
            V3_9 = 39,
            V4_0 = 40,
            V5_0 = 50,
            V6_0 = 60,
            V7_0 = 70,
            V8_0 = 80,
            V9_0 = 90,
            V11_0 = 110,
            V12_0 = 120,
            V16_0 = 160,
            V17_0 = 170,
        }

        impl fmt::Display for Version {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use Version::*;
                match self {
                    V3_5 => write!(f, "3.5.x"),
                    V3_6 => write!(f, "3.6.x"),
                    V3_7 => write!(f, "3.7.x"),
                    V3_8 => write!(f, "3.8.x"),
                    V3_9 => write!(f, "3.9.x"),
                    V4_0 => write!(f, "4.0.x"),
                    V5_0 => write!(f, "5.0.x"),
                    V6_0 => write!(f, "6.0.x"),
                    V7_0 => write!(f, "7.0.x"),
                    V8_0 => write!(f, "8.0.x"),
                    V9_0 => write!(f, "9.0.x - 10.0.x"),
                    V11_0 => write!(f, "11.0.x"),
                    V12_0 => write!(f, "12.0.x - 15.0.x"),
                    V16_0 => write!(f, "16.0.x"),
                    V17_0 => write!(f, "17.0.x or later"),
                }
            }
        }

        /// The set of functions loaded dynamically.
        #[derive(Debug, Default)]
        pub struct Functions {
            $(
                $(#[doc=$doc] #[cfg($cfg)])*
                pub $name: Option<unsafe extern fn($($pname: $pty), *) $(-> $ret)*>,
            )+
        }

        /// A dynamically loaded instance of the `libclang` library.
        #[derive(Debug)]
        pub struct SharedLibrary {
            library: libloading::Library,
            path: PathBuf,
            pub functions: Functions,
        }

        impl SharedLibrary {
            fn new(library: libloading::Library, path: PathBuf) -> Self {
                Self { library, path, functions: Functions::default() }
            }

            /// Returns the path to this `libclang` shared library.
            pub fn path(&self) -> &Path {
                &self.path
            }

            /// Returns the (minimum) version of this `libclang` shared library.
            ///
            /// If this returns `None`, it indicates that the version is too old
            /// to be supported by this crate (i.e., `3.4` or earlier). If the
            /// version of this shared library is more recent than that fully
            /// supported by this crate, the most recent fully supported version
            /// will be returned.
            pub fn version(&self) -> Option<Version> {
                macro_rules! check {
                    ($fn:expr, $version:ident) => {
                        if self.library.get::<unsafe extern fn()>($fn).is_ok() {
                            return Some(Version::$version);
                        }
                    };
                }

                unsafe {
                    check!(b"clang_CXXMethod_isExplicit", V17_0);
                    check!(b"clang_CXXMethod_isCopyAssignmentOperator", V16_0);
                    check!(b"clang_Cursor_getVarDeclInitializer", V12_0);
                    check!(b"clang_Type_getValueType", V11_0);
                    check!(b"clang_Cursor_isAnonymousRecordDecl", V9_0);
                    check!(b"clang_Cursor_getObjCPropertyGetterName", V8_0);
                    check!(b"clang_File_tryGetRealPathName", V7_0);
                    check!(b"clang_CXIndex_setInvocationEmissionPathOption", V6_0);
                    check!(b"clang_Cursor_isExternalSymbol", V5_0);
                    check!(b"clang_EvalResult_getAsLongLong", V4_0);
                    check!(b"clang_CXXConstructor_isConvertingConstructor", V3_9);
                    check!(b"clang_CXXField_isMutable", V3_8);
                    check!(b"clang_Cursor_getOffsetOfField", V3_7);
                    check!(b"clang_Cursor_getStorageClass", V3_6);
                    check!(b"clang_Type_getNumTemplateArguments", V3_5);
                }

                None
            }
        }

        thread_local!(static LIBRARY: RefCell<Option<Arc<SharedLibrary>>> = RefCell::new(None));

        /// Returns whether a `libclang` shared library is loaded on this thread.
        pub fn is_loaded() -> bool {
            LIBRARY.with(|l| l.borrow().is_some())
        }

        fn with_library<T, F>(f: F) -> Option<T> where F: FnOnce(&SharedLibrary) -> T {
            LIBRARY.with(|l| {
                match l.borrow().as_ref() {
                    Some(library) => Some(f(&library)),
                    _ => None,
                }
            })
        }

        $(
            #[cfg_attr(feature="cargo-clippy", allow(clippy::missing_safety_doc))]
            #[cfg_attr(feature="cargo-clippy", allow(clippy::too_many_arguments))]
            $(#[doc=$doc] #[cfg($cfg)])*
            pub unsafe fn $name($($pname: $pty), *) $(-> $ret)* {
                let f = with_library(|library| {
                    if let Some(function) = library.functions.$name {
                        function
                    } else {
                        panic!(
                            r#"
A `libclang` function was called that is not supported by the loaded `libclang` instance.

    called function = `{0}`
    loaded `libclang` instance = {1}

The minimum `libclang` requirement for this particular function can be found here:
https://docs.rs/clang-sys/latest/clang_sys/{0}/index.html

Instructions for installing `libclang` can be found here:
https://rust-lang.github.io/rust-bindgen/requirements.html
"#, 
                            stringify!($name),
                            library
                                .version()
                                .map(|v| format!("{}", v))
                                .unwrap_or_else(|| "unsupported version".into()),
                        );
                    }
                }).expect("a `libclang` shared library is not loaded on this thread");
                f($($pname), *)
            }

            $(#[doc=$doc] #[cfg($cfg)])*
            pub mod $name {
                pub fn is_loaded() -> bool {
                    super::with_library(|l| l.functions.$name.is_some()).unwrap_or(false)
                }
            }
        )+

        mod load {
            $(link!(@LOAD: $(#[cfg($cfg)])* fn $name($($pname: $pty), *) $(-> $ret)*);)+
        }

        /// Loads a `libclang` shared library and returns the library instance.
        ///
        /// This function does not attempt to load any functions from the shared library. The caller
        /// is responsible for loading the functions they require.
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library could not be found
        /// * the `libclang` shared library could not be opened
        pub fn load_manually() -> Result<SharedLibrary, String> {
            #[allow(dead_code)]
            mod build {
                include!(concat!(env!("OUT_DIR"), "/macros.rs"));
                pub mod common { include!(concat!(env!("OUT_DIR"), "/common.rs")); }
                pub mod dynamic { include!(concat!(env!("OUT_DIR"), "/dynamic.rs")); }
            }

            let (directory, filename) = build::dynamic::find(true)?;
            let path = directory.join(filename);

            unsafe {
                let library = libloading::Library::new(&path).map_err(|e| {
                    format!(
                        "the `libclang` shared library at {} could not be opened: {}",
                        path.display(),
                        e,
                    )
                });

                let mut library = SharedLibrary::new(library?, path);
                $(load::$name(&mut library);)+
                Ok(library)
            }
        }

        /// Loads a `libclang` shared library for use in the current thread.
        ///
        /// This functions attempts to load all the functions in the shared library. Whether a
        /// function has been loaded can be tested by calling the `is_loaded` function on the
        /// module with the same name as the function (e.g., `clang_createIndex::is_loaded()` for
        /// the `clang_createIndex` function).
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library could not be found
        /// * the `libclang` shared library could not be opened
        #[allow(dead_code)]
        pub fn load() -> Result<(), String> {
            let library = Arc::new(load_manually()?);
            LIBRARY.with(|l| *l.borrow_mut() = Some(library));
            Ok(())
        }

        /// Unloads the `libclang` shared library in use in the current thread.
        ///
        /// # Failures
        ///
        /// * a `libclang` shared library is not in use in the current thread
        pub fn unload() -> Result<(), String> {
            let library = set_library(None);
            if library.is_some() {
                Ok(())
            } else {
                Err("a `libclang` shared library is not in use in the current thread".into())
            }
        }

        /// Returns the library instance stored in TLS.
        ///
        /// This functions allows for sharing library instances between threads.
        pub fn get_library() -> Option<Arc<SharedLibrary>> {
            LIBRARY.with(|l| l.borrow_mut().clone())
        }

        /// Sets the library instance stored in TLS and returns the previous library.
        ///
        /// This functions allows for sharing library instances between threads.
        pub fn set_library(library: Option<Arc<SharedLibrary>>) -> Option<Arc<SharedLibrary>> {
            LIBRARY.with(|l| mem::replace(&mut *l.borrow_mut(), library))
        }
    )
}

#[cfg(not(feature = "runtime"))]
macro_rules! link {
    (
        $(
            $(#[doc=$doc:expr] #[cfg($cfg:meta)])*
            pub fn $name:ident($($pname:ident: $pty:ty), *) $(-> $ret:ty)*;
        )+
    ) => (
        extern {
            $(
                $(#[doc=$doc] #[cfg($cfg)])*
                pub fn $name($($pname: $pty), *) $(-> $ret)*;
            )+
        }

        $(
            $(#[doc=$doc] #[cfg($cfg)])*
            pub mod $name {
                pub fn is_loaded() -> bool { true }
            }
        )+
    )
}
