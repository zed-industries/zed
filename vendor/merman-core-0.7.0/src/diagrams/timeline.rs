use crate::{Error, ParseMetadata, Result};
use serde_json::{Value, json};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimelineRenderTask {
    pub id: i64,
    pub section: String,
    #[serde(rename = "type")]
    pub task_type: String,
    pub task: String,
    pub score: i64,
    #[serde(default)]
    pub events: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct TimelineDiagramRenderModel {
    pub title: Option<String>,
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default)]
    pub sections: Vec<String>,
    #[serde(default)]
    pub tasks: Vec<TimelineRenderTask>,
}

impl TimelineDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Default)]
struct TimelineDb {
    title: String,
    acc_title: String,
    acc_descr: String,

    current_section: String,
    sections: Vec<String>,
    tasks: Vec<TimelineRenderTask>,
    next_id: i64,
}

impl TimelineDb {
    fn clear(&mut self) {
        *self = Self::default();
    }

    fn add_section(&mut self, txt: &str) {
        self.current_section = txt.to_string();
        self.sections.push(txt.to_string());
    }

    fn add_task(&mut self, period: &str) {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(TimelineRenderTask {
            id,
            section: self.current_section.clone(),
            task_type: self.current_section.clone(),
            task: period.to_string(),
            score: 0,
            events: Vec::new(),
        });
    }

    fn add_event(&mut self, event: &str) -> Result<()> {
        let Some(last) = self.tasks.last_mut() else {
            return Err(Error::DiagramParse {
                diagram_type: "timeline".to_string(),
                message: "event without a preceding task".to_string(),
            });
        };
        last.events.push(event.to_string());
        Ok(())
    }
}

enum TimelineParseOutput {
    Empty,
    Model(TimelineDiagramRenderModel),
}

fn starts_with_ci(s: &str, prefix: &str) -> bool {
    s.get(..prefix.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
}

fn parse_keyword_arg_full_line_after_one_ws<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let t = line.trim_start();
    if !starts_with_ci(t, keyword) {
        return None;
    }
    let after = &t[keyword.len()..];
    let ws = after.chars().next()?;
    if !ws.is_whitespace() {
        return None;
    }
    Some(&after[ws.len_utf8()..])
}

fn parse_title_value(line: &str) -> Option<String> {
    let rest = parse_keyword_arg_full_line_after_one_ws(line, "title")?;
    Some(rest.to_string())
}

