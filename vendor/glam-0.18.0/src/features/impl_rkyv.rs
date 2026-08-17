#[cfg(feature = "bytecheck")]
macro_rules! impl_rkyv {
    (@bytecheck $type:ty) => {
        impl<C: ?Sized> bytecheck::CheckBytes<C> for $type {
            type Error = core::convert::Infallible;

            #[inline]
            unsafe fn check_bytes<'a>(
                value: *const Self,
                _: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                Ok(&*value)
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
        impl Archive for $type {
            type Archived = $type;
            type Resolver = ();

            #[inline]
            unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
                out.write(to_archived!(*self as Self));
            }
        }

        impl<D: Fallible + ?Sized> Deserialize<$type, D> for $type {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(from_archived!(*self))
            }
        }
    };
}

mod f32 {
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use rkyv::{from_archived, to_archived, Archive, Deserialize, Fallible, Serialize};
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
    use rkyv::{from_archived, to_archived, Archive, Deserialize, Fallible, Serialize};

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

mod i32 {
    use crate::{IVec2, IVec3, IVec4};
    use rkyv::{from_archived, to_archived, Archive, Deserialize, Fallible, Serialize};

    impl_rkyv!(IVec2);
    impl_rkyv!(IVec3);
    impl_rkyv!(IVec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};
    use rkyv::{from_archived, to_archived, Archive, Deserialize, Fallible, Serialize};

    impl_rkyv!(UVec2);
    impl_rkyv!(UVec3);
    impl_rkyv!(UVec4);
}

#[cfg(test)]
mod test {
    pub type DefaultSerializer = rkyv::ser::serializers::CoreSerializer<256, 256>;
    pub type DefaultDeserializer = rkyv::Infallible;
    use rkyv::ser::Serializer;
    use rkyv::*;
    pub fn test_archive<T>(value: &T)
    where
        T: core::fmt::Debug + PartialEq + rkyv::Serialize<DefaultSerializer>,
        T::Archived: core::fmt::Debug + PartialEq<T> + rkyv::Deserialize<T, DefaultDeserializer>,
    {
        let mut serializer = DefaultSerializer::default();
        serializer
            .serialize_value(value)
            .expect("failed to archive value");
        let len = serializer.pos();
        let buffer = serializer.into_serializer().into_inner();

        let archived_value = unsafe { rkyv::archived_root::<T>(&buffer[0..len]) };
        assert_eq!(archived_value, value);
        let mut deserializer = DefaultDeserializer::default();
        assert_eq!(
            &archived_value.deserialize(&mut deserializer).unwrap(),
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

        use crate::{IVec2, IVec3, IVec4};
        test_archive(&IVec2::new(-1, 2));
        test_archive(&IVec3::new(-1, 2, 3));
        test_archive(&IVec4::new(-1, 2, 3, 4));

        use crate::{UVec2, UVec3, UVec4};
        test_archive(&UVec2::new(1, 2));
        test_archive(&UVec3::new(1, 2, 3));
        test_archive(&UVec4::new(1, 2, 3, 4));
    }
}
