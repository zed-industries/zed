# Boundary Lint Prototype (0003-boundary-lint) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `prototypes/0003-boundary-lint`, a standalone native Rust CLI that parses `.cpp` sketch files with tree-sitter and detects/rejects six categories of logic constructs that violate Citadel's Rust/C boundary rule, proving RFC 0001 §4's detection mechanism works.

**Architecture:** A single-crate Rust binary depends on `tree-sitter` + `tree-sitter-cpp` (same versions the main Citadel `Cargo.toml` already uses). Core detection logic (`check_source`, tree-walking, violation collection) is pure and unit-tested against inline source snippets; a thin CLI layer (`check_file`, `main`) wraps it for real files. No changes to `crates/` or the IDE.

**Tech Stack:** Rust (stable, host target — not `avr-none`), `tree-sitter = "0.26.9"`, `tree-sitter-cpp` (git rev `5cb9b693cfd7bfacab1d9ff4acac1a4150700609`).

## Global Constraints

- Standalone native Rust CLI only — no changes to `crates/` or any IDE integration (editor squiggles, LSP diagnostics) in this plan.
- Only `.cpp` files are parsed; `#include` directives are never followed — the tool only examines the file(s) given on the command line.
- Exactly six rules, no more: `if`, `for`, `while`/`do-while`, ternary (`conditional_expression`), computed intermediate variable (an `init_declarator`'s `value` field subtree contains a `binary_expression`), and user-defined function-like macro (`preproc_function_def`).
- Explicitly out of scope, do not implement: `setup()`-body exemption for `for`, `.ino` file support, computed expressions outside declaration initializers (e.g. inline arithmetic in a call argument).
- Dependency versions must match the main repo's `Cargo.toml`: `tree-sitter = "0.26.9"`, `tree-sitter-cpp = { git = "https://github.com/tree-sitter/tree-sitter-cpp", rev = "5cb9b693cfd7bfacab1d9ff4acac1a4150700609" }`.
- Every violation message is Japanese, explains why the construct is rejected, and says where the logic belongs (Rust, via `extern "C"`).
- A file's walk collects every violation — it does not stop at the first one.

---

### Task 1: Core detection logic with unit tests

**Files:**
- Create: `prototypes/0003-boundary-lint/Cargo.toml`
- Create: `prototypes/0003-boundary-lint/src/main.rs`

**Interfaces:**
- Consumes: nothing (first task)
- Produces: `pub struct Violation { pub line: usize, pub column: usize, pub rule_id: &'static str, pub message: &'static str }` (all fields `pub` within the crate — single-file crate, no visibility concerns) and `pub fn check_source(source: &str) -> Result<Vec<Violation>, String>`. Task 2 calls `check_source` from its own `check_file` wrapper — do not change this signature.

- [ ] **Step 1: Write the crate manifest**

`prototypes/0003-boundary-lint/Cargo.toml`:

```toml
[workspace]

[package]
name = "boundary-lint"
version = "0.1.0"
edition = "2021"

[dependencies]
tree-sitter = "0.26.9"
tree-sitter-cpp = { git = "https://github.com/tree-sitter/tree-sitter-cpp", rev = "5cb9b693cfd7bfacab1d9ff4acac1a4150700609" }
```

- [ ] **Step 2: Write the failing tests**

`prototypes/0003-boundary-lint/src/main.rs` (full file for this step — the `main` function is a temporary empty stub; Task 2 replaces it):

```rust
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
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cd prototypes/0003-boundary-lint && cargo test`
Expected: compile error — `check_source` is not defined.

- [ ] **Step 4: Implement the detection logic**

Edit `prototypes/0003-boundary-lint/src/main.rs`: insert the code below immediately before the `fn main() {}` line that Step 2 wrote. Do not touch the `#[cfg(test)] mod tests { ... }` block below it — it stays exactly as Step 2 wrote it. After this edit, the file reads top-to-bottom as: the new code shown here, then `fn main() {}` (same empty stub, now preceded by real code instead of standing alone), then the unchanged test module.

```rust
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
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cd prototypes/0003-boundary-lint && cargo test`
Expected: all 9 tests pass (`test result: ok. 9 passed; 0 failed`).

- [ ] **Step 6: Commit**

```bash
cd /home/gooya/citadel
git add prototypes/0003-boundary-lint/Cargo.toml prototypes/0003-boundary-lint/src/main.rs
git commit -m "$(cat <<'EOF'
Add core detection logic for the boundary lint prototype

Six unit-tested rules (if/for/while/do-while/ternary/computed-
intermediate/function-macro) implemented as a pure check_source(&str)
function over the tree-sitter-cpp syntax tree. CLI wiring is Task 2.
EOF
)"
```

---

### Task 2: CLI wiring, example fixture, and README

**Files:**
- Modify: `prototypes/0003-boundary-lint/src/main.rs` (replace the stub `fn main() {}` with the real CLI; add `check_file`)
- Create: `prototypes/0003-boundary-lint/examples/bad_sketch.cpp`
- Create: `prototypes/0003-boundary-lint/README.md`

**Interfaces:**
- Consumes: `pub fn check_source(source: &str) -> Result<Vec<Violation>, String>` and `pub struct Violation { line: usize, column: usize, rule_id: &'static str, message: &'static str }` from Task 1 — do not change these.
- Produces: nothing consumed by a later task (this is the final task).

- [ ] **Step 1: Replace the stub main() with the real CLI**

In `prototypes/0003-boundary-lint/src/main.rs`, the line `fn main() {}` now sits between the detection logic (Task 1 Step 4) and the `#[cfg(test)] mod tests { ... }` block (Task 1 Step 2) — leave the test module untouched, but replace just that `fn main() {}` line with:

```rust
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

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
```

- [ ] **Step 2: Run the unit tests to confirm nothing broke**

Run: `cd prototypes/0003-boundary-lint && cargo test`
Expected: all 9 tests from Task 1 still pass.

- [ ] **Step 3: Write the deliberately-bad example sketch**

`prototypes/0003-boundary-lint/examples/bad_sketch.cpp` (not meant to build with `avr-g++` — it exists only to be parsed by `boundary-lint`, one violation of each of the six rules):

```cpp
#include <Arduino.h>

#define DOUBLE(x) ((x) * 2)

const int SENSOR_PIN = A0;
const int LED_PIN = 13;

void setup() {
    pinMode(LED_PIN, OUTPUT);
}

void loop() {
    int raw = analogRead(SENSOR_PIN);
    int out = raw * 2;

    if (out > 512) {
        digitalWrite(LED_PIN, HIGH);
    }

    for (int i = 0; i < 3; i++) {
        digitalWrite(LED_PIN, LOW);
    }

    while (raw > 1000) {
        raw--;
    }

    int state = digitalRead(LED_PIN) ? 1 : 0;
    digitalWrite(LED_PIN, state);
}
```

- [ ] **Step 4: Run against the two known-clean sketches and confirm 0 violations each**

Run:
```bash
cd /home/gooya/citadel/prototypes/0003-boundary-lint
cargo run -- ../0001-hello-blink/cpp/io.cpp
cargo run -- ../0002-arduino-core/cpp/sketch.cpp
```
Expected: each prints `<path>: OK` followed by `1 files checked, 0 violations`, and exits 0 (check with `echo $?` after each).

- [ ] **Step 5: Run against the bad example and confirm exactly 6 violations**

Run:
```bash
cd /home/gooya/citadel/prototypes/0003-boundary-lint
cargo run -- examples/bad_sketch.cpp
echo "exit: $?"
```
Expected: 6 violation blocks are printed — one each with `rule_id` `function-macro`, `computed-intermediate`, `if`, `for`, `while`, `ternary` — followed by `1 files checked, 1 file(s) have violations (6 violations)`, and `exit: 1`.

If the count is not exactly 6 (e.g. a nested `binary_expression` inside the ternary's condition also triggers `computed-intermediate`), stop and re-examine `examples/bad_sketch.cpp` rather than adjusting the detection logic — the fixture in Step 3 was designed so the ternary's condition (`digitalRead(LED_PIN)`) contains no `binary_expression`, specifically to avoid a double-count on that line.

- [ ] **Step 6: Run all three files together and confirm the combined summary**

Run:
```bash
cd /home/gooya/citadel/prototypes/0003-boundary-lint
cargo run -- ../0001-hello-blink/cpp/io.cpp ../0002-arduino-core/cpp/sketch.cpp examples/bad_sketch.cpp
```
Expected final line: `3 files checked, 1 file(s) have violations (6 violations)`.

- [ ] **Step 7: Write the README**

`prototypes/0003-boundary-lint/README.md`, with the actual command output from Steps 4-6 pasted in verbatim (not paraphrased) in place of `<paste ...>`:

```markdown
# プロトタイプ: Rust/C境界のロジック検知(boundary-lint)

RFC 0001 §4「C/C++ユーザースケッチのロジック拒否」の実行可能性プロトタイプ。tree-sitter-cppでC++スケッチファイルを解析し、Citadelの境界ルール(`CLAUDE.md`、トップレベルREADME参照)に違反するロジック構文を検知・拒否する、スタンドアロンのネイティブRust CLI。IDE(`crates/`配下)には一切変更を加えていない。

## 拒否する6ルール

| # | 構文 | 判定方法 |
|---|---|---|
| 1 | `if` | `if_statement`ノード |
| 2 | `for` | `for_statement`ノード |
| 3 | `while` / `do-while` | `while_statement` / `do_statement`ノード |
| 4 | 三項演算子 | `conditional_expression`ノード |
| 5 | 計算用中間変数 | 変数宣言(`init_declarator`)の初期化式に`binary_expression`が含まれる場合 |
| 6 | ユーザー定義関数マクロ | `preproc_function_def`ノード(`#define NAME(...) ...`) |

各違反メッセージは、拒否理由と「ロジックはRustの`no_std`クレートに実装し、`extern "C"`経由で呼び出す」という移行先を毎回明示する。

## スコープ外(RFC 0001 §4の未決定事項のまま)

- `setup()`内の`for`を例外にするかどうか
- `.ino`ファイル対応(`.cpp`のみ解析)
- 変数宣言の初期化式以外での計算式(関数呼び出し引数内のインライン計算など)
- IDE統合(エディタ上の赤線表示、`crates/languages`への組み込み)。調査の結果、CitadelのdiagnosticsパイプラインはLSP実行前提で、非LSPソースからの診断投入パターンは現状存在しない。これは別の大きな取り組みであり、本プロトタイプでは着手しない。

## 使い方

```sh
cargo run -- <file.cpp> [file2.cpp ...]
```

## 検証結果

`prototypes/0001-hello-blink/cpp/io.cpp` と `prototypes/0002-arduino-core/cpp/sketch.cpp` はいずれも境界ルールを守って書かれており、違反0件でクリーンに通ることを確認:

```
<paste the actual output of Step 4's two commands here>
```

意図的に6ルール全てに違反する `examples/bad_sketch.cpp` は、6件全ての違反が検知されることを確認:

```
<paste the actual output of Step 5 here>
```

3ファイルまとめて実行した場合の集計:

```
<paste the actual output of Step 6 here>
```
```

- [ ] **Step 8: Commit**

```bash
cd /home/gooya/citadel
git add prototypes/0003-boundary-lint/src/main.rs prototypes/0003-boundary-lint/examples/bad_sketch.cpp prototypes/0003-boundary-lint/README.md
git commit -m "$(cat <<'EOF'
Add CLI, example fixture, and README for the boundary lint prototype

Verified against prototypes/0001-hello-blink and 0002-arduino-core
(0 violations each) and a new deliberately-bad example sketch that
exercises all six rules (6 violations).
EOF
)"
git push
```
