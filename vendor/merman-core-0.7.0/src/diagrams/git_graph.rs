use crate::sanitize::sanitize_text;
use crate::{Error, MermaidConfig, ParseMetadata, Result};
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use uuid::Uuid;

const COMMIT_TYPE_NORMAL: i64 = 0;
const COMMIT_TYPE_REVERSE: i64 = 1;
const COMMIT_TYPE_HIGHLIGHT: i64 = 2;
const COMMIT_TYPE_MERGE: i64 = 3;
const COMMIT_TYPE_CHERRY_PICK: i64 = 4;

#[derive(Debug, Clone)]
struct Commit {
    id: String,
    message: String,
    seq: i64,
    commit_type: i64,
    tags: Vec<String>,
    parents: Vec<String>,
    branch: String,
    custom_type: Option<i64>,
    custom_id: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitGraphBranchRenderModel {
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitGraphCommitRenderModel {
    pub id: String,
    pub message: String,
    pub seq: i64,
    #[serde(rename = "type")]
    pub commit_type: i64,
    pub tags: Vec<String>,
    pub parents: Vec<String>,
    pub branch: String,
    #[serde(rename = "customType", skip_serializing_if = "Option::is_none")]
    pub custom_type: Option<i64>,
    #[serde(rename = "customId", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GitGraphRenderModel {
    #[serde(rename = "type")]
    pub diagram_type: String,
    pub commits: Vec<GitGraphCommitRenderModel>,
    pub branches: Vec<GitGraphBranchRenderModel>,
    #[serde(rename = "currentBranch")]
    pub current_branch: String,
    pub direction: String,
    #[serde(rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(rename = "accDescr")]
    pub acc_descr: Option<String>,
    pub warnings: Vec<String>,
}

impl GitGraphRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone)]
struct BranchConfig {
    order: i64,
}

#[derive(Debug, Clone)]
struct CommitDb {
    id: String,
    msg: String,
    commit_type: i64,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct BranchDb {
    name: String,
    order: i64,
}

#[derive(Debug, Clone)]
struct MergeDb {
    branch: String,
    id: Option<String>,
    commit_type: Option<i64>,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct CherryPickDb {
    id: String,
    target_id: String,
    parent: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct GitGraphDb {
    commits: HashMap<String, Commit>,
    commit_order: Vec<String>,
    branches: HashMap<String, Option<String>>,
    branch_config: HashMap<String, BranchConfig>,
    branch_config_order: Vec<String>,
    head: Option<String>,
    curr_branch: String,
    direction: String,
    seq: i64,
    warnings: Vec<String>,
    acc_title: String,
    acc_descr: String,
    prng: Option<XorShift64Star>,
}

#[derive(Debug, Clone, Copy)]
struct XorShift64Star {
    state: u64,
}

impl XorShift64Star {
    fn new(seed: u64) -> Self {
        let mut state = seed;
        if state == 0 {
            state = 1;
        }
        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        // Mirrors the seeded upstream renderer script used by `xtask gen-upstream-svgs`:
        //   x ^= x >> 12; x ^= x << 25; x ^= x >> 27; return x * 0x2545F4914F6CDD1D (mod 2^64)
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn next_hex_digit(&mut self) -> u8 {
        // Seeded upstream uses `Math.floor(Math.random() * 16)` where `Math.random()` is derived
        // from `next_u64() >> 11` (53 bits). This is equivalent to taking the top nibble of
        // `next_u64()`.
        ((self.next_u64() >> 60) & 0xF) as u8
    }

    fn make_random_hex(&mut self, len: usize) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(len);
        for _ in 0..len {
            let idx = self.next_hex_digit() as usize;
            out.push(HEX[idx] as char);
        }
        out
    }
}

impl GitGraphDb {
    fn clear(&mut self, config: &MermaidConfig, prng_override: Option<XorShift64Star>) {
        self.commits.clear();
        self.commit_order.clear();
        self.branches.clear();
        self.branch_config.clear();
        self.branch_config_order.clear();
        self.head = None;
        self.direction = "LR".to_string();
        self.seq = 0;
        self.warnings.clear();
        self.acc_title.clear();
        self.acc_descr.clear();

        // Mermaid gitGraph auto-generates commit ids using `utils.random({ length: 7 })`, which
        // depends on `Math.random()`. For deterministic test runs (and for reproducible upstream
        // SVG baselines), we allow injecting a seed.
        //
        // When unset, we keep Mermaid's non-deterministic behavior (random per run).
        self.prng = match prng_override {
            Some(prng) => Some(prng),
            None => {
                let mut prng = seeded_gitgraph_prng(config);
                if let Some(prng) = prng.as_mut() {
                    // The seeded upstream SVG renderer consumes one `Math.random()` value before
                    // the first gitGraph auto-id is minted.
                    let _ = prng.next_u64();
                }
                prng
            }
        };

        let main = config
            .get_str("gitGraph.mainBranchName")
            .unwrap_or("main")
            .to_string();
        let main_order = config_i64(config, "gitGraph.mainBranchOrder").unwrap_or(0);
        self.curr_branch = main.clone();

        self.branches.insert(main.clone(), None);
        self.branch_config
            .insert(main.clone(), BranchConfig { order: main_order });
        self.branch_config_order.push(main);
    }

    fn set_direction(&mut self, dir: &str) {
        self.direction = dir.to_string();
    }

    fn next_id(&mut self) -> String {
        if let Some(prng) = self.prng.as_mut() {
            prng.make_random_hex(7)
        } else {
            let hex = Uuid::new_v4().simple().to_string();
            hex.chars().take(7).collect()
        }
    }

    fn commit(&mut self, mut commit_db: CommitDb, config: &MermaidConfig) {
        let id_raw = std::mem::take(&mut commit_db.id);
        let msg_raw = std::mem::take(&mut commit_db.msg);
        let tags_raw = std::mem::take(&mut commit_db.tags);

        let id = sanitize_text(&id_raw, config);
        let msg = sanitize_text(&msg_raw, config);
        let tags: Vec<String> = tags_raw
            .into_iter()
            .map(|t| sanitize_text(&t, config))
            .collect();

        let commit_id = if id.is_empty() {
            let seq = self.seq;
            format!("{seq}-{}", self.next_id())
        } else {
            id
        };

        let parents = self
            .head
            .as_ref()
            .map(|h| vec![h.clone()])
            .unwrap_or_default();

        let new_commit = Commit {
            id: commit_id.clone(),
            message: msg,
            seq: self.seq,
            commit_type: commit_db.commit_type,
            tags,
            parents,
            branch: self.curr_branch.clone(),
            custom_type: None,
            custom_id: None,
        };
        self.seq += 1;

        self.head = Some(new_commit.id.clone());
        if self.commits.contains_key(&new_commit.id) {
            self.warnings
                .push(format!("Commit ID {} already exists", new_commit.id));
        }

        let existed = self.commits.contains_key(&new_commit.id);
        self.commits.insert(new_commit.id.clone(), new_commit);
        if !existed {
            self.commit_order.push(commit_id.clone());
        }

        self.branches
            .insert(self.curr_branch.clone(), Some(commit_id));
    }

    fn branch(&mut self, mut branch_db: BranchDb, config: &MermaidConfig) -> Result<()> {
        branch_db.name = sanitize_text(&branch_db.name, config);
        if self.branches.contains_key(&branch_db.name) {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: format!(
                    "Trying to create an existing branch. (Help: Either use a new name if you want create a new branch or try using \"checkout {}\")",
                    branch_db.name
                ),
            });
        }

        let head_id = self.head.clone();
        self.branches.insert(branch_db.name.clone(), head_id);
        self.branch_config.insert(
            branch_db.name.clone(),
            BranchConfig {
                order: branch_db.order,
            },
        );
        self.branch_config_order.push(branch_db.name.clone());
        self.checkout(&branch_db.name, config)?;
        Ok(())
    }

