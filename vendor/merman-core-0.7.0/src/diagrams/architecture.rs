use crate::{Error, ParseMetadata, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ArchitectureGroup {
    id: String,
    icon: Option<String>,
    title: Option<String>,
    in_group: Option<String>,
}

#[derive(Debug, Clone)]
struct ArchitectureEdge {
    lhs_id: String,
    lhs_dir: char,
    lhs_into: Option<bool>,
    lhs_group: Option<bool>,
    rhs_id: String,
    rhs_dir: char,
    rhs_into: Option<bool>,
    rhs_group: Option<bool>,
    title: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArchitectureNodeType {
    Service,
    Junction,
}

#[derive(Debug, Clone)]
struct ArchitectureNode {
    id: String,
    ty: ArchitectureNodeType,
    edges: Vec<usize>,
    icon: Option<String>,
    icon_text: Option<String>,
    title: Option<String>,
    in_group: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegisteredIdType {
    Node,
    Group,
}

impl std::fmt::Display for RegisteredIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisteredIdType::Node => write!(f, "node"),
            RegisteredIdType::Group => write!(f, "group"),
        }
    }
}

#[derive(Debug, Default)]
struct ArchitectureDb {
    title: String,
    acc_title: String,
    acc_descr: String,

    nodes: HashMap<String, ArchitectureNode>,
    node_order: Vec<String>,
    groups: HashMap<String, ArchitectureGroup>,
    group_order: Vec<String>,
    edges: Vec<ArchitectureEdge>,
    registered_ids: HashMap<String, RegisteredIdType>,
}

impl ArchitectureDb {
    fn clear(&mut self) {
        *self = Self::default();
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn set_acc_title(&mut self, title: String) {
        self.acc_title = title;
    }

    fn set_acc_descr(&mut self, descr: String) {
        self.acc_descr = descr;
    }

    fn render_model(&self) -> ArchitectureDiagramRenderModel {
        let title = (!self.title.trim().is_empty()).then(|| self.title.clone());
        let acc_title = (!self.acc_title.trim().is_empty()).then(|| self.acc_title.clone());
        let acc_descr = (!self.acc_descr.trim().is_empty()).then(|| self.acc_descr.clone());

        let nodes: Vec<ArchitectureRenderNode> = self
            .node_order
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .map(|n| ArchitectureRenderNode {
                id: n.id.clone(),
                node_type: match n.ty {
                    ArchitectureNodeType::Service => ArchitectureRenderNodeType::Service,
                    ArchitectureNodeType::Junction => ArchitectureRenderNodeType::Junction,
                },
                edge_indices: n.edges.clone(),
                icon: n.icon.clone(),
                icon_text: n.icon_text.clone(),
                title: n.title.clone(),
                in_group: n.in_group.clone(),
            })
            .collect();

        let groups: Vec<ArchitectureRenderGroup> = self
            .group_order
            .iter()
            .filter_map(|id| self.groups.get(id))
            .map(|g| ArchitectureRenderGroup {
                id: g.id.clone(),
                icon: g.icon.clone(),
                title: g.title.clone(),
                in_group: g.in_group.clone(),
            })
            .collect();

        let edges: Vec<ArchitectureRenderEdge> = self
            .edges
            .iter()
            .map(|e| ArchitectureRenderEdge {
                lhs_id: e.lhs_id.clone(),
                lhs_dir: e.lhs_dir,
                lhs_into: e.lhs_into,
                lhs_group: e.lhs_group,
                rhs_id: e.rhs_id.clone(),
                rhs_dir: e.rhs_dir,
                rhs_into: e.rhs_into,
                rhs_group: e.rhs_group,
                title: e.title.clone(),
            })
            .collect();

        ArchitectureDiagramRenderModel {
            title,
            acc_title,
            acc_descr,
            nodes,
            groups,
            edges,
        }
    }

    fn add_service(
        &mut self,
        id: String,
        icon: Option<String>,
        icon_text: Option<String>,
        title: Option<String>,
        in_group: Option<String>,
    ) -> Result<()> {
        if let Some(existing) = self.registered_ids.get(&id) {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: format!("The service id [{id}] is already in use by another {existing}"),
            });
        }

