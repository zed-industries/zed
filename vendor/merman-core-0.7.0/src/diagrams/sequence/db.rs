use crate::ParseMetadata;
use rustc_hash::FxHashMap;
use serde_json::{Value, json};
use std::collections::BTreeMap;

use super::Action;
use super::render_model::{
    SequenceActor, SequenceAutonumber, SequenceBox, SequenceDiagramRenderModel, SequenceMessage,
    SequenceMessagePayload, SequenceNote,
};
use super::{
    LINETYPE_ACTIVE_END, LINETYPE_ACTIVE_START, LINETYPE_AUTONUMBER, LINETYPE_CENTRAL_CONNECTION,
    LINETYPE_CENTRAL_CONNECTION_REVERSE, LINETYPE_NOTE,
};

#[derive(Debug, Clone)]
struct ParsedText {
    text: String,
    wrap: Option<bool>,
}

#[derive(Debug, Clone)]
struct Actor {
    name: String,
    description: String,
    wrap: bool,
    actor_type: String,
    box_index: Option<usize>,
    links: serde_json::Map<String, Value>,
    properties: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone)]
struct Message {
    id: String,
    from: Option<String>,
    to: Option<String>,
    message: SequenceMessagePayload,
    wrap: bool,
    message_type: i32,
    activate: bool,
    placement: Option<i32>,
    central_connection: i32,
}

#[derive(Debug, Clone)]
struct Note {
    actor: Value,
    placement: i32,
    message: String,
    wrap: bool,
}

#[derive(Debug, Clone)]
struct SeqBox {
    name: Option<String>,
    fill: String,
    wrap: bool,
    actor_keys: Vec<String>,
}

#[derive(Debug, Default)]
pub(super) struct SequenceDb {
    actors: FxHashMap<String, Actor>,
    actor_order: Vec<String>,
    messages: Vec<Message>,
    notes: Vec<Note>,
    boxes: Vec<SeqBox>,
    current_box: Option<usize>,
    wrap_enabled: Option<bool>,

    created_actors: FxHashMap<String, usize>,
    destroyed_actors: FxHashMap<String, usize>,
    last_created: Option<String>,
    last_destroyed: Option<String>,

    title: Option<String>,
    acc_title: Option<String>,
    acc_descr: Option<String>,
}

impl SequenceDb {
    pub(super) fn new(wrap_enabled: Option<bool>) -> Self {
        Self {
            wrap_enabled,
            ..Default::default()
        }
    }

    fn auto_wrap(&self) -> bool {
        self.wrap_enabled.unwrap_or(false)
    }

    fn ensure_actor(&mut self, id: &str) {
        if self.actors.contains_key(id) {
            return;
        }
        let id_owned = id.to_string();
        self.actor_order.push(id_owned.clone());
        self.actors.insert(
            id_owned.clone(),
            Actor {
                name: id_owned.clone(),
                description: id_owned,
                wrap: self.auto_wrap(),
                actor_type: "participant".to_string(),
                box_index: None,
                links: serde_json::Map::new(),
                properties: serde_json::Map::new(),
            },
        );
    }

