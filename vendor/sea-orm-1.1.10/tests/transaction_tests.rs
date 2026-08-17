#![allow(unused_imports, dead_code)]

pub mod common;

pub use common::{bakery_chain::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{prelude::*, AccessMode, DatabaseTransaction, IsolationLevel, Set, TransactionTrait};

#[sea_orm_macros::test]
pub async fn transaction() {
    let ctx = TestContext::new("transaction_test").await;
    create_tables(&ctx.db).await.unwrap();

    ctx.db
        .transaction::<_, _, DbErr>(|txn| {
            Box::pin(async move {
                let _ = bakery::ActiveModel {
                    name: Set("SeaSide Bakery".to_owned()),
                    profit_margin: Set(10.4),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                let _ = bakery::ActiveModel {
                    name: Set("Top Bakery".to_owned()),
                    profit_margin: Set(15.0),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                let bakeries = Bakery::find()
                    .filter(bakery::Column::Name.contains("Bakery"))
                    .all(txn)
                    .await?;

                assert_eq!(bakeries.len(), 2);

                Ok(())
            })
        })
        .await
        .unwrap();

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn transaction_with_reference() {
    let ctx = TestContext::new("transaction_with_reference_test").await;
    create_tables(&ctx.db).await.unwrap();

    let name1 = "SeaSide Bakery";
    let name2 = "Top Bakery";
    let search_name = "Bakery";
    ctx.db
        .transaction(|txn| _transaction_with_reference(txn, name1, name2, search_name))
        .await
        .unwrap();

    ctx.delete().await;
}

fn _transaction_with_reference<'a>(
    txn: &'a DatabaseTransaction,
    name1: &'a str,
    name2: &'a str,
    search_name: &'a str,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DbErr>> + Send + 'a>> {
    Box::pin(async move {
        let _ = bakery::ActiveModel {
            name: Set(name1.to_owned()),
            profit_margin: Set(10.4),
            ..Default::default()
        }
        .save(txn)
        .await?;

        let _ = bakery::ActiveModel {
            name: Set(name2.to_owned()),
            profit_margin: Set(15.0),
            ..Default::default()
        }
        .save(txn)
        .await?;

        let bakeries = Bakery::find()
            .filter(bakery::Column::Name.contains(search_name))
            .all(txn)
            .await?;

        assert_eq!(bakeries.len(), 2);

        Ok(())
    })
}

#[sea_orm_macros::test]
pub async fn transaction_begin_out_of_scope() -> Result<(), DbErr> {
    let ctx = TestContext::new("transaction_begin_out_of_scope_test").await;
    create_tables(&ctx.db).await?;

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 0);

    {
        // Transaction begin in this scope
        let txn = ctx.db.begin().await?;

        bakery::ActiveModel {
            name: Set("SeaSide Bakery".to_owned()),
            profit_margin: Set(10.4),
            ..Default::default()
        }
        .save(&txn)
        .await?;

        assert_eq!(bakery::Entity::find().all(&txn).await?.len(), 1);

        bakery::ActiveModel {
            name: Set("Top Bakery".to_owned()),
            profit_margin: Set(15.0),
            ..Default::default()
        }
        .save(&txn)
        .await?;

        assert_eq!(bakery::Entity::find().all(&txn).await?.len(), 2);

        // The scope ended and transaction is dropped without commit
    }

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 0);

    ctx.delete().await;
    Ok(())
}

#[sea_orm_macros::test]
pub async fn transaction_begin_commit() -> Result<(), DbErr> {
    let ctx = TestContext::new("transaction_begin_commit_test").await;
    create_tables(&ctx.db).await?;

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 0);

    {
        // Transaction begin in this scope
        let txn = ctx.db.begin().await?;

        bakery::ActiveModel {
            name: Set("SeaSide Bakery".to_owned()),
            profit_margin: Set(10.4),
            ..Default::default()
        }
        .save(&txn)
        .await?;

        assert_eq!(bakery::Entity::find().all(&txn).await?.len(), 1);

        bakery::ActiveModel {
            name: Set("Top Bakery".to_owned()),
            profit_margin: Set(15.0),
            ..Default::default()
        }
        .save(&txn)
        .await?;

        assert_eq!(bakery::Entity::find().all(&txn).await?.len(), 2);

        // Commit changes before the end of scope
        txn.commit().await?;
    }

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 2);

    ctx.delete().await;
    Ok(())
}

