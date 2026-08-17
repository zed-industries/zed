use crate::{NSEventMask, NSEventType};

impl NSEventMask {
    #[doc(alias = "NSEventMaskFromType")]
    pub fn from_type(ty: NSEventType) -> Self {
        Self(1 << ty.0)
    }
}
