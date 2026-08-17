#[derive(Debug)]
pub enum ProtobufFloatParseError {
    EmptyString,
    CannotParseFloat,
}

pub type ProtobufFloatParseResult<T> = Result<T, ProtobufFloatParseError>;

pub const PROTOBUF_NAN: &str = "nan";
pub const PROTOBUF_INF: &str = "inf";

/// Format float as in protobuf `.proto` files
pub fn format_protobuf_float(f: f64) -> String {
    if f.is_nan() {
        PROTOBUF_NAN.to_owned()
    } else if f.is_infinite() {
        if f > 0.0 {
            format!("{}", PROTOBUF_INF)
        } else {
            format!("-{}", PROTOBUF_INF)
        }
    } else {
        // TODO: make sure doesn't lose precision
        format!("{}", f)
    }
}

/// Parse float from `.proto` format
pub fn parse_protobuf_float(s: &str) -> ProtobufFloatParseResult<f64> {
    if s.is_empty() {
        return Err(ProtobufFloatParseError::EmptyString);
    }
    if s == PROTOBUF_NAN {
        return Ok(f64::NAN);
    }
    if s == PROTOBUF_INF || s == format!("+{}", PROTOBUF_INF) {
        return Ok(f64::INFINITY);
    }
    if s == format!("-{}", PROTOBUF_INF) {
        return Ok(f64::NEG_INFINITY);
    }
    match s.parse() {
        Ok(f) => Ok(f),
        Err(_) => Err(ProtobufFloatParseError::CannotParseFloat),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_protobuf_float() {
        assert_eq!("10", format_protobuf_float(10.0));
    }
}
