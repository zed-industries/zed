use super::*;
use crate::common::setup::{create_enum, create_table, create_table_without_asserts};
use sea_orm::{
    error::*, sea_query, ConnectionTrait, DatabaseConnection, DbBackend, DbConn, EntityName,
    ExecResult, Schema,
};
use sea_query::{
    extension::postgres::Type, Alias, ColumnDef, ColumnType, ForeignKeyCreateStatement, IntoIden,
    StringLen,
};

pub async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let db_backend = db.get_database_backend();

    create_log_table(db).await?;
    create_metadata_table(db).await?;
    create_repository_table(db).await?;
    create_self_join_table(db).await?;
    create_byte_primary_key_table(db).await?;
    create_satellites_table(db).await?;
    create_transaction_log_table(db).await?;

    let create_enum_stmts = match db_backend {
        DbBackend::MySql | DbBackend::Sqlite => Vec::new(),
        DbBackend::Postgres => {
            let schema = Schema::new(db_backend);
            let enum_create_stmt = Type::create()
                .as_enum(Alias::new("tea"))
                .values([Alias::new("EverydayTea"), Alias::new("BreakfastTea")])
                .to_owned();
            assert_eq!(
                db_backend.build(&enum_create_stmt),
                db_backend.build(&schema.create_enum_from_active_enum::<Tea>())
            );
            vec![enum_create_stmt]
        }
    };
    create_enum(db, &create_enum_stmts, ActiveEnum).await?;

    create_active_enum_table(db).await?;
    create_active_enum_child_table(db).await?;
    create_insert_default_table(db).await?;
    create_pi_table(db).await?;
    create_uuid_fmt_table(db).await?;
    create_edit_log_table(db).await?;
    create_teas_table(db).await?;
    create_binary_table(db).await?;
    if matches!(db_backend, DbBackend::Postgres) {
        create_bits_table(db).await?;
    }
    create_dyn_table_name_lazy_static_table(db).await?;
    create_value_type_table(db).await?;

    create_json_vec_table(db).await?;
    create_json_struct_table(db).await?;
    create_json_string_vec_table(db).await?;
    create_json_struct_vec_table(db).await?;

    if DbBackend::Postgres == db_backend {
        create_value_type_postgres_table(db).await?;
        create_collection_table(db).await?;
        create_event_trigger_table(db).await?;
        create_categories_table(db).await?;
        #[cfg(feature = "postgres-vector")]
        create_embedding_table(db).await?;
        create_host_network_table(db).await?;
    }

    Ok(())
}

pub async fn create_log_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(applog::Entity)
        .comment("app logs")
        .col(
            ColumnDef::new(applog::Column::Id)
                .integer()
                .not_null()
                .comment("ID")
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(applog::Column::Action)
                .string()
                .not_null()
                .comment("action"),
        )
        .col(
            ColumnDef::new(applog::Column::Json)
                .json()
                .not_null()
                .comment("action data"),
        )
        .col(
            ColumnDef::new(applog::Column::CreatedAt)
                .timestamp_with_time_zone()
                .not_null()
                .comment("create time"),
        )
        .to_owned();

    create_table(db, &stmt, Applog).await
}

pub async fn create_metadata_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(metadata::Entity)
        .col(
            ColumnDef::new(metadata::Column::Uuid)
                .uuid()
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(metadata::Column::Type).string().not_null())
        .col(ColumnDef::new(metadata::Column::Key).string().not_null())
        .col(ColumnDef::new(metadata::Column::Value).string().not_null())
        .col(
            ColumnDef::new_with_type(
                metadata::Column::Bytes,
                ColumnType::VarBinary(StringLen::N(32)),
            )
            .not_null(),
        )
        .col(ColumnDef::new(metadata::Column::Date).date())
        .col(ColumnDef::new(metadata::Column::Time).time())
        .to_owned();

    create_table(db, &stmt, Metadata).await
}

pub async fn create_repository_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(repository::Entity)
        .col(
            ColumnDef::new(repository::Column::Id)
                .string()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(repository::Column::Owner)
                .string()
                .not_null(),
        )
        .col(ColumnDef::new(repository::Column::Name).string().not_null())
        .col(ColumnDef::new(repository::Column::Description).string())
        .to_owned();

    create_table(db, &stmt, Repository).await
}

pub async fn create_self_join_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(self_join::Entity)
        .col(
            ColumnDef::new(self_join::Column::Uuid)
                .uuid()
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(self_join::Column::UuidRef).uuid())
        .col(ColumnDef::new(self_join::Column::Time).time())
        .foreign_key(
            ForeignKeyCreateStatement::new()
                .name("fk-self_join-uuid_ref")
                .from_tbl(SelfJoin)
                .from_col(self_join::Column::UuidRef)
                .to_tbl(SelfJoin)
                .to_col(self_join::Column::Uuid),
        )
        .to_owned();

    create_table(db, &stmt, SelfJoin).await
}