    fn checkout(&mut self, branch: &str, config: &MermaidConfig) -> Result<()> {
        let branch = sanitize_text(branch, config);
        if !self.branches.contains_key(&branch) {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: format!(
                    "Trying to checkout branch which is not yet created. (Help try using \"branch {}\")",
                    branch
                ),
            });
        }
        self.curr_branch = branch.clone();
        let id = self.branches.get(&branch).cloned().unwrap_or_default();
        self.head = id;
        Ok(())
    }

    fn merge(&mut self, mut merge_db: MergeDb, config: &MermaidConfig) -> Result<()> {
        merge_db.branch = sanitize_text(&merge_db.branch, config);
        if let Some(custom_id) = merge_db.id.as_mut() {
            *custom_id = sanitize_text(custom_id, config);
            if custom_id.is_empty() {
                merge_db.id = None;
            }
        }

        let current_branch = self.curr_branch.clone();
        let other_branch = merge_db.branch.clone();

        if current_branch == other_branch {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: "Incorrect usage of \"merge\". Cannot merge a branch to itself"
                    .to_string(),
            });
        }

        let Some(current_head_id) = self
            .branches
            .get(&current_branch)
            .and_then(|id| id.as_ref())
        else {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: format!(
                    "Incorrect usage of \"merge\". Current branch ({})has no commits",
                    current_branch
                ),
            });
        };
        let Some(current_commit) = self.commits.get(current_head_id).cloned() else {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: format!(
                    "Incorrect usage of \"merge\". Current branch ({})has no commits",
                    current_branch
                ),
            });
        };

        if !self.branches.contains_key(&other_branch) {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: format!(
                    "Incorrect usage of \"merge\". Branch to be merged ({}) does not exist",
                    other_branch
                ),
            });
        }

        let Some(other_head_id) = self.branches.get(&other_branch).and_then(|id| id.as_ref())
        else {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: format!(
                    "Incorrect usage of \"merge\". Branch to be merged ({}) has no commits",
                    other_branch
                ),
            });
        };
        let Some(other_commit) = self.commits.get(other_head_id).cloned() else {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: format!(
                    "Incorrect usage of \"merge\". Branch to be merged ({}) has no commits",
                    other_branch
                ),
            });
        };

        if current_commit.branch == other_branch {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: format!("Cannot merge branch '{}' into itself.", other_branch),
            });
        }

        if current_commit.id == other_commit.id {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: "Incorrect usage of \"merge\". Both branches have same head".to_string(),
            });
        }

        if let Some(custom_id) = merge_db.id.as_ref() {
            if self.commits.contains_key(custom_id) {
                return Err(Error::DiagramParse {
                    diagram_type: "gitGraph".to_string(),
                    message: format!(
                        "Incorrect usage of \"merge\". Commit with id:{} already exists, use different custom id",
                        custom_id
                    ),
                });
            }
        }

        let verified_branch = other_head_id.clone();
        let merge_commit_id = match merge_db.id.clone() {
            Some(id) => id,
            None => {
                let seq = self.seq;
                format!("{seq}-{}", self.next_id())
            }
        };
        let custom_id_flag = merge_db.id.is_some();

        let tags: Vec<String> = merge_db
            .tags
            .into_iter()
            .map(|t| sanitize_text(&t, config))
            .collect();

        let new_commit = Commit {
            id: merge_commit_id.clone(),
            message: format!("merged branch {} into {}", other_branch, current_branch),
            seq: self.seq,
            commit_type: COMMIT_TYPE_MERGE,
            tags,
            parents: vec![current_commit.id, verified_branch],
            branch: current_branch.clone(),
            custom_type: merge_db.commit_type,
            custom_id: Some(custom_id_flag),
        };
        self.seq += 1;

        self.head = Some(new_commit.id.clone());
        self.commits.insert(new_commit.id.clone(), new_commit);
        self.commit_order.push(merge_commit_id.clone());
        self.branches
            .insert(current_branch.clone(), Some(merge_commit_id));
        Ok(())
    }

    fn cherry_pick(&mut self, mut cp: CherryPickDb, config: &MermaidConfig) -> Result<()> {
        cp.id = sanitize_text(&cp.id, config);
        cp.target_id = sanitize_text(&cp.target_id, config);
        cp.parent = sanitize_text(&cp.parent, config);

        if cp.id.is_empty() {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message:
                    "Incorrect usage of \"cherryPick\". Source commit id should exist and provided"
                        .to_string(),
            });
        }

        let Some(source_commit) = self.commits.get(&cp.id).cloned() else {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message:
                    "Incorrect usage of \"cherryPick\". Source commit id should exist and provided"
                        .to_string(),
            });
        };
        if !cp.parent.is_empty() && !(source_commit.parents.iter().any(|p| p == &cp.parent)) {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: "Invalid operation: The specified parent commit is not an immediate parent of the cherry-picked commit.".to_string(),
            });
        }

        if source_commit.commit_type == COMMIT_TYPE_MERGE && cp.parent.is_empty() {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: "Incorrect usage of cherry-pick: If the source commit is a merge commit, an immediate parent commit must be specified.".to_string(),
            });
        }

        if cp.target_id.is_empty() || !self.commits.contains_key(&cp.target_id) {
            if source_commit.branch == self.curr_branch {
                return Err(Error::DiagramParse {
                    diagram_type: "gitGraph".to_string(),
                    message:
                        "Incorrect usage of \"cherryPick\". Source commit is already on current branch"
                            .to_string(),
                });
            }

            let current_commit_id = self
                .branches
                .get(&self.curr_branch)
                .cloned()
                .unwrap_or(None);
            if current_commit_id.is_none() {
                return Err(Error::DiagramParse {
                    diagram_type: "gitGraph".to_string(),
                    message: format!(
                        "Incorrect usage of \"cherry-pick\". Current branch ({})has no commits",
                        self.curr_branch
                    ),
                });
            }

            let tags = match cp.tags {
                Some(mut t) => {
                    t.retain(|s| !s.is_empty());
                    t.into_iter()
                        .map(|v| sanitize_text(&v, config))
                        .collect::<Vec<_>>()
                }
                None => {
                    let mut tag = format!("cherry-pick:{}", source_commit.id);
                    if source_commit.commit_type == COMMIT_TYPE_MERGE {
                        tag.push_str(&format!("|parent:{}", cp.parent));
                    }
                    vec![tag]
                }
            };

            let seq = self.seq;
            let new_id = format!("{seq}-{}", self.next_id());
            let parents = self
                .head
                .as_ref()
                .map(|h| vec![h.clone(), source_commit.id.clone()])
                .unwrap_or_default();
            let commit = Commit {
                id: new_id.clone(),
                message: format!(
                    "cherry-picked {} into {}",
                    source_commit.message, self.curr_branch
                ),
                seq: self.seq,
                commit_type: COMMIT_TYPE_CHERRY_PICK,
                tags,
                parents,
                branch: self.curr_branch.clone(),
                custom_type: None,
                custom_id: None,
            };
            self.seq += 1;

            self.head = Some(commit.id.clone());
            self.commits.insert(commit.id.clone(), commit);
            self.commit_order.push(new_id.clone());
            self.branches.insert(self.curr_branch.clone(), Some(new_id));
        }

        Ok(())
    }

    fn commits_in_seq_order(&self) -> Vec<Commit> {
        let mut out: Vec<Commit> = self.commits.values().cloned().collect();
        out.sort_by_key(|c| c.seq);
        out
    }

    fn branches_in_order(&self) -> Vec<GitGraphBranchRenderModel> {
        let mut entries: Vec<(String, f64)> = Vec::new();
        for (i, name) in self.branch_config_order.iter().enumerate() {
            let cfg = self.branch_config.get(name);
            let order = cfg.map(|c| c.order);
            let order_f = match order {
                Some(v) => v as f64,
                None => format!("0.{i}").parse::<f64>().unwrap_or(0.0),
            };
            entries.push((name.clone(), order_f));
        }

        entries.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        entries
            .into_iter()
            .map(|(name, _)| GitGraphBranchRenderModel { name })
            .collect()
    }
}

