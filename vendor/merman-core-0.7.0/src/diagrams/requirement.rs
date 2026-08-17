use crate::{Error, ParseMetadata, Result};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, HashMap};
use std::iter::Peekable;
use std::str::Lines;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequirementRenderNode {
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default, rename = "requirementId")]
    pub requirement_id: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub risk: String,
    #[serde(default, rename = "verifyMethod")]
    pub verify_method: String,
    #[serde(default)]
    pub css_styles: Vec<String>,
    #[serde(default)]
    pub classes: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequirementRenderElement {
    pub name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    #[serde(default, rename = "docRef")]
    pub doc_ref: String,
    #[serde(default)]
    pub css_styles: Vec<String>,
    #[serde(default)]
    pub classes: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequirementRenderRelationship {
    #[serde(rename = "type")]
    pub rel_type: String,
    pub src: String,
    pub dst: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequirementRenderClass {
    pub id: String,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default, rename = "textStyles")]
    pub text_styles: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequirementDiagramRenderModel {
    #[serde(default, rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(default, rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default)]
    pub direction: String,
    #[serde(default)]
    pub requirements: Vec<RequirementRenderNode>,
    #[serde(default)]
    pub elements: Vec<RequirementRenderElement>,
    #[serde(default)]
    pub relationships: Vec<RequirementRenderRelationship>,
    #[serde(default)]
    pub classes: BTreeMap<String, RequirementRenderClass>,
}

impl RequirementDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone)]
struct RequirementBuilder {
    requirement_id: String,
    text: String,
    risk: String,
    verify_method: String,
}

