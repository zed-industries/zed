use std::sync::Arc;

use util::paths::{PathMatcher, PathStyle};
use util::rel_path::RelPath;

#[derive(Debug, Clone, PartialEq)]
enum Segment {
    Literal(Arc<str>),
    Filename,
    Extname,
    Dirname(i32),
}

#[derive(Debug, Clone)]
pub struct LabelTemplate {
    segments: Vec<Segment>,
}

impl LabelTemplate {
    pub fn parse(template: &str) -> Self {
        let mut segments = Vec::new();
        let mut literal = String::new();
        let mut remaining = template;
        while !remaining.is_empty() {
            if let Some(rest) = remaining.strip_prefix("${")
                && let Some(end) = rest.find('}')
                && let Some(variable) = parse_variable(&rest[..end])
            {
                if !literal.is_empty() {
                    segments.push(Segment::Literal(Arc::from(literal.as_str())));
                    literal.clear();
                }
                segments.push(variable);
                remaining = &rest[end + 1..];
                continue;
            }
            let mut chars = remaining.chars();
            if let Some(ch) = chars.next() {
                literal.push(ch);
            }
            remaining = chars.as_str();
        }
        if !literal.is_empty() {
            segments.push(Segment::Literal(Arc::from(literal.as_str())));
        }
        Self { segments }
    }

    pub fn render(&self, path: &RelPath) -> Option<String> {
        let mut output = String::new();
        for segment in &self.segments {
            match segment {
                Segment::Literal(text) => output.push_str(text),
                Segment::Filename => {
                    output.push_str(path.file_stem()?);
                }
                Segment::Extname => {
                    output.push_str(path.extension()?);
                }
                Segment::Dirname(n) => {
                    output.push_str(dirname_nth(path, *n)?);
                }
            }
        }
        Some(output)
    }
}

fn parse_variable(inner: &str) -> Option<Segment> {
    let (name, arg) = if let Some(paren) = inner.find('(') {
        if !inner.ends_with(')') {
            return None;
        }
        let arg = &inner[paren + 1..inner.len() - 1];
        let n = arg.trim().parse::<i32>().ok()?;
        (&inner[..paren], Some(n))
    } else {
        (inner, None)
    };
    match (name.trim(), arg) {
        ("filename", None) => Some(Segment::Filename),
        ("extname", None) => Some(Segment::Extname),
        ("dirname", None) => Some(Segment::Dirname(1)),
        ("dirname", Some(n)) => Some(Segment::Dirname(n)),
        _ => None,
    }
}

fn dirname_nth(path: &RelPath, n: i32) -> Option<&str> {
    let parent = path.parent()?;
    let components: Vec<&str> = parent.components().collect();
    if components.is_empty() {
        return None;
    }
    let idx = if n > 0 {
        components.len().checked_sub(n as usize)?
    } else if n < 0 {
        let from_root = (-n) as usize;
        from_root.checked_sub(1)?
    } else {
        components.len() - 1
    };
    components.get(idx).copied()
}

#[derive(Debug, Clone)]
struct CustomLabel {
    matcher: PathMatcher,
    template: LabelTemplate,
}

#[derive(Debug, Clone, Default)]
pub struct CustomLabels {
    labels: Vec<CustomLabel>,
}

impl CustomLabels {
    pub fn from_patterns<I, S>(patterns: I) -> Self
    where
        I: IntoIterator<Item = (S, S)>,
        S: AsRef<str>,
    {
        let labels = patterns
            .into_iter()
            .filter_map(|(pattern, template)| {
                let matcher =
                    PathMatcher::new([pattern.as_ref()], PathStyle::Posix).ok()?;
                Some(CustomLabel {
                    matcher,
                    template: LabelTemplate::parse(template.as_ref()),
                })
            })
            .collect();
        Self { labels }
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    pub fn label_for(&self, path: &RelPath) -> Option<String> {
        for label in &self.labels {
            if label.matcher.is_match(path)
                && let Some(rendered) = label.template.render(path)
            {
                return Some(rendered);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::rel_path::RelPath;

    fn rel(path: &str) -> Arc<RelPath> {
        RelPath::unix(path).unwrap().into_arc()
    }

    #[test]
    fn parses_literal_template() {
        let t = LabelTemplate::parse("hello world");
        assert_eq!(t.render(&rel("a/b.txt")).as_deref(), Some("hello world"));
    }

    #[test]
    fn renders_filename_and_extname() {
        let t = LabelTemplate::parse("${filename}.${extname}");
        assert_eq!(t.render(&rel("a/b/page.tsx")).as_deref(), Some("page.tsx"));
    }

    #[test]
    fn renders_dirname_variants() {
        let path = rel("src/routes/blog/+page.svelte");
        assert_eq!(
            LabelTemplate::parse("${dirname}").render(&path).as_deref(),
            Some("blog")
        );
        assert_eq!(
            LabelTemplate::parse("${dirname(1)}")
                .render(&path)
                .as_deref(),
            Some("blog")
        );
        assert_eq!(
            LabelTemplate::parse("${dirname(2)}")
                .render(&path)
                .as_deref(),
            Some("routes")
        );
        assert_eq!(
            LabelTemplate::parse("${dirname(3)}")
                .render(&path)
                .as_deref(),
            Some("src")
        );
        assert_eq!(
            LabelTemplate::parse("${dirname(-1)}")
                .render(&path)
                .as_deref(),
            Some("src")
        );
        assert_eq!(
            LabelTemplate::parse("${dirname(-2)}")
                .render(&path)
                .as_deref(),
            Some("routes")
        );
        assert_eq!(
            LabelTemplate::parse("${dirname(-3)}")
                .render(&path)
                .as_deref(),
            Some("blog")
        );
    }

    #[test]
    fn unknown_placeholder_left_as_literal() {
        let t = LabelTemplate::parse("[${unknown}] ${filename}");
        assert_eq!(
            t.render(&rel("a/b/page.tsx")).as_deref(),
            Some("[${unknown}] page")
        );
    }

    #[test]
    fn dirname_out_of_range_drops_label() {
        let path = rel("a/b.txt");
        assert!(
            LabelTemplate::parse("${dirname(5)}")
                .render(&path)
                .is_none()
        );
    }

    #[test]
    fn label_for_picks_first_matching_pattern() {
        let labels = CustomLabels::from_patterns([
            (
                "**/src/routes/**/+page.svelte",
                "/${dirname} - Page",
            ),
            ("**/*.svelte", "${filename} (svelte)"),
        ]);
        assert_eq!(
            labels
                .label_for(&rel("src/routes/blog/+page.svelte"))
                .as_deref(),
            Some("/blog - Page")
        );
        assert_eq!(
            labels.label_for(&rel("lib/widget.svelte")).as_deref(),
            Some("widget (svelte)")
        );
    }
}
