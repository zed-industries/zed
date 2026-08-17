use crate::common::setup::rust_dec;
use sea_orm::{entity::prelude::*, ConnectionTrait};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "cake")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Decimal(Some((16, 4)))")]
    pub price: Decimal,
    pub bakery_id: Option<i32>,
    pub gluten_free: bool,
    pub serial: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bakery::Entity",
        from = "Column::BakeryId",
        to = "super::bakery::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Bakery,
    #[sea_orm(has_many = "super::lineitem::Entity")]
    Lineitem,
}

impl Related<super::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<super::baker::Entity> for Entity {
    fn to() -> RelationDef {
        super::cakes_bakers::Relation::Baker.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::cakes_bakers::Relation::Cake.def().rev())
    }
}

impl Related<super::lineitem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lineitem.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        use sea_orm::Set;
        Self {
            serial: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if self.price.as_ref() == &rust_dec(0) {
            Err(DbErr::Custom(format!(
                "[before_save] Invalid Price, insert: {insert}"
            )))
        } else {
            Ok(self)
        }
    }

    async fn after_save<C>(model: Model, _db: &C, insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        if model.price < rust_dec(0) {
            Err(DbErr::Custom(format!(
                "[after_save] Invalid Price, insert: {insert}"
            )))
        } else {
            Ok(model)
        }
    }

    async fn before_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if self.name.as_ref().contains("(err_on_before_delete)") {
            Err(DbErr::Custom(
                "[before_delete] Cannot be deleted".to_owned(),
            ))
        } else {
            Ok(self)
        }
    }

    async fn after_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if self.name.as_ref().contains("(err_on_after_delete)") {
            Err(DbErr::Custom("[after_delete] Cannot be deleted".to_owned()))
        } else {
            Ok(self)
        }
    }
}
