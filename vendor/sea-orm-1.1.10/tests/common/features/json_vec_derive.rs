pub mod json_string_vec {
    use sea_orm::entity::prelude::*;
    use sea_orm::FromJsonQueryResult;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "json_string_vec")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub str_vec: Option<StringVec>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
    pub struct StringVec(pub Vec<String>);

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod json_struct_vec {
    use sea_orm::entity::prelude::*;
    use sea_orm_macros::FromJsonQueryResult;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "json_struct_vec")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        #[sea_orm(column_type = "JsonBinary")]
        pub struct_vec: Vec<JsonColumn>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
    pub struct JsonColumn {
        pub value: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
