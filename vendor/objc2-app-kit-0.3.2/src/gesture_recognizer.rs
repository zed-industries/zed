use crate::{NSGestureRecognizer, NSGestureRecognizerState};
use objc2::extern_methods;

impl NSGestureRecognizer {
    extern_methods!(
        #[unsafe(method(state))]
        pub fn state(&self) -> NSGestureRecognizerState;
    );
}
