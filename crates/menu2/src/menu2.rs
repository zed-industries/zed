use gpui::actions;

// todo!(remove this)
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn unused() {}

actions!(
    Cancel,
    Confirm,
    SecondaryConfirm,
    SelectPrev,
    SelectNext,
    SelectFirst,
    SelectLast,
    ShowContextMenu
);
