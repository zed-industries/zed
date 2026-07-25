# RFC 0003: Debugging without stopping the chip

Status: **draft**. This RFC defines how Citadel exposes the running state of firmware to the developer. It refines the "debugging beyond serial is out of scope" line in [RFC 0002](./0002-product-scope-and-dx.md#scope) from an omission into a decision, and it inherits the boundary rules from the [README](../../README.md#architecture-the-rustc-boundary) and [RFC 0001](./0001-hybrid-architecture.md).

## Invariant

**Nothing Citadel does to observe a running device may put that device into a state the developer cannot get out of from the IDE.** Not on abnormal exit, not on a yanked USB cable, not on an IDE crash mid-session. This is a hard constraint on the debug feature, in the same class as "never lock the chip" in RFC 0002 — a debug feature that can brick a board is worse than no debug feature, because the developer now distrusts the tool as well as their own code.

## Why variable visibility is still required

Removing crashes does not remove logic errors. A swapped `>`/`<`, an off-by-one in a state transition, a sensor reading that isn't the unit the developer assumed — the compiler cannot catch these, and neither can a model reading the source, because the bug is a mismatch between the developer's mental model and the physical world. Closing that gap requires seeing the actual value at the actual moment. Without it the developer is reduced to bisecting by `Serial.print`, which is exactly the low-grade misery Citadel exists to remove.

So: variable observation is not optional. What is optional is stopping the CPU to get it.

## Why on-chip debugging is rejected on AVR

The proposal to keep a conventional stepping debugger as an "advanced users only" option does not survive contact with classic AVR hardware:

- **`probe-rs` does not support AVR.** It targets ARM and RISC-V. It is not an available implementation path here regardless of policy.
- **debugWIRE's entry condition *is* the lockout.** Enabling it means programming the `DWEN` fuse, and with `DWEN` set the ISP/SPI programming interface is disabled. Leaving that state cleanly requires a debugWIRE-capable debugger (Atmel-ICE, PICkit/Snap) to temporarily hand control back to ISP and clear the fuse. If the session dies first — crash, cable, closed lid — a plain ISP programmer cannot recover the chip and the fallback is high-voltage programming. That is the failure mode the invariant above forbids, and it is not caused by abnormal exit; it is inherent to entering the mode at all.
- On an Arduino Uno it is additionally impractical: debugWIRE takes over the `RESET` line, which the board wires to an auto-reset capacitor and the onboard USB-serial bridge.

The conclusion is not "make hardware stepping safer." It is that on classic AVR, hardware stepping and the invariant are mutually exclusive, so Citadel does not ship it. Newer AVRs with UPDI (ATtiny 1-series, ATmega4809) do not have the fuse trap and could revisit this — but they are out of scope per RFC 0002 until the first target is proven.

## Live Inspector

The device is never halted. Rust logic emits the values it was already computing, the IDE renders them as a live numeric/graph panel, and the firmware keeps running whether or not anyone is listening.

### What the developer writes

```rust
citadel_watch!(state, sensor_value);
```

A macro in the Rust logic crate, at the point where the value is meaningful. It expands to a non-blocking enqueue and nothing else. In a release build it expands to nothing at all — no flash cost, no timing change, no way to ship a debug channel by accident.

Placing it in Rust rather than C/C++ is not a stylistic preference: the values worth watching are state and computed results, and by the boundary rules those only exist on the Rust side.

### Who owns the UART

This is the part that is not obvious. The boundary permits C/C++ to call `extern "C"` Rust functions and permits Rust to *read* `extern "C"` data from C/C++ — it does not permit Rust to call back into C++. So `citadel_watch!` cannot call `Serial.write`, and having Rust drive the USART registers directly would fight the Arduino core's `HardwareSerial` for the same peripheral.

The resolution: `citadel_watch!` writes into a static ring buffer in Rust, and a Citadel-provided runtime shim on the C++ side drains it into `Serial`. The shim is vendored, not user sketch code, so it sits outside the boundary checker's scope for the same reason third-party libraries do (RFC 0001 §4) — the drain loop is allowed to be a loop because it is not the user's logic. The user's sketch gains exactly one straight-line call, which the checker accepts unchanged:

```cpp
citadel_runtime_tick();
```

- **Open:** whether the shim is inserted into the sketch by scaffolding (visible, editable, the user can delete it) or called from the vendored core behind the user's back (invisible, cannot be forgotten). Visible-and-forgettable is the safer default under "the user's logic is the user's", but a watch panel that silently shows nothing because a line was deleted is a bad first experience.

### Non-blocking, and what gets dropped

The ring buffer is fixed-size and drops on overflow. It never blocks the caller. A blocking write on a full TX buffer would make the watch macro capable of stalling the developer's control loop, which would turn the debug feature into the freeze the whole project exists to prevent.

The consequence is honest and must be surfaced in the UI, not hidden: at 115200 baud the channel carries roughly 11 KB/s, so a watch in a tight loop will drop samples. The IDE shows the drop count rather than pretending the stream is complete — a silently decimated graph is worse than a visibly gappy one.

- **Open:** sampling policy. Options are drop-newest, drop-oldest, or a rate cap in the macro. Rate-capping at the source is the cheapest and keeps flash cost down, but "every transition of this enum" is a legitimate thing to want and a rate cap loses exactly those.

### Framing

The watch stream shares one UART with whatever the developer prints themselves, so the frames must be self-synchronizing and distinguishable from plain text. A byte-stuffed frame (COBS, zero-delimited) with a fixed start byte is enough; the IDE routes bytes inside a frame to the watch panel and everything else through to the ordinary serial monitor unchanged. `Serial.print` keeps working exactly as it does today, in the same window.

- **Open:** payload encoding. Binary values plus a symbol table emitted at build time keeps the wire cheap; self-describing JSON costs flash and bandwidth on the constrained side of the link to save work on the side that has a whole toolchain available. Binary is the intended answer, pending the build-time symbol table being cheap to produce.

### Behavior on disconnect

Unplugging mid-session does nothing to the device: the ring buffer fills, drops, and the firmware carries on. Recovery on the IDE side is reopening a serial port. There is no device-side state to reconcile because the debug channel never owned any — which is the entire argument for this design over the previous section's.

## Step execution, for the cases that need it

Single-stepping is genuinely the right tool for some bugs, and the answer is a simulator, not the chip. `simavr` executes an AVR `.hex` under `avr-gdb`, with real breakpoints and real stepping, at zero risk to any hardware — there is no fuse to set because there is no chip. It also debugs the exact artifact that would have been flashed.

The limit is inherent and should be stated in the UI rather than discovered: a simulator does not have the developer's sensor, motor, or timing. It is the right tool for logic bugs in the Rust layer and the wrong one for anything involving the physical world, which is precisely the split where Live Inspector takes over.

- **Open:** whether this ships at all in the first version. It is strictly optional — Live Inspector covers the common case — and it pulls in a `simavr` dependency plus DAP wiring. Deferring it is reasonable; committing to hardware stepping later is not.

## Definition of done for this RFC

Resolved when each "Open" item above has a recorded decision and a Live Inspector prototype has run against real hardware long enough to measure its flash cost and its effect on loop timing. The invariant is not subject to the same process: it is a constraint on any future debug work in Citadel, not an item to be closed.
