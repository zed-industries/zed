// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem::MaybeUninit;

use crate::{
    ffi,
    translate::{from_glib, IntoGlib, UnsafeFrom},
    UnicodeBreakType, UnicodeScript, UnicodeType,
};

mod sealed {
    pub trait Sealed {}
    impl Sealed for char {}
}

impl UnsafeFrom<u32> for char {
    #[inline]
    unsafe fn unsafe_from(t: u32) -> Self {
        debug_assert!(
            char::try_from(t).is_ok(),
            "glib returned an invalid Unicode codepoint"
        );
        unsafe { char::from_u32_unchecked(t) }
    }
}

// rustdoc-stripper-ignore-next
/// The kind of decomposition to perform
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum DecompositionKind {
    // rustdoc-stripper-ignore-next
    /// Compatibility decomposition
    Compatibility,

    // rustdoc-stripper-ignore-next
    /// Canonical decomposition
    Canonical,
}

// rustdoc-stripper-ignore-next
/// The result of a single step of the Unicode canonical decomposition algorithm
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CharacterDecomposition {
    // rustdoc-stripper-ignore-next
    /// The character could not be decomposed further
    NoDecomposition,
    // rustdoc-stripper-ignore-next
    // A 'singleton' decomposition, which means the character was replaced by another
    Singleton(char),
    // rustdoc-stripper-ignore-next
    /// The first character may decompose further, but the second cannot
    Pair(char, char),
}

// rustdoc-stripper-ignore-next
/// This trait provides access to Unicode character classification and manipulations functions
/// provided by GLib that do not exist in the standard library
#[doc(alias = "g_unichar")]
pub trait Unichar: sealed::Sealed + Copy + Into<u32> + UnsafeFrom<u32> {
    #[doc(alias = "g_unichar_type")]
    #[doc(alias = "unichar_type")]
    #[inline]
    fn unicode_type(self) -> UnicodeType {
        unsafe { from_glib(ffi::g_unichar_type(self.into())) }
    }

    #[doc(alias = "g_unichar_break_type")]
    #[doc(alias = "unichar_break_type")]
    #[inline]
    fn break_type(self) -> UnicodeBreakType {
        unsafe { from_glib(ffi::g_unichar_break_type(self.into())) }
    }

    #[doc(alias = "g_unichar_get_script")]
    #[doc(alias = "unichar_get_script")]
    #[inline]
    fn script(self) -> UnicodeScript {
        unsafe { from_glib(ffi::g_unichar_get_script(self.into())) }
    }

    #[doc(alias = "g_unichar_combining_class")]
    #[doc(alias = "unichar_combining_class")]
    #[inline]
    fn combining_class(self) -> u8 {
        // UAX #44 § 5.7.4: The character property invariants regarding Canonical_Combining_Class
        //                  guarantee that [...] all values used will be in the range 0..254.
        // So this cast is fine
        unsafe { ffi::g_unichar_combining_class(self.into()) as u8 }
    }

    #[doc(alias = "g_unichar_ismark")]
    #[doc(alias = "unichar_ismark")]
    #[inline]
    fn is_mark(self) -> bool {
        unsafe { from_glib(ffi::g_unichar_ismark(self.into())) }
    }

    #[doc(alias = "g_unichar_isgraph")]
    #[doc(alias = "unichar_isgraph")]
    #[inline]
    fn is_graphical(self) -> bool {
        unsafe { from_glib(ffi::g_unichar_isgraph(self.into())) }
    }

    #[doc(alias = "g_unichar_ispunct")]
    #[doc(alias = "unichar_ispunct")]
    #[inline]
    fn is_punctuation(self) -> bool {
        unsafe { from_glib(ffi::g_unichar_ispunct(self.into())) }
    }

    #[doc(alias = "g_unichar_istitle")]
    #[doc(alias = "unichar_istitle")]
    #[inline]
    fn is_titlecase(self) -> bool {
        unsafe { from_glib(ffi::g_unichar_istitle(self.into())) }
    }

    #[doc(alias = "g_unichar_isdefined")]
    #[doc(alias = "unichar_isdefined")]
    #[inline]
    fn is_defined(self) -> bool {
        unsafe { from_glib(ffi::g_unichar_isdefined(self.into())) }
    }

