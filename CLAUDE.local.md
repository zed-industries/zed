# Zed プロジェクト開発メモ

## リポジトリ構成

- **オリジナル**: `zed-industries/zed` (upstream)
- **Fork**: `seri114/zed` (fork)
- **開発ブランチ**: `enhance`

## Git操作

- `origin` は `zed-industries/zed` (push権限なし)
- `fork` は `seri114/zed` (push先)
- push時は `git push fork enhance` を使用

## nightlyタグ

- ローカルとforkリポジトリのnightlyタグを頻繁に更新する
- 更新コマンド: `git tag -f nightly && git push fork nightly --force`

## コミット前のチェック

push前には必ずフォーマットチェックを行う:
```bash
cargo fmt --check
```
問題がある場合は `cargo fmt` で修正する。
