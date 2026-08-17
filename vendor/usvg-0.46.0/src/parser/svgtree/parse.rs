// Copyright 2021 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::HashMap;

use roxmltree::Error;
use simplecss::Declaration;
use svgtypes::FontShorthand;

use super::{AId, Attribute, Document, EId, NodeData, NodeId, NodeKind, ShortRange};

const SVG_NS: &str = "http://www.w3.org/2000/svg";
const XLINK_NS: &str = "http://www.w3.org/1999/xlink";
const XML_NAMESPACE_NS: &str = "http://www.w3.org/XML/1998/namespace";

impl<'input> Document<'input> {
    /// Parses a [`Document`] from a [`roxmltree::Document`].
    pub fn parse_tree(
        xml: &roxmltree::Document<'input>,
        injected_stylesheet: Option<&'input str>,
    ) -> Result<Document<'input>, Error> {
        parse(xml, injected_stylesheet)
    }

    pub(crate) fn append(&mut self, parent_id: NodeId, kind: NodeKind) -> NodeId {
        let new_child_id = NodeId::from(self.nodes.len());
        self.nodes.push(NodeData {
            parent: Some(parent_id),
            next_sibling: None,
            children: None,
            kind,
        });

        let last_child_id = self.nodes[parent_id.get_usize()].children.map(|(_, id)| id);

        if let Some(id) = last_child_id {
            self.nodes[id.get_usize()].next_sibling = Some(new_child_id);
        }

        self.nodes[parent_id.get_usize()].children = Some(
            if let Some((first_child_id, _)) = self.nodes[parent_id.get_usize()].children {
                (first_child_id, new_child_id)
            } else {
                (new_child_id, new_child_id)
            },
        );

        new_child_id
    }

    fn append_attribute(
        &mut self,
        name: AId,
        value: roxmltree::StringStorage<'input>,
        important: bool,
    ) {
        self.attrs.push(Attribute {
            name,
            value,
            important,
        });
    }
}

fn parse<'input>(
    xml: &roxmltree::Document<'input>,
    injected_stylesheet: Option<&'input str>,
) -> Result<Document<'input>, Error> {
    let mut doc = Document {
        nodes: Vec::new(),
        attrs: Vec::new(),
        links: HashMap::new(),
    };

    // build a map of id -> node for resolve_href
    let mut id_map = HashMap::new();
    for node in xml.descendants() {
        if let Some(id) = node.attribute("id") {
            if !id_map.contains_key(id) {
                id_map.insert(id, node);
            }
        }
    }

    // Add a root node.
    doc.nodes.push(NodeData {
        parent: None,
        next_sibling: None,
        children: None,
        kind: NodeKind::Root,
    });

    let style_sheet = resolve_css(xml, injected_stylesheet);

    parse_xml_node_children(
        xml.root(),
        xml.root(),
        doc.root().id,
        &style_sheet,
        false,
        0,
        &mut doc,
        &id_map,
    )?;

    // Check that the root element is `svg`.
    match doc.root().first_element_child() {
        Some(child) => {
            if child.tag_name() != Some(EId::Svg) {
                return Err(roxmltree::Error::NoRootNode);
            }
        }
        None => return Err(roxmltree::Error::NoRootNode),
    }

    // Collect all elements with `id` attribute.
    let mut links = HashMap::new();
    for node in doc.descendants() {
        if let Some(id) = node.attribute::<&str>(AId::Id) {
            links.insert(id.to_string(), node.id);
        }
    }
    doc.links = links;

    fix_recursive_patterns(&mut doc);
    fix_recursive_links(EId::ClipPath, AId::ClipPath, &mut doc);
    fix_recursive_links(EId::Mask, AId::Mask, &mut doc);
    fix_recursive_links(EId::Filter, AId::Filter, &mut doc);
    fix_recursive_fe_image(&mut doc);

    Ok(doc)
}

