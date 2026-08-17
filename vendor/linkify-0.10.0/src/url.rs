use std::char;
use std::ops::Range;

use crate::domains::find_authority_end;
use crate::scanner::Scanner;

/// Minimum valid URL length
///
/// The shortest valid URL (without a scheme) might be g.cn (Google China),
/// which consists of four characters.
/// We set this as a lower threshold for parsing URLs from plaintext
/// to avoid false-positives and as a slight performance optimization.
/// This threshold might be adjusted in the future.
const MIN_URL_LENGTH: usize = 4;

const QUOTES: &[char] = &['\'', '\"'];

/// Scan for URLs starting from the trigger character ":" (requires "://").
///
/// Based on RFC 3986.
pub struct UrlScanner {
    pub iri_parsing_enabled: bool,
}

/// Scan for plain domains (without scheme) such as `test.com` or `test.com/hi-there`.
pub struct DomainScanner {
    pub iri_parsing_enabled: bool,
}

impl Scanner for UrlScanner {
    /// Scan for an URL at the given separator index in the string.
    ///
    /// Returns `None` if none was found.
    fn scan(&self, s: &str, separator: usize) -> Option<Range<usize>> {
        // There must be something before separator for scheme
        if separator == 0 {
            return None;
        }

        if !s[separator..].starts_with("://") {
            // We only support schemes with authority, not things like `myscheme:mything`.
            return None;
        }

        let after_separator = separator + "://".len();

        // Need at least one character after '//'
        if after_separator >= s.len() {
            return None;
        }

        if let (Some(start), quote) = find_scheme_start(&s[0..separator]) {
            let scheme = &s[start..separator];
            let s = &s[after_separator..];

            let require_host = scheme_requires_host(scheme);

            if let (Some(after_authority), _) =
                find_authority_end(s, true, require_host, true, self.iri_parsing_enabled)
            {
                if let Some(end) =
                    find_url_end(&s[after_authority..], quote, self.iri_parsing_enabled)
                {
                    if after_authority == 0 && end == 0 {
                        return None;
                    }

                    let range = Range {
                        start,
                        end: after_separator + after_authority + end,
                    };
                    return Some(range);
                }
            }
        }

        None
    }
}

impl Scanner for DomainScanner {
    fn scan(&self, s: &str, separator: usize) -> Option<Range<usize>> {
        // There must be something before separator for domain, and a minimum number of characters
        if separator == 0 || s.len() < MIN_URL_LENGTH {
            return None;
        }

        if let (Some(start), quote) = find_domain_start(&s[0..separator], self.iri_parsing_enabled)
        {
            let s = &s[start..];

            if let (Some(domain_end), Some(_)) =
                find_authority_end(s, false, true, true, self.iri_parsing_enabled)
            {
                if let Some(end) = find_url_end(&s[domain_end..], quote, self.iri_parsing_enabled) {
                    let range = Range {
                        start,
                        end: start + domain_end + end,
                    };
                    return Some(range);
                }
            }
        }

        None
    }
}

/// Find start of scheme, e.g. from `https://`, start at `s` and end at `h`.
fn find_scheme_start(s: &str) -> (Option<usize>, Option<char>) {
    let mut first = None;
    let mut special = None;
    let mut quote = None;
    for (i, c) in s.char_indices().rev() {
        match c {
            'a'..='z' | 'A'..='Z' => first = Some(i),
            '0'..='9' => special = Some(i),
            '+' | '-' | '.' => {}
            '@' => return (None, None),
            c if QUOTES.contains(&c) => {
                // Check if there's a quote before the scheme,
                // and stop once we encounter one of those quotes.
                // https://github.com/robinst/linkify/issues/20
                quote = Some(c);
                break;
            }
            _ => break,
        }
    }

    // We don't want to extract "abc://foo" out of "1abc://foo".
    // ".abc://foo" and others are ok though, as they feel more like separators.
    if let Some(first) = first {
        if let Some(special) = special {
            // Comparing the byte indices with `- 1` is ok as scheme must be ASCII
            if first > 0 && first - 1 == special {
                return (None, quote);
            }
        }
    }
    (first, quote)
}

/// Whether a scheme requires that authority looks like a host name (domain or IP address) or not
/// (can contain reg-name with arbitrary allowed characters).
///
/// We could make this configurable, but let's keep it simple until someone asks (hi!).
fn scheme_requires_host(scheme: &str) -> bool {
    match scheme {
        "https" | "http" | "ftp" | "ssh" => true,
        _ => false,
    }
}

