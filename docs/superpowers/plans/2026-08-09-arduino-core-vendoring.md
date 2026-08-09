# Arduino Core Vendoring Prototype (0002-arduino-core) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `prototypes/0002-arduino-core`, a minimal blink prototype that uses the real `ArduinoCore-avr` (`Arduino.h`/`pinMode`/`digitalWrite`) instead of raw register access, verified end-to-end on the ELEGOO UNO R3 already confirmed working in `prototypes/0001-hello-blink`.

**Architecture:** Vendor `ArduinoCore-avr` as a git submodule pinned to release tag `1.8.8`. `build.sh` compiles the entire `cores/arduino/` directory (mirroring how `arduino-builder` produces `core.a`), compiles a straight-line `cpp/sketch.cpp`, builds the reused-shape Rust `no_std` logic crate, and links all three with `-Wl,--gc-sections` to drop unused core code. The real core's `main.cpp` owns `main()` — there is no custom runtime file in this prototype.

**Tech Stack:** avr-gcc/avr-g++ (AVR toolchain), Rust nightly (`avr-none` target, `-Z build-std=core`), avrdude, git submodules.

## Global Constraints

- C/C++ may only perform direct, linear I/O hand-off (no `if`/`for`/`while`/ternaries/computed intermediates) — from `CLAUDE.md` and the top-level README's architecture section.
- All logic/state transitions live in the Rust `no_std` crate; C/C++ and Rust exchange only plain data across `extern "C"`.
- The `ArduinoCore-avr` submodule must be pinned to a fixed release tag (`1.8.8`), not tracking `main` — same reproducibility rule 0001 applies to its Rust nightly pin.
- Scope is the `standard` variant (Uno/Nano/Pro Mini family) on `atmega328p` only — matches the already-verified ELEGOO UNO R3 hardware. No other boards/variants, no Serial/other peripherals, no IDE integration, no static-analysis lint.
- Verification requires both: `avr-nm` showing zero undefined symbols after linking, and an actual flash + visual LED-blink confirmation on the ELEGOO UNO R3 at `/dev/ttyACM0`.

---

### Task 1: Vendor ArduinoCore-avr as a pinned git submodule

**Files:**
- Create: `prototypes/0002-arduino-core/` (directory)
- Create: `prototypes/0002-arduino-core/vendor/ArduinoCore-avr/` (git submodule)
- Modify: `.gitmodules` (repo root)

**Interfaces:**
- Consumes: nothing (first task)
- Produces: `prototypes/0002-arduino-core/vendor/ArduinoCore-avr/cores/arduino/` (core sources, `.c`/`.cpp`) and `prototypes/0002-arduino-core/vendor/ArduinoCore-avr/variants/standard/pins_arduino.h`, both consumed by Task 3's `build.sh`.

- [ ] **Step 1: Create the prototype directory**

```bash
mkdir -p /home/gooya/citadel/prototypes/0002-arduino-core
```

- [ ] **Step 2: Add the ArduinoCore-avr submodule**

```bash
cd /home/gooya/citadel
git submodule add https://github.com/arduino/ArduinoCore-avr.git prototypes/0002-arduino-core/vendor/ArduinoCore-avr
```

- [ ] **Step 3: Pin the submodule to release tag 1.8.8**

```bash
cd /home/gooya/citadel/prototypes/0002-arduino-core/vendor/ArduinoCore-avr
git checkout 86df345b3cf46754a5db38fb983ec2808ce31303
cd /home/gooya/citadel
```

Expected: `git -C prototypes/0002-arduino-core/vendor/ArduinoCore-avr rev-parse HEAD` prints `86df345b3cf46754a5db38fb983ec2808ce31303`.

- [ ] **Step 4: Verify core files are present**

```bash
ls /home/gooya/citadel/prototypes/0002-arduino-core/vendor/ArduinoCore-avr/cores/arduino/main.cpp
ls /home/gooya/citadel/prototypes/0002-arduino-core/vendor/ArduinoCore-avr/variants/standard/pins_arduino.h
```

Expected: both files exist (no "No such file" error).

- [ ] **Step 5: Commit**

```bash
cd /home/gooya/citadel
git add .gitmodules prototypes/0002-arduino-core/vendor/ArduinoCore-avr
git commit -m "$(cat <<'EOF'
Vendor ArduinoCore-avr as a pinned submodule for the 0002 prototype

Pinned to release tag 1.8.8 for the same reproducibility reason
0001 pins a specific Rust nightly.
EOF
)"
```

---

### Task 2: Rust logic crate

**Files:**
- Create: `prototypes/0002-arduino-core/rust-toolchain.toml`
- Create: `prototypes/0002-arduino-core/rust/Cargo.toml`
- Create: `prototypes/0002-arduino-core/rust/src/lib.rs`