        if let Some(parent) = &in_group {
            if id == *parent {
                return Err(Error::DiagramParse {
                    diagram_type: "architecture".to_string(),
                    message: format!("The service [{id}] cannot be placed within itself"),
                });
            }
            let Some(parent_type) = self.registered_ids.get(parent).copied() else {
                return Err(Error::DiagramParse {
                    diagram_type: "architecture".to_string(),
                    message: format!(
                        "The service [{id}]'s parent does not exist. Please make sure the parent is created before this service"
                    ),
                });
            };
            if parent_type == RegisteredIdType::Node {
                return Err(Error::DiagramParse {
                    diagram_type: "architecture".to_string(),
                    message: format!("The service [{id}]'s parent is not a group"),
                });
            }
        }

        self.registered_ids
            .insert(id.clone(), RegisteredIdType::Node);
        if !self.nodes.contains_key(&id) {
            self.node_order.push(id.clone());
        }
        self.nodes.insert(
            id.clone(),
            ArchitectureNode {
                id,
                ty: ArchitectureNodeType::Service,
                edges: Vec::new(),
                icon,
                icon_text,
                title,
                in_group,
            },
        );
        Ok(())
    }

    fn add_junction(&mut self, id: String, in_group: Option<String>) {
        self.registered_ids
            .insert(id.clone(), RegisteredIdType::Node);
        if !self.nodes.contains_key(&id) {
            self.node_order.push(id.clone());
        }
        self.nodes.insert(
            id.clone(),
            ArchitectureNode {
                id,
                ty: ArchitectureNodeType::Junction,
                edges: Vec::new(),
                icon: None,
                icon_text: None,
                title: None,
                in_group,
            },
        );
    }

    fn add_group(
        &mut self,
        id: String,
        icon: Option<String>,
        title: Option<String>,
        in_group: Option<String>,
    ) -> Result<()> {
        if let Some(existing) = self.registered_ids.get(&id) {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: format!("The group id [{id}] is already in use by another {existing}"),
            });
        }

        if let Some(parent) = &in_group {
            if id == *parent {
                return Err(Error::DiagramParse {
                    diagram_type: "architecture".to_string(),
                    message: format!("The group [{id}] cannot be placed within itself"),
                });
            }
            let Some(parent_type) = self.registered_ids.get(parent).copied() else {
                return Err(Error::DiagramParse {
                    diagram_type: "architecture".to_string(),
                    message: format!(
                        "The group [{id}]'s parent does not exist. Please make sure the parent is created before this group"
                    ),
                });
            };
            if parent_type == RegisteredIdType::Node {
                return Err(Error::DiagramParse {
                    diagram_type: "architecture".to_string(),
                    message: format!("The group [{id}]'s parent is not a group"),
                });
            }
        }

        self.registered_ids
            .insert(id.clone(), RegisteredIdType::Group);
        if !self.groups.contains_key(&id) {
            self.group_order.push(id.clone());
        }
        self.groups.insert(
            id.clone(),
            ArchitectureGroup {
                id,
                icon,
                title,
                in_group,
            },
        );
        Ok(())
    }

    fn add_edge(&mut self, edge: ArchitectureEdge) -> Result<()> {
        if !is_dir(edge.lhs_dir) {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: format!(
                    "Invalid direction given for left hand side of edge {}--{}. Expected (L,R,T,B) got {}",
                    edge.lhs_id, edge.rhs_id, edge.lhs_dir
                ),
            });
        }
        if !is_dir(edge.rhs_dir) {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: format!(
                    "Invalid direction given for right hand side of edge {}--{}. Expected (L,R,T,B) got {}",
                    edge.lhs_id, edge.rhs_id, edge.rhs_dir
                ),
            });
        }

        if !self.nodes.contains_key(&edge.lhs_id) && !self.groups.contains_key(&edge.lhs_id) {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: format!(
                    "The left-hand id [{}] does not yet exist. Please create the service/group before declaring an edge to it.",
                    edge.lhs_id
                ),
            });
        }
        if !self.nodes.contains_key(&edge.rhs_id) && !self.groups.contains_key(&edge.rhs_id) {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: format!(
                    "The right-hand id [{}] does not yet exist. Please create the service/group before declaring an edge to it.",
                    edge.rhs_id
                ),
            });
        }

        if edge.lhs_group == Some(true) {
            if let (Some(lhs), Some(rhs)) =
                (self.nodes.get(&edge.lhs_id), self.nodes.get(&edge.rhs_id))
            {
                if let (Some(lhs_parent), Some(rhs_parent)) = (&lhs.in_group, &rhs.in_group) {
                    if lhs_parent == rhs_parent {
                        return Err(Error::DiagramParse {
                            diagram_type: "architecture".to_string(),
                            message: format!(
                                "The left-hand id [{}] is modified to traverse the group boundary, but the edge does not pass through two groups.",
                                edge.lhs_id
                            ),
                        });
                    }
                }
            }
        }
        if edge.rhs_group == Some(true) {
            if let (Some(lhs), Some(rhs)) =
                (self.nodes.get(&edge.lhs_id), self.nodes.get(&edge.rhs_id))
            {
                if let (Some(lhs_parent), Some(rhs_parent)) = (&lhs.in_group, &rhs.in_group) {
                    if lhs_parent == rhs_parent {
                        return Err(Error::DiagramParse {
                            diagram_type: "architecture".to_string(),
                            message: format!(
                                "The right-hand id [{}] is modified to traverse the group boundary, but the edge does not pass through two groups.",
                                edge.rhs_id
                            ),
                        });
                    }
                }
            }
        }

        let edge_idx = self.edges.len();
        self.edges.push(edge);
        let lhs_id = self.edges[edge_idx].lhs_id.clone();
        let rhs_id = self.edges[edge_idx].rhs_id.clone();
        if self.nodes.contains_key(&lhs_id) && self.nodes.contains_key(&rhs_id) {
            if let Some(lhs) = self.nodes.get_mut(&lhs_id) {
                lhs.edges.push(edge_idx);
            }
            if let Some(rhs) = self.nodes.get_mut(&rhs_id) {
                rhs.edges.push(edge_idx);
            }
        }
        Ok(())
    }

    fn edges_json(&self) -> Vec<Value> {
        self.edges
            .iter()
            .map(|e| {
                json!({
                    "lhsId": e.lhs_id,
                    "lhsDir": e.lhs_dir.to_string(),
                    "lhsInto": e.lhs_into,
                    "lhsGroup": e.lhs_group,
                    "rhsId": e.rhs_id,
                    "rhsDir": e.rhs_dir.to_string(),
                    "rhsInto": e.rhs_into,
                    "rhsGroup": e.rhs_group,
                    "title": e.title,
                })
            })
            .collect()
    }

    fn groups_json(&self) -> Vec<Value> {
        self.group_order
            .iter()
            .filter_map(|id| self.groups.get(id))
            .map(|g| {
                json!({
                    "id": g.id,
                    "icon": g.icon,
                    "title": g.title,
                    "in": g.in_group,
                })
            })
            .collect()
    }

    fn nodes_json(&self) -> Vec<Value> {
        self.node_order
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .map(|n| {
                let edges: Vec<Value> = n
                    .edges
                    .iter()
                    .filter_map(|idx| self.edges.get(*idx))
                    .map(|e| {
                        json!({
                            "lhsId": e.lhs_id,
                            "lhsDir": e.lhs_dir.to_string(),
                            "lhsInto": e.lhs_into,
                            "lhsGroup": e.lhs_group,
                            "rhsId": e.rhs_id,
                            "rhsDir": e.rhs_dir.to_string(),
                            "rhsInto": e.rhs_into,
                            "rhsGroup": e.rhs_group,
                            "title": e.title,
                        })
                    })
                    .collect();

                let ty = match n.ty {
                    ArchitectureNodeType::Service => "service",
                    ArchitectureNodeType::Junction => "junction",
                };

                json!({
                    "id": n.id,
                    "type": ty,
                    "edges": edges,
                    "icon": n.icon,
                    "iconText": n.icon_text,
                    "title": n.title,
                    "in": n.in_group,
                })
            })
            .collect()
    }

    fn services_json(&self) -> Vec<Value> {
        self.nodes_json()
            .into_iter()
            .filter(|n| n.get("type").and_then(|v| v.as_str()) == Some("service"))
            .collect()
    }

    fn junctions_json(&self) -> Vec<Value> {
        self.nodes_json()
            .into_iter()
            .filter(|n| n.get("type").and_then(|v| v.as_str()) == Some("junction"))
            .collect()
    }
}