impl RequirementBuilder {
    fn new() -> Self {
        Self {
            requirement_id: String::new(),
            text: String::new(),
            risk: String::new(),
            verify_method: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ElementBuilder {
    element_type: String,
    doc_ref: String,
}

impl ElementBuilder {
    fn new() -> Self {
        Self {
            element_type: String::new(),
            doc_ref: String::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct RequirementDb {
    direction: String,
    relations: Vec<RequirementRenderRelationship>,

    requirements: HashMap<String, RequirementRenderNode>,
    requirement_order: Vec<String>,

    elements: HashMap<String, RequirementRenderElement>,
    element_order: Vec<String>,

    classes: HashMap<String, RequirementRenderClass>,
}

impl RequirementDb {
    fn new() -> Self {
        Self {
            direction: "TB".to_string(),
            relations: Vec::new(),
            requirements: HashMap::new(),
            requirement_order: Vec::new(),
            elements: HashMap::new(),
            element_order: Vec::new(),
            classes: HashMap::new(),
        }
    }

    fn set_direction(&mut self, dir: &str) {
        self.direction = dir.to_string();
    }

    fn add_requirement(&mut self, name: &str, requirement_type: &str, b: RequirementBuilder) {
        if self.requirements.contains_key(name) {
            return;
        }
        self.requirement_order.push(name.to_string());
        self.requirements.insert(
            name.to_string(),
            RequirementRenderNode {
                name: name.to_string(),
                node_type: requirement_type.to_string(),
                requirement_id: b.requirement_id,
                text: b.text,
                risk: b.risk,
                verify_method: b.verify_method,
                css_styles: Vec::new(),
                classes: vec!["default".to_string()],
            },
        );
    }

    fn add_element(&mut self, name: &str, b: ElementBuilder) {
        if self.elements.contains_key(name) {
            return;
        }
        self.element_order.push(name.to_string());
        self.elements.insert(
            name.to_string(),
            RequirementRenderElement {
                name: name.to_string(),
                element_type: b.element_type,
                doc_ref: b.doc_ref,
                css_styles: Vec::new(),
                classes: vec!["default".to_string()],
            },
        );
    }

    fn add_relationship(&mut self, relationship_type: &str, src: &str, dst: &str) {
        self.relations.push(RequirementRenderRelationship {
            rel_type: relationship_type.to_string(),
            src: src.to_string(),
            dst: dst.to_string(),
        });
    }

    fn set_css_style(&mut self, ids: &[String], styles: &[String]) {
        for id in ids {
            let node_req = self.requirements.get_mut(id);
            if let Some(node) = node_req {
                push_styles(&mut node.css_styles, styles);
                continue;
            }
            let node_el = self.elements.get_mut(id);
            if let Some(node) = node_el {
                push_styles(&mut node.css_styles, styles);
                continue;
            }
        }
    }

    fn set_class(&mut self, ids: &[String], class_names: &[String]) {
        for id in ids {
            if let Some(node) = self.requirements.get_mut(id) {
                for cls in class_names {
                    node.classes.push(cls.clone());
                    if let Some(def) = self.classes.get(cls) {
                        node.css_styles.extend(def.styles.iter().cloned());
                    }
                }
                continue;
            }
            if let Some(node) = self.elements.get_mut(id) {
                for cls in class_names {
                    node.classes.push(cls.clone());
                    if let Some(def) = self.classes.get(cls) {
                        node.css_styles.extend(def.styles.iter().cloned());
                    }
                }
            }
        }
    }

    fn define_class(&mut self, ids: &[String], styles: &[String]) {
        for id in ids {
            let style_class =
                self.classes
                    .entry(id.to_string())
                    .or_insert_with(|| RequirementRenderClass {
                        id: id.to_string(),
                        styles: Vec::new(),
                        text_styles: Vec::new(),
                    });

            for s in styles {
                if s.contains("color") {
                    let new_style = s.replacen("fill", "bgFill", 1);
                    style_class.text_styles.push(new_style);
                }
                style_class.styles.push(s.clone());
            }

            for req_name in &self.requirement_order {
                if let Some(req) = self.requirements.get_mut(req_name) {
                    if req.classes.iter().any(|c| c == id) {
                        req.css_styles.extend(
                            styles
                                .iter()
                                .flat_map(|s| s.split(','))
                                .map(|s| s.to_string()),
                        );
                    }
                }
            }
            for el_name in &self.element_order {
                if let Some(el) = self.elements.get_mut(el_name) {
                    if el.classes.iter().any(|c| c == id) {
                        el.css_styles.extend(
                            styles
                                .iter()
                                .flat_map(|s| s.split(','))
                                .map(|s| s.to_string()),
                        );
                    }
                }
            }
        }
    }

    fn to_render_model(
        &self,
        acc_title: Option<String>,
        acc_descr: Option<String>,
    ) -> RequirementDiagramRenderModel {
        let requirements = self
            .requirement_order
            .iter()
            .filter_map(|k| self.requirements.get(k))
            .cloned()
            .collect::<Vec<_>>();

        let elements = self
            .element_order
            .iter()
            .filter_map(|k| self.elements.get(k))
            .cloned()
            .collect::<Vec<_>>();

        let mut classes = BTreeMap::new();
        for (k, c) in &self.classes {
            classes.insert(k.clone(), c.clone());
        }

        RequirementDiagramRenderModel {
            acc_title,
            acc_descr,
            direction: self.direction.clone(),
            requirements,
            elements,
            relationships: self.relations.clone(),
            classes,
        }
    }
}

fn push_styles(out: &mut Vec<String>, styles: &[String]) {
    for s in styles {
        if s.contains(',') {
            out.extend(s.split(',').map(|p| p.to_string()));
        } else {
            out.push(s.to_string());
        }
    }
}

pub fn parse_requirement(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let model = parse_requirement_model(code, meta)?;
    Ok(requirement_model_to_value(model, meta))
}

pub fn parse_requirement_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<RequirementDiagramRenderModel> {
    parse_requirement_model(code, meta)
}

fn requirement_model_to_value(model: RequirementDiagramRenderModel, meta: &ParseMetadata) -> Value {
    let mut out = Map::with_capacity(9);
    out.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
    out.insert("accTitle".to_string(), json!(model.acc_title));
    out.insert("accDescr".to_string(), json!(model.acc_descr));
    out.insert("direction".to_string(), json!(model.direction));
    out.insert("requirements".to_string(), json!(model.requirements));
    out.insert("elements".to_string(), json!(model.elements));
    out.insert("relationships".to_string(), json!(model.relationships));
    out.insert("classes".to_string(), json!(model.classes));
    out.insert(
        "config".to_string(),
        crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
    );
    Value::Object(out)
}

fn parse_requirement_model(
    code: &str,
    meta: &ParseMetadata,
) -> Result<RequirementDiagramRenderModel> {
    let mut db = RequirementDb::new();

    let mut acc_title: Option<String> = None;
    let mut acc_descr: Option<String> = None;

    let mut lines = code.lines().peekable();
    let mut saw_header = false;

    while let Some(raw) = lines.next() {
        let line = strip_inline_comment(raw);
        let t = line.trim();
        if t.is_empty() {
            continue;
        }

        if try_parse_acc_title(t, &mut acc_title) {
            continue;
        }
        if let Some(v) = try_parse_acc_descr(t, &mut lines)? {
            acc_descr = Some(v);
            continue;
        }

        if !saw_header {
            if t.eq_ignore_ascii_case("requirementDiagram") {
                saw_header = true;
                continue;
            }
            return Err(Error::DiagramParse {
                diagram_type: meta.diagram_type.clone(),
                message: "expected requirementDiagram".to_string(),
            });
        }

        if let Some(dir) = parse_direction(t) {
            db.set_direction(dir);
            continue;
        }

        if let Some((name, ty, classes)) = parse_requirement_def_open(t)? {
            let b = parse_requirement_body(&mut lines)?;
            db.add_requirement(&name, &ty, b);
            if let Some(classes) = classes {
                db.set_class(&[name], &classes);
            }
            continue;
        }

        if let Some((name, classes)) = parse_element_def_open(t)? {
            let b = parse_element_body(&mut lines)?;
            db.add_element(&name, b);
            if let Some(classes) = classes {
                db.set_class(&[name], &classes);
            }
            continue;
        }

        if let Some((target, classes)) = parse_shorthand_class_stmt(t)? {
            db.set_class(&[target], &classes);
            continue;
        }

        if let Some((ids, styles)) = parse_style_stmt(t)? {
            db.set_css_style(&ids, &styles);
            continue;
        }

        if let Some((ids, styles)) = parse_classdef_stmt(t)? {
            db.define_class(&ids, &styles);
            continue;
        }

        if let Some((ids, classes)) = parse_class_stmt(t)? {
            db.set_class(&ids, &classes);
            continue;
        }

        if let Some((rel, src, dst)) = parse_relationship_stmt(t)? {
            db.add_relationship(&rel, &src, &dst);
            continue;
        }

        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("unexpected requirement statement: {t}"),
        });
    }

    if !saw_header {
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: "expected requirementDiagram".to_string(),
        });
    }

    Ok(db.to_render_model(acc_title, acc_descr))
}

fn strip_inline_comment(line: &str) -> String {
    let lowered = line.trim_start().to_ascii_lowercase();
    if lowered.starts_with("style")
        || lowered.starts_with("classdef")
        || lowered.starts_with("class ")
        || lowered == "class"
    {
        return line.to_string();
    }

    let mut in_quotes = false;
    let mut idx = 0usize;
    let bytes = line.as_bytes();
    while idx < bytes.len() {
        let b = bytes[idx];
        if b == b'"' {
            in_quotes = !in_quotes;
            idx += 1;
            continue;
        }
        if !in_quotes {
            if b == b'#' {
                return line[..idx].to_string();
            }
            if b == b'%' && idx + 1 < bytes.len() && bytes[idx + 1] == b'%' {
                return line[..idx].to_string();
            }
        }
        idx += 1;
    }
    line.to_string()
}

fn try_parse_acc_title(t: &str, out: &mut Option<String>) -> bool {
    let t = t.trim_start();
    if !t.to_ascii_lowercase().starts_with("acctitle") {
        return false;
    }
    let rest = &t["acctitle".len()..];
    let rest = rest.trim_start();
    let Some(val) = rest.strip_prefix(':') else {
        return false;
    };
    let val = val.trim();
    *out = Some(val.to_string());
    true
}

fn try_parse_acc_descr<'a>(t: &str, lines: &mut Peekable<Lines<'a>>) -> Result<Option<String>> {
    let t = t.trim_start();
    if !t.to_ascii_lowercase().starts_with("accdescr") {
        return Ok(None);
    }

    let rest = &t["accdescr".len()..];
    let rest = rest.trim_start();
    if let Some(after) = rest.strip_prefix(':') {
        let val = after.trim();
        return Ok(Some(val.to_string()));
    }

    if let Some(rest) = rest.strip_prefix('{') {
        let mut buf = String::new();
        let mut after = rest.to_string();
        if let Some(end) = after.find('}') {
            after.truncate(end);
            return Ok(Some(after.trim().to_string()));
        }
        if !after.trim().is_empty() {
            buf.push_str(after.trim_end());
        }

        for raw in lines.by_ref() {
            let t = raw.trim_end();
            if let Some(pos) = t.find('}') {
                let part = t[..pos].trim_end();
                if !part.is_empty() {
                    if !buf.is_empty() {
                        buf.push('\n');
                    }
                    buf.push_str(part.trim_start());
                }
                break;
            }

            let content = t.trim_end();
            if !content.trim().is_empty() {
                if !buf.is_empty() {
                    buf.push('\n');
                }
                buf.push_str(content.trim_start());
            } else if !buf.is_empty() {
                buf.push('\n');
            }
        }

        return Ok(Some(buf.trim().to_string()));
    }

    Ok(None)
}

fn parse_direction(t: &str) -> Option<&'static str> {
    let tokens: Vec<&str> = t.split_whitespace().collect();
    for i in 0..tokens.len() {
        if tokens[i].eq_ignore_ascii_case("direction") {
            let dir = tokens.get(i + 1).copied()?;
            return match dir.to_ascii_uppercase().as_str() {
                "TB" => Some("TB"),
                "BT" => Some("BT"),
                "LR" => Some("LR"),
                "RL" => Some("RL"),
                _ => None,
            };
        }
    }
    None
}

