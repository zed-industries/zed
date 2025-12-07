use ui::SharedString;

/// Generic container struct of table-like data (CSV, TSV, etc)
#[derive(Default)]
pub struct TableData {
    pub headers: Vec<SharedString>,
    pub rows: Vec<Vec<SharedString>>,
}

impl TableData {
    pub fn from_str(raw_text: String) -> Self {
        if raw_text.trim().is_empty() {
            return Self {
                headers: vec![],
                rows: vec![],
            };
        }

        let parsed_rows = Self::parse_csv(&raw_text);
        if parsed_rows.is_empty() {
            return Self {
                headers: vec![],
                rows: vec![],
            };
        }

        let headers = parsed_rows[0].clone();
        let rows = parsed_rows.into_iter().skip(1).collect();

        Self { headers, rows }
    }

    fn parse_csv(text: &str) -> Vec<Vec<SharedString>> {
        let mut rows = Vec::new();
        let mut current_row: Vec<SharedString> = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if in_quotes {
                        if chars.peek() == Some(&'"') {
                            // Escaped quote
                            chars.next();
                            current_field.push('"');
                        } else {
                            // End of quoted field
                            in_quotes = false;
                        }
                    } else {
                        // Start of quoted field
                        in_quotes = true;
                    }
                }
                ',' if !in_quotes => {
                    // Field separator
                    current_row.push(current_field.trim().to_string().into());
                    current_field.clear();
                }
                '\n' | '\r' if !in_quotes => {
                    // Row separator (only when not inside quotes)
                    current_row.push(current_field.trim().to_string().into());
                    current_field.clear();

                    // Only add non-empty rows
                    if !current_row.is_empty()
                        && !current_row.iter().all(|field| field.trim().is_empty())
                    {
                        rows.push(current_row);
                    }
                    current_row = Vec::new();
                }
                '\r' if chars.peek() == Some(&'\n') => {
                    // Handle Windows line endings (\r\n) - skip the \r, let \n be handled above
                    continue;
                }
                _ => {
                    current_field.push(ch);
                }
            }
        }

        // Add the last field and row if not empty
        if !current_field.is_empty() || !current_row.is_empty() {
            current_row.push(current_field.trim().to_string().into());
        }
        if !current_row.is_empty() && !current_row.iter().all(|field| field.trim().is_empty()) {
            rows.push(current_row);
        }

        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_parsing_basic() {
        let csv_data = "Name,Age,City\nJohn,30,New York\nJane,25,Los Angeles";
        let parsed = TableData::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.len(), 3);
        assert_eq!(parsed.headers[0].as_ref(), "Name");
        assert_eq!(parsed.headers[1].as_ref(), "Age");
        assert_eq!(parsed.headers[2].as_ref(), "City");

        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(parsed.rows[0][0].as_ref(), "John");
        assert_eq!(parsed.rows[0][1].as_ref(), "30");
        assert_eq!(parsed.rows[0][2].as_ref(), "New York");
    }

    #[test]
    fn test_csv_parsing_with_quotes() {
        let csv_data = r#"Name,Description
"John Doe","A person with ""special"" characters"
Jane,"Simple name""#;
        let parsed = TableData::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.len(), 2);
        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(
            parsed.rows[0][1].as_ref(),
            r#"A person with "special" characters"#
        );
    }

    #[test]
    fn test_csv_parsing_with_newlines_in_quotes() {
        let csv_data = "Name,Description,Status\n\"John\nDoe\",\"A person with\nmultiple lines\",Active\n\"Jane Smith\",\"Simple\",\"Also\nActive\"";
        let parsed = TableData::from_str(csv_data.to_string());

        assert_eq!(parsed.headers.len(), 3);
        assert_eq!(parsed.headers[0].as_ref(), "Name");
        assert_eq!(parsed.headers[1].as_ref(), "Description");
        assert_eq!(parsed.headers[2].as_ref(), "Status");

        assert_eq!(parsed.rows.len(), 2);
        assert_eq!(parsed.rows[0][0].as_ref(), "John\nDoe");
        assert_eq!(parsed.rows[0][1].as_ref(), "A person with\nmultiple lines");
        assert_eq!(parsed.rows[0][2].as_ref(), "Active");

        assert_eq!(parsed.rows[1][0].as_ref(), "Jane Smith");
        assert_eq!(parsed.rows[1][1].as_ref(), "Simple");
        assert_eq!(parsed.rows[1][2].as_ref(), "Also\nActive");
    }

    #[test]
    fn test_empty_csv() {
        let parsed = TableData::from_str("".to_string());
        assert!(parsed.headers.is_empty());
        assert!(parsed.rows.is_empty());
    }
}
