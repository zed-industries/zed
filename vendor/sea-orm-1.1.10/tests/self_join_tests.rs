#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, query::*, DbBackend, IntoActiveModel, QueryOrder};

#[sea_orm_macros::test]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("self_join_tests").await;
    create_tables(&ctx.db).await?;
    create_metadata(&ctx.db).await?;
    ctx.delete().await;
    find_linked_001();
    find_also_linked_001();

    Ok(())
}

pub async fn create_metadata(db: &DatabaseConnection) -> Result<(), DbErr> {
    let model = self_join::Model {
        uuid: Uuid::new_v4(),
        uuid_ref: None,
        time: Some(Time::from_hms_opt(1, 00, 00).unwrap()),
    };

    model.clone().into_active_model().insert(db).await?;

    let linked_model = self_join::Model {
        uuid: Uuid::new_v4(),
        uuid_ref: Some(model.clone().uuid),
        time: Some(Time::from_hms_opt(2, 00, 00).unwrap()),
    };

    linked_model.clone().into_active_model().insert(db).await?;

    let not_linked_model = self_join::Model {
        uuid: Uuid::new_v4(),
        uuid_ref: None,
        time: Some(Time::from_hms_opt(3, 00, 00).unwrap()),
    };

    not_linked_model
        .clone()
        .into_active_model()
        .insert(db)
        .await?;

    assert_eq!(
        model
            .find_linked(self_join::SelfReferencingLink)
            .all(db)
            .await?,
        []
    );

    assert_eq!(
        linked_model
            .find_linked(self_join::SelfReferencingLink)
            .all(db)
            .await?,
        [model.clone()]
    );

    assert_eq!(
        not_linked_model
            .find_linked(self_join::SelfReferencingLink)
            .all(db)
            .await?,
        []
    );

    assert_eq!(
        self_join::Entity::find()
            .find_also_linked(self_join::SelfReferencingLink)
            .order_by_asc(self_join::Column::Time)
            .all(db)
            .await?,
        [
            (model.clone(), None),
            (linked_model, Some(model)),
            (not_linked_model, None),
        ]
    );

    Ok(())
}

fn find_linked_001() {
    use self_join::*;

    let self_join_model = Model {
        uuid: Uuid::default(),
        uuid_ref: None,
        time: None,
    };

    assert_eq!(
        self_join_model
            .find_linked(SelfReferencingLink)
            .build(DbBackend::MySql)
            .to_string(),
        [
            r#"SELECT `self_join`.`uuid`, `self_join`.`uuid_ref`, `self_join`.`time`"#,
            r#"FROM `self_join`"#,
            r#"INNER JOIN `self_join` AS `r0` ON `r0`.`uuid_ref` = `self_join`.`uuid`"#,
            r#"WHERE `r0`.`uuid` = '00000000-0000-0000-0000-000000000000'"#,
        ]
        .join(" ")
    );
}

fn find_also_linked_001() {
    use self_join::*;

    assert_eq!(
        Entity::find()
            .find_also_linked(SelfReferencingLink)
            .build(DbBackend::MySql)
            .to_string(),
        [
            r#"SELECT `self_join`.`uuid` AS `A_uuid`, `self_join`.`uuid_ref` AS `A_uuid_ref`, `self_join`.`time` AS `A_time`,"#,
            r#"`r0`.`uuid` AS `B_uuid`, `r0`.`uuid_ref` AS `B_uuid_ref`, `r0`.`time` AS `B_time`"#,
            r#"FROM `self_join`"#,
            r#"LEFT JOIN `self_join` AS `r0` ON `self_join`.`uuid_ref` = `r0`.`uuid`"#,
        ]
        .join(" ")
    );
}
