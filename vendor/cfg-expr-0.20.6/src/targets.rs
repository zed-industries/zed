use crate::error::{HasAtomicParseError, Reason};
use std::{borrow::Cow, ops::Deref};

mod builtins;

/// A list of all of the [builtin](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/spec/index.html#modules)
/// targets known to rustc, as of 1.54.0
pub use builtins::ALL_BUILTINS;

/// The unique identifier for a target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Triple(pub Cow<'static, str>);

/// The "abi" field
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Abi(pub Cow<'static, str>);

/// The "architecture" field
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Arch(pub Cow<'static, str>);

/// The "vendor" field, which in practice is little more than an arbitrary modifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vendor(pub Cow<'static, str>);

/// The "operating system" field, which sometimes implies an environment, and
/// sometimes isn't an actual operating system.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Os(pub Cow<'static, str>);

/// Individual target families, which describe a set of targets grouped in some logical manner,
/// typically by operating system. This includes values like `unix` and `windows`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Family(pub Cow<'static, str>);

/// The "environment" field, which specifies an ABI environment on top of the
/// operating system. In many configurations, this field is omitted, and the
/// environment is implied by the operating system.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Env(pub Cow<'static, str>);

/// The panic strategy used on this target by default.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Panic(pub Cow<'static, str>);

macro_rules! field_impls {
    ($kind:ident) => {
        impl $kind {
            /// Constructs a new instance of this field.
            ///
            /// This method accepts both owned `String`s and `&'static str`s.
            #[inline]
            pub fn new(val: impl Into<Cow<'static, str>>) -> Self {
                Self(val.into())
            }

            /// Constructs a new instance of this field from a `&'static str`.
            #[inline]
            pub const fn new_const(val: &'static str) -> Self {
                Self(Cow::Borrowed(val))
            }

            /// Returns the string for the field.
            #[inline]
            pub fn as_str(&self) -> &str {
                &*self.0
            }
        }

        impl AsRef<str> for $kind {
            #[inline]
            fn as_ref(&self) -> &str {
                &*self.0
            }
        }

        impl std::fmt::Display for $kind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

field_impls!(Triple);
field_impls!(Abi);
field_impls!(Arch);
field_impls!(Vendor);
field_impls!(Os);
field_impls!(Family);
field_impls!(Env);
field_impls!(Panic);

/// Integer size and pointers for which there's support for atomic functions.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum HasAtomic {
    /// The platform supports atomics for the given integer size in bits (e.g. `AtomicU8` if
    /// `HasAtomic::IntegerSize(8)`).
    IntegerSize(u16),

    /// The platform supports atomics for pointers (`AtomicPtr`).
    Pointer,
}

impl std::str::FromStr for HasAtomic {
    type Err = HasAtomicParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(size) = s.parse::<u16>() {
            Ok(Self::IntegerSize(size))
        } else if s == "ptr" {
            Ok(HasAtomic::Pointer)
        } else {
            Err(HasAtomicParseError {
                input: s.to_owned(),
            })
        }
    }
}

impl std::fmt::Display for HasAtomic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IntegerSize(size) => write!(f, "{size}"),
            Self::Pointer => write!(f, "ptr"),
        }
    }
}

/// A set of families for a target.
///
/// Each target can be part of one or more families. This struct represents them.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Families(Cow<'static, [Family]>);

impl Families {
    /// Constructs a new instance.
    ///
    /// This method accepts both owned `String`s and `&'static str`s.
    ///
    /// If you have a `&'static [&'static str]`, prefer [`Self::new_const`].
    #[inline]
    pub fn new(val: impl IntoIterator<Item = Family>) -> Self {
        let mut fams: Vec<_> = val.into_iter().collect();
        fams.sort_unstable();
        Self(Cow::Owned(fams))
    }

    /// Constructs a new instance of this field from a static slice of `&'static str`.
    ///
    /// `val` must be in sorted order: this constructor cannot check for that due to
    /// limitations in current versions of Rust.
    #[inline]
    pub const fn new_const(val: &'static [Family]) -> Self {
        // TODO: Check that val is sorted.
        Self(Cow::Borrowed(val))
    }

    /// Returns true if this list of families contains a given family.
    #[inline]
    pub fn contains(&self, val: &Family) -> bool {
        self.0.contains(val)
    }
}

