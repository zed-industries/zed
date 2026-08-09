#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

MCU=atmega328p
BUILD_DIR=build

mkdir -p "$BUILD_DIR"

avr-g++ -mmcu=$MCU -Os -ffunction-sections -fdata-sections -c cpp/runtime.cpp -o "$BUILD_DIR/runtime.o"
avr-g++ -mmcu=$MCU -Os -ffunction-sections -fdata-sections -c cpp/io.cpp -o "$BUILD_DIR/io.o"

(
    cd rust
    RUSTFLAGS="-C target-cpu=$MCU" cargo build --release -Z build-std=core --target avr-none
)

avr-g++ -mmcu=$MCU -Os -Wl,--gc-sections \
    -o "$BUILD_DIR/firmware.elf" \
    "$BUILD_DIR/runtime.o" "$BUILD_DIR/io.o" \
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
