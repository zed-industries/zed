# Design: Arduino core vendoring prototype (prototypes/0002-arduino-core)

Status: approved, ready for planning.

## Background

[`prototypes/0001-hello-blink`](../../../prototypes/0001-hello-blink) proved the Rust/C ABI boundary (RFC 0001 §5, now resolved) but did so by writing directly to `avr/io.h` registers, not through the real Arduino core (`Arduino.h`/`pinMode`/`digitalWrite`). That prototype's README explicitly lists this as unproven. This design covers a new, separate prototype that closes that gap.

This is not a new RFC 0001 numbered item — RFC 0001 is closed. This is a standalone verification task referenced from `prototypes/0001-hello-blink/README.md`'s "not yet proven" section.

## Goal / scope

Build `prototypes/0002-arduino-core`: a minimal blink sketch that uses the real `ArduinoCore-avr` (`Arduino.h`, `pinMode`, `digitalWrite`) instead of raw register access, with the same Rust/C boundary rules as 0001 (C++ side is straight-line I/O only, all state/logic lives in the `no_std` Rust crate). Verified end-to-end on the same hardware already confirmed working: ELEGOO UNO R3 (Arduino Uno-compatible, ATmega328P).

Out of scope: other boards/variants, Serial/other peripherals, IDE integration, static-analysis lint (RFC 0001 item 4).

## Repository layout

```
prototypes/0002-arduino-core/
├── README.md
├── build.sh
├── rust-toolchain.toml          # same pinned nightly as 0001
├── vendor/
│   └── ArduinoCore-avr/         # git submodule: arduino/ArduinoCore-avr, pinned to a release tag
├── cpp/
│   └── sketch.cpp                # setup()/loop(), matches README's example shape
└── rust/
    ├── Cargo.toml
    └── src/lib.rs                 # citadel_tick() etc., carried over from 0001
```

- The `ArduinoCore-avr` submodule is pinned to a specific release tag (not tracking `main`), for the same reproducibility reason 0001 pins a specific nightly.
- `main()` ownership belongs entirely to `vendor/ArduinoCore-avr/cores/arduino/main.cpp`. There is no `cpp/runtime.cpp` in this prototype (unlike 0001) — the real core's `main.cpp` already calls `init()` once, `setup()` once, then loops `loop()` forever.
- Only the `standard` variant (Uno/Nano/Pro Mini family) is wired up, matching the already-verified hardware.

## Build integration

`build.sh` compiles the entire `cores/arduino/` directory from the submodule (mirroring how `arduino-builder`/Arduino IDE builds `core.a` — compile everything, let the linker drop what's unused) rather than hand-picking a subset of core files:

```sh
CORE_DIR=vendor/ArduinoCore-avr/cores/arduino
VARIANT_DIR=vendor/ArduinoCore-avr/variants/standard

# Compile the whole core (.c and .cpp separately — different compiler invocations)
avr-gcc  -mmcu=$MCU -Os -ffunction-sections -fdata-sections -I"$CORE_DIR" -I"$VARIANT_DIR" -DARDUINO=<core version> -c "$CORE_DIR"/*.c   -o build/core/ (one .o per file)
avr-g++  -mmcu=$MCU -Os -ffunction-sections -fdata-sections -I"$CORE_DIR" -I"$VARIANT_DIR" -DARDUINO=<core version> -c "$CORE_DIR"/*.cpp -o build/core/ (one .o per file)

# Sketch
avr-g++ -mmcu=$MCU -Os -ffunction-sections -fdata-sections -I"$CORE_DIR" -I"$VARIANT_DIR" -c cpp/sketch.cpp -o build/sketch.o

# Rust logic (same as 0001)
( cd rust && RUSTFLAGS="-C target-cpu=$MCU" cargo build --release -Z build-std=core --target avr-none )

# Link — -Wl,--gc-sections drops the core symbols the sketch never references
avr-g++ -mmcu=$MCU -Os -Wl,--gc-sections \
    -o build/firmware.elf build/core/*.o build/sketch.o \
    -Lrust/target/avr-none/release -lcitadel_logic

avr-objcopy -O ihex -R .eeprom build/firmware.elf build/firmware.hex
```

The `-DARDUINO=...` version macro is set from the pinned submodule tag.

Rejected alternatives:
- **Delegate the whole build to `arduino-cli`/PlatformIO.** Would offload core-build complexity, but introduces a new heavyweight external tool dependency this project doesn't otherwise use, and it's unclear how to cleanly inject the `extern "C"` Rust staticlib into that tool's build graph. The existing prototypes call `avr-g++`/`avrdude` directly; this would break that pattern.
- **Hand-pick only the core files a blink sketch needs.** Fragile (easy to miss a file when a sketch later needs e.g. `Serial`), and `--gc-sections` already makes "compile everything, link only what's used" free.

## Sketch layer

`cpp/sketch.cpp`, matching the shape already documented in the top-level README's architecture example:

```cpp
#include <Arduino.h>

const int LED_PIN = LED_BUILTIN;

extern "C" uint8_t citadel_tick(); // logic lives in Rust, carried over from 0001

void setup() {
    pinMode(LED_PIN, OUTPUT);
}

void loop() {
    digitalWrite(LED_PIN, citadel_tick());
    delay(500); // core-provided delay; no branch, no computed intermediate — boundary rule holds
}
```

No `if`/`for`/`while`, no computed intermediate variables — consistent with the C/C++ boundary rule in `CLAUDE.md` and the top-level README.

## Verification plan

- `avr-nm` on the linked `.elf`: confirm `citadel_tick` resolves with zero undefined (`U`) symbols, same check as 0001.
- Flash to the ELEGOO UNO R3 via `avrdude` (`-c arduino -p atmega328p -P /dev/ttyACM0 -b 115200`), confirm write+verify succeeds.
- Visually confirm the onboard LED (pin 13) blinks.
- Record `avr-size` output and compare against 0001's `text=274 bytes` baseline — the delta is the real cost of the Arduino core, a useful data point for RFC 0001 §3 (binary size), even though that item isn't reopened by this work.

## Licensing note

`ArduinoCore-avr` is LGPL-2.1. Arduino's official FAQ position is that sketches linked against the core are not subject to LGPL's redistribution obligations, so this doesn't affect Citadel's own GPL/Apache licensing story. `prototypes/0002-arduino-core/README.md` should note this briefly, separate from the top-level license section.

## Definition of done

- `./build.sh` produces `build/firmware.hex` linking submodule core objects, the sketch, and the Rust staticlib with zero undefined symbols.
- Firmware flashed to and verified running (LED blink confirmed) on the ELEGOO UNO R3.
- `prototypes/0001-hello-blink/README.md`'s "not yet proven" bullet about the real Arduino core is updated to point at 0002 as resolving it.
