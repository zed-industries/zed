use crate::{Error, ParseMetadata, Result};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap, VecDeque};

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::empty_line_after_outer_attr)]
    er_grammar,
    "/diagrams/er_grammar.rs"
);

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErDiagramRenderModel {
    #[serde(default, rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(default, rename = "accDescr")]
    pub acc_descr: Option<String>,
    pub direction: String,
    #[serde(default)]
    pub classes: BTreeMap<String, ErClassDefRenderModel>,
    #[serde(default)]
    pub entities: BTreeMap<String, ErEntityRenderModel>,
    #[serde(default)]
    pub relationships: Vec<ErRelationshipRenderModel>,
}

impl ErDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErAttributeRenderModel {
    #[serde(rename = "type")]
    pub ty: String,
    pub name: String,
    #[serde(default)]
    pub keys: Vec<String>,
    #[serde(default)]
    pub comment: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErRelSpecRenderModel {
    #[serde(rename = "cardA")]
    pub card_a: String,
    #[serde(rename = "cardB")]
    pub card_b: String,
    #[serde(rename = "relType")]
    pub rel_type: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErRelationshipRenderModel {
    #[serde(rename = "entityA")]
    pub entity_a: String,
    #[serde(default, rename = "roleA")]
    pub role_a: String,
    #[serde(rename = "entityB")]
    pub entity_b: String,
    #[serde(default, rename = "relSpec")]
    pub rel_spec: ErRelSpecRenderModel,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErClassDefRenderModel {
    pub id: String,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default, rename = "textStyles")]
    pub text_styles: Vec<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ErEntityRenderModel {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub attributes: Vec<ErAttributeRenderModel>,
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub shape: String,
    #[serde(default, rename = "cssClasses")]
    pub css_classes: String,
    #[serde(default, rename = "cssStyles")]
    pub css_styles: Vec<String>,
}

pub(crate) type Attribute = ErAttributeRenderModel;
pub(crate) type RelSpec = ErRelSpecRenderModel;
type Relationship = ErRelationshipRenderModel;
type EntityClass = ErClassDefRenderModel;
type EntityNode = ErEntityRenderModel;

#[derive(Debug, Clone)]
enum Action {
    AddEntity {
        name: String,
        alias: Option<String>,
    },
    AddAttributes {
        entity: String,
        attributes: Vec<Attribute>,
    },
    AddRelationship {
        a: String,
        role: String,
        b: String,
        spec: RelSpec,
    },
    SetClass {
        entities: Vec<String>,
        classes: Vec<String>,
    },
    AddClassDef {
        classes: Vec<String>,
        raw: String,
    },
    AddCssStyles {
        entities: Vec<String>,
        raw: String,
    },
    SetDirection(String),
    SetAccTitle(String),
    SetAccDescr(String),
}

#[derive(Debug, Default)]
struct ErDb {
    entities: HashMap<String, EntityNode>,
    relationships: Vec<Relationship>,
    classes: HashMap<String, EntityClass>,
    direction: String,
    entity_counter: usize,
    acc_title: Option<String>,
    acc_descr: Option<String>,
}

impl ErDb {
    fn new() -> Self {
        Self {
            direction: "TB".to_string(),
            ..Default::default()
        }
    }

    fn add_entity(&mut self, name: &str, alias: Option<&str>) {
        let Some(existing) = self.entities.get_mut(name) else {
            let id = format!("entity-{name}-{}", self.entity_counter);
            self.entity_counter += 1;
            self.entities.insert(
                name.to_string(),
                EntityNode {
                    id,
                    label: name.to_string(),
                    attributes: Vec::new(),
                    alias: alias.unwrap_or("").to_string(),
                    shape: "erBox".to_string(),
                    css_classes: "default".to_string(),
                    css_styles: Vec::new(),
                },
            );
            return;
        };

        if existing.alias.is_empty() {
            if let Some(a) = alias {
                if !a.is_empty() {
                    existing.alias = a.to_string();
                }
            }
        }
    }

    fn add_attributes(&mut self, entity: &str, attributes: Vec<Attribute>) {
        self.add_entity(entity, None);
        let Some(e) = self.entities.get_mut(entity) else {
            return;
        };
        for a in attributes {
            e.attributes.push(a);
        }
    }

    fn add_relationship(&mut self, a: &str, role: &str, b: &str, spec: RelSpec) {
        let (Some(entity_a), Some(entity_b)) = (self.entities.get(a), self.entities.get(b)) else {
            return;
        };
        self.relationships.push(Relationship {
            entity_a: entity_a.id.clone(),
            role_a: role.to_string(),
            entity_b: entity_b.id.clone(),
            rel_spec: spec,
        });
    }

    fn set_class(&mut self, entities: &[String], classes: &[String]) {
        for e in entities {
            let Some(node) = self.entities.get_mut(e) else {
                continue;
            };
            for cls in classes {
                node.css_classes.push(' ');
                node.css_classes.push_str(cls);
            }
        }
    }

    fn add_class_def(&mut self, classes: &[String], styles: &[String]) {
        for id in classes {
            let entry = self
                .classes
                .entry(id.to_string())
                .or_insert_with(|| EntityClass {
                    id: id.to_string(),
                    ..Default::default()
                });

            for s in styles {
                if s.contains("color") {
                    entry.text_styles.push(s.replace("fill", "bgFill"));
                }
                entry.styles.push(s.to_string());
            }
        }
    }

    fn add_css_styles(&mut self, entities: &[String], styles: &[String]) {
        for id in entities {
            let Some(entity) = self.entities.get_mut(id) else {
                continue;
            };
            for style in styles {
                entity.css_styles.push(style.to_string());
            }
        }
    }

    fn apply(&mut self, a: Action) {
        match a {
            Action::AddEntity { name, alias } => self.add_entity(&name, alias.as_deref()),
            Action::AddAttributes { entity, attributes } => {
                self.add_attributes(&entity, attributes)
            }
            Action::AddRelationship { a, role, b, spec } => {
                self.add_relationship(&a, &role, &b, spec)
            }
            Action::SetClass { entities, classes } => self.set_class(&entities, &classes),
            Action::AddClassDef { classes, raw } => {
                let styles = split_styles(&raw);
                self.add_class_def(&classes, &styles);
            }
            Action::AddCssStyles { entities, raw } => {
                let styles = split_styles(&raw);
                self.add_css_styles(&entities, &styles);
            }
            Action::SetDirection(dir) => self.direction = dir,
            Action::SetAccTitle(t) => {
                self.acc_title = Some(t.trim().trim_start().to_string());
            }
            Action::SetAccDescr(t) => {
                // Mermaid's commonDb.ts: `sanitizeText(txt).replace(/\n\s+/g, '\n')`
                let trimmed = t.trim();
                let mut out = String::with_capacity(trimmed.len());
                let mut chars = trimmed.chars().peekable();
                while let Some(ch) = chars.next() {
                    out.push(ch);
                    if ch == '\n' {
                        while chars.peek().is_some_and(|c| c.is_whitespace()) {
                            chars.next();
                        }
                    }
                }
                self.acc_descr = Some(out);
            }
        }
    }

    fn into_render_model(self) -> ErDiagramRenderModel {
        ErDiagramRenderModel {
            acc_title: self.acc_title,
            acc_descr: self.acc_descr,
            direction: self.direction,
            classes: self.classes.into_iter().collect(),
            entities: self.entities.into_iter().collect(),
            relationships: self.relationships,
        }
    }

    fn into_model(self, meta: &ParseMetadata) -> Result<Value> {
        let mut value =
            serde_json::to_value(self.into_render_model()).map_err(|e| Error::DiagramParse {
                diagram_type: meta.diagram_type.clone(),
                message: e.to_string(),
            })?;
        let Value::Object(obj) = &mut value else {
            return Ok(value);
        };

        obj.insert("type".to_string(), json!(meta.diagram_type));
        obj.insert(
            "constants".to_string(),
            json!({
                "cardinality": {
                    "zeroOrOne": "ZERO_OR_ONE",
                    "zeroOrMore": "ZERO_OR_MORE",
                    "oneOrMore": "ONE_OR_MORE",
                    "onlyOne": "ONLY_ONE",
                    "mdParent": "MD_PARENT",
                },
                "identification": {
                    "nonIdentifying": "NON_IDENTIFYING",
                    "identifying": "IDENTIFYING",
                }
            }),
        );

        Ok(value)
    }
}

fn split_styles(raw: &str) -> Vec<String> {
    let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    compact
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn parse_er_db(code: &str, meta: &ParseMetadata) -> Result<ErDb> {
    let actions = er_grammar::ActionsParser::new()
        .parse(Lexer::new(code))
        .map_err(|e| Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("{e:?}"),
        })?;

    let mut db = ErDb::new();
    for a in actions {
        db.apply(a);
    }
    Ok(db)
}

pub fn parse_er_model_for_render(code: &str, meta: &ParseMetadata) -> Result<ErDiagramRenderModel> {
    let db = parse_er_db(code, meta)?;
    Ok(db.into_render_model())
}

pub fn parse_er(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let db = parse_er_db(code, meta)?;
    db.into_model(meta)
}

#[derive(Debug, Clone)]
enum Tok {
    ErDiagram,
    Newline,

    Name(String),
    Str(String),
    IdList(Vec<String>),
    RestOfLine(String),

    AccTitle(String),
    AccDescr(String),
    AccDescrMultiline(String),

    BlockStart,
    BlockStop,
    SquareStart,
    SquareStop,
    StyleSeparator,
    Colon,
    Comma,

    StyleKw,
    ClassDefKw,
    ClassKw,
    Direction(String),

    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    OnlyOne,
    MdParent,
    Identifying,
    NonIdentifying,

    AttrWord(String),
    AttrKey(String),
    Comment(String),
}

#[derive(Debug)]
struct LexError {
    message: String,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LexError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Default,
    Block,
    NeedIdListOnly,
    NeedIdListThenLineRest,
    NeedClassFirstIdList,
    NeedClassSecondIdList,
    LineRest,
}

struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    pending: VecDeque<(usize, Tok, usize)>,
    mode: Mode,
}

impl<'input> Lexer<'input> {
    fn new(input: &'input str) -> Self {
        Self {
            input,
            pos: 0,
            pending: VecDeque::new(),
            mode: Mode::Default,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn skip_ws_default(&mut self) {
        while let Some(b) = self.peek() {
            if b == b' ' || b == b'\t' || b == b'\r' {
                self.pos += 1;
                continue;
            }
            break;
        }
    }

    fn skip_ws_block(&mut self) {
        while let Some(b) = self.peek() {
            if matches!(b, b' ' | b'\t' | b'\r' | b'\n') {
                self.pos += 1;
                continue;
            }
            break;
        }
    }

    fn starts_with_ci(&self, s: &str) -> bool {
        self.input[self.pos..]
            .get(..s.len())
            .is_some_and(|h| h.eq_ignore_ascii_case(s))
    }

    fn starts_with_word_ci(&self, s: &str) -> bool {
        if !self.starts_with_ci(s) {
            return false;
        }
        let after = self.pos + s.len();
        if after >= self.input.len() {
            return true;
        }
        let b = self.input.as_bytes()[after];
        b.is_ascii_whitespace() || matches!(b, b':' | b'{' | b'}' | b'[' | b']' | b';')
    }

    fn read_to_newline(&mut self) -> String {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if b == b'\n' {
                break;
            }
            self.pos += 1;
        }
        self.input[start..self.pos].to_string()
    }

    fn lex_comment(&mut self) -> bool {
        if self.input[self.pos..].starts_with("%%") {
            let _ = self.read_to_newline();
            return true;
        }
        false
    }

    fn lex_newline(&mut self) -> Option<(usize, Tok, usize)> {
        if self.mode == Mode::Block {
            return None;
        }
        if self.peek()? != b'\n' {
            return None;
        }
        let start = self.pos;
        while let Some(b'\n') = self.peek() {
            self.pos += 1;
        }
        if self.mode == Mode::LineRest {
            self.mode = Mode::Default;
        }
        Some((start, Tok::Newline, self.pos))
    }

    fn lex_acc_title(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.starts_with_ci("accTitle") {
            return None;
        }
        let after = self.pos + "accTitle".len();
        let rest = &self.input[after..];
        let rest_trim = rest.trim_start();
        if !rest_trim.starts_with(':') {
            return None;
        }
        let consumed_ws = rest.len() - rest_trim.len();
        self.pos = after + consumed_ws + 1;
        let s = self.read_to_newline();
        Some(Ok((start, Tok::AccTitle(s.trim().to_string()), self.pos)))
    }

    fn lex_acc_descr(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        let start = self.pos;
        if !self.starts_with_ci("accDescr") {
            return None;
        }
        let after = self.pos + "accDescr".len();
        let rest = &self.input[after..];
        let rest_trim = rest.trim_start();
        if rest_trim.starts_with('{') {
            let consumed_ws = rest.len() - rest_trim.len();
            self.pos = after + consumed_ws + 1;
            let Some(end_rel) = self.input[self.pos..].find('}') else {
                return Some(Err(LexError {
                    message: "Unterminated accDescr block; missing '}'".to_string(),
                }));
            };
            let body = self.input[self.pos..self.pos + end_rel].to_string();
            self.pos = self.pos + end_rel + 1;
            return Some(Ok((
                start,
                Tok::AccDescrMultiline(body.trim().to_string()),
                self.pos,
            )));
        }
        let colon_pos = rest.find(':')?;
        self.pos = after + colon_pos + 1;
        let s = self.read_to_newline();
        Some(Ok((start, Tok::AccDescr(s.trim().to_string()), self.pos)))
    }

    fn lex_direction(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if !self.starts_with_word_ci("direction") {
            return None;
        }
        self.pos += "direction".len();
        self.skip_ws_default();
        let rest = &self.input[self.pos..].to_ascii_uppercase();
        let dir = if rest.starts_with("TB") {
            self.pos += 2;
            "TB"
        } else if rest.starts_with("BT") {
            self.pos += 2;
            "BT"
        } else if rest.starts_with("LR") {
            self.pos += 2;
            "LR"
        } else if rest.starts_with("RL") {
            self.pos += 2;
            "RL"
        } else {
            return None;
        };
        let _ = self.read_to_newline();
        Some((start, Tok::Direction(dir.to_string()), self.pos))
    }

    fn lex_keyword(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        if self.starts_with_word_ci("erDiagram") {
            self.pos += "erDiagram".len();
            return Some((start, Tok::ErDiagram, self.pos));
        }
        if self.starts_with_word_ci("style") {
            self.pos += "style".len();
            self.mode = Mode::NeedIdListThenLineRest;
            return Some((start, Tok::StyleKw, self.pos));
        }
        if self.starts_with_word_ci("classDef") {
            self.pos += "classDef".len();
            self.mode = Mode::NeedIdListThenLineRest;
            return Some((start, Tok::ClassDefKw, self.pos));
        }
        if self.starts_with_word_ci("class") {
            self.pos += "class".len();
            self.mode = Mode::NeedClassFirstIdList;
            return Some((start, Tok::ClassKw, self.pos));
        }
        None
    }

    fn lex_id_list(&mut self) -> Option<(usize, Tok, usize)> {
        if !matches!(
            self.mode,
            Mode::NeedIdListOnly
                | Mode::NeedIdListThenLineRest
                | Mode::NeedClassFirstIdList
                | Mode::NeedClassSecondIdList
        ) {
            return None;
        }
        let start = self.pos;
        self.skip_ws_default();
        let mut ids: Vec<String> = Vec::new();
        loop {
            let id_start = self.pos;
            let mut id_end = self.pos;
            for (rel, ch) in self.input[self.pos..].char_indices() {
                let ok =
                    !ch.is_ascii() || ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '*');
                if !ok {
                    break;
                }
                id_end = self.pos + rel + ch.len_utf8();
            }
            if id_end == id_start {
                break;
            }
            ids.push(self.input[id_start..id_end].to_string());
            self.pos = id_end;

            self.skip_ws_default();
            if self.peek() != Some(b',') {
                break;
            }
            self.pos += 1;
            self.skip_ws_default();
        }
        if ids.is_empty() {
            return None;
        }
        self.mode = match self.mode {
            Mode::NeedIdListOnly => Mode::Default,
            Mode::NeedIdListThenLineRest => Mode::LineRest,
            Mode::NeedClassFirstIdList => Mode::NeedClassSecondIdList,
            Mode::NeedClassSecondIdList => Mode::Default,
            _ => Mode::Default,
        };
        Some((start, Tok::IdList(ids), self.pos))
    }

