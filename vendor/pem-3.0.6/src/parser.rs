pub struct Captures<'a> {
    pub begin: &'a [u8],
    pub headers: &'a [u8],
    pub data: &'a [u8],
    pub end: &'a [u8],
}

pub fn parse_captures(input: &[u8]) -> Option<Captures<'_>> {
    parser_inner(input).map(|(_, cap)| cap)
}
pub fn parse_captures_iter(input: &[u8]) -> CaptureMatches<'_> {
    CaptureMatches { input }
}

pub struct CaptureMatches<'a> {
    input: &'a [u8],
}
impl<'a> Iterator for CaptureMatches<'a> {
    type Item = Captures<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        match parser_inner(self.input) {
            Some((remaining, captures)) => {
                self.input = remaining;
                Some(captures)
            }
            None => {
                self.input = &[];
                None
            }
        }
    }
}

fn parse_begin(input: &[u8]) -> Option<(&[u8], &[u8])> {
    let (input, _) = read_until(input, b"-----BEGIN ")?;
    let (input, begin) = read_until(input, b"-----")?;
    let input = skip_whitespace(input);
    Some((input, begin))
}

fn parse_payload(input: &[u8]) -> Option<(&[u8], &[u8])> {
    read_until(input, b"-----END ")
}

fn extract_headers_and_data(input: &[u8]) -> (&[u8], &[u8]) {
    if let Some((rest, headers)) = read_until(input, b"\n\n") {
        (headers, rest)
    } else if let Some((rest, headers)) = read_until(input, b"\r\n\r\n") {
        (headers, rest)
    } else {
        (&[], input)
    }
}

fn parse_end(input: &[u8]) -> Option<(&[u8], &[u8])> {
    let (remaining, end) = read_until(input, b"-----")?;
    let remaining = skip_whitespace(remaining);
    Some((remaining, end))
}

fn parser_inner(input: &[u8]) -> Option<(&[u8], Captures<'_>)> {
    // Should be equivalent to the regex
    // "(?s)-----BEGIN (?P<begin>.*?)-----[ \t\n\r]*(?P<data>.*?)-----END (?P<end>.*?)-----[ \t\n\r]*"

    // (?s)                                      # Enable dotall (. matches all characters incl \n)
    // -----BEGIN (?P<begin>.*?)-----[ \t\n\r]*  # Parse begin
    // (?P<data>.*?)                             # Parse data
    // -----END (?P<end>.*?)-----[ \t\n\r]*      # Parse end

    let (input, begin) = parse_begin(input)?;
    let (input, payload) = parse_payload(input)?;
    let (headers, data) = extract_headers_and_data(payload);
    let (remaining, end) = parse_end(input)?;

    let captures = Captures {
        begin,
        headers,
        data,
        end,
    };
    Some((remaining, captures))
}

// Equivalent to the regex [ \t\n\r]*
fn skip_whitespace(mut input: &[u8]) -> &[u8] {
    while let Some(b) = input.first() {
        match b {
            b' ' | b'\t' | b'\n' | b'\r' => {
                input = &input[1..];
            }
            _ => break,
        }
    }
    input
}
// Equivalent to (.*?) followed by a string
// Returns the remaining input (after the secondary matched string) and the matched data
fn read_until<'a>(input: &'a [u8], marker: &[u8]) -> Option<(&'a [u8], &'a [u8])> {
    // If there is no end condition, short circuit
    if marker.is_empty() {
        return Some((&[], input));
    }
    let mut index = 0;
    let mut found = 0;
    while input.len() - index >= marker.len() - found {
        if input[index] == marker[found] {
            found += 1;
        } else {
            found = 0;
        }
        index += 1;
        if found == marker.len() {
            let remaining = &input[index..];
            let matched = &input[..index - found];
            return Some((remaining, matched));
        }
    }
    None
}
