use core::hash::BuildHasher;
use core::hash::Hash;
use core::hash::Hasher;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

#[cfg(feature = "specialize")]
use crate::BuildHasherExt;
#[cfg(feature = "specialize")]
use alloc::string::String;
#[cfg(feature = "specialize")]
use alloc::vec::Vec;

/// Provides a way to get an optimized hasher for a given data type.
/// Rather than using a Hasher generically which can hash any value, this provides a way to get a specialized hash
/// for a specific type. So this may be faster for primitive types.
/// # Example
/// ```
/// use std::hash::BuildHasher;
/// use ahash::RandomState;
/// use ahash::CallHasher;
///
/// let hash_builder = RandomState::new();
/// //...
/// let value: u32 = 17;
/// let hash = u32::get_hash(&value, &hash_builder);
/// ```
/// Note that the type used to invoke `get_hash` must be the same a the type of value passed.
/// For example get a hasher specialized on `[u8]` can invoke:
/// ```
/// /// use std::hash::BuildHasher;
/// # use ahash::RandomState;
/// # use ahash::CallHasher;
/// # let hash_builder = RandomState::new();
/// let bytes: [u8; 4] = [1, 2, 3, 4];
/// let hash = <[u8]>::get_hash(&bytes, &hash_builder);
/// ```
pub trait CallHasher {
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64;
}

#[cfg(not(feature = "specialize"))]
impl<T> CallHasher for T
where
    T: Hash + ?Sized,
{
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        let mut hasher = build_hasher.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl<T> CallHasher for T
where
    T: Hash + ?Sized,
{
    #[inline]
    default fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        let mut hasher = build_hasher.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

macro_rules! call_hasher_impl {
    ($typ:ty) => {
        #[cfg(feature = "specialize")]
        impl CallHasher for $typ {
            #[inline]
            fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
                build_hasher.hash_as_u64(value)
            }
        }
    };
}
call_hasher_impl!(u8);
call_hasher_impl!(u16);
call_hasher_impl!(u32);
call_hasher_impl!(u64);
call_hasher_impl!(i8);
call_hasher_impl!(i16);
call_hasher_impl!(i32);
call_hasher_impl!(i64);

#[cfg(feature = "specialize")]
impl CallHasher for u128 {
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        build_hasher.hash_as_fixed_length(value)
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for i128 {
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        build_hasher.hash_as_fixed_length(value)
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for usize {
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        build_hasher.hash_as_fixed_length(value)
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for isize {
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        build_hasher.hash_as_fixed_length(value)
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for [u8] {
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        build_hasher.hash_as_str(value)
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for Vec<u8> {
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        build_hasher.hash_as_str(value)
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for str {
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        build_hasher.hash_as_str(value)
    }
}

#[cfg(all(feature = "specialize"))]
impl CallHasher for String {
    #[inline]
    fn get_hash<H: Hash + ?Sized, B: BuildHasher>(value: &H, build_hasher: &B) -> u64 {
        build_hasher.hash_as_str(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    #[cfg(feature = "specialize")]
    pub fn test_specialized_invoked() {
        let build_hasher = RandomState::with_seeds(1, 2, 3, 4);
        let shortened = u64::get_hash(&0, &build_hasher);
        let mut hasher = AHasher::new_with_keys(1, 2);
        0_u64.hash(&mut hasher);
        assert_ne!(hasher.finish(), shortened);
    }

    /// Tests that some non-trivial transformation takes place.
    #[test]
    pub fn test_input_processed() {
        let build_hasher = RandomState::with_seeds(2, 2, 2, 2);
        assert_ne!(0, u64::get_hash(&0, &build_hasher));
        assert_ne!(1, u64::get_hash(&0, &build_hasher));
        assert_ne!(2, u64::get_hash(&0, &build_hasher));
        assert_ne!(3, u64::get_hash(&0, &build_hasher));
        assert_ne!(4, u64::get_hash(&0, &build_hasher));
        assert_ne!(5, u64::get_hash(&0, &build_hasher));

        assert_ne!(0, u64::get_hash(&1, &build_hasher));
        assert_ne!(1, u64::get_hash(&1, &build_hasher));
        assert_ne!(2, u64::get_hash(&1, &build_hasher));
        assert_ne!(3, u64::get_hash(&1, &build_hasher));
        assert_ne!(4, u64::get_hash(&1, &build_hasher));
        assert_ne!(5, u64::get_hash(&1, &build_hasher));

        let xored = u64::get_hash(&0, &build_hasher) ^ u64::get_hash(&1, &build_hasher);
        assert_ne!(0, xored);
        assert_ne!(1, xored);
        assert_ne!(2, xored);
        assert_ne!(3, xored);
        assert_ne!(4, xored);
        assert_ne!(5, xored);
    }

    #[test]
    pub fn test_ref_independent() {
        let build_hasher = RandomState::with_seeds(1, 2, 3, 4);
        assert_eq!(u8::get_hash(&&1, &build_hasher), u8::get_hash(&1, &build_hasher));
        assert_eq!(u16::get_hash(&&2, &build_hasher), u16::get_hash(&2, &build_hasher));
        assert_eq!(u32::get_hash(&&3, &build_hasher), u32::get_hash(&3, &build_hasher));
        assert_eq!(u64::get_hash(&&4, &build_hasher), u64::get_hash(&4, &build_hasher));
        assert_eq!(u128::get_hash(&&5, &build_hasher), u128::get_hash(&5, &build_hasher));
        assert_eq!(
            str::get_hash(&"test", &build_hasher),
            str::get_hash("test", &build_hasher)
        );
        assert_eq!(
            str::get_hash(&"test", &build_hasher),
            String::get_hash(&"test".to_string(), &build_hasher)
        );
        #[cfg(feature = "specialize")]
        assert_eq!(
            str::get_hash(&"test", &build_hasher),
            <[u8]>::get_hash("test".as_bytes(), &build_hasher)
        );

        let build_hasher = RandomState::with_seeds(10, 20, 30, 40);
        assert_eq!(u8::get_hash(&&&1, &build_hasher), u8::get_hash(&1, &build_hasher));
        assert_eq!(u16::get_hash(&&&2, &build_hasher), u16::get_hash(&2, &build_hasher));
        assert_eq!(u32::get_hash(&&&3, &build_hasher), u32::get_hash(&3, &build_hasher));
        assert_eq!(u64::get_hash(&&&4, &build_hasher), u64::get_hash(&4, &build_hasher));
        assert_eq!(u128::get_hash(&&&5, &build_hasher), u128::get_hash(&5, &build_hasher));
        assert_eq!(
            str::get_hash(&&"test", &build_hasher),
            str::get_hash("test", &build_hasher)
        );
        assert_eq!(
            str::get_hash(&&"test", &build_hasher),
            String::get_hash(&"test".to_string(), &build_hasher)
        );
        #[cfg(feature = "specialize")]
        assert_eq!(
            str::get_hash(&&"test", &build_hasher),
            <[u8]>::get_hash(&"test".to_string().into_bytes(), &build_hasher)
        );
    }
}
