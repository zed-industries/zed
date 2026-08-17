#![allow(unused_imports, dead_code)]

pub mod common;
pub use common::{features::*, setup::*, TestContext};
use sea_orm::entity::prelude::*;

#[derive(DeriveIden)]
pub enum ClassName {
    Table,
    Id,
    Title,
    Text,
}

#[derive(DeriveIden)]
pub enum Book {
    #[sea_orm(iden = "book_table")]
    Table,
    Id,
    #[sea_orm(iden = "turtle")]
    Title,
    #[sea_orm(iden = "TeXt")]
    Text,
    #[sea_orm(iden = "ty_pe")]
    Type,
}

#[derive(DeriveIden)]
struct GlyphToken;

#[derive(DeriveIden)]
#[sea_orm(iden = "weRd")]
struct Word;

#[test]
fn main() -> Result<(), DbErr> {
    assert_eq!(ClassName::Table.to_string(), "class_name");
    assert_eq!(ClassName::Id.to_string(), "id");
    assert_eq!(ClassName::Title.to_string(), "title");
    assert_eq!(ClassName::Text.to_string(), "text");

    assert_eq!(Book::Id.to_string(), "id");
    assert_eq!(Book::Table.to_string(), "book_table");
    assert_eq!(Book::Title.to_string(), "turtle");
    assert_eq!(Book::Text.to_string(), "TeXt");
    assert_eq!(Book::Type.to_string(), "ty_pe");

    assert_eq!(GlyphToken.to_string(), "glyph_token");

    assert_eq!(Word.to_string(), "weRd");
    Ok(())
}
