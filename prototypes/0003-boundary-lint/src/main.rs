use tree_sitter::{Node, Parser};

pub struct Violation {
    pub line: usize,
    pub column: usize,
    pub rule_id: &'static str,
    pub message: &'static str,
}

const BANNED_STATEMENT_KINDS: &[(&str, &str, &str)] = &[
    (
        "if_statement",
        "if",
        "if文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern \"C\"関数の戻り値として結果を受け取ってください。",
    ),
    (
        "for_statement",
        "for",
        "forループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装してください。",
    ),
    (
        "while_statement",
        "while",
        "whileループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装してください。",
    ),
    (
        "do_statement",
        "do-while",
        "do-whileループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装してください。",
    ),
    (
        "conditional_expression",
        "ternary",
        "三項演算子はC/C++に書けません。条件分岐はRustのno_stdクレートに実装し、結果だけをextern \"C\"関数の戻り値として受け取ってください。",
    ),
    (
        "preproc_function_def",
        "function-macro",
        "関数形式マクロ(#define NAME(...))はC/C++に書けません。ロジックを隠す恐れがあるため禁止しています。ロジックはRustのno_stdクレートに実装してください。",
    ),
];

const COMPUTED_INTERMEDIATE_MESSAGE: &str = "計算式を含む変数初期化はC/C++に書けません。計算はRustのno_stdクレートで行い、結果だけをextern \"C\"関数の戻り値として受け取ってください。";

fn walk(node: Node, violations: &mut Vec<Violation>) {
    for &(kind, rule_id, message) in BANNED_STATEMENT_KINDS {
        if node.kind() == kind {
            let point = node.start_position();
            violations.push(Violation {
                line: point.row + 1,
                column: point.column + 1,
                rule_id,
                message,
            });
        }
    }

    if node.kind() == "init_declarator" {
        if let Some(value) = node.child_by_field_name("value") {
            if let Some(binary) = find_binary_expression(value) {
                let point = binary.start_position();
                violations.push(Violation {
                    line: point.row + 1,
                    column: point.column + 1,
                    rule_id: "computed-intermediate",
                    message: COMPUTED_INTERMEDIATE_MESSAGE,
                });
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk(child, violations);
    }
}

fn find_binary_expression(node: Node) -> Option<Node> {
    if node.kind() == "binary_expression" {
        return Some(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(found) = find_binary_expression(child) {
            return Some(found);
        }
    }
    None
}

pub fn check_source(source: &str) -> Result<Vec<Violation>, String> {
    let mut parser = Parser::new();
    let language: tree_sitter::Language = tree_sitter_cpp::LANGUAGE.into();
    parser
        .set_language(&language)
        .map_err(|e| format!("failed to load C++ grammar: {e}"))?;

    let tree = parser
        .parse(source.as_bytes(), None)
        .ok_or_else(|| "failed to parse source".to_string())?;

    let mut violations = Vec::new();
    walk(tree.root_node(), &mut violations);
    violations.sort_by_key(|v| (v.line, v.column));
    Ok(violations)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_if_statement() {
        let violations = check_source("void loop() { if (true) { } }").unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "if");
    }

    #[test]
    fn detects_for_statement() {
        let violations = check_source("void loop() { for (int i = 0; i < 1; i++) { } }").unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "for");
    }

    #[test]
    fn detects_while_statement() {
        let violations = check_source("void loop() { while (true) { } }").unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "while");
    }

    #[test]
    fn detects_do_while_statement() {
        let violations = check_source("void loop() { do { } while (true); }").unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "do-while");
    }

    #[test]
    fn detects_ternary() {
        let violations = check_source("void loop() { int x = digitalRead(1) ? 1 : 0; }").unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "ternary");
    }

    #[test]
    fn detects_computed_intermediate() {
        let violations = check_source("void loop() { int out = raw * 2; }").unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "computed-intermediate");
    }

    #[test]
    fn detects_function_like_macro() {
        let violations = check_source("#define DOUBLE(x) ((x) * 2)\nvoid loop() {}").unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "function-macro");
    }

    #[test]
    fn allows_plain_call_initializer() {
        let violations = check_source("void loop() { int raw = analogRead(0); }").unwrap();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn allows_object_like_macro() {
        let violations = check_source("#define LED_PIN 13\nvoid loop() {}").unwrap();
        assert_eq!(violations.len(), 0);
    }
}
