use crate as sea_orm;
use crate::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "rust_keyword")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub testing: i32,
    pub rust: i32,
    pub keywords: i32,
    pub r#raw_identifier: i32,
    pub r#as: i32,
    pub r#async: i32,
    pub r#await: i32,
    pub r#break: i32,
    pub r#const: i32,
    pub r#continue: i32,
    pub crate_: i32,
    pub r#dyn: i32,
    pub r#else: i32,
    pub r#enum: i32,
    pub r#extern: i32,
    pub r#false: i32,
    pub r#fn: i32,
    pub r#for: i32,
    pub r#if: i32,
    pub r#impl: i32,
    pub r#in: i32,
    pub r#let: i32,
    pub r#loop: i32,
    pub r#match: i32,
    pub r#mod: i32,
    pub r#move: i32,
    pub r#mut: i32,
    pub r#pub: i32,
    pub r#ref: i32,
    pub r#return: i32,
    pub self_: i32,
    pub r#static: i32,
    pub r#struct: i32,
    pub r#trait: i32,
    pub r#true: i32,
    pub r#type: i32,
    pub r#union: i32,
    pub r#unsafe: i32,
    pub r#use: i32,
    pub r#where: i32,
    pub r#while: i32,
    pub r#abstract: i32,
    pub r#become: i32,
    pub r#box: i32,
    pub r#do: i32,
    pub r#final: i32,
    pub r#macro: i32,
    pub r#override: i32,
    pub r#priv: i32,
    pub r#try: i32,
    pub r#typeof: i32,
    pub r#unsized: i32,
    pub r#virtual: i32,
    pub r#yield: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use crate::tests_cfg::rust_keyword::*;
    use sea_query::Iden;

    #[test]
    fn test_columns() {
        assert_eq!(Column::Id.to_string().as_str(), "id");
        assert_eq!(Column::Testing.to_string().as_str(), "testing");
        assert_eq!(Column::Rust.to_string().as_str(), "rust");
        assert_eq!(Column::Keywords.to_string().as_str(), "keywords");
        assert_eq!(Column::RawIdentifier.to_string().as_str(), "raw_identifier");
        assert_eq!(Column::As.to_string().as_str(), "as");
        assert_eq!(Column::Async.to_string().as_str(), "async");
        assert_eq!(Column::Await.to_string().as_str(), "await");
        assert_eq!(Column::Break.to_string().as_str(), "break");
        assert_eq!(Column::Const.to_string().as_str(), "const");
        assert_eq!(Column::Continue.to_string().as_str(), "continue");
        assert_eq!(Column::Dyn.to_string().as_str(), "dyn");
        assert_eq!(Column::Crate.to_string().as_str(), "crate");
        assert_eq!(Column::Else.to_string().as_str(), "else");
        assert_eq!(Column::Enum.to_string().as_str(), "enum");
        assert_eq!(Column::Extern.to_string().as_str(), "extern");
        assert_eq!(Column::False.to_string().as_str(), "false");
        assert_eq!(Column::Fn.to_string().as_str(), "fn");
        assert_eq!(Column::For.to_string().as_str(), "for");
        assert_eq!(Column::If.to_string().as_str(), "if");
        assert_eq!(Column::Impl.to_string().as_str(), "impl");
        assert_eq!(Column::In.to_string().as_str(), "in");
        assert_eq!(Column::Let.to_string().as_str(), "let");
        assert_eq!(Column::Loop.to_string().as_str(), "loop");
        assert_eq!(Column::Match.to_string().as_str(), "match");
        assert_eq!(Column::Mod.to_string().as_str(), "mod");
        assert_eq!(Column::Move.to_string().as_str(), "move");
        assert_eq!(Column::Mut.to_string().as_str(), "mut");
        assert_eq!(Column::Pub.to_string().as_str(), "pub");
        assert_eq!(Column::Ref.to_string().as_str(), "ref");
        assert_eq!(Column::Return.to_string().as_str(), "return");
        assert_eq!(Column::Self_.to_string().as_str(), "self");
        assert_eq!(Column::Static.to_string().as_str(), "static");
        assert_eq!(Column::Struct.to_string().as_str(), "struct");
        assert_eq!(Column::Trait.to_string().as_str(), "trait");
        assert_eq!(Column::True.to_string().as_str(), "true");
        assert_eq!(Column::Type.to_string().as_str(), "type");
        assert_eq!(Column::Union.to_string().as_str(), "union");
        assert_eq!(Column::Unsafe.to_string().as_str(), "unsafe");
        assert_eq!(Column::Use.to_string().as_str(), "use");
        assert_eq!(Column::Where.to_string().as_str(), "where");
        assert_eq!(Column::While.to_string().as_str(), "while");
        assert_eq!(Column::Abstract.to_string().as_str(), "abstract");
        assert_eq!(Column::Become.to_string().as_str(), "become");
        assert_eq!(Column::Box.to_string().as_str(), "box");
        assert_eq!(Column::Do.to_string().as_str(), "do");
        assert_eq!(Column::Final.to_string().as_str(), "final");
        assert_eq!(Column::Macro.to_string().as_str(), "macro");
        assert_eq!(Column::Override.to_string().as_str(), "override");
        assert_eq!(Column::Priv.to_string().as_str(), "priv");
        assert_eq!(Column::Try.to_string().as_str(), "try");
        assert_eq!(Column::Typeof.to_string().as_str(), "typeof");
        assert_eq!(Column::Unsized.to_string().as_str(), "unsized");
        assert_eq!(Column::Virtual.to_string().as_str(), "virtual");
        assert_eq!(Column::Yield.to_string().as_str(), "yield");
    }
}