fn config_i64(config: &MermaidConfig, dotted_path: &str) -> Option<i64> {
    let mut cur = config.as_value();
    for seg in dotted_path.split('.') {
        cur = cur.as_object()?.get(seg)?;
    }
    cur.as_i64()
}

fn seeded_gitgraph_prng(config: &MermaidConfig) -> Option<XorShift64Star> {
    config_i64(config, "gitGraph.seed")
        .and_then(|v| u64::try_from(v).ok())
        .filter(|v| *v != 0)
        .map(XorShift64Star::new)
}

fn commit_to_render_model(c: Commit) -> GitGraphCommitRenderModel {
    GitGraphCommitRenderModel {
        id: c.id,
        message: c.message,
        seq: c.seq,
        commit_type: c.commit_type,
        tags: c.tags,
        parents: c.parents,
        branch: c.branch,
        custom_type: c.custom_type,
        custom_id: c.custom_id,
    }
}

fn parse_commit_type(raw: &str) -> Result<i64> {
    match raw.trim() {
        "NORMAL" => Ok(COMMIT_TYPE_NORMAL),
        "REVERSE" => Ok(COMMIT_TYPE_REVERSE),
        "HIGHLIGHT" => Ok(COMMIT_TYPE_HIGHLIGHT),
        other => Err(Error::DiagramParse {
            diagram_type: "gitGraph".to_string(),
            message: format!("Unknown commit type: {other}"),
        }),
    }
}

