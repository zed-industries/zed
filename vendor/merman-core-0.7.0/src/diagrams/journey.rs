use crate::{Error, ParseMetadata, Result};
use serde_json::{Value, json};
use std::collections::BTreeSet;

fn is_false(v: &bool) -> bool {
    !*v
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JourneyRenderTask {
    pub score: i64,
    #[serde(default, rename = "scoreIsNaN", skip_serializing_if = "is_false")]
    pub score_is_nan: bool,
    #[serde(default)]
    pub people: Vec<String>,
    pub section: String,
    #[serde(rename = "type")]
    pub task_type: String,
    pub task: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct JourneyDiagramRenderModel {
    pub title: Option<String>,
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default)]
    pub sections: Vec<String>,
    #[serde(default)]
    pub tasks: Vec<JourneyRenderTask>,
    #[serde(default)]
    pub actors: Vec<String>,
}

impl JourneyDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_title(&mut self.title, config);
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Default)]
struct JourneyDb {
    title: String,
    acc_title: String,
    acc_descr: String,

    current_section: String,
    sections: Vec<String>,
    tasks: Vec<JourneyRenderTask>,
}

impl JourneyDb {
    fn clear(&mut self) {
        *self = Self::default();
    }

    fn add_section(&mut self, txt: &str) {
        self.current_section = txt.to_string();
        self.sections.push(txt.to_string());
    }

    fn add_task(&mut self, descr: &str, task_data: &str) -> Result<()> {
        let rest = task_data.strip_prefix(':').unwrap_or(task_data);
        let pieces: Vec<&str> = rest.split(':').collect();

        let score_str = pieces.first().copied().unwrap_or("");
        // Mermaid upstream uses JS `Number(...)` for parsing task scores. This means:
        // - whitespace-only => 0
        // - invalid strings => NaN (and Mermaid happily renders an SVG containing `NaN`)
        //
        // JSON snapshots cannot represent NaN, so we model it as `score=0` + `scoreIsNaN=true`,
        // and let the SVG renderer re-emit `NaN` for the relevant face/mouth coordinates.
        let score_trim = score_str.trim();
        let (score, score_is_nan) = if score_trim.is_empty() {
            (0_i64, false)
        } else {
            match score_trim.parse::<f64>() {
                Ok(v) if v.is_finite() => (v as i64, false),
                _ => (0_i64, true),
            }
        };

        let people = if pieces.len() == 1 {
            Vec::new()
        } else {
            pieces
                .get(1)
                .copied()
                .unwrap_or("")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };

        self.tasks.push(JourneyRenderTask {
            score,
            score_is_nan,
            people,
            section: self.current_section.clone(),
            task_type: self.current_section.clone(),
            task: descr.to_string(),
        });
        Ok(())
    }

    fn actors_sorted(&self) -> Vec<String> {
        let mut set = BTreeSet::<String>::new();
        for t in &self.tasks {
            for p in &t.people {
                set.insert(p.clone());
            }
        }
        set.into_iter().collect()
    }
}

enum JourneyParseOutput {
    Empty,
    Model(JourneyDiagramRenderModel),
}