    fn add_actor(
        &mut self,
        id: &str,
        description: Option<String>,
        actor_type: &str,
        config: Option<String>,
    ) -> std::result::Result<(), String> {
        let mut actor_type = actor_type.to_string();
        if let Some(config) = config.as_deref() {
            let meta = parse_participant_meta_yaml(config)?;
            if let serde_yaml::Value::Mapping(m) = meta {
                let key = serde_yaml::Value::String("type".to_string());
                if let Some(v) = m.get(&key) {
                    if let Some(t) = yaml_to_string(v) {
                        actor_type = t;
                    }
                }
            }
        }

        if let Some(current_box) = self.current_box {
            if let Some(existing) = self.actors.get(id) {
                if let Some(old_box) = existing.box_index {
                    if old_box != current_box {
                        let old_name = self.boxes[old_box]
                            .name
                            .clone()
                            .unwrap_or_else(|| "undefined".to_string());
                        let new_name = self.boxes[current_box]
                            .name
                            .clone()
                            .unwrap_or_else(|| "undefined".to_string());
                        return Err(format!(
                            "A same participant should only be defined in one Box: {} can't be in '{}' and in '{}' at the same time.",
                            existing.name, old_name, new_name
                        ));
                    }
                }
            }
        }

        let description = description
            .map(|s| self.parse_message(&s))
            .unwrap_or_else(|| ParsedText {
                text: id.to_string(),
                wrap: None,
            });

        let wrap = description.wrap.unwrap_or(self.auto_wrap());

        if let Some(existing) = self.actors.get_mut(id) {
            existing.description = description.text;
            existing.wrap = wrap;
            existing.actor_type = actor_type;
            if let Some(current_box) = self.current_box {
                if existing.box_index.is_none() {
                    existing.box_index = Some(current_box);
                }
                self.boxes[current_box].actor_keys.push(id.to_string());
            }
            return Ok(());
        }

        self.actor_order.push(id.to_string());
        self.actors.insert(
            id.to_string(),
            Actor {
                name: id.to_string(),
                description: description.text,
                wrap,
                actor_type,
                box_index: self.current_box,
                links: serde_json::Map::new(),
                properties: serde_json::Map::new(),
            },
        );

        if let Some(current_box) = self.current_box {
            self.boxes[current_box].actor_keys.push(id.to_string());
        }

        Ok(())
    }

