# Slash Commands Example Extension

This is an example extension showcasing how to write slash commands.

See: [Extensions: Slash Commands](https://zed.dev/docs/extensions/slash-commands) in the Zed Docs.

## Setup

```sh
git clone https://github.com/zed-industries/zed.git
cp -R zed/extensions/slash-commands-example .

cd slash-commands-example/
sed -i '' '/^\[lints\]/,/^$/s/^workspace = true/#workspace = true/' Cargo.toml
git init
git add .
git commit -m "initial commit"
zed .
```

## Usage

1. Open the command palette (`cmd-shift-p` or `ctrl-shift-p`).
2. Launch `zed: install dev extension`
3. Select the `slash-commands-example` folder created above
4. Open an Assistant Panel (`cmd-r`) and type `/echo` or `/pick-one` to test things out.

## Rebuild

1. Open Zed Extensions (`cmd-shift-x` or `ctrl-shift-x`).
2. You will see your Dev Extension ("Slash Commands Example") at the top.
3. Click `Rebuild`

## Troubleshooting

- MacOS: `tail -f ~/Library/Logs/Zed/Zed.log`
- Linux: `tail -f ~/.local/share/zed/logs/Zed.log`