fn parse_requirement_def_open(t: &str) -> Result<Option<RequirementDefOpen>> {
    let t = t.trim();
    if !t.ends_with('{') {
        return Ok(None);
    }

    let without_brace = t[..t.len() - 1].trim_end();
    let (ty_raw, rest) = split_first_word(without_brace).ok_or_else(|| Error::DiagramParse {
        diagram_type: "requirement".to_string(),
        message: "invalid requirement definition".to_string(),
    })?;

    let requirement_type = match ty_raw.to_ascii_lowercase().as_str() {
        "requirement" => "Requirement",
        "functionalrequirement" => "Functional Requirement",
        "interfacerequirement" => "Interface Requirement",
        "performancerequirement" => "Performance Requirement",
        "physicalrequirement" => "Physical Requirement",
        "designconstraint" => "Design Constraint",
        _ => return Ok(None),
    }
    .to_string();

    let (name, classes) = split_name_and_classes(rest.trim())?;
    if name.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: "requirement name is empty".to_string(),
        });
    }
    Ok(Some((name, requirement_type, classes)))
}

type RequirementDefOpen = (String, String, Option<Vec<String>>);

fn parse_element_def_open(t: &str) -> Result<Option<(String, Option<Vec<String>>)>> {
    let t = t.trim();
    if !t.ends_with('{') {
        return Ok(None);
    }

    let without_brace = t[..t.len() - 1].trim_end();
    let (kw, rest) = split_first_word(without_brace).ok_or_else(|| Error::DiagramParse {
        diagram_type: "requirement".to_string(),
        message: "invalid element definition".to_string(),
    })?;
    if !kw.eq_ignore_ascii_case("element") {
        return Ok(None);
    }

    let (name, classes) = split_name_and_classes(rest.trim())?;
    if name.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: "element name is empty".to_string(),
        });
    }
    Ok(Some((name, classes)))
}

