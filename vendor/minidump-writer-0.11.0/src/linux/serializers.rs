//! Functions used by Serde to serialize types that we don't own (and thus can't implement
//! [Serialize] for)

use {crate::serializers::*, serde::Serializer};

/// Serialize [goblin::error::Error]
pub fn serialize_goblin_error<S: Serializer>(
    error: &goblin::error::Error,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_generic_error(error, serializer)
}
/// Serialize [nix::Error]
pub fn serialize_nix_error<S: Serializer>(
    error: &nix::Error,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_generic_error(error, serializer)
}
/// Serialize [procfs_core::ProcError]
pub fn serialize_proc_error<S: Serializer>(
    error: &procfs_core::ProcError,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_generic_error(error, serializer)
}
/// Serialize [std::string::FromUtf8Error]
pub fn serialize_from_utf8_error<S: Serializer>(
    error: &std::string::FromUtf8Error,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_generic_error(error, serializer)
}
/// Serialize [std::time::SystemTimeError]
pub fn serialize_system_time_error<S: Serializer>(
    error: &std::time::SystemTimeError,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_generic_error(error, serializer)
}
