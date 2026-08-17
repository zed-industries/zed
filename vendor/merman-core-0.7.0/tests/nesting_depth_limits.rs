use futures::executor::block_on;
use merman_core::{Engine, MAX_DIAGRAM_NESTING_DEPTH, ParseOptions};

fn parse_ok(input: &str) {
    let engine = Engine::new();
    block_on(engine.parse_diagram(input, ParseOptions::strict()))
        .expect("deeply nested diagram should parse like Mermaid.js");
}

fn flowchart(depth: usize) -> String {
    let mut lines = vec!["flowchart TD".to_string()];
    for i in 0..depth {
        lines.push(format!("subgraph n{i}"));
    }
    lines.push("leaf[leaf]".to_string());
    for _ in 0..depth {
        lines.push("end".to_string());
    }
    lines.join("\n")
}

fn state(depth: usize) -> String {
    let mut lines = vec!["stateDiagram-v2".to_string()];
    for i in 0..depth {
        lines.push(format!("state s{i} {{"));
    }
    lines.push("leaf".to_string());
    for _ in 0..depth {
        lines.push("}".to_string());
    }
    lines.join("\n")
}

fn block(depth: usize) -> String {
    let mut lines = vec!["block-beta".to_string()];
    for i in 0..depth {
        lines.push(format!("{}block:b{i}", "  ".repeat(i)));
    }
    lines.push(format!("{}leaf", "  ".repeat(depth)));
    for i in (0..depth).rev() {
        lines.push(format!("{}end", "  ".repeat(i)));
    }
    lines.join("\n")
}

fn mindmap(depth: usize) -> String {
    let mut lines = vec!["mindmap".to_string(), "root".to_string()];
    for i in 0..depth {
        lines.push(format!("{}n{i}", "  ".repeat(i + 1)));
    }
    lines.join("\n")
}

fn treemap(depth: usize) -> String {
    let mut lines = vec!["treemap-beta".to_string()];
    for i in 0..depth {
        lines.push(format!("{}\"n{i}\"", "  ".repeat(i)));
    }
    lines.push(format!("{}\"leaf\": 1", "  ".repeat(depth)));
    lines.join("\n")
}

fn c4(depth: usize) -> String {
    let mut lines = vec!["C4Context".to_string()];
    for i in 0..depth {
        lines.push(format!("{}Boundary(b{i}, \"B{i}\") {{", "  ".repeat(i)));
    }
    lines.push(format!("{}System(s, \"S\")", "  ".repeat(depth)));
    for i in (0..depth).rev() {
        lines.push(format!("{}}}", "  ".repeat(i)));
    }
    lines.join("\n")
}

fn class_diagram(depth: usize) -> String {
    let mut lines = vec!["classDiagram".to_string()];
    for i in 0..depth {
        lines.push(format!("{}namespace N{i} {{", "  ".repeat(i)));
    }
    lines.push(format!("{}class Leaf", "  ".repeat(depth)));
    for i in (0..depth).rev() {
        lines.push(format!("{}}}", "  ".repeat(i)));
    }
    lines.join("\n")
}

#[test]
fn deeply_nested_flowchart_parses_without_custom_depth_error() {
    let depth = MAX_DIAGRAM_NESTING_DEPTH + 2;
    parse_ok(&flowchart(depth));
}

#[test]
fn deeply_nested_state_parses_without_custom_depth_error() {
    let depth = MAX_DIAGRAM_NESTING_DEPTH + 2;
    parse_ok(&state(depth));
}

#[test]
fn deeply_nested_block_parses_without_custom_depth_error() {
    let depth = MAX_DIAGRAM_NESTING_DEPTH + 2;
    parse_ok(&block(depth));
}

#[test]
fn deeply_nested_mindmap_parses_without_custom_depth_error() {
    let depth = MAX_DIAGRAM_NESTING_DEPTH + 2;
    parse_ok(&mindmap(depth));
}

#[test]
fn deeply_nested_treemap_parses_without_custom_depth_error() {
    let depth = MAX_DIAGRAM_NESTING_DEPTH + 2;
    parse_ok(&treemap(depth));
}

#[test]
fn deeply_nested_c4_parses_without_custom_depth_error() {
    let depth = MAX_DIAGRAM_NESTING_DEPTH + 2;
    parse_ok(&c4(depth));
}

#[test]
fn deeply_nested_class_parses_without_custom_depth_error() {
    let depth = MAX_DIAGRAM_NESTING_DEPTH + 2;
    parse_ok(&class_diagram(depth));
}