pub async fn create_byte_primary_key_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let mut primary_key_col = ColumnDef::new(byte_primary_key::Column::Id);
    match db.get_database_backend() {
        DbBackend::MySql => primary_key_col.binary_len(3),
        DbBackend::Sqlite | DbBackend::Postgres => primary_key_col.binary(),
    };

    let stmt = sea_query::Table::create()
        .table(byte_primary_key::Entity)
        .col(primary_key_col.not_null().primary_key())
        .col(
            ColumnDef::new(byte_primary_key::Column::Value)
                .string()
                .not_null(),
        )
        .to_owned();

    create_table_without_asserts(db, &stmt).await
}

pub async fn create_active_enum_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(active_enum::Entity.table_ref())
        .col(
            ColumnDef::new(active_enum::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(active_enum::Column::Category).string_len(1))
        .col(ColumnDef::new(active_enum::Column::Color).integer())
        .col(
            ColumnDef::new(active_enum::Column::Tea)
                .enumeration(TeaEnum, [TeaVariant::EverydayTea, TeaVariant::BreakfastTea]),
        )
        .to_owned();

    create_table(db, &create_table_stmt, ActiveEnum).await
}

pub async fn create_active_enum_child_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(active_enum_child::Entity.table_ref())
        .col(
            ColumnDef::new(active_enum_child::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(active_enum_child::Column::ParentId)
                .integer()
                .not_null(),
        )
        .col(ColumnDef::new(active_enum_child::Column::Category).string_len(1))
        .col(ColumnDef::new(active_enum_child::Column::Color).integer())
        .col(
            ColumnDef::new(active_enum_child::Column::Tea)
                .enumeration(TeaEnum, [TeaVariant::EverydayTea, TeaVariant::BreakfastTea]),
        )
        .foreign_key(
            ForeignKeyCreateStatement::new()
                .name("fk-active_enum_child-active_enum")
                .from_tbl(ActiveEnumChild)
                .from_col(active_enum_child::Column::ParentId)
                .to_tbl(ActiveEnum)
                .to_col(active_enum::Column::Id),
        )
        .to_owned();

    create_table(db, &create_table_stmt, ActiveEnumChild).await
}

pub async fn create_satellites_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(satellite::Entity)
        .col(
            ColumnDef::new(satellite::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(satellite::Column::SatelliteName)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(satellite::Column::LaunchDate)
                .timestamp_with_time_zone()
                .not_null()
                .default("2022-01-26 16:24:00"),
        )
        .col(
            ColumnDef::new(satellite::Column::DeploymentDate)
                .timestamp_with_time_zone()
                .not_null()
                .default("2022-01-26 16:24:00"),
        )
        .to_owned();

    create_table(db, &stmt, Satellite).await
}

pub async fn create_transaction_log_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(transaction_log::Entity)
        .col(
            ColumnDef::new(transaction_log::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(transaction_log::Column::Date)
                .date()
                .not_null(),
        )
        .col(
            ColumnDef::new(transaction_log::Column::Time)
                .time()
                .not_null(),
        )
        .col(
            ColumnDef::new(transaction_log::Column::DateTime)
                .date_time()
                .not_null(),
        )
        .col(
            ColumnDef::new(transaction_log::Column::DateTimeTz)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt, TransactionLog).await
}

