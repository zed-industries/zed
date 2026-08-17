pub(super) fn escape_xml_ampersands_preserving_xml_entities(
    raw: &str,
) -> std::borrow::Cow<'_, str> {
    fn is_xml_predefined_entity(entity: &str) -> bool {
        matches!(entity, "amp" | "lt" | "gt" | "quot" | "apos")
    }

    fn is_xml_numeric_entity(entity: &str) -> bool {
        if let Some(hex) = entity
            .strip_prefix("#x")
            .or_else(|| entity.strip_prefix("#X"))
        {
            return !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit());
        }
        if let Some(dec) = entity.strip_prefix('#') {
            return !dec.is_empty() && dec.chars().all(|c| c.is_ascii_digit());
        }
        false
    }

    if !raw.as_bytes().contains(&b'&') {
        return std::borrow::Cow::Borrowed(raw);
    }

    let mut out = String::with_capacity(raw.len());
    let mut i = 0usize;
    while let Some(rel) = raw[i..].find('&') {
        let amp = i + rel;
        out.push_str(&raw[i..amp]);

        let tail = &raw[amp + 1..];
        if let Some(semi_rel) = tail.find(';') {
            let semi = amp + 1 + semi_rel;
            let entity = &raw[amp + 1..semi];
            if is_xml_predefined_entity(entity) || is_xml_numeric_entity(entity) {
                out.push_str(&raw[amp..=semi]);
                i = semi + 1;
                continue;
            }
        }

        out.push_str("&amp;");
        i = amp + 1;
    }
    out.push_str(&raw[i..]);
    std::borrow::Cow::Owned(out)
}

#[derive(Debug, Clone)]
enum ForeignObjectFragmentNode {
    Text(String),
    Element(ForeignObjectFragmentElement),
    RawTag(String),
}

