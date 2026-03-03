use std::fmt;
use std::fmt::Write as _;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<CellValue>>,
    pub total_row_count: Option<u64>,
    pub affected_rows: Option<u64>,
    pub execution_time: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    Date(String),
    Time(String),
    Timestamp(String),
    Json(String),
    Uuid(String),
    Blob(Vec<u8>),
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellValue::Null => write!(f, "NULL"),
            CellValue::Boolean(b) => write!(f, "{}", b),
            CellValue::Integer(i) => write!(f, "{}", i),
            CellValue::Float(v) => write!(f, "{}", v),
            CellValue::Text(s) => write!(f, "{}", s),
            CellValue::Date(s) => write!(f, "{}", s),
            CellValue::Time(s) => write!(f, "{}", s),
            CellValue::Timestamp(s) => write!(f, "{}", s),
            CellValue::Json(s) => write!(f, "{}", s),
            CellValue::Uuid(s) => write!(f, "{}", s),
            CellValue::Blob(bytes) => write!(f, "<{} bytes>", bytes.len()),
        }
    }
}

impl CellValue {
    pub fn is_null(&self) -> bool {
        matches!(self, CellValue::Null)
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, CellValue::Boolean(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            CellValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, CellValue::Integer(_) | CellValue::Float(_))
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            CellValue::Integer(n) => Some(*n as f64),
            CellValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn to_sql_value(&self) -> String {
        match self {
            CellValue::Null => "NULL".to_string(),
            CellValue::Boolean(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
            CellValue::Integer(n) => n.to_string(),
            CellValue::Float(f) => f.to_string(),
            CellValue::Text(s) => format!("'{}'", s.replace('\'', "''")),
            CellValue::Date(s) => format!("'{}'", s.replace('\'', "''")),
            CellValue::Time(s) => format!("'{}'", s.replace('\'', "''")),
            CellValue::Timestamp(s) => format!("'{}'", s.replace('\'', "''")),
            CellValue::Json(s) => format!("'{}'", s.replace('\'', "''")),
            CellValue::Uuid(s) => format!("'{}'", s.replace('\'', "''")),
            CellValue::Blob(bytes) => format!("X'{}'", hex_encode(bytes)),
        }
    }

    pub fn to_tsv_value(&self) -> String {
        match self {
            CellValue::Null => String::new(),
            CellValue::Text(s)
            | CellValue::Date(s)
            | CellValue::Time(s)
            | CellValue::Timestamp(s)
            | CellValue::Json(s)
            | CellValue::Uuid(s) => {
                if s.contains('\t') || s.contains('\n') || s.contains('\r') {
                    s.replace('\t', "    ")
                        .replace('\n', "\\n")
                        .replace('\r', "\\r")
                } else {
                    s.clone()
                }
            }
            other => other.to_string(),
        }
    }

    pub fn to_csv_value(&self) -> String {
        match self {
            CellValue::Null => String::new(),
            CellValue::Text(s)
            | CellValue::Date(s)
            | CellValue::Time(s)
            | CellValue::Timestamp(s)
            | CellValue::Json(s)
            | CellValue::Uuid(s) => {
                if s.contains(',') || s.contains('"') || s.contains('\n') {
                    format!("\"{}\"", s.replace('"', "\"\""))
                } else {
                    s.clone()
                }
            }
            other => other.to_string(),
        }
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(hex, "{:02x}", byte).expect("writing to String cannot fail");
    }
    hex
}
