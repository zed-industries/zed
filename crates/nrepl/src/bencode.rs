//! Bencode codec for the nREPL transport.
//!
//! nREPL uses a small subset of bencode:
//!
//! - Integers: `i<decimal>e` (signed, may be negative; `i0e` is allowed,
//!   leading zeros are not).
//! - Byte strings: `<len>:<bytes>` (any bytes, not necessarily UTF-8).
//! - Lists: `l<elements>e`.
//! - Dictionaries: `d(<bytestring-key><value>)*e`.
//!
//! The reference spec requires dict keys to be sorted lexicographically and
//! integers to be canonical. **Real-world nREPL servers do not always honor
//! that.** This decoder is deliberately tolerant on input — it accepts
//! unsorted dict keys and only rejects truly malformed input. The encoder
//! emits sorted dict keys so our outgoing frames are always canonical.
//!
//! See:
//! - <https://nrepl.org/nrepl/0.6/design/transports.html>
//! - <https://wiki.theory.org/BitTorrentSpecification#Bencoding>

use std::collections::BTreeMap;

use anyhow::{Result, anyhow};

/// A decoded bencode value.
///
/// We use `BTreeMap<Vec<u8>, Value>` for dicts so ordering is deterministic
/// (matching the canonical-form requirement on the wire) and so keys can be
/// non-UTF-8 bytes — the spec defines keys as byte strings, even though
/// every key nREPL actually uses is ASCII.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Dict(BTreeMap<Vec<u8>, Value>),
}

impl Value {
    /// Convenience constructor for an ASCII byte-string value.
    pub fn str(s: impl Into<String>) -> Self {
        Value::Bytes(s.into().into_bytes())
    }

    /// Returns the value as a UTF-8 string slice, if it is a byte string and
    /// the bytes are valid UTF-8.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Bytes(b) => std::str::from_utf8(b).ok(),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(b) => Some(b.as_slice()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(items) => Some(items.as_slice()),
            _ => None,
        }
    }

    pub fn as_dict(&self) -> Option<&BTreeMap<Vec<u8>, Value>> {
        match self {
            Value::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Looks up a key in a dict value. Returns `None` for non-dicts and
    /// missing keys alike — callers that need to distinguish should match
    /// on the value directly.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_dict().and_then(|d| d.get(key.as_bytes()))
    }
}

/// Helper for building dict values without sprinkling `into_bytes()` calls
/// at every call site.
pub fn dict<I, K>(entries: I) -> Value
where
    I: IntoIterator<Item = (K, Value)>,
    K: Into<String>,
{
    let mut map = BTreeMap::new();
    for (k, v) in entries {
        map.insert(k.into().into_bytes(), v);
    }
    Value::Dict(map)
}

/// Encodes a value into a freshly-allocated byte vector.
pub fn encode(value: &Value) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    encode_into(value, &mut out);
    out
}

/// Encodes a value, appending to an existing buffer.
pub fn encode_into(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Int(n) => {
            out.push(b'i');
            // `itoa` would shave a few cycles here, but it's not in scope
            // and the volume of bencode we encode is small.
            out.extend_from_slice(n.to_string().as_bytes());
            out.push(b'e');
        }
        Value::Bytes(b) => {
            out.extend_from_slice(b.len().to_string().as_bytes());
            out.push(b':');
            out.extend_from_slice(b);
        }
        Value::List(items) => {
            out.push(b'l');
            for item in items {
                encode_into(item, out);
            }
            out.push(b'e');
        }
        Value::Dict(map) => {
            out.push(b'd');
            // BTreeMap iterates in sorted order, which is exactly what the
            // canonical encoding requires.
            for (k, v) in map {
                out.extend_from_slice(k.len().to_string().as_bytes());
                out.push(b':');
                out.extend_from_slice(k);
                encode_into(v, out);
            }
            out.push(b'e');
        }
    }
}

/// Outcome of `decode_one`. The codec is fed bytes incrementally from a
/// socket, so "not enough data yet" is a normal, recoverable state — not an
/// error.
#[derive(Debug)]
pub enum DecodeOutcome {
    /// A complete value was parsed. `consumed` bytes should be removed from
    /// the front of the input buffer.
    Value { value: Value, consumed: usize },
    /// The input is a valid prefix of a bencode value but not yet complete.
    /// The caller should read more bytes and try again.
    Incomplete,
}

