# RFC 0001 プロトタイプ: hello-blink

[RFC 0001](../../docs/rfcs/0001-hybrid-architecture.md) の未解決項目のうち、Rust/Cハイブリッドアーキテクチャが実際にビルド・リンクできるかを検証する最小プロトタイプ。ATmega328P(Arduino Uno/Nano/Pro Mini相当)向け。

## 構成

- `rust/` — ロジック層。`#![no_std]` staticlib。`citadel_tick()` がLED点滅の状態(0/1)を計算する。分岐・状態はすべてここにある。
- `cpp/io.cpp` — I/O層(ユーザースケッチに相当)。`citadel_setup()`/`citadel_loop()` は直線的なレジスタ読み書きのみで、if/for/whileや計算用の中間変数を持たない。`citadel_tick()` の戻り値をそのままポートに書き込むだけ。
- `cpp/runtime.cpp` — `main()` を所有する側(実運用ではArduinoコアのwiring.cに相当)。`citadel_setup()` を一度呼び、`citadel_loop()` を無限に呼ぶだけのランタイム。制御フローが必要なのはこの層であり、ユーザースケッチ(`io.cpp`)ではない。
- `build.sh` — `avr-g++` でのコンパイル → `cargo +nightly` でのRustビルド → リンク → `avr-objcopy` での `.hex` 生成 → `avr-size`/`avr-nm` による検証、を一括実行する。

## このプロトタイプが実証したこと(ホスト上でのビルドのみ)

`./build.sh` を実行すると:
- `avr-g++` でコンパイルしたオブジェクトと `rustc`(LLVM)でコンパイルした `libcitadel_logic.a` が1つの `.elf` にリンクできる(RFC 0001 §5: ABI/リンク相互運用性)
- `avr-nm` で `citadel_setup`/`citadel_loop`/`citadel_tick` が全て解決され、未定義参照(`U`)がゼロであることを確認できる
- `avr-objcopy` で `.hex` が生成できる(RFC 0001 §2)
- 固定した nightly(`rust-toolchain.toml` 参照)でのビルドが再現できる(RFC 0001 §1)
- `avr-size` の実測値(text 274 bytes、ATmega328Pの32KB中 約0.8%)が得られる(RFC 0001 §3 の参考データ点 — このプロトタイプ自体は最小限のプログラムなので、「代表的なスケッチ」での再測定は別途必要)

## 実機での検証結果

ELEGOO UNO R3(Arduino Uno互換ボード)で実機検証済み。RFC 0001 §5の「実機にフラッシュして検証」はクローズ。

```sh
./build.sh
avrdude -c arduino -p atmega328p -P /dev/ttyACM0 -b 115200 -U flash:w:build/firmware.hex:i
```

- `avrdude` による書き込み・ベリファイ成功(274 bytes、差分なし)
- 13番ピン(オンボードLED, PB5)の点滅を目視確認済み

`-P` はシリアルポートに合わせて変更する。

## このプロトタイプが実証していないこと

- 本物のArduinoコア(`Arduino.h`/`pinMode`/`digitalWrite`)は使っていない。`avr/io.h` によるレジスタ直叩き。Arduinoコアのベンダリングは別タスク。
- C/C++の静的解析によるロジック拒否(RFC 0001 §4)はこのプロトタイプの対象外。
