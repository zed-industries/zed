use sqlx::error::DatabaseError;
use sqlx::sqlite::{SqliteConnectOptions, SqliteError};
use sqlx::ConnectOptions;
use sqlx::TypeInfo;
use sqlx::{sqlite::Sqlite, Column, Executor};
use sqlx_test::new;
use std::env;

#[sqlx_macros::test]
async fn it_describes_simple() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let info = conn.describe("SELECT * FROM tweet").await?;
    let columns = info.columns();

    assert_eq!(columns[0].name(), "id");
    assert_eq!(columns[1].name(), "text");
    assert_eq!(columns[2].name(), "is_sent");
    assert_eq!(columns[3].name(), "owner_id");

    assert_eq!(columns[0].ordinal(), 0);
    assert_eq!(columns[1].ordinal(), 1);
    assert_eq!(columns[2].ordinal(), 2);
    assert_eq!(columns[3].ordinal(), 3);

    assert_eq!(info.nullable(0), Some(false));
    assert_eq!(info.nullable(1), Some(false));
    assert_eq!(info.nullable(2), Some(false));
    assert_eq!(info.nullable(3), Some(true)); // owner_id

    assert_eq!(columns[0].type_info().name(), "INTEGER");
    assert_eq!(columns[1].type_info().name(), "TEXT");
    assert_eq!(columns[2].type_info().name(), "BOOLEAN");
    assert_eq!(columns[3].type_info().name(), "INTEGER");

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_variables() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    // without any context, we resolve to NULL
    let info = conn.describe("SELECT ?1").await?;

    assert_eq!(info.columns()[0].type_info().name(), "NULL");
    assert_eq!(info.nullable(0), Some(true)); // nothing prevents the value from being bound to null

    // context can be provided by using CAST(_ as _)
    let info = conn.describe("SELECT CAST(?1 AS REAL)").await?;

    assert_eq!(info.columns()[0].type_info().name(), "REAL");
    assert_eq!(info.nullable(0), Some(true)); // nothing prevents the value from being bound to null

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_expression() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn
        .describe("SELECT 1 + 10, 5.12 * 2, 'Hello', x'deadbeef', null")
        .await?;

    let columns = d.columns();

    assert_eq!(columns[0].type_info().name(), "INTEGER");
    assert_eq!(columns[0].name(), "1 + 10");
    assert_eq!(d.nullable(0), Some(false)); // literal constant

    assert_eq!(columns[1].type_info().name(), "REAL");
    assert_eq!(columns[1].name(), "5.12 * 2");
    assert_eq!(d.nullable(1), Some(false)); // literal constant

    assert_eq!(columns[2].type_info().name(), "TEXT");
    assert_eq!(columns[2].name(), "'Hello'");
    assert_eq!(d.nullable(2), Some(false)); // literal constant

    assert_eq!(columns[3].type_info().name(), "BLOB");
    assert_eq!(columns[3].name(), "x'deadbeef'");
    assert_eq!(d.nullable(3), Some(false)); // literal constant

    assert_eq!(columns[4].type_info().name(), "NULL");
    assert_eq!(columns[4].name(), "null");
    assert_eq!(d.nullable(4), Some(true)); // literal null

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_temporary_table() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    conn.execute(
        "CREATE TEMPORARY TABLE IF NOT EXISTS empty_all_types_and_nulls(
        i1 integer NULL,
        r1 real NULL,
        t1 text NULL,
        b1 blob NULL,
        i2 INTEGER NOT NULL,
        r2 REAL NOT NULL,
        t2 TEXT NOT NULL,
        b2 BLOB NOT NULL
        )",
    )
    .await?;

    let d = conn
        .describe("SELECT * FROM empty_all_types_and_nulls")
        .await?;
    assert_eq!(d.columns().len(), 8);

    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    assert_eq!(d.column(1).type_info().name(), "REAL");
    assert_eq!(d.nullable(1), Some(true));

    assert_eq!(d.column(2).type_info().name(), "TEXT");
    assert_eq!(d.nullable(2), Some(true));

    assert_eq!(d.column(3).type_info().name(), "BLOB");
    assert_eq!(d.nullable(3), Some(true));

    assert_eq!(d.column(4).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(4), Some(false));

    assert_eq!(d.column(5).type_info().name(), "REAL");
    assert_eq!(d.nullable(5), Some(false));

    assert_eq!(d.column(6).type_info().name(), "TEXT");
    assert_eq!(d.nullable(6), Some(false));

    assert_eq!(d.column(7).type_info().name(), "BLOB");
    assert_eq!(d.nullable(7), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_expression_from_empty_table() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    conn.execute("CREATE TEMP TABLE _temp_empty ( name TEXT NOT NULL, a INT )")
        .await?;

    let d = conn
        .describe("SELECT COUNT(*), a + 1, name, 5.12, 'Hello' FROM _temp_empty")
        .await?;

    assert_eq!(d.columns()[0].type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false)); // COUNT(*)

    assert_eq!(d.columns()[1].type_info().name(), "INTEGER");
    assert_eq!(d.nullable(1), Some(true)); // `a+1` is nullable, because a is nullable

    assert_eq!(d.columns()[2].type_info().name(), "TEXT");
    assert_eq!(d.nullable(2), Some(true)); // `name` is not nullable, but the query can be null due to zero rows

    assert_eq!(d.columns()[3].type_info().name(), "REAL");
    assert_eq!(d.nullable(3), Some(false)); // literal constant

    assert_eq!(d.columns()[4].type_info().name(), "TEXT");
    assert_eq!(d.nullable(4), Some(false)); // literal constant

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_expression_from_empty_table_with_star() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    conn.execute("CREATE TEMP TABLE _temp_empty ( name TEXT, a INT )")
        .await?;

    let d = conn
        .describe("SELECT *, 5, 'Hello' FROM _temp_empty")
        .await?;

    assert_eq!(d.columns()[0].type_info().name(), "TEXT");
    assert_eq!(d.columns()[1].type_info().name(), "INTEGER");
    assert_eq!(d.columns()[2].type_info().name(), "INTEGER");
    assert_eq!(d.columns()[3].type_info().name(), "TEXT");

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_insert() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn
        .describe("INSERT INTO tweet (id, text) VALUES (2, 'Hello')")
        .await?;

    assert_eq!(d.columns().len(), 0);

    let d = conn
        .describe("INSERT INTO tweet (id, text) VALUES (2, 'Hello'); SELECT last_insert_rowid();")
        .await?;

    assert_eq!(d.columns().len(), 1);
    assert_eq!(d.columns()[0].type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_insert_with_read_only() -> anyhow::Result<()> {
    sqlx_test::setup_if_needed();

    let mut options: SqliteConnectOptions = env::var("DATABASE_URL")?.parse().unwrap();
    options = options.read_only(true);

    let mut conn = options.connect().await?;

    let d = conn
        .describe("INSERT INTO tweet (id, text) VALUES (2, 'Hello')")
        .await?;

    assert_eq!(d.columns().len(), 0);

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_insert_with_returning() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn
        .describe("INSERT INTO tweet (id, text) VALUES (2, 'Hello') RETURNING *")
        .await?;

    assert_eq!(d.columns().len(), 4);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));
    assert_eq!(d.column(1).type_info().name(), "TEXT");
    assert_eq!(d.nullable(1), Some(false));

    let d = conn
        .describe("INSERT INTO accounts (name, is_active) VALUES ('a', true) RETURNING id")
        .await?;

    assert_eq!(d.columns().len(), 1);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_bound_columns_non_null() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;
    let d = conn
        .describe("INSERT INTO tweet (id, text) VALUES ($1, $2) returning *")
        .await?;

    assert_eq!(d.columns().len(), 4);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));
    assert_eq!(d.column(1).type_info().name(), "TEXT");
    assert_eq!(d.nullable(1), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_update_with_returning() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn
        .describe("UPDATE accounts SET is_active=true WHERE name=?1 RETURNING id")
        .await?;

    assert_eq!(d.columns().len(), 1);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("UPDATE accounts SET is_active=true WHERE id=?1 RETURNING *")
        .await?;

    assert_eq!(d.columns().len(), 3);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));
    assert_eq!(d.column(1).type_info().name(), "TEXT");
    assert_eq!(d.nullable(1), Some(false));
    assert_eq!(d.column(2).type_info().name(), "BOOLEAN");
    //assert_eq!(d.nullable(2), Some(false)); //query analysis is allowed to notice that it is always set to true by the update

    let d = conn
        .describe("UPDATE accounts SET is_active=true WHERE id=?1 RETURNING id")
        .await?;

    assert_eq!(d.columns().len(), 1);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_delete_with_returning() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn
        .describe("DELETE FROM accounts WHERE name=?1 RETURNING id")
        .await?;

    assert_eq!(d.columns().len(), 1);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_bad_statement() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let err = conn.describe("SELECT 1 FROM not_found").await.unwrap_err();
    let err = err
        .as_database_error()
        .unwrap()
        .downcast_ref::<SqliteError>();

    assert_eq!(err.message(), "no such table: not_found");
    assert_eq!(err.code().as_deref(), Some("1"));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_left_join() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn.describe("select accounts.id from accounts").await?;

    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("select tweet.id from accounts left join tweet on owner_id = accounts.id")
        .await?;

    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe(
            "select tweet.id, accounts.id from accounts left join tweet on owner_id = accounts.id",
        )
        .await?;

    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    assert_eq!(d.column(1).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(1), Some(false));

    let d = conn
        .describe(
            "select tweet.id, accounts.id from accounts inner join tweet on owner_id = accounts.id",
        )
        .await?;

    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    assert_eq!(d.column(1).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(1), Some(false));

    let d = conn
        .describe(
            "select tweet.id, accounts.id from accounts left join tweet on tweet.id = accounts.id",
        )
        .await?;

    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    assert_eq!(d.column(1).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(1), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_group_by() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn.describe("select id from accounts group by id").await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("SELECT name from accounts GROUP BY 1 LIMIT -1 OFFSET 1")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "TEXT");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("SELECT sum(id), sum(is_sent) from tweet GROUP BY owner_id")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));
    assert_eq!(d.column(1).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(1), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_ungrouped_aggregate() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn.describe("select count(1) from accounts").await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn.describe("SELECT sum(is_sent) from tweet").await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe("SELECT coalesce(sum(is_sent),0) from tweet")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_literal_subquery() -> anyhow::Result<()> {
    async fn assert_literal_described(
        conn: &mut sqlx::SqliteConnection,
        query: &str,
    ) -> anyhow::Result<()> {
        let info = conn.describe(query).await?;

        assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
        assert_eq!(info.nullable(0), Some(false), "{query}");
        assert_eq!(info.column(1).type_info().name(), "NULL", "{query}");
        assert_eq!(info.nullable(1), Some(true), "{query}");

        Ok(())
    }

    let mut conn = new::<Sqlite>().await?;
    assert_literal_described(&mut conn, "SELECT 'a', NULL").await?;
    assert_literal_described(&mut conn, "SELECT * FROM (SELECT 'a', NULL)").await?;
    assert_literal_described(
        &mut conn,
        "WITH cte AS (SELECT 'a', NULL) SELECT * FROM cte",
    )
    .await?;
    assert_literal_described(
        &mut conn,
        "WITH cte AS MATERIALIZED (SELECT 'a', NULL) SELECT * FROM cte",
    )
    .await?;
    assert_literal_described(
        &mut conn,
        "WITH RECURSIVE cte(a,b) AS (SELECT 'a', NULL UNION ALL SELECT a||a, NULL FROM cte WHERE length(a)<3) SELECT * FROM cte",
    )
    .await?;

    Ok(())
}

async fn assert_tweet_described(
    conn: &mut sqlx::SqliteConnection,
    query: &str,
) -> anyhow::Result<()> {
    let info = conn.describe(query).await?;
    let columns = info.columns();

    assert_eq!(columns[0].name(), "id", "{query}");
    assert_eq!(columns[1].name(), "text", "{query}");
    assert_eq!(columns[2].name(), "is_sent", "{query}");
    assert_eq!(columns[3].name(), "owner_id", "{query}");

    assert_eq!(columns[0].ordinal(), 0, "{query}");
    assert_eq!(columns[1].ordinal(), 1, "{query}");
    assert_eq!(columns[2].ordinal(), 2, "{query}");
    assert_eq!(columns[3].ordinal(), 3, "{query}");

    assert_eq!(info.nullable(0), Some(false), "{query}");
    assert_eq!(info.nullable(1), Some(false), "{query}");
    assert_eq!(info.nullable(2), Some(false), "{query}");
    assert_eq!(info.nullable(3), Some(true), "{query}");

    assert_eq!(columns[0].type_info().name(), "INTEGER", "{query}");
    assert_eq!(columns[1].type_info().name(), "TEXT", "{query}");
    assert_eq!(columns[2].type_info().name(), "BOOLEAN", "{query}");
    assert_eq!(columns[3].type_info().name(), "INTEGER", "{query}");

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_table_subquery() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;
    assert_tweet_described(&mut conn, "SELECT * FROM tweet").await?;
    assert_tweet_described(&mut conn, "SELECT * FROM (SELECT * FROM tweet)").await?;
    assert_tweet_described(
        &mut conn,
        "WITH cte AS (SELECT * FROM tweet) SELECT * FROM cte",
    )
    .await?;
    assert_tweet_described(
        &mut conn,
        "WITH cte AS MATERIALIZED (SELECT * FROM tweet) SELECT * FROM cte",
    )
    .await?;

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_table_order_by() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;
    assert_tweet_described(&mut conn, "SELECT * FROM tweet ORDER BY id").await?;
    assert_tweet_described(&mut conn, "SELECT * FROM tweet ORDER BY id NULLS LAST").await?;
    assert_tweet_described(
        &mut conn,
        "SELECT * FROM tweet ORDER BY owner_id DESC, text ASC",
    )
    .await?;

    async fn assert_literal_order_by_described(
        conn: &mut sqlx::SqliteConnection,
        query: &str,
    ) -> anyhow::Result<()> {
        let info = conn.describe(query).await?;

        assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
        assert_eq!(info.nullable(0), Some(false), "{query}");
        assert_eq!(info.column(1).type_info().name(), "TEXT", "{query}");
        assert_eq!(info.nullable(1), Some(false), "{query}");

        Ok(())
    }

    assert_literal_order_by_described(&mut conn, "SELECT 'a', text FROM tweet ORDER BY id").await?;
    assert_literal_order_by_described(
        &mut conn,
        "SELECT 'a', text FROM tweet ORDER BY id NULLS LAST",
    )
    .await?;
    assert_literal_order_by_described(&mut conn, "SELECT 'a', text FROM tweet ORDER BY text")
        .await?;
    assert_literal_order_by_described(
        &mut conn,
        "SELECT 'a', text FROM tweet ORDER BY text NULLS LAST",
    )
    .await?;
    assert_literal_order_by_described(
        &mut conn,
        "SELECT 'a', text FROM tweet ORDER BY text DESC NULLS LAST",
    )
    .await?;

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_union() -> anyhow::Result<()> {
    async fn assert_union_described(
        conn: &mut sqlx::SqliteConnection,
        query: &str,
    ) -> anyhow::Result<()> {
        let info = conn.describe(query).await?;

        assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
        assert_eq!(info.nullable(0), Some(false), "{query}");
        assert_eq!(info.column(1).type_info().name(), "TEXT", "{query}");
        assert_eq!(info.nullable(1), Some(true), "{query}");
        assert_eq!(info.column(2).type_info().name(), "INTEGER", "{query}");
        assert_eq!(info.nullable(2), Some(true), "{query}");
        //TODO: mixed type columns not handled correctly
        //assert_eq!(info.column(3).type_info().name(), "NULL", "{query}");
        //assert_eq!(info.nullable(3), Some(false), "{query}");

        Ok(())
    }

    let mut conn = new::<Sqlite>().await?;
    assert_union_described(
        &mut conn,
        "SELECT 'txt','a',null,'b' UNION ALL SELECT 'int',NULL,1,2 ",
    )
    .await?;

    //TODO: insert into temp-table not merging datatype/nullable of all operations - currently keeping last-writer
    //assert_union_described(&mut conn, "SELECT 'txt','a',null,'b' UNION SELECT 'int',NULL,1,2 ").await?;

    assert_union_described(
        &mut conn,
        "SELECT 'tweet',text,owner_id id,null from tweet
        UNION SELECT 'account',name,id,is_active from accounts
        UNION SELECT 'account',name,id,is_active from accounts_view
        UNION SELECT 'dummy',null,null,null
        ORDER BY id
        ",
    )
    .await?;

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_having_group_by() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn
        .describe(
            r#"
        WITH tweet_reply_unq as ( --tweets with a single response
          SELECT tweet_id id
          FROM tweet_reply
          GROUP BY tweet_id
          HAVING COUNT(1) = 1
        ) 
        SELECT 
          (
    		  SELECT COUNT(*) 
    		  FROM (
    		    SELECT NULL
    			FROM tweet
    			JOIN tweet_reply_unq
    			  USING (id)
                WHERE tweet.owner_id = accounts.id
              )
          ) single_reply_count
        FROM accounts
        WHERE id = ?1
        "#,
        )
        .await?;

    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    Ok(())
}

//documents failures originally found through property testing
#[sqlx_macros::test]
async fn it_describes_strange_queries() -> anyhow::Result<()> {
    async fn assert_single_column_described(
        conn: &mut sqlx::SqliteConnection,
        query: &str,
        typename: &str,
        nullable: bool,
    ) -> anyhow::Result<()> {
        let info = conn.describe(query).await?;
        assert_eq!(info.column(0).type_info().name(), typename, "{query}");
        assert_eq!(info.nullable(0), Some(nullable), "{query}");

        Ok(())
    }

    let mut conn = new::<Sqlite>().await?;

    assert_single_column_described(
        &mut conn,
        "SELECT true FROM (SELECT true) a ORDER BY true",
        "INTEGER",
        false,
    )
    .await?;

    assert_single_column_described(
        &mut conn,
        "
    	SELECT true
    	FROM (
    	    SELECT 'a'
    	)
    	CROSS JOIN (
    	    SELECT 'b'
    	    FROM (SELECT 'c')
            CROSS JOIN accounts
            ORDER BY id
            LIMIT 1
            )
    	",
        "INTEGER",
        false,
    )
    .await?;

    assert_single_column_described(
        &mut conn,
        "SELECT true FROM tweet
            ORDER BY true ASC NULLS LAST",
        "INTEGER",
        false,
    )
    .await?;

    assert_single_column_described(
        &mut conn,
        "SELECT true LIMIT -1 OFFSET -1",
        "INTEGER",
        false,
    )
    .await?;

    assert_single_column_described(
        &mut conn,
        "SELECT true FROM tweet J LIMIT 10 OFFSET 1000000",
        "INTEGER",
        false,
    )
    .await?;

    assert_single_column_described(
        &mut conn,
        "SELECT text
        FROM (SELECT null)
        CROSS JOIN (
            SELECT text
            FROM tweet 
            GROUP BY text
        )
        LIMIT -1 OFFSET -1",
        "TEXT",
        false,
    )
    .await?;

    assert_single_column_described(
        &mut conn,
        "SELECT EYH.id,COUNT(EYH.id)
    	FROM accounts EYH",
        "INTEGER",
        true,
    )
    .await?;

    assert_single_column_described(
        &mut conn,
        "SELECT SUM(tweet.text) FROM (SELECT NULL FROM accounts_view LIMIT -1 OFFSET 1) CROSS JOIN tweet",
        "REAL",
        true, // null if accounts view has fewer rows than the offset
    )
    .await?;

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_func_date() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let query = "SELECT date();";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(false), "{query}");

    let query = "SELECT date('now');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT date('now', 'start of month');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT date(:datebind);";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}");
    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_func_time() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let query = "SELECT time();";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(false), "{query}");

    let query = "SELECT time('now');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT time('now', 'start of month');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT time(:datebind);";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}");
    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_func_datetime() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let query = "SELECT datetime();";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(false), "{query}");

    let query = "SELECT datetime('now');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT datetime('now', 'start of month');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT datetime(:datebind);";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}");
    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_func_julianday() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let query = "SELECT julianday();";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "REAL", "{query}");
    assert_eq!(info.nullable(0), Some(false), "{query}");

    let query = "SELECT julianday('now');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "REAL", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT julianday('now', 'start of month');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "REAL", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT julianday(:datebind);";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "REAL", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}");
    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_func_strftime() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let query = "SELECT strftime('%s','now');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT strftime('%s', 'now', 'start of month');";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}"); //can't prove that it's not-null yet

    let query = "SELECT strftime('%s',:datebind);";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}");
    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_with_recursive() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let query = "
        WITH RECURSIVE schedule(begin_date) AS (
             SELECT datetime('2022-10-01')
             WHERE datetime('2022-10-01') < datetime('2022-11-03')
             UNION ALL
             SELECT datetime(begin_date,'+1 day')
             FROM schedule
             WHERE datetime(begin_date) < datetime(?2)
         )
         SELECT
             begin_date
         FROM schedule
         GROUP BY begin_date
        ";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}");

    let query = "
        WITH RECURSIVE schedule(begin_date) AS MATERIALIZED (
             SELECT datetime('2022-10-01')
             WHERE datetime('2022-10-01') < datetime('2022-11-03')
             UNION ALL
             SELECT datetime(begin_date,'+1 day')
             FROM schedule
             WHERE datetime(begin_date) < datetime(?2)
         )
         SELECT
             begin_date
         FROM schedule
         GROUP BY begin_date
        ";
    let info = conn.describe(query).await?;
    assert_eq!(info.column(0).type_info().name(), "TEXT", "{query}");
    assert_eq!(info.nullable(0), Some(true), "{query}");

    Ok(())
}

#[sqlx_macros::test]
async fn it_describes_analytical_function() -> anyhow::Result<()> {
    let mut conn = new::<Sqlite>().await?;

    let d = conn
        .describe("select row_number() over () from accounts")
        .await?;
    dbg!(&d);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn.describe("select rank() over () from accounts").await?;
    dbg!(&d);
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("select dense_rank() over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("select percent_rank() over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "REAL");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("select cume_dist() over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "REAL");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("select ntile(1) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("select lag(id) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe("select lag(name) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "TEXT");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe("select lead(id) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe("select lead(name) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "TEXT");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe("select first_value(id) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe("select first_value(name) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "TEXT");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe("select last_value(id) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(false));

    let d = conn
        .describe("select first_value(name) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "TEXT");
    //assert_eq!(d.nullable(0), Some(false)); //this should be null, but it's hard to prove that it will be

    let d = conn
        .describe("select nth_value(id,10) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "INTEGER");
    assert_eq!(d.nullable(0), Some(true));

    let d = conn
        .describe("select nth_value(name,10) over () from accounts")
        .await?;
    assert_eq!(d.column(0).type_info().name(), "TEXT");
    assert_eq!(d.nullable(0), Some(true));

    Ok(())
}
