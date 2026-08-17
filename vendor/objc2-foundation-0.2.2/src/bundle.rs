use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use crate::Foundation::NSBundle;

impl UnwindSafe for NSBundle {}
impl RefUnwindSafe for NSBundle {}

impl NSBundle {
    #[cfg(feature = "NSString")]
    #[cfg(feature = "NSDictionary")]
    #[cfg(feature = "NSObject")]
    pub fn name(&self) -> Option<objc2::rc::Retained<crate::Foundation::NSString>> {
        use crate::Foundation::{NSCopying, NSString};
        use objc2::runtime::AnyObject;

        let info = self.infoDictionary()?;
        // TODO: Use ns_string!
        let name = info.get(&NSString::from_str("CFBundleName"))?;
        let ptr: *const AnyObject = name;
        let ptr: *const NSString = ptr.cast();
        // SAFETY: TODO
        let name = unsafe { ptr.as_ref().unwrap_unchecked() };
        Some(name.copy())
    }
}

impl fmt::Debug for NSBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to NSObject
        (**self).fmt(f)
    }
}
