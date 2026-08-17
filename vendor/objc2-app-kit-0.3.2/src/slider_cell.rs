#![cfg(all(feature = "NSActionCell", feature = "NSCell"))]
use crate::NSSliderCell;
use objc2::extern_methods;

impl NSSliderCell {
    extern_methods!(
        #[unsafe(method(isVertical))]
        pub fn isVertical(&self) -> bool;
    );
}