pub async fn create_insert_default_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(insert_default::Entity.table_ref())
        .col(
            ColumnDef::new(insert_default::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .to_owned();

    create_table(db, &create_table_stmt, InsertDefault).await
}

pub async fn create_json_vec_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(json_vec::Entity.table_ref())
        .col(
            ColumnDef::new(json_vec::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(json_vec::Column::StrVec).json())
        .to_owned();

    create_table(db, &create_table_stmt, JsonVec).await
}

pub async fn create_json_struct_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(json_struct::Entity)
        .col(
            ColumnDef::new(json_struct::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(json_struct::Column::Json).json().not_null())
        .col(
            ColumnDef::new(json_struct::Column::JsonValue)
                .json()
                .not_null(),
        )
        .col(ColumnDef::new(json_struct::Column::JsonValueOpt).json())
        .to_owned();

    create_table(db, &stmt, JsonStruct).await
}

pub async fn create_json_string_vec_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(JsonStringVec.table_ref())
        .col(
            ColumnDef::new(json_vec_derive::json_string_vec::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(json_vec_derive::json_string_vec::Column::StrVec).json())
        .to_owned();

    create_table(db, &create_table_stmt, JsonStringVec).await
}

pub async fn create_json_struct_vec_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(JsonStructVec.table_ref())
        .col(
            ColumnDef::new(json_vec_derive::json_struct_vec::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(json_vec_derive::json_struct_vec::Column::StructVec)
                .json_binary()
                .not_null(),
        )
        .to_owned();

    create_table(db, &create_table_stmt, JsonStructVec).await
}

pub async fn create_collection_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    db.execute(sea_orm::Statement::from_string(
        db.get_database_backend(),
        "CREATE EXTENSION IF NOT EXISTS citext",
    ))
    .await?;

    let stmt = sea_query::Table::create()
        .table(collection::Entity)
        .col(
            ColumnDef::new(collection::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(collection::Column::Name)
                .custom(Alias::new("citext"))
                .not_null(),
        )
        .col(
            ColumnDef::new(collection::Column::Integers)
                .array(sea_query::ColumnType::Integer)
                .not_null(),
        )
        .col(ColumnDef::new(collection::Column::IntegersOpt).array(sea_query::ColumnType::Integer))
        .col(
            ColumnDef::new(collection::Column::Teas)
                .array(sea_query::ColumnType::Enum {
                    name: TeaEnum.into_iden(),
                    variants: vec![
                        TeaVariant::EverydayTea.into_iden(),
                        TeaVariant::BreakfastTea.into_iden(),
                    ],
                })
                .not_null(),
        )
        .col(
            ColumnDef::new(collection::Column::TeasOpt).array(sea_query::ColumnType::Enum {
                name: TeaEnum.into_iden(),
                variants: vec![
                    TeaVariant::EverydayTea.into_iden(),
                    TeaVariant::BreakfastTea.into_iden(),
                ],
            }),
        )
        .col(
            ColumnDef::new(collection::Column::Colors)
                .array(sea_query::ColumnType::Integer)
                .not_null(),
        )
        .col(ColumnDef::new(collection::Column::ColorsOpt).array(sea_query::ColumnType::Integer))
        .col(
            ColumnDef::new(collection::Column::Uuid)
                .array(sea_query::ColumnType::Uuid)
                .not_null(),
        )
        .col(
            ColumnDef::new(collection::Column::UuidHyphenated)
                .array(sea_query::ColumnType::Uuid)
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt, Collection).await
}

pub async fn create_host_network_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(host_network::Entity)
        .col(
            ColumnDef::new(host_network::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(host_network::Column::Hostname)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(host_network::Column::Ipaddress)
                .inet()
                .not_null(),
        )
        .col(
            ColumnDef::new(host_network::Column::Network)
                .cidr()
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt, HostNetwork).await
}

pub async fn create_pi_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(pi::Entity)
        .col(
            ColumnDef::new(pi::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(pi::Column::Decimal)
                .decimal_len(11, 10)
                .not_null(),
        )
        .col(
            ColumnDef::new(pi::Column::BigDecimal)
                .decimal_len(11, 10)
                .not_null(),
        )
        .col(ColumnDef::new(pi::Column::DecimalOpt).decimal_len(11, 10))
        .col(ColumnDef::new(pi::Column::BigDecimalOpt).decimal_len(11, 10))
        .to_owned();

    create_table(db, &stmt, Pi).await
}

pub async fn create_event_trigger_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(event_trigger::Entity)
        .col(
            ColumnDef::new(event_trigger::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(event_trigger::Column::Events)
                .array(sea_query::ColumnType::String(StringLen::None))
                .not_null(),
        )
        .to_owned();

    create_table(db, &stmt, EventTrigger).await
}

pub async fn create_uuid_fmt_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(uuid_fmt::Entity)
        .col(
            ColumnDef::new(uuid_fmt::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(uuid_fmt::Column::Uuid).uuid().not_null())
        .col(
            ColumnDef::new(uuid_fmt::Column::UuidBraced)
                .uuid()
                .not_null(),
        )
        .col(
            ColumnDef::new(uuid_fmt::Column::UuidHyphenated)
                .uuid()
                .not_null(),
        )
        .col(
            ColumnDef::new(uuid_fmt::Column::UuidSimple)
                .uuid()
                .not_null(),
        )
        .col(ColumnDef::new(uuid_fmt::Column::UuidUrn).uuid().not_null())
        .to_owned();

    create_table(db, &stmt, UuidFmt).await
}

pub async fn create_edit_log_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = sea_query::Table::create()
        .table(edit_log::Entity)
        .col(
            ColumnDef::new(edit_log::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(edit_log::Column::Action).string().not_null())
        .col(ColumnDef::new(edit_log::Column::Values).json().not_null())
        .to_owned();

    create_table(db, &stmt, EditLog).await
}

pub async fn create_teas_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(teas::Entity.table_ref())
        .col(
            ColumnDef::new(teas::Column::Id)
                .enumeration(TeaEnum, [TeaVariant::EverydayTea, TeaVariant::BreakfastTea])
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(teas::Column::Category).string_len(1))
        .col(ColumnDef::new(teas::Column::Color).integer())
        .to_owned();

    create_table(db, &create_table_stmt, Teas).await
}

