use anyhow::{Result, anyhow};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Dict(Vec<(Vec<u8>, Value)>),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Bytes(bytes) => std::str::from_utf8(bytes).ok(),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Dict(entries) => {
                let target_key = key.as_bytes();
                entries
                    .iter()
                    .find(|(entry_key, _)| entry_key.as_slice() == target_key)
                    .map(|(_, value)| value)
            }
            _ => None,
        }
    }
}

pub fn encode(value: &Value) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    encode_into(value, &mut out);
    out
}

fn encode_into(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Int(number) => {
            out.push(b'i');
            out.extend_from_slice(number.to_string().as_bytes());
            out.push(b'e');
        }
        Value::Bytes(bytes) => {
            out.extend_from_slice(bytes.len().to_string().as_bytes());
            out.push(b':');
            out.extend_from_slice(bytes);
        }
        Value::List(items) => {
            out.push(b'l');
            for item in items {
                encode_into(item, out);
            }
            out.push(b'e');
        }
        Value::Dict(entries) => {
            out.push(b'd');
            for (key, val) in entries {
                out.extend_from_slice(key.len().to_string().as_bytes());
                out.push(b':');
                out.extend_from_slice(key);
                encode_into(val, out);
            }
            out.push(b'e');
        }
    }
}

pub fn decode_available(buffer: &[u8]) -> Result<Option<(Value, usize)>> {
    let mut parser = Parser {
        input: buffer,
        position: 0,
    };
    match parser.parse_value() {
        Ok(value) => Ok(Some((value, parser.position))),
        Err(ParseError::Incomplete) => Ok(None),
        Err(ParseError::Malformed(err)) => Err(err),
    }
}

#[derive(Debug)]
enum ParseError {
    Incomplete,
    Malformed(anyhow::Error),
}

impl From<anyhow::Error> for ParseError {
    fn from(error: anyhow::Error) -> Self {
        ParseError::Malformed(error)
    }
}

