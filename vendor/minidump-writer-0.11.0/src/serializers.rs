//! Functions used by Serde to serialize types that we don't own (and thus can't implement
//! [Serialize] for)

use serde::Serializer;
/// Useful for types that implement [Error][std::error::Error] and don't need any special
/// treatment.
pub fn serialize_generic_error<S: Serializer, E: std::error::Error>(
    error: &E,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    // I guess we'll have to see if it's more useful to store the debug representation of a
    // foreign error type or something else (like maybe iterating its error chain into a
    // list?)
    let dbg = format!("{error:#?}");
    serializer.serialize_str(&dbg)
}
/// Serialize [std::io::Error]
pub fn serialize_io_error<S: Serializer>(
    error: &std::io::Error,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_generic_error(error, serializer)
}
/// Serialize [scroll::Error]
pub fn serialize_scroll_error<S: Serializer>(
    error: &scroll::Error,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serialize_generic_error(error, serializer)
}
