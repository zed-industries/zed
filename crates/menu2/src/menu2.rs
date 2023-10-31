// todo!(use actions! macro)

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cancel;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Confirm;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SecondaryConfirm;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectPrev;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectNext;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectFirst;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectLast;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShowContextMenu;
