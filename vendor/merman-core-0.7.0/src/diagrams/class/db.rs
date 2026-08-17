use crate::common::parse_generic_types;
use crate::models::class_diagram as class_typed;
use crate::sanitize::sanitize_text;
use crate::utils::format_url;
use crate::{MermaidConfig, ParseMetadata};
use indexmap::IndexMap;
use serde_json::Value;

use super::ast::{Action, RelationData};
use super::{
    LINE_DOTTED, LINE_SOLID, MERMAID_DOM_ID_PREFIX, REL_AGGREGATION, REL_COMPOSITION,
    REL_DEPENDENCY, REL_EXTENSION, REL_LOLLIPOP, REL_NONE,
};

#[derive(Debug, Clone)]
struct ClassMember {
    member_type: String,
    visibility: String,
    id: String,
    classifier: String,
    parameters: String,
    return_type: String,
    display_text: String,
    css_style: String,
}

impl ClassMember {
    fn new(input: &str, member_type: &str, config: &MermaidConfig) -> Self {
        let input = sanitize_text(input, config);
        let mut m = Self {
            member_type: member_type.to_string(),
            visibility: String::new(),
            id: String::new(),
            classifier: String::new(),
            parameters: String::new(),
            return_type: String::new(),
            display_text: String::new(),
            css_style: String::new(),
        };
        m.parse_member(&input, member_type);
        m
    }

    fn parse_method_signature_like_upstream(input: &str) -> Option<(&str, &str, &str, &str, &str)> {
        let s = input.trim();
        if s.is_empty() {
            return None;
        }

        if let Some(parsed) = Self::parse_method_signature_from_offset(s, true) {
            return Some(parsed);
        }
        Self::parse_method_signature_from_offset(s, false)
    }

    fn parse_method_signature_from_offset(
        input: &str,
        allow_visibility: bool,
    ) -> Option<(&str, &str, &str, &str, &str)> {
        let (visibility, rest) = if allow_visibility {
            match input.as_bytes()[0] {
                b'#' | b'+' | b'~' | b'-' => (&input[..1], &input[1..]),
                _ => ("", input),
            }
        } else {
            ("", input)
        };

        let paren_close = rest.rfind(')')?;
        let paren_open = rest[..paren_close].rfind('(')?;
        if paren_open == 0 {
            return None;
        }

        let name = &rest[..paren_open];
        let params = rest[paren_open + 1..paren_close].trim();
        let after_paren = &rest[paren_close + 1..];

        let (classifier, return_type) = if let Some(first) = after_paren.chars().next() {
            if first == '$' || first == '*' {
                (
                    &after_paren[..first.len_utf8()],
                    after_paren[first.len_utf8()..].trim(),
                )
            } else if is_js_regex_whitespace(first) {
                ("", after_paren[first.len_utf8()..].trim())
            } else {
                ("", after_paren.trim())
            }
        } else {
            ("", "")
        };

        Some((visibility, name, params, classifier, return_type))
    }

    fn parse_member(&mut self, input: &str, member_type: &str) {
        let input = input.trim();
        if member_type == "method" {
            if let Some((visibility, id, params, classifier, return_type)) =
                Self::parse_method_signature_like_upstream(input)
            {
                if matches!(visibility, "#" | "+" | "~" | "-") {
                    self.visibility = visibility.to_string();
                }
                self.id = id.to_string();
                self.parameters = params.to_string();
                self.classifier = classifier.to_string();
                self.return_type = return_type.to_string();
            }

            if self.classifier.is_empty() {
                if let Some(last) = self.return_type.chars().last() {
                    if last == '$' || last == '*' {
                        self.classifier = last.to_string();
                        self.return_type.pop();
                        self.return_type = self.return_type.trim().to_string();
                    }
                }
            }
        } else {
            let first = input.chars().next().unwrap_or('\0');
            let last = input.chars().last().unwrap_or('\0');
            let mut start = 0usize;
            let mut end = input.len();
            if matches!(first, '#' | '+' | '~' | '-') {
                self.visibility = first.to_string();
                start = first.len_utf8();
            }
            if last == '$' || last == '*' {
                self.classifier = last.to_string();
                end = input.len() - last.len_utf8();
            }
            self.id = input[start..end].to_string();
        }

        if self.id.starts_with(' ') {
            self.id = format!(" {}", self.id.trim());
        } else {
            self.id = self.id.trim().to_string();
        }

        self.css_style = match self.classifier.as_str() {
            "*" => "font-style:italic;".to_string(),
            "$" => "text-decoration:underline;".to_string(),
            _ => String::new(),
        };

        let mut display = format!("{}{}", self.visibility, parse_generic_types(&self.id));
        if member_type == "method" {
            display.push('(');
            display.push_str(&parse_generic_types(self.parameters.trim()));
            display.push(')');
            if !self.return_type.is_empty() {
                display.push_str(" : ");
                display.push_str(&parse_generic_types(self.return_type.trim()));
            }
        }
        self.display_text = display.trim().to_string();
    }