fn is_dir(c: char) -> bool {
    matches!(c, 'L' | 'R' | 'T' | 'B')
}

fn strip_inline_comment(line: &str) -> &str {
    let mut in_quote = false;
    let mut quote_char: Option<char> = None;
    let mut it = line.char_indices().peekable();
    while let Some((idx, ch)) = it.next() {
        if in_quote {
            if ch == '\\' {
                it.next();
                continue;
            }
            if Some(ch) == quote_char {
                in_quote = false;
                quote_char = None;
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            in_quote = true;
            quote_char = Some(ch);
            continue;
        }
        if ch == '%' && it.peek().is_some_and(|(_, next)| *next == '%') {
            return &line[..idx];
        }
    }
    line
}

fn starts_with_kw(line: &str, kw: &str) -> bool {
    let t = line.trim_start();
    if !t.starts_with(kw) {
        return false;
    }
    let rest = &t[kw.len()..];
    rest.is_empty() || rest.chars().next().is_some_and(|c| c.is_whitespace())
}

fn parse_title_stmt(line: &str) -> Option<String> {
    if !starts_with_kw(line, "title") {
        return None;
    }
    let t = line.trim_start();
    let rest = &t["title".len()..];
    let rest = rest.strip_prefix(|c: char| c.is_whitespace()).unwrap_or("");
    Some(rest.to_string())
}

fn parse_acc_title_stmt(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("accTitle") {
        return None;
    }
    let rest = &t["accTitle".len()..];
    let rest = rest.trim_start();
    let rest = rest.strip_prefix(':')?;
    Some(rest.trim().to_string())
}

fn parse_acc_descr_stmt_single(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("accDescr") {
        return None;
    }
    let rest = &t["accDescr".len()..];
    let rest = rest.trim_start();
    let rest = rest.strip_prefix(':')?;
    Some(rest.trim().to_string())
}

fn parse_acc_descr_block(lines: &mut std::str::Lines<'_>, first_line: &str) -> Option<String> {
    let t = first_line.trim_start();
    if !t.starts_with("accDescr") {
        return None;
    }
    let rest = t["accDescr".len()..].trim_start();
    let rest = rest.strip_prefix('{')?;

    let mut buf = String::new();
    if let Some(end) = rest.find('}') {
        buf.push_str(&rest[..end]);
        return Some(buf.trim().to_string());
    }
    buf.push_str(rest);
    buf.push('\n');

    for line in lines {
        if let Some(end) = line.find('}') {
            buf.push_str(&line[..end]);
            break;
        }
        buf.push_str(line);
        buf.push('\n');
    }
    Some(buf.trim().to_string())
}

fn take_id_prefix(input: &str) -> Option<(&str, &str)> {
    let mut last_word_end: Option<usize> = None;
    let mut seen_any = false;
    for (idx, ch) in input.char_indices() {
        let is_word = ch.is_ascii_alphanumeric() || ch == '_';
        let is_allowed = is_word || ch == '-';
        if !seen_any {
            if !is_word {
                return None;
            }
            seen_any = true;
            last_word_end = Some(idx + ch.len_utf8());
            continue;
        }
        if !is_allowed {
            break;
        }
        if is_word {
            last_word_end = Some(idx + ch.len_utf8());
        }
    }
    let end = last_word_end?;
    Some((&input[..end], &input[end..]))
}

fn take_bracketed(input: &str, open: char, close: char) -> Option<(String, &str)> {
    let mut it = input.char_indices();
    let (_, first) = it.next()?;
    if first != open {
        return None;
    }
    for (idx, ch) in it {
        if ch == close {
            let inner = input[1..idx].to_string();
            return Some((inner, &input[idx + close.len_utf8()..]));
        }
    }
    None
}

fn parse_group_stmt(db: &mut ArchitectureDb, line: &str) -> Result<bool> {
    if !starts_with_kw(line, "group") {
        return Ok(false);
    }
    let t = line.trim_start();
    let mut rest = t["group".len()..].trim_start();
    let Some((id, tail)) = take_id_prefix(rest) else {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "invalid group id".to_string(),
        });
    };
    let id = id.to_string();
    rest = tail.trim_start();

    let mut icon = None;
    if let Some((i, tail)) = take_bracketed(rest, '(', ')') {
        icon = Some(i.trim().to_string());
        rest = tail.trim_start();
    }

    let mut title = None;
    if let Some((t, tail)) = take_bracketed(rest, '[', ']') {
        title = Some(t.trim().to_string());
        rest = tail.trim_start();
    }

    let mut in_group = None;
    if starts_with_kw(rest, "in") {
        rest = rest.trim_start()["in".len()..].trim_start();
        let Some((parent, tail)) = take_id_prefix(rest) else {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: "invalid group parent id".to_string(),
            });
        };
        in_group = Some(parent.to_string());
        rest = tail.trim();
    }

    if !rest.trim().is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "unexpected trailing input".to_string(),
        });
    }

    db.add_group(id, icon, title, in_group)?;
    Ok(true)
}

