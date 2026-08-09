# プロトタイプ拡張: 本物のArduinoコアを使ったhello-blink

[`prototypes/0001-hello-blink`](../0001-hello-blink) は `avr/io.h` のレジスタ直叩きでRust/CのABI境界を検証した(RFC 0001 §5、resolved済み)。このプロトタイプは、本物の `ArduinoCore-avr`(`Arduino.h`/`pinMode`/`digitalWrite`)を使っても同じ境界ルールが成立することを検証する、独立した検証タスク。

## 構成

- `vendor/ArduinoCore-avr/` — [arduino/ArduinoCore-avr](https://github.com/arduino/ArduinoCore-avr) をgit submoduleとしてタグ `1.8.8` に固定。`cores/arduino/`(コア本体)と `variants/standard/`(Uno/Nano/Pro Mini系のピン配置)を使用。
- `cpp/sketch.cpp` — ユーザースケッチ相当。`setup()`/`loop()` は `pinMode`/`digitalWrite`/`delay` の直線呼び出しのみで、if/for/whileや計算用の中間変数を持たない。
- `rust/` — ロジック層。`#![no_std]` staticlib。`citadel_tick()` がLED状態(0/1)をトグルする。0001と違い、点滅のタイミングはRust側のカウンタではなく `sketch.cpp` の `delay(500)` が担う。
- `build.sh` — `cores/arduino/` 配下の全ソースをコンパイル(`arduino-builder`と同じ方式)→ スケッチをコンパイル → `cargo` でRustをビルド(nightlyの固定は `rust-toolchain.toml` による)→ リンク(`-Wl,--gc-sections` で未使用コアシンボルを刈る)→ `.hex` 生成、を一括実行する。
- 境界ルール(Rust/Cの分担)が適用されるのはCitadelが生成するコードとユーザーが書くコード(ここでは `cpp/sketch.cpp`)であり、ベンダリングした `ArduinoCore-avr` 自体の内部実装(`digitalWrite`/`delay`など)は対象外(libcの内部にルールが及ばないのと同様)。

## 実機での検証結果

ELEGOO UNO R3(Arduino Uno互換ボード)で実機検証済み。

```sh
git submodule update --init vendor/ArduinoCore-avr
./build.sh
avrdude -c arduino -p atmega328p -P /dev/ttyACM0 -b 115200 -U flash:w:build/firmware.hex:i
```

- `avrdude` による書き込み・ベリファイ成功
- 13番ピン(オンボードLED)の点滅を目視確認済み
- `avr-size` 実測値: text=2376 bytes(0001のtext=274 bytesとの差分がArduinoコア分のコスト)

`-P` はシリアルポートに合わせて変更する。

## ライセンスについて

`ArduinoCore-avr` はLGPL-2.1。Arduino公式のFAQによれば、スケッチ+コアをリンクした成果物はLGPLの再配布義務を負わない。Citadel本体のGPL/Apacheライセンス表記([トップレベルREADME](../../README.md#licensing)参照)とは別枠の注記。