pub(crate) fn parse_tag_name(node: roxmltree::Node) -> Option<EId> {
    if !node.is_element() {
        return None;
    }

    if !matches!(node.tag_name().namespace(), None | Some(SVG_NS)) {
        return None;
    }

    EId::from_str(node.tag_name().name())
}

fn parse_xml_node_children<'input>(
    parent: roxmltree::Node<'_, 'input>,
    origin: roxmltree::Node,
    parent_id: NodeId,
    style_sheet: &simplecss::StyleSheet,
    ignore_ids: bool,
    depth: u32,
    doc: &mut Document<'input>,
    id_map: &HashMap<&str, roxmltree::Node<'_, 'input>>,
) -> Result<(), Error> {
    for node in parent.children() {
        parse_xml_node(
            node,
            origin,
            parent_id,
            style_sheet,
            ignore_ids,
            depth,
            doc,
            id_map,
        )?;
    }

    Ok(())
}

fn parse_xml_node<'input>(
    node: roxmltree::Node<'_, 'input>,
    origin: roxmltree::Node,
    parent_id: NodeId,
    style_sheet: &simplecss::StyleSheet,
    ignore_ids: bool,
    depth: u32,
    doc: &mut Document<'input>,
    id_map: &HashMap<&str, roxmltree::Node<'_, 'input>>,
) -> Result<(), Error> {
    if depth > 1024 {
        return Err(Error::NodesLimitReached);
    }

    let mut tag_name = match parse_tag_name(node) {
        Some(id) => id,
        None => return Ok(()),
    };

    if tag_name == EId::Style {
        return Ok(());
    }

    // TODO: remove?
    // Treat links as groups.
    if tag_name == EId::A {
        tag_name = EId::G;
    }

    let node_id = parse_svg_element(node, parent_id, tag_name, style_sheet, ignore_ids, doc)?;
    if tag_name == EId::Text {
        super::text::parse_svg_text_element(node, node_id, style_sheet, doc)?;
    } else if tag_name == EId::Use {
        parse_svg_use_element(node, origin, node_id, style_sheet, depth + 1, doc, id_map)?;
    } else {
        parse_xml_node_children(
            node,
            origin,
            node_id,
            style_sheet,
            ignore_ids,
            depth + 1,
            doc,
            id_map,
        )?;
    }

    Ok(())
}