**Interfaces:**
- Consumes: nothing
- Produces: `citadel_tick() -> u8` (`extern "C"`, `#[no_mangle]`), a `#![no_std]` `staticlib` at `rust/target/avr-none/release/libcitadel_logic.a`. Consumed by Task 3's `cpp/sketch.cpp` (declared `extern "C"`) and by Task 3's `build.sh` link step (`-lcitadel_logic`).

- [ ] **Step 1: Write the toolchain pin**

`prototypes/0002-arduino-core/rust-toolchain.toml`:

```toml
[toolchain]
channel = "nightly-2026-08-06"
components = ["rust-src"]
```

- [ ] **Step 2: Write the crate manifest**

`prototypes/0002-arduino-core/rust/Cargo.toml`:

```toml
[workspace]

[package]
name = "citadel_logic"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
crate-type = ["staticlib"]

[profile.release]
panic = "abort"
opt-level = "s"
lto = true
```

- [ ] **Step 3: Write the logic crate**

`prototypes/0002-arduino-core/rust/src/lib.rs`:

```rust
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Unlike 0001's busy-loop counter, pacing here comes from the Arduino
// core's delay(500) call in the sketch, so every call toggles directly.
static mut LED_STATE: u8 = 0;

#[no_mangle]
pub extern "C" fn citadel_tick() -> u8 {
    unsafe {
        LED_STATE ^= 1;
        LED_STATE
    }
}
```

- [ ] **Step 4: Build and verify the staticlib**

```bash
cd /home/gooya/citadel/prototypes/0002-arduino-core/rust
RUSTFLAGS="-C target-cpu=atmega328p" cargo build --release -Z build-std=core --target avr-none
ls target/avr-none/release/libcitadel_logic.a
```

Expected: build succeeds, `libcitadel_logic.a` exists.

- [ ] **Step 5: Commit**

```bash
cd /home/gooya/citadel
git add prototypes/0002-arduino-core/rust-toolchain.toml prototypes/0002-arduino-core/rust/Cargo.toml prototypes/0002-arduino-core/rust/src/lib.rs
git commit -m "$(cat <<'EOF'
Add citadel_logic Rust crate for the 0002 prototype

Same no_std/staticlib shape as 0001, but citadel_tick() toggles on
every call since the sketch's delay(500) provides timing instead of
a busy-loop counter.
EOF
)"
```

---

### Task 3: Sketch layer and build script

**Files:**
- Create: `prototypes/0002-arduino-core/cpp/sketch.cpp`
- Create: `prototypes/0002-arduino-core/build.sh` (executable)

**Interfaces:**
- Consumes: `citadel_tick() -> u8` from Task 2 (declared `extern "C"` in `sketch.cpp`); `cores/arduino/*.{c,cpp}` and `variants/standard/pins_arduino.h` from Task 1.
- Produces: `prototypes/0002-arduino-core/build/firmware.hex`, consumed by Task 5's `avrdude` flash step.

- [ ] **Step 1: Write the sketch**

`prototypes/0002-arduino-core/cpp/sketch.cpp`:

```cpp
#include <Arduino.h>

const int LED_PIN = LED_BUILTIN;

extern "C" uint8_t citadel_tick(void); // logic lives in Rust

void setup() {
    pinMode(LED_PIN, OUTPUT);
}

void loop() {
    digitalWrite(LED_PIN, citadel_tick());
    delay(500); // core-provided delay; no branch, no computed intermediate
}
```

- [ ] **Step 2: Write the build script**

`prototypes/0002-arduino-core/build.sh`:

