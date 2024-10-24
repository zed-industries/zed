pub use duckdb::*;

pub fn write_to_table<T>(
    table_name: &str,
    rows: &[T],
    connection: &duckdb::Connection,
) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    let mut stmt = connection.prepare(&format!(
        "INSERT INTO {} SELECT * FROM json_each(?)",
        table_name
    ))?;
    let json = serde_json::to_string(rows)?;
    stmt.execute([json])?;
    Ok(())
}
