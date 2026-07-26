# RFC 0004: Build identity on the device

Status: **draft**. This RFC defines how a developer answers "which build is on this board?" without guessing. It extends the Git integration in [RFC 0002](./0002-product-scope-and-dx.md#git-and-github-integration) from source management to the flashed artifact, and it deliberately does *not* extend the debug channel in [RFC 0003](./0003-debugging-without-stopping-the-chip.md) — the reasoning for that separation is below.

## What this is not

The idea that motivated this RFC was broader: embed the Git commit hash, use it to catch corruption during flashing, and have the device re-hash its own flash at boot to detect bit rot. Two thirds of that does not survive contact with the tools, and recording why keeps it from being re-proposed.

**Transfer corruption is already solved.** `avrdude` reads flash back and compares it byte-for-byte against the `.hex` after every write; `-V` exists precisely to turn that off. A single flipped bit anywhere in the image fails the write with a verification error, today, with no Citadel involvement. Adding a hash check on top detects nothing the existing check misses.

**A commit hash is not an image hash.** It identifies a source tree, not a build artifact. The same commit built with a different `avr-gcc`, a different path prefix, or a different pinned nightly ([RFC 0001 §1](./0001-hybrid-architecture.md#1-rusts-avr-target-is-nightly-only)) produces a different `.hex`. So a device cannot hash its own flash and compare the result to a commit hash — those values are not comparable, and no amount of care on the device side makes them so. Integrity and identity are two separate values with two separate mechanisms, and only identity is worth what it costs here.

Boot-time flash integrity checking is therefore out of scope. Not rejected as unsound — it is a real technique — but it requires a post-link CRC patch, a whole-flash scan on every boot, and a design for what the device should *do* on mismatch, all to catch a failure mode that AVR flash retention makes rare and that a reflash resolves anyway.

## Design

The marker is inert data. No firmware, Rust or C++, reads it, refers to it, or knows it exists.

```
[ build   ] append a marker to the linked image, after link, before flashing
[ flash   ] avrdude writes it like any other bytes; its own verify pass covers it
[ enquiry ] the IDE reads flash back and scans for the marker
```

Because nothing on the device participates, this is unaffected by whether the build is debug or release, whether `citadel_runtime_tick()` is present in the sketch (RFC 0003's open question), or whether the firmware is currently running, crashed, or in a boot loop. Those were the reasons to reject the obvious alternative of having the firmware answer over the serial link: the boards whose identity is actually unknown are disproportionately the ones running a release build or not running at all.

### Marker format

ASCII, starting at the first flash page boundary after the program image and NUL-padded to fill whole pages, so the marker never shares a page with program code:

```
CITADELv1 <commit-sha-hex> <tree-sha-hex>
```

ASCII rather than packed binary — 91 bytes instead of 49 — so `strings` on the image or an eyeballed readback dump answers the question with no Citadel and no decoder. On a 32 KB ATmega328P one 128-byte page is 0.4% of flash.

`<tree-sha>` is the tree object for the working tree at the moment of flashing, not `HEAD`'s tree. This makes the dirty case first-class instead of a flag: when the two hashes agree the build was clean, and when they disagree the tree hash distinguishes builds that a `-dirty` suffix would collapse into one indistinguishable string. Since flashing an uncommitted working tree is the normal case during development, a scheme that cannot tell those apart answers the question this RFC exists to answer only rarely.

Placement is derived from the linked ELF, not fixed. Reserving an address near the top of flash would mean knowing the bootloader size, which varies by board and depends on the unresolved board-identification question in [RFC 0002](./0002-product-scope-and-dx.md#board-and-toolchain-detection). Appending after the program needs only the image's end address, and the reader does not need the address at all — it scans. The build fails if the marker would not fit in the application region.

The marker is appended as extra Intel HEX records after link, not declared as a `const` in C++. Arduino's build uses `-fdata-sections -Wl,--gc-sections`, which collects an unreferenced constant; `__attribute__((used))` stops the compiler from dropping it but not the linker, and the reliable fix is a `KEEP()` in the linker script, which means editing the vendored Arduino core that RFC 0002 puts out of scope. Post-link, none of this applies: no linker involvement, no toolchain version sensitivity, and nothing that touches the Rust/C boundary, because the marker is neither Rust nor C.

### Producing the tree hash

The working tree is hashed through a throwaway index so the user's own index is never touched:

```
GIT_INDEX_FILE=<temp> git add -A && git write-tree
```

`-A` includes untracked-but-not-ignored files, which affect the build and therefore belong in its identity.

### Keeping dirty trees resolvable

A tree object written this way is unreachable, so `git gc` prunes it — typically within two weeks. After that the device names a hash whose contents no longer exist, which is worse than not recording it, because it looks like an answer.

Dirty builds therefore get a commit pinning the tree:

```
git commit-tree <tree> -p HEAD -m "flashed <board> at <time>"
git update-ref refs/citadel/flashed/<tree> <commit>
```

Same cost as pinning the bare tree, but it also records when the build was flashed and which commit it sat on top of, and naming the ref after the tree hash means the value the device reports resolves directly. Re-flashing an unchanged dirty tree writes the same ref, so refs accumulate per distinct tree rather than per flash.

Clean builds write nothing: `HEAD` already pins the tree.

`refs/citadel/*` sits outside `refs/heads` and `refs/tags`, so it is not pushed, not fetched, and does not appear in `git log` or the branch list. Nothing the user sees changes.

### Reading it back

Readback is an explicit action ("what is on this board?"), never automatic. It costs seconds and resets the device, and the IDE already knows what it flashed in the current session — the case that needs answering is the unlabeled board on the desk, which is a deliberate act, not a side effect of plugging in. Making it automatic would require a local ledger keyed by USB serial number, and a ledger that can drift out of sync with reality is the wrong foundation for a feature whose entire purpose is telling the truth about what is on the chip.

The IDE scans the readback for the magic string and resolves what it finds against the repository: commit subject, clean or dirty, and for dirty builds the pinned ref. Two matches means an ambiguous image and is reported as such rather than resolved by picking one; nine ASCII bytes appearing by chance in AVR machine code is not a case worth handling silently.

## Known limits

- The marker identifies **sources, not the image**. Two builds of the same tree with different toolchain versions carry the same marker and are not the same binary. Making the marker imply the image requires reproducible builds, which is a separate problem and not one this RFC needs solved.
- Readback needs the programmer connected. On a bootloader board that is the same USB cable, but an ISP-only setup may not have the wiring present when the question is asked.

## Open items

- **SHA-256 repositories.** Git's SHA-256 object format makes the payload 139 bytes, which exceeds the 128-byte page of an ATmega328P and would need two pages. Not a blocker — needs a decision on whether to support it at all, given no Citadel project has a reason to opt in.
- **Local-only refs.** A dirty build's tree resolves on the machine that flashed it and nowhere else, because `refs/citadel/*` is not pushed. Sharing a board between two developers therefore reproduces the original problem in a smaller form. Whether that is acceptable, or whether the refs should be pushable, needs a decision.
- **The commit-tree message.** It records board and time, which makes it non-deterministic: the same tree flashed twice to different boards yields the same ref pointing at the first commit, silently discarding the second flash's context. Either the message drops the varying parts or the ref scheme accounts for them.

## Definition of done for this RFC

Resolved when each open item above has a recorded decision and a marker has survived a full round trip on real hardware — appended after link, written by `avrdude`, read back, and resolved against the repository — for both a clean and a dirty build.