    #[doc(alias = "g_unichar_iswide")]
    #[doc(alias = "unichar_iswide")]
    #[inline]
    fn is_wide(self) -> bool {
        unsafe { from_glib(ffi::g_unichar_iswide(self.into())) }
    }

    #[doc(alias = "g_unichar_iswide_cjk")]
    #[doc(alias = "unichar_iswide_cjk")]
    #[inline]
    fn is_wide_cjk(self) -> bool {
        unsafe { from_glib(ffi::g_unichar_iswide_cjk(self.into())) }
    }

    #[doc(alias = "g_unichar_iszerowidth")]
    #[doc(alias = "unichar_iszerowidth")]
    #[inline]
    fn is_zero_width(self) -> bool {
        unsafe { from_glib(ffi::g_unichar_iszerowidth(self.into())) }
    }

    #[doc(alias = "g_unichar_totitle")]
    #[doc(alias = "unichar_totitle")]
    #[inline]
    fn to_titlecase(self) -> Self {
        unsafe { Self::unsafe_from(ffi::g_unichar_totitle(self.into())) }
    }

    #[doc(alias = "g_unichar_get_mirror_char")]
    #[doc(alias = "unichar_get_mirror_char")]
    #[inline]
    fn mirror_char(self) -> Option<Self> {
        // SAFETY: If g_unichar_get_mirror_char returns true, it will initialize `mirrored`
        unsafe {
            let mut mirrored = MaybeUninit::uninit();
            let res = from_glib(ffi::g_unichar_get_mirror_char(
                self.into(),
                mirrored.as_mut_ptr(),
            ));
            if res {
                Some(Self::unsafe_from(mirrored.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "g_unichar_fully_decompose")]
    #[doc(alias = "unichar_fully_decompose")]
    #[inline]
    fn fully_decompose(self, decomposition_kind: DecompositionKind) -> Vec<Self> {
        let compat = match decomposition_kind {
            DecompositionKind::Compatibility => true,
            DecompositionKind::Canonical => false,
        };
        let buffer_len = ffi::G_UNICHAR_MAX_DECOMPOSITION_LENGTH as usize;

        // SAFETY: We assume glib only ever writes valid Unicode codepoints in the provided buffer
        //         and that it does not lie about the
        unsafe {
            let mut buffer = Vec::<Self>::with_capacity(buffer_len);
            let decomposition_length = ffi::g_unichar_fully_decompose(
                self.into(),
                compat.into_glib(),
                buffer.as_mut_ptr().cast(),
                buffer_len,
            );
            debug_assert!(decomposition_length <= buffer_len);
            buffer.set_len(decomposition_length);
            buffer
        }
    }

    #[doc(alias = "g_unichar_decompose")]
    #[doc(alias = "unichar_decompose")]
    #[inline]
    fn decompose(self) -> CharacterDecomposition {
        // SAFETY: `a` and `b` will always be init after the g_unichar_decompose call returns
        unsafe {
            let mut a = MaybeUninit::uninit();
            let mut b = MaybeUninit::uninit();
            let res = from_glib(ffi::g_unichar_decompose(
                self.into(),
                a.as_mut_ptr(),
                b.as_mut_ptr(),
            ));

            if res {
                let (a, b) = (a.assume_init(), b.assume_init());
                if b == 0 {
                    CharacterDecomposition::Singleton(char::unsafe_from(a))
                } else {
                    CharacterDecomposition::Pair(char::unsafe_from(a), char::unsafe_from(b))
                }
            } else {
                CharacterDecomposition::NoDecomposition
            }
        }
    }

    #[doc(alias = "g_unichar_compose")]
    #[doc(alias = "unichar_compose")]
    #[inline]
    fn compose(a: char, b: char) -> Option<Self> {
        // SAFETY: If g_unichar_compose returns true, it will initialize `out`
        unsafe {
            let mut out = MaybeUninit::uninit();
            let res = from_glib(ffi::g_unichar_compose(a.into(), b.into(), out.as_mut_ptr()));

            if res {
                Some(Self::unsafe_from(out.assume_init()))
            } else {
                None
            }
        }
    }
}

impl Unichar for char {}
