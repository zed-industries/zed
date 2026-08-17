// implementations of bytemuck Zeroable and Pod traits for BigEndian and LittleEndian variants of
// types which are themselves Zeroable and Pod. This enables use of bytemuck::cast* methods to
// safely transmute to and from these types.

use crate::{BigEndian, LittleEndian, Primitive};
use bytemuck::{Pod, Zeroable};

// SAFETY: T and T::Storage is Zeroable and Pod. LittleEndian is repr(transparent)
// this satisfies the safety contracts of Zeroable and Pod.
#[cfg(feature = "bytemuck")]
unsafe impl<T> Zeroable for LittleEndian<T>
where
    T: Primitive + Zeroable,
    T::Storage: Zeroable,
{
}
#[cfg(feature = "bytemuck")]
unsafe impl<T> Pod for LittleEndian<T>
where
    T: Primitive + Pod + Copy + 'static,
    T::Storage: Pod,
{
}

// SAFETY: T and T::Storage is Zeroable and Pod. BigEndian is repr(transparent)
// this satisfies the safety contracts of Zeroable and Pod.
#[cfg(feature = "bytemuck")]
unsafe impl<T> Zeroable for BigEndian<T>
where
    T: Primitive + Zeroable,
    T::Storage: Zeroable,
{
}
#[cfg(feature = "bytemuck")]
unsafe impl<T> Pod for BigEndian<T>
where
    T: Primitive + Pod + Copy + 'static,
    T::Storage: Pod,
{
}

#[cfg(test)]
mod tests {
    use crate::*;
    use bytemuck::Pod;

    #[test]
    fn check_impl_pod() {
        fn assert_impl_pod<T: Pod>() {}

        assert_impl_pod::<u16_le>();
        assert_impl_pod::<u16_be>();
        assert_impl_pod::<u32_le>();
        assert_impl_pod::<u32_be>();
        assert_impl_pod::<u64_le>();
        assert_impl_pod::<u64_be>();

        assert_impl_pod::<i16_le>();
        assert_impl_pod::<i16_be>();
        assert_impl_pod::<i32_le>();
        assert_impl_pod::<i32_be>();
        assert_impl_pod::<i64_le>();
        assert_impl_pod::<i64_be>();

        assert_impl_pod::<f32_le>();
        assert_impl_pod::<f32_be>();
        assert_impl_pod::<f64_le>();
        assert_impl_pod::<f64_be>();

        // char is not Pod, so these types shouldn't be either
        //assert_impl_pod::<char_be>();
        //assert_impl_pod::<char_le>();
    }
}
