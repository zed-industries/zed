use crate::baseline::BaselineRegistryProfile;
use crate::{MermaidConfig, Result};
use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
#[error("No diagram type detected matching given configuration for text: {text}")]
pub struct DetectTypeError {
    /// Input after front-matter, directives, and Mermaid comments have been removed.
    pub text: String,
}

/// Predicate used by [`DetectorRegistry`] to recognize one Mermaid diagram family.
pub type DetectorFn = fn(text: &str, config: &mut MermaidConfig) -> bool;

/// One diagram detector entry.
#[derive(Debug, Clone)]
pub struct Detector {
    /// Mermaid diagram type id returned when the detector matches.
    pub id: &'static str,
    /// Detection predicate. It may read and update Mermaid config, matching upstream behavior.
    pub detector: DetectorFn,
}

/// Ordered registry that detects Mermaid diagram types.
///
/// Detector order is semantically significant because Mermaid registers overlapping diagram
/// syntaxes in a fixed order.
#[derive(Debug, Clone)]
pub struct DetectorRegistry {
    detectors: Vec<Detector>,
    profile: BaselineRegistryProfile,
}

impl DetectorRegistry {
    /// Creates an empty detector registry.
    pub fn new() -> Self {
        Self::with_profile(BaselineRegistryProfile::Full)
    }

    fn with_profile(profile: BaselineRegistryProfile) -> Self {
        Self {
            detectors: Vec::new(),
            profile,
        }
    }

    /// Adds a detector entry to the end of the ordered registry.
    pub fn add(&mut self, detector: Detector) {
        self.detectors.push(detector);
    }

    /// Adds a detector function to the end of the ordered registry.
    pub fn add_fn(&mut self, id: &'static str, detector: DetectorFn) {
        self.add(Detector { id, detector });
    }

    /// Detects a Mermaid diagram type after stripping front-matter, directives, and comments.
    pub fn detect_type(&self, text: &str, config: &mut MermaidConfig) -> Result<&'static str> {
        let no_frontmatter = remove_frontmatter(text);
        let no_directives = remove_directives(no_frontmatter.as_ref());
        let cleaned = crate::utils::cleanup_mermaid_comments(no_directives.as_ref());

        if let Some(id) =
            crate::family::fast_detect_by_leading_keyword(cleaned.as_ref(), self.profile)
        {
            return Ok(id);
        }

        for det in &self.detectors {
            if (det.detector)(cleaned.as_ref(), config) {
                return Ok(det.id);
            }
        }

        Err(DetectTypeError {
            text: cleaned.into_owned(),
        }
        .into())
    }

    /// Detects a diagram type assuming the input is already pre-cleaned:
    /// no front-matter, no directives, and no Mermaid `%%` comments.
    pub fn detect_type_precleaned(
        &self,
        text: &str,
        config: &mut MermaidConfig,
    ) -> Result<&'static str> {
        if let Some(id) = crate::family::fast_detect_by_leading_keyword(text, self.profile) {
            return Ok(id);
        }

        for det in &self.detectors {
            if (det.detector)(text, config) {
                return Ok(det.id);
            }
        }

        Err(DetectTypeError {
            text: text.to_string(),
        }
        .into())
    }

    /// Builds the full detector registry for the pinned Mermaid baseline.
    ///
    /// This matches Mermaid's `includeLargeFeatures=true` registration profile.
    pub fn pinned_mermaid_baseline_full() -> Self {
        let mut reg = Self::with_profile(BaselineRegistryProfile::Full);
        for fact in crate::family::detector_facts(BaselineRegistryProfile::Full) {
            reg.add_fn(fact.id, fact.detector);
        }

        reg
    }

    /// Builds the small detector registry for the pinned Mermaid baseline.
    ///
    /// This matches the base Mermaid registration profile without large feature diagrams.
    pub fn pinned_mermaid_baseline_tiny() -> Self {
        let mut reg = Self::with_profile(BaselineRegistryProfile::Tiny);
        for fact in crate::family::detector_facts(BaselineRegistryProfile::Tiny) {
            reg.add_fn(fact.id, fact.detector);
        }

        reg
    }

    /// Builds the detector registry selected by this crate's feature flags.
    #[cfg(feature = "large-features")]
    pub fn for_pinned_mermaid_baseline() -> Self {
        Self::pinned_mermaid_baseline_full()
    }

    /// Builds the detector registry selected by this crate's feature flags.
    #[cfg(not(feature = "large-features"))]
    pub fn for_pinned_mermaid_baseline() -> Self {
        Self::pinned_mermaid_baseline_tiny()
    }

    #[cfg(test)]
    pub(crate) fn detector_ids(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.detectors.iter().map(|detector| detector.id)
    }
}

