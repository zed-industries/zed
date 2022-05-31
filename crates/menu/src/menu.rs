#[derive(Clone)]
pub struct SelectIndex(pub usize);

gpui::actions!(
    menu,
    [
        Cancel,
        Confirm,
        SelectPrev,
        SelectNext,
        SelectFirst,
        SelectLast
    ]
);

gpui::impl_internal_actions!(menu, [SelectIndex]);