fn starts_with_ci(s: &str, prefix: &str) -> bool {
    s.get(..prefix.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
}

fn split_hash_or_semi(s: &str) -> &str {
    let mut end = s.len();
    for (i, c) in s.char_indices() {
        if c == '#' || c == ';' {
            end = i;
            break;
        }
    }
    &s[..end]
}

fn parse_keyword_arg_one_ws(line: &str, keyword: &str) -> Option<String> {
    let t = line.trim_start();
    if !starts_with_ci(t, keyword) {
        return None;
    }
    let after = &t[keyword.len()..];
    let ws = after.chars().next()?;
    if !ws.is_whitespace() {
        return None;
    }
    let rest = &after[ws.len_utf8()..];
    Some(split_hash_or_semi(rest).to_string())
}

fn parse_key_colon_value(line: &str, key: &str) -> Option<String> {
    let t = line.trim_start();
    if !starts_with_ci(t, key) {
        return None;
    }
    let rest = t[key.len()..].trim_start();
    let rest = rest.strip_prefix(':')?;
    Some(split_hash_or_semi(rest).trim().to_string())
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

fn strip_comment_prefix(line: &str) -> &str {
    let t = line.trim_start();
    if t.starts_with('#') {
        return "";
    }
    if t.starts_with("%%") && !t.starts_with("%%{") {
        return "";
    }
    split_hash_or_semi(line)
}

pub fn parse_journey(code: &str, meta: &ParseMetadata) -> Result<Value> {
    match parse_journey_model(code, meta)? {
        JourneyParseOutput::Empty => Ok(json!({})),
        JourneyParseOutput::Model(model) => Ok(json!({
            "type": meta.diagram_type,
            "title": model.title,
            "accTitle": model.acc_title,
            "accDescr": model.acc_descr,
            "sections": model.sections,
            "tasks": model.tasks,
            "actors": model.actors,
        })),
    }
}

pub fn parse_journey_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<JourneyDiagramRenderModel> {
    match parse_journey_model(code, meta)? {
        JourneyParseOutput::Empty => Ok(JourneyDiagramRenderModel::default()),
        JourneyParseOutput::Model(model) => Ok(model),
    }
}

fn parse_journey_model(code: &str, meta: &ParseMetadata) -> Result<JourneyParseOutput> {
    let mut db = JourneyDb::default();
    db.clear();

    let mut lines = code.lines();
    let mut header_seen = false;

    while let Some(line) = lines.next() {
        let stripped = strip_comment_prefix(line);
        let t = stripped.trim();
        if t.is_empty() {
            continue;
        }

        if !header_seen {
            if starts_with_ci(t, "journey") {
                header_seen = true;
                let rest = t["journey".len()..].trim_start();
                if !rest.is_empty() {
                    return Err(Error::DiagramParse {
                        diagram_type: meta.diagram_type.clone(),
                        message: "unexpected content after journey header".to_string(),
                    });
                }
                continue;
            }
            return Err(Error::DiagramParse {
                diagram_type: meta.diagram_type.clone(),
                message: "expected journey header".to_string(),
            });
        }

        if let Some(v) = parse_keyword_arg_one_ws(stripped, "title") {
            db.title = v;
            continue;
        }
        if let Some(v) = parse_key_colon_value(stripped, "accTitle") {
            db.acc_title = v;
            continue;
        }
        if let Some(v) = parse_key_colon_value(stripped, "accDescr") {
            db.acc_descr = v;
            continue;
        }
        if let Some(v) = parse_acc_descr_block(&mut lines, stripped) {
            db.acc_descr = v;
            continue;
        }
        if let Some(v) = parse_keyword_arg_one_ws(stripped, "section") {
            let v = v.split(':').next().unwrap_or("").to_string();
            db.add_section(&v);
            continue;
        }

        let Some(colon) = stripped.find(':') else {
            return Err(Error::DiagramParse {
                diagram_type: meta.diagram_type.clone(),
                message: format!("unrecognized statement: {t}"),
            });
        };
        let task_name = stripped[..colon].to_string();
        let task_data = stripped[colon..].to_string();
        if task_name.trim().is_empty() || task_data.trim().is_empty() {
            continue;
        }
        db.add_task(&task_name, &task_data)?;
    }

    if !header_seen {
        return Ok(JourneyParseOutput::Empty);
    }

    let actors = db.actors_sorted();

    Ok(JourneyParseOutput::Model(JourneyDiagramRenderModel {
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
        actors,
    }))
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
    fn journey_title_definition_parses() {
        let model = parse("journey\ntitle Adding journey diagram functionality to mermaid");
        assert_eq!(
            model["title"],
            json!("Adding journey diagram functionality to mermaid")
        );
    }

    #[test]
    fn journey_parses_acc_descr_block_and_title_and_acc_title() {
        let model = parse(
            "journey\n\
accDescr {\n\
  A user journey for\n\
  family shopping\n\
}\n\
title Adding journey diagram functionality to mermaid\n\
accTitle: Adding acc journey diagram functionality to mermaid\n\
section Order from website\n",
        );
        assert_eq!(
            model["accDescr"],
            json!("A user journey for\nfamily shopping")
        );
        assert_eq!(
            model["title"],
            json!("Adding journey diagram functionality to mermaid")
        );
        assert_eq!(
            model["accTitle"],
            json!("Adding acc journey diagram functionality to mermaid")
        );
    }

    #[test]
    fn journey_parses_acc_title_without_description() {
        let model = parse(
            "journey\n\
accTitle: The title\n\
section Order from website\n",
        );
        assert_eq!(model["accTitle"], json!("The title"));
        assert!(model["accDescr"].is_null());
    }

    #[test]
    fn journey_parses_acc_descr_single_line() {
        let model = parse(
            "journey\n\
accDescr: A user journey for family shopping\n\
title Adding journey diagram functionality to mermaid\n\
section Order from website\n",
        );
        assert_eq!(
            model["accDescr"],
            json!("A user journey for family shopping")
        );
        assert_eq!(
            model["title"],
            json!("Adding journey diagram functionality to mermaid")
        );
    }

    #[test]
    fn journey_allows_section_titles_with_br_variants() {
        let model = parse(
            "journey\n\
title Adding gantt diagram functionality to mermaid\n\
section Line1<br>Line2<br/>Line3</br />Line4<br\t/>Line5\n",
        );
        let sections = model["sections"].as_array().unwrap();
        assert_eq!(sections.len(), 1);
    }

    #[test]
    fn journey_parses_tasks_and_people_like_upstream() {
        let model = parse(
            "journey\n\
title Adding journey diagram functionality to mermaid\n\
section Documentation\n\
A task: 5: Alice, Bob, Charlie\n\
B task: 3:Bob, Charlie\n\
C task: 5\n\
D task: 5: Charlie, Alice\n\
E task: 5:\n\
section Another section\n\
P task: 5:\n\
Q task: 5:\n\
R task: 5:\n",
        );

        let tasks = model["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 8);

        assert_eq!(
            tasks[0],
            json!({
                "score": 5,
                "people": ["Alice", "Bob", "Charlie"],
                "section": "Documentation",
                "task": "A task",
                "type": "Documentation",
            })
        );
        assert_eq!(
            tasks[1],
            json!({
                "score": 3,
                "people": ["Bob", "Charlie"],
                "section": "Documentation",
                "task": "B task",
                "type": "Documentation",
            })
        );
        assert_eq!(
            tasks[2],
            json!({
                "score": 5,
                "people": [],
                "section": "Documentation",
                "task": "C task",
                "type": "Documentation",
            })
        );
        assert_eq!(
            tasks[3],
            json!({
                "score": 5,
                "people": ["Charlie", "Alice"],
                "section": "Documentation",
                "task": "D task",
                "type": "Documentation",
            })
        );
        assert_eq!(
            tasks[4],
            json!({
                "score": 5,
                "people": [""],
                "section": "Documentation",
                "task": "E task",
                "type": "Documentation",
            })
        );
        assert_eq!(
            tasks[5],
            json!({
                "score": 5,
                "people": [""],
                "section": "Another section",
                "task": "P task",
                "type": "Another section",
            })
        );
        assert_eq!(
            tasks[6],
            json!({
                "score": 5,
                "people": [""],
                "section": "Another section",
                "task": "Q task",
                "type": "Another section",
            })
        );
        assert_eq!(
            tasks[7],
            json!({
                "score": 5,
                "people": [""],
                "section": "Another section",
                "task": "R task",
                "type": "Another section",
            })
        );
    }

    #[test]
    fn journey_db_tasks_and_actors_should_be_added_matches_upstream_spec() {
        let mut db = JourneyDb::default();
        db.clear();

        db.acc_title = "Shopping".to_string();
        db.acc_descr = "A user journey for family shopping".to_string();
        db.add_section("Journey to the shops");
        db.add_task("Get car keys", ":5:Dad").unwrap();
        db.add_task("Go to car", ":3:Dad, Mum, Child#1, Child#2")
            .unwrap();
        db.add_task("Drive to supermarket", ":4:Dad").unwrap();
        db.add_section("Do shopping");
        db.add_task("Go shopping", ":5:Mum").unwrap();

        let actors = db.actors_sorted();
        assert_eq!(
            db.tasks
                .iter()
                .map(|t| {
                    json!({
                        "score": t.score,
                        "people": t.people,
                        "section": t.section,
                        "task": t.task,
                        "type": t.task_type,
                    })
                })
                .collect::<Vec<_>>(),
            vec![
                json!({
                    "score": 5,
                    "people": ["Dad"],
                    "section": "Journey to the shops",
                    "task": "Get car keys",
                    "type": "Journey to the shops",
                }),
                json!({
                    "score": 3,
                    "people": ["Dad", "Mum", "Child#1", "Child#2"],
                    "section": "Journey to the shops",
                    "task": "Go to car",
                    "type": "Journey to the shops",
                }),
                json!({
                    "score": 4,
                    "people": ["Dad"],
                    "section": "Journey to the shops",
                    "task": "Drive to supermarket",
                    "type": "Journey to the shops",
                }),
                json!({
                    "score": 5,
                    "people": ["Mum"],
                    "section": "Do shopping",
                    "task": "Go shopping",
                    "type": "Do shopping",
                }),
            ]
        );

        assert_eq!(
            actors,
            vec![
                "Child#1".to_string(),
                "Child#2".to_string(),
                "Dad".to_string(),
                "Mum".to_string()
            ]
        );
        assert_eq!(
            db.sections,
            vec![
                "Journey to the shops".to_string(),
                "Do shopping".to_string()
            ]
        );
    }

    #[test]
    fn journey_db_clear_resets_state() {
        let mut db = JourneyDb::default();
        db.add_section("weekends skip test");
        db.add_task("test1", "4: id1, id3").unwrap();
        db.add_task("test2", "2: id2").unwrap();

        db.clear();

        assert!(db.title.is_empty());
        assert!(db.acc_title.is_empty());
        assert!(db.acc_descr.is_empty());
        assert!(db.sections.is_empty());
        assert!(db.tasks.is_empty());
        assert!(db.actors_sorted().is_empty());
    }
}