fn remove_frontmatter(text: &str) -> Cow<'_, str> {
    let leading_len = text.len() - text.trim_start().len();
    let trimmed = &text[leading_len..];
    let Some(after_marker) = trimmed.strip_prefix("---") else {
        return Cow::Borrowed(text);
    };
    let Some(open_line_end) = after_marker.find('\n') else {
        return Cow::Borrowed(text);
    };
    if !after_marker[..open_line_end].trim().is_empty() {
        return Cow::Borrowed(text);
    }

    let body_start = leading_len + 3 + open_line_end + 1;
    let rest = &text[body_start..];
    let mut offset = 0usize;

    for line in rest.split_inclusive('\n') {
        let without_newline = line.trim_end_matches(['\r', '\n']);
        if without_newline.trim() == "---" {
            return Cow::Borrowed(&rest[offset + line.len()..]);
        }
        offset += line.len();
    }

    Cow::Borrowed(text)
}

fn remove_directives(text: &str) -> Cow<'_, str> {
    if !text.contains("%%{") {
        return Cow::Borrowed(text);
    }

    let mut out = String::with_capacity(text.len());
    let mut pos = 0;
    while let Some(rel) = text[pos..].find("%%{") {
        let start = pos + rel;
        out.push_str(&text[pos..start]);
        let after_start = start + 3;
        if let Some(rel_end) = text[after_start..].find("}%%") {
            let end = after_start + rel_end + 3;
            pos = end;
        } else {
            return Cow::Owned(out);
        }
    }
    out.push_str(&text[pos..]);
    Cow::Owned(out)
}

impl Default for DetectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn detector_frontmatter_unparsed(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("---")
}

pub(crate) fn detector_error(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim().eq_ignore_ascii_case("error")
}

pub(crate) fn detector_c4(txt: &str, _config: &mut MermaidConfig) -> bool {
    // Matches Mermaid's upstream regex exactly (note the missing grouping in JS).
    txt.trim_start_matches(char::is_whitespace)
        .starts_with("C4Context")
        || txt.contains("C4Container")
        || txt.contains("C4Component")
        || txt.contains("C4Dynamic")
        || txt.contains("C4Deployment")
}

pub(crate) fn detector_kanban(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("kanban")
}

pub(crate) fn detector_class_dagre_d3(txt: &str, config: &mut MermaidConfig) -> bool {
    if config.get_str("class.defaultRenderer") == Some("dagre-wrapper") {
        return false;
    }
    txt.trim_start().starts_with("classDiagram")
}

pub(crate) fn detector_class_v2(txt: &str, config: &mut MermaidConfig) -> bool {
    if txt.trim_start().starts_with("classDiagram")
        && config.get_str("class.defaultRenderer") == Some("dagre-wrapper")
    {
        return true;
    }
    txt.trim_start().starts_with("classDiagram-v2")
}

pub(crate) fn detector_er(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("erDiagram")
}

pub(crate) fn detector_gantt(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("gantt")
}

pub(crate) fn detector_info(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("info")
}

pub(crate) fn detector_pie(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("pie")
}

pub(crate) fn detector_requirement(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("requirement")
}

pub(crate) fn detector_sequence(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("sequenceDiagram")
}

pub(crate) fn detector_flowchart_elk(txt: &str, config: &mut MermaidConfig) -> bool {
    let trimmed = txt.trim_start();
    if trimmed.starts_with("flowchart-elk")
        || ((trimmed.starts_with("flowchart") || trimmed.starts_with("graph"))
            && config.get_str("flowchart.defaultRenderer") == Some("elk"))
    {
        config.set_value("layout", serde_json::Value::String("elk".to_string()));
        return true;
    }
    false
}