/// Find the start of a plain domain URL (no scheme), e.g. from `blog.`, start at `g` and end at `b`.
/// The rules are:
/// - Domain is labels separated by `.`. Because we're starting at the first `.`, we only need to
///   handle one label.
/// - Label can not start or end with `-`
/// - Label can contain letters, digits, `-` or Unicode if iri_allowed flag is true
fn find_domain_start(s: &str, iri_parsing_enabled: bool) -> (Option<usize>, Option<char>) {
    let mut first = None;
    let mut quote = None;

    for (i, c) in s.char_indices().rev() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => first = Some(i),
            '\u{80}'..=char::MAX if iri_parsing_enabled => first = Some(i),
            // If we had something valid like `https://www.` we'd have found it with the ":"
            // scanner already. We don't want to allow `.../www.example.com` just by itself.
            // We *could* allow `//www.example.com` (scheme-relative URLs) in the future.
            '/' => return (None, None),
            // Similar to above, if this was an email we'd have found it already.
            '@' => return (None, None),
            // If this was a valid domain, we'd have extracted it already from the previous "."
            '.' => return (None, None),
            '-' => {
                if first == None {
                    // Domain label can't end with `-`
                    return (None, None);
                } else {
                    first = Some(i);
                }
            }
            c if QUOTES.contains(&c) => {
                // Check if there's a quote before, and stop once we encounter one of those quotes,
                // e.g. with `"www.example.com"`
                quote = Some(c);
                break;
            }
            _ => break,
        }
    }

    if let Some(first) = first {
        if s[first..].starts_with('-') {
            // Domain label can't start with `-`
            return (None, None);
        }
    }

    (first, quote)
}

/// Find the end of a URL. At this point we already scanned past a valid authority. So e.g. in
/// `https://example.com/foo` we're starting at `/` and want to end at `o`.
fn find_url_end(s: &str, quote: Option<char>, iri_parsing_enabled: bool) -> Option<usize> {
    let mut round = 0;
    let mut square = 0;
    let mut curly = 0;
    let mut single_quote = false;

    let mut previous_can_be_last = true;
    let mut end = Some(0);

    if !s[0..].starts_with("/") && !s[0..].starts_with("?") {
        return Some(0);
    }

    for (i, c) in s.char_indices() {
        let can_be_last = match c {
            '\u{00}'..='\u{1F}' | ' ' | '|' | '\"' | '<' | '>' | '`' | '\u{7F}'..='\u{9F}' => {
                // These can never be part of an URL, so stop now. See RFC 3986 and RFC 3987.
                // Some characters are not in the above list, even they are not in "unreserved"
                // or "reserved":
                //   '\\', '^', '{', '}'
                // The reason for this is that other link detectors also allow them. Also see
                // below, we require the braces to be balanced.
                break;
            }
            '?' | '!' | '.' | ',' | ':' | ';' | '*' => {
                // These may be part of an URL but not at the end. It's not that the spec
                // doesn't allow them, but they are frequently used in plain text as delimiters
                // where they're not meant to be part of the URL.
                false
            }
            '/' => {
                // This may be part of an URL and at the end, but not if the previous character
                // can't be the end of an URL
                previous_can_be_last
            }
            '(' => {
                round += 1;
                false
            }
            ')' => {
                round -= 1;
                if round < 0 {
                    // More closing than opening brackets, stop now
                    break;
                }
                true
            }
            '[' => {
                square += 1;
                false
            }
            ']' => {
                square -= 1;
                if square < 0 {
                    // More closing than opening brackets, stop now
                    break;
                }
                true
            }
            '{' => {
                curly += 1;
                false
            }
            '}' => {
                curly -= 1;
                if curly < 0 {
                    // More closing than opening brackets, stop now
                    break;
                }
                true
            }
            _ if Some(c) == quote => {
                // Found matching quote from beginning of URL, stop now
                break;
            }
            '\'' => {
                single_quote = !single_quote;
                // A single quote can only be the end of an URL if there's an even number
                !single_quote
            }
            '\u{80}'..=char::MAX if !iri_parsing_enabled => false,

            _ => true,
        };
        if can_be_last {
            end = Some(i + c.len_utf8());
        }
        previous_can_be_last = can_be_last;
    }

    end
}