pub(crate) fn parse_svg_element<'input>(
    xml_node: roxmltree::Node<'_, 'input>,
    parent_id: NodeId,
    tag_name: EId,
    style_sheet: &simplecss::StyleSheet,
    ignore_ids: bool,
    doc: &mut Document<'input>,
) -> Result<NodeId, Error> {
    let attrs_start_idx = doc.attrs.len();

    // Copy presentational attributes first.
    for attr in xml_node.attributes() {
        match attr.namespace() {
            None | Some(SVG_NS) | Some(XLINK_NS) | Some(XML_NAMESPACE_NS) => {}
            _ => continue,
        }

        let aid = match AId::from_str(attr.name()) {
            Some(v) => v,
            None => continue,
        };

        // During a `use` resolving, all `id` attributes must be ignored.
        // Otherwise we will get elements with duplicated id's.
        if ignore_ids && aid == AId::Id {
            continue;
        }

        // For some reason those properties are allowed only inside a `style` attribute and CSS.
        if matches!(aid, AId::MixBlendMode | AId::Isolation | AId::FontKerning) {
            continue;
        } else if aid == AId::ImageRendering
            && matches!(
                attr.value(),
                "smooth" | "high-quality" | "crisp-edges" | "pixelated"
            )
        {
            continue;
        }

        append_attribute(
            parent_id,
            tag_name,
            aid,
            attr.value_storage().clone(),
            false,
            doc,
        );
    }

    let mut insert_attribute = |aid, value: &str, important: bool| {
        // Check that attribute already exists.
        let idx = doc.attrs[attrs_start_idx..]
            .iter_mut()
            .position(|a| a.name == aid);

        // Append an attribute as usual.
        let added = append_attribute(
            parent_id,
            tag_name,
            aid,
            roxmltree::StringStorage::new_owned(value),
            important,
            doc,
        );

        // Check that attribute was actually added, because it could be skipped.
        if added {
            if let Some(idx) = idx {
                let last_idx = doc.attrs.len() - 1;
                let existing_idx = attrs_start_idx + idx;

                // See https://developer.mozilla.org/en-US/docs/Web/CSS/important
                // When a declaration is important, the order of precedence is reversed.
                // Declarations marked as important in the user-agent style sheets override
                // all important declarations in the user style sheets. Similarly, all important
                // declarations in the user style sheets override all important declarations in the
                // author's style sheets. Finally, all important declarations take precedence over
                // all animations.
                //
                // Which means:
                // 1) Existing is not important, new is not important -> swap
                // 2) Existing is important, new is not important -> don't swap
                // 3) Existing is not important, new is important -> swap
                // 4) Existing is important, new is important -> don't swap (since the order
                // is reversed, so existing important attributes take precedence over new
                // important attributes)
                let has_precedence = !doc.attrs[existing_idx].important;

                if has_precedence {
                    doc.attrs.swap(existing_idx, last_idx);
                }

                // Remove last.
                doc.attrs.pop();
            }
        }
    };

    let mut write_declaration = |declaration: &Declaration| {
        // TODO: perform XML attribute normalization
        let imp = declaration.important;
        let val = declaration.value;

        if declaration.name == "marker" {
            insert_attribute(AId::MarkerStart, val, imp);
            insert_attribute(AId::MarkerMid, val, imp);
            insert_attribute(AId::MarkerEnd, val, imp);
        } else if declaration.name == "font" {
            if let Ok(shorthand) = FontShorthand::from_str(val) {
                // First we need to reset all values to their default.
                insert_attribute(AId::FontStyle, "normal", imp);
                insert_attribute(AId::FontVariant, "normal", imp);
                insert_attribute(AId::FontWeight, "normal", imp);
                insert_attribute(AId::FontStretch, "normal", imp);
                insert_attribute(AId::LineHeight, "normal", imp);
                insert_attribute(AId::FontSizeAdjust, "none", imp);
                insert_attribute(AId::FontKerning, "auto", imp);
                insert_attribute(AId::FontVariantCaps, "normal", imp);
                insert_attribute(AId::FontVariantLigatures, "normal", imp);
                insert_attribute(AId::FontVariantNumeric, "normal", imp);
                insert_attribute(AId::FontVariantEastAsian, "normal", imp);
                insert_attribute(AId::FontVariantPosition, "normal", imp);

                // Then, we set the properties that have been declared.
                shorthand
                    .font_stretch
                    .map(|s| insert_attribute(AId::FontStretch, s, imp));
                shorthand
                    .font_weight
                    .map(|s| insert_attribute(AId::FontWeight, s, imp));
                shorthand
                    .font_variant
                    .map(|s| insert_attribute(AId::FontVariant, s, imp));
                shorthand
                    .font_style
                    .map(|s| insert_attribute(AId::FontStyle, s, imp));
                insert_attribute(AId::FontSize, shorthand.font_size, imp);
                insert_attribute(AId::FontFamily, shorthand.font_family, imp);
            } else {
                log::warn!(
                    "Failed to parse {} value: '{}'",
                    AId::Font,
                    declaration.value
                );
            }
        } else if let Some(aid) = AId::from_str(declaration.name) {
            // Parse only the presentation attributes.
            if aid.is_presentation() {
                insert_attribute(aid, val, imp);
            }
        }
    };

    // Apply CSS.
    for rule in &style_sheet.rules {
        if rule.selector.matches(&XmlNode(xml_node)) {
            for declaration in &rule.declarations {
                write_declaration(declaration);
            }
        }
    }

    // Split a `style` attribute.
    if let Some(value) = xml_node.attribute("style") {
        for declaration in simplecss::DeclarationTokenizer::from(value) {
            write_declaration(&declaration);
        }
    }

    if doc.nodes.len() > 1_000_000 {
        return Err(Error::NodesLimitReached);
    }

    let node_id = doc.append(
        parent_id,
        NodeKind::Element {
            tag_name,
            attributes: ShortRange::new(attrs_start_idx as u32, doc.attrs.len() as u32),
        },
    );

    Ok(node_id)
}

