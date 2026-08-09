# プロトタイプ: Rust/C境界のロジック検知(boundary-lint)

RFC 0001 §4「C/C++ユーザースケッチのロジック拒否」の実行可能性プロトタイプ。tree-sitter-cppでC++スケッチファイルを解析し、Citadelの境界ルール(`CLAUDE.md`、トップレベルREADME参照)に違反するロジック構文を検知・拒否する、スタンドアロンのネイティブRust CLI。IDE(`crates/`配下)には一切変更を加えていない。

## 拒否する7ルール

| # | 構文 | 判定方法 |
|---|---|---|
| 1 | `if` | `if_statement`ノード |
| 2 | `for` | `for_statement`ノード |
| 3 | `while` / `do-while` | `while_statement` / `do_statement`ノード |
| 4 | `switch` | `switch_statement`ノード |
| 5 | 三項演算子 | `conditional_expression`ノード |
| 6 | 計算用中間変数 | 変数宣言(`init_declarator`)の初期化式に`binary_expression`が含まれる場合(ただし`const`修飾されている場合は対象外 — 下記参照) |
| 7 | ユーザー定義関数マクロ | `preproc_function_def`ノード(`#define NAME(...) ...`) |

各違反メッセージは、拒否理由と「ロジックはRustの`no_std`クレートに実装し、`extern "C"`経由で呼び出す」という移行先を毎回明示する。

### オブジェクト形式マクロの本体もチェックする

関数形式マクロ(`#define NAME(...)`)は`preproc_function_def`として無条件禁止だが、オブジェクト形式マクロ(`#define NAME value`)は`#define LED_PIN 13`のような正当な定数宣言に使われるため許可している。ただしtree-sitterはマクロ本体を展開せず不透明な`preproc_arg`トークンとして扱うため、素朴な実装では`#define BLINK_IF_HOT if (...) { ... }`のようにロジックを隠すことができてしまっていた。

これに対処するため、`preproc_def`(オブジェクト形式マクロ)を見つけるたびに、その本体テキストを`void __macro_check() { <本体> }`という形で単体パースし直し、同じ`walk()`ロジックを再帰的に適用する。`#define LED_PIN 13`のような定数値の本体は文として閉じていない(末尾に`;`がない)ため、この再パースは構文エラーになり何も検知されない — これがちょうど「ロジックを隠していない」ことの判定にもなっている。一方`#define BLINK_IF_HOT if (...) { ... };`のように文として成立する本体は正しく再パースでき、中に隠れた`if`などがそのまま検知される(検知位置は本体内の実際の行ではなく`#define`自体の行になる)。

### ランタイム定型文とユーザースケッチの区別

`prototypes/0001-hello-blink/cpp/runtime.cpp`の`main()`は、`citadel_setup()`を一度呼んでから`citadel_loop()`を無限に呼び続けるだけの定型文(0002ではvendored Arduinoコア自身の`main.cpp`が同じ役割を果たす — これはツールの解析対象に含まれないので問題にならない)。ファイル単位で「これはランタイム側」と判定する概念は持たせず、代わりに`for`ルールに構造的な例外を1つだけ設けた: 初期化式・条件式・更新式が全て空の無限`for(;;)`で、かつ本体が関数呼び出し文だけで構成されている場合に限り例外とする。本体に`if`や宣言など呼び出し以外の文が1つでもあれば通常通り検知される(`for`の例外は素通りしても、その中の`if`は普通に検知される)。ファイル名やコメントによる判定ではなく構造だけで判定するため、任意のファイル名を付けて素通りさせることはできない。

## スコープ外(RFC 0001 §4の未決定事項のまま)

- `setup()`内の`for`を例外にするかどうか(上記の無限ディスパッチループ例外とは別の論点として残る)
- `.ino`ファイル対応(`.cpp`のみ解析)
- 変数宣言の初期化式以外での計算式(関数呼び出し引数内のインライン計算など)
- IDE統合(エディタ上の赤線表示、`crates/languages`への組み込み)。調査の結果、CitadelのdiagnosticsパイプラインはLSP実行前提で、非LSPソースからの診断投入パターンは現状存在しない。これは別の大きな取り組みであり、本プロトタイプでは着手しない。

