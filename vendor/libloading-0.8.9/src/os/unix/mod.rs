// A hack for docs.rs to build documentation that has both windows and linux documentation in the
// same rustdoc build visible.
#[cfg(all(libloading_docs, not(unix)))]
mod unix_imports {}
#[cfg(any(not(libloading_docs), unix))]
mod unix_imports {
    pub(super) use std::os::unix::ffi::OsStrExt;
}

pub use self::consts::*;
use self::unix_imports::*;
use std::ffi::{CStr, OsStr};
use std::os::raw;
use std::{fmt, marker, mem, ptr};
use util::{cstr_cow_from_bytes, ensure_compatible_types};

mod consts;

/// Run code and handle errors reported by `dlerror`.
///
/// This function first executes the `closure` function containing calls to the functions that
/// report their errors via `dlerror`. This closure may return either `None` or `Some(*)` to
/// further affect operation of this function.
///
/// In case the `closure` returns `None`, `with_dlerror` inspects the `dlerror`. `dlerror` may
/// decide to not provide any error description, in which case `Err(None)` is returned to the
/// caller. Otherwise the `error` callback is invoked to allow inspection and conversion of the
/// error message. The conversion result is returned as `Err(Some(Error))`.
///
/// If the operations that report their errors via `dlerror` were all successful, `closure` should
/// return `Some(T)` instead. In this case `dlerror` is not inspected at all.
///
/// # Notes
///
/// The whole `dlerror` handling scheme is done via setting and querying some global state. For
/// that reason it is not safe to use dynamic library loading in MT-capable environment at all.
/// Only in POSIX 2008+TC1 a thread-local state was allowed for `dlerror`, making the dl* family of
/// functions possibly MT-safe, depending on the implementation of `dlerror`.
///
/// In practice (as of 2020-04-01) most of the widely used targets use a thread-local for error
/// state and have been doing so for a long time.
pub fn with_dlerror<T, F, Error>(closure: F, error: fn(&CStr) -> Error) -> Result<T, Option<Error>>
where
    F: FnOnce() -> Option<T>,
{
    // We used to guard all uses of dl* functions with our own mutex. This made them safe to use in
    // MT programs provided the only way a program used dl* was via this library. However, it also
    // had a number of downsides or cases where it failed to handle the problems. For instance,
    // if any other library called `dlerror` internally concurrently with `libloading` things would
    // still go awry.
    //
    // On platforms where `dlerror` is still MT-unsafe, `dlsym` (`Library::get`) can spuriously
    // succeed and return a null pointer for a symbol when the actual symbol look-up operation
    // fails. Instances where the actual symbol _could_ be `NULL` are platform specific. For
    // instance on GNU glibc based-systems (an excerpt from dlsym(3)):
    //
    // > The value of a symbol returned by dlsym() will never be NULL if the shared object is the
    // > result of normal compilation,  since  a  global  symbol is never placed at the NULL
    // > address. There are nevertheless cases where a lookup using dlsym() may return NULL as the
    // > value of a symbol. For example, the symbol value may be  the  result of a GNU indirect
    // > function (IFUNC) resolver function that returns NULL as the resolved value.

    // While we could could call `dlerror` here to clear the previous error value, only the `dlsym`
    // call depends on it being cleared beforehand and only in some cases too. We will instead
    // clear the error inside the dlsym binding instead.
    //
    // In all the other cases, clearing the error here will only be hiding misuse of these bindings
    // or a bug in implementation of dl* family of functions.
    closure().ok_or_else(|| unsafe {
        // This code will only get executed if the `closure` returns `None`.
        let dlerror_str = dlerror();
        if dlerror_str.is_null() {
            // In non-dlsym case this may happen when there’re bugs in our bindings or there’s
            // non-libloading user of libdl; possibly in another thread.
            None
        } else {
            // You can’t even rely on error string being static here; call to subsequent dlerror
            // may invalidate or overwrite the error message. Why couldn’t they simply give up the
            // ownership over the message?
            // TODO: should do locale-aware conversion here. OTOH Rust doesn’t seem to work well in
            // any system that uses non-utf8 locale, so I doubt there’s a problem here.
            Some(error(CStr::from_ptr(dlerror_str)))
            // Since we do a copy of the error string above, maybe we should call dlerror again to
            // let libdl know it may free its copy of the string now?
        }
    })
}

/// A platform-specific counterpart of the cross-platform [`Library`](crate::Library).
pub struct Library {
    handle: *mut raw::c_void,
}

unsafe impl Send for Library {}