    fn into_value(self) -> Value {
        let mut obj = serde_json::Map::with_capacity(8);
        obj.insert("memberType".to_string(), Value::String(self.member_type));
        obj.insert("visibility".to_string(), Value::String(self.visibility));
        obj.insert("id".to_string(), Value::String(self.id));
        obj.insert("classifier".to_string(), Value::String(self.classifier));
        obj.insert("parameters".to_string(), Value::String(self.parameters));
        obj.insert("returnType".to_string(), Value::String(self.return_type));
        obj.insert("displayText".to_string(), Value::String(self.display_text));
        obj.insert("cssStyle".to_string(), Value::String(self.css_style));
        Value::Object(obj)
    }

    fn into_typed(self) -> class_typed::ClassMember {
        class_typed::ClassMember {
            member_type: self.member_type,
            visibility: self.visibility,
            id: self.id,
            classifier: self.classifier,
            parameters: self.parameters,
            return_type: self.return_type,
            display_text: self.display_text,
            css_style: self.css_style,
        }
    }
}

fn is_js_regex_whitespace(ch: char) -> bool {
    matches!(
        ch,
        '\t' | '\n' | '\u{000B}' | '\u{000C}' | '\r' | ' ' | '\u{00A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202F}'
                | '\u{205F}'
                | '\u{3000}'
                | '\u{FEFF}'
    )
}

fn collapse_acc_descr_newline_whitespace(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        out.push(ch);
        if ch == '\n' {
            while chars
                .peek()
                .is_some_and(|next| is_js_regex_whitespace(*next))
            {
                chars.next();
            }
        }
    }

    out
}

#[derive(Debug, Clone)]
struct ClassNode {
    id: String,
    type_param: String,
    label: String,
    text: String,
    css_classes: String,
    methods: Vec<ClassMember>,
    members: Vec<ClassMember>,
    annotations: Vec<String>,
    styles: Vec<String>,
    dom_id: String,
    parent: Option<String>,
    link: Option<String>,
    link_target: Option<String>,
    tooltip: Option<String>,
    have_callback: bool,
    callback: Option<serde_json::Map<String, Value>>,
    callback_effective: bool,
}

impl ClassNode {
    fn into_typed(self) -> class_typed::ClassNode {
        class_typed::ClassNode {
            id: self.id,
            type_param: self.type_param,
            label: self.label,
            text: self.text,
            css_classes: self.css_classes,
            methods: self
                .methods
                .into_iter()
                .map(ClassMember::into_typed)
                .collect(),
            members: self
                .members
                .into_iter()
                .map(ClassMember::into_typed)
                .collect(),
            annotations: self.annotations,
            styles: self.styles,
            dom_id: self.dom_id,
            parent: self.parent,
            link: self.link,
            link_target: self.link_target,
            tooltip: self.tooltip,
            have_callback: self.have_callback,
            callback: self.callback,
            callback_effective: self.callback_effective,
        }
    }
}

#[derive(Debug, Clone)]
struct ClassNote {
    id: String,
    class_id: Option<String>,
    text: String,
    parent: Option<String>,
}

impl ClassNote {
    fn into_typed(self) -> class_typed::ClassNote {
        class_typed::ClassNote {
            id: self.id,
            class_id: self.class_id,
            text: self.text,
            parent: self.parent,
        }
    }
}

#[derive(Debug, Clone)]
struct Interface {
    id: String,
    label: String,
    class_id: String,
}

impl Interface {
    fn into_typed(self) -> class_typed::ClassInterface {
        class_typed::ClassInterface {
            id: self.id,
            label: self.label,
            class_id: self.class_id,
        }
    }
}

#[derive(Debug, Clone)]
struct Namespace {
    id: String,
    label: String,
    dom_id: String,
    class_ids: Vec<String>,
    note_ids: Vec<String>,
    parent: Option<String>,
    explicit: bool,
}