fn take_quoted(input: &str) -> Option<(String, &str)> {
    let mut it = input.char_indices();
    let (_, q) = it.next()?;
    if q != '"' && q != '\'' {
        return None;
    }
    let mut escaped = false;
    for (idx, ch) in it {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == q {
            let inner = input[1..idx].to_string();
            return Some((inner, &input[idx + q.len_utf8()..]));
        }
    }
    None
}

fn parse_service_stmt(db: &mut ArchitectureDb, line: &str) -> Result<bool> {
    if !starts_with_kw(line, "service") {
        return Ok(false);
    }
    let t = line.trim_start();
    let mut rest = t["service".len()..].trim_start();
    let Some((id, tail)) = take_id_prefix(rest) else {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "invalid service id".to_string(),
        });
    };
    let id = id.to_string();
    rest = tail.trim_start();

    let mut icon = None;
    let mut icon_text = None;
    if let Some((i, tail)) = take_bracketed(rest, '(', ')') {
        icon = Some(i.trim().to_string());
        rest = tail.trim_start();
    } else if let Some((s, tail)) = take_quoted(rest) {
        icon_text = Some(s);
        rest = tail.trim_start();
    }

    let mut title = None;
    if let Some((t, tail)) = take_bracketed(rest, '[', ']') {
        title = Some(t.trim().to_string());
        rest = tail.trim_start();
    }

    let mut in_group = None;
    if starts_with_kw(rest, "in") {
        rest = rest.trim_start()["in".len()..].trim_start();
        let Some((parent, tail)) = take_id_prefix(rest) else {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: "invalid service parent id".to_string(),
            });
        };
        in_group = Some(parent.to_string());
        rest = tail.trim();
    }

    if !rest.trim().is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "unexpected trailing input".to_string(),
        });
    }

    db.add_service(id, icon, icon_text, title, in_group)?;
    Ok(true)
}

