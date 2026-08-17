#![allow(unused_imports, dead_code)]

pub mod common;

pub use sea_orm::{entity::*, error::*, query::*, sea_query, tests_cfg::*, Database, DbConn};

// cargo test --features sqlx-sqlite,runtime-async-std-native-tls --test basic
// export DATABASE_URL=mysql://root:root@localhost:3306
// export DATABASE_URL=sqlite::memory:
#[sea_orm_macros::test]
#[cfg(feature = "sqlx-sqlite")]
async fn main() -> Result<(), DbErr> {
    dotenv::from_filename(".env.local").ok();
    dotenv::from_filename(".env").ok();

    let base_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());

    let db: DbConn = Database::connect(&base_url).await?;
    setup_schema(&db).await?;
    crud_cake(&db).await?;

    Ok(())
}

#[cfg(feature = "sqlx-sqlite")]
async fn setup_schema(db: &DbConn) -> Result<(), DbErr> {
    use sea_query::*;

    let stmt = sea_query::Table::create()
        .table(cake::Entity)
        .col(
            ColumnDef::new(cake::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(cake::Column::Name).string())
        .to_owned();

    let builder = db.get_database_backend();
    let result = db.execute(builder.build(&stmt)).await?;
    println!("Create table cake: {result:?}");

    Ok(())
}

#[cfg(feature = "sqlx-sqlite")]
async fn crud_cake(db: &DbConn) -> Result<(), DbErr> {
    let apple = cake::ActiveModel {
        name: Set("Apple Pie".to_owned()),
        ..Default::default()
    };

    let mut apple = apple.save(db).await?;

    println!();
    println!("Inserted: {apple:?}");

    assert_eq!(
        apple,
        cake::ActiveModel {
            id: Unchanged(1),
            name: Unchanged("Apple Pie".to_owned()),
        }
    );

    apple.name = Set("Lemon Tart".to_owned());

    let apple = apple.save(db).await?;

    println!();
    println!("Updated: {apple:?}");

    let count = cake::Entity::find().count(db).await?;

    println!();
    println!("Count: {count:?}");
    assert_eq!(count, 1);

    let apple = cake::Entity::find_by_id(1).one(db).await?;

    assert_eq!(
        Some(cake::Model {
            id: 1,
            name: "Lemon Tart".to_owned(),
        }),
        apple
    );

    let apple: cake::Model = apple.unwrap();

    let result = apple.delete(db).await?;

    println!();
    println!("Deleted: {result:?}");

    let apple = cake::Entity::find_by_id(1).one(db).await?;

    assert_eq!(None, apple);

    let count = cake::Entity::find().count(db).await?;

    println!();
    println!("Count: {count:?}");
    assert_eq!(count, 0);

    Ok(())
}
