#![cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
use crate::NSSlider;
use objc2::extern_methods;

impl NSSlider {
    extern_methods!(
        #[unsafe(method(isVertical))]
        pub fn isVertical(&self) -> bool;
    );
}