fn parse_junction_stmt(db: &mut ArchitectureDb, line: &str) -> Result<bool> {
    if !starts_with_kw(line, "junction") {
        return Ok(false);
    }
    let t = line.trim_start();
    let mut rest = t["junction".len()..].trim_start();
    let Some((id, tail)) = take_id_prefix(rest) else {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "invalid junction id".to_string(),
        });
    };
    let id = id.to_string();
    rest = tail.trim_start();

    let mut in_group = None;
    if starts_with_kw(rest, "in") {
        rest = rest.trim_start()["in".len()..].trim_start();
        let Some((parent, tail)) = take_id_prefix(rest) else {
            return Err(Error::DiagramParse {
                diagram_type: "architecture".to_string(),
                message: "invalid junction parent id".to_string(),
            });
        };
        in_group = Some(parent.to_string());
        rest = tail.trim();
    }

    if !rest.trim().is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "unexpected trailing input".to_string(),
        });
    }

    db.add_junction(id, in_group);
    Ok(true)
}

fn parse_id_with_optional_group_modifier(input: &str) -> Result<(String, Option<bool>, &str)> {
    let Some((id, rest)) = take_id_prefix(input) else {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "invalid id".to_string(),
        });
    };
    let mut rest = rest;
    let mut group = None;
    if rest.starts_with("{group}") {
        group = Some(true);
        rest = &rest["{group}".len()..];
    }
    Ok((id.to_string(), group, rest))
}

fn is_arch_dir(ch: char) -> bool {
    matches!(ch, 'L' | 'R' | 'T' | 'B')
}

