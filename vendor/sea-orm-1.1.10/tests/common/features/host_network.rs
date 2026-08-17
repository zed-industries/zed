use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "host_network")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub hostname: String,
    pub ipaddress: IpNetwork,
    #[sea_orm(column_type = "Cidr")]
    pub network: IpNetwork,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
