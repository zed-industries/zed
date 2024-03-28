use gpui::{anchored, deferred, Anchored, Deferred};

/// `overlay` gives you a floating element that will avoid overflowing the window bounds.
/// Its children should have no margin to avoid measurement issues.
///
/// `overlay` uses a combination of the `Deferred` and `Anchored` elements to achieve this.
pub fn overlay(anchored_builder: impl FnOnce(Anchored) -> Anchored) -> Deferred {
    deferred(anchored_builder(anchored()))
}