fn split_first_word(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();
    if input.is_empty() {
        return None;
    }
    let mut iter = input.splitn(2, char::is_whitespace);
    let first = iter.next()?;
    let rest = iter.next().unwrap_or("");
    Some((first, rest))
}

fn split_name_and_classes(input: &str) -> Result<(String, Option<Vec<String>>)> {
    let input = input.trim();
    if input.is_empty() {
        return Ok((String::new(), None));
    }

    if let Some(pos) = input.find(":::") {
        let name_raw = input[..pos].trim_end();
        let classes_raw = input[pos + 3..].trim();
        let (name, _) = parse_id_or_name(name_raw)?;
        let classes = parse_id_list_all(classes_raw)?;
        return Ok((name, Some(classes)));
    }

    let (name, _) = parse_id_or_name(input)?;
    Ok((name, None))
}

fn parse_id_or_name(input: &str) -> Result<(String, &str)> {
    let input = input.trim_start();
    if input.starts_with('"') {
        if let Some((val, rest)) = parse_quoted_prefix(input) {
            return Ok((val, rest));
        }
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: "unterminated string".to_string(),
        });
    }
    Ok((input.trim().to_string(), ""))
}

fn parse_quoted_prefix(input: &str) -> Option<(String, &str)> {
    let mut chars = input.chars();
    if chars.next()? != '"' {
        return None;
    }
    let mut out = String::new();
    let mut idx = 1usize;
    for c in chars {
        idx += c.len_utf8();
        if c == '"' {
            return Some((out, &input[idx..]));
        }
        out.push(c);
    }
    None
}

