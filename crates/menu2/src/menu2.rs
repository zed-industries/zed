use serde_derive::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Cancel;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Confirm;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SecondaryConfirm;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SelectPrev;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SelectNext;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SelectFirst;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct SelectLast;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ShowContextMenu;