#[sea_orm_macros::test]
pub async fn transaction_begin_rollback() -> Result<(), DbErr> {
    let ctx = TestContext::new("transaction_begin_rollback_test").await;
    create_tables(&ctx.db).await?;

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 0);

    {
        // Transaction begin in this scope
        let txn = ctx.db.begin().await?;

        bakery::ActiveModel {
            name: Set("SeaSide Bakery".to_owned()),
            profit_margin: Set(10.4),
            ..Default::default()
        }
        .save(&txn)
        .await?;

        assert_eq!(bakery::Entity::find().all(&txn).await?.len(), 1);

        bakery::ActiveModel {
            name: Set("Top Bakery".to_owned()),
            profit_margin: Set(15.0),
            ..Default::default()
        }
        .save(&txn)
        .await?;

        assert_eq!(bakery::Entity::find().all(&txn).await?.len(), 2);

        // Rollback changes before the end of scope
        txn.rollback().await?;
    }

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 0);

    ctx.delete().await;
    Ok(())
}

#[sea_orm_macros::test]
pub async fn transaction_closure_commit() -> Result<(), DbErr> {
    let ctx = TestContext::new("transaction_closure_commit_test").await;
    create_tables(&ctx.db).await?;

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 0);

    let res = ctx
        .db
        .transaction::<_, _, DbErr>(|txn| {
            Box::pin(async move {
                bakery::ActiveModel {
                    name: Set("SeaSide Bakery".to_owned()),
                    profit_margin: Set(10.4),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                assert_eq!(bakery::Entity::find().all(txn).await?.len(), 1);

                bakery::ActiveModel {
                    name: Set("Top Bakery".to_owned()),
                    profit_margin: Set(15.0),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                assert_eq!(bakery::Entity::find().all(txn).await?.len(), 2);

                Ok(())
            })
        })
        .await;

    assert!(res.is_ok());

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 2);

    ctx.delete().await;
    Ok(())
}

