//! Validator for `format` keyword.
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
    sync::Arc,
};

use email_address::EmailAddress;
use fancy_regex::Regex;
use serde_json::{Map, Value};
use std::sync::LazyLock;
use unicode_general_category::{get_general_category, GeneralCategory};
use uuid_simd::{parse_hyphenated, Out};

use crate::{
    compiler, ecma,
    error::ValidationError,
    keywords::CompilationResult,
    paths::{LazyLocation, Location},
    thread::ThreadBound,
    types::JsonType,
    validator::{Validate, ValidationContext},
    Draft,
};

static URI_TEMPLATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?:(?:[^\x00-\x20"'<>%\\^`{|}]|%[0-9a-f]{2})|\{[+#./;?&=,!@|]?(?:[a-z0-9_]|%[0-9a-f]{2})+(?::[1-9][0-9]{0,3}|\*)?(?:,(?:[a-z0-9_]|%[0-9a-f]{2})+(?::[1-9][0-9]{0,3}|\*)?)*})*\z"#
    )
    .expect("Is a valid regex")
});

fn is_valid_json_pointer(pointer: &str) -> bool {
    if pointer.is_empty() {
        // An empty string is a valid JSON Pointer
        return true;
    }

    let mut chars = pointer.chars();

    // The first character must be a '/'
    if chars.next() != Some('/') {
        return false;
    }
    is_valid_json_pointer_impl(chars)
}

fn is_valid_relative_json_pointer(s: &str) -> bool {
    let mut chars = s.chars();

    // Parse the non-negative integer part
    match chars.next() {
        Some('0') => {
            // If it starts with '0', it must be followed by '#' or '/'
            match chars.next() {
                Some('#') => chars.next().is_none(),
                Some('/') => is_valid_json_pointer_impl(chars),
                None => true,
                _ => false,
            }
        }
        Some(c) if c.is_ascii_digit() => {
            // Parse the rest of the integer
            while let Some(c) = chars.next() {
                match c {
                    '#' => return chars.next().is_none(),
                    '/' => return is_valid_json_pointer_impl(chars),
                    c if c.is_ascii_digit() => {}
                    _ => return false,
                }
            }
            // Valid if it's just a number
            true
        }
        _ => false,
    }
}

#[inline]
fn is_valid_json_pointer_impl<I: Iterator<Item = char>>(chars: I) -> bool {
    let mut escaped = false;
    for c in chars {
        match c {
            // '/' is only allowed as a separator between reference tokens
            '/' if !escaped => escaped = false,
            '~' if !escaped => escaped = true,
            '0' | '1' if escaped => escaped = false,
            // These ranges cover all allowed unescaped characters
            '\x00'..='\x2E' | '\x30'..='\x7D' | '\x7F'..='\u{10FFFF}' if !escaped => {}
            // Any other character or combination is invalid
            _ => return false,
        }
    }
    // If we end in an escaped state, it's invalid
    !escaped
}

fn is_valid_date(date: &str) -> bool {
    if date.len() != 10 {
        return false;
    }

    let bytes = date.as_bytes();

    // Check format: YYYY-MM-DD
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }

    // Parse year (YYYY)
    let Some(year) = parse_four_digits(&bytes[0..4]) else {
        return false;
    };

    // Parse month (MM)
    let Some(month) = parse_two_digits(&bytes[5..7]) else {
        return false;
    };
    if !(1..=12).contains(&month) {
        return false;
    }

    // Parse day (DD)
    let Some(day) = parse_two_digits(&bytes[8..10]) else {
        return false;
    };
    if day == 0 {
        return false;
    }

    // Check day validity
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11 => day <= 30,
        2 => {
            if is_leap_year(year) {
                day <= 29
            } else {
                day <= 28
            }
        }
        _ => unreachable!("Month value is checked above"),
    }
}

#[inline]
fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[inline]
fn parse_four_digits(bytes: &[u8]) -> Option<u16> {
    let value = u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

    // Check if all bytes are ASCII digits
    if value.wrapping_sub(0x3030_3030) & 0xF0F0_F0F0 == 0 {
        let val = (value & 0x0F0F_0F0F).wrapping_mul(2561) >> 8;
        Some(((val & 0x00FF_00FF).wrapping_mul(6_553_601) >> 16) as u16)
    } else {
        None
    }
}

#[inline]
fn parse_two_digits(bytes: &[u8]) -> Option<u8> {
    let value = u16::from_ne_bytes([bytes[0], bytes[1]]);

    // Check if both bytes are ASCII digits
    if value.wrapping_sub(0x3030) & 0xF0F0 == 0 {
        Some(((value & 0x0F0F).wrapping_mul(2561) >> 8) as u8)
    } else {
        None
    }
}