impl Namespace {
    fn into_typed(self) -> class_typed::Namespace {
        class_typed::Namespace {
            id: self.id,
            label: self.label,
            dom_id: self.dom_id,
            class_ids: self.class_ids,
            note_ids: self.note_ids,
            parent: self.parent,
            explicit: self.explicit,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct StyleClass {
    id: String,
    styles: Vec<String>,
    text_styles: Vec<String>,
}

impl StyleClass {
    fn into_typed(self) -> class_typed::StyleClass {
        class_typed::StyleClass {
            id: self.id,
            styles: self.styles,
            text_styles: self.text_styles,
        }
    }
}

#[derive(Debug)]
pub(super) struct ClassDb<'a> {
    direction: String,
    classes: IndexMap<String, ClassNode>,
    relations: Vec<RelationData>,
    notes: Vec<ClassNote>,
    interfaces: Vec<Interface>,
    namespaces: IndexMap<String, Namespace>,
    namespace_stack: Vec<String>,
    style_classes: IndexMap<String, StyleClass>,
    class_counter: usize,
    namespace_counter: usize,
    acc_title: Option<String>,
    acc_descr: Option<String>,
    security_level: Option<&'a str>,
    config: &'a MermaidConfig,
}

impl<'a> ClassDb<'a> {
    pub(super) fn new(config: &'a MermaidConfig) -> Self {
        Self {
            direction: "TB".to_string(),
            classes: IndexMap::new(),
            relations: Vec::new(),
            notes: Vec::new(),
            interfaces: Vec::new(),
            namespaces: IndexMap::new(),
            namespace_stack: Vec::new(),
            style_classes: IndexMap::new(),
            class_counter: 0,
            namespace_counter: 0,
            acc_title: None,
            acc_descr: None,
            security_level: config.get_str("securityLevel"),
            config,
        }
    }
}

impl<'a> ClassDb<'a> {
    fn split_class_name_and_type(&self, id: &str) -> (String, String) {
        let id = sanitize_text(id, self.config);
        let (left, right) = if let Some((left, right)) = id.split_once('~') {
            (
                left.to_string(),
                right.split('~').next().unwrap_or("").to_string(),
            )
        } else {
            (id, String::new())
        };

        let class_name = sanitize_text(&left, self.config);
        let type_param = if right.is_empty() {
            right
        } else {
            sanitize_text(&right, self.config)
        };

        (class_name, type_param)
    }

    pub(super) fn add_class(&mut self, id: &str) {
        let (class_name, type_param) = self.split_class_name_and_type(id);
        if self.classes.contains_key(&class_name) {
            return;
        }
        let dom_id = format!("{MERMAID_DOM_ID_PREFIX}{class_name}-{}", self.class_counter);
        self.class_counter += 1;
        let text = if type_param.is_empty() {
            class_name.clone()
        } else {
            format!("{class_name}&lt;{type_param}&gt;")
        };
        self.classes.insert(
            class_name.clone(),
            ClassNode {
                id: class_name.clone(),
                type_param: type_param.clone(),
                label: class_name.clone(),
                text,
                css_classes: "default".to_string(),
                methods: Vec::new(),
                members: Vec::new(),
                annotations: Vec::new(),
                styles: Vec::new(),
                dom_id,
                parent: None,
                link: None,
                link_target: None,
                tooltip: None,
                have_callback: false,
                callback: None,
                callback_effective: false,
            },
        );
    }

    pub(super) fn set_class_label(&mut self, id: &str, label: &str) {
        let (class_name, type_param) = self.split_class_name_and_type(id);
        self.add_class(&class_name);
        let Some(c) = self.classes.get_mut(&class_name) else {
            return;
        };
        let label = sanitize_text(label, self.config);
        c.label = label.clone();
        c.text = if type_param.is_empty() {
            label
        } else {
            format!("{label}<{type_param}>")
        };
    }

    pub(super) fn set_direction(&mut self, dir: &str) {
        self.direction = dir.to_string();
    }

    fn cleanup_label(&self, label: &str) -> String {
        let t = label.trim();
        let t = t.strip_prefix(':').unwrap_or(t);
        sanitize_text(t.trim(), self.config)
    }

    pub(super) fn add_member(&mut self, class_name: &str, member: &str) {
        self.add_class(class_name);
        let (class_name, _) = self.split_class_name_and_type(class_name);
        let Some(c) = self.classes.get_mut(&class_name) else {
            return;
        };

        let member_string = member.trim();
        if member_string.is_empty() {
            return;
        }
        if member_string.starts_with("<<") && member_string.ends_with(">>") {
            c.annotations.push(sanitize_text(
                member_string
                    .trim_start_matches("<<")
                    .trim_end_matches(">>"),
                self.config,
            ));
            return;
        }
        if member_string.contains(')') {
            c.methods
                .push(ClassMember::new(member_string, "method", self.config));
            return;
        }
        c.members
            .push(ClassMember::new(member_string, "attribute", self.config));
    }

    fn add_members(&mut self, class_name: &str, mut members: Vec<String>) {
        members.reverse();
        for m in members {
            self.add_member(class_name, &m);
        }
    }

    fn add_annotation(&mut self, class_name: &str, annotation: &str) {
        self.add_class(class_name);
        let (class_name, _) = self.split_class_name_and_type(class_name);
        if let Some(c) = self.classes.get_mut(&class_name) {
            c.annotations.push(sanitize_text(annotation, self.config));
        }
    }

    pub(super) fn set_css_class(&mut self, ids: &str, css_class: &str) {
        for raw in ids.split(',') {
            let id = raw.trim();
            if id.is_empty() {
                continue;
            }
            let (class_name, _) = self.split_class_name_and_type(id);
            if let Some(c) = self.classes.get_mut(&class_name) {
                c.css_classes.push(' ');
                c.css_classes.push_str(css_class);
            }
        }
    }

    fn set_tooltip(&mut self, id: &str, tooltip: &str) {
        let (class_name, _) = self.split_class_name_and_type(id);
        if let Some(c) = self.classes.get_mut(&class_name) {
            c.tooltip = Some(sanitize_text(tooltip, self.config));
        }
    }

    fn set_link(&mut self, id: &str, url: &str, target: Option<String>) {
        let (class_name, _) = self.split_class_name_and_type(id);
        if let Some(c) = self.classes.get_mut(&class_name) {
            c.link = format_url(url, self.config);

            let final_target = if self.security_level == Some("sandbox") {
                "_top".to_string()
            } else if let Some(t) = target.clone() {
                sanitize_text(&t, self.config)
            } else {
                "_blank".to_string()
            };
            c.link_target = Some(final_target);
        }
        self.set_css_class(&class_name, "clickable");
    }

    fn set_click_event(&mut self, id: &str, function: &str, args: Option<String>) {
        let (class_name, _) = self.split_class_name_and_type(id);
        if let Some(c) = self.classes.get_mut(&class_name) {
            c.have_callback = true;
            let mut map = serde_json::Map::new();
            map.insert("function".to_string(), Value::String(function.to_string()));
            let args = args.and_then(|s| {
                let t = s.trim().to_string();
                if t.is_empty() { None } else { Some(t) }
            });
            if let Some(args) = args.clone() {
                map.insert("args".to_string(), Value::String(args.clone()));
            }
            c.callback = Some(map);
            c.callback_effective = self.security_level == Some("loose");
        }
        self.set_css_class(&class_name, "clickable");
    }

    fn parse_styles(raw: &str) -> Vec<String> {
        raw.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    fn set_css_style(&mut self, id: &str, styles: Vec<String>) {
        let Some(c) = self.classes.get_mut(id) else {
            return;
        };
        for s in styles {
            for part in s.split(',') {
                let t = part.trim();
                if !t.is_empty() {
                    c.styles.push(t.to_string());
                }
            }
        }
    }

    fn define_class(&mut self, id: &str, styles: Vec<String>) {
        let entry = self
            .style_classes
            .entry(id.to_string())
            .or_insert_with(|| StyleClass {
                id: id.to_string(),
                ..Default::default()
            });

        for s in &styles {
            if s.contains("color") {
                entry.text_styles.push(s.replace("fill", "bgFill"));
            }
            entry.styles.push(s.to_string());
        }

        for c in self.classes.values_mut() {
            if !c.css_classes.contains(id) {
                continue;
            }
            for s in &styles {
                for part in s.split(',') {
                    let t = part.trim();
                    if !t.is_empty() {
                        c.styles.push(t.to_string());
                    }
                }
            }
        }
    }

    fn current_namespace(&self) -> Option<&str> {
        self.namespace_stack.last().map(String::as_str)
    }

    fn resolve_qualified_namespace_id(&self, id: &str) -> String {
        let id = id.trim();
        if let Some(prefix) = self.current_namespace() {
            format!("{prefix}.{id}")
        } else {
            id.to_string()
        }
    }

    fn namespace_ancestor_ids(qualified_id: &str) -> Vec<String> {
        let parts: Vec<&str> = qualified_id.split('.').collect();
        let mut ids = Vec::with_capacity(parts.len());
        let mut current = String::new();
        for part in parts {
            if current.is_empty() {
                current.push_str(part);
            } else {
                current.push('.');
                current.push_str(part);
            }
            ids.push(current.clone());
        }
        ids
    }

    fn add_note(&mut self, class_id: Option<String>, text: &str) {
        let note_id = format!("note{}", self.notes.len());
        let parent = self.current_namespace().map(str::to_string);
        self.notes.push(ClassNote {
            id: note_id.clone(),
            class_id,
            text: text.to_string(),
            parent: parent.clone(),
        });
        if let Some(parent) = parent {
            if let Some(ns) = self.namespaces.get_mut(&parent) {
                ns.note_ids.push(note_id);
            }
        }
    }

    fn add_namespace(&mut self, id: &str, label: Option<String>) {
        let qualified_id = self.resolve_qualified_namespace_id(id);
        self.namespace_stack.push(qualified_id.clone());

        if self.namespaces.contains_key(&qualified_id) {
            if let Some(ns) = self.namespaces.get_mut(&qualified_id) {
                ns.explicit = true;
                if let Some(label) = label {
                    ns.label = sanitize_text(&label, self.config);
                }
            }
            return;
        }

        let parts: Vec<&str> = qualified_id.split('.').collect();
        let ancestor_ids = Self::namespace_ancestor_ids(&qualified_id);
        for (idx, current_id) in ancestor_ids.iter().enumerate() {
            let parent = idx
                .checked_sub(1)
                .and_then(|i| ancestor_ids.get(i))
                .cloned();
            let is_leaf = idx == ancestor_ids.len() - 1;
            let node_label = if is_leaf {
                label
                    .as_deref()
                    .map(|s| sanitize_text(s, self.config))
                    .unwrap_or_else(|| parts[idx].to_string())
            } else {
                parts[idx].to_string()
            };

            if let Some(ns) = self.namespaces.get_mut(current_id) {
                if is_leaf {
                    ns.explicit = true;
                    ns.label = node_label;
                }
                if ns.parent.is_none() {
                    ns.parent = parent;
                }
                continue;
            }

            let dom_id = format!(
                "{MERMAID_DOM_ID_PREFIX}{current_id}-{}",
                self.namespace_counter
            );
            self.namespace_counter += 1;
            self.namespaces.insert(
                current_id.clone(),
                Namespace {
                    id: current_id.clone(),
                    label: node_label,
                    dom_id,
                    class_ids: Vec::new(),
                    note_ids: Vec::new(),
                    parent,
                    explicit: is_leaf,
                },
            );
        }
    }

    fn pop_namespace(&mut self) {
        self.namespace_stack.pop();
    }

    fn add_classes_to_namespace(&mut self, namespace: &str, class_names: &[String]) {
        let namespace = self
            .current_namespace()
            .map(str::to_string)
            .unwrap_or_else(|| namespace.to_string());
        if !self.namespaces.contains_key(&namespace) {
            return;
        }
        let mut ids = Vec::new();
        for name in class_names {
            let (class_name, _) = self.split_class_name_and_type(name);
            self.add_class(name);
            if let Some(c) = self.classes.get_mut(&class_name) {
                c.parent = Some(namespace.clone());
            }
            ids.push(class_name);
        }
        if let Some(ns) = self.namespaces.get_mut(&namespace) {
            for id in ids {
                if !ns.class_ids.contains(&id) {
                    ns.class_ids.push(id);
                }
            }
        }
    }

    fn existing_relation_class_id(&self, class_name: &str) -> Option<String> {
        if self.classes.contains_key(class_name) {
            return Some(class_name.to_string());
        }

        None
    }

    fn add_relation_class(&mut self, id: &str) -> String {
        let (class_name, _) = self.split_class_name_and_type(id);
        if let Some(existing_id) = self.existing_relation_class_id(&class_name) {
            return existing_id;
        }

        self.add_class(id);
        class_name
    }

    fn add_relation(&mut self, mut rel: RelationData) {
        let original_id1 = rel.id1.clone();
        let original_id2 = rel.id2.clone();

        let invalid_types = [
            REL_LOLLIPOP,
            REL_AGGREGATION,
            REL_COMPOSITION,
            REL_DEPENDENCY,
            REL_EXTENSION,
        ];

        if rel.relation.type1 == REL_LOLLIPOP && !invalid_types.contains(&rel.relation.type2) {
            let id2_name = self.add_relation_class(&original_id2);
            let iface_id = format!("interface{}", self.interfaces.len());
            self.interfaces.push(Interface {
                id: iface_id.clone(),
                label: rel.id1.clone(),
                class_id: id2_name.clone(),
            });
            rel.id1 = iface_id;
            rel.id2 = id2_name;
        } else if rel.relation.type2 == REL_LOLLIPOP && !invalid_types.contains(&rel.relation.type1)
        {
            let id1_name = self.add_relation_class(&original_id1);
            let iface_id = format!("interface{}", self.interfaces.len());
            self.interfaces.push(Interface {
                id: iface_id.clone(),
                label: rel.id2.clone(),
                class_id: id1_name.clone(),
            });
            rel.id1 = id1_name;
            rel.id2 = iface_id;
        } else {
            let id1_name = self.add_relation_class(&original_id1);
            let id2_name = self.add_relation_class(&original_id2);
            rel.id1 = id1_name;
            rel.id2 = id2_name;
        }

        self.relations.push(rel);
    }

    pub(super) fn apply(&mut self, action: Action) -> std::result::Result<(), String> {
        match action {
            Action::SetDirection(d) => {
                self.set_direction(&d);
                Ok(())
            }
            Action::SetAccTitle(t) => {
                self.acc_title = Some(t.trim_start().to_string());
                Ok(())
            }
            Action::SetAccDescr(t) => {
                let trimmed = t.trim().to_string();
                self.acc_descr = Some(collapse_acc_descr_newline_whitespace(&trimmed));
                Ok(())
            }

            Action::AddNamespace { id, label } => {
                self.add_namespace(&id, label);
                Ok(())
            }
            Action::AddClassesToNamespace {
                namespace,
                class_ids,
            } => {
                self.add_classes_to_namespace(&namespace, &class_ids);
                Ok(())
            }
            Action::PopNamespace => {
                self.pop_namespace();
                Ok(())
            }

            Action::AddClass { id } => {
                self.add_class(&id);
                Ok(())
            }
            Action::SetClassLabel { id, label } => {
                self.set_class_label(&id, &label);
                Ok(())
            }
            Action::SetCssClass { ids, css_class } => {
                self.set_css_class(&ids, &css_class);
                Ok(())
            }
            Action::SetCssStyle { id, raw } => {
                let styles = Self::parse_styles(&raw);
                self.set_css_style(&id, styles);
                Ok(())
            }
            Action::DefineClass { id, raw } => {
                let styles = Self::parse_styles(&raw);
                self.define_class(&id, styles);
                Ok(())
            }
            Action::SetLink { id, url, target } => {
                self.set_link(&id, &url, target);
                Ok(())
            }
            Action::SetTooltip { id, tooltip } => {
                self.set_tooltip(&id, &tooltip);
                Ok(())
            }
            Action::SetClickEvent { id, function, args } => {
                self.set_click_event(&id, &function, args);
                Ok(())
            }
            Action::AddMembers { id, members } => {
                self.add_members(&id, members);
                Ok(())
            }
            Action::AddMember { id, member } => {
                let cleaned = self.cleanup_label(&member);
                self.add_member(&id, &cleaned);
                Ok(())
            }
            Action::AddAnnotation { id, annotation } => {
                self.add_annotation(&id, &annotation);
                Ok(())
            }
            Action::AddRelation { mut data } => {
                if let Some(t) = data.title.take() {
                    data.title = Some(self.cleanup_label(&t));
                }
                if let Some(t) = data.relation_title1.take() {
                    data.relation_title1 = Some(sanitize_text(t.trim(), self.config));
                }
                if let Some(t) = data.relation_title2.take() {
                    data.relation_title2 = Some(sanitize_text(t.trim(), self.config));
                }
                self.add_relation(data);
                Ok(())
            }
            Action::AddNote { class_id, text } => {
                self.add_note(class_id, text.trim());
                Ok(())
            }
        }
    }

    fn hierarchical_namespaces_enabled(&self) -> bool {
        self.config
            .get_bool("class.hierarchicalNamespaces")
            .unwrap_or(true)
    }

    fn resolve_explicit_ancestor_from(
        namespaces: &IndexMap<String, Namespace>,
        id: Option<&str>,
    ) -> Option<String> {
        let mut current = id.map(str::to_string);
        while let Some(id) = current {
            let ns = namespaces.get(id.as_str())?;
            if ns.explicit {
                return Some(id);
            }
            current = ns.parent.clone();
        }
        None
    }

    fn rebuild_namespace_memberships(&mut self) {
        for ns in self.namespaces.values_mut() {
            ns.class_ids.clear();
            ns.note_ids.clear();
        }
        for (id, class_node) in &self.classes {
            let Some(parent) = class_node.parent.as_deref() else {
                continue;
            };
            if let Some(ns) = self.namespaces.get_mut(parent) {
                if !ns.class_ids.contains(id) {
                    ns.class_ids.push(id.clone());
                }
            }
        }
        for note in &self.notes {
            let Some(parent) = note.parent.as_deref() else {
                continue;
            };
            if let Some(ns) = self.namespaces.get_mut(parent) {
                if !ns.note_ids.contains(&note.id) {
                    ns.note_ids.push(note.id.clone());
                }
            }
        }
    }

    fn apply_namespace_render_config(&mut self) {
        if self.hierarchical_namespaces_enabled() {
            self.rebuild_namespace_memberships();
            return;
        }

        let class_parent_updates: Vec<(String, Option<String>)> = self
            .classes
            .iter()
            .map(|(id, class_node)| {
                (
                    id.clone(),
                    Self::resolve_explicit_ancestor_from(
                        &self.namespaces,
                        class_node.parent.as_deref(),
                    ),
                )
            })
            .collect();
        let note_parent_updates: Vec<(String, Option<String>)> = self
            .notes
            .iter()
            .map(|note| {
                (
                    note.id.clone(),
                    Self::resolve_explicit_ancestor_from(&self.namespaces, note.parent.as_deref()),
                )
            })
            .collect();

        self.namespaces.retain(|_, ns| ns.explicit);
        for ns in self.namespaces.values_mut() {
            ns.label = ns.id.clone();
            ns.parent = None;
        }
        for (id, parent) in class_parent_updates {
            if let Some(class_node) = self.classes.get_mut(&id) {
                class_node.parent = parent;
            }
        }
        for (id, parent) in note_parent_updates {
            if let Some(note) = self.notes.iter_mut().find(|note| note.id == id) {
                note.parent = parent;
            }
        }
        self.rebuild_namespace_memberships();
    }

    pub(super) fn into_model(mut self, meta: &ParseMetadata) -> Value {
        self.apply_namespace_render_config();

        let mut classes_json = serde_json::Map::with_capacity(self.classes.len());
        for (id, c) in self.classes {
            let methods: Vec<Value> = c.methods.into_iter().map(ClassMember::into_value).collect();
            let members: Vec<Value> = c.members.into_iter().map(ClassMember::into_value).collect();

            let mut obj = serde_json::Map::with_capacity(16);
            obj.insert("id".to_string(), Value::String(c.id));
            obj.insert("type".to_string(), Value::String(c.type_param));
            obj.insert("label".to_string(), Value::String(c.label));
            obj.insert("text".to_string(), Value::String(c.text));
            obj.insert("cssClasses".to_string(), Value::String(c.css_classes));
            obj.insert("methods".to_string(), Value::Array(methods));
            obj.insert("members".to_string(), Value::Array(members));
            obj.insert(
                "annotations".to_string(),
                Value::Array(c.annotations.into_iter().map(Value::String).collect()),
            );
            obj.insert(
                "styles".to_string(),
                Value::Array(c.styles.into_iter().map(Value::String).collect()),
            );
            obj.insert("domId".to_string(), Value::String(c.dom_id));
            obj.insert(
                "parent".to_string(),
                c.parent.map(Value::String).unwrap_or(Value::Null),
            );
            obj.insert(
                "link".to_string(),
                c.link.map(Value::String).unwrap_or(Value::Null),
            );
            obj.insert(
                "linkTarget".to_string(),
                c.link_target.map(Value::String).unwrap_or(Value::Null),
            );
            obj.insert(
                "tooltip".to_string(),
                c.tooltip.map(Value::String).unwrap_or(Value::Null),
            );
            obj.insert("haveCallback".to_string(), Value::Bool(c.have_callback));
            obj.insert(
                "callback".to_string(),
                c.callback.map(Value::Object).unwrap_or(Value::Null),
            );
            obj.insert(
                "callbackEffective".to_string(),
                Value::Bool(c.callback_effective),
            );
            classes_json.insert(id, Value::Object(obj));
        }

        let mut relations_json = Vec::with_capacity(self.relations.len());
        for (idx, r) in self.relations.into_iter().enumerate() {
            let mut rel_obj = serde_json::Map::with_capacity(3);
            rel_obj.insert("type1".to_string(), Value::Number(r.relation.type1.into()));
            rel_obj.insert("type2".to_string(), Value::Number(r.relation.type2.into()));
            rel_obj.insert(
                "lineType".to_string(),
                Value::Number(r.relation.line_type.into()),
            );

            let mut obj = serde_json::Map::with_capacity(7);
            obj.insert("id".to_string(), Value::String(idx.to_string()));
            obj.insert("id1".to_string(), Value::String(r.id1));
            obj.insert("id2".to_string(), Value::String(r.id2));
            obj.insert(
                "relationTitle1".to_string(),
                Value::String(r.relation_title1.unwrap_or_else(|| "none".to_string())),
            );
            obj.insert(
                "relationTitle2".to_string(),
                Value::String(r.relation_title2.unwrap_or_else(|| "none".to_string())),
            );
            obj.insert(
                "title".to_string(),
                Value::String(r.title.unwrap_or_default()),
            );
            obj.insert("relation".to_string(), Value::Object(rel_obj));
            relations_json.push(Value::Object(obj));
        }

        let mut notes_json = Vec::with_capacity(self.notes.len());
        for n in self.notes {
            let mut obj = serde_json::Map::with_capacity(4);
            obj.insert("id".to_string(), Value::String(n.id));
            obj.insert(
                "class".to_string(),
                n.class_id.map(Value::String).unwrap_or(Value::Null),
            );
            obj.insert("text".to_string(), Value::String(n.text));
            if let Some(parent) = n.parent {
                obj.insert("parent".to_string(), Value::String(parent));
            }
            notes_json.push(Value::Object(obj));
        }

        let mut interfaces_json = Vec::with_capacity(self.interfaces.len());
        for i in self.interfaces {
            let mut obj = serde_json::Map::with_capacity(3);
            obj.insert("id".to_string(), Value::String(i.id));
            obj.insert("label".to_string(), Value::String(i.label));
            obj.insert("classId".to_string(), Value::String(i.class_id));
            interfaces_json.push(Value::Object(obj));
        }

        let mut namespaces_json = serde_json::Map::with_capacity(self.namespaces.len());
        for (k, ns) in self.namespaces {
            let mut obj = serde_json::Map::with_capacity(7);
            obj.insert("id".to_string(), Value::String(ns.id));
            obj.insert("label".to_string(), Value::String(ns.label));
            obj.insert("domId".to_string(), Value::String(ns.dom_id));
            obj.insert(
                "classIds".to_string(),
                Value::Array(ns.class_ids.into_iter().map(Value::String).collect()),
            );
            obj.insert(
                "noteIds".to_string(),
                Value::Array(ns.note_ids.into_iter().map(Value::String).collect()),
            );
            obj.insert(
                "parent".to_string(),
                ns.parent.map(Value::String).unwrap_or(Value::Null),
            );
            obj.insert("explicit".to_string(), Value::Bool(ns.explicit));
            namespaces_json.insert(k, Value::Object(obj));
        }

        let mut style_classes_json = serde_json::Map::with_capacity(self.style_classes.len());
        for (k, sc) in self.style_classes {
            let mut obj = serde_json::Map::with_capacity(3);
            obj.insert("id".to_string(), Value::String(sc.id));
            obj.insert(
                "styles".to_string(),
                Value::Array(sc.styles.into_iter().map(Value::String).collect()),
            );
            obj.insert(
                "textStyles".to_string(),
                Value::Array(sc.text_styles.into_iter().map(Value::String).collect()),
            );
            style_classes_json.insert(k, Value::Object(obj));
        }

        let mut line_type_obj = serde_json::Map::with_capacity(2);
        line_type_obj.insert("line".to_string(), Value::Number(LINE_SOLID.into()));
        line_type_obj.insert("dottedLine".to_string(), Value::Number(LINE_DOTTED.into()));

        let mut relation_type_obj = serde_json::Map::with_capacity(6);
        relation_type_obj.insert("none".to_string(), Value::Number(REL_NONE.into()));
        relation_type_obj.insert(
            "aggregation".to_string(),
            Value::Number(REL_AGGREGATION.into()),
        );
        relation_type_obj.insert("extension".to_string(), Value::Number(REL_EXTENSION.into()));
        relation_type_obj.insert(
            "composition".to_string(),
            Value::Number(REL_COMPOSITION.into()),
        );
        relation_type_obj.insert(
            "dependency".to_string(),
            Value::Number(REL_DEPENDENCY.into()),
        );
        relation_type_obj.insert("lollipop".to_string(), Value::Number(REL_LOLLIPOP.into()));

        let mut constants_obj = serde_json::Map::with_capacity(2);
        constants_obj.insert("lineType".to_string(), Value::Object(line_type_obj));
        constants_obj.insert("relationType".to_string(), Value::Object(relation_type_obj));

        let mut obj = serde_json::Map::with_capacity(10);
        obj.insert("type".to_string(), Value::String(meta.diagram_type.clone()));
        obj.insert("direction".to_string(), Value::String(self.direction));
        obj.insert(
            "accTitle".to_string(),
            self.acc_title.map(Value::String).unwrap_or(Value::Null),
        );
        obj.insert(
            "accDescr".to_string(),
            self.acc_descr.map(Value::String).unwrap_or(Value::Null),
        );
        obj.insert("classes".to_string(), Value::Object(classes_json));
        obj.insert("relations".to_string(), Value::Array(relations_json));
        obj.insert("notes".to_string(), Value::Array(notes_json));
        obj.insert("interfaces".to_string(), Value::Array(interfaces_json));
        obj.insert("namespaces".to_string(), Value::Object(namespaces_json));
        obj.insert(
            "styleClasses".to_string(),
            Value::Object(style_classes_json),
        );
        obj.insert("constants".to_string(), Value::Object(constants_obj));
        Value::Object(obj)
    }

    pub(super) fn into_typed_model(mut self, meta: &ParseMetadata) -> class_typed::ClassDiagram {
        self.apply_namespace_render_config();

        let classes = self
            .classes
            .into_iter()
            .map(|(id, c)| (id, c.into_typed()))
            .collect();

        let relations = self
            .relations
            .into_iter()
            .enumerate()
            .map(|(idx, r)| class_typed::ClassRelation {
                id: idx.to_string(),
                id1: r.id1,
                id2: r.id2,
                relation_title_1: r.relation_title1.unwrap_or_else(|| "none".to_string()),
                relation_title_2: r.relation_title2.unwrap_or_else(|| "none".to_string()),
                title: r.title.unwrap_or_default(),
                relation: class_typed::RelationShape {
                    type1: r.relation.type1,
                    type2: r.relation.type2,
                    line_type: r.relation.line_type,
                },
            })
            .collect();

        let notes = self.notes.into_iter().map(ClassNote::into_typed).collect();
        let interfaces = self
            .interfaces
            .into_iter()
            .map(Interface::into_typed)
            .collect();
        let namespaces = self
            .namespaces
            .into_iter()
            .map(|(k, ns)| (k, ns.into_typed()))
            .collect();
        let style_classes = self
            .style_classes
            .into_iter()
            .map(|(k, sc)| (k, sc.into_typed()))
            .collect();

        class_typed::ClassDiagram {
            diagram_type: meta.diagram_type.clone(),
            direction: self.direction,
            acc_title: self.acc_title,
            acc_descr: self.acc_descr,
            classes,
            relations,
            notes,
            interfaces,
            namespaces,
            style_classes,
            constants: class_typed::ClassConstants {
                line_type: class_typed::ClassLineTypeConstants {
                    line: LINE_SOLID,
                    dotted_line: LINE_DOTTED,
                },
                relation_type: class_typed::ClassRelationTypeConstants {
                    none: REL_NONE,
                    aggregation: REL_AGGREGATION,
                    extension: REL_EXTENSION,
                    composition: REL_COMPOSITION,
                    dependency: REL_DEPENDENCY,
                    lollipop: REL_LOLLIPOP,
                },
            },
        }
    }
}