#[sea_orm_macros::test]
pub async fn transaction_closure_rollback() -> Result<(), DbErr> {
    let ctx = TestContext::new("transaction_closure_rollback_test").await;
    create_tables(&ctx.db).await?;

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 0);

    let res = ctx
        .db
        .transaction::<_, _, DbErr>(|txn| {
            Box::pin(async move {
                bakery::ActiveModel {
                    name: Set("SeaSide Bakery".to_owned()),
                    profit_margin: Set(10.4),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                assert_eq!(bakery::Entity::find().all(txn).await?.len(), 1);

                bakery::ActiveModel {
                    name: Set("Top Bakery".to_owned()),
                    profit_margin: Set(15.0),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                assert_eq!(bakery::Entity::find().all(txn).await?.len(), 2);

                bakery::ActiveModel {
                    id: Set(1),
                    name: Set("Duplicated primary key".to_owned()),
                    profit_margin: Set(20.0),
                }
                .insert(txn)
                .await?; // Throw error and rollback

                // This line won't be reached
                unreachable!();

                #[allow(unreachable_code)]
                Ok(())
            })
        })
        .await;

    assert!(res.is_err());

    assert_eq!(bakery::Entity::find().all(&ctx.db).await?.len(), 0);

    ctx.delete().await;
    Ok(())
}

#[sea_orm_macros::test]
pub async fn transaction_with_active_model_behaviour() -> Result<(), DbErr> {
    let ctx = TestContext::new("transaction_with_active_model_behaviour_test").await;
    create_tables(&ctx.db).await?;

    if let Ok(txn) = ctx.db.begin().await {
        assert_eq!(
            cake::ActiveModel {
                name: Set("Cake with invalid price".to_owned()),
                price: Set(rust_dec(0)),
                gluten_free: Set(false),
                ..Default::default()
            }
            .save(&txn)
            .await,
            Err(DbErr::Custom(
                "[before_save] Invalid Price, insert: true".to_owned()
            ))
        );

        assert_eq!(cake::Entity::find().all(&txn).await?.len(), 0);

        assert_eq!(
            cake::ActiveModel {
                name: Set("Cake with invalid price".to_owned()),
                price: Set(rust_dec(-10)),
                gluten_free: Set(false),
                ..Default::default()
            }
            .save(&txn)
            .await,
            Err(DbErr::Custom(
                "[after_save] Invalid Price, insert: true".to_owned()
            ))
        );

        assert_eq!(cake::Entity::find().all(&txn).await?.len(), 1);

        let readonly_cake_1 = cake::ActiveModel {
            name: Set("Readonly cake (err_on_before_delete)".to_owned()),
            price: Set(rust_dec(10)),
            gluten_free: Set(true),
            ..Default::default()
        }
        .save(&txn)
        .await?;

        assert_eq!(cake::Entity::find().all(&txn).await?.len(), 2);

        assert_eq!(
            readonly_cake_1.delete(&txn).await.err(),
            Some(DbErr::Custom(
                "[before_delete] Cannot be deleted".to_owned()
            ))
        );

        assert_eq!(cake::Entity::find().all(&txn).await?.len(), 2);

        let readonly_cake_2 = cake::ActiveModel {
            name: Set("Readonly cake (err_on_after_delete)".to_owned()),
            price: Set(rust_dec(10)),
            gluten_free: Set(true),
            ..Default::default()
        }
        .save(&txn)
        .await?;

        assert_eq!(cake::Entity::find().all(&txn).await?.len(), 3);

        assert_eq!(
            readonly_cake_2.delete(&txn).await.err(),
            Some(DbErr::Custom("[after_delete] Cannot be deleted".to_owned()))
        );

        assert_eq!(cake::Entity::find().all(&txn).await?.len(), 2);
    }

    assert_eq!(cake::Entity::find().all(&ctx.db).await?.len(), 0);

    ctx.delete().await;
    Ok(())
}

#[sea_orm_macros::test]
pub async fn transaction_nested() {
    let ctx = TestContext::new("transaction_nested_test").await;
    create_tables(&ctx.db).await.unwrap();

    ctx.db
        .transaction::<_, _, DbErr>(|txn| {
            Box::pin(async move {
                let _ = bakery::ActiveModel {
                    name: Set("SeaSide Bakery".to_owned()),
                    profit_margin: Set(10.4),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                let _ = bakery::ActiveModel {
                    name: Set("Top Bakery".to_owned()),
                    profit_margin: Set(15.0),
                    ..Default::default()
                }
                .save(txn)
                .await?;

                // Try nested transaction committed
                txn.transaction::<_, _, DbErr>(|txn| {
                    Box::pin(async move {
                        let _ = bakery::ActiveModel {
                            name: Set("Nested Bakery".to_owned()),
                            profit_margin: Set(88.88),
                            ..Default::default()
                        }
                        .save(txn)
                        .await?;

                        let bakeries = Bakery::find()
                            .filter(bakery::Column::Name.contains("Bakery"))
                            .all(txn)
                            .await?;

                        assert_eq!(bakeries.len(), 3);

                        // Try nested-nested transaction rollbacked
                        let is_err = txn
                            .transaction::<_, _, DbErr>(|txn| {
                                Box::pin(async move {
                                    let _ = bakery::ActiveModel {
                                        name: Set("Rock n Roll Bakery".to_owned()),
                                        profit_margin: Set(28.8),
                                        ..Default::default()
                                    }
                                    .save(txn)
                                    .await?;

                                    let bakeries = Bakery::find()
                                        .filter(bakery::Column::Name.contains("Bakery"))
                                        .all(txn)
                                        .await?;

                                    assert_eq!(bakeries.len(), 4);

                                    if true {
                                        Err(DbErr::Query(RuntimeErr::Internal(
                                            "Force Rollback!".to_owned(),
                                        )))
                                    } else {
                                        Ok(())
                                    }
                                })
                            })
                            .await
                            .is_err();

                        assert!(is_err);

                        let bakeries = Bakery::find()
                            .filter(bakery::Column::Name.contains("Bakery"))
                            .all(txn)
                            .await?;

                        assert_eq!(bakeries.len(), 3);

                        // Try nested-nested transaction committed
                        txn.transaction::<_, _, DbErr>(|txn| {
                            Box::pin(async move {
                                let _ = bakery::ActiveModel {
                                    name: Set("Rock n Roll Bakery".to_owned()),
                                    profit_margin: Set(28.8),
                                    ..Default::default()
                                }
                                .save(txn)
                                .await?;

                                let bakeries = Bakery::find()
                                    .filter(bakery::Column::Name.contains("Bakery"))
                                    .all(txn)
                                    .await?;

                                assert_eq!(bakeries.len(), 4);

                                Ok(())
                            })
                        })
                        .await
                        .unwrap();

                        let bakeries = Bakery::find()
                            .filter(bakery::Column::Name.contains("Bakery"))
                            .all(txn)
                            .await?;

                        assert_eq!(bakeries.len(), 4);

                        Ok(())
                    })
                })
                .await
                .unwrap();

                // Try nested transaction rollbacked
                let is_err = txn
                    .transaction::<_, _, DbErr>(|txn| {
                        Box::pin(async move {
                            let _ = bakery::ActiveModel {
                                name: Set("Rock n Roll Bakery".to_owned()),
                                profit_margin: Set(28.8),
                                ..Default::default()
                            }
                            .save(txn)
                            .await?;

                            let bakeries = Bakery::find()
                                .filter(bakery::Column::Name.contains("Bakery"))
                                .all(txn)
                                .await?;

                            assert_eq!(bakeries.len(), 5);

                            // Try nested-nested transaction committed
                            txn.transaction::<_, _, DbErr>(|txn| {
                                Box::pin(async move {
                                    let _ = bakery::ActiveModel {
                                        name: Set("Rock n Roll Bakery".to_owned()),
                                        profit_margin: Set(28.8),
                                        ..Default::default()
                                    }
                                    .save(txn)
                                    .await?;

                                    let bakeries = Bakery::find()
                                        .filter(bakery::Column::Name.contains("Bakery"))
                                        .all(txn)
                                        .await?;

                                    assert_eq!(bakeries.len(), 6);

                                    Ok(())
                                })
                            })
                            .await
                            .unwrap();

                            let bakeries = Bakery::find()
                                .filter(bakery::Column::Name.contains("Bakery"))
                                .all(txn)
                                .await?;

                            assert_eq!(bakeries.len(), 6);

                            // Try nested-nested transaction rollbacked
                            let is_err = txn
                                .transaction::<_, _, DbErr>(|txn| {
                                    Box::pin(async move {
                                        let _ = bakery::ActiveModel {
                                            name: Set("Rock n Roll Bakery".to_owned()),
                                            profit_margin: Set(28.8),
                                            ..Default::default()
                                        }
                                        .save(txn)
                                        .await?;

                                        let bakeries = Bakery::find()
                                            .filter(bakery::Column::Name.contains("Bakery"))
                                            .all(txn)
                                            .await?;

                                        assert_eq!(bakeries.len(), 7);

                                        if true {
                                            Err(DbErr::Query(RuntimeErr::Internal(
                                                "Force Rollback!".to_owned(),
                                            )))
                                        } else {
                                            Ok(())
                                        }
                                    })
                                })
                                .await
                                .is_err();

                            assert!(is_err);

                            let bakeries = Bakery::find()
                                .filter(bakery::Column::Name.contains("Bakery"))
                                .all(txn)
                                .await?;

                            assert_eq!(bakeries.len(), 6);

                            if true {
                                Err(DbErr::Query(RuntimeErr::Internal(
                                    "Force Rollback!".to_owned(),
                                )))
                            } else {
                                Ok(())
                            }
                        })
                    })
                    .await
                    .is_err();

                assert!(is_err);

                let bakeries = Bakery::find()
                    .filter(bakery::Column::Name.contains("Bakery"))
                    .all(txn)
                    .await?;

                assert_eq!(bakeries.len(), 4);

                Ok(())
            })
        })
        .await
        .unwrap();

    let bakeries = Bakery::find()
        .filter(bakery::Column::Name.contains("Bakery"))
        .all(&ctx.db)
        .await
        .unwrap();

    assert_eq!(bakeries.len(), 4);

    ctx.delete().await;
}

#[sea_orm_macros::test]
pub async fn transaction_with_config() {
    let ctx = TestContext::new("transaction_with_config").await;
    create_tables(&ctx.db).await.unwrap();

    for (i, (isolation_level, access_mode)) in [
        (IsolationLevel::RepeatableRead, None),
        (IsolationLevel::ReadCommitted, None),
        (IsolationLevel::ReadUncommitted, Some(AccessMode::ReadWrite)),
        (IsolationLevel::Serializable, Some(AccessMode::ReadWrite)),
    ]
    .into_iter()
    .enumerate()
    {
        let name1 = format!("SeaSide Bakery {}", i);
        let name2 = format!("Top Bakery {}", i);
        let search_name = format!("Bakery {}", i);
        ctx.db
            .transaction_with_config(
                |txn| _transaction_with_config(txn, name1, name2, search_name),
                Some(isolation_level),
                access_mode,
            )
            .await
            .unwrap();
    }

    ctx.db
        .transaction_with_config::<_, _, DbErr>(
            |txn| {
                Box::pin(async move {
                    let bakeries = Bakery::find()
                        .filter(bakery::Column::Name.contains("Bakery"))
                        .all(txn)
                        .await?;

                    assert_eq!(bakeries.len(), 8);

                    Ok(())
                })
            },
            None,
            Some(AccessMode::ReadOnly),
        )
        .await
        .unwrap();

    ctx.delete().await;
}

fn _transaction_with_config<'a>(
    txn: &'a DatabaseTransaction,
    name1: String,
    name2: String,
    search_name: String,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DbErr>> + Send + 'a>> {
    Box::pin(async move {
        let _ = bakery::ActiveModel {
            name: Set(name1),
            profit_margin: Set(10.4),
            ..Default::default()
        }
        .save(txn)
        .await?;

        let _ = bakery::ActiveModel {
            name: Set(name2),
            profit_margin: Set(15.0),
            ..Default::default()
        }
        .save(txn)
        .await?;

        let bakeries = Bakery::find()
            .filter(bakery::Column::Name.contains(&search_name))
            .all(txn)
            .await?;

        assert_eq!(bakeries.len(), 2);

        Ok(())
    })
}