/// Attempts to decode a single value from the front of `input`.
///
/// Returns:
/// - `Ok(DecodeOutcome::Value { .. })` on success.
/// - `Ok(DecodeOutcome::Incomplete)` if more bytes are needed.
/// - `Err(_)` only on truly malformed input (the connection should be
///   considered poisoned in that case).
pub fn decode_one(input: &[u8]) -> Result<DecodeOutcome> {
    let mut p = Parser { input, pos: 0 };
    match p.parse_value() {
        Ok(value) => Ok(DecodeOutcome::Value {
            value,
            consumed: p.pos,
        }),
        Err(ParseError::Incomplete) => Ok(DecodeOutcome::Incomplete),
        Err(ParseError::Malformed(e)) => Err(e),
    }
}

#[derive(Debug)]
enum ParseError {
    /// More bytes are needed; the input so far is a valid prefix.
    Incomplete,
    /// The input is definitively invalid.
    Malformed(anyhow::Error),
}

impl From<anyhow::Error> for ParseError {
    fn from(e: anyhow::Error) -> Self {
        ParseError::Malformed(e)
    }
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Result<u8, ParseError> {
        self.input
            .get(self.pos)
            .copied()
            .ok_or(ParseError::Incomplete)
    }

    fn bump(&mut self) -> Result<u8, ParseError> {
        let b = self.peek()?;
        self.pos += 1;
        Ok(b)
    }

    fn expect(&mut self, expected: u8) -> Result<(), ParseError> {
        let actual = self.bump()?;
        if actual != expected {
            return Err(ParseError::Malformed(anyhow!(
                "expected {:?} at position {}, found {:?}",
                expected as char,
                self.pos - 1,
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
                self.pos
            ))),
        }
    }