impl Deref for Families {
    type Target = [Family];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[Family]> for Families {
    #[inline]
    fn as_ref(&self) -> &[Family] {
        &self.0
    }
}

impl std::fmt::Display for Families {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let len = self.0.len();
        for (idx, family) in self.0.iter().enumerate() {
            write!(f, "{family}")?;
            if idx + 1 < len {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

/// A set of [`HasAtomic`] instances a target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HasAtomics(Cow<'static, [HasAtomic]>);

impl HasAtomics {
    /// Constructs a new instance.
    ///
    /// If you have a `&'static [HasAtomic]`, prefer [`Self::new_const`].
    #[inline]
    pub fn new(val: impl IntoIterator<Item = HasAtomic>) -> Self {
        let mut has_atomics: Vec<_> = val.into_iter().collect();
        has_atomics.sort_unstable();
        Self(Cow::Owned(has_atomics))
    }

    /// Constructs a new instance of this struct from a static slice of [`HasAtomic`].
    ///
    /// `val` must be in sorted order: this constructor cannot check for that due to
    /// limitations in current versions of Rust.
    #[inline]
    pub const fn new_const(val: &'static [HasAtomic]) -> Self {
        // TODO: Check that val is sorted.
        Self(Cow::Borrowed(val))
    }

    /// Returns true if this list of families contains a given family.
    #[inline]
    pub fn contains(&self, val: HasAtomic) -> bool {
        self.0.contains(&val)
    }
}

impl Deref for HasAtomics {
    type Target = [HasAtomic];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[HasAtomic]> for HasAtomics {
    #[inline]
    fn as_ref(&self) -> &[HasAtomic] {
        &self.0
    }
}

impl std::fmt::Display for HasAtomics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let len = self.0.len();
        for (idx, has_atomic) in self.0.iter().enumerate() {
            write!(f, "{has_atomic}")?;
            if idx + 1 < len {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

macro_rules! target_enum {
    (
        $(#[$outer:meta])*
        pub enum $kind:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $name:ident $(= $value:expr)?,
            )+
        }
    ) => {
        $(#[$outer])*
        #[allow(non_camel_case_types)]
        pub enum $kind {
            $(
                $(#[$inner $($args)*])*
                $name $(= $value)?,
            )+
        }

        impl_from_str! {
            $kind {
                $(
                    $(#[$inner $($args)*])*
                    $name $(= $value)?,
                )+
            }
        }
    };
}

macro_rules! impl_from_str {
    (
        $kind:ident {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $name:ident $(= $value:expr)?,
            )+
        }
    ) => {
        impl std::str::FromStr for $kind {
            type Err = Reason;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($name) => Ok(Self::$name),)+
                    _ => Err(Reason::Unexpected(&[$(stringify!($name),)+])),
                }
            }
        }
    };
}

target_enum! {
    /// The endian types known to rustc
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum Endian {
        big,
        little,
    }
}

/// Contains information regarding a particular target known to rustc
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetInfo {
    /// The target's unique identifier
    pub triple: Triple,
    /// The target's operating system, if any. Used by the
    /// [target_os](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os)
    /// predicate.
    pub os: Option<Os>,
    /// The target's ABI, if any. Used by the
    /// [target_abi](https://github.com/rust-lang/rust/issues/80970) predicate.
    pub abi: Option<Abi>,
    /// The target's CPU architecture. Used by the
    /// [target_arch](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch)
    /// predicate.
    pub arch: Arch,
    /// The target's ABI/libc used, if any. Used by the
    /// [target_env](https://doc.rust-lang.org/reference/conditional-compilation.html#target_env)
    /// predicate.
    pub env: Option<Env>,
    /// The target's vendor, if any. Used by the
    /// [target_vendor](https://doc.rust-lang.org/reference/conditional-compilation.html#target_vendor)
    /// predicate.
    pub vendor: Option<Vendor>,
    /// The target's families, if any. Used by the
    /// [target_family](https://doc.rust-lang.org/reference/conditional-compilation.html#target_family)
    /// predicate.
    pub families: Families,
    /// The size of the target's pointer type. Used by the
    /// [target_pointer_width](https://doc.rust-lang.org/reference/conditional-compilation.html#target_pointer_width)
    /// predicate.
    pub pointer_width: u8,
    /// The target's endianness. Used by the
    /// [target_endian](https://doc.rust-lang.org/reference/conditional-compilation.html#target_endian)
    /// predicate.
    pub endian: Endian,
    /// The target's support for atomics. Used by the `has_target_atomics` predicate.
    pub has_atomics: HasAtomics,
    /// The panic strategy used on this target by default. Used by the
    /// [panic](https://doc.rust-lang.org/beta/reference/conditional-compilation.html#panic) predicate.
    pub panic: Panic,
}

/// Attempts to find the `TargetInfo` for the specified target triple
///
/// ```
/// assert!(cfg_expr::targets::get_builtin_target_by_triple("x86_64-unknown-linux-musl").is_some());
/// ```
pub fn get_builtin_target_by_triple(triple: &str) -> Option<&'static TargetInfo> {
    ALL_BUILTINS
        .binary_search_by(|ti| ti.triple.as_ref().cmp(triple))
        .map(|i| &ALL_BUILTINS[i])
        .ok()
}

/// Retrieves the version of rustc for which the built-in targets were
/// retrieved from.
///
/// Targets may be added and removed between different rustc versions.
pub fn rustc_version() -> &'static str {
    builtins::RUSTC_VERSION
}

#[cfg(test)]
mod test {
    use crate::targets::get_builtin_target_by_triple;
    use std::collections::{BTreeSet, HashSet};

    // rustc's target-list is currently sorted lexicographically
    // by the target-triple, so ensure that stays the case
    #[test]
    fn targets_are_sorted() {
        for window in super::ALL_BUILTINS.windows(2) {
            assert!(window[0].triple < window[1].triple);
        }
    }

    // Ensure our workaround for https://github.com/rust-lang/rust/issues/36156
    // still functions
    #[test]
    fn has_ios() {
        assert_eq!(
            8,
            super::ALL_BUILTINS
                .iter()
                .filter(|ti| ti.os == Some(super::Os::ios))
                .count()
        );
    }

    // Ensure that TargetInfo can be used as keys for btree and hash-based data structures.
    #[test]
    fn set_map_key() {
        let target_info =
            get_builtin_target_by_triple("x86_64-unknown-linux-gnu").expect("known target");

        let mut btree_set = BTreeSet::new();
        btree_set.insert(target_info);

        let mut hash_set = HashSet::new();
        hash_set.insert(target_info);
    }

    #[test]
    fn family_comp() {
        let a = super::Families::new([super::Family::unix, super::Family::wasm]);
        let b = super::Families::new([super::Family::wasm, super::Family::unix]);

        assert_eq!(a, b);
    }
}