fn parse_section_value(line: &str) -> Option<String> {
    let rest = parse_keyword_arg_full_line_after_one_ws(line, "section")?;
    let end = rest.find(':').unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

fn split_statement_suffix_hash_or_semi(s: &str) -> &str {
    let mut end = s.len();
    for (i, c) in s.char_indices() {
        if c == '#' || c == ';' {
            end = i;
            break;
        }
    }
    &s[..end]
}

fn parse_key_colon_value_hash_or_semi(line: &str, key: &str) -> Option<String> {
    let t = line.trim_start();
    if !starts_with_ci(t, key) {
        return None;
    }
    let rest = t[key.len()..].trim_start();
    let rest = rest.strip_prefix(':')?;
    Some(split_statement_suffix_hash_or_semi(rest).trim().to_string())
}

fn parse_acc_descr_block(lines: &mut std::str::Lines<'_>, first_line: &str) -> Option<String> {
    let t = first_line.trim_start();
    if !starts_with_ci(t, "accDescr") {
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

fn split_events_from_colon_whitespace(input: &str) -> Result<Vec<String>> {
    let mut s = input;
    let mut out = Vec::new();

    while !s.is_empty() {
        let Some(colon) = s.find(':') else {
            return Err(Error::DiagramParse {
                diagram_type: "timeline".to_string(),
                message: format!("invalid event token: {input}"),
            });
        };
        if colon != 0 {
            return Err(Error::DiagramParse {
                diagram_type: "timeline".to_string(),
                message: format!("invalid event token: {input}"),
            });
        }
        let after_colon = &s[1..];
        let Some(ws) = after_colon.chars().next() else {
            return Err(Error::DiagramParse {
                diagram_type: "timeline".to_string(),
                message: "invalid event token: missing whitespace after ':'".to_string(),
            });
        };
        if !ws.is_whitespace() {
            return Err(Error::DiagramParse {
                diagram_type: "timeline".to_string(),
                message: "invalid event token: missing whitespace after ':'".to_string(),
            });
        }
        s = &after_colon[ws.len_utf8()..];

        let mut next_boundary: Option<usize> = None;
        for (i, ch) in s.char_indices() {
            if ch != ':' {
                continue;
            }
            let Some(next) = s[i + 1..].chars().next() else {
                continue;
            };
            if next.is_whitespace() {
                next_boundary = Some(i);
                break;
            }
        }

        let (event, rest) = match next_boundary {
            Some(i) => (&s[..i], &s[i..]),
            None => (s, ""),
        };
        out.push(event.to_string());
        s = rest;
    }

    Ok(out)
}

pub fn parse_timeline(code: &str, meta: &ParseMetadata) -> Result<Value> {
    match parse_timeline_model(code, meta)? {
        TimelineParseOutput::Empty => Ok(json!({})),
        TimelineParseOutput::Model(model) => Ok(json!({
            "type": meta.diagram_type,
            "title": model.title,
            "accTitle": model.acc_title,
            "accDescr": model.acc_descr,
            "sections": model.sections,
            "tasks": model.tasks,
        })),
    }
}

pub fn parse_timeline_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<TimelineDiagramRenderModel> {
    match parse_timeline_model(code, meta)? {
        TimelineParseOutput::Empty => Ok(TimelineDiagramRenderModel::default()),
        TimelineParseOutput::Model(model) => Ok(model),
    }
}

fn parse_timeline_model(code: &str, meta: &ParseMetadata) -> Result<TimelineParseOutput> {
    let mut db = TimelineDb::default();
    db.clear();

    let mut lines = code.lines();
    let mut header_seen = false;

    while let Some(line) = lines.next() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }

        if !header_seen {
            if starts_with_ci(t, "timeline") {
                header_seen = true;
                let rest = t["timeline".len()..].trim_start();
                if !rest.is_empty() {
                    return Err(Error::DiagramParse {
                        diagram_type: meta.diagram_type.clone(),
                        message: "unexpected content after timeline header".to_string(),
                    });
                }
                continue;
            }
            return Err(Error::DiagramParse {
                diagram_type: meta.diagram_type.clone(),
                message: "expected timeline header".to_string(),
            });
        }

        let stripped = line.trim_start();
        if stripped.starts_with('#') {
            continue;
        }

        if let Some(v) = parse_title_value(line) {
            db.title = v;
            continue;
        }
        if let Some(v) = parse_key_colon_value_hash_or_semi(line, "accTitle") {
            db.acc_title = v;
            continue;
        }
        if let Some(v) = parse_key_colon_value_hash_or_semi(line, "accDescr") {
            db.acc_descr = v;
            continue;
        }
        if let Some(v) = parse_acc_descr_block(&mut lines, line) {
            db.acc_descr = v;
            continue;
        }
        if let Some(v) = parse_section_value(line) {
            db.add_section(&v);
            continue;
        }

        let trimmed = stripped;
        if trimmed.starts_with(':') {
            let events = split_events_from_colon_whitespace(trimmed)?;
            for e in events {
                db.add_event(&e)?;
            }
            continue;
        }

        let mut end = trimmed.len();
        for (i, ch) in trimmed.char_indices() {
            if ch == ':' || ch == '#' {
                end = i;
                break;
            }
        }
        let period = trimmed[..end].to_string();
        if period.trim().is_empty() {
            continue;
        }
        db.add_task(&period);

        let rest = &trimmed[end..];
        if rest.starts_with('#') {
            continue;
        }
        if rest.is_empty() {
            continue;
        }
        if rest.starts_with(':') {
            let events = split_events_from_colon_whitespace(rest)?;
            for e in events {
                db.add_event(&e)?;
            }
            continue;
        }
        return Err(Error::DiagramParse {
            diagram_type: meta.diagram_type.clone(),
            message: format!("unrecognized statement: {trimmed}"),
        });
    }

    if !header_seen {
        return Ok(TimelineParseOutput::Empty);
    }

    Ok(TimelineParseOutput::Model(TimelineDiagramRenderModel {
        title: if db.title.is_empty() {
            None
        } else {
            Some(db.title)
        },
        acc_title: if db.acc_title.is_empty() {
            None
        } else {
            Some(db.acc_title)
        },
        acc_descr: if db.acc_descr.is_empty() {
            None
        } else {
            Some(db.acc_descr)
        },
        sections: db.sections,
        tasks: db.tasks,
    }))
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
    fn timeline_simple_section_definition() {
        let model = parse(
            r#"
timeline
section abc-123
"#,
        );
        assert_eq!(model["sections"][0].as_str().unwrap(), "abc-123");
    }

    #[test]
    fn timeline_section_with_two_tasks() {
        let model = parse(
            r#"
timeline
section abc-123
task1
task2
"#,
        );
        let tasks = model["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 2);
        for task in tasks {
            assert_eq!(task["section"].as_str().unwrap(), "abc-123");
            assert!(matches!(task["task"].as_str().unwrap(), "task1" | "task2"));
        }
    }

    #[test]
    fn timeline_two_sections_and_two_tasks_each() {
        let model = parse(
            r#"
timeline
section abc-123
task1
task2
section abc-456
task3
task4
"#,
        );
        assert_eq!(
            model["sections"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect::<Vec<_>>(),
            vec!["abc-123".to_string(), "abc-456".to_string()]
        );

        let tasks = model["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 4);
        for t in tasks {
            let section = t["section"].as_str().unwrap();
            let task = t["task"].as_str().unwrap().trim();
            assert!(matches!(section, "abc-123" | "abc-456"));
            assert!(matches!(task, "task1" | "task2" | "task3" | "task4"));
            if section == "abc-123" {
                assert!(matches!(task, "task1" | "task2"));
            } else {
                assert!(matches!(task, "task3" | "task4"));
            }
        }
    }

    #[test]
    fn timeline_tasks_and_events() {
        let model = parse(
            r#"
timeline
section abc-123
task1: event1
task2: event2: event3
"#,
        );
        let tasks = model["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 2);
        for t in tasks {
            let task = t["task"].as_str().unwrap().trim();
            match task {
                "task1" => {
                    assert_eq!(
                        t["events"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|v| v.as_str().unwrap().to_string())
                            .collect::<Vec<_>>(),
                        vec!["event1".to_string()]
                    );
                }
                "task2" => {
                    assert_eq!(
                        t["events"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|v| v.as_str().unwrap().to_string())
                            .collect::<Vec<_>>(),
                        vec!["event2".to_string(), "event3".to_string()]
                    );
                }
                _ => panic!("unexpected task: {task}"),
            }
        }
    }

    #[test]
    fn timeline_events_support_markdown_link() {
        let model = parse(
            r#"
timeline
section abc-123
task1: [event1](http://example.com)
task2: event2: event3
"#,
        );
        let tasks = model["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 2);
        for t in tasks {
            let task = t["task"].as_str().unwrap().trim();
            match task {
                "task1" => {
                    assert_eq!(
                        t["events"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|v| v.as_str().unwrap().to_string())
                            .collect::<Vec<_>>(),
                        vec!["[event1](http://example.com)".to_string()]
                    );
                }
                "task2" => {}
                _ => panic!("unexpected task: {task}"),
            }
        }
    }

    #[test]
    fn timeline_multiline_events_are_attached_to_previous_task() {
        let model = parse(
            r#"
timeline
section abc-123
task1: event1
task2: event2: event3
     : event4: event5
"#,
        );
        let tasks = model["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 2);
        for t in tasks {
            let task = t["task"].as_str().unwrap().trim();
            match task {
                "task1" => {
                    assert_eq!(t["events"].as_array().unwrap().len(), 1);
                }
                "task2" => {
                    assert_eq!(
                        t["events"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|v| v.as_str().unwrap().to_string())
                            .collect::<Vec<_>>(),
                        vec![
                            "event2".to_string(),
                            "event3".to_string(),
                            "event4".to_string(),
                            "event5".to_string()
                        ]
                    );
                }
                _ => panic!("unexpected task: {task}"),
            }
        }
    }

    #[test]
    fn timeline_allows_semicolons_in_title_section_and_events() {
        let model = parse(
            r#"
timeline
title ;my;title;
section ;a;bc-123;
;ta;sk1;: ;ev;ent1; : ;ev;ent2; : ;ev;ent3;
"#,
        );
        assert_eq!(model["title"].as_str().unwrap(), ";my;title;");
        assert_eq!(model["sections"][0].as_str().unwrap(), ";a;bc-123;");

        let tasks = model["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 1);
        let events = tasks[0]["events"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            events,
            vec![
                ";ev;ent1; ".to_string(),
                ";ev;ent2; ".to_string(),
                ";ev;ent3;".to_string()
            ]
        );
    }

    #[test]
    fn timeline_allows_hashtags_in_title_section_and_events() {
        let model = parse(
            r#"
timeline
title #my#title#
section #a#bc-123#
task1: #ev#ent1# : #ev#ent2# : #ev#ent3#
"#,
        );
        assert_eq!(model["title"].as_str().unwrap(), "#my#title#");
        assert_eq!(model["sections"][0].as_str().unwrap(), "#a#bc-123#");

        let tasks = model["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0]["task"].as_str().unwrap(), "task1");
        let events = tasks[0]["events"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            events,
            vec![
                "#ev#ent1# ".to_string(),
                "#ev#ent2# ".to_string(),
                "#ev#ent3#".to_string()
            ]
        );
    }
}
