use crate::actually_private::Private;
use crate::lossy;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::string::String;
use core::cmp::Ordering;
use core::ffi::{c_char, CStr};
use core::fmt::{self, Debug, Display};
use core::hash::{Hash, Hasher};
use core::marker::{PhantomData, PhantomPinned};
use core::mem::MaybeUninit;
use core::pin::Pin;
use core::slice;
use core::str::{self, Utf8Error};

extern "C" {
    #[link_name = "cxxbridge1$cxx_string$init"]
    fn string_init(this: &mut MaybeUninit<CxxString>, ptr: *const u8, len: usize);
    #[link_name = "cxxbridge1$cxx_string$destroy"]
    fn string_destroy(this: &mut MaybeUninit<CxxString>);
    #[link_name = "cxxbridge1$cxx_string$data"]
    fn string_data(this: &CxxString) -> *const u8;
    #[link_name = "cxxbridge1$cxx_string$length"]
    fn string_length(this: &CxxString) -> usize;
    #[link_name = "cxxbridge1$cxx_string$clear"]
    fn string_clear(this: Pin<&mut CxxString>);
    #[link_name = "cxxbridge1$cxx_string$reserve_total"]
    fn string_reserve_total(this: Pin<&mut CxxString>, new_cap: usize);
    #[link_name = "cxxbridge1$cxx_string$push"]
    fn string_push(this: Pin<&mut CxxString>, ptr: *const u8, len: usize);
}

/// Binding to C++ `std::string`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `CxxString` by value. C++'s string
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a CxxString through a reference or smart pointer, as in `&CxxString`
/// or `UniquePtr<CxxString>`.
#[repr(C)]
pub struct CxxString {
    _private: [u8; 0],
    _pinned: PhantomData<PhantomPinned>,
}

/// Construct a C++ std::string on the Rust stack.
///
/// # Syntax
///
/// In statement position:
///
/// ```
/// # use cxx::let_cxx_string;
/// # let expression = "";
/// let_cxx_string!(var = expression);
/// ```
///
/// The `expression` may have any type that implements `AsRef<[u8]>`. Commonly
/// it will be a string literal, but for example `&[u8]` and `String` would work
/// as well.
///
/// The macro expands to something resembling `let $var: Pin<&mut CxxString> =
/// /*???*/;`. The resulting [`Pin`] can be deref'd to `&CxxString` as needed.
///
/// # Example
///
/// ```
/// use cxx::{let_cxx_string, CxxString};
///
/// fn f(s: &CxxString) {/* ... */}
///
/// fn main() {
///     let_cxx_string!(s = "example");
///     f(&s);
/// }
/// ```
#[macro_export]
macro_rules! let_cxx_string {
    ($var:ident = $value:expr $(,)?) => {
        let mut cxx_stack_string = $crate::private::StackString::new();
        #[allow(unused_mut, unused_unsafe)]
        let mut $var = match $value {
            let_cxx_string => unsafe { cxx_stack_string.init(let_cxx_string) },
        };
    };
}

impl CxxString {
    /// `CxxString` is not constructible via `new`. Instead, use the
    /// [`let_cxx_string!`] macro.
    pub fn new<T: Private>() -> Self {
        unreachable!()
    }

    /// Returns the length of the string in bytes.
    ///
    /// Matches the behavior of C++ [std::string::size][size].
    ///
    /// [size]: https://en.cppreference.com/w/cpp/string/basic_string/size
    pub fn len(&self) -> usize {
        unsafe { string_length(self) }
    }

    /// Returns true if `self` has a length of zero bytes.
    ///
    /// Matches the behavior of C++ [std::string::empty][empty].
    ///
    /// [empty]: https://en.cppreference.com/w/cpp/string/basic_string/empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a byte slice of this string's contents.
    pub fn as_bytes(&self) -> &[u8] {
        let data = self.as_ptr();
        let len = self.len();
        unsafe { slice::from_raw_parts(data, len) }
    }