fn parse_requirement_body(lines: &mut Peekable<Lines<'_>>) -> Result<RequirementBuilder> {
    let mut b = RequirementBuilder::new();
    for raw in lines.by_ref() {
        let line = strip_inline_comment(raw);
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t == "}" {
            return Ok(b);
        }

        let Some((k, v)) = split_key_value(t) else {
            return Err(Error::DiagramParse {
                diagram_type: "requirement".to_string(),
                message: format!("invalid requirement body line: {t}"),
            });
        };
        let key = k.to_ascii_lowercase();
        let value = parse_simple_value(v)?;
        match key.as_str() {
            "id" => b.requirement_id = value,
            "text" => b.text = value,
            "risk" => b.risk = normalize_risk(&value)?,
            "verifymethod" => b.verify_method = normalize_verify_method(&value)?,
            _ => {
                return Err(Error::DiagramParse {
                    diagram_type: "requirement".to_string(),
                    message: format!("unexpected requirement body key: {k}"),
                });
            }
        }
    }

    Err(Error::DiagramParse {
        diagram_type: "requirement".to_string(),
        message: "unterminated requirement block".to_string(),
    })
}

fn parse_element_body(lines: &mut Peekable<Lines<'_>>) -> Result<ElementBuilder> {
    let mut b = ElementBuilder::new();
    for raw in lines.by_ref() {
        let line = strip_inline_comment(raw);
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if t == "}" {
            return Ok(b);
        }

        let Some((k, v)) = split_key_value(t) else {
            return Err(Error::DiagramParse {
                diagram_type: "requirement".to_string(),
                message: format!("invalid element body line: {t}"),
            });
        };
        let key = k.to_ascii_lowercase();
        let value = parse_simple_value(v)?;
        match key.as_str() {
            "type" => b.element_type = value,
            "docref" => b.doc_ref = value,
            _ => {
                return Err(Error::DiagramParse {
                    diagram_type: "requirement".to_string(),
                    message: format!("unexpected element body key: {k}"),
                });
            }
        }
    }

    Err(Error::DiagramParse {
        diagram_type: "requirement".to_string(),
        message: "unterminated element block".to_string(),
    })
}

fn split_key_value(input: &str) -> Option<(&str, &str)> {
    let idx = input.find(':')?;
    let key = input[..idx].trim();
    let value = input[idx + 1..].trim();
    if key.is_empty() {
        return None;
    }
    Some((key, value))
}

fn parse_simple_value(input: &str) -> Result<String> {
    let input = input.trim();
    if input.starts_with('"') {
        if let Some((val, rest)) = parse_quoted_prefix(input) {
            if !rest.trim().is_empty() {
                return Err(Error::DiagramParse {
                    diagram_type: "requirement".to_string(),
                    message: format!("unexpected trailing tokens after string: {}", rest.trim()),
                });
            }
            return Ok(val.trim().to_string());
        }
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: "unterminated string".to_string(),
        });
    }
    Ok(input.trim().to_string())
}

fn normalize_risk(input: &str) -> Result<String> {
    match input.trim().to_ascii_lowercase().as_str() {
        "low" => Ok("Low".to_string()),
        "medium" => Ok("Medium".to_string()),
        "high" => Ok("High".to_string()),
        other => Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: format!("invalid risk level: {other}"),
        }),
    }
}

fn normalize_verify_method(input: &str) -> Result<String> {
    match input.trim().to_ascii_lowercase().as_str() {
        "analysis" => Ok("Analysis".to_string()),
        "demonstration" => Ok("Demonstration".to_string()),
        "inspection" => Ok("Inspection".to_string()),
        "test" => Ok("Test".to_string()),
        other => Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: format!("invalid verify method: {other}"),
        }),
    }
}

fn parse_shorthand_class_stmt(t: &str) -> Result<Option<(String, Vec<String>)>> {
    let t = t.trim();
    if t.is_empty() || t.ends_with('{') {
        return Ok(None);
    }
    let Some(pos) = t.find(":::") else {
        return Ok(None);
    };
    let left = t[..pos].trim_end();
    let right = t[pos + 3..].trim_start();
    if left.is_empty() || right.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: format!("invalid class shorthand statement: {t}"),
        });
    }
    let (target, _) = parse_id_or_name(left)?;
    let classes = parse_id_list_all(right)?;
    Ok(Some((target, classes)))
}

