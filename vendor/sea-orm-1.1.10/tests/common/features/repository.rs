use super::edit_log;
use sea_orm::{entity::prelude::*, ConnectionTrait, Set, TryIntoModel};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "repository")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, db: &C, _: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let model = self.clone().try_into_model()?;
        insert_edit_log("before_save", &model, db).await?;
        Ok(self)
    }

    async fn after_save<C>(model: Model, db: &C, _: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        insert_edit_log("after_save", &model, db).await?;
        Ok(model)
    }

    async fn before_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let model = self.clone().try_into_model()?;
        insert_edit_log("before_delete", &model, db).await?;
        Ok(self)
    }

    async fn after_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let model = self.clone().try_into_model()?;
        insert_edit_log("after_delete", &model, db).await?;
        Ok(self)
    }
}

async fn insert_edit_log<T, M, C>(action: T, model: &M, db: &C) -> Result<(), DbErr>
where
    T: Into<String>,
    M: Serialize,
    C: ConnectionTrait,
{
    edit_log::ActiveModel {
        action: Set(action.into()),
        values: Set(serde_json::json!(model)),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(())
}