// That being said... this section in the volume 2 of POSIX.1-2008 states:
//
// > All functions defined by this volume of POSIX.1-2008 shall be thread-safe, except that the
// > following functions need not be thread-safe.
//
// With notable absence of any dl* function other than dlerror in the list. By “this volume”
// I suppose they refer precisely to the “volume 2”. dl* family of functions are specified
// by this same volume, so the conclusion is indeed that dl* functions are required by POSIX
// to be thread-safe. Great!
//
// See for more details:
//
//  * https://github.com/nagisa/rust_libloading/pull/17
//  * http://pubs.opengroup.org/onlinepubs/9699919799/functions/V2_chap02.html#tag_15_09_01
unsafe impl Sync for Library {}

impl Library {
    /// Find and eagerly load a shared library (module).
    ///
    /// If the `filename` contains a [path separator], the `filename` is interpreted as a `path` to
    /// a file. Otherwise, platform-specific algorithms are employed to find a library with a
    /// matching file name.
    ///
    /// This is equivalent to <code>[Library::open](filename, [RTLD_LAZY] | [RTLD_LOCAL])</code>.
    ///
    /// [path separator]: std::path::MAIN_SEPARATOR
    ///
    /// # Safety
    ///
    /// When a library is loaded, initialisation routines contained within the library are executed.
    /// For the purposes of safety, the execution of these routines is conceptually the same calling an
    /// unknown foreign function and may impose arbitrary requirements on the caller for the call
    /// to be sound.
    ///
    /// Additionally, the callers of this function must also ensure that execution of the
    /// termination routines contained within the library is safe as well. These routines may be
    /// executed when the library is unloaded.
    #[inline]
    pub unsafe fn new<P: AsRef<OsStr>>(filename: P) -> Result<Library, crate::Error> {
        Library::open(Some(filename), RTLD_LAZY | RTLD_LOCAL)
    }

    /// Load the `Library` representing the current executable.
    ///
    /// [`Library::get`] calls of the returned `Library` will look for symbols in following
    /// locations in order:
    ///
    /// 1. The original program image;
    /// 2. Any executable object files (e.g. shared libraries) loaded at program startup;
    /// 3. Any executable object files loaded at runtime (e.g. via other `Library::new` calls or via
    ///    calls to the `dlopen` function).
    ///
    /// Note that the behaviour of a `Library` loaded with this method is different from that of
    /// Libraries loaded with [`os::windows::Library::this`].
    ///
    /// This is equivalent to <code>[Library::open](None, [RTLD_LAZY] | [RTLD_LOCAL])</code>.
    ///
    /// [`os::windows::Library::this`]: crate::os::windows::Library::this
    #[inline]
    pub fn this() -> Library {
        unsafe {
            // SAFE: this does not load any new shared library images, no danger in it executing
            // initialiser routines.
            Library::open(None::<&OsStr>, RTLD_LAZY | RTLD_LOCAL).expect("this should never fail")
        }
    }

    /// Find and load an executable object file (shared library).
    ///
    /// See documentation for [`Library::this`] for further description of the behaviour
    /// when the `filename` is `None`. Otherwise see [`Library::new`].
    ///
    /// Corresponds to `dlopen(filename, flags)`.
    ///
    /// # Safety
    ///
    /// When a library is loaded, initialisation routines contained within the library are executed.
    /// For the purposes of safety, the execution of these routines is conceptually the same calling an
    /// unknown foreign function and may impose arbitrary requirements on the caller for the call
    /// to be sound.
    ///
    /// Additionally, the callers of this function must also ensure that execution of the
    /// termination routines contained within the library is safe as well. These routines may be
    /// executed when the library is unloaded.
    pub unsafe fn open<P>(filename: Option<P>, flags: raw::c_int) -> Result<Library, crate::Error>
    where
        P: AsRef<OsStr>,
    {
        let filename = match filename {
            None => None,
            Some(ref f) => Some(cstr_cow_from_bytes(f.as_ref().as_bytes())?),
        };
        with_dlerror(
            move || {
                let result = dlopen(
                    match filename {
                        None => ptr::null(),
                        Some(ref f) => f.as_ptr(),
                    },
                    flags,
                );
                // ensure filename lives until dlopen completes
                drop(filename);
                if result.is_null() {
                    None
                } else {
                    Some(Library { handle: result })
                }
            },
            |desc| crate::Error::DlOpen { desc: desc.into() },
        )
        .map_err(|e| e.unwrap_or(crate::Error::DlOpenUnknown))
    }