    fn lex_rest_of_line(&mut self) -> Option<(usize, Tok, usize)> {
        if self.mode != Mode::LineRest {
            return None;
        }
        let start = self.pos;
        self.skip_ws_default();
        let s = self.read_to_newline();
        self.mode = Mode::Default;
        Some((
            start,
            Tok::RestOfLine(s.trim().trim_end_matches(';').to_string()),
            self.pos,
        ))
    }

    fn lex_rel_tokens(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        let s = &self.input[self.pos..];

        let lower = s.to_ascii_lowercase();
        for (pat, tok) in [
            ("optionally to", Tok::NonIdentifying),
            ("one or zero", Tok::ZeroOrOne),
            ("zero or one", Tok::ZeroOrOne),
            ("one or more", Tok::OneOrMore),
            ("one or many", Tok::OneOrMore),
            ("zero or more", Tok::ZeroOrMore),
            ("zero or many", Tok::ZeroOrMore),
            ("only one", Tok::OnlyOne),
        ] {
            if lower.starts_with(pat) {
                self.pos += pat.len();
                return Some((start, tok, self.pos));
            }
        }

        if lower.starts_with("many(0)") {
            self.pos += "many(0)".len();
            return Some((start, Tok::ZeroOrMore, self.pos));
        }
        if lower.starts_with("many(1)") {
            self.pos += "many(1)".len();
            return Some((start, Tok::OneOrMore, self.pos));
        }
        if lower.starts_with("0+") {
            self.pos += "0+".len();
            return Some((start, Tok::ZeroOrMore, self.pos));
        }
        if lower.starts_with("1+") {
            self.pos += "1+".len();
            return Some((start, Tok::OneOrMore, self.pos));
        }
        if lower.starts_with("many") {
            self.pos += "many".len();
            return Some((start, Tok::ZeroOrMore, self.pos));
        }
        if lower.starts_with("one") {
            self.pos += "one".len();
            return Some((start, Tok::OnlyOne, self.pos));
        }
        if lower.starts_with('1') {
            self.pos += 1;
            return Some((start, Tok::OnlyOne, self.pos));
        }
        if lower.starts_with("to") {
            self.pos += "to".len();
            return Some((start, Tok::Identifying, self.pos));
        }

        for (pat, tok) in [
            ("||", Tok::OnlyOne),
            ("|o", Tok::ZeroOrOne),
            ("o|", Tok::ZeroOrOne),
            ("|{", Tok::OneOrMore),
            ("o{", Tok::ZeroOrMore),
            ("}|", Tok::OneOrMore),
            ("}o", Tok::ZeroOrMore),
        ] {
            if s.starts_with(pat) {
                self.pos += pat.len();
                return Some((start, tok, self.pos));
            }
        }

        if s.starts_with("..") || s.starts_with(".-") || s.starts_with("-.") {
            self.pos += 2;
            return Some((start, Tok::NonIdentifying, self.pos));
        }
        if s.starts_with("--") {
            self.pos += 2;
            return Some((start, Tok::Identifying, self.pos));
        }

        if s.starts_with('u')
            && self
                .input
                .as_bytes()
                .get(self.pos.wrapping_sub(1))
                .copied()
                .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r'))
            && self
                .input
                .as_bytes()
                .get(self.pos + 1)
                .copied()
                .is_some_and(|b| matches!(b, b'-' | b'.'))
        {
            self.pos += 1;
            return Some((start, Tok::MdParent, self.pos));
        }

        None
    }