pub(crate) fn detector_flowchart_v2(txt: &str, config: &mut MermaidConfig) -> bool {
    if config.get_str("flowchart.defaultRenderer") == Some("dagre-d3") {
        return false;
    }
    if config.get_str("flowchart.defaultRenderer") == Some("elk") {
        config.set_value("layout", serde_json::Value::String("elk".to_string()));
    }

    if txt.trim_start().starts_with("graph")
        && config.get_str("flowchart.defaultRenderer") == Some("dagre-wrapper")
    {
        return true;
    }
    txt.trim_start().starts_with("flowchart")
}

pub(crate) fn detector_flowchart_dagre_d3_graph(txt: &str, config: &mut MermaidConfig) -> bool {
    if matches!(
        config.get_str("flowchart.defaultRenderer"),
        Some("dagre-wrapper" | "elk")
    ) {
        return false;
    }
    txt.trim_start().starts_with("graph")
}

pub(crate) fn detector_timeline(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("timeline")
}

pub(crate) fn detector_git_graph(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("gitGraph")
}

pub(crate) fn detector_state_dagre_d3(txt: &str, config: &mut MermaidConfig) -> bool {
    if config.get_str("state.defaultRenderer") == Some("dagre-wrapper") {
        return false;
    }
    txt.trim_start().starts_with("stateDiagram")
}

pub(crate) fn detector_state_v2(txt: &str, config: &mut MermaidConfig) -> bool {
    let trimmed = txt.trim_start();
    if trimmed.starts_with("stateDiagram-v2") {
        return true;
    }
    trimmed.starts_with("stateDiagram")
        && config.get_str("state.defaultRenderer") == Some("dagre-wrapper")
}

pub(crate) fn detector_journey(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("journey")
}

pub(crate) fn detector_quadrant(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("quadrantChart")
}

pub(crate) fn detector_sankey(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("sankey")
}

pub(crate) fn detector_packet(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("packet")
}

pub(crate) fn detector_xychart(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("xychart")
}

pub(crate) fn detector_block(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("block")
}

pub(crate) fn detector_tree_view(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("treeView-beta")
}

pub(crate) fn detector_ishikawa(txt: &str, _config: &mut MermaidConfig) -> bool {
    let t = txt.trim_start();
    starts_with_header_case_insensitive(t, "ishikawa-beta")
        || starts_with_header_case_insensitive(t, "ishikawa")
}

pub(crate) fn detector_eventmodeling(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("eventmodeling")
}

fn starts_with_header_case_insensitive(text: &str, header: &str) -> bool {
    let Some(actual) = text.get(..header.len()) else {
        return false;
    };
    if !actual.eq_ignore_ascii_case(header) {
        return false;
    }
    text[header.len()..]
        .chars()
        .next()
        .map_or(true, |c| c.is_whitespace() || c == ';')
}

pub(crate) fn detector_radar(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("radar-beta")
}

pub(crate) fn detector_treemap(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("treemap")
}

pub(crate) fn detector_venn(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("venn-beta")
}

pub(crate) fn detector_mindmap(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("mindmap")
}

pub(crate) fn detector_architecture(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("architecture")
}

pub(crate) fn detector_zenuml(txt: &str, _config: &mut MermaidConfig) -> bool {
    txt.trim_start().starts_with("zenuml")
}

#[cfg(test)]
mod remove_directives_tests {
    use super::remove_directives;
    use std::borrow::Cow;

    #[test]
    fn no_directives_is_borrowed() {
        let s = "flowchart TD; A-->B;";
        assert!(matches!(remove_directives(s), Cow::Borrowed(_)));
    }

    #[test]
    fn removes_directive_block() {
        let s = "%%{init: {\"theme\": \"dark\"}}%%\nflowchart TD; A-->B;";
        let out = remove_directives(s);
        assert!(out.as_ref().contains("flowchart TD"));
        assert!(!out.as_ref().contains("init"));
    }

    #[test]
    fn unterminated_directive_truncates_at_start() {
        let s = "flowchart\n%%{init: {\"theme\": \"dark\"}}\nA-->B;";
        let out = remove_directives(s);
        assert_eq!(out.as_ref().trim(), "flowchart");
    }
}