struct LineParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> LineParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn skip_ws(&mut self) {
        while self.peek_char().is_some_and(|c| c.is_whitespace()) {
            self.bump();
        }
    }

    fn parse_word_until_ws_or_colon(&mut self) -> Option<String> {
        self.skip_ws();
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || c == ':' {
                break;
            }
            self.bump();
        }
        if self.pos == start {
            return None;
        }
        Some(self.input[start..self.pos].to_string())
    }

    fn consume_char(&mut self, ch: char) -> bool {
        self.skip_ws();
        if self.peek_char() == Some(ch) {
            self.bump();
            return true;
        }
        false
    }

    fn parse_quoted(&mut self) -> Result<String> {
        self.skip_ws();
        if self.peek_char() != Some('"') {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: "expected quoted string".to_string(),
            });
        }
        self.bump();
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c == '"' {
                break;
            }
            self.bump();
        }
        if self.peek_char() != Some('"') {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: "unterminated quoted string".to_string(),
            });
        }
        let s = self.input[start..self.pos].to_string();
        self.bump();
        Ok(s)
    }

    fn parse_name_token(&mut self) -> Result<String> {
        self.skip_ws();
        if self.peek_char() == Some('"') {
            return self.parse_quoted();
        }
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                break;
            }
            self.bump();
        }
        if self.pos == start {
            return Err(Error::DiagramParse {
                diagram_type: "gitGraph".to_string(),
                message: "expected name".to_string(),
            });
        }
        Ok(self.input[start..self.pos].to_string())
    }

    fn parse_kv_pairs(&mut self) -> Result<Vec<(String, String)>> {
        let mut out = Vec::new();
        while !self.is_eof() {
            self.skip_ws();
            if self.is_eof() {
                break;
            }
            let Some(key) = self.parse_word_until_ws_or_colon() else {
                break;
            };
            self.skip_ws();
            if !self.consume_char(':') {
                return Err(Error::DiagramParse {
                    diagram_type: "gitGraph".to_string(),
                    message: format!("expected ':' after {key}"),
                });
            }
            self.skip_ws();
            let value = if self.peek_char() == Some('"') {
                self.parse_quoted()?
            } else {
                self.parse_word_until_ws_or_colon().unwrap_or_default()
            };
            out.push((key, value));
        }
        Ok(out)
    }
}

fn parse_header_line(line: &str) -> Result<Option<String>> {
    let t = line.trim_start();
    if !t.starts_with("gitGraph") {
        return Err(Error::DiagramParse {
            diagram_type: "gitGraph".to_string(),
            message: "expected gitGraph header".to_string(),
        });
    }
    let rest = t["gitGraph".len()..].trim();
    if rest.is_empty() || rest == ":" {
        return Ok(None);
    }
    let rest = rest.trim_end_matches(':').trim();
    if rest.is_empty() {
        return Ok(None);
    }
    let dir = rest.split_whitespace().next().unwrap_or("");
    match dir {
        "LR" | "TB" | "BT" => Ok(Some(dir.to_string())),
        _ => Ok(None),
    }
}

fn parse_acc_title(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("accTitle") {
        return None;
    }
    let rest = &t["accTitle".len()..];
    let rest = rest.trim_start();
    if !rest.starts_with(':') {
        return None;
    }
    Some(rest[1..].trim().to_string())
}

fn parse_acc_descr_inline(line: &str) -> Option<String> {
    let t = line.trim_start();
    if !t.starts_with("accDescr") {
        return None;
    }
    let rest = &t["accDescr".len()..];
    let rest = rest.trim_start();
    if !rest.starts_with(':') {
        return None;
    }
    Some(rest[1..].trim().to_string())
}

fn parse_acc_descr_block_start(line: &str) -> bool {
    let t = line.trim_start();
    if !t.starts_with("accDescr") {
        return false;
    }
    let rest = t["accDescr".len()..].trim_start();
    rest.starts_with('{')
}

pub fn parse_git_graph(code: &str, meta: &ParseMetadata) -> Result<Value> {
    let model = parse_git_graph_model(code, meta)?;
    let mut out = Map::with_capacity(9);
    out.insert("type".to_string(), Value::String(model.diagram_type));
    out.insert("commits".to_string(), json!(model.commits));
    out.insert("branches".to_string(), json!(model.branches));
    out.insert("currentBranch".to_string(), json!(model.current_branch));
    out.insert("direction".to_string(), json!(model.direction));
    out.insert("accTitle".to_string(), json!(model.acc_title));
    out.insert("accDescr".to_string(), json!(model.acc_descr));
    out.insert("warnings".to_string(), json!(model.warnings));
    out.insert(
        "config".to_string(),
        crate::config::clone_value_nonrecursive(meta.effective_config.as_value()),
    );
    Ok(Value::Object(out))
}

pub fn parse_git_graph_model_for_render(
    code: &str,
    meta: &ParseMetadata,
) -> Result<GitGraphRenderModel> {
    parse_git_graph_model(code, meta)
}