    fn parse_message(&self, raw: &str) -> ParsedText {
        let trimmed = raw.trim();
        fn strip_prefix_ci<'a>(s: &'a str, prefix: &[u8]) -> Option<&'a str> {
            let bytes = s.as_bytes();
            if bytes.len() < prefix.len() {
                return None;
            }
            for i in 0..prefix.len() {
                if !bytes[i].eq_ignore_ascii_case(&prefix[i]) {
                    return None;
                }
            }
            Some(&s[prefix.len()..])
        }

        let (wrap, cleaned) = if trimmed.len() >= 5
            && matches!(
                trimmed.as_bytes().first().copied(),
                Some(b':' | b'w' | b'W' | b'n' | b'N')
            ) {
            if let Some(rest) = strip_prefix_ci(trimmed, b":wrap:") {
                (Some(true), rest.trim())
            } else if let Some(rest) = strip_prefix_ci(trimmed, b"wrap:") {
                (Some(true), rest.trim())
            } else if let Some(rest) = strip_prefix_ci(trimmed, b":nowrap:") {
                (Some(false), rest.trim())
            } else if let Some(rest) = strip_prefix_ci(trimmed, b"nowrap:") {
                (Some(false), rest.trim())
            } else {
                (None, trimmed)
            }
        } else {
            (None, trimmed)
        };

        ParsedText {
            text: cleaned.to_string(),
            wrap,
        }
    }

    fn add_signal(
        &mut self,
        from: Option<String>,
        to: Option<String>,
        message: Option<ParsedText>,
        message_type: i32,
        activate: bool,
        placement: Option<i32>,
        central_connection: i32,
    ) {
        let msg_text = message.unwrap_or(ParsedText {
            text: String::new(),
            wrap: None,
        });
        let wrap = msg_text.wrap.unwrap_or(self.auto_wrap());

        self.messages.push(Message {
            id: self.messages.len().to_string(),
            from,
            to,
            message: SequenceMessagePayload::Text(msg_text.text),
            wrap,
            message_type,
            activate,
            placement,
            central_connection,
        });
    }

    fn activation_count(&self, actor: &str) -> i32 {
        if actor.is_empty() {
            return 0;
        }
        let mut count = 0;
        for msg in &self.messages {
            if msg.message_type == LINETYPE_ACTIVE_START
                && msg.from.as_deref().is_some_and(|a| a == actor)
            {
                count += 1;
            }
            if msg.message_type == LINETYPE_ACTIVE_END
                && msg.from.as_deref().is_some_and(|a| a == actor)
            {
                count -= 1;
            }
        }
        count
    }

    fn add_autonumber(&mut self, start: Option<f64>, step: Option<f64>, visible: bool) {
        self.messages.push(Message {
            id: self.messages.len().to_string(),
            from: None,
            to: None,
            message: SequenceMessagePayload::Autonumber(SequenceAutonumber {
                start,
                step,
                visible,
            }),
            wrap: false,
            message_type: LINETYPE_AUTONUMBER,
            activate: false,
            placement: None,
            central_connection: 0,
        });
    }

    fn add_note(&mut self, actors: Vec<String>, placement: i32, raw_text: String) {
        let parsed = self.parse_message(&raw_text);
        let wrap = parsed.wrap.unwrap_or(self.auto_wrap());

        let actor_value = match actors.as_slice() {
            [a] => Value::String(a.clone()),
            [a, b] => json!([a, b]),
            _ => json!(actors),
        };

        self.notes.push(Note {
            actor: actor_value.clone(),
            placement,
            message: parsed.text.clone(),
            wrap,
        });

        let (from, to) = match actors.as_slice() {
            [a] => (Some(a.clone()), Some(a.clone())),
            [a, b] => (Some(a.clone()), Some(b.clone())),
            _ => (
                actors.first().cloned(),
                actors.get(1).cloned().or_else(|| actors.first().cloned()),
            ),
        };

        self.messages.push(Message {
            id: self.messages.len().to_string(),
            from,
            to,
            message: SequenceMessagePayload::Text(parsed.text),
            wrap,
            message_type: LINETYPE_NOTE,
            activate: false,
            placement: Some(placement),
            central_connection: 0,
        });
    }

    pub(super) fn apply(&mut self, action: Action) -> std::result::Result<(), String> {
        match action {
            Action::SetTitle(t) => {
                self.title = Some(t.trim().to_string());
                Ok(())
            }
            Action::SetAccTitle(t) => {
                self.acc_title = Some(t.trim().to_string());
                Ok(())
            }
            Action::SetAccDescr(t) => {
                self.acc_descr = Some(t.trim().to_string());
                Ok(())
            }

            Action::EnsureParticipant { id } => {
                self.ensure_actor(&id);
                Ok(())
            }
            Action::AddParticipant {
                id,
                description,
                draw,
                config,
            } => self.add_actor(&id, description, &draw, config),

            Action::CreateParticipant {
                id,
                description,
                draw,
                config,
            } => {
                if self.actors.contains_key(&id) {
                    return Err("It is not possible to have actors with the same id, even if one is destroyed before the next is created. Use 'AS' aliases to simulate the behavior".to_string());
                }
                self.last_created = Some(id.clone());
                self.add_actor(&id, description, &draw, config)?;
                self.created_actors.insert(id, self.messages.len());
                Ok(())
            }
            Action::DestroyParticipant { id } => {
                self.last_destroyed = Some(id.clone());
                self.destroyed_actors.insert(id, self.messages.len());
                Ok(())
            }

            Action::ControlSignal { signal_type, text } => {
                let msg = text.as_deref().map(|t| self.parse_message(t));
                self.add_signal(None, None, msg, signal_type, false, None, 0);
                Ok(())
            }

            Action::BoxStart { header } => {
                self.add_box(&header);
                Ok(())
            }
            Action::BoxEnd => {
                self.current_box = None;
                Ok(())
            }

            Action::AddLinks { actor, text } => {
                self.add_links(&actor, &text);
                Ok(())
            }
            Action::AddLink { actor, text } => {
                self.add_link(&actor, &text);
                Ok(())
            }
            Action::AddProperties { actor, text } => {
                self.add_properties(&actor, &text);
                Ok(())
            }
            Action::AddDetails { actor, text } => {
                let _ = (actor, text);
                Ok(())
            }

            Action::AddMessage {
                from,
                to,
                signal_type,
                text,
                activate,
                central_connection,
            } => {
                if let Some(last_created) = self.last_created.clone() {
                    if to != last_created {
                        return Err(format!(
                            "The created participant {last_created} does not have an associated creating message after its declaration. Please check the sequence diagram."
                        ));
                    }
                    self.last_created = None;
                } else if let Some(last_destroyed) = self.last_destroyed.clone() {
                    if from != last_destroyed && to != last_destroyed {
                        return Err(format!(
                            "The destroyed participant {last_destroyed} does not have an associated destroying message after its declaration. Please check the sequence diagram."
                        ));
                    }
                    self.last_destroyed = None;
                }

                let msg = self.parse_message(&text);
                self.add_signal(
                    Some(from),
                    Some(to),
                    Some(msg),
                    signal_type,
                    activate,
                    None,
                    central_connection,
                );
                Ok(())
            }

            Action::ActiveStart { actor } => {
                self.add_signal(
                    Some(actor),
                    None,
                    None,
                    LINETYPE_ACTIVE_START,
                    false,
                    None,
                    0,
                );
                Ok(())
            }
            Action::ActiveEnd { actor } => {
                if self.activation_count(&actor) < 1 {
                    return Err(format!(
                        "Trying to inactivate an inactive participant ({actor})"
                    ));
                }
                self.add_signal(Some(actor), None, None, LINETYPE_ACTIVE_END, false, None, 0);
                Ok(())
            }
            Action::CentralConnection { actor } => {
                self.add_signal(
                    Some(actor),
                    None,
                    None,
                    LINETYPE_CENTRAL_CONNECTION,
                    false,
                    None,
                    0,
                );
                Ok(())
            }
            Action::CentralConnectionReverse { actor } => {
                self.add_signal(
                    Some(actor),
                    None,
                    None,
                    LINETYPE_CENTRAL_CONNECTION_REVERSE,
                    false,
                    None,
                    0,
                );
                Ok(())
            }

            Action::AddNote {
                actors,
                placement,
                text,
            } => {
                self.add_note(actors, placement, text);
                Ok(())
            }

            Action::Autonumber {
                start,
                step,
                visible,
            } => {
                self.add_autonumber(start, step, visible);
                Ok(())
            }
        }
    }

    fn add_box(&mut self, raw: &str) {
        let data = self.parse_box_data(raw);
        let wrap = data.wrap.unwrap_or(self.auto_wrap());
        self.boxes.push(SeqBox {
            name: data.text,
            fill: data.color,
            wrap,
            actor_keys: Vec::new(),
        });
        self.current_box = Some(self.boxes.len() - 1);
    }

    fn parse_box_data(&self, raw: &str) -> BoxData {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return BoxData {
                text: None,
                color: "transparent".to_string(),
                wrap: None,
            };
        }

        let (color_candidate, title_candidate) = split_color_and_title(trimmed);
        let mut color = if color_candidate.trim().is_empty() {
            "transparent".to_string()
        } else {
            color_candidate.trim().to_string()
        };
        let mut title = title_candidate.trim().to_string();

        if !is_css_color_value(&color) {
            color = "transparent".to_string();
            title = trimmed.to_string();
        }

        let parsed_title = self.parse_message(&title);
        let text = if parsed_title.text.is_empty() {
            None
        } else {
            Some(parsed_title.text)
        };

        BoxData {
            text,
            color,
            wrap: parsed_title.wrap,
        }
    }

    fn add_links(&mut self, actor: &str, raw_text: &str) {
        let s = unescape_entities(raw_text);
        let Ok(v) = serde_json::from_str::<Value>(&s) else {
            return;
        };
        let Some(obj) = v.as_object() else {
            return;
        };
        let Some(a) = self.actors.get_mut(actor) else {
            return;
        };
        for (k, v) in obj {
            if let Some(url) = v.as_str() {
                a.links.insert(k.clone(), Value::String(url.to_string()));
            }
        }
    }

    fn add_link(&mut self, actor: &str, raw_text: &str) {
        let s = unescape_entities(raw_text);
        let Some(idx) = s.find('@') else {
            return;
        };
        let (left, right) = s.split_at(idx);
        let label = left.strip_suffix(' ').unwrap_or(left).trim();
        let url = right.trim_start_matches('@').trim();
        if label.is_empty() || url.is_empty() {
            return;
        }
        let Some(a) = self.actors.get_mut(actor) else {
            return;
        };
        a.links
            .insert(label.to_string(), Value::String(url.to_string()));
    }

    fn add_properties(&mut self, actor: &str, raw_text: &str) {
        let s = unescape_entities(raw_text);
        let Ok(v) = serde_json::from_str::<Value>(&s) else {
            return;
        };
        let Some(obj) = v.as_object() else {
            return;
        };
        let Some(a) = self.actors.get_mut(actor) else {
            return;
        };
        for (k, v) in obj {
            a.properties.insert(k.clone(), v.clone());
        }
    }

    pub(super) fn into_model(self, meta: &ParseMetadata) -> Value {
        self.into_render_model().to_compat_json(&meta.diagram_type)
    }

    pub(super) fn into_render_model(mut self) -> SequenceDiagramRenderModel {
        let mut actors = std::mem::take(&mut self.actors);
        let mut actors_typed = BTreeMap::new();
        for id in &self.actor_order {
            if let Some(a) = actors.remove(id) {
                actors_typed.insert(
                    id.clone(),
                    SequenceActor {
                        name: a.name,
                        description: a.description,
                        wrap: a.wrap,
                        actor_type: a.actor_type,
                        links: a.links,
                        properties: a.properties,
                    },
                );
            }
        }

        let messages = std::mem::take(&mut self.messages)
            .into_iter()
            .map(|m| SequenceMessage {
                id: m.id,
                from: m.from,
                to: m.to,
                message: m.message,
                wrap: m.wrap,
                message_type: m.message_type,
                activate: m.activate,
                placement: m.placement,
                central_connection: m.central_connection,
            })
            .collect();

        let notes = std::mem::take(&mut self.notes)
            .into_iter()
            .map(|n| SequenceNote {
                actor: n.actor,
                placement: n.placement,
                message: n.message,
                wrap: n.wrap,
            })
            .collect();

        let boxes = std::mem::take(&mut self.boxes)
            .into_iter()
            .map(|b| SequenceBox {
                name: b.name,
                wrap: b.wrap,
                fill: b.fill,
                actor_keys: b.actor_keys,
            })
            .collect();

        SequenceDiagramRenderModel {
            title: self.title,
            acc_title: self.acc_title,
            acc_descr: self.acc_descr,
            actor_order: std::mem::take(&mut self.actor_order),
            actors: actors_typed,
            messages,
            notes,
            boxes,
            created_actors: std::mem::take(&mut self.created_actors)
                .into_iter()
                .collect(),
            destroyed_actors: std::mem::take(&mut self.destroyed_actors)
                .into_iter()
                .collect(),
        }
    }
}