`const uint8_t MASK = (1 << PB5);` のようなビルド時定数の初期化式は、`declaration`が`const`修飾されている場合に「計算用中間変数」ルールの対象から外すことで誤検知を修正済み(`CLAUDE.md`がboard constantsの宣言をC/C++側に認めているため)。ただしこれは構文だけを見たヒューリスティックで、tree-sitterには型やコンパイル時定数評価の情報がない — `const int cheat = raw * 2;` のように`const`さえ付ければ本物の実行時計算もすり抜けられてしまう、という新しいトレードオフを持ち込んでいる点は既知の限界として明記しておく。

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

意図的に7つの構文ルール+オブジェクト形式マクロの隠しロジックに違反する `examples/bad_sketch.cpp` は、8件全ての違反が検知されることを確認(`#define BLINK_IF_HOT ...`が隠す`if`が`4:1`、実際に書かれた`if`が`17:5`の2件のif違反があることに注意):

```
$ cargo run -- examples/bad_sketch.cpp
examples/bad_sketch.cpp:3:1: error: function-macro
  関数形式マクロ(#define NAME(...))はC/C++に書けません。ロジックを隠す恐れがあるため禁止しています。ロジックはRustのno_stdクレートに実装してください。

examples/bad_sketch.cpp:4:1: error: if
  if文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:15:15: error: computed-intermediate
  計算式を含む変数初期化はC/C++に書けません。計算はRustのno_stdクレートで行い、結果だけをextern "C"関数の戻り値として受け取ってください。

examples/bad_sketch.cpp:17:5: error: if
  if文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:21:5: error: for
  forループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:25:5: error: while
  whileループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:29:5: error: switch
  switch文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:38:17: error: ternary
  三項演算子はC/C++に書けません。条件分岐はRustのno_stdクレートに実装し、結果だけをextern "C"関数の戻り値として受け取ってください。

1 files checked, 1 file(s) have violations (8 violations)
exit: 1
```

3ファイルまとめて実行した場合の集計:

```
$ cargo run -- ../0001-hello-blink/cpp/io.cpp ../0002-arduino-core/cpp/sketch.cpp examples/bad_sketch.cpp
../0001-hello-blink/cpp/io.cpp: OK
../0002-arduino-core/cpp/sketch.cpp: OK
examples/bad_sketch.cpp:3:1: error: function-macro
  関数形式マクロ(#define NAME(...))はC/C++に書けません。ロジックを隠す恐れがあるため禁止しています。ロジックはRustのno_stdクレートに実装してください。

examples/bad_sketch.cpp:4:1: error: if
  if文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:15:15: error: computed-intermediate
  計算式を含む変数初期化はC/C++に書けません。計算はRustのno_stdクレートで行い、結果だけをextern "C"関数の戻り値として受け取ってください。

examples/bad_sketch.cpp:17:5: error: if
  if文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:21:5: error: for
  forループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:25:5: error: while
  whileループはC/C++に書けません。繰り返し制御はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:29:5: error: switch
  switch文はC/C++に書けません。この判断はRustのno_stdクレートに実装し、extern "C"関数の戻り値として結果を受け取ってください。

examples/bad_sketch.cpp:38:17: error: ternary
  三項演算子はC/C++に書けません。条件分岐はRustのno_stdクレートに実装し、結果だけをextern "C"関数の戻り値として受け取ってください。

3 files checked, 1 file(s) have violations (8 violations)
exit: 1
```

`prototypes/0001-hello-blink/cpp/runtime.cpp`(無限ディスパッチループの例外に該当する、唯一の`for`を含むファイル)も違反0件でクリーンに通ることを確認:

```
$ cargo run -- ../0001-hello-blink/cpp/runtime.cpp
../0001-hello-blink/cpp/runtime.cpp: OK
1 files checked, 0 violations
exit: 0
```