fn parse_edge_stmt(db: &mut ArchitectureDb, line: &str) -> Result<bool> {
    let mut rest = line.trim_start();
    if rest.is_empty() {
        return Ok(false);
    }
    if starts_with_kw(rest, "group")
        || starts_with_kw(rest, "service")
        || starts_with_kw(rest, "junction")
        || starts_with_kw(rest, "title")
        || starts_with_kw(rest, "accTitle")
        || starts_with_kw(rest, "accDescr")
    {
        return Ok(false);
    }

    let (lhs_id, lhs_group, tail) = parse_id_with_optional_group_modifier(rest)?;
    rest = tail.trim_start();

    let mut lhs_into = None;
    let mut rhs_into = None;
    let mut title = None;

    rest = rest.strip_prefix(':').ok_or_else(|| Error::DiagramParse {
        diagram_type: "architecture".to_string(),
        message: "expected ':' for lhs port".to_string(),
    })?;
    rest = rest.trim_start();
    let lhs_dir: char = rest.chars().next().ok_or_else(|| Error::DiagramParse {
        diagram_type: "architecture".to_string(),
        message: "expected lhs direction".to_string(),
    })?;
    if !is_arch_dir(lhs_dir) {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "invalid lhs direction".to_string(),
        });
    }
    rest = &rest[lhs_dir.len_utf8()..];

    rest = rest.trim_start();
    if let Some(ch) = rest.chars().next() {
        if ch == '<' || ch == '>' {
            lhs_into = Some(true);
            rest = &rest[ch.len_utf8()..];
        }
    }

    rest = rest.trim_start();
    if rest.starts_with("--") {
        rest = &rest[2..];
    } else if rest.starts_with('-') {
        rest = &rest[1..];
        rest = rest.trim_start();
        let (t, tail) = take_bracketed(rest, '[', ']').ok_or_else(|| Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "expected edge title".to_string(),
        })?;
        title = Some(t.trim().to_string());
        rest = tail.trim_start();
        rest = rest.strip_prefix('-').ok_or_else(|| Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "expected '-' after edge title".to_string(),
        })?;
    } else {
        return Ok(false);
    }

    rest = rest.trim_start();
    if let Some(ch) = rest.chars().next() {
        if ch == '<' || ch == '>' {
            rhs_into = Some(true);
            rest = &rest[ch.len_utf8()..];
        }
    }

    rest = rest.trim_start();
    let rhs_dir: char = rest.chars().next().ok_or_else(|| Error::DiagramParse {
        diagram_type: "architecture".to_string(),
        message: "expected rhs direction".to_string(),
    })?;
    if !is_arch_dir(rhs_dir) {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "invalid rhs direction".to_string(),
        });
    }
    rest = &rest[rhs_dir.len_utf8()..];

    rest = rest.trim_start();
    rest = rest.strip_prefix(':').ok_or_else(|| Error::DiagramParse {
        diagram_type: "architecture".to_string(),
        message: "expected ':' for rhs port".to_string(),
    })?;

    rest = rest.trim_start();
    if rest.starts_with(':') {
        rest = &rest[1..];
        rest = rest.trim_start();
    }
    let (rhs_id, rhs_group, tail) = parse_id_with_optional_group_modifier(rest)?;
    rest = tail.trim();

    if !rest.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "architecture".to_string(),
            message: "unexpected trailing input".to_string(),
        });
    }

    db.add_edge(ArchitectureEdge {
        lhs_id,
        lhs_dir,
        lhs_into,
        lhs_group,
        rhs_id,
        rhs_dir,
        rhs_into,
        rhs_group,
        title,
    })?;

    Ok(true)
}

