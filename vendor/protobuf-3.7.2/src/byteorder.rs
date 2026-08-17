/// Expose cfg as constant to be able to typecheck both versions.
pub(crate) const LITTLE_ENDIAN: bool = cfg!(target_endian = "little");