macro_rules! handle_offset {
    ($sign:tt, $i:ident, $bytes:expr, $hour:expr, $minute:expr, $second:expr) => {{
        if $bytes.len() - $i != 6 {
            return false;
        }
        $i += 1;
        if $bytes[$i + 2] != b':' {
            return false;
        }
        let Some(offset_hh) = parse_two_digits(&$bytes[$i..$i + 2]) else {
            return false;
        };
        let Some(offset_mm) = parse_two_digits(&$bytes[$i + 3..$i + 5]) else {
            return false;
        };
        if offset_hh > 23 || offset_mm > 59 {
            return false;
        }

        if $second == 60 {
            let mut utc_hh = i16::from($hour);
            let mut utc_mm = i16::from($minute);
            let offset_hh = i16::from(offset_hh);
            let offset_mm = i16::from(offset_mm);

            // Apply offset based on the sign (+ or -)
            utc_hh $sign offset_hh;
            utc_mm $sign offset_mm;

            // Adjust for minute overflow/underflow
            utc_hh += utc_mm / 60;
            utc_mm %= 60;
            if utc_mm < 0 {
                utc_mm += 60;
                utc_hh -= 1;
            }

            // Adjust for hour overflow/underflow
            utc_hh = (utc_hh + 24) % 24;
            utc_hh == 23 && utc_mm == 59
        } else {
            true
        }
    }};
}

fn is_valid_time(time: &str) -> bool {
    let bytes = time.as_bytes();
    let len = bytes.len();

    if len < 9 {
        // Minimum valid time is "HH:MM:SSZ"
        return false;
    }

    // Check HH:MM:SS format
    if bytes[2] != b':' || bytes[5] != b':' {
        return false;
    }

    // Parse hour (HH)
    let Some(hour) = parse_two_digits(&bytes[..2]) else {
        return false;
    };
    // Parse minute (MM)
    let Some(minute) = parse_two_digits(&bytes[3..5]) else {
        return false;
    };
    // Parse second (SS)
    let Some(second) = parse_two_digits(&bytes[6..8]) else {
        return false;
    };

    if hour > 23 || minute > 59 || second > 60 {
        return false;
    }

    let mut i = 8;

    // Check fractional seconds
    if i < len && bytes[i] == b'.' {
        i += 1;
        let mut has_digit = false;
        while i < len && bytes[i].is_ascii_digit() {
            has_digit = true;
            i += 1;
        }
        if !has_digit {
            return false;
        }
    }

    // Check offset
    if i == len {
        return false;
    }

    match bytes[i] {
        b'Z' | b'z' => i == len - 1 && (second != 60 || (hour == 23 && minute == 59)),
        b'+' => handle_offset!(-=, i, bytes, hour, minute, second),
        b'-' => handle_offset!(+=, i, bytes, hour, minute, second),
        _ => false,
    }
}

fn is_valid_datetime(datetime: &str) -> bool {
    // Find the position of 'T' or 't' separator
    let Some(t_pos) = datetime.bytes().position(|b| b == b'T' || b == b't') else {
        return false;
    };

    // Split the string into date and time parts
    let (date_part, time_part) = datetime.split_at(t_pos);

    is_valid_date(date_part) && is_valid_time(&time_part[1..])
}

fn is_valid_email_impl<F>(email: &str, is_valid_hostname_impl: F) -> bool
where
    F: Fn(&str) -> bool,
{
    if let Ok(parsed) = EmailAddress::from_str(email) {
        let domain = parsed.domain();
        if let Some(domain) = domain.strip_prefix('[').and_then(|d| d.strip_suffix(']')) {
            if let Some(domain) = domain.strip_prefix("IPv6:") {
                domain.parse::<Ipv6Addr>().is_ok()
            } else {
                domain.parse::<Ipv4Addr>().is_ok()
            }
        } else {
            is_valid_hostname_impl(domain)
        }
    } else {
        false
    }
}

fn is_valid_email(email: &str) -> bool {
    is_valid_email_impl(email, is_valid_hostname)
}

fn is_valid_idn_email(email: &str) -> bool {
    is_valid_email_impl(email, is_valid_idn_hostname)
}