fn append_attribute<'input>(
    parent_id: NodeId,
    tag_name: EId,
    aid: AId,
    value: roxmltree::StringStorage<'input>,
    important: bool,
    doc: &mut Document<'input>,
) -> bool {
    match aid {
        // The `style` attribute will be split into attributes, so we don't need it.
        AId::Style |
        // No need to copy a `class` attribute since CSS were already resolved.
        AId::Class => return false,
        _ => {}
    }

    // Ignore `xlink:href` on `tspan` (which was originally `tref` or `a`),
    // because we will convert `tref` into `tspan` anyway.
    if tag_name == EId::Tspan && aid == AId::Href {
        return false;
    }

    if aid.allows_inherit_value() && &*value == "inherit" {
        return resolve_inherit(parent_id, aid, doc);
    }

    doc.append_attribute(aid, value, important);
    true
}

fn resolve_inherit(parent_id: NodeId, aid: AId, doc: &mut Document) -> bool {
    if aid.is_inheritable() {
        // Inheritable attributes can inherit a value from an any ancestor.
        let node_id = doc
            .get(parent_id)
            .ancestors()
            .find(|n| n.has_attribute(aid))
            .map(|n| n.id);
        if let Some(node_id) = node_id {
            if let Some(attr) = doc
                .get(node_id)
                .attributes()
                .iter()
                .find(|a| a.name == aid)
                .cloned()
            {
                doc.attrs.push(Attribute {
                    name: aid,
                    value: attr.value,
                    important: attr.important,
                });

                return true;
            }
        }
    } else {
        // Non-inheritable attributes can inherit a value only from a direct parent.
        if let Some(attr) = doc
            .get(parent_id)
            .attributes()
            .iter()
            .find(|a| a.name == aid)
            .cloned()
        {
            doc.attrs.push(Attribute {
                name: aid,
                value: attr.value,
                important: attr.important,
            });

            return true;
        }
    }

    // Fallback to a default value if possible.
    let value = match aid {
        AId::ImageRendering | AId::ShapeRendering | AId::TextRendering => "auto",

        AId::ClipPath
        | AId::Filter
        | AId::MarkerEnd
        | AId::MarkerMid
        | AId::MarkerStart
        | AId::Mask
        | AId::Stroke
        | AId::StrokeDasharray
        | AId::TextDecoration => "none",

        AId::FontStretch
        | AId::FontStyle
        | AId::FontVariant
        | AId::FontWeight
        | AId::LetterSpacing
        | AId::WordSpacing => "normal",

        AId::Fill | AId::FloodColor | AId::StopColor => "black",

        AId::FillOpacity
        | AId::FloodOpacity
        | AId::Opacity
        | AId::StopOpacity
        | AId::StrokeOpacity => "1",

        AId::ClipRule | AId::FillRule => "nonzero",

        AId::BaselineShift => "baseline",
        AId::ColorInterpolationFilters => "linearRGB",
        AId::Direction => "ltr",
        AId::Display => "inline",
        AId::FontSize => "medium",
        AId::Overflow => "visible",
        AId::StrokeDashoffset => "0",
        AId::StrokeLinecap => "butt",
        AId::StrokeLinejoin => "miter",
        AId::StrokeMiterlimit => "4",
        AId::StrokeWidth => "1",
        AId::TextAnchor => "start",
        AId::Visibility => "visible",
        AId::WritingMode => "lr-tb",
        _ => return false,
    };

    doc.append_attribute(aid, roxmltree::StringStorage::Borrowed(value), false);
    true
}

fn resolve_href<'a, 'input: 'a>(
    node: roxmltree::Node<'a, 'input>,
    id_map: &HashMap<&str, roxmltree::Node<'a, 'input>>,
) -> Option<roxmltree::Node<'a, 'input>> {
    let link_value = node
        .attribute((XLINK_NS, "href"))
        .or_else(|| node.attribute("href"))?;

    let link_id = svgtypes::IRI::from_str(link_value).ok()?.0;

    id_map.get(link_id).copied()
}