pub fn parse_architecture(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let mut db = ArchitectureDb::default();
    db.clear();

    let mut lines = code.lines();
    let mut found_header = false;
    let mut header_tail: Option<String> = None;
    for line in lines.by_ref() {
        let t = strip_inline_comment(line);
        let trimmed = t.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("architecture-beta") {
            let rest = rest.trim_start();
            if !rest.is_empty() {
                header_tail = Some(rest.to_string());
            }
            found_header = true;
            break;
        }
        break;
    }

    if !found_header {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected architecture-beta header".to_string(),
        });
    }

    let mut process_line = |raw: &str, lines: &mut std::str::Lines<'_>| -> Result<()> {
        let line = strip_inline_comment(raw);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        if let Some(v) = parse_title_stmt(trimmed) {
            db.set_title(v);
            return Ok(());
        }
        if let Some(v) = parse_acc_title_stmt(trimmed) {
            db.set_acc_title(v);
            return Ok(());
        }
        if let Some(v) = parse_acc_descr_stmt_single(trimmed) {
            db.set_acc_descr(v);
            return Ok(());
        }
        if let Some(v) = parse_acc_descr_block(lines, trimmed) {
            db.set_acc_descr(v);
            return Ok(());
        }

        if parse_group_stmt(&mut db, trimmed)? {
            return Ok(());
        }
        if parse_service_stmt(&mut db, trimmed)? {
            return Ok(());
        }
        if parse_junction_stmt(&mut db, trimmed)? {
            return Ok(());
        }
        if parse_edge_stmt(&mut db, trimmed)? {
            return Ok(());
        }

        Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("unrecognized statement: {trimmed}"),
        })
    };

    if let Some(tail) = &header_tail {
        process_line(tail, &mut lines)?;
    }

    while let Some(line) = lines.next() {
        process_line(line, &mut lines)?;
    }

    let mut config = crate::config::clone_value_nonrecursive(meta.effective_config.as_value());
    if meta.config.as_value().get("layout").is_none() {
        if let Some(obj) = config.as_object_mut() {
            obj.insert("layout".to_string(), Value::String("dagre".to_string()));
        }
    }

    let groups = db.groups_json();
    let nodes = db.nodes_json();
    let services = db.services_json();
    let junctions = db.junctions_json();
    let edges = db.edges_json();

    let mut out = serde_json::Map::with_capacity(10);
    out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
    out.insert(
        "title".to_string(),
        if db.title.is_empty() {
            Value::Null
        } else {
            Value::String(db.title.clone())
        },
    );
    out.insert(
        "accTitle".to_string(),
        if db.acc_title.is_empty() {
            Value::Null
        } else {
            Value::String(db.acc_title.clone())
        },
    );
    out.insert(
        "accDescr".to_string(),
        if db.acc_descr.is_empty() {
            Value::Null
        } else {
            Value::String(db.acc_descr.clone())
        },
    );
    out.insert("groups".to_string(), Value::Array(groups));
    out.insert("nodes".to_string(), Value::Array(nodes));
    out.insert("services".to_string(), Value::Array(services));
    out.insert("junctions".to_string(), Value::Array(junctions));
    out.insert("edges".to_string(), Value::Array(edges));
    out.insert("config".to_string(), config);
    Ok(Value::Object(out))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureDiagramRenderModel {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(default, rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default)]
    pub nodes: Vec<ArchitectureRenderNode>,
    #[serde(default)]
    pub groups: Vec<ArchitectureRenderGroup>,
    #[serde(default)]
    pub edges: Vec<ArchitectureRenderEdge>,
}

impl ArchitectureDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArchitectureRenderNodeType {
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "junction")]
    Junction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureRenderNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: ArchitectureRenderNodeType,
    #[serde(default)]
    pub edge_indices: Vec<usize>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default, rename = "iconText")]
    pub icon_text: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, rename = "in")]
    pub in_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureRenderGroup {
    pub id: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default, rename = "in")]
    pub in_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureRenderEdge {
    #[serde(rename = "lhsId")]
    pub lhs_id: String,
    #[serde(rename = "lhsDir")]
    pub lhs_dir: char,
    #[serde(default, rename = "lhsInto")]
    pub lhs_into: Option<bool>,
    #[serde(default, rename = "lhsGroup")]
    pub lhs_group: Option<bool>,
    #[serde(rename = "rhsId")]
    pub rhs_id: String,
    #[serde(rename = "rhsDir")]
    pub rhs_dir: char,
    #[serde(default, rename = "rhsInto")]
    pub rhs_into: Option<bool>,
    #[serde(default, rename = "rhsGroup")]
    pub rhs_group: Option<bool>,
    #[serde(default)]
    pub title: Option<String>,
}

