// Copyright 2021 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::comparison_chain)]

use roxmltree::Error;

use super::{AId, Document, EId, NodeId, NodeKind, SvgNode};

const XLINK_NS: &str = "http://www.w3.org/1999/xlink";

pub(crate) fn parse_svg_text_element<'input>(
    parent: roxmltree::Node<'_, 'input>,
    parent_id: NodeId,
    style_sheet: &simplecss::StyleSheet,
    doc: &mut Document<'input>,
) -> Result<(), Error> {
    debug_assert_eq!(parent.tag_name().name(), "text");

    let space = if doc.get(parent_id).has_attribute(AId::Space) {
        get_xmlspace(doc, parent_id, XmlSpace::Default)
    } else {
        if let Some(node) = doc
            .get(parent_id)
            .ancestors()
            .find(|n| n.has_attribute(AId::Space))
        {
            get_xmlspace(doc, node.id, XmlSpace::Default)
        } else {
            XmlSpace::Default
        }
    };

    parse_svg_text_element_impl(parent, parent_id, style_sheet, space, doc)?;

    trim_text_nodes(parent_id, space, doc);
    Ok(())
}

fn parse_svg_text_element_impl<'input>(
    parent: roxmltree::Node<'_, 'input>,
    parent_id: NodeId,
    style_sheet: &simplecss::StyleSheet,
    space: XmlSpace,
    doc: &mut Document<'input>,
) -> Result<(), Error> {
    for node in parent.children() {
        if node.is_text() {
            let text = trim_text(node.text().unwrap(), space);
            doc.append(parent_id, NodeKind::Text(text));
            continue;
        }

        let mut tag_name = match super::parse::parse_tag_name(node) {
            Some(v) => v,
            None => continue,
        };

        if tag_name == EId::A {
            // Treat links as simple text.
            tag_name = EId::Tspan;
        }

        if !matches!(tag_name, EId::Tspan | EId::Tref | EId::TextPath) {
            continue;
        }

        // `textPath` must be a direct `text` child.
        if tag_name == EId::TextPath && parent.tag_name().name() != "text" {
            continue;
        }

        // We are converting `tref` into `tspan` to simplify later use.
        let mut is_tref = false;
        if tag_name == EId::Tref {
            tag_name = EId::Tspan;
            is_tref = true;
        }

        let node_id =
            super::parse::parse_svg_element(node, parent_id, tag_name, style_sheet, false, doc)?;
        let space = get_xmlspace(doc, node_id, space);

        if is_tref {
            let link_value = node
                .attribute((XLINK_NS, "href"))
                .or_else(|| node.attribute("href"));

            if let Some(href) = link_value {
                if let Some(text) = resolve_tref_text(node.document(), href) {
                    let text = trim_text(&text, space);
                    doc.append(node_id, NodeKind::Text(text));
                }
            }
        } else {
            parse_svg_text_element_impl(node, node_id, style_sheet, space, doc)?;
        }
    }

    Ok(())
}

