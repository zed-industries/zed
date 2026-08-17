//! Flowchart-specific rendering helpers.

#[inline]
pub(super) fn contains_ascii_case_insensitive(haystack: &str, needle_lower_ascii: &[u8]) -> bool {
    let h = haystack.as_bytes();
    let n = needle_lower_ascii;
    if n.is_empty() {
        return true;
    }
    if h.len() < n.len() {
        return false;
    }

    for i in 0..=h.len() - n.len() {
        let mut ok = true;
        for j in 0..n.len() {
            let mut b = h[i + j];
            if b.is_ascii_uppercase() {
                b += b'a' - b'A';
            }
            if b != n[j] {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }
    false
}

#[inline]
pub(super) fn flowchart_html_contains_img_tag(text: &str) -> bool {
    contains_ascii_case_insensitive(text, b"<img")
}

pub(super) struct OptionalStyleAttr<'a>(pub(super) &'a str);

impl std::fmt::Display for OptionalStyleAttr<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.trim().is_empty() {
            return Ok(());
        }
        write!(
            f,
            r#" style="{}""#,
            super::super::util::escape_attr_display(self.0)
        )
    }
}

pub(super) struct OptionalStyleXmlAttr<'a>(pub(super) &'a str);

impl std::fmt::Display for OptionalStyleXmlAttr<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.trim();
        if s.is_empty() {
            return Ok(());
        }
        write!(
            f,
            r#" style="{}""#,
            super::super::util::escape_xml_display(s)
        )
    }
}