    unsafe fn get_impl<T, F>(&self, symbol: &[u8], on_null: F) -> Result<Symbol<T>, crate::Error>
    where
        F: FnOnce() -> Result<Symbol<T>, crate::Error>,
    {
        ensure_compatible_types::<T, *mut raw::c_void>()?;
        let symbol = cstr_cow_from_bytes(symbol)?;
        // `dlsym` may return nullptr in two cases: when a symbol genuinely points to a null
        // pointer or the symbol cannot be found. In order to detect this case a double dlerror
        // pattern must be used, which is, sadly, a little bit racy.
        //
        // We try to leave as little space as possible for this to occur, but we can’t exactly
        // fully prevent it.
        let result = with_dlerror(
            || {
                dlerror();
                let symbol = dlsym(self.handle, symbol.as_ptr());
                if symbol.is_null() {
                    None
                } else {
                    Some(Symbol {
                        pointer: symbol,
                        pd: marker::PhantomData,
                    })
                }
            },
            |desc| crate::Error::DlSym { desc: desc.into() },
        );
        match result {
            Err(None) => on_null(),
            Err(Some(e)) => Err(e),
            Ok(x) => Ok(x),
        }
    }

    /// Get a pointer to a function or static variable by symbol name.
    ///
    /// The `symbol` may not contain any null bytes, with the exception of the last byte. Providing a
    /// null terminated `symbol` may help to avoid an allocation.
    ///
    /// Symbol is interpreted as-is; no mangling is done. This means that symbols like `x::y` are
    /// most likely invalid.
    ///
    /// # Safety
    ///
    /// Users of this API must specify the correct type of the function or variable loaded. Using a
    /// `Symbol` with a wrong type is undefined.
    ///
    /// # Platform-specific behaviour
    ///
    /// Implementation of thread local variables is extremely platform specific and uses of such
    /// variables that work on e.g. Linux may have unintended behaviour on other targets.
    ///
    /// On POSIX implementations where the `dlerror` function is not confirmed to be MT-safe (such
    /// as FreeBSD), this function will unconditionally return an error when the underlying `dlsym`
    /// call returns a null pointer. There are rare situations where `dlsym` returns a genuine null
    /// pointer without it being an error. If loading a null pointer is something you care about,
    /// consider using the [`Library::get_singlethreaded`] call.
    #[inline(always)]
    pub unsafe fn get<T>(&self, symbol: &[u8]) -> Result<Symbol<T>, crate::Error> {
        extern crate cfg_if;
        cfg_if::cfg_if! {
            // These targets are known to have MT-safe `dlerror`.
            if #[cfg(any(
                target_os = "linux",
                target_os = "android",
                target_os = "openbsd",
                target_os = "macos",
                target_os = "ios",
                target_os = "solaris",
                target_os = "illumos",
                target_os = "redox",
                target_os = "fuchsia",
                target_os = "cygwin",
            ))] {
                self.get_singlethreaded(symbol)
            } else {
                self.get_impl(symbol, || Err(crate::Error::DlSymUnknown))
            }
        }
    }

    /// Get a pointer to function or static variable by symbol name.
    ///
    /// The `symbol` may not contain any null bytes, with the exception of the last byte. Providing a
    /// null terminated `symbol` may help to avoid an allocation.
    ///
    /// Symbol is interpreted as-is; no mangling is done. This means that symbols like `x::y` are
    /// most likely invalid.
    ///
    /// # Safety
    ///
    /// Users of this API must specify the correct type of the function or variable loaded.
    ///
    /// It is up to the user of this library to ensure that no other calls to an MT-unsafe
    /// implementation of `dlerror` occur during the execution of this function. Failing that, the
    /// behaviour of this function is not defined.
    ///
    /// # Platform-specific behaviour
    ///
    /// The implementation of thread-local variables is extremely platform specific and uses of such
    /// variables that work on e.g. Linux may have unintended behaviour on other targets.
    #[inline(always)]
    pub unsafe fn get_singlethreaded<T>(&self, symbol: &[u8]) -> Result<Symbol<T>, crate::Error> {
        self.get_impl(symbol, || {
            Ok(Symbol {
                pointer: ptr::null_mut(),
                pd: marker::PhantomData,
            })
        })
    }

    /// Convert the `Library` to a raw handle.
    ///
    /// The handle returned by this function shall be usable with APIs which accept handles
    /// as returned by `dlopen`.
    pub fn into_raw(self) -> *mut raw::c_void {
        let handle = self.handle;
        mem::forget(self);
        handle
    }

    /// Convert a raw handle returned by `dlopen`-family of calls to a `Library`.
    ///
    /// # Safety
    ///
    /// The pointer shall be a result of a successful call of the `dlopen`-family of functions or a
    /// pointer previously returned by `Library::into_raw` call. It must be valid to call `dlclose`
    /// with this pointer as an argument.
    pub unsafe fn from_raw(handle: *mut raw::c_void) -> Library {
        Library { handle }
    }

    /// Unload the library.
    ///
    /// This method might be a no-op, depending on the flags with which the `Library` was opened,
    /// what library was opened or other platform specifics.
    ///
    /// You only need to call this if you are interested in handling any errors that may arise when
    /// library is unloaded. Otherwise the implementation of `Drop` for `Library` will close the
    /// library and ignore the errors were they arise.
    ///
    /// The underlying data structures may still get leaked if an error does occur.
    pub fn close(self) -> Result<(), crate::Error> {
        let result = with_dlerror(
            || {
                if unsafe { dlclose(self.handle) } == 0 {
                    Some(())
                } else {
                    None
                }
            },
            |desc| crate::Error::DlClose { desc: desc.into() },
        )
        .map_err(|e| e.unwrap_or(crate::Error::DlCloseUnknown));
        // While the library is not free'd yet in case of an error, there is no reason to try
        // dropping it again, because all that will do is try calling `dlclose` again. only
        // this time it would ignore the return result, which we already seen failing…
        std::mem::forget(self);
        result
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.handle);
        }
    }
}