    fn lex_punct(&mut self) -> Option<(usize, Tok, usize)> {
        let start = self.pos;
        match self.peek()? {
            b'{' => {
                self.pos += 1;
                self.mode = Mode::Block;
                Some((start, Tok::BlockStart, self.pos))
            }
            b'}' => {
                if self.mode != Mode::Block {
                    return None;
                }
                self.pos += 1;
                self.mode = Mode::Default;
                Some((start, Tok::BlockStop, self.pos))
            }
            b'[' => {
                self.pos += 1;
                Some((start, Tok::SquareStart, self.pos))
            }
            b']' => {
                self.pos += 1;
                Some((start, Tok::SquareStop, self.pos))
            }
            b':' => {
                if self.input[self.pos..].starts_with(":::") {
                    self.pos += 3;
                    self.mode = Mode::NeedIdListOnly;
                    return Some((start, Tok::StyleSeparator, self.pos));
                }
                self.pos += 1;
                Some((start, Tok::Colon, self.pos))
            }
            b',' => {
                self.pos += 1;
                Some((start, Tok::Comma, self.pos))
            }
            _ => None,
        }
    }

    fn lex_block_token(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        if self.mode != Mode::Block {
            return None;
        }
        let start = self.pos;
        self.skip_ws_block();
        if self.pos >= self.input.len() {
            return Some(Err(LexError {
                message: "EOF inside attribute block".to_string(),
            }));
        }
        if self.peek() == Some(b'}') {
            return None;
        }
        if self.peek() == Some(b',') {
            self.pos += 1;
            return Some(Ok((start, Tok::Comma, self.pos)));
        }
        if self.peek() == Some(b'"') {
            self.pos += 1;
            let Some(rel_end) = self.input[self.pos..].find('"') else {
                return Some(Err(LexError {
                    message: "Unterminated comment string; missing '\"'".to_string(),
                }));
            };
            let s = self.input[self.pos..self.pos + rel_end].to_string();
            self.pos = self.pos + rel_end + 1;
            return Some(Ok((start, Tok::Comment(s), self.pos)));
        }
        if let Some(two) = self.input[self.pos..].get(..2) {
            let two_upper = two.to_ascii_uppercase();
            if matches!(two_upper.as_str(), "PK" | "FK" | "UK") {
                let prev_ok = self.pos == 0
                    || matches!(
                        self.input.as_bytes()[self.pos - 1],
                        b' ' | b'\t' | b'\r' | b'\n' | b','
                    );
                let next_ok = self
                    .input
                    .as_bytes()
                    .get(self.pos + 2)
                    .copied()
                    .map(|b| b.is_ascii_whitespace() || matches!(b, b',' | b'"' | b'}'))
                    .unwrap_or(true);
                if prev_ok && next_ok {
                    self.pos += 2;
                    return Some(Ok((start, Tok::AttrKey(two_upper), self.pos)));
                }
            }
        }

        let start_word = self.pos;
        let mut end = self.pos;
        for (rel, ch) in self.input[self.pos..].char_indices() {
            if ch.is_whitespace() || matches!(ch, ',' | '"' | '}') {
                break;
            }
            end = self.pos + rel + ch.len_utf8();
        }
        if end == start_word {
            self.pos += self.peek().map(|_| 1).unwrap_or(0);
            return Some(Err(LexError {
                message: format!("Unexpected character inside attribute block at {start_word}"),
            }));
        }
        self.pos = end;
        let raw = &self.input[start_word..end];
        let tilde_count = raw.chars().filter(|&c| c == '~').count();
        if tilde_count >= 2 {
            return Some(Ok((start, Tok::AttrWord(raw.to_string()), self.pos)));
        }

        let mut chars = raw.chars();
        let first = chars.next()?;
        let first_ok = first == '*' || first == '_' || first.is_alphabetic() || !first.is_ascii();
        let rest_ok = chars.all(|c| {
            c == '*'
                || c == '-'
                || c == '_'
                || c.is_ascii_digit()
                || c.is_alphabetic()
                || matches!(c, '[' | ']' | '(' | ')')
                || !c.is_ascii()
        });
        if !first_ok || !rest_ok {
            return Some(Err(LexError {
                message: "Invalid attribute word".to_string(),
            }));
        }
        Some(Ok((start, Tok::AttrWord(raw.to_string()), self.pos)))
    }

