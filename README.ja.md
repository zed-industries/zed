# Citadel

*[English](./README.md)*

**Citadel**（別名 *Citadel-Duino*）は [Zed](https://github.com/zed-industries/zed) のフォークで、Arduino（AVR）向け組み込み開発に特化した次世代IDEとして開発中です。

> 本プロジェクトは独立した非営利のフォークです。Zed Industries, Inc. とは提携・後援関係にありません。「Zed」はZed Industries, Inc. の商標です。

### Philosophy

> "If it compiles, it runs safely. Never freeze the system, never lock the chip."

### Citadelを作る理由

| 怒り (Frustration) | 対策 (Countermeasure) |
|---|---|
| IDEがクソ重くて作業効率が下がる | ZedのGPU超高速描画（GPUI）とRust製の爆速コアで解決 |
| ビルドが通っても実機でフリーズして原因不明 | コアロジックをRust (no_std) で実装し、メモリ破壊をコンパイル時に排除 |
| 不正終了でIDEが固まり、マイコンのポートも掴みっぱなしで詰む | バックエンドを完全別プロセス化。自動WDT挿入＆レスキューシーケンスで物理・論理の両面からフェイルセーフ |
| 微妙なコードの変更をGitで綺麗に管理・追跡しにくい | Zed標準のインラインGit連携とブランチ連動型ビルドで進捗を地続きに |

スコープ・開発体験の方針・意図的に対象外としている範囲は [RFC 0002](./docs/rfcs/0002-product-scope-and-dx.md)（英語）を参照。実装可能になる前に解決すべき技術的課題は [RFC 0001](./docs/rfcs/0001-hybrid-architecture.md)（英語）で管理している。

### アーキテクチャ: RustとCの境界線

Citadelはハードウェア I/O とロジックの間に明確な境界線を引く。これがプロジェクト全体の土台となるルール。

- **C/C++** が許されるのは直線的な I/O の受け渡しのみ — ピンを読む、ピンに書く、バイトを送る。`if` も `for`/`while` も、計算のための中間変数も禁止。ピン番号などボード固有の定数だけはここに置く。I/O部分までRustで書こうとすると途方もない時間がかかり組み込みエコシステム全体を安全化できない — この境界線があるからこそ現実的なトレードオフになる。
- **Rust** が状態遷移・計算・制御判断などロジックのすべてを担う。分岐や計算が許されるのはここだけ。
- 境界を越えるのは単なるデータの受け渡しのみ: C/C++は `extern "C"` で宣言されたRustの関数を呼び出し、Rustは `extern "C"` でC/C++側に定義された変数・定数を読み取る。
- `if`・`for`・`while`・三項演算子などロジック構造を含むC/C++ソースはビルドが通らない — これはスタイルガイドではなくコンパイル時のゲート。

| 項目 | 従来のArduino/C++開発 | Citadel（ハイブリッド＋厳格ルール） |
|---|---|---|
| C++資産（ライブラリ） | 100% そのまま使える | 100% そのまま使える |
| I/O・周辺機器の制御 | C++で記述（バグの危険あり） | C++で直進処理（直線的な呼び出しのみ） |
| 条件分岐・状態管理 | C++で記述（メモリ破壊・バグの巣窟） | 100% Rustで記述（コンパイル時安全保証） |
| C++にロジックを書いた時 | コンパイルが通って実機で挙動不審に | IDEが構文解析してErrorで弾く |
| 割り込み（ISR）の危険性 | volatile忘れやデータ競合でフリーズ | Rustの型システム（Mutex）で物理遮断 |
| 万が一のフリーズ/ハング | IDEごと固まりマイコンもロック | プロセス分離＆自動WDTで即座に復帰 |

Citadelが許可する `loop()` の唯一の形:

```cpp
#include <Arduino.h>

const int SENSOR_PIN = A0;
const int MOTOR_PIN  = 9;

extern "C" int process_sensor_value(int raw); // ロジックはRust側

void setup() {
    pinMode(SENSOR_PIN, INPUT);
    pinMode(MOTOR_PIN, OUTPUT);
}

void loop() {
    int raw = analogRead(SENSOR_PIN);
    int out = process_sensor_value(raw); // if/for/計算はここには書けない
    analogWrite(MOTOR_PIN, out);
}
```

```
[ 1. 入力 (C/C++) ]
   センサー値 / タイマー割込み / 受信データ
                │
                ▼ データの受け渡し
   ┌─────────────────────────┐
   │   2. ロジック (Rust)     │  ◄── 絶対安全の要塞
   │   型安全な状態遷移       │
   │   メモリ破壊のゼロ化     │
   │   計算・制御判断         │
   └─────────────────────────┘
                │
                ▼ 返り値
   [ 3. 出力 (C/C++) ]
   モータ駆動 / ディスプレイ表示 / 送信
```

上の図はランタイムのデータフロー。ビルド時にはこの2つを別々にコンパイルし、IDEのワンボタンで1つのバイナリに結合する:

```
[ ユーザー ] ──( ワンボタン押下 )──> [ Citadel IDE ]
                                         │
    ┌────────────────────────────────────┴────────────────────────────────────┐
    ▼                                                                         ▼
【1. I/O・スケッチ部】                                                    【2. ロジック部】
  avr-g++ でビルド                                                         cargo build でビルド
  (既存C++ライブラリをそのまま利用)                                         (コンパイラがメモリ安全をガチガチに検証)
    │                                                                         │
    └────────────────────────────────────┬────────────────────────────────────┘
                                         ▼
                             【3. 静的リンク & 変換】
                               avr-gcc で1つに結合 ➔ .hex 生成
                                         │
                                         ▼
                             【4. 自動書き込み】
                               avrdude で実機へ転送！
```

### 現在の状態

初期段階のフォーク — まだArduino IDEとして機能しません。現状はRFC策定・ツールチェーン検証の段階です。

### ビルド方法

現時点ではupstream Zedからビルド手順は変更していません:

- [macOS向けビルド](./docs/src/development/macos.md)
- [Linux向けビルド](./docs/src/development/linux.md)
- [Windows向けビルド](./docs/src/development/windows.md)

### ライセンス

CitadelはZedのフォークであり、ライセンスもそのまま継承しています: ソースコードは主に **GPL-3.0-or-later**、一部 **Apache-2.0** 表示のあるコンポーネントを含みます（[LICENSE-GPL](./LICENSE-GPL)、[LICENSE-APACHE](./LICENSE-APACHE) を参照）。

利用者向けの注記: GPLが適用されるのはIDE本体であり、あなたがCitadelを使って書いた・コンパイルしたコードには適用されません。Citadelで作成したスケッチ／ファームウェアがGPLになることはありません。

サードパーティ依存関係のライセンス情報が正しく提供されないとCIが通りません。ライセンス遵守には [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) を利用しています — 仕組みについては現時点では変更していないupstream Zedの `script/licenses/zed-licenses.toml` を参照してください。