pub async fn create_categories_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(categories::Entity.table_ref())
        .col(
            ColumnDef::new(categories::Column::Id)
                .integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(categories::Column::Categories)
                .array(ColumnType::String(StringLen::N(1))),
        )
        .to_owned();

    create_table(db, &create_table_stmt, Categories).await
}

#[cfg(feature = "postgres-vector")]
pub async fn create_embedding_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    db.execute(sea_orm::Statement::from_string(
        db.get_database_backend(),
        "CREATE EXTENSION IF NOT EXISTS vector",
    ))
    .await?;

    let create_table_stmt = sea_query::Table::create()
        .table(embedding::Entity.table_ref())
        .col(
            ColumnDef::new(embedding::Column::Id)
                .integer()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(embedding::Column::Embedding)
                .vector(None)
                .not_null(),
        )
        .to_owned();

    create_table(db, &create_table_stmt, Embedding).await
}

pub async fn create_binary_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(binary::Entity.table_ref())
        .col(
            ColumnDef::new(binary::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(binary::Column::Binary).binary().not_null())
        .col(
            ColumnDef::new(binary::Column::Binary10)
                .binary_len(10)
                .not_null(),
        )
        .col(
            ColumnDef::new(binary::Column::VarBinary16)
                .var_binary(16)
                .not_null(),
        )
        .to_owned();

    create_table(db, &create_table_stmt, Binary).await
}

pub async fn create_bits_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let create_table_stmt = sea_query::Table::create()
        .table(bits::Entity.table_ref())
        .col(
            ColumnDef::new(bits::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(bits::Column::Bit0)
                .custom(Alias::new("BIT"))
                .not_null(),
        )
        .col(
            ColumnDef::new(bits::Column::Bit1)
                .custom(Alias::new("BIT(1)"))
                .not_null(),
        )
        .col(
            ColumnDef::new(bits::Column::Bit8)
                .custom(Alias::new("BIT(8)"))
                .not_null(),
        )
        .col(
            ColumnDef::new(bits::Column::Bit16)
                .custom(Alias::new("BIT(16)"))
                .not_null(),
        )
        .col(
            ColumnDef::new(bits::Column::Bit32)
                .custom(Alias::new("BIT(32)"))
                .not_null(),
        )
        .col(
            ColumnDef::new(bits::Column::Bit64)
                .custom(Alias::new("BIT(64)"))
                .not_null(),
        )
        .to_owned();

    create_table(db, &create_table_stmt, Bits).await
}

pub async fn create_dyn_table_name_lazy_static_table(db: &DbConn) -> Result<(), DbErr> {
    use dyn_table_name_lazy_static::*;

    let entities = [
        Entity {
            table_name: TableName::from_str_truncate("dyn_table_name_lazy_static_1"),
        },
        Entity {
            table_name: TableName::from_str_truncate("dyn_table_name_lazy_static_2"),
        },
    ];
    for entity in entities {
        let create_table_stmt = sea_query::Table::create()
            .table(entity.table_ref())
            .col(
                ColumnDef::new(Column::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Column::Name).string().not_null())
            .to_owned();

        create_table(db, &create_table_stmt, entity).await?;
    }

    Ok(())
}

pub async fn create_value_type_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let general_stmt = sea_query::Table::create()
        .table(value_type::value_type_general::Entity)
        .col(
            ColumnDef::new(value_type::value_type_general::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(value_type::value_type_general::Column::Number)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(value_type::value_type_general::Column::Tag1)
                .string()
                .not_null(),
        )
        .col(
            ColumnDef::new(value_type::value_type_general::Column::Tag2)
                .string()
                .not_null(),
        )
        .to_owned();

    create_table(db, &general_stmt, value_type::value_type_general::Entity).await
}

pub async fn create_value_type_postgres_table(db: &DbConn) -> Result<ExecResult, DbErr> {
    let postgres_stmt = sea_query::Table::create()
        .table(value_type::value_type_pg::Entity)
        .col(
            ColumnDef::new(value_type::value_type_pg::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(value_type::value_type_pg::Column::Number)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(value_type::value_type_pg::Column::StrVec)
                .array(sea_query::ColumnType::String(StringLen::None))
                .not_null(),
        )
        .to_owned();

    create_table(db, &postgres_stmt, value_type::value_type_pg::Entity).await
}
