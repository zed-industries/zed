use std::fmt::Write;

use crate::query_result::{CellValue, QueryResult};
use crate::schema::{DatabaseSchema, TableInfo};

pub fn generate_csv(result: &QueryResult) -> String {
    let mut output = result.columns.join(",");
    output.push('\n');
    for row in &result.rows {
        let line: Vec<String> = row.iter().map(|cell| cell.to_csv_value()).collect();
        output.push_str(&line.join(","));
        output.push('\n');
    }
    output
}

pub fn generate_json(result: &QueryResult) -> String {
    let mut objects = Vec::with_capacity(result.rows.len());
    for row in &result.rows {
        let mut entries = Vec::with_capacity(result.columns.len());
        for (i, cell) in row.iter().enumerate() {
            let key = result.columns.get(i).map(|s| s.as_str()).unwrap_or("");
            let value = match cell {
                CellValue::Null => "null".to_string(),
                CellValue::Boolean(b) => b.to_string(),
                CellValue::Integer(n) => n.to_string(),
                CellValue::Float(f) => f.to_string(),
                CellValue::Text(s)
                | CellValue::Date(s)
                | CellValue::Time(s)
                | CellValue::Timestamp(s)
                | CellValue::Uuid(s) => {
                    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
                }
                CellValue::Json(s) => {
                    // JSON is already valid JSON, embed directly
                    s.clone()
                }
                CellValue::Blob(bytes) => {
                    format!("\"<{} bytes>\"", bytes.len())
                }
            };
            entries.push(format!("\"{}\":{}", key.replace('"', "\\\""), value));
        }
        objects.push(format!("{{{}}}", entries.join(",")));
    }
    format!("[{}]", objects.join(","))
}

