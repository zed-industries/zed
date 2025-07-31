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
        /// Cancels the current menu operation.
        Cancel,
        /// Confirms the selected menu item.
        Confirm,
        /// Performs secondary confirmation action.
        SecondaryConfirm,
        /// Selects the previous item in the menu.
        SelectPrevious,
        /// Selects the next item in the menu.
        SelectNext,
        /// Selects the first item in the menu.
        SelectFirst,
        /// Selects the last item in the menu.
        SelectLast,
        /// Restarts the menu from the beginning.
        Restart,
        EndSlot,
    ]
);