    /// Produces a pointer to the first character of the string.
    ///
    /// Matches the behavior of C++ [std::string::data][data].
    ///
    /// Note that the return type may look like `const char *` but is not a
    /// `const char *` in the typical C sense, as C++ strings may contain
    /// internal null bytes. As such, the returned pointer only makes sense as a
    /// string in combination with the length returned by [`len()`][len].
    ///
    /// Modifying the string data through this pointer has undefined behavior.
    ///
    /// [data]: https://en.cppreference.com/w/cpp/string/basic_string/data
    /// [len]: #method.len
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { string_data(self) }
    }

    /// Produces a nul-terminated string view of this string's contents.
    ///
    /// Matches the behavior of C++ [std::string::c_str][c_str].
    ///
    /// If this string contains no internal '\0' bytes, then
    /// `self.as_c_str().count_bytes() == self.len()`. But if it does, the CStr
    /// only refers to the part of the string up to the first nul byte.
    ///
    /// [c_str]: https://en.cppreference.com/w/cpp/string/basic_string/c_str
    pub fn as_c_str(&self) -> &CStr {
        // Since C++11, string[string.size()] is guaranteed to be \0.
        unsafe { CStr::from_ptr(self.as_ptr().cast::<c_char>()) }
    }

    /// Validates that the C++ string contains UTF-8 data and produces a view of
    /// it as a Rust &amp;str, otherwise an error.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.as_bytes())
    }

    /// If the contents of the C++ string are valid UTF-8, this function returns
    /// a view as a Cow::Borrowed &amp;str. Otherwise replaces any invalid UTF-8
    /// sequences with the U+FFFD [replacement character] and returns a
    /// Cow::Owned String.
    ///
    /// [replacement character]: char::REPLACEMENT_CHARACTER
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.as_bytes())
    }

    /// Removes all characters from the string.
    ///
    /// Matches the behavior of C++ [std::string::clear][clear].
    ///
    /// Note: **unlike** the guarantee of Rust's `std::string::String::clear`,
    /// the C++ standard does not require that capacity is unchanged by this
    /// operation. In practice existing implementations do not change the
    /// capacity but all pointers, references, and iterators into the string
    /// contents are nevertheless invalidated.
    ///
    /// [clear]: https://en.cppreference.com/w/cpp/string/basic_string/clear
    pub fn clear(self: Pin<&mut Self>) {
        unsafe { string_clear(self) }
    }

    /// Ensures that this string's capacity is at least `additional` bytes
    /// larger than its length.
    ///
    /// The capacity may be increased by more than `additional` bytes if the
    /// implementation chooses, to amortize the cost of frequent reallocations.
    ///
    /// **The meaning of the argument is not the same as
    /// [std::string::reserve][reserve] in C++.** The C++ standard library and
    /// Rust standard library both have a `reserve` method on strings, but in
    /// C++ code the argument always refers to total capacity, whereas in Rust
    /// code it always refers to additional capacity. This API on `CxxString`
    /// follows the Rust convention, the same way that for the length accessor
    /// we use the Rust conventional `len()` naming and not C++ `size()` or
    /// `length()`.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows usize.
    ///
    /// [reserve]: https://en.cppreference.com/w/cpp/string/basic_string/reserve
    pub fn reserve(self: Pin<&mut Self>, additional: usize) {
        let new_cap = self
            .len()
            .checked_add(additional)
            .expect("CxxString capacity overflow");
        unsafe { string_reserve_total(self, new_cap) }
    }

    /// Appends a given string slice onto the end of this C++ string.
    pub fn push_str(self: Pin<&mut Self>, s: &str) {
        self.push_bytes(s.as_bytes());
    }

    /// Appends arbitrary bytes onto the end of this C++ string.
    pub fn push_bytes(self: Pin<&mut Self>, bytes: &[u8]) {
        unsafe { string_push(self, bytes.as_ptr(), bytes.len()) }
    }
}

impl Display for CxxString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        lossy::display(self.as_bytes(), f)
    }
}

impl Debug for CxxString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        lossy::debug(self.as_bytes(), f)
    }
}

impl PartialEq for CxxString {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<CxxString> for str {
    fn eq(&self, other: &CxxString) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<str> for CxxString {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for CxxString {}

impl PartialOrd for CxxString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CxxString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl Hash for CxxString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}

impl fmt::Write for Pin<&mut CxxString> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.as_mut().push_str(s);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Pin<&mut CxxString> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.as_mut().push_bytes(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct StackString {
    // Static assertions in cxx.cc validate that this is large enough and
    // aligned enough.
    space: MaybeUninit<[usize; 8]>,
}

impl StackString {
    pub fn new() -> Self {
        StackString {
            space: MaybeUninit::uninit(),
        }
    }

    pub unsafe fn init(&mut self, value: impl AsRef<[u8]>) -> Pin<&mut CxxString> {
        let value = value.as_ref();
        unsafe {
            let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<CxxString>>();
            string_init(this, value.as_ptr(), value.len());
            Pin::new_unchecked(&mut *this.as_mut_ptr())
        }
    }
}

impl Drop for StackString {
    fn drop(&mut self) {
        unsafe {
            let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<CxxString>>();
            string_destroy(this);
        }
    }
}
