macro_rules! impl_serde_vec2 {
    ($t:ty, $vec2:ident) => {
        impl Serialize for $vec2 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($vec2), 2)?;
                state.serialize_field(&self.x)?;
                state.serialize_field(&self.y)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $vec2 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Vec2Visitor;

                impl<'de> Visitor<'de> for Vec2Visitor {
                    type Value = $vec2;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($vec2)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$vec2, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        Ok($vec2::new(x, y))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($vec2), 2, Vec2Visitor)
            }
        }

        #[test]
        fn test_vec2_serde() {
            let a = $vec2::new(1 as $t, 2 as $t);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(SX2, serialized);
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = serde_json::from_str::<$vec2>(SX0);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec2>(SX1);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec2>(SX3);
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_vec3 {
    ($t:ty, $vec3:ident) => {
        impl_serde_vec3!($t, $vec3, test_vec3_serde);
    };
    ($t:ty, $vec3:ident, $test_name:ident) => {
        impl Serialize for $vec3 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($vec3), 3)?;
                state.serialize_field(&self.x)?;
                state.serialize_field(&self.y)?;
                state.serialize_field(&self.z)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $vec3 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Vec3Visitor;

                impl<'de> Visitor<'de> for Vec3Visitor {
                    type Value = $vec3;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($vec3)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$vec3, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        let z = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                        Ok($vec3::new(x, y, z))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($vec3), 3, Vec3Visitor)
            }
        }

        #[test]
        fn $test_name() {
            let a = $vec3::new(1 as $t, 2 as $t, 3 as $t);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(SX3, serialized);
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = serde_json::from_str::<$vec3>(SX0);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec3>(SX1);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec3>(SX2);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec3>(SX4);
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_vec4 {
    ($t:ty, $vec4:ident) => {
        impl Serialize for $vec4 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($vec4), 4)?;
                state.serialize_field(&self.x)?;
                state.serialize_field(&self.y)?;
                state.serialize_field(&self.z)?;
                state.serialize_field(&self.w)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $vec4 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Vec4Visitor;

                impl<'de> Visitor<'de> for Vec4Visitor {
                    type Value = $vec4;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($vec4)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$vec4, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        let z = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                        let w = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                        Ok($vec4::new(x, y, z, w))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($vec4), 4, Vec4Visitor)
            }
        }

        #[test]
        fn test_vec4_serde() {
            let a = $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(SX4, serialized);
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = serde_json::from_str::<$vec4>(SX0);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec4>(SX1);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec4>(SX2);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec4>(SX3);
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$vec4>(SX5);
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_quat {
    ($t:ty, $quat:ident) => {
        impl Serialize for $quat {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($quat), 4)?;
                state.serialize_field(&self.x)?;
                state.serialize_field(&self.y)?;
                state.serialize_field(&self.z)?;
                state.serialize_field(&self.w)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $quat {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct QuatVisitor;

                impl<'de> Visitor<'de> for QuatVisitor {
                    type Value = $quat;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($quat)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$quat, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let x = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        let z = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                        let w = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                        Ok($quat::from_xyzw(x, y, z, w))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($quat), 4, QuatVisitor)
            }
        }

        #[test]
        fn test_quat_serde() {
            let a = $quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = serde_json::from_str::<$quat>("[]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$quat>("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$quat>("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$quat>("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$quat>("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_mat2 {
    ($t:ty, $mat2:ident) => {
        impl Serialize for $mat2 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let f: &[_; 4] = self.as_ref();
                let mut state = serializer.serialize_tuple_struct(stringify!($mat2), 4)?;
                state.serialize_field(&f[0])?;
                state.serialize_field(&f[1])?;
                state.serialize_field(&f[2])?;
                state.serialize_field(&f[3])?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $mat2 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat2Visitor;

                impl<'de> Visitor<'de> for Mat2Visitor {
                    type Value = $mat2;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($mat2)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$mat2, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut f = { [0.0; 4] };
                        for i in 0..4 {
                            f[i] = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok($mat2::from_cols_array(&f))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($mat2), 4, Mat2Visitor)
            }
        }

        #[test]
        fn test_mat2_serde() {
            let a = $mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(serialized, "[1.0,2.0,3.0,4.0]");
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = serde_json::from_str::<$mat2>("[]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat2>("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat2>("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat2>("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat2>("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat2>("[[1.0,2.0],[3.0,4.0]]");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_mat3 {
    ($t:ty, $mat3:ident) => {
        impl Serialize for $mat3 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let (m00, m01, m02) = self.x_axis.into();
                let (m10, m11, m12) = self.y_axis.into();
                let (m20, m21, m22) = self.z_axis.into();

                let mut state = serializer.serialize_tuple_struct(stringify!($mat3), 9)?;
                state.serialize_field(&m00)?;
                state.serialize_field(&m01)?;
                state.serialize_field(&m02)?;
                state.serialize_field(&m10)?;
                state.serialize_field(&m11)?;
                state.serialize_field(&m12)?;
                state.serialize_field(&m20)?;
                state.serialize_field(&m21)?;
                state.serialize_field(&m22)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $mat3 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat3Visitor;

                impl<'de> Visitor<'de> for Mat3Visitor {
                    type Value = $mat3;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($mat3)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$mat3, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut f = { [0.0; 9] };
                        for i in 0..9 {
                            f[i] = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok($mat3::from_cols_array(&f))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($mat3), 9, Mat3Visitor)
            }
        }

        #[test]
        fn test_mat3_serde() {
            let a = $mat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(serialized, "[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0]");
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = serde_json::from_str::<$mat3>("[]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat3>("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat3>("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat3>("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat3>("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized =
                serde_json::from_str::<$mat3>("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_mat4 {
    ($t:ty, $mat4:ident) => {
        impl Serialize for $mat4 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_tuple_struct(stringify!($mat4), 16)?;
                for f in self.as_ref() {
                    state.serialize_field(f)?;
                }
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $mat4 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Mat4Visitor;

                impl<'de> Visitor<'de> for Mat4Visitor {
                    type Value = $mat4;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("struct ", stringify!($mat4)))
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$mat4, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut f = { [0.0; 16] };
                        for i in 0..16 {
                            f[i] = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok($mat4::from_cols_array(&f))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($mat4), 16, Mat4Visitor)
            }
        }

        #[test]
        fn test_mat4_serde() {
            let a = $mat4::from_cols_array(&[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ]);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(
                serialized,
                "[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0]"
            );
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);
            let deserialized = serde_json::from_str::<$mat4>("[]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat4>("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat4>("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat4>("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat4>("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized =
                serde_json::from_str::<$mat4>("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$mat4>(
                "[[1.0,2.0,3.0,4.0],[5.0,6.0,7.0,8.0],[9.0,10.0,11.0,12.0][13.0,14.0,15.0,16.0]]",
            );
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_vec_types {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_serde_vec2!($t, $vec2);
        impl_serde_vec3!($t, $vec3);
        impl_serde_vec4!($t, $vec4);
    };
}

macro_rules! impl_serde_float_types {
    ($t:ty, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_serde_mat2!($t, $mat2);
        impl_serde_mat3!($t, $mat3);
        impl_serde_mat4!($t, $mat4);
        impl_serde_quat!($t, $quat);
        impl_serde_vec_types!($t, $vec2, $vec3, $vec4);
    };
}

#[cfg(test)]
mod test_float {
    pub const SX0: &str = "[]";
    pub const SX1: &str = "[1.0]";
    pub const SX2: &str = "[1.0,2.0]";
    pub const SX3: &str = "[1.0,2.0,3.0]";
    pub const SX4: &str = "[1.0,2.0,3.0,4.0]";
    pub const SX5: &str = "[1.0,2.0,3.0,4.0,5.0]";
}

#[cfg(test)]
mod test_int {
    pub const SX0: &str = "[]";
    pub const SX1: &str = "[1]";
    pub const SX2: &str = "[1,2]";
    pub const SX3: &str = "[1,2,3]";
    pub const SX4: &str = "[1,2,3,4]";
    pub const SX5: &str = "[1,2,3,4,5]";
}

mod f32 {
    #[cfg(test)]
    use super::test_float::*;
    use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_float_types!(f32, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);
    impl_serde_vec3!(f32, Vec3A, test_vec3a_serde);
}

mod f64 {
    #[cfg(test)]
    use super::test_float::*;
    use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_float_types!(f64, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
}

mod i32 {
    #[cfg(test)]
    use super::test_int::*;
    use crate::{IVec2, IVec3, IVec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_vec_types!(i32, IVec2, IVec3, IVec4);
}

mod u32 {
    #[cfg(test)]
    use super::test_int::*;
    use crate::{UVec2, UVec3, UVec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_vec_types!(u32, UVec2, UVec3, UVec4);
}

mod affine3a {
    use crate::{Affine3A, Mat3, Vec3};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl Serialize for Affine3A {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Serialize column-wise as 3x4 matrix:
            let mut state = serializer.serialize_tuple_struct("Affine3A", 12)?;
            state.serialize_field(&self.x_axis.x)?;
            state.serialize_field(&self.x_axis.y)?;
            state.serialize_field(&self.x_axis.z)?;
            state.serialize_field(&self.y_axis.x)?;
            state.serialize_field(&self.y_axis.y)?;
            state.serialize_field(&self.y_axis.z)?;
            state.serialize_field(&self.z_axis.x)?;
            state.serialize_field(&self.z_axis.y)?;
            state.serialize_field(&self.z_axis.z)?;
            state.serialize_field(&self.w_axis.x)?;
            state.serialize_field(&self.w_axis.y)?;
            state.serialize_field(&self.w_axis.z)?;
            state.end()
        }
    }

    impl<'de> Deserialize<'de> for Affine3A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct Affine3Visitor;

            impl<'de> Visitor<'de> for Affine3Visitor {
                type Value = Affine3A;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct Affine3A")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Affine3A, V::Error>
                where
                    V: SeqAccess<'de>,
                {
                    let mut f = { [0.0; 12] };
                    for (i, v) in f.iter_mut().enumerate() {
                        *v = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                    }
                    let m = Mat3::from_cols_array(&[
                        f[0], f[1], f[2], //
                        f[3], f[4], f[5], //
                        f[6], f[7], f[8], //
                    ]);
                    let t = Vec3::new(f[9], f[10], f[11]);
                    Ok(Affine3A::from_mat3_translation(m, t))
                }
            }

            deserializer.deserialize_tuple_struct("Affine3A", 12, Affine3Visitor)
        }
    }

    #[test]
    fn test_affine3_serde() {
        use crate::{Quat, Vec3};

        let a = Affine3A::from_scale_rotation_translation(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::IDENTITY,
            Vec3::new(4.0, 5.0, 6.0),
        );
        let serialized = serde_json::to_string(&a).unwrap();
        assert_eq!(
            serialized,
            "[1.0,0.0,0.0,0.0,2.0,0.0,0.0,0.0,3.0,4.0,5.0,6.0]"
        );
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(a, deserialized);

        let deserialized = serde_json::from_str::<Affine3A>("[]");
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<Affine3A>("[1.0]");
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<Affine3A>("[1.0,2.0]");
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<Affine3A>("[1.0,2.0,3.0]");
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<Affine3A>("[1.0,2.0,3.0,4.0,5.0]");
        assert!(deserialized.is_err());
        let deserialized =
            serde_json::from_str::<Affine3A>("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<Affine3A>(
            "[[1.0,2.0,3.0,4.0],[5.0,6.0,7.0,8.0],[9.0,10.0,11.0,12.0][13.0,14.0,15.0,16.0]]",
        );
        assert!(deserialized.is_err());
    }
}
