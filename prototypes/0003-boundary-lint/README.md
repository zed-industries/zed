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
$ cargo run -- ../0001-hello-blink/cpp/io.cpp
../0001-hello-blink/cpp/io.cpp: OK
1 files checked, 0 violations
exit: 0

$ cargo run -- ../0002-arduino-core/cpp/sketch.cpp
../0002-arduino-core/cpp/sketch.cpp: OK
1 files checked, 0 violations
exit: 0
```

意図的に6ルール全てに違反する `examples/bad_sketch.cpp` は、6件全ての違反が検知されることを確認:

```
$ cargo run -- examples/bad_sketch.cpp
examples/bad_sketch.cpp:3:1: error: function-macro
  関数形式マクロ(#define NAME(...))はC/C++に書けません。ロジックを隠す恐れがあるため禁止しています。ロジックはRustのno_stdクレートに実装してください。

examples/bad_sketch.cpp:14:15: error: computed-intermediate
  計算式を含む変数初期化はC/C++に書けません。計算はRustのno_stdクレートで行い、結果だけをextern "C"関数の戻り値として受け取ってください。

examples/bad_sketch.cpp:16:5: error: if
  if文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:20:5: error: for
  forループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装してください。

examples/bad_sketch.cpp:24:5: error: while
  whileループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装してください。

examples/bad_sketch.cpp:28:17: error: ternary
  三項演算子はC/C++に書けません。条件分岐はRustのno_stdクレートに実装し、結果だけをextern "C"関数の戻り値として受け取ってください。

1 files checked, 1 file(s) have violations (6 violations)
exit: 1
```

3ファイルまとめて実行した場合の集計:

```
$ cargo run -- ../0001-hello-blink/cpp/io.cpp ../0002-arduino-core/cpp/sketch.cpp examples/bad_sketch.cpp
../0001-hello-blink/cpp/io.cpp: OK
../0002-arduino-core/cpp/sketch.cpp: OK
examples/bad_sketch.cpp:3:1: error: function-macro
  関数形式マクロ(#define NAME(...))はC/C++に書けません。ロジックを隠す恐れがあるため禁止しています。ロジックはRustのno_stdクレートに実装してください。

examples/bad_sketch.cpp:14:15: error: computed-intermediate
  計算式を含む変数初期化はC/C++に書けません。計算はRustのno_stdクレートで行い、結果だけをextern "C"関数の戻り値として受け取ってください。

examples/bad_sketch.cpp:16:5: error: if
  if文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:20:5: error: for
  forループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装してください。

examples/bad_sketch.cpp:24:5: error: while
  whileループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装してください。

examples/bad_sketch.cpp:28:17: error: ternary
  三項演算子はC/C++に書けません。条件分岐はRustのno_stdクレートに実装し、結果だけをextern "C"関数の戻り値として受け取ってください。

3 files checked, 1 file(s) have violations (6 violations)
exit: 1
```
