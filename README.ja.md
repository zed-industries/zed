# Citadel

*[English](./README.md)*

**Citadel**（別名 *Citadel-Duino*）は [Zed](https://github.com/zed-industries/zed) のフォークで、Arduino（AVR）向け組み込み開発に特化した次世代IDEとして開発中です。

> 本プロジェクトは独立した非営利のフォークです。Zed Industries, Inc. とは提携・後援関係にありません。「Zed」はZed Industries, Inc. の商標です。

### Philosophy

> "If it compiles, it runs safely. Never freeze the system, never lock the chip."

### Citadelを作る理由

| 怒り (Frustration) | 対策 (Countermeasure) |
|---|---|
| IDEがクソ重くて作業効率が下がる | ZedのGPU超高速描画（GPUI）とRust製の爆速コアで解決 |
| ビルドが通っても実機でフリーズして原因不明 | コアロジックをRust (no_std) で実装し、メモリ破壊をコンパイル時に排除 |
| 不正終了でIDEが固まり、マイコンのポートも掴みっぱなしで詰む | バックエンドを完全別プロセス化。自動WDT挿入＆レスキューシーケンスで物理・論理の両面からフェイルセーフ |
| 微妙なコードの変更をGitで綺麗に管理・追跡しにくい | Zed標準のインラインGit連携とブランチ連動型ビルドで進捗を地続きに |

アーキテクチャ・スコープ・ロードマップの全体像はプロジェクトRFC（作成中）を参照。

### 現在の状態

初期段階のフォーク — まだArduino IDEとして機能しません。現状はRFC策定・ツールチェーン検証の段階です。

### ビルド方法

現時点ではupstream Zedからビルド手順は変更していません:

- [macOS向けビルド](./docs/src/development/macos.md)
- [Linux向けビルド](./docs/src/development/linux.md)
- [Windows向けビルド](./docs/src/development/windows.md)

### ライセンス

CitadelはZedのフォークであり、ライセンスもそのまま継承しています: ソースコードは主に **GPL-3.0-or-later**、一部 **Apache-2.0** 表示のあるコンポーネントを含みます（[LICENSE-GPL](./LICENSE-GPL)、[LICENSE-APACHE](./LICENSE-APACHE) を参照）。

利用者向けの注記: GPLが適用されるのはIDE本体であり、あなたがCitadelを使って書いた・コンパイルしたコードには適用されません。Citadelで作成したスケッチ／ファームウェアがGPLになることはありません。

サードパーティ依存関係のライセンス情報が正しく提供されないとCIが通りません。ライセンス遵守には [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) を利用しています — 仕組みについては現時点では変更していないupstream Zedの `script/licenses/zed-licenses.toml` を参照してください。
