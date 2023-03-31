#[derive(Clone, PartialEq)]
pub struct SelectIndex(pub usize);

gpui::actions!(
    menu,
    [
        Cancel,
        Confirm,
        SelectPrevious,
        SelectNext,
        SelectFirst,
        SelectLast
    ]
);

gpui::impl_internal_actions!(menu, [SelectIndex]);
