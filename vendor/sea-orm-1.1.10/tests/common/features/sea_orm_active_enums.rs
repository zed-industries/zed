use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum Category {
    #[sea_orm(string_value = "B")]
    Big,
    #[sea_orm(string_value = "S")]
    Small,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Color {
    #[sea_orm(num_value = 0)]
    Black,
    #[sea_orm(num_value = 1)]
    White,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, DeriveDisplay)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tea")]
pub enum Tea {
    #[sea_orm(string_value = "EverydayTea")]
    EverydayTea,
    #[sea_orm(string_value = "BreakfastTea")]
    BreakfastTea,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "media_type")]
pub enum MediaType {
    #[sea_orm(string_value = "UNKNOWN")]
    Unknown,
    #[sea_orm(string_value = "BITMAP")]
    Bitmap,
    #[sea_orm(string_value = "DRAWING")]
    Drawing,
    #[sea_orm(string_value = "AUDIO")]
    Audio,
    #[sea_orm(string_value = "VIDEO")]
    Video,
    #[sea_orm(string_value = "MULTIMEDIA")]
    Multimedia,
    #[sea_orm(string_value = "OFFICE")]
    Office,
    #[sea_orm(string_value = "TEXT")]
    Text,
    #[sea_orm(string_value = "EXECUTABLE")]
    Executable,
    #[sea_orm(string_value = "ARCHIVE")]
    Archive,
    #[sea_orm(string_value = "3D")]
    _3D,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, DeriveDisplay)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tea")]
pub enum DisplayTea {
    #[sea_orm(string_value = "EverydayTea", display_value = "Everyday")]
    EverydayTea,
    #[sea_orm(string_value = "BreakfastTea", display_value = "Breakfast")]
    BreakfastTea,
}
