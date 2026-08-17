//! This module contains functions references for reflection in generated code.

#![doc(hidden)]

pub use crate::reflect::acc::v2::map::make_map_simpler_accessor;
pub use crate::reflect::acc::v2::map::make_map_simpler_accessor_new;
pub use crate::reflect::acc::v2::repeated::make_vec_simpler_accessor;
pub use crate::reflect::acc::v2::singular::make_message_field_accessor;
pub use crate::reflect::acc::v2::singular::make_option_accessor;
pub use crate::reflect::acc::v2::singular::make_simpler_field_accessor;
pub use crate::reflect::acc::v2::singular::oneof::make_oneof_copy_has_get_set_simpler_accessors;
pub use crate::reflect::acc::v2::singular::oneof::make_oneof_deref_has_get_set_simpler_accessor;
pub use crate::reflect::acc::v2::singular::oneof::make_oneof_enum_accessors;
pub use crate::reflect::acc::v2::singular::oneof::make_oneof_message_has_get_mut_set_accessor;