pub(super) fn fast_parse_sequence_signals_only_db(
    code: &str,
    wrap_enabled: Option<bool>,
) -> Option<SequenceDb> {
    // Fast-path for very small sequence diagrams that only contain signal statements.
    //
    // This avoids the LALRPOP parser + token stream overhead for tiny inputs, while preserving
    // correctness by falling back to the full parser on anything unrecognized.
    //
    // Current target fixture: benches `sequence_tiny`:
    //
    //   sequenceDiagram
    //     Alice->>Bob: Hi
    //
    // Keep the fast-path conservative to avoid surprising behavior differences.
    if code.len() > 256 {
        return None;
    }

    fn eq_ascii_ci(a: &str, b: &str) -> bool {
        a.eq_ignore_ascii_case(b)
    }

    #[derive(Clone, Copy)]
    enum Activation {
        None,
        Plus,
        Minus,
    }

    struct Signal<'a> {
        from: &'a str,
        to: &'a str,
        ty: i32,
        text: &'a str,
        activation: Activation,
    }

    fn parse_signal_line(line: &str) -> Option<Signal<'_>> {
        let s = line.trim();
        if s.is_empty() {
            return None;
        }
        // Keep the fast-path strict: semicolons are handled by the full lexer/comment rules.
        if s.contains(';') {
            return None;
        }
        if s.contains("()") {
            return None;
        }

        let bytes = s.as_bytes();
        let mut sig_start: Option<usize> = None;
        let mut i = 0usize;
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'<' {
                sig_start = Some(i);
                break;
            }
            if b == b'-' {
                let next = bytes.get(i + 1).copied();
                if matches!(next, Some(b'-' | b'>' | b'x' | b')')) {
                    sig_start = Some(i);
                    break;
                }
            }
            i += 1;
        }
        let sig_start = sig_start?;
        let from = s[..sig_start].trim();
        if from.is_empty() {
            return None;
        }

        let rest = &s[sig_start..];
        let (sig_len, ty) = if rest.starts_with("<<-->>") {
            (6, 34)
        } else if rest.starts_with("<<->>") {
            (5, 33)
        } else if rest.starts_with("-->>") {
            (4, 1)
        } else if rest.starts_with("->>") {
            (3, 0)
        } else if rest.starts_with("-->") {
            (3, 6)
        } else if rest.starts_with("->") {
            (2, 5)
        } else if rest.starts_with("--x") {
            (3, 4)
        } else if rest.starts_with("-x") {
            (2, 3)
        } else if rest.starts_with("--)") {
            (3, 25)
        } else if rest.starts_with("-)") {
            (2, 24)
        } else {
            return None;
        };

        let mut p = sig_start + sig_len;
        while p < bytes.len() && bytes[p].is_ascii_whitespace() {
            p += 1;
        }

        let activation = match bytes.get(p).copied() {
            Some(b'+') => {
                p += 1;
                Activation::Plus
            }
            Some(b'-') => {
                p += 1;
                Activation::Minus
            }
            _ => Activation::None,
        };

        while p < bytes.len() && bytes[p].is_ascii_whitespace() {
            p += 1;
        }

        let to_start = p;
        while p < bytes.len() {
            let b = bytes[p];
            if b.is_ascii_whitespace() || b == b':' {
                break;
            }
            p += 1;
        }
        let to = s[to_start..p].trim();
        if to.is_empty() {
            return None;
        }

        while p < bytes.len() && bytes[p].is_ascii_whitespace() {
            p += 1;
        }
        if bytes.get(p).copied()? != b':' {
            return None;
        }
        p += 1;
        let text = s[p..].trim();

        Some(Signal {
            from,
            to,
            ty,
            text,
            activation,
        })
    }

    let mut header_seen = false;
    let mut non_empty_lines = 0usize;
    let mut signals: Vec<Signal<'_>> = Vec::with_capacity(2);
    for raw in code.lines() {
        let t = raw.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with("%%") {
            continue;
        }
        non_empty_lines += 1;
        if non_empty_lines > 8 {
            return None;
        }
        if !header_seen {
            if !eq_ascii_ci(t, "sequenceDiagram") {
                return None;
            }
            header_seen = true;
            continue;
        }
        let sig = parse_signal_line(t)?;
        signals.push(sig);
        if signals.len() > 4 {
            return None;
        }
    }

    if signals.is_empty() {
        return None;
    }

    let mut db = SequenceDb::new(wrap_enabled);
    for sig in signals {
        db.ensure_actor(sig.from);
        db.ensure_actor(sig.to);

        let activate = matches!(sig.activation, Activation::Plus);
        let msg = db.parse_message(sig.text);
        db.add_signal(
            Some(sig.from.to_string()),
            Some(sig.to.to_string()),
            Some(msg),
            sig.ty,
            activate,
            None,
            0,
        );

        match sig.activation {
            Activation::Plus => {
                db.add_signal(
                    Some(sig.to.to_string()),
                    None,
                    None,
                    LINETYPE_ACTIVE_START,
                    false,
                    None,
                    0,
                );
            }
            Activation::Minus => {
                if db.activation_count(sig.from) < 1 {
                    return None;
                }
                db.add_signal(
                    Some(sig.from.to_string()),
                    None,
                    None,
                    LINETYPE_ACTIVE_END,
                    false,
                    None,
                    0,
                );
            }
            Activation::None => {}
        }
    }

    Some(db)
}

