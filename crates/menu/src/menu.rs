#[derive(Clone, PartialEq)]
pub struct SelectIndex(pub usize);

gpui::actions!(
    menu,
    [
        Cancel,
        Confirm,
        SelectPrev,
        SelectNext,
        SelectFirst,
        SelectLast,
        Autocomplete
    ]
);

gpui::impl_internal_actions!(menu, [SelectIndex]);
