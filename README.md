# Citadel

*[日本語](./README.ja.md)*

**Citadel** (aka *Citadel-Duino*) is a fork of [Zed](https://github.com/zed-industries/zed), being reworked into a next-generation IDE dedicated to Arduino (AVR) embedded development.

> This is an independent, non-commercial fork. It is not affiliated with, endorsed by, or sponsored by Zed Industries, Inc. "Zed" is a trademark of Zed Industries, Inc.

### Philosophy

> "If it compiles, it runs safely. Never freeze the system, never lock the chip."

### Why Citadel exists

| 怒り (Frustration) | 対策 (Countermeasure) |
|---|---|
| IDEがクソ重くて作業効率が下がる | ZedのGPU超高速描画（GPUI）とRust製の爆速コアで解決 |
| ビルドが通っても実機でフリーズして原因不明 | コアロジックをRust (no_std) で実装し、メモリ破壊をコンパイル時に排除 |
| 不正終了でIDEが固まり、マイコンのポートも掴みっぱなしで詰む | バックエンドを完全別プロセス化。自動WDT挿入＆レスキューシーケンスで物理・論理の両面からフェイルセーフ |
| 微妙なコードの変更をGitで綺麗に管理・追跡しにくい | Zed標準のインラインGit連携とブランチ連動型ビルドで進捗を地続きに |

Full architecture, scope, and roadmap: see the project RFC (in progress).

### Status

Early-stage fork — not yet functional as an Arduino IDE. Currently at the RFC / toolchain-verification stage.

### Developing

Build instructions are currently unchanged from upstream Zed:

- [Building on macOS](./docs/src/development/macos.md)
- [Building on Linux](./docs/src/development/linux.md)
- [Building on Windows](./docs/src/development/windows.md)

### Licensing

Citadel is a fork of Zed and inherits its licensing as-is: source code is licensed primarily under **GPL-3.0-or-later**, with **Apache-2.0** components where marked (see [LICENSE-GPL](./LICENSE-GPL) and [LICENSE-APACHE](./LICENSE-APACHE)).

Note for users: GPL applies to the IDE itself, not to code you write and compile with it. Sketches/firmware you build using Citadel are **not** subject to GPL just because the IDE is.

License information for third-party dependencies must be correctly provided for CI to pass. We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses — see upstream Zed's `script/licenses/zed-licenses.toml` for the mechanism, inherited unchanged for now.