#[derive(Debug, Clone)]
struct BoxData {
    text: Option<String>,
    color: String,
    wrap: Option<bool>,
}

fn unescape_entities(input: &str) -> String {
    input.replace("&equals;", "=").replace("&amp;", "&")
}

fn split_color_and_title(input: &str) -> (&str, &str) {
    let lower = input.to_ascii_lowercase();
    for prefix in ["rgba", "rgb", "hsla", "hsl"] {
        if lower.starts_with(prefix) {
            if let Some(end) = input.find(')') {
                let color = &input[..=end];
                let rest = &input[end + 1..];
                return (color.trim(), rest);
            }
        }
    }

    let mut end = 0usize;
    for (idx, c) in input.char_indices() {
        if c.is_ascii_alphanumeric() || c == '_' {
            end = idx + c.len_utf8();
            continue;
        }
        break;
    }
    (&input[..end], &input[end..])
}

fn parse_participant_meta_yaml(yaml_body: &str) -> std::result::Result<serde_yaml::Value, String> {
    let yaml_data = if yaml_body.contains('\n') {
        format!("{yaml_body}\n")
    } else {
        format!("{{\n{yaml_body}\n}}")
    };
    serde_yaml::from_str(&yaml_data).map_err(|e| format!("{e}"))
}

fn yaml_to_string(v: &serde_yaml::Value) -> Option<String> {
    match v {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        serde_yaml::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn is_css_color_value(input: &str) -> bool {
    let t = input.trim();
    if t.is_empty() {
        return false;
    }
    let lower = t.to_ascii_lowercase();
    if lower == "transparent" {
        return true;
    }
    if (lower.starts_with("rgb(")
        || lower.starts_with("rgba(")
        || lower.starts_with("hsl(")
        || lower.starts_with("hsla("))
        && lower.ends_with(')')
    {
        return true;
    }
    if !lower.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    CSS_COLOR_KEYWORDS.binary_search(&lower.as_str()).is_ok()
}

static CSS_COLOR_KEYWORDS: &[&str] = &[
    "aliceblue",
    "antiquewhite",
    "aqua",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "black",
    "blanchedalmond",
    "blue",
    "blueviolet",
    "brown",
    "burlywood",
    "cadetblue",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "cyan",
    "darkblue",
    "darkcyan",
    "darkgoldenrod",
    "darkgray",
    "darkgreen",
    "darkgrey",
    "darkkhaki",
    "darkmagenta",
    "darkolivegreen",
    "darkorange",
    "darkorchid",
    "darkred",
    "darksalmon",
    "darkseagreen",
    "darkslateblue",
    "darkslategray",
    "darkslategrey",
    "darkturquoise",
    "darkviolet",
    "deeppink",
    "deepskyblue",
    "dimgray",
    "dimgrey",
    "dodgerblue",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "fuchsia",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "gray",
    "green",
    "greenyellow",
    "grey",
    "honeydew",
    "hotpink",
    "indianred",
    "indigo",
    "ivory",
    "khaki",
    "lavender",
    "lavenderblush",
    "lawngreen",
    "lemonchiffon",
    "lightblue",
    "lightcoral",
    "lightcyan",
    "lightgoldenrodyellow",
    "lightgray",
    "lightgreen",
    "lightgrey",
    "lightpink",
    "lightsalmon",
    "lightseagreen",
    "lightskyblue",
    "lightslategray",
    "lightslategrey",
    "lightsteelblue",
    "lightyellow",
    "lime",
    "limegreen",
    "linen",
    "magenta",
    "maroon",
    "mediumaquamarine",
    "mediumblue",
    "mediumorchid",
    "mediumpurple",
    "mediumseagreen",
    "mediumslateblue",
    "mediumspringgreen",
    "mediumturquoise",
    "mediumvioletred",
    "midnightblue",
    "mintcream",
    "mistyrose",
    "moccasin",
    "navajowhite",
    "navy",
    "oldlace",
    "olive",
    "olivedrab",
    "orange",
    "orangered",
    "orchid",
    "palegoldenrod",
    "palegreen",
    "paleturquoise",
    "palevioletred",
    "papayawhip",
    "peachpuff",
    "peru",
    "pink",
    "plum",
    "powderblue",
    "purple",
    "rebeccapurple",
    "red",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "sienna",
    "silver",
    "skyblue",
    "slateblue",
    "slategray",
    "slategrey",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "teal",
    "thistle",
    "tomato",
    "turquoise",
    "violet",
    "wheat",
    "white",
    "whitesmoke",
    "yellow",
    "yellowgreen",
];