    fn lex_name_or_str(&mut self) -> Option<std::result::Result<(usize, Tok, usize), LexError>> {
        if self.mode == Mode::Block {
            return None;
        }
        let start = self.pos;
        if self.peek()? == b'"' {
            self.pos += 1;
            let Some(rel_end) = self.input[self.pos..].find('"') else {
                return Some(Err(LexError {
                    message: "Unterminated string literal; missing '\"'".to_string(),
                }));
            };
            let s = self.input[self.pos..self.pos + rel_end].to_string();
            self.pos = self.pos + rel_end + 1;
            let is_entity_name = !s.is_empty()
                && !s.contains('%')
                && !s.contains('\\')
                && !s.contains('\r')
                && !s.contains('\n')
                && !s.contains('\u{0008}')
                && !s.contains('\u{000B}');
            if is_entity_name {
                return Some(Ok((start, Tok::Name(s), self.pos)));
            }
            return Some(Ok((start, Tok::Str(s), self.pos)));
        }

        let mut end = self.pos;
        for (rel, ch) in self.input[self.pos..].char_indices() {
            let ok = !ch.is_ascii() || ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '*');
            if !ok {
                break;
            }
            end = self.pos + rel + ch.len_utf8();
        }
        if end == self.pos {
            return None;
        }
        let s = self.input[self.pos..end].to_string();
        self.pos = end;
        Some(Ok((start, Tok::Name(s), self.pos)))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = std::result::Result<(usize, Tok, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.pending.pop_front() {
            return Some(Ok(tok));
        }

        loop {
            match self.mode {
                Mode::Block => self.skip_ws_block(),
                _ => self.skip_ws_default(),
            }

            if self.pos >= self.input.len() {
                if self.mode == Mode::Block {
                    return Some(Err(LexError {
                        message: "EOF inside attribute block".to_string(),
                    }));
                }
                return None;
            }

            if self.lex_comment() {
                continue;
            }

            if let Some(tok) = self.lex_block_token() {
                return Some(tok);
            }

            if let Some(tok) = self.lex_rest_of_line() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_newline() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_acc_title() {
                return Some(tok);
            }

            if let Some(tok) = self.lex_acc_descr() {
                return Some(tok);
            }

            if let Some(tok) = self.lex_direction() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_keyword() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_id_list() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_punct() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_rel_tokens() {
                return Some(Ok(tok));
            }

            if let Some(tok) = self.lex_name_or_str() {
                return Some(tok);
            }

            let start = self.pos;
            self.pos += 1;
            return Some(Err(LexError {
                message: format!("Unexpected character at {start}"),
            }));
        }
    }
}
