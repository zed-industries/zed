---
name: jcodemunch-guide
description: >
  Use jCodeMunch MCP tools to explore and retrieve code via structured
  symbol search instead of brute-reading full files. Apply when
  jcodemunch-mcp is connected and you need to understand a repo,
  find a function/class, trace callers, or pull targeted context.
---

# jCodeMunch Agent Guide

When jcodemunch-mcp is available, always prefer its tools over reading
full files. The core loop is:

1. **Index** the repo once.
2. **Discover** structure with outlines.
3. **Search** for the exact symbol or text you need.
4. **Retrieve** only that source.

## Core commands

### Indexing — do this once per repo

| Command | When |
|---|---|
| `index_repo(url="owner/repo")` | Index a GitHub repo |
| `index_folder(path="/abs/path")` | Index a local directory |
| `list_repos` | See what is already indexed |
| `invalidate_cache(repo="...")` + re-index | Stale index |

Indexes live under `~/.code-index/`. After indexing, prefer `resolve_repo`
to confirm the repo ID when working locally.

### Discovery — understand structure before reading code

| Command | When |
|---|---|
| `suggest_queries(repo="...")` | Unfamiliar repo — find entry points and example queries |
| `get_repo_outline(repo="...")` | High-level view: dirs, languages, symbol counts |
| `get_file_tree(repo="...", path_prefix="src")` | Browse file structure |
| `get_file_outline(repo="...", file_path="src/foo.py")` | See all symbols in a file (signatures + summaries) |

Always call `get_file_outline` before pulling source. Use it to pick the
right symbols without reading the entire file.

### Search — find what you need

| Command | When |
|---|---|
| `search_symbols(repo="...", query="thing")` | Find by name, signature, or summary |
| `search_symbols(..., kind="function")` | Narrow to functions, classes, methods, constants |
| `search_symbols(..., language="python")` | Narrow to a language |
| `search_symbols(..., file_pattern="*.py")` | Narrow to a glob |
| `search_symbols(..., fuzzy=true)` | Typos or partial names |
| `search_symbols(..., semantic=true)` | Concept search when name is unknown ("db pool" → `connection_pool`) |
| `search_symbols(..., sort_by="centrality")` | Rank by architectural importance (PageRank) |
| `search_text(repo="...", query="TODO")` | Find text in comments, strings, or non-symbol content |
| `search_text(..., is_regex=true, context_lines=2)` | Regex search with surrounding lines |

### Retrieval — pull only what you need

| Command | When |
|---|---|
| `get_symbol_source(repo="...", symbol_id="file::name#kind")` | Fetch one symbol |
| `get_symbol_source(..., symbol_ids=["a","b"])` | Batch fetch multiple symbols |
| `get_context_bundle(repo="...", symbol_id="...")` | Symbol + imports in one call |
| `get_context_bundle(..., token_budget=4000)` | Cap total response size |
| `get_ranked_context(repo="...", query="...", token_budget=4000)` | Best-fit symbols for a task, ranked and budgeted |
| `get_file_content(repo="...", file_path="...", start_line=10, end_line=40)` | Read a line range |
| `get_context_bundle(..., output_format="markdown", include_budget_report=true)` | Human-readable + budget info |

When you need multiple symbols, batch them with `symbol_ids[]` instead of
calling `get_symbol_source` repeatedly.

### Relationship & impact analysis

| Command | When |
|---|---|
| `find_references(repo="...", identifier="Foo")` | Where is this used? |
| `check_references(repo="...", identifier="Foo")` | Quick yes/no dead-code check |
| `find_importers(repo="...", file_path="src/foo.py")` | What imports this file? |
| `get_blast_radius(repo="...", symbol="file::name#kind")` | What breaks if I change this? |
| `get_call_hierarchy(repo="...", symbol_id="...", direction="callers")` | Who calls this? What does it call? |
| `get_dependency_graph(repo="...", file="...", direction="imports")` | File-level deps up to 3 hops |
| `get_dependency_cycles(repo="...")` | Circular dependencies |
| `find_dead_code(repo="...")` | Unreachable symbols |
| `get_hotspots(repo="...", top_n=10)` | Risky parts: high complexity × churn |
| `get_changed_symbols(repo="...", since_sha="...", until_sha="...")` | What symbols changed between commits? |
| `get_class_hierarchy(repo="...", class_name="Foo")` | Inheritance chain |

### Session tools

| Command | When |
|---|---|
| `plan_turn(repo="...", query="...")` | Opening move: confidence + recommended symbols/files |
| `get_session_context` | What files have I already read/edited this session? |
| `register_edit(file_path="...")` | Invalidate caches after editing a file |

## Mental model

Symbol IDs are stable and structured:

```
{file_path}::{qualified_name}#{kind}
```

Examples: `src/auth.py::AuthHandler.login#method`, `config.py::MAX_RETRIES#constant`

### Response metadata

Every retrieval response carries `_meta.confidence` (0–1 retrieval quality)
and `_meta.freshness`. Use `verify=true` on `get_symbol_source` when
correctness is critical.

## Workflow patterns

```
New repo?
  suggest_queries → get_repo_outline → get_file_tree

Find a function by name?
  search_symbols(query="...", kind="function")

Typo?
  search_symbols(query="...", fuzzy=true)

Concept search?
  search_symbols(query="...", semantic=true)

Need symbol + imports?
  get_context_bundle

Task-driven retrieval with token budget?
  get_ranked_context(query="...", token_budget=4000)

What imports this file?
  find_importers

What breaks if I change X?
  get_blast_radius → find_importers

Is this dead?
  find_dead_code / check_references

Starting a new task in an already-indexed repo?
  plan_turn(query="...")

Already read files this session?
  get_session_context (don't re-read)
```

## Rules

- **Never** brute-read a full file when jcodemunch tools can retrieve just
  the symbols you need.
- Always call `get_file_outline` before pulling source — see the API
  surface first.
- Use `search_symbols` before `get_file_content`.
- Batch with `symbol_ids[]` instead of repeated single-symbol calls.
- When a repo may be stale, `invalidate_cache` + re-index before
  retrieving.
- Use `get_session_context` to avoid re-reading files you already touched.
- Use `plan_turn` as the opening move in unfamiliar repos — it surfaces
  entry points and recommended symbols.