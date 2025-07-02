use gpui::actions;

// If the zed binary doesn't use anything in this crate, it will be optimized away
// and the actions won't initialize. So we just provide an empty initialization function
// to be called from main.
//
// These may provide relevant context:
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

actions!(
    menu,
    [
        /// Cancel the current menu operation
        Cancel,
        /// Confirm the selected menu item
        Confirm,
        /// Perform secondary confirmation action
        SecondaryConfirm,
        /// Select the previous item in the menu
        SelectPrevious,
        /// Select the next item in the menu
        SelectNext,
        /// Select the first item in the menu
        SelectFirst,
        /// Select the last item in the menu
        SelectLast,
        /// Restart the menu from the beginning
        Restart,
        EndSlot,
    ]
);
