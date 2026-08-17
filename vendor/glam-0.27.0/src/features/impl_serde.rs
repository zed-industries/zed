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

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            let a = $vec2::new(V1, V2);
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

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            let a = $vec3::new(V1, V2, V3);
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

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            let a = $vec4::new(V1, V2, V3, V4);
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

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        impl_serde_mat3!($t, $mat3, test_mat3_serde);
    };
    ($t:ty, $mat3:ident, $test_name:ident) => {
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

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        fn $test_name() {
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

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

macro_rules! impl_serde_affine2 {
    ($t:ty, $affine2:ident) => {
        impl Serialize for $affine2 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                // Serialize column-wise as 3x4 matrix:
                let mut state = serializer.serialize_tuple_struct(stringify!($affine2), 6)?;
                state.serialize_field(&self.x_axis.x)?;
                state.serialize_field(&self.x_axis.y)?;
                state.serialize_field(&self.y_axis.x)?;
                state.serialize_field(&self.y_axis.y)?;
                state.serialize_field(&self.z_axis.x)?;
                state.serialize_field(&self.z_axis.y)?;
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for $affine2 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Affine2Visitor;

                impl<'de> Visitor<'de> for Affine2Visitor {
                    type Value = $affine2;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("struct $affine2")
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$affine2, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut f = [0.0; 6];
                        for (i, v) in f.iter_mut().enumerate() {
                            *v = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok($affine2::from_cols_array(&f))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($affine2), 6, Affine2Visitor)
            }
        }

        #[test]
        fn test_affine2_serde() {
            let a = $affine2::from_cols_array(&[1.0, 0.0, 2.0, 0.0, 3.0, 4.0]);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(serialized, "[1.0,0.0,2.0,0.0,3.0,4.0]");
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);

            let deserialized = serde_json::from_str::<$affine2>("[]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine2>("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine2>("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine2>("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine2>("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine2>("[[1.0,2.0],[3.0,4.0],[5.0,6.0]]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine2>(
                "[[1.0,2.0,3.0,4.0],[5.0,6.0,7.0,8.0],[9.0,10.0,11.0,12.0][13.0,14.0,15.0,16.0]]",
            );
            assert!(deserialized.is_err());
        }
    };
}

macro_rules! impl_serde_affine3 {
    ($t:ty, $affine3:ident) => {
        impl Serialize for $affine3 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                // Serialize column-wise as 3x4 matrix:
                let mut state = serializer.serialize_tuple_struct(stringify!($affine3), 12)?;
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

        impl<'de> Deserialize<'de> for $affine3 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct Affine3Visitor;

                impl<'de> Visitor<'de> for Affine3Visitor {
                    type Value = $affine3;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("struct $affine3")
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<$affine3, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut f = [0.0; 12];
                        for (i, v) in f.iter_mut().enumerate() {
                            *v = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok($affine3::from_cols_array(&f))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($affine3), 12, Affine3Visitor)
            }
        }

        #[test]
        fn test_affine3_serde() {
            let a = $affine3::from_cols_array(&[
                1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 4.0, 5.0, 6.0,
            ]);
            let serialized = serde_json::to_string(&a).unwrap();
            assert_eq!(
                serialized,
                "[1.0,0.0,0.0,0.0,2.0,0.0,0.0,0.0,3.0,4.0,5.0,6.0]"
            );
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(a, deserialized);

            let deserialized = serde_json::from_str::<$affine3>("[]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine3>("[1.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine3>("[1.0,2.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine3>("[1.0,2.0,3.0]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine3>("[1.0,2.0,3.0,4.0,5.0]");
            assert!(deserialized.is_err());
            let deserialized =
                serde_json::from_str::<$affine3>("[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]");
            assert!(deserialized.is_err());
            let deserialized = serde_json::from_str::<$affine3>(
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
    ($t:ty, $affine2:ident, $affine3:ident, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_serde_affine2!($t, $affine2);
        impl_serde_affine3!($t, $affine3);
        impl_serde_mat2!($t, $mat2);
        impl_serde_mat3!($t, $mat3);
        impl_serde_mat4!($t, $mat4);
        impl_serde_quat!($t, $quat);
        impl_serde_vec_types!($t, $vec2, $vec3, $vec4);
    };
}

#[cfg(test)]
mod test_f32 {
    pub const V1: f32 = 1.0;
    pub const V2: f32 = 2.0;
    pub const V3: f32 = 3.0;
    pub const V4: f32 = 4.0;
}

#[cfg(test)]
mod test_f64 {
    pub const V1: f64 = 1.0;
    pub const V2: f64 = 2.0;
    pub const V3: f64 = 3.0;
    pub const V4: f64 = 4.0;
}

#[cfg(test)]
mod test_i16 {
    pub const V1: i16 = 1;
    pub const V2: i16 = 2;
    pub const V3: i16 = 3;
    pub const V4: i16 = 4;
}

#[cfg(test)]
mod test_i32 {
    pub const V1: i32 = 1;
    pub const V2: i32 = 2;
    pub const V3: i32 = 3;
    pub const V4: i32 = 4;
}

#[cfg(test)]
mod test_i64 {
    pub const V1: i64 = 1;
    pub const V2: i64 = 2;
    pub const V3: i64 = 3;
    pub const V4: i64 = 4;
}

#[cfg(test)]
mod test_u16 {
    pub const V1: u16 = 1;
    pub const V2: u16 = 2;
    pub const V3: u16 = 3;
    pub const V4: u16 = 4;
}

#[cfg(test)]
mod test_u32 {
    pub const V1: u32 = 1;
    pub const V2: u32 = 2;
    pub const V3: u32 = 3;
    pub const V4: u32 = 4;
}

#[cfg(test)]
mod test_u64 {
    pub const V1: u64 = 1;
    pub const V2: u64 = 2;
    pub const V3: u64 = 3;
    pub const V4: u64 = 4;
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

#[cfg(test)]
mod test_bool_mask {
    pub const SX0: &str = "[]";
    pub const SX1: &str = "[true]";
    pub const SX2: &str = "[true,true]";
    pub const SX3: &str = "[true,true,true]";
    pub const SX4: &str = "[true,true,true,true]";
    pub const SX5: &str = "[true,true,true,true,true]";
    pub const V1: bool = true;
    pub const V2: bool = true;
    pub const V3: bool = true;
    pub const V4: bool = true;
}

mod bool {
    #[cfg(test)]
    use super::test_bool_mask::*;
    use crate::{BVec2, BVec3, BVec4};
    #[cfg(not(feature = "scalar-math"))]
    use crate::{BVec3A, BVec4A};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_vec2!(bool, BVec2);
    impl_serde_vec3!(bool, BVec3);
    impl_serde_vec4!(bool, BVec4);

    #[cfg(not(feature = "scalar-math"))]
    impl Serialize for BVec3A {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_tuple_struct("BVec3A", 3)?;
            let a: [bool; 3] = (*self).into();
            state.serialize_field(&a[0])?;
            state.serialize_field(&a[1])?;
            state.serialize_field(&a[2])?;
            state.end()
        }
    }

    #[cfg(not(feature = "scalar-math"))]
    impl<'de> Deserialize<'de> for BVec3A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct Vec3Visitor;

            impl<'de> Visitor<'de> for Vec3Visitor {
                type Value = BVec3A;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str("struct BVec3A")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<BVec3A, V::Error>
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
                    Ok(BVec3A::new(x, y, z))
                }
            }

            deserializer.deserialize_tuple_struct(stringify!($vec3), 3, Vec3Visitor)
        }
    }

    #[cfg(not(feature = "scalar-math"))]
    #[test]
    fn test_bvec3a_serde() {
        let a = BVec3A::new(V1, V2, V3);
        let serialized = serde_json::to_string(&a).unwrap();
        assert_eq!(SX3, serialized);
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(a, deserialized);
        let deserialized = serde_json::from_str::<BVec3A>(SX0);
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<BVec3A>(SX1);
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<BVec3A>(SX2);
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<BVec3A>(SX4);
        assert!(deserialized.is_err());
    }

    #[cfg(not(feature = "scalar-math"))]
    impl Serialize for BVec4A {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_tuple_struct(stringify!(BVec4A), 4)?;
            let a: [bool; 4] = (*self).into();
            state.serialize_field(&a[0])?;
            state.serialize_field(&a[1])?;
            state.serialize_field(&a[2])?;
            state.serialize_field(&a[2])?;
            state.end()
        }
    }

    #[cfg(not(feature = "scalar-math"))]
    impl<'de> Deserialize<'de> for BVec4A {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct Vec4Visitor;

            impl<'de> Visitor<'de> for Vec4Visitor {
                type Value = BVec4A;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str(concat!("struct ", stringify!(BVec4A)))
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<BVec4A, V::Error>
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
                    Ok(BVec4A::new(x, y, z, w))
                }
            }

            deserializer.deserialize_tuple_struct(stringify!(BVec4A), 4, Vec4Visitor)
        }
    }

    #[cfg(not(feature = "scalar-math"))]
    #[test]
    fn test_bvec4a_serde() {
        let a = BVec4A::new(V1, V2, V3, V4);
        let serialized = serde_json::to_string(&a).unwrap();
        assert_eq!(SX4, serialized);
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(a, deserialized);
        let deserialized = serde_json::from_str::<BVec4A>(SX0);
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<BVec4A>(SX1);
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<BVec4A>(SX2);
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<BVec4A>(SX3);
        assert!(deserialized.is_err());
        let deserialized = serde_json::from_str::<BVec4A>(SX5);
        assert!(deserialized.is_err());
    }
}

mod f32 {
    #[cfg(test)]
    use super::test_f32::*;
    #[cfg(test)]
    use super::test_float::*;
    use crate::{Affine2, Affine3A, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_float_types!(f32, Affine2, Affine3A, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);
    impl_serde_mat3!(f32, Mat3A, test_mat3a_serde);
    impl_serde_vec3!(f32, Vec3A, test_vec3a_serde);
}

mod f64 {
    #[cfg(test)]
    use super::test_f64::*;
    #[cfg(test)]
    use super::test_float::*;
    use crate::{DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_float_types!(
        f64, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4
    );
}

mod i16 {
    #[cfg(test)]
    use super::test_i16::*;
    #[cfg(test)]
    use super::test_int::*;
    use crate::{I16Vec2, I16Vec3, I16Vec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_vec_types!(i16, I16Vec2, I16Vec3, I16Vec4);
}

mod i32 {
    #[cfg(test)]
    use super::test_i32::*;
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

mod i64 {
    #[cfg(test)]
    use super::test_i64::*;
    #[cfg(test)]
    use super::test_int::*;
    use crate::{I64Vec2, I64Vec3, I64Vec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_vec_types!(i64, I64Vec2, I64Vec3, I64Vec4);
}

mod u16 {
    #[cfg(test)]
    use super::test_int::*;
    #[cfg(test)]
    use super::test_u16::*;
    use crate::{U16Vec2, U16Vec3, U16Vec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_vec_types!(u16, U16Vec2, U16Vec3, U16Vec4);
}

mod u32 {
    #[cfg(test)]
    use super::test_int::*;
    #[cfg(test)]
    use super::test_u32::*;
    use crate::{UVec2, UVec3, UVec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_vec_types!(u32, UVec2, UVec3, UVec4);
}

mod u64 {
    #[cfg(test)]
    use super::test_int::*;
    #[cfg(test)]
    use super::test_u64::*;
    use crate::{U64Vec2, U64Vec3, U64Vec4};
    use core::fmt;
    use serde::{
        de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
        ser::{Serialize, SerializeTupleStruct, Serializer},
    };

    impl_serde_vec_types!(u64, U64Vec2, U64Vec3, U64Vec4);
}

mod euler {
    use crate::EulerRot;

    impl serde::Serialize for EulerRot {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match *self {
                EulerRot::ZYX => {
                    serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 0u32, "ZYX")
                }
                EulerRot::ZXY => {
                    serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 1u32, "ZXY")
                }
                EulerRot::YXZ => {
                    serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 2u32, "YXZ")
                }
                EulerRot::YZX => {
                    serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 3u32, "YZX")
                }
                EulerRot::XYZ => {
                    serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 4u32, "XYZ")
                }
                EulerRot::XZY => {
                    serde::Serializer::serialize_unit_variant(serializer, "EulerRot", 5u32, "XZY")
                }
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for EulerRot {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[allow(clippy::upper_case_acronyms)]
            enum Field {
                ZYX,
                ZXY,
                YXZ,
                YZX,
                XYZ,
                XZY,
            }
            struct FieldVisitor;

            impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                type Value = Field;
                fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(formatter, "variant identifier")
                }
                fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match value {
                        0u64 => Ok(Field::ZYX),
                        1u64 => Ok(Field::ZXY),
                        2u64 => Ok(Field::YXZ),
                        3u64 => Ok(Field::YZX),
                        4u64 => Ok(Field::XYZ),
                        5u64 => Ok(Field::XZY),
                        _ => Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Unsigned(value),
                            &"variant index 0 <= i < 6",
                        )),
                    }
                }
                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match value {
                        "ZYX" => Ok(Field::ZYX),
                        "ZXY" => Ok(Field::ZXY),
                        "YXZ" => Ok(Field::YXZ),
                        "YZX" => Ok(Field::YZX),
                        "XYZ" => Ok(Field::XYZ),
                        "XZY" => Ok(Field::XZY),
                        _ => Err(serde::de::Error::unknown_variant(value, VARIANTS)),
                    }
                }
                fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match value {
                        b"ZYX" => Ok(Field::ZYX),
                        b"ZXY" => Ok(Field::ZXY),
                        b"YXZ" => Ok(Field::YXZ),
                        b"YZX" => Ok(Field::YZX),
                        b"XYZ" => Ok(Field::XYZ),
                        b"XZY" => Ok(Field::XZY),
                        _ => {
                            #[cfg(feature = "std")]
                            let value = &String::from_utf8_lossy(value);
                            #[cfg(not(feature = "std"))]
                            let value =
                                core::str::from_utf8(value).unwrap_or("\u{fffd}\u{fffd}\u{fffd}");
                            Err(serde::de::Error::unknown_variant(value, VARIANTS))
                        }
                    }
                }
            }
            impl<'de> serde::Deserialize<'de> for Field {
                #[inline]
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    serde::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
                }
            }
            struct Visitor<'de> {
                marker: core::marker::PhantomData<EulerRot>,
                lifetime: core::marker::PhantomData<&'de ()>,
            }
            impl<'de> serde::de::Visitor<'de> for Visitor<'de> {
                type Value = EulerRot;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter<'_>,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "enum EulerRot")
                }
                fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::EnumAccess<'de>,
                {
                    match serde::de::EnumAccess::variant(data)? {
                        (Field::ZYX, variant) => {
                            serde::de::VariantAccess::unit_variant(variant)?;
                            Ok(EulerRot::ZYX)
                        }
                        (Field::ZXY, variant) => {
                            serde::de::VariantAccess::unit_variant(variant)?;
                            Ok(EulerRot::ZXY)
                        }
                        (Field::YXZ, variant) => {
                            serde::de::VariantAccess::unit_variant(variant)?;
                            Ok(EulerRot::YXZ)
                        }
                        (Field::YZX, variant) => {
                            serde::de::VariantAccess::unit_variant(variant)?;
                            Ok(EulerRot::YZX)
                        }
                        (Field::XYZ, variant) => {
                            serde::de::VariantAccess::unit_variant(variant)?;
                            Ok(EulerRot::XYZ)
                        }
                        (Field::XZY, variant) => {
                            serde::de::VariantAccess::unit_variant(variant)?;
                            Ok(EulerRot::XZY)
                        }
                    }
                }
            }
            const VARIANTS: &[&str] = &["ZYX", "ZXY", "YXZ", "YZX", "XYZ", "XZY"];
            serde::Deserializer::deserialize_enum(
                deserializer,
                "EulerRot",
                VARIANTS,
                Visitor {
                    marker: core::marker::PhantomData::<EulerRot>,
                    lifetime: core::marker::PhantomData,
                },
            )
        }
    }

    #[test]
    fn test_euler_rot_serde() {
        let a = EulerRot::XYZ;
        let serialized = serde_json::to_string(&a).unwrap();
        assert_eq!("\"XYZ\"", serialized);
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(a, deserialized);
    }
}
