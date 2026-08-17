#[cfg(feature = "bytecheck")]
macro_rules! impl_rkyv {
    (@bytecheck $type:ty) => {
        // SAFETY: All bit patterns are valid for these primitive types.
        // https://docs.rs/bytecheck/0.8.1/src/bytecheck/lib.rs.html#352
        unsafe impl<C: Fallible +?Sized> rkyv::bytecheck::CheckBytes<C> for $type {
            #[inline]
            unsafe fn check_bytes(
                _value: *const Self,
                _: &mut C,
            ) -> Result<(), C::Error> {
                Ok(())
            }
        }
    };

    ($type:ty) => {
        impl_rkyv_derive!(@serialize $type);
        impl_rkyv_derive!(@archive_deserialize $type);
        impl_rkyv!(@bytecheck $type);
    };
}

#[cfg(not(feature = "bytecheck"))]
macro_rules! impl_rkyv {
    ($type:ty) => {
        impl_rkyv_derive!(@serialize $type);
        impl_rkyv_derive!(@archive_deserialize $type);
    };
}

macro_rules! impl_rkyv_derive {
    (@serialize $type:ty) => {
        impl<S: Fallible + ?Sized> Serialize<S> for $type {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }
    };

    (@archive_deserialize $type:ty) => {
        // SAFETY: All glam types have a fully defined data layout.
        unsafe impl rkyv::traits::NoUndef for $type {}
        // SAFETY: All glam types have a stable, well-defined layout that is identical on all
        // targets.
        unsafe impl rkyv::Portable for $type {}
        impl Archive for $type {
            type Archived = $type;
            type Resolver = ();

            #[inline]
            fn resolve(&self, _: Self::Resolver, out: Place<Self::Archived>) {
                out.write(*self)
            }
        }

        impl<D: Fallible + ?Sized> Deserialize<$type, D> for $type {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(*self)
            }
        }
    };
}

mod f32 {
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};
    impl_rkyv!(Affine2);
    impl_rkyv!(Affine3A);
    impl_rkyv!(Mat2);
    impl_rkyv!(Mat3);
    impl_rkyv!(Mat3A);
    impl_rkyv!(Mat4);
    impl_rkyv!(Quat);
    impl_rkyv!(Vec2);
    impl_rkyv!(Vec3);
    impl_rkyv!(Vec3A);
    impl_rkyv!(Vec4);
}

mod f64 {
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(DAffine2);
    impl_rkyv!(DAffine3);
    impl_rkyv!(DMat2);
    impl_rkyv!(DMat3);
    impl_rkyv!(DMat4);
    impl_rkyv!(DQuat);
    impl_rkyv!(DVec2);
    impl_rkyv!(DVec3);
    impl_rkyv!(DVec4);
}