fn parse_git_graph_model(code: &str, meta: &ParseMetadata) -> Result<GitGraphRenderModel> {
    let mut lines = code.lines();
    let Some(header) = lines.next() else {
        return Err(Error::DiagramParse {
            diagram_type: "gitGraph".to_string(),
            message: "empty input".to_string(),
        });
    };

    let direction = parse_header_line(header)?;

    let effective_config = &meta.effective_config;
    let prng_override = if seeded_gitgraph_prng(effective_config).is_some() {
        // Upstream committed SVG fixtures are generated after a successful `mermaid.parse(code)`
        // followed by `mermaid.render(...)`. Seeded gitGraph auto ids consume the global
        // `Math.random()` stream during that warm-up parse, so mirror that state before building
        // the render model used for SVG parity.
        let mut warmup = new_gitgraph_db();
        warmup.clear(effective_config, None);
        if let Some(d) = direction.as_deref() {
            warmup.set_direction(d);
        }
        parse_git_graph_body(lines.clone(), &mut warmup, effective_config)?;
        warmup.prng
    } else {
        None
    };

    let mut db = new_gitgraph_db();
    db.clear(effective_config, prng_override);
    if let Some(d) = direction {
        db.set_direction(&d);
    }
    parse_git_graph_body(lines, &mut db, effective_config)?;

    let commits = db
        .commits_in_seq_order()
        .into_iter()
        .map(commit_to_render_model)
        .collect::<Vec<_>>();

    Ok(GitGraphRenderModel {
        diagram_type: meta.diagram_type.clone(),
        commits,
        branches: db.branches_in_order(),
        current_branch: db.curr_branch,
        direction: db.direction,
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
        warnings: db.warnings,
    })
}

fn new_gitgraph_db() -> GitGraphDb {
    GitGraphDb {
        commits: HashMap::new(),
        commit_order: Vec::new(),
        branches: HashMap::new(),
        branch_config: HashMap::new(),
        branch_config_order: Vec::new(),
        head: None,
        curr_branch: "main".to_string(),
        direction: "LR".to_string(),
        seq: 0,
        warnings: Vec::new(),
        acc_title: String::new(),
        acc_descr: String::new(),
        prng: None,
    }
}

