use super::{FlowSubGraph, Stmt, SubgraphBlock, TitleKind, strip_wrapping_backticks, unquote};
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum StatementItem {
    Id(String),
    Dir(String),
}

struct EvalFrame<'a> {
    statements: &'a [Stmt],
    index: usize,
    subgraph: Option<&'a SubgraphBlock>,
    items: Vec<StatementItem>,
}

pub(super) struct SubgraphBuilder {
    sub_count: usize,
    pub(super) subgraphs: Vec<FlowSubGraph>,
    inherit_dir: bool,
    global_dir: Option<String>,
}

impl SubgraphBuilder {
    pub(super) fn new(inherit_dir: bool, global_dir: Option<String>) -> Self {
        Self {
            sub_count: 0,
            subgraphs: Vec::new(),
            inherit_dir,
            global_dir,
        }
    }

    pub(super) fn visit_statements(&mut self, statements: &[Stmt]) {
        let _ = self.eval_statements(statements);
    }

    fn eval_statements(&mut self, statements: &[Stmt]) -> Vec<StatementItem> {
        enum EvalStep<'a> {
            Statement(&'a Stmt),
            Finish,
        }

        let mut stack = vec![EvalFrame {
            statements,
            index: 0,
            subgraph: None,
            items: Vec::new(),
        }];
        let mut root_items = Vec::new();

        while !stack.is_empty() {
            let step = {
                let Some(frame) = stack.last_mut() else {
                    return root_items;
                };
                if frame.index >= frame.statements.len() {
                    EvalStep::Finish
                } else {
                    let stmt = &frame.statements[frame.index];
                    frame.index += 1;
                    EvalStep::Statement(stmt)
                }
            };

            match step {
                EvalStep::Statement(Stmt::Subgraph(sg)) => stack.push(EvalFrame {
                    statements: &sg.statements,
                    index: 0,
                    subgraph: Some(sg),
                    items: Vec::new(),
                }),
                EvalStep::Statement(stmt) => {
                    if let Some(frame) = stack.last_mut() {
                        push_statement_items(&mut frame.items, stmt);
                    }
                }
                EvalStep::Finish => {
                    let Some(frame) = stack.pop() else {
                        return root_items;
                    };
                    if let Some(sg) = frame.subgraph {
                        let id = self.eval_subgraph_from_items(sg, frame.items);
                        if let Some(parent) = stack.last_mut() {
                            parent.items.push(StatementItem::Id(id));
                        }
                    } else {
                        root_items = frame.items;
                    }
                }
            }
        }

        root_items
    }

    fn eval_subgraph_from_items(
        &mut self,
        sg: &SubgraphBlock,
        items: Vec<StatementItem>,
    ) -> String {
        let mut seen: HashSet<String> = HashSet::new();
        let mut members: Vec<String> = Vec::new();
        let mut dir: Option<String> = None;

        for item in items {
            match item {
                StatementItem::Dir(d) => dir = Some(d),
                StatementItem::Id(id) => {
                    if id.trim().is_empty() {
                        continue;
                    }
                    if seen.insert(id.clone()) {
                        members.push(id);
                    }
                }
            }
        }

        let dir = dir.or_else(|| {
            if self.inherit_dir {
                self.global_dir.clone()
            } else {
                None
            }
        });

        let raw_id = unquote(&sg.header.raw_id);
        let (title_raw, title_kind) =
            parse_subgraph_title(&sg.header.raw_title, sg.header.id_equals_title);
        let id_raw = strip_wrapping_backticks(raw_id.trim()).0;

        let mut id: Option<String> = {
            let trimmed = id_raw.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        };

        // Mirror Mermaid `FlowDB.addSubGraph(...)`:
        // `if (_id === _title && /\\s/.exec(_title.text)) id = undefined;`
        //
        // The important nuance is that this checks the untrimmed title token (including any
        // extra whitespace that may have been captured into the header).
        if sg.header.id_equals_title && sg.header.raw_title.chars().any(|c| c.is_whitespace()) {
            id = None;
        }

        let id = id.unwrap_or_else(|| format!("subGraph{}", self.sub_count));
        let title = title_raw.trim().to_string();
        let label_type = match title_kind {
            TitleKind::Text => "text",
            TitleKind::String => "string",
            TitleKind::Markdown => "markdown",
        }
        .to_string();

        self.sub_count += 1;

        members.retain(|m| !subgraphs_exist(&self.subgraphs, m));

        self.subgraphs.push(FlowSubGraph {
            id: id.clone(),
            nodes: members,
            title,
            classes: Vec::new(),
            styles: Vec::new(),
            dir,
            label_type,
        });

        id
    }
}

fn push_statement_items(out: &mut Vec<StatementItem>, stmt: &Stmt) {
    match stmt {
        Stmt::Chain { nodes, edges } => {
            // Mermaid FlowDB's subgraph membership list is based on the Jison
            // `vertexStatement.nodes` shape, which prepends the last node in a chain first
            // (e.g. `a-->b` yields `[b, a]`).
            //
            // For node-only group statements (e.g. `A & B`), there are no edges and the list
            // preserves the input order.
            if edges.is_empty() {
                for n in nodes {
                    out.push(StatementItem::Id(n.id.clone()));
                }
            } else {
                for n in nodes.iter().rev() {
                    out.push(StatementItem::Id(n.id.clone()));
                }
            }
        }
        Stmt::Node(n) => out.push(StatementItem::Id(n.id.clone())),
        Stmt::Direction(d) => out.push(StatementItem::Dir(d.clone())),
        Stmt::ShapeData { target, .. } => out.push(StatementItem::Id(target.clone())),
        Stmt::Subgraph(_)
        | Stmt::Style(_)
        | Stmt::ClassDef(_)
        | Stmt::ClassAssign(_)
        | Stmt::Click(_)
        | Stmt::LinkStyle(_) => {}
    }
}

pub(super) fn subgraphs_exist(subgraphs: &[FlowSubGraph], node_id: &str) -> bool {
    subgraphs
        .iter()
        .any(|sg| sg.nodes.iter().any(|n| n == node_id))
}

fn parse_subgraph_title(raw_title: &str, id_equals_title: bool) -> (String, TitleKind) {
    let trimmed = raw_title.trim();
    let quoted = (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''));
    let unquoted = if quoted {
        // Keep flowchart subgraph titles raw (strip only surrounding quotes).
        // This matches upstream and avoids mangling backslash-heavy labels.
        unquote(trimmed)
    } else {
        trimmed.to_string()
    };

    let (no_backticks, is_markdown) = strip_wrapping_backticks(unquoted.trim());
    if is_markdown {
        return (no_backticks, TitleKind::Markdown);
    }

    if !id_equals_title && quoted {
        return (unquoted, TitleKind::String);
    }

    (unquoted, TitleKind::Text)
}