fn is_valid_hostname(hostname: &str) -> bool {
    const VALID_CHARS: [bool; 256] = {
        let mut table = [false; 256];
        let mut byte: u8 = 0;
        while byte < 255 {
            table[byte as usize] = matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-');
            byte += 1;
        }
        // Handle byte 255 separately to avoid overflow
        table[255] = matches!(255u8, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-');
        table
    };

    fn label_allows_punycode(label: &[u8]) -> bool {
        label.len() >= 4
            && label[0] == b'x'
            && label[1] == b'n'
            && label[2] == b'-'
            && label[3] == b'-'
    }

    fn validate_label(label: &[u8]) -> bool {
        if label.is_empty()
            || label.len() > 63
            || label[0] == b'-'
            || *label.last().unwrap() == b'-'
        {
            return false;
        }

        if label.len() >= 4 && label[2] == b'-' && label[3] == b'-' && !label_allows_punycode(label)
        {
            return false;
        }

        true
    }

    fn contains_punycode_label(hostname: &[u8]) -> bool {
        hostname.split(|&b| b == b'.').any(|label| {
            label.len() >= 4
                && label[0] == b'x'
                && label[1] == b'n'
                && label[2] == b'-'
                && label[3] == b'-'
        })
    }

    let hostname_bytes = hostname.as_bytes();
    let len = hostname_bytes.len();
    if len == 0 || len > 253 || hostname_bytes[len - 1] == b'.' {
        return false;
    }

    let mut label_start = 0;
    let mut i = 0;

    while i < len {
        if hostname_bytes[i] == b'.' {
            if !validate_label(&hostname_bytes[label_start..i]) {
                return false;
            }
            label_start = i + 1;
        } else if !VALID_CHARS[hostname_bytes[i] as usize] {
            return false;
        }
        i += 1;
    }

    if !validate_label(&hostname_bytes[label_start..]) {
        return false;
    }

    if contains_punycode_label(hostname_bytes) {
        use idna::punycode;
        for label in hostname_bytes.split(|&b| b == b'.') {
            if label.len() >= 4
                && label[0] == b'x'
                && label[1] == b'n'
                && label[2] == b'-'
                && label[3] == b'-'
            {
                let payload =
                    std::str::from_utf8(&label[4..]).expect("ASCII label already validated");
                let Some(decoded) = punycode::decode_to_string(payload) else {
                    return false;
                };
                if !validate_unicode_label(&decoded) {
                    return false;
                }
            }
        }
    }

    true
}

fn validate_unicode_label(label: &str) -> bool {
    let mut chars = label.chars().peekable();
    if let Some(&first) = chars.peek() {
        let category = get_general_category(first);
        if matches!(
            category,
            GeneralCategory::SpacingMark
                | GeneralCategory::NonspacingMark
                | GeneralCategory::EnclosingMark
        ) {
            return false;
        }
    }
    let mut previous = None;
    let mut has_katakana_middle_dot = false;
    let mut has_hiragana_katakana_han = false;
    let mut has_arabic_indic_digits = false;
    let mut has_extended_arabic_indic_digits = false;

    while let Some(current) = chars.next() {
        match current {
            // ZERO WIDTH JOINER
            // https://www.rfc-editor.org/rfc/rfc5892#appendix-A.2
            '\u{200D}'
                if !previous.is_some_and(|prev| {
                    matches!(
                        prev,
                        '\u{094D}'
                            | '\u{09CD}'
                            | '\u{0A4D}'
                            | '\u{0ACD}'
                            | '\u{0B4D}'
                            | '\u{0BCD}'
                            | '\u{0C4D}'
                            | '\u{0CCD}'
                            | '\u{0D4D}'
                            | '\u{0DCA}'
                            | '\u{0E3A}'
                            | '\u{0F84}'
                            | '\u{1039}'
                            | '\u{1714}'
                            | '\u{1734}'
                            | '\u{17D2}'
                            | '\u{1A60}'
                            | '\u{1B44}'
                            | '\u{1BAA}'
                            | '\u{1BF2}'
                            | '\u{1BF3}'
                            | '\u{2D7F}'
                            | '\u{A806}'
                            | '\u{A8C4}'
                            | '\u{A953}'
                            | '\u{ABED}'
                            | '\u{10A3F}'
                            | '\u{11046}'
                            | '\u{1107F}'
                            | '\u{110B9}'
                            | '\u{11133}'
                            | '\u{111C0}'
                            | '\u{11235}'
                            | '\u{112EA}'
                            | '\u{1134D}'
                            | '\u{11442}'
                            | '\u{114C2}'
                            | '\u{115BF}'
                            | '\u{1163F}'
                            | '\u{116B6}'
                            | '\u{1172B}'
                            | '\u{11839}'
                            | '\u{119E0}'
                            | '\u{11A34}'
                            | '\u{11A47}'
                            | '\u{11A99}'
                            | '\u{11C3F}'
                            | '\u{11D44}'
                            | '\u{11D45}'
                            | '\u{11D97}'
                    )
                }) =>
            {
                return false;
            }
            // MIDDLE DOT
            // https://www.rfc-editor.org/rfc/rfc5892#appendix-A.3
            '\u{00B7}' if previous != Some('l') || chars.peek() != Some(&'l') => return false,
            // Greek KERAIA
            // https://www.rfc-editor.org/rfc/rfc5892#appendix-A.4
            '\u{0375}'
                if !chars
                    .peek()
                    .is_some_and(|next| ('\u{0370}'..='\u{03FF}').contains(next)) =>
            {
                return false
            }
            // Hebrew GERESH and GERSHAYIM
            // https://www.rfc-editor.org/rfc/rfc5892#appendix-A.5
            // https://www.rfc-editor.org/rfc/rfc5892#appendix-A.6
            '\u{05F3}' | '\u{05F4}'
                if !previous.is_some_and(|prev| ('\u{0590}'..='\u{05FF}').contains(&prev)) =>
            {
                return false
            }
            // KATAKANA MIDDLE DOT
            '\u{30FB}' => has_katakana_middle_dot = true,
            // Hiragana, Katakana, or Han
            // https://www.rfc-editor.org/rfc/rfc5892#appendix-A.7
            '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}' | '\u{4E00}'..='\u{9FFF}' => {
                has_hiragana_katakana_han = true;
            }
            // ARABIC-INDIC DIGITS
            // https://www.rfc-editor.org/rfc/rfc5892#appendix-A.8
            '\u{0660}'..='\u{0669}' => has_arabic_indic_digits = true,
            // EXTENDED ARABIC-INDIC DIGITS
            // https://www.rfc-editor.org/rfc/rfc5892#appendix-A.9
            '\u{06F0}'..='\u{06F9}' => has_extended_arabic_indic_digits = true,
            // DISALLOWED
            '\u{0640}' | '\u{07FA}' | '\u{302E}' | '\u{302F}' | '\u{3031}' | '\u{3032}'
            | '\u{3033}' | '\u{3034}' | '\u{3035}' | '\u{303B}' => return false,

            _ => {}
        }
        previous = Some(current);
    }

    if (has_katakana_middle_dot && !has_hiragana_katakana_han)
        || (has_arabic_indic_digits && has_extended_arabic_indic_digits)
    {
        return false;
    }

    true
}

fn is_valid_idn_hostname(hostname: &str) -> bool {
    use idna::uts46::{AsciiDenyList, DnsLength, Hyphens, Uts46};

    let Ok(ascii_hostname) = Uts46::new().to_ascii(
        hostname.as_bytes(),
        AsciiDenyList::STD3,
        // Prohibit hyphens in the first, third, fourth, and last position in the label
        Hyphens::Check,
        DnsLength::Verify,
    ) else {
        return false;
    };

    if !is_valid_hostname(&ascii_hostname) {
        return false;
    }

    let (unicode_hostname, _) = Uts46::new().to_unicode(
        ascii_hostname.as_bytes(),
        AsciiDenyList::EMPTY,
        Hyphens::Allow,
    );

    unicode_hostname
        .split('.')
        .all(|label| !label.is_empty() && validate_unicode_label(label))
}

#[inline]
fn unit_index(units: &[u8], unit: u8) -> Option<usize> {
    units.iter().position(|&u| u == unit)
}

fn is_valid_duration(duration: &str) -> bool {
    let bytes = duration.as_bytes();
    let len = bytes.len();

    if len < 2 || bytes[0] != b'P' {
        return false;
    }

    let mut i = 1;
    let mut has_component = false;
    let mut has_time = false;
    let mut last_date_unit = 0;
    let mut last_time_unit = 0;
    let mut has_weeks = false;
    let mut has_time_component = false;
    let mut seen_units = 0u8;

    let date_units = [b'Y', b'M', b'W', b'D'];
    let time_units = [b'H', b'M', b'S'];

    while i < len {
        if bytes[i] == b'T' {
            if has_time {
                return false;
            }
            has_time = true;
            i += 1;
            continue;
        }

        let start = i;
        while i < len && bytes[i].is_ascii_digit() {
            i += 1;
        }

        if i == start || i == len {
            return false;
        }

        let unit = bytes[i];

        if !has_time {
            if let Some(idx) = unit_index(&date_units, unit) {
                if unit == b'W' {
                    if has_component {
                        return false;
                    }
                    has_weeks = true;
                } else if has_weeks {
                    return false;
                }
                if idx < last_date_unit || (seen_units & (1 << idx) != 0) {
                    return false;
                }
                seen_units |= 1 << idx;
                last_date_unit = idx;
            } else {
                return false;
            }
        } else if let Some(idx) = unit_index(&time_units, unit) {
            if idx < last_time_unit || (seen_units & (1 << (idx + 4)) != 0) {
                return false;
            }
            seen_units |= 1 << (idx + 4);
            last_time_unit = idx;
            has_time_component = true;
        } else {
            return false;
        }

        has_component = true;
        i += 1;
    }

    has_component && (!has_time || has_time_component)
}

fn is_valid_ipv4(ip: &str) -> bool {
    Ipv4Addr::from_str(ip).is_ok()
}

fn is_valid_ipv6(ip: &str) -> bool {
    Ipv6Addr::from_str(ip).is_ok()
}

fn is_valid_iri(iri: &str) -> bool {
    referencing::Iri::parse(iri).is_ok()
}

fn is_valid_iri_reference(iri_reference: &str) -> bool {
    referencing::IriRef::parse(iri_reference).is_ok()
}

fn is_valid_uri(uri: &str) -> bool {
    referencing::Uri::parse(uri).is_ok()
}

fn is_valid_uri_reference(uri_reference: &str) -> bool {
    referencing::UriRef::parse(uri_reference).is_ok()
}

fn is_valid_regex(regex: &str) -> bool {
    ecma::to_rust_regex(regex).is_ok()
}

fn is_valid_uri_template(uri_template: &str) -> bool {
    URI_TEMPLATE_RE
        .is_match(uri_template)
        .expect("Simple URI_TEMPLATE_RE pattern")
}

fn is_valid_uuid(uuid: &str) -> bool {
    let mut out = [0; 16];
    parse_hyphenated(uuid.as_bytes(), Out::from_mut(&mut out)).is_ok()
}

macro_rules! format_validators {
    ($(($validator:ident, $format:expr, $validation_fn:ident)),+ $(,)?) => {
        $(
            struct $validator {
                location: Location,
            }

            impl $validator {
                pub(crate) fn compile<'a>(ctx: &compiler::Context) -> CompilationResult<'a> {
                    let location = ctx.location().join("format");
                    Ok(Box::new($validator { location }))
                }
            }

            impl Validate for $validator {
                fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
                    if let Value::String(item) = instance {
                        $validation_fn(item)
                    } else {
                        true
                    }
                }

                fn validate<'i>(
                    &self,
                    instance: &'i Value,
                    location: &LazyLocation,
                    ctx: &mut ValidationContext,
                ) -> Result<(), ValidationError<'i>> {
                    if let Value::String(_item) = instance {
                        if !self.is_valid(instance, ctx) {
                            return Err(ValidationError::format(
                                self.location.clone(),
                                location.into(),
                                instance,
                                $format,
                            ));
                        }
                    }
                    Ok(())
                }
            }
        )+
    };
}
format_validators!(
    (DateValidator, "date", is_valid_date),
    (DateTimeValidator, "date-time", is_valid_datetime),
    (DurationValidator, "duration", is_valid_duration),
    (EmailValidator, "email", is_valid_email),
    (HostnameValidator, "hostname", is_valid_hostname),
    (IdnEmailValidator, "idn-email", is_valid_idn_email),
    (IdnHostnameValidator, "idn-hostname", is_valid_idn_hostname),
    (IpV4Validator, "ipv4", is_valid_ipv4),
    (IpV6Validator, "ipv6", is_valid_ipv6),
    (IriValidator, "iri", is_valid_iri),
    (
        IriReferenceValidator,
        "iri-reference",
        is_valid_iri_reference
    ),
    (JsonPointerValidator, "json-pointer", is_valid_json_pointer),
    (RegexValidator, "regex", is_valid_regex),
    (
        RelativeJsonPointerValidator,
        "relative-json-pointer",
        is_valid_relative_json_pointer
    ),
    (TimeValidator, "time", is_valid_time),
    (UriValidator, "uri", is_valid_uri),
    (
        UriReferenceValidator,
        "uri-reference",
        is_valid_uri_reference
    ),
    (UriTemplateValidator, "uri-template", is_valid_uri_template),
    (UuidValidator, "uuid", is_valid_uuid),
);

