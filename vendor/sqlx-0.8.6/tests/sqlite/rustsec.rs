use sqlx::{Connection, Error, SqliteConnection};

// https://rustsec.org/advisories/RUSTSEC-2024-0363.html
//
// Similar theory to the Postgres exploit in `tests/postgres/rustsec.rs` but much simpler
// since we just want to overflow the query length itself.
#[sqlx::test]
async fn rustsec_2024_0363() -> anyhow::Result<()> {
    let overflow_len = 4 * 1024 * 1024 * 1024; // 4 GiB

    // `real_query_prefix` plus `fake_message` will be the first query that SQLite "sees"
    //
    // Rather contrived because this already represents a regular SQL injection,
    // but this is the easiest way to demonstrate the exploit for SQLite.
    let real_query_prefix = "INSERT INTO injection_target(message) VALUES ('";
    let fake_message = "fake_msg') RETURNING id;";
    let real_query_suffix = "') RETURNING id";

    // Our actual payload is another query
    let real_payload =
        "\nUPDATE injection_target SET message = 'you''ve been pwned!' WHERE id = 1;\n--";

    // This will parse the query up to `real_payload`.
    let fake_payload_len = real_query_prefix.len() + fake_message.len();

    // Pretty easy to see that this will overflow to `fake_payload_len`
    let target_len = overflow_len + fake_payload_len;

    let inject_len = target_len - real_query_prefix.len() - real_query_suffix.len();

    let pad_len = inject_len - fake_message.len() - real_payload.len();

    let mut injected_value = String::with_capacity(inject_len);
    injected_value.push_str(fake_message);
    injected_value.push_str(real_payload);

    let padding = " ".repeat(pad_len);
    injected_value.push_str(&padding);

    let query = format!("{real_query_prefix}{injected_value}{real_query_suffix}");

    assert_eq!(query.len(), target_len);

    let mut conn = SqliteConnection::connect("sqlite://:memory:").await?;

    sqlx::raw_sql(
        "CREATE TABLE injection_target(id INTEGER PRIMARY KEY, message TEXT);\n\
            INSERT INTO injection_target(message) VALUES ('existing message');",
    )
    .execute(&mut conn)
    .await?;

    let res = sqlx::raw_sql(&query).execute(&mut conn).await;

    if let Err(e) = res {
        // Connection rejected the query; we're happy.
        if matches!(e, Error::Protocol(_)) {
            return Ok(());
        }

        panic!("unexpected error: {e:?}");
    }

    let messages: Vec<String> =
        sqlx::query_scalar("SELECT message FROM injection_target ORDER BY id")
            .fetch_all(&mut conn)
            .await?;

    // If the injection succeeds, `messages` will look like:
    // ["you've been pwned!'.to_string(), "fake_msg".to_string()]
    assert_eq!(
        messages,
        ["existing message".to_string(), "fake_msg".to_string()]
    );

    // Injection didn't affect our database; we're happy.
    Ok(())
}