fn parse_svg_use_element<'input>(
    node: roxmltree::Node<'_, 'input>,
    origin: roxmltree::Node,
    parent_id: NodeId,
    style_sheet: &simplecss::StyleSheet,
    depth: u32,
    doc: &mut Document<'input>,
    id_map: &HashMap<&str, roxmltree::Node<'_, 'input>>,
) -> Result<(), Error> {
    let link = match resolve_href(node, id_map) {
        Some(v) => v,
        None => return Ok(()),
    };

    if link == node || link == origin {
        log::warn!(
            "Recursive 'use' detected. '{}' will be skipped.",
            node.attribute((SVG_NS, "id")).unwrap_or_default()
        );
        return Ok(());
    }

    // Make sure we're linked to an SVG element.
    if parse_tag_name(link).is_none() {
        return Ok(());
    }

    // Check that none of the linked node's children reference current `use` node
    // via other `use` node.
    //
    // Example:
    // <g id="g1">
    //     <use xlink:href="#use1" id="use2"/>
    // </g>
    // <use xlink:href="#g1" id="use1"/>
    //
    // `use2` should be removed.
    //
    // Also, child should not reference its parent:
    // <g id="g1">
    //     <use xlink:href="#g1" id="use1"/>
    // </g>
    //
    // `use1` should be removed.
    let mut is_recursive = false;
    for link_child in link
        .descendants()
        .skip(1)
        .filter(|n| n.has_tag_name((SVG_NS, "use")))
    {
        if let Some(link2) = resolve_href(link_child, id_map) {
            if link2 == node || link2 == link {
                is_recursive = true;
                break;
            }
        }
    }

    if is_recursive {
        log::warn!(
            "Recursive 'use' detected. '{}' will be skipped.",
            node.attribute((SVG_NS, "id")).unwrap_or_default()
        );
        return Ok(());
    }

    parse_xml_node(
        link,
        node,
        parent_id,
        style_sheet,
        true,
        depth + 1,
        doc,
        id_map,
    )
}

fn resolve_css<'a>(
    xml: &'a roxmltree::Document<'a>,
    style_sheet: Option<&'a str>,
) -> simplecss::StyleSheet<'a> {
    let mut sheet = simplecss::StyleSheet::new();

    // Injected style sheets do not override internal ones (we mimic the logic of rsvg-convert),
    // so we need to parse it first.
    if let Some(style_sheet) = style_sheet {
        sheet.parse_more(style_sheet);
    }

    for node in xml.descendants().filter(|n| n.has_tag_name("style")) {
        match node.attribute("type") {
            Some("text/css") => {}
            Some(_) => continue,
            None => {}
        }

        let text = match node.text() {
            Some(v) => v,
            None => continue,
        };

        sheet.parse_more(text);
    }

    sheet
}

struct XmlNode<'a, 'input: 'a>(roxmltree::Node<'a, 'input>);

impl simplecss::Element for XmlNode<'_, '_> {
    fn parent_element(&self) -> Option<Self> {
        self.0.parent_element().map(XmlNode)
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.0.prev_sibling_element().map(XmlNode)
    }

    fn has_local_name(&self, local_name: &str) -> bool {
        self.0.tag_name().name() == local_name
    }

    fn attribute_matches(&self, local_name: &str, operator: simplecss::AttributeOperator) -> bool {
        match self.0.attribute(local_name) {
            Some(value) => operator.matches(value),
            None => false,
        }
    }

    fn pseudo_class_matches(&self, class: simplecss::PseudoClass) -> bool {
        match class {
            simplecss::PseudoClass::FirstChild => self.prev_sibling_element().is_none(),
            // TODO: lang
            _ => false, // Since we are querying a static SVG we can ignore other pseudo-classes.
        }
    }
}

