use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "baker")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub contact_details: Json,
    pub bakery_id: Option<i32>,
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
}

impl Related<super::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<super::cake::Entity> for Entity {
    fn to() -> RelationDef {
        super::cakes_bakers::Relation::Cake.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::cakes_bakers::Relation::Baker.def().rev())
    }
}

pub struct BakedForCustomer;

impl Linked for BakedForCustomer {
    type FromEntity = Entity;

    type ToEntity = super::customer::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::cakes_bakers::Relation::Baker.def().rev(),
            super::cakes_bakers::Relation::Cake.def(),
            super::lineitem::Relation::Cake.def().rev(),
            super::lineitem::Relation::Order.def(),
            super::order::Relation::Customer.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
