use super::{BorshSchemaContainer, Declaration, Definition, Fields};

pub use max_size::Error as SchemaMaxSerializedSizeError;
use max_size::{is_zero_size, ZeroSizeError};
pub use validate::Error as SchemaContainerValidateError;

mod max_size;
mod validate;
