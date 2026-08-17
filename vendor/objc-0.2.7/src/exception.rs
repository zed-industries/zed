use objc_exception;

use rc::StrongPtr;
use runtime::Object;

pub unsafe fn try<F, R>(closure: F) -> Result<R, StrongPtr>
        where F: FnOnce() -> R {
    objc_exception::try(closure).map_err(|exception| {
        StrongPtr::new(exception as *mut Object)
    })
}
