fn state_url_scheme_like(input: &str) -> Option<&str> {
    let lower = input.to_ascii_lowercase();

    let last_colon = input.rfind(':');
    let last_entity = lower.rfind("&colon;");

    let mut best_end = None::<usize>;

    if let Some(idx) = last_colon {
        best_end = Some(idx + 1);
    }

    if let Some(idx) = last_entity {
        let end = idx + "&colon;".len();
        if best_end.is_none_or(|cur| end > cur) {
            best_end = Some(end);
        }
    }

    best_end.map(|end| &input[..end])
}

fn state_is_invalid_protocol_like(url_scheme: &str) -> bool {
    let lower = url_scheme.to_ascii_lowercase();
    let trimmed = lower.trim();
    let trimmed = trimmed.trim_start_matches(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'));

    trimmed.starts_with("javascript")
        || trimmed.starts_with("data")
        || trimmed.starts_with("vbscript")
}

pub(super) fn state_link_href_allowed(url: &str) -> bool {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return false;
    }

    if matches!(trimmed.as_bytes().first(), Some(b'.' | b'/')) {
        return true;
    }

    let trimmed_url = trimmed.trim_start();
    let Some(url_scheme) = state_url_scheme_like(trimmed_url) else {
        return true;
    };

    !state_is_invalid_protocol_like(url_scheme)
}
