#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{
    entity::prelude::*, entity::*, DatabaseConnection, DerivePartialModel, FromQueryResult,
};
use serde_json::json;

#[cfg(feature = "postgres-vector")]
mod test {

    #[sea_orm_macros::test]
    async fn main() -> Result<(), DbErr> {
        let ctx = TestContext::new("embedding_tests").await;
        create_tables(&ctx.db).await?;
        insert_embedding(&ctx.db).await?;
        update_embedding(&ctx.db).await?;
        select_embedding(&ctx.db).await?;
        ctx.delete().await;

        Ok(())
    }

    pub async fn insert_embedding(db: &DatabaseConnection) -> Result<(), DbErr> {
        use embedding::*;

        assert_eq!(
            Model {
                id: 1,
                embedding: PgVector::from(vec![1.]),
            }
            .into_active_model()
            .insert(db)
            .await?,
            Model {
                id: 1,
                embedding: PgVector::from(vec![1.]),
            }
        );

        assert_eq!(
            Model {
                id: 2,
                embedding: PgVector::from(vec![1., 2.]),
            }
            .into_active_model()
            .insert(db)
            .await?,
            Model {
                id: 2,
                embedding: PgVector::from(vec![1., 2.]),
            }
        );

        assert_eq!(
            Model {
                id: 3,
                embedding: PgVector::from(vec![1., 2., 3.]),
            }
            .into_active_model()
            .insert(db)
            .await?,
            Model {
                id: 3,
                embedding: PgVector::from(vec![1., 2., 3.]),
            }
        );

        assert_eq!(
            Entity::find_by_id(3).into_json().one(db).await?,
            Some(json!({
                "id": 3,
                "embedding": [1., 2., 3.],
            }))
        );

        Ok(())
    }

    pub async fn update_embedding(db: &DatabaseConnection) -> Result<(), DbErr> {
        use embedding::*;

        let model = Entity::find_by_id(1).one(db).await?.unwrap();

        ActiveModel {
            embedding: Set(PgVector::from(vec![10.])),
            ..model.into_active_model()
        }
        .update(db)
        .await?;

        ActiveModel {
            id: Unchanged(3),
            embedding: Set(PgVector::from(vec![10., 20., 30.])),
        }
        .update(db)
        .await?;

        Ok(())
    }

    pub async fn select_embedding(db: &DatabaseConnection) -> Result<(), DbErr> {
        use embedding::*;

        #[derive(DerivePartialModel, FromQueryResult, Debug, PartialEq)]
        #[sea_orm(entity = "Entity")]
        struct PartialSelectResult {
            embedding: PgVector,
        }

        let result = Entity::find_by_id(1)
            .into_partial_model::<PartialSelectResult>()
            .one(db)
            .await?;

        assert_eq!(
            result,
            Some(PartialSelectResult {
                embedding: PgVector::from(vec![10.]),
            })
        );

        let result = Entity::find_by_id(2)
            .into_partial_model::<PartialSelectResult>()
            .one(db)
            .await?;

        assert_eq!(
            result,
            Some(PartialSelectResult {
                embedding: PgVector::from(vec![1., 2.]),
            })
        );

        let result = Entity::find_by_id(3)
            .into_partial_model::<PartialSelectResult>()
            .one(db)
            .await?;

        assert_eq!(
            result,
            Some(PartialSelectResult {
                embedding: PgVector::from(vec![10., 20., 30.]),
            })
        );

        Ok(())
    }
}