impl fmt::Debug for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Library@{:p}", self.handle))
    }
}

/// Symbol from a library.
///
/// A major difference compared to the cross-platform `Symbol` is that this does not ensure that the
/// `Symbol` does not outlive the `Library` it comes from.
pub struct Symbol<T> {
    pointer: *mut raw::c_void,
    pd: marker::PhantomData<T>,
}

impl<T> Symbol<T> {
    /// Convert the loaded `Symbol` into a raw pointer.
    pub fn into_raw(self) -> *mut raw::c_void {
        self.pointer
    }

    /// Convert the loaded `Symbol` into a raw pointer.
    /// For unix this does the same as into_raw.
    pub fn as_raw_ptr(self) -> *mut raw::c_void {
        self.pointer
    }
}

impl<T> Symbol<Option<T>> {
    /// Lift Option out of the symbol.
    pub fn lift_option(self) -> Option<Symbol<T>> {
        if self.pointer.is_null() {
            None
        } else {
            Some(Symbol {
                pointer: self.pointer,
                pd: marker::PhantomData,
            })
        }
    }
}

unsafe impl<T: Send> Send for Symbol<T> {}
unsafe impl<T: Sync> Sync for Symbol<T> {}

impl<T> Clone for Symbol<T> {
    fn clone(&self) -> Symbol<T> {
        Symbol { ..*self }
    }
}

impl<T> ::std::ops::Deref for Symbol<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            // Additional reference level for a dereference on `deref` return value.
            &*(&self.pointer as *const *mut _ as *const T)
        }
    }
}

impl<T> fmt::Debug for Symbol<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut info = mem::MaybeUninit::<DlInfo>::uninit();
            if dladdr(self.pointer, info.as_mut_ptr()) != 0 {
                let info = info.assume_init();
                if info.dli_sname.is_null() {
                    f.write_str(&format!(
                        "Symbol@{:p} from {:?}",
                        self.pointer,
                        CStr::from_ptr(info.dli_fname)
                    ))
                } else {
                    f.write_str(&format!(
                        "Symbol {:?}@{:p} from {:?}",
                        CStr::from_ptr(info.dli_sname),
                        self.pointer,
                        CStr::from_ptr(info.dli_fname)
                    ))
                }
            } else {
                f.write_str(&format!("Symbol@{:p}", self.pointer))
            }
        }
    }
}

// Platform specific things
#[cfg_attr(any(target_os = "linux", target_os = "android"), link(name = "dl"))]
#[cfg_attr(any(target_os = "freebsd", target_os = "dragonfly"), link(name = "c"))]
extern "C" {
    fn dlopen(filename: *const raw::c_char, flags: raw::c_int) -> *mut raw::c_void;
    fn dlclose(handle: *mut raw::c_void) -> raw::c_int;
    fn dlsym(handle: *mut raw::c_void, symbol: *const raw::c_char) -> *mut raw::c_void;
    fn dlerror() -> *mut raw::c_char;
    fn dladdr(addr: *mut raw::c_void, info: *mut DlInfo) -> raw::c_int;
}

#[repr(C)]
struct DlInfo {
    dli_fname: *const raw::c_char,
    dli_fbase: *mut raw::c_void,
    dli_sname: *const raw::c_char,
    dli_saddr: *mut raw::c_void,
}