fn fix_recursive_patterns(doc: &mut Document) {
    while let Some(node_id) = find_recursive_pattern(AId::Fill, doc) {
        let idx = doc.get(node_id).attribute_id(AId::Fill).unwrap();
        doc.attrs[idx].value = roxmltree::StringStorage::Borrowed("none");
    }

    while let Some(node_id) = find_recursive_pattern(AId::Stroke, doc) {
        let idx = doc.get(node_id).attribute_id(AId::Stroke).unwrap();
        doc.attrs[idx].value = roxmltree::StringStorage::Borrowed("none");
    }
}

fn find_recursive_pattern(aid: AId, doc: &mut Document) -> Option<NodeId> {
    for pattern_node in doc
        .root()
        .descendants()
        .filter(|n| n.tag_name() == Some(EId::Pattern))
    {
        for node in pattern_node.descendants() {
            let value = match node.attribute(aid) {
                Some(v) => v,
                None => continue,
            };

            if let Ok(svgtypes::Paint::FuncIRI(link_id, _)) = svgtypes::Paint::from_str(value) {
                if link_id == pattern_node.element_id() {
                    // If a pattern child has a link to the pattern itself
                    // then we have to replace it with `none`.
                    // Otherwise we will get endless loop/recursion and stack overflow.
                    return Some(node.id);
                } else {
                    // Check that linked node children doesn't link this pattern.
                    if let Some(linked_node) = doc.element_by_id(link_id) {
                        for node2 in linked_node.descendants() {
                            let value2 = match node2.attribute(aid) {
                                Some(v) => v,
                                None => continue,
                            };

                            if let Ok(svgtypes::Paint::FuncIRI(link_id2, _)) =
                                svgtypes::Paint::from_str(value2)
                            {
                                if link_id2 == pattern_node.element_id() {
                                    return Some(node2.id);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn fix_recursive_links(eid: EId, aid: AId, doc: &mut Document) {
    while let Some(node_id) = find_recursive_link(eid, aid, doc) {
        let idx = doc.get(node_id).attribute_id(aid).unwrap();
        doc.attrs[idx].value = roxmltree::StringStorage::Borrowed("none");
    }
}

fn find_recursive_link(eid: EId, aid: AId, doc: &Document) -> Option<NodeId> {
    for node in doc
        .root()
        .descendants()
        .filter(|n| n.tag_name() == Some(eid))
    {
        for child in node.descendants() {
            if let Some(link) = child.node_attribute(aid) {
                if link == node {
                    // If an element child has a link to the element itself
                    // then we have to replace it with `none`.
                    // Otherwise we will get endless loop/recursion and stack overflow.
                    return Some(child.id);
                } else {
                    // Check that linked node children doesn't link this element.
                    for node2 in link.descendants() {
                        if let Some(link2) = node2.node_attribute(aid) {
                            if link2 == node {
                                return Some(node2.id);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Detects cases like:
///
/// ```xml
/// <filter id="filter1">
///   <feImage xlink:href="#rect1"/>
/// </filter>
/// <rect id="rect1" x="36" y="36" width="120" height="120" fill="green" filter="url(#filter1)"/>
/// ```
fn fix_recursive_fe_image(doc: &mut Document) {
    let mut ids = Vec::new();
    for fe_node in doc
        .root()
        .descendants()
        .filter(|n| n.tag_name() == Some(EId::FeImage))
    {
        if let Some(link) = fe_node.node_attribute(AId::Href) {
            if let Some(filter_uri) = link.attribute::<&str>(AId::Filter) {
                let filter_id = fe_node.parent().unwrap().element_id();
                for func in svgtypes::FilterValueListParser::from(filter_uri).flatten() {
                    if let svgtypes::FilterValue::Url(url) = func {
                        if url == filter_id {
                            ids.push(link.id);
                        }
                    }
                }
            }
        }
    }

    for id in ids {
        let idx = doc.get(id).attribute_id(AId::Filter).unwrap();
        doc.attrs[idx].value = roxmltree::StringStorage::Borrowed("none");
    }
}