fn parse_style_stmt(t: &str) -> Result<Option<(Vec<String>, Vec<String>)>> {
    let t = t.trim_start();
    if !t.to_ascii_lowercase().starts_with("style") {
        return Ok(None);
    }
    let rest = &t["style".len()..];
    let rest = rest.trim_start();
    let (ids, styles_str) = split_list_and_rest(rest)?;
    let styles = split_csv(styles_str);
    if ids.is_empty() || styles.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: format!("invalid style statement: {t}"),
        });
    }
    Ok(Some((ids, styles)))
}

fn parse_classdef_stmt(t: &str) -> Result<Option<(Vec<String>, Vec<String>)>> {
    let t = t.trim_start();
    if !t.to_ascii_lowercase().starts_with("classdef") {
        return Ok(None);
    }
    let rest = &t["classdef".len()..];
    let rest = rest.trim_start();
    let (ids, styles_str) = split_list_and_rest(rest)?;
    let styles = split_csv(styles_str);
    if ids.is_empty() || styles.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: format!("invalid classDef statement: {t}"),
        });
    }
    Ok(Some((ids, styles)))
}

fn parse_class_stmt(t: &str) -> Result<Option<(Vec<String>, Vec<String>)>> {
    let t = t.trim_start();
    if !t.to_ascii_lowercase().starts_with("class") {
        return Ok(None);
    }
    if t.to_ascii_lowercase().starts_with("classdef") {
        return Ok(None);
    }
    let rest = &t["class".len()..];
    let rest = rest.trim_start();
    let (ids, classes_str) = split_list_and_rest(rest)?;
    let classes = parse_id_list_all(classes_str)?;
    if ids.is_empty() || classes.is_empty() {
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: format!("invalid class statement: {t}"),
        });
    }
    Ok(Some((ids, classes)))
}

fn parse_relationship_stmt(t: &str) -> Result<Option<(String, String, String)>> {
    let t = t.trim();
    if t.is_empty() {
        return Ok(None);
    }

    if let Some(pos) = t.find("<-") {
        let left = t[..pos].trim_end();
        let rest = t[pos + 2..].trim_start();
        let (rel, right) = split_once_dash(rest)?;
        let relationship = normalize_relationship(rel)?;
        if relationship.is_empty() {
            return Ok(None);
        }
        let src = parse_simple_value(right)?;
        let dst = parse_simple_value(left)?;
        return Ok(Some((relationship, src, dst)));
    }

    if let Some(pos) = t.find("->") {
        let right = t[pos + 2..].trim_start();
        let left_part = t[..pos].trim_end();
        let (src, rel) = split_once_dash(left_part)?;
        let relationship = normalize_relationship(rel)?;
        if relationship.is_empty() {
            return Ok(None);
        }
        let src = parse_simple_value(src)?;
        let dst = parse_simple_value(right)?;
        return Ok(Some((relationship, src, dst)));
    }

    Ok(None)
}

fn split_once_dash(input: &str) -> Result<(&str, &str)> {
    let Some(idx) = input.find('-') else {
        return Err(Error::DiagramParse {
            diagram_type: "requirement".to_string(),
            message: format!("invalid relationship statement: {input}"),
        });
    };
    Ok((input[..idx].trim(), input[idx + 1..].trim()))
}

fn normalize_relationship(input: &str) -> Result<String> {
    let rel = input.trim().to_ascii_lowercase();
    match rel.as_str() {
        "contains" | "copies" | "derives" | "satisfies" | "verifies" | "refines" | "traces" => {
            Ok(rel)
        }
        _ => Ok(String::new()),
    }
}

fn split_list_and_rest(input: &str) -> Result<(Vec<String>, &str)> {
    let mut cur = input.trim_start();
    let mut items = Vec::new();

    loop {
        cur = cur.trim_start();
        if cur.is_empty() {
            break;
        }

        let (item, rest) = if cur.starts_with('"') {
            parse_quoted_prefix(cur).ok_or_else(|| Error::DiagramParse {
                diagram_type: "requirement".to_string(),
                message: "unterminated string".to_string(),
            })?
        } else {
            let mut end = 0usize;
            for (i, c) in cur.char_indices() {
                if c == ',' || c.is_whitespace() {
                    break;
                }
                end = i + c.len_utf8();
            }
            if end == 0 {
                return Err(Error::DiagramParse {
                    diagram_type: "requirement".to_string(),
                    message: "expected identifier".to_string(),
                });
            }
            (cur[..end].to_string(), &cur[end..])
        };

        items.push(item);
        cur = rest.trim_start();
        if cur.starts_with(',') {
            cur = &cur[1..];
            continue;
        }
        break;
    }

    let rest = cur.trim_start();
    Ok((items, rest))
}