fn parse_git_graph_body<'a, I>(
    lines: I,
    db: &mut GitGraphDb,
    effective_config: &MermaidConfig,
) -> Result<()>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut pending_acc_descr_block = false;
    let mut acc_descr_lines: Vec<String> = Vec::new();

    for raw in lines {
        let line = raw.trim_end_matches('\r');
        let trimmed = line.trim();
        if pending_acc_descr_block {
            if trimmed.starts_with('}') {
                pending_acc_descr_block = false;
                db.acc_descr = acc_descr_lines.join("\n");
                acc_descr_lines.clear();
                continue;
            }
            let t = trimmed.trim();
            if !t.is_empty() {
                acc_descr_lines.push(t.to_string());
            }
            continue;
        }

        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("%%") {
            continue;
        }

        if let Some(v) = parse_acc_title(trimmed) {
            db.acc_title = v;
            continue;
        }
        if parse_acc_descr_block_start(trimmed) {
            pending_acc_descr_block = true;
            continue;
        }
        if let Some(v) = parse_acc_descr_inline(trimmed) {
            db.acc_descr = v;
            continue;
        }

        let mut lp = LineParser::new(trimmed);
        let Some(cmd) = lp.parse_word_until_ws_or_colon() else {
            continue;
        };

        match cmd.as_str() {
            "commit" => {
                lp.skip_ws();
                let mut commit_db = CommitDb {
                    id: String::new(),
                    msg: String::new(),
                    commit_type: COMMIT_TYPE_NORMAL,
                    tags: Vec::new(),
                };
                if lp.peek_char() == Some('"') {
                    commit_db.msg = lp.parse_quoted()?;
                } else {
                    let kv = lp.parse_kv_pairs()?;
                    for (k, v) in kv {
                        match k.as_str() {
                            "id" => commit_db.id = v,
                            "msg" => commit_db.msg = v,
                            "tag" => commit_db.tags.push(v),
                            "type" => commit_db.commit_type = parse_commit_type(&v)?,
                            other => {
                                return Err(Error::DiagramParse {
                                    diagram_type: "gitGraph".to_string(),
                                    message: format!("unexpected commit field: {other}"),
                                });
                            }
                        }
                    }
                }
                db.commit(commit_db, effective_config);
            }
            "branch" => {
                let name = lp.parse_name_token()?;
                let kv = lp.parse_kv_pairs()?;
                let mut order = 0i64;
                for (k, v) in kv {
                    if k == "order" {
                        order = v.trim().parse::<i64>().map_err(|e| Error::DiagramParse {
                            diagram_type: "gitGraph".to_string(),
                            message: e.to_string(),
                        })?;
                    }
                }
                db.branch(BranchDb { name, order }, effective_config)?;
            }
            "checkout" | "switch" => {
                let name = lp.parse_name_token()?;
                db.checkout(&name, effective_config)?;
            }
            "merge" => {
                let branch = lp.parse_name_token()?;
                let kv = lp.parse_kv_pairs()?;
                let mut merge_db = MergeDb {
                    branch,
                    id: None,
                    commit_type: None,
                    tags: Vec::new(),
                };
                for (k, v) in kv {
                    match k.as_str() {
                        "id" => merge_db.id = Some(v),
                        "tag" => merge_db.tags.push(v),
                        "type" => merge_db.commit_type = Some(parse_commit_type(&v)?),
                        _ => {}
                    }
                }
                db.merge(merge_db, effective_config)?;
            }
            "cherry-pick" | "cherryPick" => {
                let kv = lp.parse_kv_pairs()?;
                let mut id = String::new();
                let mut parent = String::new();
                let mut tags: Option<Vec<String>> = None;
                for (k, v) in kv {
                    match k.as_str() {
                        "id" => id = v,
                        "parent" => parent = v,
                        "tag" => tags.get_or_insert_with(Vec::new).push(v),
                        _ => {}
                    }
                }
                db.cherry_pick(
                    CherryPickDb {
                        id,
                        target_id: String::new(),
                        parent,
                        tags,
                    },
                    effective_config,
                )?;
            }
            _ => {
                return Err(Error::DiagramParse {
                    diagram_type: "gitGraph".to_string(),
                    message: format!("Unknown statement: {cmd}"),
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Engine, ParseOptions, RenderSemanticModel};
    use futures::executor::block_on;

    fn parse(text: &str) -> Value {
        let engine = Engine::new();
        block_on(engine.parse_diagram(text, ParseOptions::default()))
            .unwrap()
            .unwrap()
            .model
    }

    fn parse_err(text: &str) -> String {
        let engine = Engine::new();
        match block_on(engine.parse_diagram(text, ParseOptions::default())).unwrap_err() {
            Error::DiagramParse { message, .. } => message,
            other => other.to_string(),
        }
    }

    fn parse_with_seed(text: &str, seed: i64) -> Value {
        let engine = Engine::new().with_site_config(MermaidConfig::from_value(
            json!({ "gitGraph": { "seed": seed } }),
        ));
        block_on(engine.parse_diagram(text, ParseOptions::default()))
            .unwrap()
            .unwrap()
            .model
    }

    fn commit_ids(model: &Value) -> Vec<String> {
        model["commits"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .filter_map(|c| c["id"].as_str().map(|s| s.to_string()))
            .collect()
    }

    #[test]
    fn should_handle_gitgraph_definition_and_defaults() {
        let model = parse("gitGraph:\n commit\n");
        assert_eq!(model["commits"].as_array().unwrap().len(), 1);
        assert_eq!(model["currentBranch"].as_str().unwrap(), "main");
        assert_eq!(model["direction"].as_str().unwrap(), "LR");
        assert_eq!(model["branches"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn parse_gitgraph_render_model_uses_typed_variant_without_changing_json_parse() {
        let engine = Engine::new().with_site_config(MermaidConfig::from_value(json!({
            "gitGraph": { "seed": 1 }
        })));
        let input = r#"
gitGraph TB:
accTitle: Git accTitle
accDescr: Git accDescription
commit id:"C0"
branch feature
checkout feature
commit id:"F1" tag:"v1"
checkout main
merge feature id:"M1"
"#;

        let parsed = engine
            .parse_diagram_for_render_model_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();

        assert_eq!(parsed.meta.diagram_type, "gitGraph");
        match parsed.model {
            RenderSemanticModel::GitGraph(model) => {
                assert_eq!(model.diagram_type, "gitGraph");
                assert_eq!(model.direction, "TB");
                assert_eq!(model.current_branch, "main");
                assert_eq!(model.acc_title.as_deref(), Some("Git accTitle"));
                assert_eq!(model.acc_descr.as_deref(), Some("Git accDescription"));
                assert_eq!(model.branches.len(), 2);
                assert_eq!(model.branches[0].name, "main");
                assert_eq!(model.commits.len(), 3);
                assert_eq!(model.commits[1].id, "F1");
                assert_eq!(model.commits[1].tags, vec!["v1".to_string()]);
                assert_eq!(model.commits[2].commit_type, COMMIT_TYPE_MERGE);
            }
            other => panic!("gitGraph render parse should return typed model, got {other:?}"),
        }

        let parsed_json = engine
            .parse_diagram_sync(input, ParseOptions::strict())
            .unwrap()
            .unwrap();
        assert_eq!(parsed_json.model["type"], json!("gitGraph"));
        assert_eq!(parsed_json.model["direction"], json!("TB"));
        assert_eq!(parsed_json.model["currentBranch"], json!("main"));
        assert_eq!(parsed_json.model["accTitle"], json!("Git accTitle"));
        assert_eq!(parsed_json.model["branches"][0]["name"], json!("main"));
        assert_eq!(parsed_json.model["commits"][1]["id"], json!("F1"));
        assert_eq!(parsed_json.model["commits"][1]["tags"], json!(["v1"]));
        assert!(parsed_json.model.get("config").is_some());
    }

    #[test]
    fn seeded_auto_commit_ids_match_upstream_seeded_svg_pipeline() {
        let model = parse_with_seed("gitGraph:\ncommit\n", 1);
        let ids = commit_ids(&model);
        assert_eq!(ids, vec!["0-5b722bd".to_string()]);
    }

    #[test]
    fn seeded_auto_commit_ids_are_direction_invariant() {
        let base = commit_ids(&parse_with_seed("gitGraph:\ncommit\n", 1));
        let tb = commit_ids(&parse_with_seed("gitGraph TB:\ncommit\n", 1));
        let bt = commit_ids(&parse_with_seed("gitGraph BT:\ncommit\n", 1));
        assert_eq!(base, tb);
        assert_eq!(base, bt);
        assert_eq!(base, vec!["0-5b722bd".to_string()]);
    }

    #[test]
    fn should_handle_set_direction_tb_and_bt() {
        let model = parse("gitGraph TB:\ncommit\n");
        assert_eq!(model["direction"].as_str().unwrap(), "TB");
        let model = parse("gitGraph BT:\ncommit\n");
        assert_eq!(model["direction"].as_str().unwrap(), "BT");
    }

    #[test]
    fn should_checkout_and_switch_branch() {
        let model = parse("gitGraph:\nbranch new\ncheckout new\n");
        assert_eq!(model["commits"].as_array().unwrap().len(), 0);
        assert_eq!(model["currentBranch"].as_str().unwrap(), "new");

        let model = parse("gitGraph:\nbranch new\nswitch new\n");
        assert_eq!(model["commits"].as_array().unwrap().len(), 0);
        assert_eq!(model["currentBranch"].as_str().unwrap(), "new");
    }

    #[test]
    fn should_add_commits_to_checked_out_branch() {
        let model = parse("gitGraph:\nbranch new\ncheckout new\ncommit\ncommit\n");
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits.len(), 2);
        assert_eq!(model["currentBranch"].as_str().unwrap(), "new");
        assert_eq!(commits[0]["branch"].as_str().unwrap(), "new");
        assert_eq!(commits[1]["branch"].as_str().unwrap(), "new");
        assert_eq!(commits[1]["parents"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn should_handle_commit_with_args_and_message_variants() {
        let model = parse("gitGraph:\ncommit \"a commit\"\n");
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0]["message"].as_str().unwrap(), "a commit");

        let model = parse("gitGraph:\ncommit msg: \"test commit\"\n");
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits[0]["message"].as_str().unwrap(), "test commit");

        let model = parse("gitGraph:\ncommit id:\"1111\"\n");
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits[0]["id"].as_str().unwrap(), "1111");

        let model = parse("gitGraph:\ncommit tag:\"test\"\n");
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(
            commits[0]["tags"].as_array().unwrap()[0].as_str().unwrap(),
            "test"
        );

        let model = parse("gitGraph:\ncommit tag:\"a\" tag:\"b\"\n");
        let commits = model["commits"].as_array().unwrap();
        let tags = commits[0]["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].as_str().unwrap(), "a");
        assert_eq!(tags[1].as_str().unwrap(), "b");

        let model = parse("gitGraph:\ncommit type: HIGHLIGHT\n");
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits[0]["type"].as_i64().unwrap(), 2);

        let model = parse("gitGraph:\ncommit id:\"1111\" tag: \"test tag\"\n");
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits[0]["id"].as_str().unwrap(), "1111");
        assert_eq!(
            commits[0]["tags"].as_array().unwrap()[0].as_str().unwrap(),
            "test tag"
        );

        let model = parse(
            "gitGraph:\ncommit id:\"1111\" type:REVERSE tag: \"test tag\" msg:\"test msg\"\n",
        );
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits[0]["id"].as_str().unwrap(), "1111");
        assert_eq!(commits[0]["type"].as_i64().unwrap(), 1);
        assert_eq!(commits[0]["message"].as_str().unwrap(), "test msg");
        assert_eq!(
            commits[0]["tags"].as_array().unwrap()[0].as_str().unwrap(),
            "test tag"
        );
    }

    #[test]
    fn commit_errors_on_unknown_fields() {
        let err =
            parse_err("gitGraph\ncommit id:\"2\" msg:\"Malformed commit\" oops:\"ignored\"\n");
        assert_eq!(err, "unexpected commit field: oops");
    }

    #[test]
    fn should_handle_three_straight_commits() {
        let model = parse("gitGraph:\ncommit\ncommit\ncommit\n");
        assert_eq!(model["commits"].as_array().unwrap().len(), 3);
        assert_eq!(model["branches"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn should_handle_new_branch_creation_and_names() {
        let model = parse("gitGraph:\ncommit\nbranch testBranch\n");
        assert_eq!(model["commits"].as_array().unwrap().len(), 1);
        assert_eq!(model["currentBranch"].as_str().unwrap(), "testBranch");
        assert_eq!(model["branches"].as_array().unwrap().len(), 2);

        let model = parse("gitGraph:\ncommit\nbranch azAZ_-./test\n");
        assert_eq!(model["currentBranch"].as_str().unwrap(), "azAZ_-./test");
        assert_eq!(model["branches"].as_array().unwrap().len(), 2);

        let model = parse("gitGraph:\ncommit\nbranch 1.0.1\n");
        assert_eq!(model["currentBranch"].as_str().unwrap(), "1.0.1");
        assert_eq!(model["branches"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn should_allow_quoted_branch_names_and_merge() {
        let model = parse(
            "gitGraph:\ncommit\nbranch \"branch\"\ncheckout \"branch\"\ncommit\ncheckout main\nmerge \"branch\"\n",
        );
        assert_eq!(model["commits"].as_array().unwrap().len(), 3);
        assert_eq!(model["currentBranch"].as_str().unwrap(), "main");
        assert_eq!(model["branches"].as_array().unwrap().len(), 2);
        assert_eq!(
            model["branches"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|b| b["name"].as_str())
                .collect::<Vec<_>>(),
            vec!["main", "branch"]
        );
    }

    #[test]
    fn should_handle_branch_order_sorting() {
        let model = parse(
            "gitGraph:\ncommit\nbranch test1 order: 3\nbranch test2 order: 2\nbranch test3 order: 1\n",
        );
        assert_eq!(
            model["branches"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|b| b["name"].as_str())
                .collect::<Vec<_>>(),
            vec!["main", "test3", "test2", "test1"]
        );

        let model = parse("gitGraph:\ncommit\nbranch test1 order: 1\nbranch test2\nbranch test3\n");
        assert_eq!(
            model["branches"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|b| b["name"].as_str())
                .collect::<Vec<_>>(),
            vec!["main", "test2", "test3", "test1"]
        );
    }

    #[test]
    fn should_handle_merge_with_two_parents() {
        let model = parse(
            "gitGraph:\ncommit\nbranch testBranch\ncheckout testBranch\ncommit\ncheckout main\ncommit\nmerge testBranch\n",
        );
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits.len(), 4);
        let merge = &commits[3];
        assert_eq!(merge["branch"].as_str().unwrap(), "main");
        assert_eq!(merge["parents"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn should_support_cherry_picking_commits() {
        let model = parse(
            "gitGraph\ncommit id: \"ZERO\"\nbranch develop\ncommit id:\"A\"\ncheckout main\ncherry-pick id:\"A\"\n",
        );
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(commits.len(), 3);
        assert_eq!(commits[2]["branch"].as_str().unwrap(), "main");
        assert_eq!(
            commits[2]["tags"].as_array().unwrap()[0].as_str().unwrap(),
            "cherry-pick:A"
        );

        let model = parse(
            "gitGraph\ncommit id: \"ZERO\"\nbranch develop\ncommit id:\"A\"\ncheckout main\ncherry-pick id:\"A\" tag:\"MyTag\"\n",
        );
        let commits = model["commits"].as_array().unwrap();
        assert_eq!(
            commits[2]["tags"].as_array().unwrap()[0].as_str().unwrap(),
            "MyTag"
        );

        let model = parse(
            "gitGraph\ncommit id: \"ZERO\"\nbranch develop\ncommit id:\"A\"\ncheckout main\ncherry-pick id:\"A\" tag:\"\"\n",
        );
        let commits = model["commits"].as_array().unwrap();
        assert!(commits[2]["tags"].as_array().unwrap().is_empty());
    }

    #[test]
    fn should_support_cherry_picking_merge_commits_and_validate_parent() {
        let err = parse_err(
            "gitGraph\ncommit id: \"ZERO\"\nbranch feature\nbranch release\ncheckout feature\ncommit id: \"A\"\ncommit id: \"B\"\ncheckout main\nmerge feature id: \"M\"\ncheckout release\ncommit id: \"C\"\ncherry-pick id:\"M\"\n",
        );
        assert!(err.contains("Incorrect usage of cherry-pick: If the source commit is a merge commit, an immediate parent commit must be specified."));

        let err = parse_err(
            "gitGraph\ncommit id: \"ZERO\"\nbranch feature\nbranch release\ncheckout feature\ncommit id: \"A\"\ncommit id: \"B\"\ncheckout main\nmerge feature id: \"M\"\ncheckout release\ncommit id: \"C\"\ncherry-pick id:\"M\" parent: \"A\"\n",
        );
        assert!(err.contains("Invalid operation: The specified parent commit is not an immediate parent of the cherry-picked commit."));
    }

    #[test]
    fn should_throw_error_when_try_to_branch_existing_branch() {
        let err = parse_err("gitGraph\ncommit\nbranch testBranch\ncommit\nbranch main\n");
        assert!(err.contains("Trying to create an existing branch."));

        let err = parse_err("gitGraph\ncommit\nbranch testBranch\ncommit\nbranch testBranch\n");
        assert!(err.contains("Trying to create an existing branch."));
    }

    #[test]
    fn should_throw_error_when_try_to_checkout_unknown_branch() {
        let err = parse_err("gitGraph\ncommit\ncheckout testBranch\n");
        assert_eq!(
            err,
            "Trying to checkout branch which is not yet created. (Help try using \"branch testBranch\")"
        );
    }

    #[test]
    fn should_throw_error_when_trying_to_merge_without_commits_or_unknown_branch() {
        let err = parse_err("gitGraph\nmerge testBranch\n");
        assert_eq!(
            err,
            "Incorrect usage of \"merge\". Current branch (main)has no commits"
        );

        let err = parse_err("gitGraph\ncommit\nmerge testBranch\n");
        assert_eq!(
            err,
            "Incorrect usage of \"merge\". Branch to be merged (testBranch) does not exist"
        );

        let err = parse_err("gitGraph\nbranch test1\ncheckout main\ncommit\nmerge test1\n");
        assert_eq!(
            err,
            "Incorrect usage of \"merge\". Branch to be merged (test1) has no commits"
        );
    }

    #[test]
    fn should_throw_error_when_trying_to_merge_branch_to_itself() {
        let err = parse_err("gitGraph\ncommit\nbranch testBranch\nmerge testBranch\n");
        assert_eq!(
            err,
            "Incorrect usage of \"merge\". Cannot merge a branch to itself"
        );
    }

    #[test]
    fn should_throw_error_when_using_existing_id_as_merge_id() {
        let err = parse_err(
            "gitGraph\ncommit id: \"1-111\"\nbranch testBranch\ncommit id: \"2-222\"\ncheckout main\nmerge testBranch id: \"1-111\"\n",
        );
        assert!(err.contains("Incorrect usage of \"merge\". Commit with id:1-111 already exists, use different custom id"));
    }

    #[test]
    fn should_throw_error_when_trying_to_merge_branches_having_same_heads() {
        let err =
            parse_err("gitGraph\ncommit\nbranch testBranch\ncheckout main\nmerge testBranch\n");
        assert_eq!(
            err,
            "Incorrect usage of \"merge\". Both branches have same head"
        );
    }

    #[test]
    fn should_handle_accessibility_title_and_description() {
        let model = parse(
            "gitGraph:\naccTitle: This is a title\naccDescr: This is a description\ncommit\n",
        );
        assert_eq!(model["accTitle"].as_str().unwrap(), "This is a title");
        assert_eq!(model["accDescr"].as_str().unwrap(), "This is a description");

        let model = parse(
            "gitGraph:\naccTitle: This is a title\naccDescr {\n  This is a description\n  using multiple lines\n}\ncommit\n",
        );
        assert_eq!(model["accTitle"].as_str().unwrap(), "This is a title");
        assert_eq!(
            model["accDescr"].as_str().unwrap(),
            "This is a description\nusing multiple lines"
        );
    }

    #[test]
    fn should_work_with_unsafe_properties_as_ids_and_branch_names() {
        for prop in ["__proto__", "constructor"] {
            let model = parse(&format!(
                "gitGraph\ncommit id:\"{prop}\"\nbranch {prop}\ncheckout {prop}\ncommit\ncheckout main\nmerge {prop}\n"
            ));
            assert_eq!(model["commits"].as_array().unwrap().len(), 3);
            assert_eq!(commit_ids(&model)[0], prop);
            assert_eq!(model["currentBranch"].as_str().unwrap(), "main");
            assert_eq!(model["branches"].as_array().unwrap().len(), 2);
        }
    }

    #[test]
    fn should_log_warning_when_two_commits_have_same_id() {
        let model = parse("gitGraph\ncommit id:\"working on MDR\"\ncommit id:\"working on MDR\"\n");
        let warnings = model["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>();
        assert!(warnings.contains(&"Commit ID working on MDR already exists"));
    }
}
