use serde::Serialize;

/// Writes the given rows to the specified Clickhouse table.
pub async fn write_to_table<T: clickhouse::Row + Serialize + std::fmt::Debug>(
    table: &str,
    rows: &[T],
    clickhouse_client: &clickhouse::Client,
) -> anyhow::Result<()> {
    if rows.is_empty() {
        return Ok(());
    }

    let mut insert = clickhouse_client.insert(table)?;

    for event in rows {
        insert.write(event).await?;
    }

    insert.end().await?;

    let event_count = rows.len();
    log::info!(
        "wrote {event_count} {event_specifier} to '{table}'",
        event_specifier = if event_count == 1 { "event" } else { "events" }
    );

    Ok(())
}
