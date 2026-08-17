use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::NSBundle;

impl UnwindSafe for NSBundle {}
impl RefUnwindSafe for NSBundle {}

impl NSBundle {
    #[cfg(feature = "NSString")]
    #[cfg(feature = "NSDictionary")]
    pub fn name(&self) -> Option<objc2::rc::Retained<crate::NSString>> {
        let info = self.infoDictionary()?;
        let name = info.objectForKey(crate::ns_string!("CFBundleName"))?;
        Some(name.downcast().expect("CFBundleName to be NSString"))
    }
}
