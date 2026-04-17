# Zed

> A text adventure set inside the Zed code editor. Fix the Unclosed Brace. Escape the editor.

## What is it?

- A classic text adventure game themed around the Zed code editor
- 9 rooms modeled after real Zed features: Editor Pane, Terminal, AI Panel, Git Repo, and more
- 8 collectible items: LSP Wand, Diff Prism, Autosave Amulet, Semicolon Key, and others
- One goal: find and fix the Unclosed Brace bug that trapped you inside
- Beautiful TUI powered by ratatui, runs perfectly in Zed's integrated terminal

## Why?

- Zed has a built-in terminal — why not play a game in it?
- Text adventures are the original developer entertainment
- The puzzles mirror real editor workflows: using language servers, reading diffs, managing git tokens
- Every room description is a love letter to the features that make Zed great

## Quick Start

```bash
cargo install zed
zed
```

Or run from source:

```bash
git clone https://github.com/zed-industries/zed
cargo run --release
```

## Commands

| Command | Description |
|---|---|
| `go north` / `n` | Move in a direction |
| `take [item]` | Add item to inventory |
| `drop [item]` | Leave item in current room |
| `examine [item]` | Inspect an item closely |
| `inventory` / `i` | List your items |
| `look` | Describe current room |
| `give [item] to [target]` | Give item to character |
| `use [item]` | Use an item |
| `use [item] on [target]` | Use item on target |
| `open [thing]` | Open something |
| `help` | Show this list |
| `quit` | Exit the game |

## The Map

```
[Command Palette]
       |
[AI Panel] — [Editor Pane] — [File Tree] — [Extensions Gallery]
                   |
             [Terminal] — [Git Repo]
                   |
             [The Buffer] — [Settings Vault]
```

## Win Condition

Collect all 6 key items and use them to unlock the Settings Vault.
Then use the LSP Wand on the Unclosed Brace to fix it and escape.

## Exit Codes

- `0` — Game completed (you won or quit normally)
- `1` — Runtime error

## License

MIT OR Apache-2.0

---

## O que é? (Português)

- Um jogo de aventura em texto temático do editor Zed
- 9 salas baseadas em funcionalidades reais do Zed
- 8 itens para coletar com puzzles lógicos
- Objetivo: encontrar e corrigir o bug Unclosed Brace para escapar
- Interface TUI colorida com ratatui, roda no terminal integrado do Zed

## Por que existe?

- O Zed tem terminal integrado — aproveite para jogar
- Os puzzles espelham fluxos reais de desenvolvimento
- Cada descrição de sala celebra uma feature do Zed
- É divertido e foi escrito inteiramente em Rust
