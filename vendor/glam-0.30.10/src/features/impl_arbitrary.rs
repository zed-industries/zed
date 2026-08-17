use crate::{
    BVec2, BVec3, BVec3A, BVec4, BVec4A, DQuat, DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4,
    I64Vec2, I64Vec3, I64Vec4, I8Vec2, I8Vec3, I8Vec4, IVec2, IVec3, IVec4, Quat, U16Vec2, U16Vec3,
    U16Vec4, U64Vec2, U64Vec3, U64Vec4, U8Vec2, U8Vec3, U8Vec4, USizeVec2, USizeVec3, USizeVec4,
    UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4,
};

macro_rules! arbitrary_vector_impl {
    ($($ty:ty),*) => {
        $(
            impl arbitrary::Arbitrary<'_> for $ty {
                fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
                    u.arbitrary().map(Self::from_array)
                }
            }
        )*
    };
}

arbitrary_vector_impl!(
    BVec2, BVec3, BVec3A, BVec4, BVec4A, DQuat, DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4,
    I64Vec2, I64Vec3, I64Vec4, I8Vec2, I8Vec3, I8Vec4, IVec2, IVec3, IVec4, Quat, U16Vec2, U16Vec3,
    U16Vec4, U64Vec2, U64Vec3, U64Vec4, U8Vec2, U8Vec3, U8Vec4, USizeVec2, USizeVec3, USizeVec4,
    UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4
);

use crate::{Affine2, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DMat4, Mat2, Mat3, Mat3A, Mat4};

macro_rules! arbitrary_matrix_impl {
    ($($ty:ty),*) => {
        $(
            impl arbitrary::Arbitrary<'_> for $ty {
                fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
                    let array = u.arbitrary()?;
                    Ok(Self::from_cols_array(&array))
                }
            }
        )*
    };
}

arbitrary_matrix_impl!(
    Affine2, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DMat4, Mat2, Mat3, Mat3A, Mat4
);

#[cfg(test)]
mod test {
    use arbitrary::Unstructured;
    use core::mem::size_of;

    use crate::{
        Affine2, Affine3A, BVec2, BVec3, BVec3A, BVec4, BVec4A, DAffine2, DAffine3, DMat2, DMat3,
        DMat4, DQuat, DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4,
        I8Vec2, I8Vec3, I8Vec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat3A, Mat4, Quat, U16Vec2,
        U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, U8Vec2, U8Vec3, U8Vec4, USizeVec2, USizeVec3,
        USizeVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4,
    };

    #[test]
    fn test_arbitrary_f32() {
        // The float arbitrary impl converts byte array -> integer bits -> float bits.
        // Writing little endian floats to the buffer allows tests without precision issues.
        let mut bytes = [0u8; 16 * size_of::<f32>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<f32>()).enumerate() {
            chunk.copy_from_slice(&((i + 1) as f32).to_le_bytes());
        }

        assert_eq!(
            Affine2::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            Affine3A::from_cols_array(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0
            ]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );

        assert_eq!(
            Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            Mat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            Mat3A::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            Mat4::from_cols_array(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0
            ]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );

        assert_eq!(
            Quat::from_array([1.0, 2.0, 3.0, 4.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );

        assert_eq!(
            Vec2::new(1.0, 2.0),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            Vec3A::new(1.0, 2.0, 3.0),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_f64() {
        // The float arbitrary impl converts byte array -> integer bits -> float bits.
        // Writing little endian floats to the buffer allows tests without precision issues.
        let mut bytes = [0u8; 16 * size_of::<f64>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<f64>()).enumerate() {
            chunk.copy_from_slice(&((i + 1) as f64).to_le_bytes());
        }

        assert_eq!(
            DAffine2::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            DAffine3::from_cols_array(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0
            ]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );

        assert_eq!(
            DMat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            DMat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            DMat4::from_cols_array(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0
            ]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );

        assert_eq!(
            DQuat::from_array([1.0, 2.0, 3.0, 4.0]),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );

        assert_eq!(
            DVec2::new(1.0, 2.0),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            DVec3::new(1.0, 2.0, 3.0),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            DVec4::new(1.0, 2.0, 3.0, 4.0),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_usize() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<usize>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<usize>()).enumerate() {
            chunk.copy_from_slice(&(i + 1).to_le_bytes());
        }

        assert_eq!(
            USizeVec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            USizeVec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            USizeVec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_u8() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<u8>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<u8>()).enumerate() {
            chunk.copy_from_slice(&(i as u8 + 1).to_le_bytes());
        }

        assert_eq!(
            U8Vec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            U8Vec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            U8Vec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_i8() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<i8>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<i8>()).enumerate() {
            chunk.copy_from_slice(&(i as i8 + 1).to_le_bytes());
        }

        assert_eq!(
            I8Vec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            I8Vec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            I8Vec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_u16() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<u16>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<u16>()).enumerate() {
            chunk.copy_from_slice(&(i as u16 + 1).to_le_bytes());
        }

        assert_eq!(
            U16Vec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            U16Vec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            U16Vec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_i16() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<i16>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<i16>()).enumerate() {
            chunk.copy_from_slice(&(i as i16 + 1).to_le_bytes());
        }

        assert_eq!(
            I16Vec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            I16Vec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            I16Vec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_u32() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<u32>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<u32>()).enumerate() {
            chunk.copy_from_slice(&(i as u32 + 1).to_le_bytes());
        }

        assert_eq!(
            UVec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            UVec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            UVec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_i32() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<i32>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<i32>()).enumerate() {
            chunk.copy_from_slice(&(i as i32 + 1).to_le_bytes());
        }

        assert_eq!(
            IVec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            IVec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            IVec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_u64() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<u64>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<u64>()).enumerate() {
            chunk.copy_from_slice(&(i as u64 + 1).to_le_bytes());
        }

        assert_eq!(
            U64Vec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            U64Vec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            U64Vec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_i64() {
        // The integer arbitrary impl converts little endian bytes.
        let mut bytes = [0u8; 4 * size_of::<i64>()];
        for (i, chunk) in bytes.chunks_exact_mut(size_of::<i64>()).enumerate() {
            chunk.copy_from_slice(&(i as i64 + 1).to_le_bytes());
        }

        assert_eq!(
            I64Vec2::new(1, 2),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            I64Vec3::new(1, 2, 3),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            I64Vec4::new(1, 2, 3, 4),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }

    #[test]
    fn test_arbitrary_bool() {
        // The bool arbitrary impl checks if u8 == 1.
        let bytes = [1u8, 0u8, 1u8, 0u8];
        assert_eq!(
            BVec2::new(true, false),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            BVec3::new(true, false, true),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            BVec3A::new(true, false, true),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            BVec4::new(true, false, true, false),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
        assert_eq!(
            BVec4A::new(true, false, true, false),
            Unstructured::new(&bytes).arbitrary().unwrap()
        );
    }
}
