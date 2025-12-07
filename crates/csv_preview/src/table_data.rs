use ui::SharedString;

/// Generic container struct of table-like data (CSV, TSV, etc)
#[derive(Default)]
pub struct TableData {
    pub headers: Vec<SharedString>,
    pub rows: Vec<Vec<SharedString>>,
}

impl TableData {
    pub fn from_str(raw_text: String) -> Self {
        let mut lines = raw_text.lines().collect::<Vec<_>>();
        if lines.is_empty() {
            return Self {
                headers: vec![],
                rows: vec![],
            };
        }

        let headers = if !lines.is_empty() {
            let first_line = lines.remove(0);
            Self::parse_csv_line(first_line)
        } else {
            vec![]
        };

        let rows: Vec<Vec<SharedString>> = lines
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .map(Self::parse_csv_line)
            .collect();

        Self { headers, rows }
    }

    fn parse_csv_line(line: &str) -> Vec<SharedString> {
        let mut result = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if in_quotes {
                        if chars.peek() == Some(&'"') {
                            chars.next();
                            current_field.push('"');
                        } else {
                            in_quotes = false;
                        }
                    } else {
                        in_quotes = true;
                    }
                }
                ',' if !in_quotes => {
                    result.push(current_field.trim().to_string().into());
                    current_field.clear();
                }
                _ => {
                    current_field.push(ch);
                }
            }
        }

        result.push(current_field.trim().to_string().into());
        result
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
    fn test_empty_csv() {
        let parsed = TableData::from_str("".to_string());
        assert!(parsed.headers.is_empty());
        assert!(parsed.rows.is_empty());
    }
}
