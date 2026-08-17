use alloc::vec::Vec;
use core::convert::TryInto;
use core::mem;
use core::ptr;

use crate::vk;
pub type VkResult<T> = Result<T, vk::Result>;

impl vk::Result {
    #[inline]
    pub fn result(self) -> VkResult<()> {
        self.result_with_success(())
    }

    #[inline]
    pub fn result_with_success<T>(self, v: T) -> VkResult<T> {
        match self {
            Self::SUCCESS => Ok(v),
            _ => Err(self),
        }
    }

    #[inline]
    pub unsafe fn assume_init_on_success<T>(self, v: mem::MaybeUninit<T>) -> VkResult<T> {
        self.result().map(move |()| v.assume_init())
    }

    #[inline]
    pub unsafe fn set_vec_len_on_success<T>(self, mut v: Vec<T>, len: usize) -> VkResult<Vec<T>> {
        self.result().map(move |()| {
            v.set_len(len);
            v
        })
    }
}

/// Repeatedly calls `f` until it does not return [`vk::Result::INCOMPLETE`] anymore, ensuring all
/// available data has been read into the vector.
///
/// See for example [`vkEnumerateInstanceExtensionProperties`]: the number of available items may
/// change between calls; [`vk::Result::INCOMPLETE`] is returned when the count increased (and the
/// vector is not large enough after querying the initial size), requiring Ash to try again.
///
/// [`vkEnumerateInstanceExtensionProperties`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
pub(crate) unsafe fn read_into_uninitialized_vector<N: Copy + Default + TryInto<usize>, T>(
    f: impl Fn(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: core::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, ptr::null_mut()).result()?;
        let mut data =
            Vec::with_capacity(count.try_into().expect("`N` failed to convert to `usize`"));

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            break err_code.set_vec_len_on_success(
                data,
                count.try_into().expect("`N` failed to convert to `usize`"),
            );
        }
    }
}

/// Repeatedly calls `f` until it does not return [`vk::Result::INCOMPLETE`] anymore, ensuring all
/// available data has been read into the vector.
///
/// Items in the target vector are [`default()`][Default::default()]-initialized which is required
/// for [`vk::BaseOutStructure`]-like structs where [`vk::BaseOutStructure::s_type`] needs to be a
/// valid type and [`vk::BaseOutStructure::p_next`] a valid or [`null`][ptr::null_mut()]
/// pointer.
///
/// See for example [`vkEnumerateInstanceExtensionProperties`]: the number of available items may
/// change between calls; [`vk::Result::INCOMPLETE`] is returned when the count increased (and the
/// vector is not large enough after querying the initial size), requiring Ash to try again.
///
/// [`vkEnumerateInstanceExtensionProperties`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
pub(crate) unsafe fn read_into_defaulted_vector<
    N: Copy + Default + TryInto<usize>,
    T: Default + Clone,
>(
    f: impl Fn(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: core::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, ptr::null_mut()).result()?;
        let mut data = alloc::vec![Default::default(); count.try_into().expect("`N` failed to convert to `usize`")];

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            break err_code.set_vec_len_on_success(
                data,
                count.try_into().expect("`N` failed to convert to `usize`"),
            );
        }
    }
}

#[cfg(feature = "debug")]
pub(crate) fn debug_flags<Value: Into<u64> + Copy>(
    f: &mut core::fmt::Formatter<'_>,
    known: &[(Value, &'static str)],
    value: Value,
) -> core::fmt::Result {
    let mut first = true;
    let mut accum = value.into();
    for &(bit, name) in known {
        let bit = bit.into();
        if bit != 0 && accum & bit == bit {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            accum &= !bit;
        }
    }
    if accum != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{accum:b}")?;
    }
    Ok(())
}