pub fn generate_sql_insert(result: &QueryResult, table_name: &str) -> String {
    if result.rows.is_empty() {
        return format!("-- No data to insert into {}\n", table_name);
    }

    let escaped_table = table_name.replace('"', "\"\"");
    let columns_list = result
        .columns
        .iter()
        .map(|c| format!("\"{}\"", c.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(", ");

    let mut output = String::new();
    for row in &result.rows {
        output.push_str(&format!(
            "INSERT INTO \"{}\" ({}) VALUES (",
            escaped_table, columns_list
        ));
        let values: Vec<String> = row.iter().map(|cell| cell.to_sql_value()).collect();
        output.push_str(&values.join(", "));
        output.push_str(");\n");
    }
    output
}

pub fn generate_tsv(result: &QueryResult) -> String {
    let mut output = result.columns.join("\t");
    output.push('\n');
    for row in &result.rows {
        let line: Vec<String> = row.iter().map(|cell| cell.to_tsv_value()).collect();
        output.push_str(&line.join("\t"));
        output.push('\n');
    }
    output
}

pub fn generate_html(result: &QueryResult) -> String {
    if result.columns.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str("<table style=\"border-collapse:collapse;border:1px solid #ccc;\">\n");

    output.push_str("<thead><tr>");
    for col in &result.columns {
        output.push_str(&format!(
            "<th style=\"border:1px solid #ccc;padding:4px 8px;background:#f5f5f5;\">{}</th>",
            html_escape(col)
        ));
    }
    output.push_str("</tr></thead>\n");

    output.push_str("<tbody>\n");
    for row in &result.rows {
        output.push_str("<tr>");
        for (i, _col) in result.columns.iter().enumerate() {
            let cell = row.get(i).map(|c| c.to_string()).unwrap_or_default();
            output.push_str(&format!(
                "<td style=\"border:1px solid #ccc;padding:4px 8px;\">{}</td>",
                html_escape(&cell)
            ));
        }
        output.push_str("</tr>\n");
    }
    output.push_str("</tbody>\n");
    output.push_str("</table>");

    output
}

pub fn generate_sql_ddl_dml(result: &QueryResult, table_name: &str) -> String {
    if result.columns.is_empty() {
        return String::new();
    }

    let escaped_table = table_name.replace('"', "\"\"");
    let mut output = String::new();

    // Infer column types from the first non-null value in each column
    let column_types: Vec<&str> = (0..result.columns.len())
        .map(|col_idx| {
            for row in &result.rows {
                if let Some(cell) = row.get(col_idx) {
                    match cell {
                        CellValue::Null => continue,
                        CellValue::Boolean(_) => return "BOOLEAN",
                        CellValue::Integer(_) => return "INTEGER",
                        CellValue::Float(_) => return "REAL",
                        CellValue::Text(_) => return "TEXT",
                        CellValue::Date(_) => return "DATE",
                        CellValue::Time(_) => return "TIME",
                        CellValue::Timestamp(_) => return "TIMESTAMP",
                        CellValue::Json(_) => return "JSON",
                        CellValue::Uuid(_) => return "UUID",
                        CellValue::Blob(_) => return "BLOB",
                    }
                }
            }
            "TEXT"
        })
        .collect();

    // CREATE TABLE
    output.push_str(&format!("CREATE TABLE \"{}\" (\n", escaped_table));
    for (i, col) in result.columns.iter().enumerate() {
        let escaped_col = col.replace('"', "\"\"");
        let col_type = column_types.get(i).copied().unwrap_or("TEXT");
        output.push_str(&format!("    \"{}\" {}", escaped_col, col_type));
        if i < result.columns.len() - 1 {
            output.push(',');
        }
        output.push('\n');
    }
    output.push_str(");\n\n");

    // INSERT statements
    output.push_str(&generate_sql_insert(result, table_name));

    output
}

fn html_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

pub fn generate_markdown(result: &QueryResult) -> String {
    if result.columns.is_empty() {
        return String::new();
    }

    let mut output = String::new();

    // Header row
    output.push('|');
    for col in &result.columns {
        output.push_str(&format!(" {} |", col));
    }
    output.push('\n');

    // Separator row
    output.push('|');
    for _ in &result.columns {
        output.push_str(" --- |");
    }
    output.push('\n');

    // Data rows
    for row in &result.rows {
        output.push('|');
        for (i, _col) in result.columns.iter().enumerate() {
            let cell = row.get(i).map(|c| c.to_string()).unwrap_or_default();
            // Escape pipe characters in cell values for markdown
            let escaped = cell.replace('|', "\\|");
            output.push_str(&format!(" {} |", escaped));
        }
        output.push('\n');
    }
    output
}

pub fn generate_ddl_from_schema(schema: &DatabaseSchema) -> String {
    let mut output = String::new();

    for table in &schema.tables {
        generate_table_ddl(table, &mut output);
        output.push('\n');
    }

    output
}

pub fn generate_table_ddl(table: &TableInfo, output: &mut String) {
    let keyword = match table.table_kind {
        crate::schema::TableKind::View => "CREATE VIEW",
        crate::schema::TableKind::MaterializedView => "CREATE MATERIALIZED VIEW",
        _ => "CREATE TABLE",
    };

    let escaped_name = table.name.replace('"', "\"\"");
    writeln!(output, "{keyword} \"{escaped_name}\" (").ok();

    let pk_columns: Vec<&str> = table
        .columns
        .iter()
        .filter(|c| c.primary_key)
        .map(|c| c.name.as_str())
        .collect();

    for (i, column) in table.columns.iter().enumerate() {
        let escaped_col = column.name.replace('"', "\"\"");
        write!(output, "    \"{escaped_col}\" {}", column.data_type).ok();

        if !column.nullable {
            write!(output, " NOT NULL").ok();
        }

        if column.primary_key && pk_columns.len() == 1 {
            write!(output, " PRIMARY KEY").ok();
        }

        if let Some(default) = &column.default_value {
            write!(output, " DEFAULT {default}").ok();
        }

        if i < table.columns.len() - 1 || pk_columns.len() > 1 || !table.foreign_keys.is_empty() {
            output.push(',');
        }
        output.push('\n');
    }

    if pk_columns.len() > 1 {
        let pk_cols = pk_columns
            .iter()
            .map(|c| format!("\"{}\"", c.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(", ");
        write!(output, "    PRIMARY KEY ({pk_cols})").ok();
        if !table.foreign_keys.is_empty() {
            output.push(',');
        }
        output.push('\n');
    }

    for (i, fk) in table.foreign_keys.iter().enumerate() {
        let from = fk.from_column.replace('"', "\"\"");
        let to_table = fk.to_table.replace('"', "\"\"");
        let to_col = fk.to_column.replace('"', "\"\"");
        write!(
            output,
            "    FOREIGN KEY (\"{from}\") REFERENCES \"{to_table}\"(\"{to_col}\")"
        )
        .ok();
        if i < table.foreign_keys.len() - 1 {
            output.push(',');
        }
        output.push('\n');
    }

    writeln!(output, ");").ok();

    for index in &table.indexes {
        let unique = if index.unique { "UNIQUE " } else { "" };
        let escaped_idx = index.name.replace('"', "\"\"");
        let idx_cols = index
            .columns
            .iter()
            .map(|c| format!("\"{}\"", c.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            output,
            "CREATE {unique}INDEX \"{escaped_idx}\" ON \"{escaped_name}\" ({idx_cols});"
        )
        .ok();
    }
}

pub fn generate_xlsx(result: &QueryResult) -> anyhow::Result<Vec<u8>> {
    use rust_xlsxwriter::Workbook;

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for (col_index, col_name) in result.columns.iter().enumerate() {
        worksheet.write_string(0, col_index as u16, col_name)?;
    }

    for (row_index, row) in result.rows.iter().enumerate() {
        let xlsx_row = (row_index + 1) as u32;
        for (col_index, cell) in row.iter().enumerate() {
            let xlsx_col = col_index as u16;
            match cell {
                CellValue::Null => {}
                CellValue::Boolean(b) => {
                    worksheet.write_boolean(xlsx_row, xlsx_col, *b)?;
                }
                CellValue::Integer(n) => {
                    worksheet.write_number(xlsx_row, xlsx_col, *n as f64)?;
                }
                CellValue::Float(f) => {
                    worksheet.write_number(xlsx_row, xlsx_col, *f)?;
                }
                CellValue::Blob(bytes) => {
                    worksheet.write_string(xlsx_row, xlsx_col, &format!("<{} bytes>", bytes.len()))?;
                }
                other => {
                    worksheet.write_string(xlsx_row, xlsx_col, &other.to_string())?;
                }
            }
        }
    }

    let buffer = workbook.save_to_buffer()?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ColumnInfo, ForeignKeyInfo, IndexInfo};
    use std::time::Duration;

    fn sample_result() -> QueryResult {
        QueryResult {
            columns: vec!["id".to_string(), "name".to_string(), "score".to_string()],
            rows: vec![
                vec![
                    CellValue::Integer(1),
                    CellValue::Text("Alice".to_string()),
                    CellValue::Float(95.5),
                ],
                vec![
                    CellValue::Integer(2),
                    CellValue::Text("Bob".to_string()),
                    CellValue::Null,
                ],
            ],
            total_row_count: Some(2),
            affected_rows: None,
            execution_time: Duration::from_millis(10),
        }
    }

    #[test]
    fn test_generate_csv() {
        let result = sample_result();
        let csv = generate_csv(&result);
        assert!(csv.starts_with("id,name,score\n"));
        assert!(csv.contains("1,Alice,95.5"));
        assert!(csv.contains("2,Bob,"));
    }

    #[test]
    fn test_generate_json() {
        let result = sample_result();
        let json = generate_json(&result);
        assert!(json.starts_with("[{"));
        assert!(json.contains("\"name\":\"Alice\""));
        assert!(json.contains("\"score\":null"));
    }

    #[test]
    fn test_generate_sql_insert() {
        let result = sample_result();
        let sql = generate_sql_insert(&result, "users");
        assert!(sql.contains("INSERT INTO \"users\""));
        assert!(sql.contains("(\"id\", \"name\", \"score\")"));
        assert!(sql.contains("VALUES (1, 'Alice', 95.5)"));
        assert!(sql.contains("VALUES (2, 'Bob', NULL)"));
    }

    #[test]
    fn test_generate_sql_insert_empty() {
        let result = QueryResult {
            columns: vec!["id".to_string()],
            rows: vec![],
            total_row_count: Some(0),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let sql = generate_sql_insert(&result, "empty_table");
        assert!(sql.contains("No data to insert"));
    }

    #[test]
    fn test_generate_sql_insert_special_chars() {
        let result = QueryResult {
            columns: vec!["name".to_string()],
            rows: vec![vec![CellValue::Text("O'Brien".to_string())]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let sql = generate_sql_insert(&result, "people");
        assert!(sql.contains("'O''Brien'"));
    }

    #[test]
    fn test_generate_markdown() {
        let result = sample_result();
        let md = generate_markdown(&result);
        let lines: Vec<&str> = md.lines().collect();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "| id | name | score |");
        assert_eq!(lines[1], "| --- | --- | --- |");
        assert!(lines[2].contains("Alice"));
        assert!(lines[3].contains("NULL"));
    }

    #[test]
    fn test_generate_markdown_empty() {
        let result = QueryResult {
            columns: vec![],
            rows: vec![],
            total_row_count: None,
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let md = generate_markdown(&result);
        assert!(md.is_empty());
    }

    #[test]
    fn test_generate_markdown_pipe_in_value() {
        let result = QueryResult {
            columns: vec!["data".to_string()],
            rows: vec![vec![CellValue::Text("a|b".to_string())]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let md = generate_markdown(&result);
        assert!(md.contains("a\\|b"));
    }

    #[test]
    fn test_generate_tsv() {
        let result = sample_result();
        let tsv = generate_tsv(&result);
        assert!(tsv.starts_with("id\tname\tscore\n"));
        assert!(tsv.contains("1\tAlice\t95.5"));
        assert!(tsv.contains("2\tBob\t"));
    }

    #[test]
    fn test_generate_tsv_with_tabs_in_value() {
        let result = QueryResult {
            columns: vec!["data".to_string()],
            rows: vec![vec![CellValue::Text("hello\tworld".to_string())]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let tsv = generate_tsv(&result);
        // Tabs in values should be replaced with spaces
        assert!(!tsv.lines().nth(1).unwrap_or("").contains('\t'));
        assert!(tsv.contains("hello    world"));
    }

    #[test]
    fn test_generate_tsv_with_newlines_in_value() {
        let result = QueryResult {
            columns: vec!["data".to_string()],
            rows: vec![vec![CellValue::Text("line1\nline2".to_string())]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let tsv = generate_tsv(&result);
        assert!(tsv.contains("line1\\nline2"));
    }

    #[test]
    fn test_generate_tsv_empty() {
        let result = QueryResult {
            columns: vec!["id".to_string()],
            rows: vec![],
            total_row_count: Some(0),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let tsv = generate_tsv(&result);
        assert_eq!(tsv, "id\n");
    }

    #[test]
    fn test_generate_tsv_null_values() {
        let result = QueryResult {
            columns: vec!["a".to_string(), "b".to_string()],
            rows: vec![vec![CellValue::Null, CellValue::Integer(42)]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let tsv = generate_tsv(&result);
        assert!(tsv.contains("\t42"));
    }

    #[test]
    fn test_generate_html() {
        let result = sample_result();
        let html = generate_html(&result);
        assert!(html.starts_with("<table"));
        assert!(html.contains("<thead>"));
        assert!(html.contains("<tbody>"));
        assert!(html.contains("<th"));
        assert!(html.contains(">id</th>"));
        assert!(html.contains(">name</th>"));
        assert!(html.contains(">Alice</td>"));
        assert!(html.contains(">NULL</td>"));
        assert!(html.ends_with("</table>"));
    }

    #[test]
    fn test_generate_html_empty_columns() {
        let result = QueryResult {
            columns: vec![],
            rows: vec![],
            total_row_count: None,
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let html = generate_html(&result);
        assert!(html.is_empty());
    }

    #[test]
    fn test_generate_html_escapes_special_chars() {
        let result = QueryResult {
            columns: vec!["data".to_string()],
            rows: vec![vec![CellValue::Text("<script>alert('xss')</script>".to_string())]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let html = generate_html(&result);
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn test_generate_html_escapes_ampersand() {
        let result = QueryResult {
            columns: vec!["name".to_string()],
            rows: vec![vec![CellValue::Text("A & B".to_string())]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let html = generate_html(&result);
        assert!(html.contains("A &amp; B"));
    }

    #[test]
    fn test_generate_sql_ddl_dml() {
        let result = sample_result();
        let sql = generate_sql_ddl_dml(&result, "users");
        assert!(sql.contains("CREATE TABLE \"users\""));
        assert!(sql.contains("\"id\" INTEGER"));
        assert!(sql.contains("\"name\" TEXT"));
        assert!(sql.contains("\"score\" REAL"));
        assert!(sql.contains("INSERT INTO \"users\""));
        assert!(sql.contains("VALUES (1, 'Alice', 95.5)"));
    }

    #[test]
    fn test_generate_sql_ddl_dml_empty_columns() {
        let result = QueryResult {
            columns: vec![],
            rows: vec![],
            total_row_count: None,
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let sql = generate_sql_ddl_dml(&result, "empty");
        assert!(sql.is_empty());
    }

    #[test]
    fn test_generate_sql_ddl_dml_type_inference() {
        let result = QueryResult {
            columns: vec![
                "int_col".to_string(),
                "float_col".to_string(),
                "text_col".to_string(),
                "blob_col".to_string(),
                "null_col".to_string(),
            ],
            rows: vec![vec![
                CellValue::Integer(1),
                CellValue::Float(2.71),
                CellValue::Text("hello".to_string()),
                CellValue::Blob(vec![0xDE, 0xAD]),
                CellValue::Null,
            ]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let sql = generate_sql_ddl_dml(&result, "typed");
        assert!(sql.contains("\"int_col\" INTEGER"));
        assert!(sql.contains("\"float_col\" REAL"));
        assert!(sql.contains("\"text_col\" TEXT"));
        assert!(sql.contains("\"blob_col\" BLOB"));
        // null_col should default to TEXT since all values are null
        assert!(sql.contains("\"null_col\" TEXT"));
    }

    #[test]
    fn test_generate_sql_ddl_dml_infers_from_non_null_row() {
        let result = QueryResult {
            columns: vec!["val".to_string()],
            rows: vec![
                vec![CellValue::Null],
                vec![CellValue::Integer(42)],
            ],
            total_row_count: Some(2),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let sql = generate_sql_ddl_dml(&result, "test");
        // Should infer INTEGER from the second row
        assert!(sql.contains("\"val\" INTEGER"));
    }

    #[test]
    fn test_generate_sql_ddl_dml_special_table_name() {
        let result = QueryResult {
            columns: vec!["id".to_string()],
            rows: vec![vec![CellValue::Integer(1)]],
            total_row_count: Some(1),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let sql = generate_sql_ddl_dml(&result, "my\"table");
        assert!(sql.contains("CREATE TABLE \"my\"\"table\""));
    }

    fn sample_table() -> TableInfo {
        TableInfo {
            name: "users".to_string(),
            columns: vec![
                ColumnInfo {
                    name: "id".to_string(),
                    data_type: "INTEGER".to_string(),
                    nullable: false,
                    primary_key: true,
                    default_value: None,
                },
                ColumnInfo {
                    name: "name".to_string(),
                    data_type: "TEXT".to_string(),
                    nullable: false,
                    primary_key: false,
                    default_value: None,
                },
                ColumnInfo {
                    name: "email".to_string(),
                    data_type: "TEXT".to_string(),
                    nullable: true,
                    primary_key: false,
                    default_value: Some("NULL".to_string()),
                },
            ],
            indexes: vec![IndexInfo {
                name: "idx_users_email".to_string(),
                columns: vec!["email".to_string()],
                unique: true,
            }],
            foreign_keys: vec![],
            row_count: Some(100),
            table_kind: crate::schema::TableKind::Table,
            ddl: None,
        }
    }

    #[test]
    fn test_generate_table_ddl_basic() {
        let table = sample_table();
        let mut output = String::new();
        generate_table_ddl(&table, &mut output);
        assert!(output.contains("CREATE TABLE \"users\""));
        assert!(output.contains("\"id\" INTEGER NOT NULL PRIMARY KEY"));
        assert!(output.contains("\"name\" TEXT NOT NULL"));
        assert!(output.contains("\"email\" TEXT DEFAULT NULL"));
        assert!(output.contains("CREATE UNIQUE INDEX \"idx_users_email\""));
    }

    #[test]
    fn test_generate_table_ddl_with_foreign_keys() {
        let table = TableInfo {
            name: "orders".to_string(),
            columns: vec![
                ColumnInfo {
                    name: "id".to_string(),
                    data_type: "INTEGER".to_string(),
                    nullable: false,
                    primary_key: true,
                    default_value: None,
                },
                ColumnInfo {
                    name: "user_id".to_string(),
                    data_type: "INTEGER".to_string(),
                    nullable: false,
                    primary_key: false,
                    default_value: None,
                },
            ],
            indexes: vec![],
            foreign_keys: vec![ForeignKeyInfo {
                from_column: "user_id".to_string(),
                to_table: "users".to_string(),
                to_column: "id".to_string(),
            }],
            row_count: None,
            table_kind: crate::schema::TableKind::Table,
            ddl: None,
        };
        let mut output = String::new();
        generate_table_ddl(&table, &mut output);
        assert!(output.contains("FOREIGN KEY (\"user_id\") REFERENCES \"users\"(\"id\")"));
    }

    #[test]
    fn test_generate_ddl_from_schema() {
        let schema = DatabaseSchema {
            tables: vec![sample_table()],
        };
        let ddl = generate_ddl_from_schema(&schema);
        assert!(ddl.contains("CREATE TABLE \"users\""));
        assert!(ddl.contains("CREATE UNIQUE INDEX"));
    }

    #[test]
    fn test_generate_xlsx_produces_bytes() {
        let result = sample_result();
        let bytes = generate_xlsx(&result).expect("generate_xlsx should succeed");
        assert!(!bytes.is_empty());
        // XLSX files start with PK (ZIP magic bytes)
        assert_eq!(bytes[0], 0x50); // P
        assert_eq!(bytes[1], 0x4B); // K
    }

    #[test]
    fn test_generate_xlsx_empty() {
        let result = QueryResult {
            columns: vec!["id".to_string()],
            rows: vec![],
            total_row_count: Some(0),
            affected_rows: None,
            execution_time: Duration::from_millis(1),
        };
        let bytes = generate_xlsx(&result).expect("generate_xlsx should succeed");
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_generate_table_ddl_view() {
        let table = TableInfo {
            name: "active_users".to_string(),
            columns: vec![ColumnInfo {
                name: "name".to_string(),
                data_type: "TEXT".to_string(),
                nullable: true,
                primary_key: false,
                default_value: None,
            }],
            indexes: vec![],
            foreign_keys: vec![],
            row_count: None,
            table_kind: crate::schema::TableKind::View,
            ddl: None,
        };
        let mut output = String::new();
        generate_table_ddl(&table, &mut output);
        assert!(output.contains("CREATE VIEW \"active_users\""));
    }
}
