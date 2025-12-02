Windows / 日本語キーボード / IME の取り扱い
=========================================

概要
----
このドキュメントは gpui が Windows 上で日本語キーボード（半角/全角、変換、無変換など）や IME 合成をどのように扱うか、アプリ側でどう扱うべきかを説明します。

重要点
-------
- gpui は WM_IME 系のメッセージ（ImmGetCompositionStringW を使う合成道路）を正しくサポートしており、テキスト合成（marked text / composition）を EntityInputHandler を通じて扱えます。
- 一部の日本語キーボード固有キー（半角/全角切替など）は OS/IME が VK_PROCESSKEY 経由で扱うことがあり、その場合 ImmGetVirtualKey が特定の値（例: 0xF0..0xF4）を返すことが観測されました。

今回の実装変更
---------------
- gpui の Windows キーハンドリングで以下のキー名をアプリ側に渡せるようにしました:
  - "ime_toggle" — 半角/全角や IME モード切替に使われる IME 固有の vkey (0xF0..0xF4 のケース)
  - "convert" — VK_CONVERT
  - "nonconvert" — VK_NONCONVERT
  - "kana" — VK_KANA

アプリ側の推奨実装
------------------
1. 通常の日本語入力は IME の合成フローに任せ、EntityInputHandler の replace_and_mark_text_in_range / replace_text_in_range を実装してください。これで候補のマーキング・確定・キャンセルが正しく扱えます。
2. 半角/全角などのモード切替をアプリで検出して独自の UI を出したい場合、KeyDown/KeyUp における `keystroke.key` を監視し、`ime_toggle` / `convert` / `nonconvert` / `kana` を分岐対象にしてください。
3. IME の実装やキーボードレイアウトによって vkey の値は変化する可能性があるため、アプリは IME の合成イベント (composition/result) を主要な入力経路として扱い、特殊キーは補助的に扱う方が堅牢です。

例
--
- `examples/ime_input.rs` を参照してください（簡易的なテキスト入力例）。

この変更は Windows 固有の扱いに関する互換性向上であり、Linux/macOS の既存の IME 合成パスには影響しません。
