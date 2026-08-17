pub(crate) fn find_matching_brace(text: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, ch) in text[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn find_tag_end(svg: &str, start: usize) -> Option<usize> {
    let mut quote = None;
    for (offset, ch) in svg[start..].char_indices() {
        match ch {
            '"' | '\'' if quote == Some(ch) => quote = None,
            '"' | '\'' if quote.is_none() => quote = Some(ch),
            '>' if quote.is_none() => return Some(start + offset),
            _ => {}
        }
    }
    None
}

pub(crate) fn extract_root_svg_id(svg: &str) -> Option<String> {
    let start = svg.find("<svg")?;
    let end = find_tag_end(svg, start)?;
    let tag = &svg[start..=end];
    extract_quoted_attr(tag, "id").map(ToOwned::to_owned)
}

pub(crate) fn extract_quoted_attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!(r#"{name}=""#);
    let i = tag.find(&needle)?;
    let rest = &tag[i + needle.len()..];
    let end = rest.find('"')?;
    Some(rest[..end].trim())
}

pub(crate) fn escape_xml_attr(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}