mod i8 {
    use crate::{I8Vec2, I8Vec3, I8Vec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(I8Vec2);
    impl_rkyv!(I8Vec3);
    impl_rkyv!(I8Vec4);
}

mod i16 {
    use crate::{I16Vec2, I16Vec3, I16Vec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(I16Vec2);
    impl_rkyv!(I16Vec3);
    impl_rkyv!(I16Vec4);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(IVec2);
    impl_rkyv!(IVec3);
    impl_rkyv!(IVec4);
}

mod i64 {
    use crate::{I64Vec2, I64Vec3, I64Vec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(I64Vec2);
    impl_rkyv!(I64Vec3);
    impl_rkyv!(I64Vec4);
}

mod u8 {
    use crate::{U8Vec2, U8Vec3, U8Vec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(U8Vec2);
    impl_rkyv!(U8Vec3);
    impl_rkyv!(U8Vec4);
}

mod u16 {
    use crate::{U16Vec2, U16Vec3, U16Vec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(U16Vec2);
    impl_rkyv!(U16Vec3);
    impl_rkyv!(U16Vec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(UVec2);
    impl_rkyv!(UVec3);
    impl_rkyv!(UVec4);
}

mod u64 {
    use crate::{U64Vec2, U64Vec3, U64Vec4};
    use rkyv::{rancor::Fallible, Archive, Deserialize, Place, Serialize};

    impl_rkyv!(U64Vec2);
    impl_rkyv!(U64Vec3);
    impl_rkyv!(U64Vec4);
}

#[cfg(test)]
mod test {
    /// The serializer type expected by [`rkyv::to_bytes()`].
    pub type TestSerializer<'a> = rkyv::api::high::HighSerializer<
        rkyv::util::AlignedVec,
        rkyv::ser::allocator::ArenaHandle<'a>,
        rkyv::rancor::Panic,
    >;
    /// The deserializer type expected by [`rkyv::deserialize()`].
    pub type TestDeserializer = rkyv::api::high::HighDeserializer<rkyv::rancor::Panic>;
    pub fn test_archive<T>(value: &T)
    where
        T: core::fmt::Debug
            + PartialEq
            + rkyv::Portable
            + for<'a> rkyv::Serialize<TestSerializer<'a>>,
        T::Archived: core::fmt::Debug + PartialEq<T> + rkyv::Deserialize<T, TestDeserializer>,
    {
        let buffer = rkyv::to_bytes(value).unwrap();

        // SAFETY: all bit patterns are valid for the primitive types used by glam.  There is
        // no need to write special-cased conditional tests that rely on bytecheck for the safe
        // rkyv::access() wrapper.
        let archived_value = unsafe { rkyv::access_unchecked::<T::Archived>(&buffer) };
        assert_eq!(archived_value, value);
        assert_eq!(
            &rkyv::deserialize::<T, rkyv::rancor::Panic>(archived_value).unwrap(),
            value
        );
    }

    #[test]
    fn test_rkyv() {
        use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
        test_archive(&Affine2::from_cols_array(&[1.0, 0.0, 2.0, 0.0, 3.0, 4.0]));
        test_archive(&Affine3A::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
        ]));
        test_archive(&Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]));
        test_archive(&Mat3::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
        ]));
        test_archive(&Mat3A::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
        ]));
        test_archive(&Mat4::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]));
        test_archive(&Quat::from_xyzw(1.0, 2.0, 3.0, 4.0));
        test_archive(&Vec2::new(1.0, 2.0));
        test_archive(&Vec3::new(1.0, 2.0, 3.0));
        test_archive(&Vec3A::new(1.0, 2.0, 3.0));
        test_archive(&Vec4::new(1.0, 2.0, 3.0, 4.0));

        use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
        test_archive(&DAffine2::from_cols_array(&[1.0, 0.0, 2.0, 0.0, 3.0, 4.0]));
        test_archive(&DAffine3::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
        ]));
        test_archive(&DMat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]));
        test_archive(&DMat3::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
        ]));
        test_archive(&DMat4::from_cols_array(&[
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]));
        test_archive(&DQuat::from_xyzw(1.0, 2.0, 3.0, 4.0));
        test_archive(&DVec2::new(1.0, 2.0));
        test_archive(&DVec3::new(1.0, 2.0, 3.0));
        test_archive(&DVec4::new(1.0, 2.0, 3.0, 4.0));

        use crate::{I8Vec2, I8Vec3, I8Vec4};
        test_archive(&I8Vec2::new(-1, 2));
        test_archive(&I8Vec3::new(-1, 2, 3));
        test_archive(&I8Vec4::new(-1, 2, 3, 4));

        use crate::{I16Vec2, I16Vec3, I16Vec4};
        test_archive(&I16Vec2::new(-1, 2));
        test_archive(&I16Vec3::new(-1, 2, 3));
        test_archive(&I16Vec4::new(-1, 2, 3, 4));

        use crate::{IVec2, IVec3, IVec4};
        test_archive(&IVec2::new(-1, 2));
        test_archive(&IVec3::new(-1, 2, 3));
        test_archive(&IVec4::new(-1, 2, 3, 4));

        use crate::{I64Vec2, I64Vec3, I64Vec4};
        test_archive(&I64Vec2::new(-1, 2));
        test_archive(&I64Vec3::new(-1, 2, 3));
        test_archive(&I64Vec4::new(-1, 2, 3, 4));

        use crate::{U8Vec2, U8Vec3, U8Vec4};
        test_archive(&U8Vec2::new(1, 2));
        test_archive(&U8Vec3::new(1, 2, 3));
        test_archive(&U8Vec4::new(1, 2, 3, 4));

        use crate::{U16Vec2, U16Vec3, U16Vec4};
        test_archive(&U16Vec2::new(1, 2));
        test_archive(&U16Vec3::new(1, 2, 3));
        test_archive(&U16Vec4::new(1, 2, 3, 4));

        use crate::{UVec2, UVec3, UVec4};
        test_archive(&UVec2::new(1, 2));
        test_archive(&UVec3::new(1, 2, 3));
        test_archive(&UVec4::new(1, 2, 3, 4));

        use crate::{U64Vec2, U64Vec3, U64Vec4};
        test_archive(&U64Vec2::new(1, 2));
        test_archive(&U64Vec3::new(1, 2, 3));
        test_archive(&U64Vec4::new(1, 2, 3, 4));
    }
}
