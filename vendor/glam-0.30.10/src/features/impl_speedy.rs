use {
    crate::{
        Affine2, Affine3A, BVec2, BVec3, BVec4, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat,
        DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat3A, Mat4, Quat, UVec2, UVec3,
        UVec4, Vec2, Vec3, Vec3A, Vec4,
    },
    speedy::{Context, Readable, Reader, Writable, Writer},
};

macro_rules! impl_for_vec {
    ($T:ty, $ctor:ident, $comp_ty:ty, $comp_read_fn:ident, $comp_write_fn:ident, $($comp:ident),+) => {
        impl<'a, C> Readable<'a, C> for $T
        where
            C: Context,
        {
            #[inline]
            fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
                $(
                    let $comp = reader.$comp_read_fn()?;
                )+

                Ok(<$T>::$ctor($($comp),+))
            }

            #[inline]
            fn minimum_bytes_needed() -> usize {
                let mut size = 0;
                $(
                    let $comp = <$comp_ty as Readable::<'a, C>>::minimum_bytes_needed();
                    size += $comp;
                )+
                size
            }
        }

        impl<C> Writable<C> for $T
        where
            C: Context,
        {
            #[inline]
            fn write_to<T: ?Sized + Writer<C>>(&self, writer: &mut T) -> Result<(), C::Error> {
                $(
                    writer.$comp_write_fn(self.$comp)?;
                )+

                Ok(())
            }

            #[inline]
            fn bytes_needed(&self) -> Result<usize, C::Error> {
                let mut size = 0;
                $(
                    size += Writable::<C>::bytes_needed(&self.$comp)?;
                )+
                Ok( size )
            }
        }
    };
}

impl_for_vec! {Vec2, new, f32, read_f32, write_f32, x, y}
impl_for_vec! {Vec3, new, f32, read_f32, write_f32, x, y, z}
impl_for_vec! {Vec3A, new, f32, read_f32, write_f32, x, y, z}
impl_for_vec! {Vec4, new, f32, read_f32, write_f32, x, y, z, w}

impl_for_vec! {DVec2, new, f64, read_f64, write_f64, x, y}
impl_for_vec! {DVec3, new, f64, read_f64, write_f64, x, y, z}
impl_for_vec! {DVec4, new, f64, read_f64, write_f64, x, y, z, w}

impl_for_vec! {IVec2, new, i32, read_i32, write_i32, x, y}
impl_for_vec! {IVec3, new, i32, read_i32, write_i32, x, y, z}
impl_for_vec! {IVec4, new, i32, read_i32, write_i32, x, y, z, w}

impl_for_vec! {UVec2, new, u32, read_u32, write_u32, x, y}
impl_for_vec! {UVec3, new, u32, read_u32, write_u32, x, y, z}
impl_for_vec! {UVec4, new, u32, read_u32, write_u32, x, y, z, w}

impl_for_vec! {Quat, from_xyzw, f32, read_f32, write_f32, x, y, z, w}
impl_for_vec! {DQuat, from_xyzw, f64, read_f64, write_f64, x, y, z, w}

macro_rules! impl_for_bvec {
    ($T:ty, $($comp:ident),+) => {
        impl<'a, C> Readable<'a, C> for $T
        where
            C: Context,
        {
            #[inline]
            #[allow(unused_assignments)]
            fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
                let mask = reader.read_u8()?;
                let mut shift = 0;

                $(
                    let $comp = (mask & (1 << shift)) != 0;
                    shift += 1;
                )+

                Ok(<$T>::new($($comp),+))
            }

            #[inline]
            fn minimum_bytes_needed() -> usize {
                <u8 as Readable::<'a, C>>::minimum_bytes_needed()
            }
        }

        impl<C> Writable<C> for $T
        where
            C: Context,
        {
            #[inline]
            fn write_to<T: ?Sized + Writer<C>>(&self, writer: &mut T) -> Result<(), C::Error> {
                writer.write_u8(self.bitmask() as u8)
            }

            #[inline]
            fn bytes_needed(&self) -> Result<usize, C::Error> {
                Writable::<C>::bytes_needed(&0u8)
            }
        }
    };
}

impl_for_bvec! {BVec2, x, y}
impl_for_bvec! {BVec3, x, y, z}
impl_for_bvec! {BVec4, x, y, z, w}