#[derive(Debug, Clone)]
struct ForeignObjectFragmentElement {
    raw_open: String,
    raw_close: Option<String>,
    name_lc: String,
    children: Vec<ForeignObjectFragmentNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ForeignObjectNamespace {
    Svg,
    Html,
}

fn is_foreign_object_void_tag(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn is_svg_tag_for_foreign_object(name: &str) -> bool {
    matches!(
        name,
        "a" | "altglyph"
            | "altglyphdef"
            | "altglyphitem"
            | "animate"
            | "animatecolor"
            | "animatemotion"
            | "animatetransform"
            | "circle"
            | "clippath"
            | "defs"
            | "desc"
            | "ellipse"
            | "feblend"
            | "fecolormatrix"
            | "fecomponenttransfer"
            | "fecomposite"
            | "feconvolvematrix"
            | "fediffuselighting"
            | "fedisplacementmap"
            | "fedistantlight"
            | "fedropshadow"
            | "feflood"
            | "fefunca"
            | "fefuncb"
            | "fefuncg"
            | "fefuncr"
            | "fegaussianblur"
            | "feimage"
            | "femerge"
            | "femergenode"
            | "femorphology"
            | "feoffset"
            | "fepointlight"
            | "fespecularlighting"
            | "fespotlight"
            | "fetile"
            | "feturbulence"
            | "filter"
            | "font"
            | "foreignobject"
            | "g"
            | "glyph"
            | "glyphref"
            | "hkern"
            | "image"
            | "line"
            | "lineargradient"
            | "marker"
            | "mask"
            | "metadata"
            | "mpath"
            | "path"
            | "pattern"
            | "polygon"
            | "polyline"
            | "radialgradient"
            | "rect"
            | "set"
            | "stop"
            | "svg"
            | "switch"
            | "symbol"
            | "text"
            | "textpath"
            | "title"
            | "tref"
            | "tspan"
            | "use"
            | "view"
    )
}

fn is_svg_html_integration_point(name: &str) -> bool {
    matches!(name, "foreignobject")
}

fn classify_foreign_object_element_namespace(
    parent_ns: ForeignObjectNamespace,
    name_lc: &str,
) -> ForeignObjectNamespace {
    match parent_ns {
        ForeignObjectNamespace::Html => {
            if name_lc == "svg" {
                ForeignObjectNamespace::Svg
            } else {
                ForeignObjectNamespace::Html
            }
        }
        ForeignObjectNamespace::Svg => {
            if is_svg_tag_for_foreign_object(name_lc) {
                ForeignObjectNamespace::Svg
            } else {
                ForeignObjectNamespace::Html
            }
        }
    }
}

fn child_namespace_for_foreign_object_element(
    element_ns: ForeignObjectNamespace,
    name_lc: &str,
) -> ForeignObjectNamespace {
    match element_ns {
        ForeignObjectNamespace::Html => ForeignObjectNamespace::Html,
        ForeignObjectNamespace::Svg => {
            if is_svg_html_integration_point(name_lc) {
                ForeignObjectNamespace::Html
            } else {
                ForeignObjectNamespace::Svg
            }
        }
    }
}

fn parse_foreign_object_fragment(raw: &str) -> Vec<ForeignObjectFragmentNode> {
    fn push_node(
        stack: &mut [ForeignObjectFragmentElement],
        roots: &mut Vec<ForeignObjectFragmentNode>,
        node: ForeignObjectFragmentNode,
    ) {
        if let Some(parent) = stack.last_mut() {
            parent.children.push(node);
        } else {
            roots.push(node);
        }
    }

    fn tag_name_from_inner(inner: &str) -> Option<String> {
        let mut j = 0usize;
        let bytes = inner.as_bytes();
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        let start = j;
        while j < bytes.len() {
            let c = bytes[j] as char;
            if c.is_ascii_whitespace() || c == '/' {
                break;
            }
            j += 1;
        }
        (start < j).then(|| inner[start..j].to_ascii_lowercase())
    }

    let mut roots = Vec::new();
    let mut stack: Vec<ForeignObjectFragmentElement> = Vec::new();
    let mut cursor = 0usize;

    while cursor < raw.len() {
        let Some(lt_rel) = raw[cursor..].find('<') else {
            if cursor < raw.len() {
                push_node(
                    &mut stack,
                    &mut roots,
                    ForeignObjectFragmentNode::Text(raw[cursor..].to_string()),
                );
            }
            break;
        };

        let lt = cursor + lt_rel;
        if lt > cursor {
            push_node(
                &mut stack,
                &mut roots,
                ForeignObjectFragmentNode::Text(raw[cursor..lt].to_string()),
            );
        }

        let Some(gt_rel) = raw[lt..].find('>') else {
            push_node(
                &mut stack,
                &mut roots,
                ForeignObjectFragmentNode::Text(raw[lt..].to_string()),
            );
            break;
        };

        let gt = lt + gt_rel;
        let raw_tag = raw[lt..=gt].to_string();
        let inner = raw[lt + 1..gt].trim();

        if inner.is_empty() {
            push_node(
                &mut stack,
                &mut roots,
                ForeignObjectFragmentNode::RawTag(raw_tag),
            );
            cursor = gt + 1;
            continue;
        }

        match inner.as_bytes()[0] as char {
            '!' | '?' => {
                push_node(
                    &mut stack,
                    &mut roots,
                    ForeignObjectFragmentNode::RawTag(raw_tag),
                );
            }
            '/' => {
                let Some(name_lc) = tag_name_from_inner(&inner[1..]) else {
                    push_node(
                        &mut stack,
                        &mut roots,
                        ForeignObjectFragmentNode::RawTag(raw_tag),
                    );
                    cursor = gt + 1;
                    continue;
                };

                if let Some(pos) = stack.iter().rposition(|el| el.name_lc == name_lc) {
                    let mut orphaned = stack.split_off(pos + 1);
                    for mut orphan in orphaned.drain(..) {
                        if orphan.raw_close.is_none() {
                            orphan.raw_close = Some(format!("</{}>", orphan.name_lc));
                        }
                        push_node(
                            &mut stack,
                            &mut roots,
                            ForeignObjectFragmentNode::Element(orphan),
                        );
                    }
                    if let Some(mut element) = stack.pop() {
                        element.raw_close = Some(raw_tag);
                        push_node(
                            &mut stack,
                            &mut roots,
                            ForeignObjectFragmentNode::Element(element),
                        );
                    } else {
                        push_node(
                            &mut stack,
                            &mut roots,
                            ForeignObjectFragmentNode::RawTag(raw_tag),
                        );
                    }
                } else {
                    push_node(
                        &mut stack,
                        &mut roots,
                        ForeignObjectFragmentNode::RawTag(raw_tag),
                    );
                }
            }
            _ => {
                let Some(name_lc) = tag_name_from_inner(inner) else {
                    push_node(
                        &mut stack,
                        &mut roots,
                        ForeignObjectFragmentNode::RawTag(raw_tag),
                    );
                    cursor = gt + 1;
                    continue;
                };
                let self_closed = inner.ends_with('/') || is_foreign_object_void_tag(&name_lc);
                let element = ForeignObjectFragmentElement {
                    raw_open: raw_tag,
                    raw_close: None,
                    name_lc,
                    children: Vec::new(),
                };
                if self_closed {
                    push_node(
                        &mut stack,
                        &mut roots,
                        ForeignObjectFragmentNode::Element(element),
                    );
                } else {
                    stack.push(element);
                }
            }
        }

        cursor = gt + 1;
    }

    while let Some(mut element) = stack.pop() {
        if element.raw_close.is_none() {
            element.raw_close = Some(format!("</{}>", element.name_lc));
        }
        push_node(
            &mut stack,
            &mut roots,
            ForeignObjectFragmentNode::Element(element),
        );
    }

    roots
}

fn serialize_foreign_object_fragment(nodes: Vec<ForeignObjectFragmentNode>) -> String {
    enum Frame {
        Node(ForeignObjectFragmentNode),
        Close(String),
    }

    let mut out = String::new();
    let mut stack: Vec<Frame> = nodes.into_iter().rev().map(Frame::Node).collect();

    while let Some(frame) = stack.pop() {
        match frame {
            Frame::Node(ForeignObjectFragmentNode::Text(text))
            | Frame::Node(ForeignObjectFragmentNode::RawTag(text)) => out.push_str(&text),
            Frame::Node(ForeignObjectFragmentNode::Element(mut element)) => {
                out.push_str(&element.raw_open);
                if let Some(raw_close) = element.raw_close.take() {
                    stack.push(Frame::Close(raw_close));
                }
                let children = std::mem::take(&mut element.children);
                stack.extend(children.into_iter().rev().map(Frame::Node));
            }
            Frame::Close(raw_close) => out.push_str(&raw_close),
        }
    }

    out
}

fn node_allowed_in_svg_content(
    node: &ForeignObjectFragmentNode,
    child_ns: ForeignObjectNamespace,
) -> bool {
    match node {
        ForeignObjectFragmentNode::Text(_) | ForeignObjectFragmentNode::RawTag(_) => true,
        ForeignObjectFragmentNode::Element(element) => {
            classify_foreign_object_element_namespace(child_ns, &element.name_lc)
                == ForeignObjectNamespace::Svg
        }
    }
}

fn rewrite_foreign_object_fragment_nodes(
    nodes: Vec<ForeignObjectFragmentNode>,
    parent_ns: ForeignObjectNamespace,
) -> Vec<ForeignObjectFragmentNode> {
    struct PendingElement {
        element: ForeignObjectFragmentElement,
        element_ns: ForeignObjectNamespace,
        child_ns: ForeignObjectNamespace,
    }

    struct Frame {
        parent_ns: ForeignObjectNamespace,
        iter: std::vec::IntoIter<ForeignObjectFragmentNode>,
        out: Vec<ForeignObjectFragmentNode>,
        pending: Option<PendingElement>,
    }

    let mut stack = vec![Frame {
        parent_ns,
        iter: nodes.into_iter(),
        out: Vec::new(),
        pending: None,
    }];

    loop {
        let Some(frame) = stack.last_mut() else {
            return Vec::new();
        };

        if let Some(node) = frame.iter.next() {
            match node {
                ForeignObjectFragmentNode::Text(_) | ForeignObjectFragmentNode::RawTag(_) => {
                    frame.out.push(node);
                }
                ForeignObjectFragmentNode::Element(mut element) => {
                    let element_ns = classify_foreign_object_element_namespace(
                        frame.parent_ns,
                        &element.name_lc,
                    );
                    let child_ns =
                        child_namespace_for_foreign_object_element(element_ns, &element.name_lc);
                    let children = std::mem::take(&mut element.children);
                    stack.push(Frame {
                        parent_ns: child_ns,
                        iter: children.into_iter(),
                        out: Vec::new(),
                        pending: Some(PendingElement {
                            element,
                            element_ns,
                            child_ns,
                        }),
                    });
                }
            }
            continue;
        }

        let Some(frame) = stack.pop() else {
            return Vec::new();
        };
        let Some(pending) = frame.pending else {
            return frame.out;
        };

        let Some(parent) = stack.last_mut() else {
            return frame.out;
        };

        let mut element = pending.element;
        element.children = frame.out;

        if pending.element_ns == ForeignObjectNamespace::Svg
            && !is_svg_html_integration_point(&element.name_lc)
        {
            let mut kept = Vec::new();
            let mut moved = Vec::new();
            let mut keep_prefix = true;

            for child in element.children {
                if keep_prefix && node_allowed_in_svg_content(&child, pending.child_ns) {
                    kept.push(child);
                } else {
                    keep_prefix = false;
                    moved.push(child);
                }
            }

            element.children = kept;
            parent.out.push(ForeignObjectFragmentNode::Element(element));
            parent.out.extend(moved);
        } else {
            parent.out.push(ForeignObjectFragmentNode::Element(element));
        }
    }
}

fn normalize_raw_xhtml_fragment_for_foreign_object(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len() + 16);
    let mut i = 0usize;
    let bytes = raw.as_bytes();
    while i < bytes.len() {
        let Some(lt_rel) = raw[i..].find('<') else {
            out.push_str(&raw[i..]);
            break;
        };
        let lt = i + lt_rel;
        out.push_str(&raw[i..lt]);
        let Some(gt_rel) = raw[lt..].find('>') else {
            out.push_str(&raw[lt..]);
            break;
        };
        let gt = lt + gt_rel;
        let inner = raw[lt + 1..gt].trim();

        if inner.is_empty() {
            out.push_str("<>");
            i = gt + 1;
            continue;
        }

        let first = inner.as_bytes()[0] as char;
        if matches!(first, '/' | '!' | '?') {
            out.push('<');
            out.push_str(inner);
            out.push('>');
            i = gt + 1;
            continue;
        }

        let mut j = 0usize;
        let inner_bytes = inner.as_bytes();
        while j < inner_bytes.len() && inner_bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        let name_start = j;
        while j < inner_bytes.len() {
            let c = inner_bytes[j] as char;
            if c.is_ascii_whitespace() || c == '/' {
                break;
            }
            j += 1;
        }
        let tag_name = inner[name_start..j].trim();
        if tag_name.is_empty() {
            out.push('<');
            out.push_str(inner);
            out.push('>');
            i = gt + 1;
            continue;
        }
        let tag_name_lc = tag_name.to_ascii_lowercase();

        let mut rest = inner[j..].trim();
        let mut self_close = false;
        if rest.ends_with('/') {
            self_close = true;
            rest = rest[..rest.len().saturating_sub(1)].trim_end();
        }

        out.push('<');
        out.push_str(tag_name);

        let mut k = 0usize;
        let rest_bytes = rest.as_bytes();
        while k < rest_bytes.len() {
            while k < rest_bytes.len() && rest_bytes[k].is_ascii_whitespace() {
                k += 1;
            }
            if k >= rest_bytes.len() {
                break;
            }

            let attr_start = k;
            while k < rest_bytes.len() {
                let c = rest_bytes[k] as char;
                if c.is_ascii_whitespace() || c == '=' {
                    break;
                }
                k += 1;
            }
            let attr_name = rest[attr_start..k].trim();
            if attr_name.is_empty() {
                break;
            }
            while k < rest_bytes.len() && rest_bytes[k].is_ascii_whitespace() {
                k += 1;
            }

            if k < rest_bytes.len() && rest_bytes[k] as char == '=' {
                k += 1;
                while k < rest_bytes.len() && rest_bytes[k].is_ascii_whitespace() {
                    k += 1;
                }
                if k >= rest_bytes.len() {
                    out.push(' ');
                    out.push_str(attr_name);
                    out.push_str("=\"\"");
                    break;
                }
                let q = rest_bytes[k] as char;
                if q == '"' || q == '\'' {
                    let quote = q;
                    k += 1;
                    let val_start = k;
                    while k < rest_bytes.len() && rest_bytes[k] as char != quote {
                        k += 1;
                    }
                    let val = &rest[val_start..k];
                    if k < rest_bytes.len() {
                        k += 1;
                    }
                    out.push(' ');
                    out.push_str(attr_name);
                    out.push_str("=\"");
                    out.push_str(val);
                    out.push('"');
                } else {
                    let val_start = k;
                    while k < rest_bytes.len() {
                        let c = rest_bytes[k] as char;
                        if c.is_ascii_whitespace() {
                            break;
                        }
                        k += 1;
                    }
                    let val = &rest[val_start..k];
                    out.push(' ');
                    out.push_str(attr_name);
                    out.push_str("=\"");
                    out.push_str(val);
                    out.push('"');
                }
            } else {
                out.push(' ');
                out.push_str(attr_name);
                out.push_str("=\"");
                out.push_str(attr_name);
                out.push('"');
            }
        }

        if is_foreign_object_void_tag(tag_name_lc.as_str()) || self_close {
            out.push_str(" />");
        } else {
            out.push('>');
        }
        i = gt + 1;
    }
    out
}

pub(super) fn normalize_xhtml_fragment_for_foreign_object(raw: &str) -> String {
    let parsed = parse_foreign_object_fragment(raw);
    let rewritten = rewrite_foreign_object_fragment_nodes(parsed, ForeignObjectNamespace::Svg);
    let rewritten = serialize_foreign_object_fragment(rewritten);
    normalize_raw_xhtml_fragment_for_foreign_object(&rewritten)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_xhtml_fragment_splits_root_svg_anchor_from_html_children() {
        assert_eq!(
            normalize_xhtml_fragment_for_foreign_object(
                r#"<a href='https://example.com'><code>code</code></a>"#,
            ),
            r#"<a href="https://example.com"></a><code>code</code>"#,
        );
    }

    #[test]
    fn normalize_xhtml_fragment_preserves_anchor_inside_html_context() {
        assert_eq!(
            normalize_xhtml_fragment_for_foreign_object(
                r#"<p><a href='https://example.com'><code>code</code></a></p>"#,
            ),
            r#"<p><a href="https://example.com"><code>code</code></a></p>"#,
        );
    }

    #[test]
    fn normalize_xhtml_fragment_splits_svg_content_before_html_children() {
        assert_eq!(
            normalize_xhtml_fragment_for_foreign_object(r#"<g>x<b>y</b>z</g>"#),
            r#"<g>x</g><b>y</b>z"#,
        );
    }

    #[test]
    fn normalize_xhtml_fragment_handles_deep_nested_html_with_small_stack() {
        const DEPTH: usize = 2_048;
        let handle = std::thread::Builder::new()
            .name("architecture-deep-xhtml-fragment".to_string())
            .stack_size(64 * 1024)
            .spawn(|| {
                let mut raw = String::new();
                for _ in 0..DEPTH {
                    raw.push_str("<span>");
                }
                raw.push_str("Icon");
                for _ in 0..DEPTH {
                    raw.push_str("</span>");
                }
                normalize_xhtml_fragment_for_foreign_object(&raw)
            })
            .expect("spawn deep XHTML fragment test");

        let normalized = handle
            .join()
            .expect("deep XHTML fragment normalization should not overflow");
        assert!(normalized.contains("Icon"));
    }
}
