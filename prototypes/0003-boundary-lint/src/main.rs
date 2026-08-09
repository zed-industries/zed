use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;
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
        "forループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装し、extern \"C\"関数の戻り値として結果を受け取ってください。",
    ),
    (
        "while_statement",
        "while",
        "whileループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装し、extern \"C\"関数の戻り値として結果を受け取ってください。",
    ),
    (
        "do_statement",
        "do-while",
        "do-whileループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装し、extern \"C\"関数の戻り値として結果を受け取ってください。",
    ),
    (
        "switch_statement",
        "switch",
        "switch文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern \"C\"関数の戻り値として結果を受け取ってください。",
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

fn walk(node: Node, source: &[u8], violations: &mut Vec<Violation>) {
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

    if node.kind() == "init_declarator" && !enclosing_declaration_is_const(node) {
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

    if node.kind() == "preproc_def" {
        check_object_macro_body(node, source, violations);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk(child, source, violations);
    }
}

// Object-like macros (`#define NAME value`) are otherwise allowed (unlike
// function-like macros, which are banned outright), but tree-sitter treats
// their body as an opaque `preproc_arg` token — so `#define BLINK_IF_HOT
// if (...) { ... }` would hide an if-statement from the walk above entirely.
// Re-parse the macro body on its own, wrapped in a synthetic function, and
// reuse the same walk() to catch banned constructs inside it too. A plain
// constant body (`#define LED_PIN 13`) has no trailing `;` and fails to
// parse as a statement when wrapped, which is exactly the signal we want to
// tell it apart from a real hidden statement — nothing is flagged for it.
fn check_object_macro_body(macro_def: Node, source: &[u8], violations: &mut Vec<Violation>) {
    let Some(value) = macro_def.child_by_field_name("value") else {
        return;
    };
    let Ok(body_text) = value.utf8_text(source) else {
        return;
    };

    let wrapped = format!("void __macro_check() {{ {body_text} }}");
    let mut parser = Parser::new();
    let language: tree_sitter::Language = tree_sitter_cpp::LANGUAGE.into();
    if parser.set_language(&language).is_err() {
        return;
    }
    let Some(tree) = parser.parse(wrapped.as_bytes(), None) else {
        return;
    };
    if tree.root_node().has_error() {
        return;
    }

    let mut inner_violations = Vec::new();
    walk(tree.root_node(), wrapped.as_bytes(), &mut inner_violations);
    if inner_violations.is_empty() {
        return;
    }

    let point = macro_def.start_position();
    for inner in inner_violations {
        violations.push(Violation {
            line: point.row + 1,
            column: point.column + 1,
            rule_id: inner.rule_id,
            message: inner.message,
        });
    }
}

// A `const`-qualified declaration (e.g. `const uint8_t MASK = (1 << PB5);`) is
// the board-constant idiom CLAUDE.md explicitly allows, not a runtime
// calculation — skip the computed-intermediate check for it. This is a
// syntactic heuristic (tree-sitter has no type/const-expression evaluation),
// so it will also excuse a genuinely runtime-computed `const` declaration;
// documented as a known limitation.
fn enclosing_declaration_is_const(init_declarator: Node) -> bool {
    let Some(declaration) = init_declarator.parent() else {
        return false;
    };
    if declaration.kind() != "declaration" {
        return false;
    }
    let mut cursor = declaration.walk();
    for child in declaration.children(&mut cursor) {
        if child.kind() != "type_qualifier" {
            continue;
        }
        let mut qualifier_cursor = child.walk();
        for qualifier in child.children(&mut qualifier_cursor) {
            if qualifier.kind() == "const" {
                return true;
            }
        }
    }
    false
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

    if tree.root_node().has_error() {
        return Err(
            "ファイルを正しくパースできませんでした(構文エラーがあります)。壊れた構文木からは違反検知を行えないため、境界チェックを実行できません。構文を修正してから再実行してください。"
                .to_string(),
        );
    }

    let mut violations = Vec::new();
    walk(tree.root_node(), source.as_bytes(), &mut violations);
    violations.sort_by_key(|v| (v.line, v.column));
    Ok(violations)
}

fn check_file(path: &Path) -> Result<Vec<Violation>, String> {
    let source = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    check_source(&source)
}

fn main() -> ExitCode {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("usage: boundary-lint <file.cpp> [file2.cpp ...]");
        return ExitCode::FAILURE;
    }

    let mut files_checked = 0usize;
    let mut files_with_violations = 0usize;
    let mut total_violations = 0usize;

    for path_str in &paths {
        let path = Path::new(path_str);
        files_checked += 1;

        match check_file(path) {
            Ok(violations) if violations.is_empty() => {
                println!("{}: OK", path.display());
            }
            Ok(violations) => {
                files_with_violations += 1;
                total_violations += violations.len();
                for violation in &violations {
                    println!(
                        "{}:{}:{}: error: {}\n  {}\n",
                        path.display(),
                        violation.line,
                        violation.column,
                        violation.rule_id,
                        violation.message
                    );
                }
            }
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
    }

    if total_violations == 0 {
        println!("{files_checked} files checked, 0 violations");
        ExitCode::SUCCESS
    } else {
        println!(
            "{files_checked} files checked, {files_with_violations} file(s) have violations ({total_violations} violations)"
        );
        ExitCode::FAILURE
    }
}

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
    fn detects_switch_statement() {
        let violations =
            check_source("void loop() { switch (1) { case 0: break; } }").unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "switch");
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

    #[test]
    fn detects_logic_hidden_in_object_macro() {
        let violations = check_source(
            "#define BLINK_IF_HOT if (analogRead(0) > 512) digitalWrite(13, HIGH);\nvoid loop() {}",
        )
        .unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_id, "if");
        assert_eq!(violations[0].line, 1);
    }

    #[test]
    fn allows_object_macro_with_plain_call_body() {
        let violations = check_source(
            "#define TURN_ON digitalWrite(13, HIGH);\nvoid loop() {}",
        )
        .unwrap();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn allows_const_bitshift_declaration() {
        let violations =
            check_source("void loop() { const uint8_t MASK = (1 << 5); }").unwrap();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn rejects_unparseable_source() {
        let result = check_source(
            "void loop() {\n    struct { int a;\n    if (analogRead(0) > 512) { digitalWrite(13, HIGH); }\n}",
        );
        assert!(result.is_err());
    }
}
