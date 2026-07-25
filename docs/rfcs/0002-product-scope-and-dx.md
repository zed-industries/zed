# RFC 0002: Product scope and developer experience

Status: **draft**. This RFC defines what Citadel is for, what it does and does not cover, and the developer-experience commitments that follow. It deliberately does not re-derive the Rust/C boundary — see the [README](../../README.md#architecture-the-rustc-boundary) for the boundary rules and [RFC 0001](./0001-hybrid-architecture.md) for the unresolved technical items that gate implementation.

## Mission

Remove the unreasonable stress from embedded development — sluggish IDEs, on-device freezes with no traceable cause, C++ memory corruption — so the failure modes that remain are the developer's own logic, not the tooling's.

Stated as a constraint rather than a slogan: *if it compiles, it runs safely; never freeze the system, never lock the chip.*

### Why this matters more now, not less

LLMs generate firmware faster than anyone can review it. The bottleneck moves from "writing code" to "trusting code you didn't write line by line." Citadel's answer is not to review harder but to make the dangerous shapes unrepresentable: logic lives in Rust where the compiler checks it, and the C/C++ side is mechanically restricted to straight-line I/O (RFC 0001 §4). A generator that emits a branch in C/C++ gets a build error, not a warning — the guardrail holds regardless of who or what typed the code.

## Scope

In scope:

- Arduino/AVR firmware authoring, building, and flashing, for projects created by Citadel *and* existing Arduino sketches adopted into it.
- Enforcement of the Rust/C boundary at build time.
- Project scaffolding, board detection, and Git/GitHub integration (below).
- Process isolation and recovery so IDE liveness is independent of build and serial-port state.

Out of scope for now, listed so it stays a decision rather than an omission:

- Non-AVR targets (ARM/ESP). The boundary rules are target-independent, but the toolchain work in RFC 0001 is AVR-specific; second targets come after the first one is proven on hardware.
- Debugging beyond serial (no JTAG/SWD, no on-chip debugger UI).
- Rewriting the vendored Arduino core or existing C++ libraries. Third-party C++ libraries stay usable as-is; they are outside the boundary check by design.

## Project scaffolding

`citadel init` (and the IDE's "new project" flow — same code path) produces a project that is ready to build and ready to commit, with no follow-up setup:

```
my-citadel-project/
├── .git/                 # git init + initial commit already done
├── .gitignore            # target/, build artifacts, serial logs
├── .claude/
│   ├── CLAUDE.md         # Citadel boundary rules, as AI-facing instructions
│   └── skills/           # user's own skills
├── docs/                 # schematics, pin assignment notes
├── rust/                 # logic layer — no_std crate
│   ├── Cargo.toml
│   └── src/lib.rs
└── cpp/                  # I/O layer — straight-line hand-off only
    └── io.cpp
```

The `rust/` + `cpp/` split is not cosmetic: it is what makes the boundary check cheap and unambiguous. The checker's scope is "the sketch files in `cpp/`", never vendored libraries — the open scope/macro questions are RFC 0001 §4.

`.claude/CLAUDE.md` is generated with the boundary rules stated as constraints on the generator ("no logic, no pointer arithmetic, no state in C/C++; put it in Rust"). It is a convenience, not a control: the build-time check is the actual enforcement, and the generated file is the user's to edit. Treating a prompt file as the guardrail would be exactly the mistake this architecture exists to avoid.

## Board and toolchain detection

Choosing a part number from a dropdown before writing any code (the Microchip Studio flow) is a decision users are not yet equipped to make. Instead: on connect, read the device signature in the background, identify the chip, and set the target and clock accordingly, surfacing what it picked so a wrong guess is visible and overridable.

- **Open:** the signature only identifies the chip, not the board. ATmega328P is Uno, Nano, and Pro Mini, which differ in clock and bootloader baud. Needs a decision on how to resolve that ambiguity — probe the bootloader, ask once and remember per USB VID/PID, or default to Uno.
- **Open:** interaction with the pinned Rust nightly (RFC 0001 §1) — detection can select a target the pinned toolchain wasn't smoke-tested against.

## Git and GitHub integration

- `citadel init` creates the local repository and the initial commit. With `gh` available and authenticated, it can also create the remote and push. Repository creation is outward-facing and irreversible enough that it is opt-in per project, not implied by `init`.
- On close, offer a commit: a diff-derived message draft plus "commit this session's work?". The point is not tidy history, it is that every working state is recoverable — "it worked five minutes ago" should be a checkout, not a memory.
- **Open:** whether the message draft is generated locally or by a model, and what happens when no model is configured. A diff-derived draft (files touched, added/removed) works with no model at all and is the fallback.

## Failsafe behavior

Three independent layers, so no single stuck component takes the others down:

1. **UI liveness.** The editor never blocks on build, upload, or serial I/O. Zed's GPUI foundation gives fast startup and rendering; the commitment here is that background work stays background — a hung `avrdude` must not be able to stall a keystroke.
2. **Process isolation.** Toolchain and serial-port ownership live in separate processes. An abnormal IDE exit cannot leave a port held by a dead editor, and a crashed backend is restartable without restarting the editor.
3. **Device recovery.** A watchdog path so a device stuck in a bad state can be recovered from the IDE rather than by physically power-cycling it.

- **Open:** layer 3 needs a design. Automatic WDT insertion into user firmware is a behavior change to code the user wrote, which cuts against "the user's logic is the user's"; the alternative is an opt-in WDT and an explicit rescue command. Needs a decision before implementation, and it is not covered by RFC 0001's items.

## Definition of done for this RFC

This RFC is descriptive of intent and stays open until each "Open" item above has a recorded decision. It does not gate implementation the way RFC 0001 does — the scaffolding and Git work can proceed independently of the toolchain items — but detection and failsafe work should not start before their open questions are answered.
