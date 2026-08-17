use std::borrow::Cow;

/// Decodes Mermaid's `encodeEntities` placeholders and shorthand `#...;` sequences into Unicode.
///
/// Upstream Mermaid runs `encodeEntities(text)` before parsing, and later uses `decodeEntities`
/// + browser `entityDecode(...)` to turn placeholders into actual characters.
///
/// In `merman` we decode these into Unicode as part of headless parsing so that:
/// - layout measurements operate on the same final text
/// - SVG output matches upstream DOM output
pub fn decode_mermaid_entities_to_unicode(input: &str) -> Cow<'_, str> {
    // Fast path: nothing to decode.
    if !input.contains('#') && !input.contains('&') && !input.contains('ﬂ') && !input.contains('¶')
    {
        return Cow::Borrowed(input);
    }

    // Step 1: Mermaid placeholders -> `&...;` / `&#...;`
    let mut s = input.to_string();
    if s.contains('ﬂ') || s.contains('¶') {
        s = s.replace("ﬂ°°", "&#").replace("ﬂ°", "&").replace("¶ß", ";");
    }

    // Step 2 (shorthand): `#...;` -> `&...;` / `&#...;`
    //
    // This is primarily for older headless code paths / fixtures that bypass upstream-like
    // preprocessing. It is intentionally conservative and only rewrites `#\w+;` patterns.
    if s.contains('#') {
        let mut out = String::with_capacity(s.len());
        let mut it = s.chars().peekable();
        let mut prev: Option<char> = None;
        while let Some(ch) = it.next() {
            if ch != '#' {
                out.push(ch);
                prev = Some(ch);
                continue;
            }

            // Do not treat `&#...;` as Mermaid shorthand `#...;`.
            if prev == Some('&') {
                out.push('#');
                prev = Some('#');
                continue;
            }

            let mut entity = String::new();
            let mut ok = false;
            for _ in 0..64 {
                match it.peek().copied() {
                    Some(';') => {
                        it.next();
                        ok = true;
                        break;
                    }
                    Some(c) if c.is_ascii_alphanumeric() || c == '_' || c == '+' => {
                        entity.push(c);
                        it.next();
                    }
                    _ => break,
                }
            }

            if !ok {
                out.push('#');
                out.push_str(&entity);
                continue;
            }

            let is_int = entity.chars().all(|c| c.is_ascii_digit() || c == '+')
                && entity.chars().any(|c| c.is_ascii_digit());
            if is_int {
                out.push('&');
                out.push('#');
                out.push_str(&entity);
                out.push(';');
            } else {
                out.push('&');
                out.push_str(&entity);
                out.push(';');
            }
            prev = Some(';');
        }
        s = out;
    }

    // Step 3: HTML entity decode (`&nbsp;`, `&#9829;`, `&infin;`, ...)
    //
    // Use a standards-based entity decoder so named entities match browser behavior.
    Cow::Owned(htmlize::unescape(&s).into_owned())
}