struct CustomFormatValidator {
    location: Location,
    format_name: String,
    check: Arc<dyn Format>,
}
impl CustomFormatValidator {
    pub(crate) fn compile<'a>(
        ctx: &compiler::Context,
        format_name: String,
        check: Arc<dyn Format>,
    ) -> CompilationResult<'a> {
        let location = ctx.location().join("format");
        Ok(Box::new(CustomFormatValidator {
            location,
            format_name,
            check,
        }))
    }
}

impl Validate for CustomFormatValidator {
    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if self.is_valid(instance, ctx) {
            Ok(())
        } else {
            Err(ValidationError::format(
                self.location.clone(),
                location.into(),
                instance,
                self.format_name.clone(),
            ))
        }
    }

    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        if let Value::String(item) = instance {
            self.check.is_valid(item)
        } else {
            true
        }
    }
}

pub(crate) trait Format: ThreadBound + 'static {
    fn is_valid(&self, value: &str) -> bool;
}

impl<F> Format for F
where
    F: Fn(&str) -> bool + ThreadBound + 'static,
{
    #[inline]
    fn is_valid(&self, value: &str) -> bool {
        self(value)
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    if !ctx.validates_formats_by_default() {
        return None;
    }

    if let Value::String(format) = schema {
        if let Some((name, func)) = ctx.get_format(format) {
            return Some(CustomFormatValidator::compile(
                ctx,
                name.clone(),
                func.clone(),
            ));
        }
        let draft = ctx.draft();
        match format.as_str() {
            "date" => Some(DateValidator::compile(ctx)),
            "date-time" => Some(DateTimeValidator::compile(ctx)),
            "duration" if draft >= Draft::Draft201909 => Some(DurationValidator::compile(ctx)),
            "email" => Some(EmailValidator::compile(ctx)),
            "hostname" => Some(HostnameValidator::compile(ctx)),
            "idn-email" => Some(IdnEmailValidator::compile(ctx)),
            "idn-hostname" if draft >= Draft::Draft7 => Some(IdnHostnameValidator::compile(ctx)),
            "ipv4" => Some(IpV4Validator::compile(ctx)),
            "ipv6" => Some(IpV6Validator::compile(ctx)),
            "iri" if draft >= Draft::Draft7 => Some(IriValidator::compile(ctx)),
            "iri-reference" if draft >= Draft::Draft7 => Some(IriReferenceValidator::compile(ctx)),
            "json-pointer" if draft >= Draft::Draft6 => Some(JsonPointerValidator::compile(ctx)),
            "regex" => Some(RegexValidator::compile(ctx)),
            "relative-json-pointer" if draft >= Draft::Draft7 => {
                Some(RelativeJsonPointerValidator::compile(ctx))
            }
            "time" => Some(TimeValidator::compile(ctx)),
            "uri" => Some(UriValidator::compile(ctx)),
            "uri-reference" if draft >= Draft::Draft6 => Some(UriReferenceValidator::compile(ctx)),
            "uri-template" if draft >= Draft::Draft6 => Some(UriTemplateValidator::compile(ctx)),
            "uuid" if draft >= Draft::Draft201909 => Some(UuidValidator::compile(ctx)),
            name => {
                if ctx.are_unknown_formats_ignored() {
                    None
                } else {
                    Some(Err(ValidationError::custom(
                        Location::new().join("format"),
                        ctx.location().clone(),
                        schema,
                        format!("Unknown format: '{name}'. Adjust configuration to ignore unrecognized formats"),
                    )))
                }
            }
        }
    } else {
        Some(Err(ValidationError::single_type_error(
            Location::new(),
            ctx.location().clone(),
            schema,
            JsonType::String,
        )))
    }
}

#[cfg(test)]
mod tests {
    use referencing::Draft;
    use serde_json::json;
    use test_case::test_case;

    use crate::tests_util;

    use super::*;

    #[test]
    fn ignored_format() {
        let schema = json!({"format": "custom", "type": "string"});
        let instance = json!("foo");
        let validator = crate::validator_for(&schema).unwrap();
        assert!(validator.is_valid(&instance));
    }

    #[test]
    fn format_validation() {
        let schema = json!({"format": "email", "type": "string"});
        let email_instance = json!("email@example.com");
        let not_email_instance = json!("foo");

        let with_validation = crate::options()
            .should_validate_formats(true)
            .build(&schema)
            .unwrap();
        let without_validation = crate::options()
            .should_validate_formats(false)
            .build(&schema)
            .unwrap();

        assert!(with_validation.is_valid(&email_instance));
        assert!(!with_validation.is_valid(&not_email_instance));
        assert!(without_validation.is_valid(&email_instance));
        assert!(without_validation.is_valid(&not_email_instance));
    }

    #[test]
    fn ecma_regex() {
        // See GH-230
        let schema = json!({"format": "regex", "type": "string"});
        let instance = json!("^\\cc$");
        let validator = crate::validator_for(&schema).unwrap();
        assert!(validator.is_valid(&instance));
    }

    #[test]
    fn location() {
        tests_util::assert_schema_location(&json!({"format": "date"}), &json!("bla"), "/format");
    }

    #[test]
    fn uuid() {
        let schema = json!({"format": "uuid", "type": "string"});

        let passing_instance = json!("f308a72c-fa84-11eb-9a03-0242ac130003");
        let failing_instance = json!("1");

        let validator = crate::options()
            .with_draft(Draft::Draft201909)
            .should_validate_formats(true)
            .build(&schema)
            .unwrap();

        assert!(validator.is_valid(&passing_instance));
        assert!(!validator.is_valid(&failing_instance));
    }

    #[test]
    fn uri() {
        let schema = json!({"format": "uri", "type": "string"});

        let passing_instance = json!("https://phillip.com");
        let failing_instance = json!("redis");

        tests_util::is_valid(&schema, &passing_instance);
        tests_util::is_not_valid(&schema, &failing_instance);
    }

    #[test_case("P1Y1Y")]
    #[test_case("PT1H1H")]
    fn test_invalid_duration(input: &str) {
        assert!(!is_valid_duration(input));
    }

    #[test]
    fn unknown_formats_should_not_be_ignored() {
        let schema = json!({ "format": "custom", "type": "string"});
        let error = crate::options()
            .should_validate_formats(true)
            .should_ignore_unknown_formats(false)
            .build(&schema)
            .expect_err("the validation error should be returned");

        assert_eq!(
            error.to_string(),
            "Unknown format: 'custom'. Adjust configuration to ignore unrecognized formats"
        );
    }

    #[test_case("2023-01-01", true; "valid regular date")]
    #[test_case("2020-02-29", true; "valid leap year date")]
    #[test_case("2021-02-28", true; "valid non-leap year date")]
    #[test_case("1900-02-28", true; "valid century non-leap year")]
    #[test_case("2000-02-29", true; "valid leap century year")]
    #[test_case("1999-12-31", true; "valid end of year date")]
    #[test_case("202-12-01", false; "invalid short year")]
    #[test_case("2023-1-01", false; "invalid short month")]
    #[test_case("2023-12-1", false; "invalid short day")]
    #[test_case("2023/12/01", false; "invalid separators")]
    #[test_case("2023-13-01", false; "invalid month too high")]
    #[test_case("2023-00-01", false; "invalid month too low")]
    #[test_case("2023-12-32", false; "invalid day too high")]
    #[test_case("2023-11-31", false; "invalid day for 30-day month")]
    #[test_case("2023-02-30", false; "invalid day for February in non-leap year")]
    #[test_case("2021-02-29", false; "invalid day for non-leap year")]
    #[test_case("2023-12-00", false; "invalid day too low")]
    #[test_case("99999-12-01", false; "year too long")]
    #[test_case("1900-02-29", false; "invalid leap century non-leap year")]
    #[test_case("2000-02-30", false; "invalid day for leap century year")]
    #[test_case("2400-02-29", true; "valid leap year in distant future")]
    #[test_case("0000-01-01", true; "valid boundary start date")]
    #[test_case("9999-12-31", true; "valid boundary end date")]
    #[test_case("aaaa-01-12", false; "Malformed (letters in year)")]
    #[test_case("2000-bb-12", false; "Malformed (letters in month)")]
    #[test_case("2000-01-cc", false; "Malformed (letters in day)")]
    fn test_is_valid_date(input: &str, expected: bool) {
        assert_eq!(is_valid_date(input), expected);
    }

    #[test_case("23:59:59Z", true; "valid time with Z")]
    #[test_case("00:00:00Z", true; "valid midnight time with Z")]
    #[test_case("12:30:45.123Z", true; "valid time with fractional seconds and Z")]
    #[test_case("23:59:60Z", true; "valid leap second UTC time")]
    #[test_case("12:30:45+01:00", true; "valid time with positive offset")]
    #[test_case("12:30:45-01:00", true; "valid time with negative offset")]
    #[test_case("23:59:60+00:00", true; "valid leap second with offset UTC 00:00")]
    #[test_case("23:59:59+01:00", true; "valid time with +01:00 offset")]
    #[test_case("23:59:59A", false; "invalid time with non-Z/non-offset letter")]
    #[test_case("12:3:45Z", false; "invalid time with missing digit in minute")]
    #[test_case("12:30:4Z", false; "invalid time with missing digit in second")]
    #[test_case("12-30-45Z", false; "invalid time with wrong separator")]
    #[test_case("12:30:45Z+01:00", false; "invalid time with Z and offset together")]
    #[test_case("12:30:45A01:00", false; "invalid time with wrong separator between time and offset")]
    #[test_case("12:30:45++01:00", false; "invalid double plus in offset")]
    #[test_case("12:30:45+01:60", false; "invalid minute in offset")]
    #[test_case("12:30:45+24:00", false; "invalid hour in offset")]
    #[test_case("12:30:45.", false; "invalid time with incomplete fractional second")]
    #[test_case("24:00:00Z", false; "invalid hour > 23")]
    #[test_case("12:60:00Z", false; "invalid minute > 59")]
    #[test_case("12:30:61Z", false; "invalid second > 60")]
    #[test_case("12:30:60+01:00", false; "invalid leap second with non-UTC offset")]
    #[test_case("23:59:60Z+01:00", false; "invalid leap second with non-zero offset")]
    #[test_case("23:59:60+00:30", false; "invalid leap second with non-zero minute offset")]
    #[test_case("23:59:60Z", true; "valid leap second at the end of day")]
    #[test_case("23:59:60+00:00", true; "valid leap second with zero offset")]
    #[test_case("ab:59:59Z", false; "invalid time with letters in hour")]
    #[test_case("23:ab:59Z", false; "invalid time with letters in minute")]
    #[test_case("23:59:abZ", false; "invalid time with letters in second")]
    #[test_case("23:59:59aZ", false; "invalid time with letter after seconds")]
    #[test_case("12:30:45+ab:00", false; "invalid offset hour with letters")]
    #[test_case("12:30:45+01:ab", false; "invalid offset minute with letters")]
    #[test_case("12:30:45.abcZ", false; "invalid fractional seconds with letters")]
    fn test_is_valid_time(input: &str, expected: bool) {
        assert_eq!(is_valid_time(input), expected);
    }

    #[test]
    fn test_is_valid_datetime() {
        assert!(!is_valid_datetime(""));
    }

    #[test_case("127.0.0.1", true)]
    #[test_case("192.168.1.1", true)]
    #[test_case("10.0.0.1", true)]
    #[test_case("0.0.0.0", true)]
    #[test_case("256.1.2.3", false; "first octet too large")]
    #[test_case("1.256.3.4", false; "second octet too large")]
    #[test_case("1.2.256.4", false; "third octet too large")]
    #[test_case("1.2.3.256", false; "fourth octet too large")]
    #[test_case("01.2.3.4", false; "leading zero in first octet")]
    #[test_case("1.02.3.4", false; "leading zero in second octet")]
    #[test_case("1.2.03.4", false; "leading zero in third octet")]
    #[test_case("1.2.3.04", false; "leading zero in fourth octet")]
    #[test_case("1.2.3", false; "too few octets")]
    #[test_case("1.2.3.4.5", false; "too many octets")]
    fn ip_v4(input: &str, expected: bool) {
        let validator = crate::options()
            .should_validate_formats(true)
            .build(&json!({"format": "ipv4", "type": "string"}))
            .expect("Invalid schema");
        assert_eq!(validator.is_valid(&json!(input)), expected);
    }

    #[test]
    fn test_is_valid_datetime_panic() {
        is_valid_datetime("2624-04-25t23:14:04-256\x112");
    }

    #[test_case("example.com" ; "simple valid hostname")]
    #[test_case("xn--bcher-kva.com" ; "valid punycode")]
    #[test_case("münchen.de" ; "valid IDN")]
    #[test_case("test\u{094D}\u{200D}example.com" ; "valid zero width joiner after virama")]
    #[test_case("۱۲۳.example.com" ; "valid extended arabic-indic digits")]
    #[test_case("ひらがな・カタカナ.com" ; "valid katakana middle dot")]
    fn test_valid_idn_hostnames(input: &str) {
        assert!(is_valid_idn_hostname(input));
    }

    #[test_case("xn--ll-0ea" ; "punycode with valid middle dot context")]
    #[test_case("xn--11b2ezcw70k" ; "zero width joiner preceded by virama")]
    fn test_valid_punycode_hostnames(input: &str) {
        assert!(is_valid_hostname(input));
    }

    #[test_case("ex--ample.com" ; "hyphen at 3rd & 4th position")]
    #[test_case("-example.com" ; "leading hyphen")]
    #[test_case("example-.com" ; "trailing hyphen")]
    #[test_case("xn--example.com" ; "invalid punycode")]
    #[test_case("xn--x" ; "too short punycode label")]
    #[test_case("xn--vek" ; "katakana middle dot without companions")]
    #[test_case("xn--l-fda" ; "middle dot with nothing preceding")]
    #[test_case("xn--l-gda" ; "middle dot with nothing following")]
    #[test_case("xn--02b508i" ; "zero width joiner not preceded by virama")]
    #[test_case("xn--a-2hc5h" ; "hebrew geresh not preceded by hebrew")]
    #[test_case("xn--a-2hc8h" ; "hebrew gershayim not preceded by hebrew")]
    #[test_case("test\u{200D}example.com" ; "zero width joiner not after virama")]
    #[test_case("test\u{0061}\u{200D}example.com" ; "zero width joiner after non-virama")]
    #[test_case("" ; "empty string")]
    #[test_case("." ; "single dot")]
    #[test_case("example..com" ; "consecutive dots")]
    #[test_case("exa mple.com" ; "contains space")]
    #[test_case("example.com." ; "trailing dot")]
    #[test_case("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.com" ; "too long")]
    #[test_case("xn--bcher-.com" ; "invalid punycode with hyphen")]
    #[test_case("١۲٣.example.com" ; "mixed arabic-indic digits")]
    #[test_case("example・com" ; "katakana middle dot without hiragana/katakana/han")]
    fn test_invalid_idn_hostnames(input: &str) {
        assert!(!is_valid_idn_hostname(input));
    }

    #[test_case("xn--l-fda" ; "middle dot with nothing preceding")]
    #[test_case("xn--l-gda" ; "middle dot with nothing following")]
    #[test_case("xn--02b508i" ; "zero width joiner not preceded by anything")]
    #[test_case("xn--11b2er09f" ; "zero width joiner not preceded by virama")]
    #[test_case("xn--hello-zed" ; "punycode beginning with nonspacing mark")]
    #[test_case("xn--hello-txk" ; "punycode beginning with spacing combining mark")]
    #[test_case("xn--hello-6bf" ; "punycode beginning with enclosing mark")]
    #[test_case("XN--aa---o47jg78q" ; "uppercase punycode prefix rejected")]
    fn test_invalid_punycode_hostnames(input: &str) {
        assert!(!is_valid_hostname(input));
    }

    #[test]
    fn test_invalid_hostname() {
        assert!(!is_valid_hostname("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.com"));
    }

    #[test_case(""; "empty string")]
    #[test_case("/"; "root")]
    #[test_case("/foo"; "simple key")]
    #[test_case("/foo/0"; "array index")]
    #[test_case("/foo/bar"; "nested keys")]
    #[test_case("/f~0o/b~1r"; "escaped characters")]
    #[test_case("/foo/bar/"; "trailing slash")]
    #[test_case("/foo//bar"; "empty reference token")]
    fn test_valid_json_pointer(pointer: &str) {
        assert!(is_valid_json_pointer(pointer));
    }

    #[test_case("foo"; "missing leading slash")]
    #[test_case("/foo/~"; "incomplete escape")]
    #[test_case("/foo/~2"; "invalid escape")]
    #[test_case("/foo\x7E"; "unescaped tilde")]
    fn test_invalid_json_pointer(pointer: &str) {
        assert!(!is_valid_json_pointer(pointer));
    }

    #[test_case("0"; "zero")]
    #[test_case("1"; "positive integer")]
    #[test_case("10"; "multi-digit integer")]
    #[test_case("0#"; "zero with hash")]
    #[test_case("1#"; "positive integer with hash")]
    #[test_case("0/"; "zero with slash")]
    #[test_case("1/foo"; "integer with json pointer")]
    #[test_case("10/foo/bar"; "multi-digit integer with json pointer")]
    fn test_valid_relative_json_pointer(pointer: &str) {
        assert!(is_valid_relative_json_pointer(pointer));
    }

    #[test_case(""; "empty string")]
    #[test_case("-1"; "negative integer")]
    #[test_case("01"; "leading zero")]
    #[test_case("1.5"; "decimal")]
    #[test_case("a"; "non-digit")]
    #[test_case("1a"; "digit followed by non-digit")]
    #[test_case("1#/"; "hash not at end")]
    #[test_case("1/~"; "incomplete escape in json pointer")]
    fn test_invalid_relative_json_pointer(pointer: &str) {
        assert!(!is_valid_relative_json_pointer(pointer));
    }
}
