// Take a look at the license at the top of the repository in the LICENSE file.

use std::num::NonZeroU32;

use objc2_io_kit::IOObjectRelease;

type IoObject = NonZeroU32;

pub(crate) struct IOReleaser(IoObject);

impl IOReleaser {
    pub(crate) fn new(obj: u32) -> Option<Self> {
        IoObject::new(obj).map(Self)
    }

    #[cfg(feature = "disk")]
    pub(crate) unsafe fn new_unchecked(obj: u32) -> Self {
        // Chance at catching in-development mistakes
        debug_assert_ne!(obj, 0);
        unsafe { Self(IoObject::new_unchecked(obj)) }
    }

    #[inline]
    pub(crate) fn inner(&self) -> u32 {
        self.0.get()
    }
}

impl Drop for IOReleaser {
    fn drop(&mut self) {
        unsafe { IOObjectRelease(self.0.get() as _) };
    }
}