fn resolve_tref_text(xml: &roxmltree::Document, href: &str) -> Option<String> {
    let id = svgtypes::IRI::from_str(href).ok()?.0;

    // Find linked element in the original tree.
    let node = xml.descendants().find(|n| n.attribute("id") == Some(id))?;

    // `tref` should be linked to an SVG element.
    super::parse::parse_tag_name(node)?;

    // 'All character data within the referenced element, including character data enclosed
    // within additional markup, will be rendered.'
    //
    // So we don't care about attributes and everything. Just collecting text nodes data.
    //
    // Note: we have to filter nodes by `is_text()` first since `text()` will look up
    // for text nodes in element children therefore we will get duplicates.
    let text: String = node
        .descendants()
        .filter(|n| n.is_text())
        .filter_map(|n| n.text())
        .collect();
    if text.is_empty() { None } else { Some(text) }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum XmlSpace {
    Default,
    Preserve,
}

fn get_xmlspace(doc: &Document, node_id: NodeId, default: XmlSpace) -> XmlSpace {
    match doc.get(node_id).attribute(AId::Space) {
        Some("preserve") => XmlSpace::Preserve,
        Some(_) => XmlSpace::Default,
        _ => default,
    }
}

trait StrTrim {
    fn remove_first_space(&mut self);
    fn remove_last_space(&mut self);
}

impl StrTrim for String {
    fn remove_first_space(&mut self) {
        debug_assert_eq!(self.chars().next().unwrap(), ' ');
        self.drain(0..1);
    }

    fn remove_last_space(&mut self) {
        debug_assert_eq!(self.chars().next_back().unwrap(), ' ');
        self.pop();
    }
}

/// Prepares text nodes according to the spec: https://www.w3.org/TR/SVG11/text.html#WhiteSpace
///
/// This function handles:
/// - 'xml:space' processing
/// - tabs and newlines removing/replacing
/// - spaces trimming
fn trim_text_nodes(text_elem_id: NodeId, xmlspace: XmlSpace, doc: &mut Document) {
    let mut nodes = Vec::new(); // TODO: allocate only once
    collect_text_nodes(doc.get(text_elem_id), 0, &mut nodes);

    // `trim` method has already collapsed all spaces into a single one,
    // so we have to check only for one leading or trailing space.

    if nodes.len() == 1 {
        // Process element with a single text node child.

        let node_id = nodes[0].0;

        if xmlspace == XmlSpace::Default {
            if let NodeKind::Text(ref mut text) = doc.nodes[node_id.get_usize()].kind {
                match text.len() {
                    0 => {} // An empty string. Do nothing.
                    1 => {
                        // If string has only one character and it's a space - clear this string.
                        if text.as_bytes()[0] == b' ' {
                            text.clear();
                        }
                    }
                    _ => {
                        // 'text' has at least 2 bytes, so indexing is safe.
                        let c1 = text.as_bytes()[0];
                        let c2 = text.as_bytes()[text.len() - 1];

                        if c1 == b' ' {
                            text.remove_first_space();
                        }

                        if c2 == b' ' {
                            text.remove_last_space();
                        }
                    }
                }
            }
        } else {
            // Do nothing when xml:space=preserve.
        }
    } else if nodes.len() > 1 {
        // Process element with many text node children.

        // We manage all text nodes as a single text node
        // and trying to remove duplicated spaces across nodes.
        //
        // For example    '<text>Text <tspan> text </tspan> text</text>'
        // is the same is '<text>Text <tspan>text</tspan> text</text>'

        let mut i = 0;
        let len = nodes.len() - 1;
        let mut last_non_empty: Option<NodeId> = None;
        while i < len {
            // Process pairs.
            let (mut node1_id, depth1) = nodes[i];
            let (node2_id, depth2) = nodes[i + 1];

            if doc.get(node1_id).text().is_empty() {
                if let Some(n) = last_non_empty {
                    node1_id = n;
                }
            }

            // Parent of the text node is always an element node and always exist,
            // so unwrap is safe.
            let xmlspace1 = get_xmlspace(doc, doc.get(node1_id).parent().unwrap().id, xmlspace);
            let xmlspace2 = get_xmlspace(doc, doc.get(node2_id).parent().unwrap().id, xmlspace);

            // >text<..>text<
            //  1  2    3  4
            let (c1, c2, c3, c4) = {
                let text1 = doc.get(node1_id).text();
                let text2 = doc.get(node2_id).text();

                let bytes1 = text1.as_bytes();
                let bytes2 = text2.as_bytes();

                let c1 = bytes1.first().cloned();
                let c2 = bytes1.last().cloned();
                let c3 = bytes2.first().cloned();
                let c4 = bytes2.last().cloned();

                (c1, c2, c3, c4)
            };

            // NOTE: xml:space processing is mostly an undefined behavior,
            // because everyone do it differently.
            // We're mimicking the Chrome behavior.

            // Remove space from the second text node if both nodes has bound spaces.
            // From: '<text>Text <tspan> text</tspan></text>'
            // To:   '<text>Text <tspan>text</tspan></text>'
            //
            // See text-tspan-02-b.svg for details.
            if depth1 < depth2 {
                if c3 == Some(b' ') {
                    if xmlspace2 == XmlSpace::Default {
                        if let NodeKind::Text(ref mut text) = doc.nodes[node2_id.get_usize()].kind {
                            text.remove_first_space();
                        }
                    }
                }
            } else {
                if c2 == Some(b' ') && c2 == c3 {
                    if xmlspace1 == XmlSpace::Default && xmlspace2 == XmlSpace::Default {
                        if let NodeKind::Text(ref mut text) = doc.nodes[node1_id.get_usize()].kind {
                            text.remove_last_space();
                        }
                    } else {
                        if xmlspace1 == XmlSpace::Preserve && xmlspace2 == XmlSpace::Default {
                            if let NodeKind::Text(ref mut text) =
                                doc.nodes[node2_id.get_usize()].kind
                            {
                                text.remove_first_space();
                            }
                        }
                    }
                }
            }

            let is_first = i == 0;
            let is_last = i == len - 1;

            if is_first
                && c1 == Some(b' ')
                && xmlspace1 == XmlSpace::Default
                && !doc.get(node1_id).text().is_empty()
            {
                // Remove a leading space from a first text node.
                if let NodeKind::Text(ref mut text) = doc.nodes[node1_id.get_usize()].kind {
                    text.remove_first_space();
                }
            } else if is_last
                && c4 == Some(b' ')
                && !doc.get(node2_id).text().is_empty()
                && xmlspace2 == XmlSpace::Default
            {
                // Remove a trailing space from a last text node.
                // Also check that 'text2' is not empty already.
                if let NodeKind::Text(ref mut text) = doc.nodes[node2_id.get_usize()].kind {
                    text.remove_last_space();
                }
            }

            if is_last
                && c2 == Some(b' ')
                && !doc.get(node1_id).text().is_empty()
                && doc.get(node2_id).text().is_empty()
                && doc.get(node1_id).text().ends_with(' ')
            {
                if let NodeKind::Text(ref mut text) = doc.nodes[node1_id.get_usize()].kind {
                    text.remove_last_space();
                }
            }

            if !doc.get(node1_id).text().trim().is_empty() {
                last_non_empty = Some(node1_id);
            }

            i += 1;
        }
    }

    // TODO: find a way to remove all empty text nodes
}

fn collect_text_nodes(parent: SvgNode, depth: usize, nodes: &mut Vec<(NodeId, usize)>) {
    for child in parent.children() {
        if child.is_text() {
            nodes.push((child.id, depth));
        } else if child.is_element() {
            collect_text_nodes(child, depth + 1, nodes);
        }
    }
}

fn trim_text(text: &str, space: XmlSpace) -> String {
    let mut s = String::with_capacity(text.len());

    let mut prev = '0';
    for c in text.chars() {
        // \r, \n and \t should be converted into spaces.
        let c = match c {
            '\r' | '\n' | '\t' => ' ',
            _ => c,
        };

        // Skip continuous spaces.
        if space == XmlSpace::Default && c == ' ' && c == prev {
            continue;
        }

        prev = c;

        s.push(c);
    }

    s
}