fn parse_id_list_all(input: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let mut cur = input.trim_start();
    while !cur.is_empty() {
        let (item, rest) = if cur.starts_with('"') {
            parse_quoted_prefix(cur).ok_or_else(|| Error::DiagramParse {
                diagram_type: "requirement".to_string(),
                message: "unterminated string".to_string(),
            })?
        } else {
            let mut end = cur.len();
            for (i, c) in cur.char_indices() {
                if c == ',' {
                    end = i;
                    break;
                }
            }
            (cur[..end].trim().to_string(), &cur[end..])
        };
        if !item.is_empty() {
            out.push(item);
        }
        cur = rest.trim_start();
        if cur.starts_with(',') {
            cur = &cur[1..];
            continue;
        }
        break;
    }
    Ok(out)
}

fn split_csv(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, ParseOptions};
    use futures::executor::block_on;
    use serde_json::json;

    fn parse(text: &str) -> Value {
        let engine = Engine::new();
        block_on(engine.parse_diagram(text, ParseOptions::default()))
            .unwrap()
            .unwrap()
            .model
    }

    #[test]
    fn requirement_full_requirement_definition_is_parsed() {
        let model = parse(
            r#"requirementDiagram

requirement test_req {
  id: test_id
  text: the test text.
  risk: high
  verifymethod: analysis
}
"#,
        );

        assert_eq!(model["requirements"].as_array().unwrap().len(), 1);
        assert_eq!(model["requirements"][0]["name"], json!("test_req"));
        assert_eq!(model["requirements"][0]["type"], json!("Requirement"));
        assert_eq!(model["requirements"][0]["requirementId"], json!("test_id"));
        assert_eq!(model["requirements"][0]["text"], json!("the test text."));
        assert_eq!(model["requirements"][0]["risk"], json!("High"));
        assert_eq!(model["requirements"][0]["verifyMethod"], json!("Analysis"));
        assert_eq!(model["elements"].as_array().unwrap().len(), 0);
        assert_eq!(model["relationships"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn requirement_full_element_definition_is_parsed() {
        let model = parse(
            r#"requirementDiagram

element test_el {
  type: test_type
  docref: test_ref
}
"#,
        );

        assert_eq!(model["requirements"].as_array().unwrap().len(), 0);
        assert_eq!(model["elements"].as_array().unwrap().len(), 1);
        assert_eq!(model["elements"][0]["name"], json!("test_el"));
        assert_eq!(model["elements"][0]["type"], json!("test_type"));
        assert_eq!(model["elements"][0]["docRef"], json!("test_ref"));
        assert_eq!(model["relationships"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn requirement_acc_title_and_acc_descr_are_parsed() {
        let model = parse(
            r#"requirementDiagram
accTitle: test title
accDescr: my chart description
element test_name {
  type: test_type
  docref: test_ref
}
"#,
        );

        assert_eq!(model["accTitle"], json!("test title"));
        assert_eq!(model["accDescr"], json!("my chart description"));
    }

    #[test]
    fn requirement_multiline_acc_descr_is_parsed() {
        let model = parse(
            r#"requirementDiagram
accTitle: test title
accDescr {
  my chart description
line 2
}
element test_name {
  type: test_type
  docref: test_ref
}
"#,
        );

        assert_eq!(model["accTitle"], json!("test title"));
        assert_eq!(model["accDescr"], json!("my chart description\nline 2"));
    }

    #[test]
    fn requirement_relationship_is_parsed() {
        let model = parse(
            r#"requirementDiagram

a - contains -> b
"#,
        );
        assert_eq!(model["relationships"].as_array().unwrap().len(), 1);
        assert_eq!(model["relationships"][0]["type"], json!("contains"));
        assert_eq!(model["relationships"][0]["src"], json!("a"));
        assert_eq!(model["relationships"][0]["dst"], json!("b"));
    }

    #[test]
    fn requirement_relationship_left_arrow_is_parsed() {
        let model = parse(
            r#"requirementDiagram

a <- contains - b
"#,
        );
        assert_eq!(model["relationships"].as_array().unwrap().len(), 1);
        assert_eq!(model["relationships"][0]["type"], json!("contains"));
        assert_eq!(model["relationships"][0]["src"], json!("b"));
        assert_eq!(model["relationships"][0]["dst"], json!("a"));
    }

    #[test]
    fn requirement_proto_and_constructor_ids_are_accepted() {
        for id in ["__proto__", "constructor"] {
            let model = parse(&format!(
                r#"requirementDiagram
requirement {id} {{
  id: 1
  text: the test text.
  risk: high
  verifymethod: test
}}
"#
            ));
            assert_eq!(model["requirements"].as_array().unwrap().len(), 1);
        }

        for id in ["__proto__", "constructor"] {
            let model = parse(&format!(
                r#"requirementDiagram
element {id} {{
  type: simulation
}}
"#
            ));
            assert_eq!(model["elements"].as_array().unwrap().len(), 1);
        }
    }

    #[test]
    fn requirement_style_statement_applies_to_requirement() {
        let model = parse(
            r#"requirementDiagram

requirement test_req {
}
style test_req fill:#f9f,stroke:#333,stroke-width:4px
"#,
        );

        assert_eq!(
            model["requirements"][0]["cssStyles"],
            json!(["fill:#f9f", "stroke:#333", "stroke-width:4px"])
        );
    }

    #[test]
    fn requirement_style_statement_applies_to_element() {
        let model = parse(
            r#"requirementDiagram

element test_element {
}
style test_element fill:#f9f,stroke:#333,stroke-width:4px
"#,
        );

        assert_eq!(
            model["elements"][0]["cssStyles"],
            json!(["fill:#f9f", "stroke:#333", "stroke-width:4px"])
        );
    }

    #[test]
    fn requirement_style_statement_applies_to_multiple_things() {
        let model = parse(
            r#"requirementDiagram

requirement test_requirement {
}
element test_element {
}
style test_requirement,test_element fill:#f9f,stroke:#333,stroke-width:4px
"#,
        );

        assert_eq!(
            model["requirements"][0]["cssStyles"],
            json!(["fill:#f9f", "stroke:#333", "stroke-width:4px"])
        );
        assert_eq!(
            model["elements"][0]["cssStyles"],
            json!(["fill:#f9f", "stroke:#333", "stroke-width:4px"])
        );
    }

    #[test]
    fn requirement_classdef_and_class_statement_are_parsed() {
        let model = parse(
            r#"requirementDiagram

requirement myReq {
}
classDef myClass fill:#f9f,stroke:#333,stroke-width:4px
class myReq myClass
"#,
        );

        assert_eq!(
            model["requirements"][0]["classes"],
            json!(["default", "myClass"])
        );
        assert_eq!(
            model["requirements"][0]["cssStyles"],
            json!(["fill:#f9f", "stroke:#333", "stroke-width:4px"])
        );
        assert_eq!(model["classes"]["myClass"]["id"], json!("myClass"));
        assert_eq!(
            model["classes"]["myClass"]["styles"],
            json!(["fill:#f9f", "stroke:#333", "stroke-width:4px"])
        );
        assert_eq!(model["classes"]["myClass"]["textStyles"], json!([]));
    }

    #[test]
    fn requirement_shorthand_class_statement_is_supported() {
        let model = parse(
            r#"requirementDiagram

requirement myReq {
}
classDef myClass fill:#f9f,stroke:#333,stroke-width:4px
myReq:::myClass
"#,
        );
        assert_eq!(
            model["requirements"][0]["classes"],
            json!(["default", "myClass"])
        );
    }

    #[test]
    fn requirement_shorthand_is_supported_in_definition() {
        let model = parse(
            r#"requirementDiagram

requirement myReq:::class1 {
}
element myElem:::class1,class2 {
}

classDef class1 fill:#f9f,stroke:#333,stroke-width:4px
classDef class2 color:blue
"#,
        );
        assert_eq!(
            model["requirements"][0]["classes"],
            json!(["default", "class1"])
        );
        assert_eq!(
            model["elements"][0]["classes"],
            json!(["default", "class1", "class2"])
        );
    }

    #[test]
    fn requirement_direction_is_parsed() {
        for dir in ["TB", "BT", "LR", "RL"] {
            let model = parse(&format!("requirementDiagram\n\ndirection {dir}\n"));
            assert_eq!(model["direction"], json!(dir));
        }
    }
}