```bash
#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

MCU=atmega328p
F_CPU=16000000L
BUILD_DIR=build
CORE_DIR=vendor/ArduinoCore-avr/cores/arduino
VARIANT_DIR=vendor/ArduinoCore-avr/variants/standard

CORE_DEFINES="-DF_CPU=$F_CPU -DARDUINO=10808 -DARDUINO_AVR_UNO -DARDUINO_ARCH_AVR"
CORE_INCLUDES="-I$CORE_DIR -I$VARIANT_DIR"

mkdir -p "$BUILD_DIR/core"

echo "=== compiling ArduinoCore-avr (cores/arduino) ==="
for src in "$CORE_DIR"/*.c; do
    obj="$BUILD_DIR/core/$(basename "${src%.c}").o"
    avr-gcc -mmcu=$MCU -Os -w -std=gnu11 -ffunction-sections -fdata-sections \
        $CORE_DEFINES $CORE_INCLUDES -c "$src" -o "$obj"
done
for src in "$CORE_DIR"/*.cpp; do
    obj="$BUILD_DIR/core/$(basename "${src%.cpp}").o"
    avr-g++ -mmcu=$MCU -Os -w -std=gnu++11 -fpermissive -fno-exceptions -fno-threadsafe-statics \
        -ffunction-sections -fdata-sections \
        $CORE_DEFINES $CORE_INCLUDES -c "$src" -o "$obj"
done

echo "=== compiling sketch ==="
avr-g++ -mmcu=$MCU -Os -std=gnu++11 -fpermissive -fno-exceptions -fno-threadsafe-statics \
    -ffunction-sections -fdata-sections \
    $CORE_DEFINES $CORE_INCLUDES -c cpp/sketch.cpp -o "$BUILD_DIR/sketch.o"

echo "=== building Rust logic crate ==="
(
    cd rust
    RUSTFLAGS="-C target-cpu=$MCU" cargo build --release -Z build-std=core --target avr-none
)

echo "=== linking ==="
avr-g++ -mmcu=$MCU -Os -Wl,--gc-sections \
    -o "$BUILD_DIR/firmware.elf" \
    "$BUILD_DIR"/core/*.o "$BUILD_DIR/sketch.o" \
    -Lrust/target/avr-none/release -lcitadel_logic

avr-objcopy -O ihex -R .eeprom "$BUILD_DIR/firmware.elf" "$BUILD_DIR/firmware.hex"

echo "=== avr-size ==="
avr-size "$BUILD_DIR/firmware.elf"

echo "=== citadel_* symbols (link/ABI interop check) ==="
avr-nm "$BUILD_DIR/firmware.elf" | grep -i citadel

echo "=== undefined symbols (should be empty) ==="
avr-nm "$BUILD_DIR/firmware.elf" | grep " U " || echo "(none)"

echo "=== output ==="
echo "$BUILD_DIR/firmware.hex"
```

- [ ] **Step 3: Make it executable**

```bash
chmod +x /home/gooya/citadel/prototypes/0002-arduino-core/build.sh
```

- [ ] **Step 4: Run the build and verify it succeeds**

```bash
cd /home/gooya/citadel/prototypes/0002-arduino-core
./build.sh
```

Expected: script completes without error, prints `build/firmware.hex` at the end, and the "undefined symbols" section prints `(none)` (i.e. `citadel_tick` and all core-internal references resolve).

- [ ] **Step 5: Commit**

```bash
cd /home/gooya/citadel
git add prototypes/0002-arduino-core/cpp/sketch.cpp prototypes/0002-arduino-core/build.sh
git commit -m "$(cat <<'EOF'
Add sketch and build script for the 0002 prototype

Compiles the whole vendored ArduinoCore-avr cores/arduino/ directory
(same approach arduino-builder uses) and links it with the sketch and
the Rust logic crate, relying on --gc-sections to drop unused core
code.
EOF
)"
```

---

### Task 4: Prototype README (procedure, pre-hardware-verification)

**Files:**
- Create: `prototypes/0002-arduino-core/README.md`

**Interfaces:**
- Consumes: nothing beyond the file layout established in Tasks 1-3.
- Produces: a README section named exactly `## 実機での検証手順(手動)`, which Task 5 replaces with `## 実機での検証結果` once hardware verification is done (matches the pattern already used in `prototypes/0001-hello-blink/README.md`).

- [ ] **Step 1: Write the README**

`prototypes/0002-arduino-core/README.md`:

```markdown
# プロトタイプ拡張: 本物のArduinoコアを使ったhello-blink

[`prototypes/0001-hello-blink`](../0001-hello-blink) は `avr/io.h` のレジスタ直叩きでRust/CのABI境界を検証した(RFC 0001 §5、resolved済み)。このプロトタイプは、本物の `ArduinoCore-avr`(`Arduino.h`/`pinMode`/`digitalWrite`)を使っても同じ境界ルールが成立することを検証する、独立した検証タスク。

## 構成

- `vendor/ArduinoCore-avr/` — [arduino/ArduinoCore-avr](https://github.com/arduino/ArduinoCore-avr) をgit submoduleとしてタグ `1.8.8` に固定。`cores/arduino/`(コア本体)と `variants/standard/`(Uno/Nano/Pro Mini系のピン配置)を使用。
- `cpp/sketch.cpp` — ユーザースケッチ相当。`setup()`/`loop()` は `pinMode`/`digitalWrite`/`delay` の直線呼び出しのみで、if/for/whileや計算用の中間変数を持たない。
- `rust/` — ロジック層。`#![no_std]` staticlib。`citadel_tick()` がLED状態(0/1)をトグルする。0001と違い、点滅のタイミングはRust側のカウンタではなく `sketch.cpp` の `delay(500)` が担う。
- `build.sh` — `cores/arduino/` 配下の全ソースをコンパイル(`arduino-builder`と同じ方式)→ スケッチをコンパイル → `cargo +nightly` でRustをビルド → リンク(`-Wl,--gc-sections` で未使用コアシンボルを刈る)→ `.hex` 生成、を一括実行する。

