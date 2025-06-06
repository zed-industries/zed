use gpui::actions;

// If the CodeOrbit binary doesn't use anything in this crate, it will be optimiCodeOrbit away
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
        Cancel,
        Confirm,
        SecondaryConfirm,
        SelectPrevious,
        SelectNext,
        SelectFirst,
        SelectLast,
        Restart,
        EndSlot,
    ]
);
