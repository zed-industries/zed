use crate::Result;
use std::borrow::Cow;

use crate::svg::pipeline::{SvgPostprocessContext, SvgPostprocessor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CssOverridePolicy {
    #[default]
    Preserve,
    StripExistingImportant,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CssOverridePostprocessor {
    policy: CssOverridePolicy,
}

impl CssOverridePostprocessor {
    pub fn new(policy: CssOverridePolicy) -> Self {
        Self { policy }
    }

    pub fn strip_existing_important() -> Self {
        Self::new(CssOverridePolicy::StripExistingImportant)
    }

    pub fn policy(&self) -> CssOverridePolicy {
        self.policy
    }
}

impl SvgPostprocessor for CssOverridePostprocessor {
    fn name(&self) -> &'static str {
        "css-override"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        _ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>> {
        match self.policy {
            CssOverridePolicy::Preserve => Ok(svg),
            CssOverridePolicy::StripExistingImportant => {
                Ok(Cow::Owned(strip_css_important(svg.as_ref())))
            }
        }
    }
}

pub(crate) fn strip_css_important(svg: &str) -> String {
    let mut out = String::with_capacity(svg.len());
    let mut copied_until = 0usize;
    let mut search_from = 0usize;
    let mut stripped = false;

    while let Some(rel) = svg[search_from..].find('!') {
        let bang = search_from + rel;
        if let Some((start, end)) = css_important_match_bounds_at_bang(svg, bang) {
            out.push_str(&svg[copied_until..start]);
            copied_until = end;
            search_from = end;
            stripped = true;
            continue;
        }

        search_from = bang + 1;
    }

    if !stripped {
        return svg.to_string();
    }

    out.push_str(&svg[copied_until..]);
    out
}

fn css_important_match_bounds_at_bang(svg: &str, bang: usize) -> Option<(usize, usize)> {
    let marker_end = bang + "!important".len();
    if !svg
        .get(bang..marker_end)?
        .eq_ignore_ascii_case("!important")
    {
        return None;
    }

    if let Some(next) = svg.get(marker_end..).and_then(|tail| tail.chars().next()) {
        if is_css_regex_word_char(next) {
            return None;
        }
    }

    let start = svg[..bang]
        .char_indices()
        .rev()
        .find(|(_, ch)| !ch.is_whitespace())
        .map(|(idx, ch)| idx + ch.len_utf8())
        .unwrap_or(0);

    Some((start, marker_end))
}

fn is_css_regex_word_char(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::pipeline::SvgPipeline;

    #[test]
    fn css_override_strips_important_only_when_requested() {
        let svg = r#"<svg><style>.node{fill:red !important;}</style></svg>"#;

        let preserve = SvgPipeline::parity()
            .with_postprocessor(CssOverridePostprocessor::new(CssOverridePolicy::Preserve))
            .process_to_string(svg)
            .unwrap();
        let strip = SvgPipeline::parity()
            .with_postprocessor(CssOverridePostprocessor::strip_existing_important())
            .process_to_string(svg)
            .unwrap();

        assert!(preserve.contains("!important"));
        assert!(!strip.contains("!important"));
    }

    #[test]
    fn css_override_important_scanner_preserves_regex_boundaries() {
        assert_eq!(
            strip_css_important(
                ".a{fill:red\t!important;stroke:blue !IMPORTANT;color:green !importantfoo;}"
            ),
            ".a{fill:red;stroke:blue;color:green !importantfoo;}"
        );
        assert_eq!(
            strip_css_important(".a{fill:red!important-border;color:blue !importanté;}"),
            ".a{fill:red-border;color:blue !importanté;}"
        );
    }
}