## 実機での検証手順(手動)

```sh
git submodule update --init vendor/ArduinoCore-avr
./build.sh
avrdude -c arduino -p atmega328p -P /dev/ttyACM0 -b 115200 -U flash:w:build/firmware.hex:i
```

`-P` はシリアルポートに合わせて変更する。書き込み後、13番ピン(オンボードLED)が一定間隔で点滅すれば成功。

## ライセンスについて

`ArduinoCore-avr` はLGPL-2.1。Arduino公式のFAQによれば、スケッチ+コアをリンクした成果物はLGPLの再配布義務を負わない。Citadel本体のGPL/Apacheライセンス表記([トップレベルREADME](../../README.md#licensing)参照)とは別枠の注記。
```

- [ ] **Step 2: Commit**

```bash
cd /home/gooya/citadel
git add prototypes/0002-arduino-core/README.md
git commit -m "Add README for the 0002 Arduino core vendoring prototype"
```

---

### Task 5: Hardware verification and cross-references

**Files:**
- Modify: `prototypes/0002-arduino-core/README.md`
- Modify: `prototypes/0001-hello-blink/README.md`

**Interfaces:**
- Consumes: `build/firmware.hex` from Task 3; the ELEGOO UNO R3 connected at `/dev/ttyACM0`.
- Produces: nothing consumed by later tasks (this is the final task).

- [ ] **Step 1: Confirm the board is connected**

```bash
ls -la /dev/ttyACM0
```

Expected: device present (same ELEGOO UNO R3 already used for 0001).

- [ ] **Step 2: Flash the firmware**

```bash
cd /home/gooya/citadel/prototypes/0002-arduino-core
avrdude -c arduino -p atmega328p -P /dev/ttyACM0 -b 115200 -U flash:w:build/firmware.hex:i
```

Expected: `avrdude done. Thank you.` with the verify step reporting matching bytes and no mismatch.

- [ ] **Step 3: Confirm the LED blinks**

Ask the user to visually confirm the onboard LED (pin 13) blinks at roughly 1 Hz (500ms on/off, driven by `delay(500)`). Do not proceed to Step 4 until confirmed.

- [ ] **Step 4: Record avr-size and compute the delta vs 0001**

```bash
avr-size /home/gooya/citadel/prototypes/0002-arduino-core/build/firmware.elf
```

Compare the reported `text` value against 0001's recorded baseline of `text=274 bytes` (from `prototypes/0001-hello-blink/README.md`). Note both numbers for Step 5.

- [ ] **Step 5: Update the 0002 README with verification results**

In `prototypes/0002-arduino-core/README.md`, replace the `## 実機での検証手順(手動)` section:

```markdown
## 実機での検証結果

ELEGOO UNO R3(Arduino Uno互換ボード)で実機検証済み。

```sh
git submodule update --init vendor/ArduinoCore-avr
./build.sh
avrdude -c arduino -p atmega328p -P /dev/ttyACM0 -b 115200 -U flash:w:build/firmware.hex:i
```

- `avrdude` による書き込み・ベリファイ成功
- 13番ピン(オンボードLED)の点滅を目視確認済み
- `avr-size` 実測値: text=<Step 4で得た値> bytes(0001のtext=274 bytesとの差分がArduinoコア分のコスト)

`-P` はシリアルポートに合わせて変更する。
```

Fill in `<Step 4で得た値>` with the actual number measured in Step 4 — do not leave it as a placeholder.

- [ ] **Step 6: Update 0001's README to point at this prototype**

In `prototypes/0001-hello-blink/README.md`, under `## このプロトタイプが実証していないこと`, replace:

```markdown
- 本物のArduinoコア(`Arduino.h`/`pinMode`/`digitalWrite`)は使っていない。`avr/io.h` によるレジスタ直叩き。Arduinoコアのベンダリングは別タスク。
```

with:

```markdown
- 本物のArduinoコア(`Arduino.h`/`pinMode`/`digitalWrite`)は[`prototypes/0002-arduino-core`](../0002-arduino-core)で別途検証済み。このプロトタイプ自体は意図的に`avr/io.h`直叩きのまま(ABI境界の最小検証に集中するため)。
```

- [ ] **Step 7: Commit**

```bash
cd /home/gooya/citadel
git add prototypes/0002-arduino-core/README.md prototypes/0001-hello-blink/README.md
git commit -m "$(cat <<'EOF'
Record ELEGOO UNO R3 hardware verification for the 0002 prototype

avrdude write/verify succeeded and the onboard LED blink was visually
confirmed, closing the "real Arduino core" gap 0001's README flagged
as unproven.
EOF
)"
git push
```