macro_rules! impl_for_mat {
    ($T:ty, $comp_count:literal, $comp_ty:ty) => {
        impl<'a, C> Readable<'a, C> for $T
        where
            C: Context,
        {
            #[inline]
            fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
                let mut values = [Default::default(); $comp_count];
                for v in &mut values {
                    *v = reader.read_value()?;
                }
                Ok(<$T>::from_cols_array(&values))
            }

            #[inline]
            fn minimum_bytes_needed() -> usize {
                <$comp_ty as Readable<'a, C>>::minimum_bytes_needed() * $comp_count
            }
        }

        impl<C> Writable<C> for $T
        where
            C: Context,
        {
            #[inline]
            fn write_to<T: ?Sized + Writer<C>>(&self, writer: &mut T) -> Result<(), C::Error> {
                for comp in self.to_cols_array().iter() {
                    writer.write_value(comp)?
                }

                Ok(())
            }

            #[inline]
            fn bytes_needed(&self) -> Result<usize, C::Error> {
                let mut size = 0;
                for comp in self.to_cols_array().iter() {
                    size += Writable::<C>::bytes_needed(comp)?;
                }
                Ok(size)
            }
        }
    };
}

impl_for_mat! { Mat2, 4, f32 }
impl_for_mat! { Mat3, 9, f32 }
impl_for_mat! { Mat3A, 9, f32 }
impl_for_mat! { Mat4, 16, f32 }

impl_for_mat! { DMat2, 4, f64 }
impl_for_mat! { DMat3, 9, f64 }
impl_for_mat! { DMat4, 16, f64 }

impl_for_mat! { Affine2, 6, f32 }
impl_for_mat! { Affine3A, 12, f32 }

impl_for_mat! { DAffine2, 6, f64 }
impl_for_mat! { DAffine3, 12, f64 }

#[test]
fn test_speedy() {
    use speedy::Endianness;

    macro_rules! test_vec {
        ($T:ty, $ctor:ident, $($values:literal),+) => {{
            let original = <$T>::$ctor($($values as _),+);
            let serialized = original.write_to_vec_with_ctx(Endianness::NATIVE).unwrap();
            let deserialized = <$T>::read_from_buffer_with_ctx(Endianness::NATIVE, &serialized).unwrap();
            assert_eq!(original, deserialized);
        }}
    }

    test_vec!(Vec2, new, 1, 2);
    test_vec!(Vec3, new, 1, 2, 3);
    test_vec!(Vec3A, new, 1, 2, 3);
    test_vec!(Vec4, new, 1, 2, 3, 4);

    test_vec!(DVec2, new, 1, 2);
    test_vec!(DVec3, new, 1, 2, 3);
    test_vec!(DVec4, new, 1, 2, 3, 4);

    test_vec!(IVec2, new, 1, 2);
    test_vec!(IVec3, new, 1, 2, 3);
    test_vec!(IVec4, new, 1, 2, 3, 4);

    test_vec!(UVec2, new, 1, 2);
    test_vec!(UVec3, new, 1, 2, 3);
    test_vec!(UVec4, new, 1, 2, 3, 4);

    test_vec!(Quat, from_xyzw, 1, 2, 3, 4);
    test_vec!(DQuat, from_xyzw, 1, 2, 3, 4);

    for a in [false, true] {
        for b in [false, true] {
            let original = BVec2::new(a, b);
            let serialized = original.write_to_vec_with_ctx(Endianness::NATIVE).unwrap();
            let deserialized =
                BVec2::read_from_buffer_with_ctx(Endianness::NATIVE, &serialized).unwrap();
            assert_eq!(original, deserialized);
        }
    }

    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                let original = BVec3::new(a, b, c);
                let serialized = original.write_to_vec_with_ctx(Endianness::NATIVE).unwrap();
                let deserialized =
                    BVec3::read_from_buffer_with_ctx(Endianness::NATIVE, &serialized).unwrap();
                assert_eq!(original, deserialized);
            }
        }
    }

    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                for d in [false, true] {
                    let original = BVec4::new(a, b, c, d);
                    let serialized = original.write_to_vec_with_ctx(Endianness::NATIVE).unwrap();
                    let deserialized =
                        BVec4::read_from_buffer_with_ctx(Endianness::NATIVE, &serialized).unwrap();
                    assert_eq!(original, deserialized);
                }
            }
        }
    }

    macro_rules! test_mat {
        ($T:ty) => {{
            let mut cols = <$T>::IDENTITY.to_cols_array();
            for (i, c) in cols.iter_mut().enumerate() {
                *c = (i + 1) as _;
            }

            let original = <$T>::from_cols_array(&cols);
            let serialized = original.write_to_vec_with_ctx(Endianness::NATIVE).unwrap();
            let deserialized =
                <$T>::read_from_buffer_with_ctx(Endianness::NATIVE, &serialized).unwrap();
            assert_eq!(original, deserialized);
        }};
    }

    test_mat!(Mat2);
    test_mat!(Mat3);
    test_mat!(Mat3A);
    test_mat!(Mat4);

    test_mat!(DMat2);
    test_mat!(DMat3);
    test_mat!(DMat4);

    test_mat!(Affine2);
    test_mat!(Affine3A);

    test_mat!(DAffine2);
    test_mat!(DAffine3);
}
