use crate::sys::{
    JNI_VERSION_1_1, JNI_VERSION_1_2, JNI_VERSION_1_4, JNI_VERSION_1_6, JNI_VERSION_1_8,
};

/// JNI Version
///
/// This maps to the `jni_sys::JNI_VERSION_1_*` constants.
#[derive(Debug, Copy, Clone)]
#[allow(missing_docs)]
pub enum JNIVersion {
    V1,
    V2,
    V4,
    V6,
    V8,
    Invalid(i32),
}

impl From<i32> for JNIVersion {
    fn from(other: i32) -> Self {
        match other {
            JNI_VERSION_1_1 => JNIVersion::V1,
            JNI_VERSION_1_2 => JNIVersion::V2,
            JNI_VERSION_1_4 => JNIVersion::V4,
            JNI_VERSION_1_6 => JNIVersion::V6,
            JNI_VERSION_1_8 => JNIVersion::V8,
            v => JNIVersion::Invalid(v),
        }
    }
}

impl From<JNIVersion> for i32 {
    fn from(other: JNIVersion) -> Self {
        match other {
            JNIVersion::V1 => JNI_VERSION_1_1,
            JNIVersion::V2 => JNI_VERSION_1_2,
            JNIVersion::V4 => JNI_VERSION_1_4,
            JNIVersion::V6 => JNI_VERSION_1_6,
            JNIVersion::V8 => JNI_VERSION_1_8,
            JNIVersion::Invalid(v) => v,
        }
    }
}