struct Parser<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Result<u8, ParseError> {
        self.input
            .get(self.position)
            .copied()
            .ok_or(ParseError::Incomplete)
    }

    fn bump(&mut self) -> Result<u8, ParseError> {
        let byte = self.peek()?;
        self.position += 1;
        Ok(byte)
    }

    fn expect(&mut self, expected: u8) -> Result<(), ParseError> {
        let actual = self.bump()?;
        if actual != expected {
            return Err(ParseError::Malformed(anyhow!(
                "expected {:?} at position {}, found {:?}",
                expected as char,
                self.position - 1,
                actual as char
            )));
        }
        Ok(())
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        match self.peek()? {
            b'i' => self.parse_int(),
            b'l' => self.parse_list(),
            b'd' => self.parse_dict(),
            b'0'..=b'9' => self.parse_bytes().map(Value::Bytes),
            other => Err(ParseError::Malformed(anyhow!(
                "unexpected byte {:?} at position {} (expected i/l/d/digit)",
                other as char,
                self.position
            ))),
        }
    }

    fn parse_int(&mut self) -> Result<Value, ParseError> {
        self.expect(b'i')?;
        let start = self.position;
        let mut end = None;
        for (index, byte) in self.input[start..].iter().enumerate() {
            if *byte == b'e' {
                end = Some(start + index);
                break;
            }
        }
        let end = end.ok_or(ParseError::Incomplete)?;
        let digits = &self.input[start..end];
        if digits.is_empty() {
            return Err(ParseError::Malformed(anyhow!("empty integer at {}", start)));
        }
        let string = std::str::from_utf8(digits)
            .map_err(|_| ParseError::Malformed(anyhow!("non-ASCII integer at {}", start)))?;
        let bytes = string.as_bytes();
        let invalid = match bytes {
            [b'-'] => true,
            [b'-', b'0'] => true,
            [b'0', _, ..] => true,
            [b'-', b'0', _, ..] => true,
            _ => false,
        };
        if invalid {
            return Err(ParseError::Malformed(anyhow!(
                "non-canonical integer {:?} at {}",
                string,
                start
            )));
        }
        let number: i64 = string
            .parse()
            .map_err(|error| ParseError::Malformed(anyhow!("integer parse: {error}")))?;
        self.position = end + 1;
        Ok(Value::Int(number))
    }

    fn parse_bytes(&mut self) -> Result<Vec<u8>, ParseError> {
        let start = self.position;
        let mut colon = None;
        for (index, byte) in self.input[start..].iter().enumerate() {
            if *byte == b':' {
                colon = Some(start + index);
                break;
            }
            if !byte.is_ascii_digit() {
                return Err(ParseError::Malformed(anyhow!(
                    "non-digit {:?} in byte-string length at {}",
                    *byte as char,
                    start + index
                )));
            }
        }
        let colon = colon.ok_or(ParseError::Incomplete)?;
        let len_digits = &self.input[start..colon];
        if len_digits.is_empty() {
            return Err(ParseError::Malformed(anyhow!(
                "missing byte-string length at {start}"
            )));
        }
        if len_digits.len() > 1 && len_digits[0] == b'0' {
            return Err(ParseError::Malformed(anyhow!(
                "non-canonical byte-string length at {start}"
            )));
        }
        let length: usize = std::str::from_utf8(len_digits)
            .ok()
            .and_then(|str_slice| str_slice.parse().ok())
            .ok_or_else(|| {
                ParseError::Malformed(anyhow!("invalid byte-string length at {start}"))
            })?;
        let body_start = colon + 1;
        let body_end = body_start
            .checked_add(length)
            .ok_or_else(|| ParseError::Malformed(anyhow!("byte-string length overflow")))?;
        if body_end > self.input.len() {
            return Err(ParseError::Incomplete);
        }
        let bytes = self.input[body_start..body_end].to_vec();
        self.position = body_end;
        Ok(bytes)
    }

    fn parse_list(&mut self) -> Result<Value, ParseError> {
        self.expect(b'l')?;
        let mut items = Vec::new();
        loop {
            if self.peek()? == b'e' {
                self.position += 1;
                return Ok(Value::List(items));
            }
            items.push(self.parse_value()?);
        }
    }

    fn parse_dict(&mut self) -> Result<Value, ParseError> {
        self.expect(b'd')?;
        let mut entries = Vec::new();
        loop {
            if self.peek()? == b'e' {
                self.position += 1;
                return Ok(Value::Dict(entries));
            }
            let key = self.parse_bytes()?;
            let value = self.parse_value()?;
            entries.push((key, value));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_nested_dict() {
        let nested_dict = Value::Dict(vec![
            (
                b"inner".to_vec(),
                Value::Dict(vec![
                    (b"answer".to_vec(), Value::Int(42)),
                    (b"status".to_vec(), Value::Bytes(b"done".to_vec())),
                ]),
            ),
            (
                b"list".to_vec(),
                Value::List(vec![
                    Value::Int(1),
                    Value::Bytes(b"two".to_vec()),
                    Value::Int(3),
                ]),
            ),
            (b"op".to_vec(), Value::Bytes(b"eval".to_vec())),
        ]);

        let encoded = encode(&nested_dict);
        let decoded = decode_available(&encoded).unwrap();
        assert_eq!(decoded, Some((nested_dict, encoded.len())));

        let (val, _) = decoded.unwrap();
        assert_eq!(val.get("op").and_then(|v| v.as_str()), Some("eval"));
        let inner = val.get("inner").unwrap();
        assert_eq!(inner.get("status").and_then(|v| v.as_str()), Some("done"));
    }

    #[test]
    fn test_partial_buffer_returns_none() {
        let original = Value::Dict(vec![
            (b"id".to_vec(), Value::Bytes(b"123".to_vec())),
            (b"value".to_vec(), Value::Int(100)),
        ]);
        let encoded = encode(&original);

        for len in 0..encoded.len() {
            let partial = &encoded[..len];
            assert_eq!(decode_available(partial).unwrap(), None);
        }
    }

    #[test]
    fn test_two_concatenated_messages() {
        let msg1 = Value::Dict(vec![
            (b"id".to_vec(), Value::Bytes(b"msg-1".to_vec())),
            (b"op".to_vec(), Value::Bytes(b"eval".to_vec())),
        ]);
        let msg2 = Value::Dict(vec![
            (b"id".to_vec(), Value::Bytes(b"msg-2".to_vec())),
            (b"status".to_vec(), Value::Bytes(b"done".to_vec())),
        ]);

        let mut concatenated = encode(&msg1);
        let msg1_len = concatenated.len();
        let encoded_msg2 = encode(&msg2);
        let msg2_len = encoded_msg2.len();
        concatenated.extend_from_slice(&encoded_msg2);

        let decoded1 = decode_available(&concatenated).unwrap();
        assert_eq!(decoded1, Some((msg1, msg1_len)));

        let remaining = &concatenated[msg1_len..];
        let decoded2 = decode_available(remaining).unwrap();
        assert_eq!(decoded2, Some((msg2, msg2_len)));
    }
}
