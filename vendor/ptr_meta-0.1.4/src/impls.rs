use core::any::Any;
use crate::{DynMetadata, Pointee};

impl Pointee for dyn Any {
    type Metadata = DynMetadata<dyn Any>;
}