    fn parse_int(&mut self) -> Result<Value, ParseError> {
        self.expect(b'i')?;
        let start = self.pos;
        // Find the terminating 'e'. If we run off the end, we don't have
        // the whole integer yet.
        let mut end = None;
        for (i, b) in self.input[start..].iter().enumerate() {
            if *b == b'e' {
                end = Some(start + i);
                break;
            }
        }
        let end = end.ok_or(ParseError::Incomplete)?;
        let digits = &self.input[start..end];
        if digits.is_empty() {
            return Err(ParseError::Malformed(anyhow!("empty integer at {}", start)));
        }
        // Reject obviously malformed integers: `-` alone, `-0`, leading zeros
        // (other than `0` itself). nREPL servers don't actually emit these,
        // and accepting them would mask bugs.
        let s = std::str::from_utf8(digits)
            .map_err(|_| ParseError::Malformed(anyhow!("non-ASCII integer at {}", start)))?;
        let bytes = s.as_bytes();
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
                s,
                start
            )));
        }
        let n: i64 = s
            .parse()
            .map_err(|e| ParseError::Malformed(anyhow!("integer parse: {e}")))?;
        self.pos = end + 1;
        Ok(Value::Int(n))
    }

    fn parse_bytes(&mut self) -> Result<Vec<u8>, ParseError> {
        let start = self.pos;
        // Length prefix: digits followed by ':'.
        let mut colon = None;
        for (i, b) in self.input[start..].iter().enumerate() {
            if *b == b':' {
                colon = Some(start + i);
                break;
            }
            if !b.is_ascii_digit() {
                return Err(ParseError::Malformed(anyhow!(
                    "non-digit {:?} in byte-string length at {}",
                    *b as char,
                    start + i
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
        // No leading zeros (other than `0` itself).
        if len_digits.len() > 1 && len_digits[0] == b'0' {
            return Err(ParseError::Malformed(anyhow!(
                "non-canonical byte-string length at {start}"
            )));
        }
        let len: usize = std::str::from_utf8(len_digits)
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| {
                ParseError::Malformed(anyhow!("invalid byte-string length at {start}"))
            })?;
        let body_start = colon + 1;
        let body_end = body_start
            .checked_add(len)
            .ok_or_else(|| ParseError::Malformed(anyhow!("byte-string length overflow")))?;
        if body_end > self.input.len() {
            return Err(ParseError::Incomplete);
        }
        let bytes = self.input[body_start..body_end].to_vec();
        self.pos = body_end;
        Ok(bytes)
    }

    fn parse_list(&mut self) -> Result<Value, ParseError> {
        self.expect(b'l')?;
        let mut items = Vec::new();
        loop {
            if self.peek()? == b'e' {
                self.pos += 1;
                return Ok(Value::List(items));
            }
            items.push(self.parse_value()?);
        }
    }

    fn parse_dict(&mut self) -> Result<Value, ParseError> {
        self.expect(b'd')?;
        let mut map = BTreeMap::new();
        loop {
            if self.peek()? == b'e' {
                self.pos += 1;
                return Ok(Value::Dict(map));
            }
            // Per the spec, dict keys MUST be byte strings. We do NOT enforce
            // sorted order on input — real nREPL servers occasionally emit
            // unsorted keys and rejecting them would silently drop replies.
            let key = self.parse_bytes()?;
            let value = self.parse_value()?;
            if map.insert(key.clone(), value).is_some() {
                // Duplicate key. The spec is ambiguous; nREPL never does this
                // in practice, so flag it loudly rather than silently
                // overwrite.
                return Err(ParseError::Malformed(anyhow!(
                    "duplicate dict key {:?}",
                    String::from_utf8_lossy(&key)
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(value: Value) {
        let encoded = encode(&value);
        match decode_one(&encoded).expect("decode") {
            DecodeOutcome::Value {
                value: decoded,
                consumed,
            } => {
                assert_eq!(consumed, encoded.len(), "did not consume full input");
                assert_eq!(decoded, value);
            }
            DecodeOutcome::Incomplete => panic!("complete input reported as incomplete"),
        }
    }

    #[test]
    fn encode_int() {
        assert_eq!(encode(&Value::Int(0)), b"i0e");
        assert_eq!(encode(&Value::Int(42)), b"i42e");
        assert_eq!(encode(&Value::Int(-7)), b"i-7e");
    }

    #[test]
    fn encode_bytes() {
        assert_eq!(encode(&Value::Bytes(b"".to_vec())), b"0:");
        assert_eq!(encode(&Value::str("eval")), b"4:eval");
        assert_eq!(encode(&Value::Bytes(b"\x00\xff".to_vec())), b"2:\x00\xff");
    }

    #[test]
    fn encode_list() {
        let v = Value::List(vec![Value::Int(1), Value::str("ab")]);
        assert_eq!(encode(&v), b"li1e2:abe");
    }

    #[test]
    fn encode_dict_sorts_keys() {
        // Keys are inserted in the "wrong" order; the encoder must emit them
        // sorted. This is what the spec requires for canonical form.
        let v = dict([("op", Value::str("eval")), ("code", Value::str("(+ 1 2)"))]);
        assert_eq!(encode(&v), b"d4:code7:(+ 1 2)2:op4:evale");
    }

    #[test]
    fn roundtrip_int() {
        roundtrip(Value::Int(0));
        roundtrip(Value::Int(123_456_789));
        roundtrip(Value::Int(-1));
        roundtrip(Value::Int(i64::MIN));
        roundtrip(Value::Int(i64::MAX));
    }

    #[test]
    fn roundtrip_bytes() {
        roundtrip(Value::Bytes(Vec::new()));
        roundtrip(Value::str("hello"));
        roundtrip(Value::Bytes((0u8..=255).collect()));
    }

    #[test]
    fn roundtrip_list() {
        roundtrip(Value::List(Vec::new()));
        roundtrip(Value::List(vec![
            Value::Int(1),
            Value::str("two"),
            Value::List(vec![Value::Int(3)]),
        ]));
    }

    #[test]
    fn roundtrip_dict() {
        roundtrip(dict([
            ("id", Value::str("1")),
            ("op", Value::str("eval")),
            ("code", Value::str("(+ 1 2)")),
            ("session", Value::str("abc-123")),
        ]));
    }

    #[test]
    fn roundtrip_nested() {
        roundtrip(dict([
            ("status", Value::List(vec![Value::str("done")])),
            (
                "value",
                Value::List(vec![Value::str("3"), Value::str(":foo")]),
            ),
            ("id", Value::str("42")),
        ]));
    }

    #[test]
    fn decode_accepts_unsorted_dict_keys() {
        // Real nREPL servers occasionally emit unsorted keys. We must accept
        // them; rejecting would silently drop replies (see design doc risk
        // register).
        let bytes = b"d2:op4:eval4:code7:(+ 1 2)e";
        match decode_one(bytes).expect("decode") {
            DecodeOutcome::Value { value, consumed } => {
                assert_eq!(consumed, bytes.len());
                assert_eq!(value.get("op").and_then(Value::as_str), Some("eval"));
                assert_eq!(value.get("code").and_then(Value::as_str), Some("(+ 1 2)"));
            }
            DecodeOutcome::Incomplete => panic!(),
        }
    }

    #[test]
    fn decode_partial_returns_incomplete() {
        // Walk every prefix of a real-ish reply and confirm we report
        // Incomplete (never panic, never return a wrong value) until the
        // full message has arrived.
        let full = encode(&dict([
            ("id", Value::str("1")),
            ("session", Value::str("s")),
            ("value", Value::str("3")),
            ("status", Value::List(vec![Value::str("done")])),
        ]));
        for n in 0..full.len() {
            match decode_one(&full[..n]).expect("decode prefix") {
                DecodeOutcome::Incomplete => {}
                DecodeOutcome::Value { .. } => {
                    panic!("prefix of length {n} parsed as a complete value")
                }
            }
        }
        match decode_one(&full).expect("decode full") {
            DecodeOutcome::Value { consumed, .. } => assert_eq!(consumed, full.len()),
            DecodeOutcome::Incomplete => panic!("full input reported as incomplete"),
        }
    }

    #[test]
    fn decode_leaves_trailing_bytes() {
        // Two messages back-to-back: the first decode should report the
        // exact number of bytes consumed, leaving the second message for
        // the next call.
        let mut buf = encode(&Value::Int(1));
        let first_len = buf.len();
        buf.extend_from_slice(&encode(&Value::str("abc")));
        match decode_one(&buf).expect("decode") {
            DecodeOutcome::Value { value, consumed } => {
                assert_eq!(value, Value::Int(1));
                assert_eq!(consumed, first_len);
            }
            DecodeOutcome::Incomplete => panic!(),
        }
        match decode_one(&buf[first_len..]).expect("decode remainder") {
            DecodeOutcome::Value { value, consumed } => {
                assert_eq!(value, Value::str("abc"));
                assert_eq!(consumed, buf.len() - first_len);
            }
            DecodeOutcome::Incomplete => panic!(),
        }
    }

    #[test]
    fn decode_rejects_malformed_integer() {
        // Empty integer.
        assert!(decode_one(b"ie").is_err());
        // Bare minus sign.
        assert!(decode_one(b"i-e").is_err());
        // Negative zero.
        assert!(decode_one(b"i-0e").is_err());
        // Leading zero.
        assert!(decode_one(b"i01e").is_err());
        // Non-digits.
        assert!(decode_one(b"i1ae").is_err());
    }

    #[test]
    fn decode_rejects_malformed_bytes() {
        // Leading-zero length.
        assert!(decode_one(b"01:a").is_err());
        // Non-digit in length.
        assert!(decode_one(b"a:x").is_err());
    }

    #[test]
    fn decode_rejects_unknown_type_byte() {
        assert!(decode_one(b"x").is_err());
    }

    #[test]
    fn decode_rejects_duplicate_dict_keys() {
        // The spec doesn't explicitly forbid this, but nREPL never emits it
        // and silently overwriting would mask bugs.
        let bytes = b"d1:ai1e1:ai2ee";
        assert!(decode_one(bytes).is_err());
    }

    #[test]
    fn truncated_inside_byte_string_is_incomplete() {
        // Length says 5 bytes, only 3 provided.
        match decode_one(b"5:abc").expect("decode") {
            DecodeOutcome::Incomplete => {}
            DecodeOutcome::Value { .. } => panic!("truncated bytes parsed as complete"),
        }
    }
}