pub fn parse_architecture_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<ArchitectureDiagramRenderModel> {
    let mut db = ArchitectureDb::default();
    db.clear();

    let mut lines = code.lines();
    let mut found_header = false;
    let mut header_tail: Option<String> = None;
    for line in lines.by_ref() {
        let t = strip_inline_comment(line);
        let trimmed = t.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("architecture-beta") {
            let rest = rest.trim_start();
            if !rest.is_empty() {
                header_tail = Some(rest.to_string());
            }
            found_header = true;
            break;
        }
        break;
    }

    if !found_header {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected architecture-beta header".to_string(),
        });
    }

    let mut process_line = |raw: &str, lines: &mut std::str::Lines<'_>| -> Result<()> {
        let line = strip_inline_comment(raw);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        if let Some(v) = parse_title_stmt(trimmed) {
            db.set_title(v);
            return Ok(());
        }
        if let Some(v) = parse_acc_title_stmt(trimmed) {
            db.set_acc_title(v);
            return Ok(());
        }
        if let Some(v) = parse_acc_descr_stmt_single(trimmed) {
            db.set_acc_descr(v);
            return Ok(());
        }
        if let Some(v) = parse_acc_descr_block(lines, trimmed) {
            db.set_acc_descr(v);
            return Ok(());
        }

        if parse_group_stmt(&mut db, trimmed)? {
            return Ok(());
        }
        if parse_service_stmt(&mut db, trimmed)? {
            return Ok(());
        }
        if parse_junction_stmt(&mut db, trimmed)? {
            return Ok(());
        }
        if parse_edge_stmt(&mut db, trimmed)? {
            return Ok(());
        }

        Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("unrecognized statement: {trimmed}"),
        })
    };

    if let Some(tail) = &header_tail {
        process_line(tail, &mut lines)?;
    }

    while let Some(line) = lines.next() {
        process_line(line, &mut lines)?;
    }

    Ok(db.render_model())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, ParseOptions};
    use futures::executor::block_on;

    fn parse(text: &str) -> Value {
        let engine = Engine::new();
        block_on(engine.parse_diagram(text, ParseOptions::default()))
            .unwrap()
            .unwrap()
            .model
    }

    #[test]
    fn architecture_accepts_header_only() {
        let _ = parse("architecture-beta");
    }

    #[test]
    fn architecture_accepts_simple_service() {
        let model = parse("architecture-beta\n  service db\n");
        assert_eq!(model["services"].as_array().unwrap().len(), 1);
        assert_eq!(model["services"][0]["id"].as_str().unwrap(), "db");
    }

    #[test]
    fn architecture_title_on_first_line() {
        let model = parse("architecture-beta title Simple Architecture Diagram");
        assert_eq!(
            model["title"].as_str().unwrap(),
            "Simple Architecture Diagram"
        );
    }

    #[test]
    fn architecture_title_on_another_line() {
        let model = parse("architecture-beta\n  title Simple Architecture Diagram\n");
        assert_eq!(
            model["title"].as_str().unwrap(),
            "Simple Architecture Diagram"
        );
    }

    #[test]
    fn architecture_accessibility_title_and_descr() {
        let model = parse(
            "architecture-beta\n  accTitle: Accessibility Title\n  accDescr: Accessibility Description\n",
        );
        assert_eq!(model["accTitle"].as_str().unwrap(), "Accessibility Title");
        assert_eq!(
            model["accDescr"].as_str().unwrap(),
            "Accessibility Description"
        );
    }

    #[test]
    fn architecture_multiline_acc_descr() {
        let model = parse("architecture-beta\n  accDescr {\n    Accessibility Description\n  }\n");
        assert_eq!(
            model["accDescr"].as_str().unwrap(),
            "Accessibility Description"
        );
    }

    #[test]
    fn architecture_edge_with_ports_is_parsed() {
        let model =
            parse("architecture-beta\n  service db\n  service server\n  db:L -- R:server\n");
        let edges = model["edges"].as_array().unwrap();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0]["lhsId"].as_str().unwrap(), "db");
        assert_eq!(edges[0]["lhsDir"].as_str().unwrap(), "L");
        assert_eq!(edges[0]["rhsId"].as_str().unwrap(), "server");
        assert_eq!(edges[0]["rhsDir"].as_str().unwrap(), "R");
    }

    #[test]
    fn architecture_edge_with_title_is_parsed() {
        let model = parse("architecture-beta\n  service a\n  service b\n  a:L -[Label]- R:b\n");
        let edges = model["edges"].as_array().unwrap();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0]["title"].as_str().unwrap(), "Label");
        assert_eq!(edges[0]["lhsDir"].as_str().unwrap(), "L");
        assert_eq!(edges[0]["rhsDir"].as_str().unwrap(), "R");
    }

    #[test]
    fn architecture_rejects_legacy_edge_shorthand() {
        let engine = Engine::new();
        let err = block_on(engine.parse_diagram(
            "architecture-beta\n  service a\n  service b\n  a (T--B) b\n",
            ParseOptions::default(),
        ))
        .unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("expected ':' for lhs port") || msg.contains("unrecognized"));
    }
}
