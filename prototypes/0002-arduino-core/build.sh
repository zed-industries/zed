#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

MCU=atmega328p
F_CPU=16000000L
BUILD_DIR=build
CORE_DIR=vendor/ArduinoCore-avr/cores/arduino
VARIANT_DIR=vendor/ArduinoCore-avr/variants/standard

# ARDUINO=10808 keep in sync with the submodule tag (1.8.8)
CORE_DEFINES="-DF_CPU=$F_CPU -DARDUINO=10808 -DARDUINO_AVR_UNO -DARDUINO_ARCH_AVR"
CORE_INCLUDES="-I$CORE_DIR -I$VARIANT_DIR"

rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/core"

echo "=== compiling ArduinoCore-avr (cores/arduino) ==="
for src in "$CORE_DIR"/*.c; do
    obj="$BUILD_DIR/core/$(basename "$src").o"
    avr-gcc -mmcu=$MCU -Os -w -std=gnu11 -ffunction-sections -fdata-sections \
        $CORE_DEFINES $CORE_INCLUDES -c "$src" -o "$obj"
done
for src in "$CORE_DIR"/*.cpp; do
    obj="$BUILD_DIR/core/$(basename "$src").o"
    avr-g++ -mmcu=$MCU -Os -w -std=gnu++11 -fpermissive -fno-exceptions -fno-threadsafe-statics \
        -ffunction-sections -fdata-sections \
        $CORE_DEFINES $CORE_INCLUDES -c "$src" -o "$obj"
done
for src in "$CORE_DIR"/*.S; do
    obj="$BUILD_DIR/core/$(basename "$src").o"
    avr-gcc -mmcu=$MCU -x assembler-with-cpp -w -ffunction-sections -fdata-sections \
        $CORE_DEFINES $CORE_INCLUDES -c "$src" -o "$obj"
done

echo "=== archiving core objects ==="
avr-ar rcs "$BUILD_DIR/core.a" "$BUILD_DIR"/core/*.o

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
    "$BUILD_DIR/sketch.o" "$BUILD_DIR/core.a" \
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
