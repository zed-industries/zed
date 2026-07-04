# Что ещё добавить в форк Zed (branch `database-viewer`), чтобы он стал «do-everything» инструментом

## Executive summary

Форк уже закрыл самый уникальный класс задач — нативную панель БД с редактируемым SQL-баром и two-phase MCP write-путём (propose/apply с апрувом), что в самом Zed невозможно даже как расширение (нет регистрации dock-панелей, нет доступа к keychain, нет прямой регистрации в in-process MCP manager — прямо подтверждено обсуждением #50628). Поэтому основной вопрос теперь — какие *смежные* возможности дадут максимум пользы соло-разработчику на Go, который живёт в редакторе и активно гоняет Claude Code. Анализ 63 находок показывает три сильных вектора: (1) AI/agentic-слой, где почти всё строится поверх LLM API + editor-state и потому имеет высокую forkability (semantic index, PR-агент, memory, coverage-агент, custom MCP-инструменты по образцу вашего DB MCP); (2) недостающие «базовые» IDE-фичи с гигантским community-спросом (Test Runner — 802 👍, hover-значения в дебаггере, staged/unstaged git); (3) панель-образные фичи (HTTP/REST-клиент, GitHub PR-review), которые *обязаны* жить в ядре по той же причине, что и ваша DB-панель. Ключевой стратегический вывод: ваш паттерн «MCP-инструмент, читающий/пишущий через живое состояние панели» — это редкий дифференциатор, которого нет ни в апстриме, ни у Cursor/Windsurf, и его дешевле всего расширять на новые домены (terminal, buffers, debugger). Ниже — ранжированный шорт-лист, разбор по категориям и три рекомендации, с чего начать.

---

## Уже есть в Zed — не переизобретать

Всё нижеперечисленное `shipped` в апстриме; строить заново нет смысла — только расширять точечно.

- **Дебаггер на DAP** (базовый UI есть; не хватает лишь hover-значений и т.п.).
- **MCP-клиент полностью**: stdio + HTTP-транспорт, OAuth/preregistration для remote-серверов, auto-reload tool-list, MCP по SSH на remote-хосте, отдельная settings-подстраница. ([MCP docs](https://zed.dev/docs/ai/mcp), PR #39021, #51768, #46756)
- **Agent Client Protocol (ACP) + внешние агенты** (Claude Code, Codex, Gemini CLI, Cursor-agent как first-class процессы; Apache-лицензированный `claude-code-acp`). ([zed.dev/acp](https://zed.dev/acp))
- **Parallel Agents + Threads Sidebar** с git-worktree-изоляцией и Terminal Threads внутри Agent Panel. ([parallel-agents](https://zed.dev/docs/ai/parallel-agents))
- **Agent Skills (SKILL.md) + глобальный AGENTS.md**, Agent Profiles (scoped tool/MCP-права), in-thread search.
- **Zeta/Zeta2 edit-prediction** (open-weight, pluggable-провайдеры: Copilot/Supermaven/Codestral/Mercury).
- **Agent review/diff UI с checkpoints** (accept/reject по ханкам, Restore Checkpoint).
- **Agent sandboxing**: NetworkAccess-allowlist, seccomp/seatbelt/WSL, permission-prompt UI, settings-страница. (PR #59218/#59219/#59448)
- **Git**: Git Graph panel (в `git_ui`), богатая Git Panel (diff-stat, split-view, commit-view, search by hash, markdown-commit-messages), named bookmarks, rainbow brackets, resizable/preview-пикеры, unified Text Finder.
- **Dev Containers** — нативная Rust-реализация devcontainer.json (PR #52338).
- **Extension-точки**: LSP, DAP-адаптеры (+ locators, IPv6), themes/icons/snippets, MCP/context-серверы.

---

## Шорт-лист кандидатов (ранжировано)

Исключены `shipped`-фичи. Ранг ~ ценность×реализуемость, с уклоном в соло-AI-backend-разработчика на Go.

| # | Feature | Категория | Что даёт | Статус в апстриме | Реализуемость в форке | Ценность (1-5) |
|---|---------|-----------|----------|-------------------|------------------------|----------------|
| 1 | **Custom MCP-инструменты, привязанные к состоянию редактора** (обобщение вашего DB MCP на terminal/buffers/debugger) | AI/Agents | Агент читает/пишет через ту же живую панель, что видит юзер, с safety-rails (TTL-токены, DML-классификатор) | absent (нет generic-механизма) | **high** — паттерн уже доказан в вашем форке | 5 |
| 2 | **Codebase semantic search / embeddings-индекс** | AI/Agents | Поиск всех «callers/related» по тысячам файлов без grep-цикла агента; главный gap vs Cursor | absent (disc. [#52337](https://github.com/zed-industries/zed/discussions/52337)) | **med** — фоновый индекс + MCP-tool, без правок рендера | 5 |
| 3 | **Test Runner integration** (gutter run/debug, дерево тестов, inline pass/fail) | Testing | Самый востребованный gap во всём репо; ядро «настоящей IDE» | requested — [#5242, 802👍](https://github.com/zed-industries/zed/issues/5242) | **med** — «giant task» по словам мейнтейнеров, но форкабельно | 5 |
| 4 | **PR-authoring агент** (issue→ветка→коммиты→PR по вашим CLAUDE.md-правилам) | AI/Agents | Композиция gh CLI + skill + worktree; закрывает «file issue → get PR» | absent | **high** — всё уже есть, только fork-side skill/tooling | 4 |
| 5 | **Coverage-in-gutter + test-writing агент** | Testing | Overlay покрытия в gutter (как git-status) + skill, дописывающий тесты | requested ([#19384](https://github.com/zed-industries/zed/issues/19384), #45973) | **high** — gutter-decoration hook уже есть | 4 |
| 6 | **Debugger hover-значения** | Debugging | Hover над переменной → текущее значение (как в VS Code) | requested — [#32932, 115👍](https://github.com/zed-industries/zed/issues/32932) | **high** — точечная надстройка над готовым DAP | 4 |
| 7 | **Turnkey cross-session memory / «Memories»** | AI/Agents | Persistent факты/preferences между сессиями (mutable AGENTS.md) | absent ([builder.io обзор](https://www.builder.io/blog/zed-ai-2026)) | **high** — MCP-сервер поверх SQLite/kv | 4 |
| 8 | **HTTP/REST-клиент панель** (.http-файлы, ответы inline) | HTTP/API | Postman-in-editor; для backend-разработчика — ежедневная нужда; панель = ядро | requested (панель-образное, core-only) | **med** — новая core-панель по образцу DB/Git-панели | 4 |
| 9 | **GitHub PR review внутри Zed** (просмотр PR, diff, комменты, approve/merge) | Collaboration/VCS | Замыкает петлю review↔edit; активно обсуждается | requested — [disc. #34759, 72 comm.](https://github.com/zed-industries/zed/discussions/34759) | **med** — новый GitHub-клиент + auth + панель | 4 |
| 10 | **Staged/Unstaged diffs + line-by-line staging** | VCS | Раздельные staged/unstaged ханки, `git add -p` в GUI | requested — [#26560, 520👍](https://github.com/zed-industries/zed/issues/26560) | **low** — трогает центральную git-модель | 4 |
| 11 | **Difftastic (AST-aware) diffs** | VCS | Читаемые diff для рефакторингов | requested — [#9721, 271👍](https://github.com/zed-industries/zed/issues/9721) | **high** — альтернативный diff-backend за setting | 3 |
| 12 | **Easymotion/flash-style vim-прыжки** | Editor UX | Прыжок к любой видимой точке по label | requested — [#4930, 444👍](https://github.com/zed-industries/zed/issues/4930) | **high** — аддитивно к существующему vim-crate | 3 |
| 13 | **Undo-tree панель** (визуальная история буфера) | Editor UX | Навигация к любому прошлому состоянию, не только линейно | requested ([#17455, 120👍](https://github.com/zed-industries/zed/issues/17455)) | **high** — UI поверх готовой per-buffer истории | 3 |
| 14 | **Открытие `.code-workspace`** | Editor UX | Миграция VS Code-конфигов multi-root | requested ([#9459, 327👍](https://github.com/zed-industries/zed/issues/9459)) | **high** — парсинг JSON → worktree-модель | 2 |
| 15 | **SSH/remote agent-dispatch (à la Cursor Agents Window)** | AI/Agents | Агенты на remote/SSH-таргетах, не только local+worktree | absent | **med** — переиспользует remote-dev SSH Zed | 3 |

---

## По категориям

### AI / Agents
- **Custom editor-state MCP-инструменты (обобщение DB MCP).** Нет generic-механизма «инструменты, привязанные к живому состоянию панели» ни в Zed, ни у конкурентов. Ваш `propose_write`/`apply_write` с SessionMode — это и есть паттерн; обобщение (стабильный внутренний API «дай состояние активной панели/буфера/терминала/debugger») дёшево открывает новые editor-state-aware инструменты. Источник — inferred (без URL), но подтверждён отсутствием эквивалента в [MCP docs](https://zed.dev/docs/ai/mcp). **Главный дифференциатор форка.**
- **Semantic index / embeddings.** Zed намеренно не индексирует; главный функциональный gap vs Cursor для больших кодовых баз. Реализуемо как фоновый пайплайн (embeddings-store + инкрементальный re-index на save) + MCP-tool, без правок рендера. Открытое обсуждение [#52337](https://github.com/zed-industries/zed/discussions/52337). Confidence: medium.
- **PR-authoring агент.** Нет нативного «issue→PR». Композиция `gh pr create` + skill + worktree-изоляция + ваши PR-hygiene-правила (release-notes-формат из CLAUDE.md). Никаких upstream-работ. Confidence: medium.
- **Cross-session memory.** У Cursor/Windsurf есть, у Zed — нет; назван «single biggest AI-UX gap». MCP-сервер поверх local kv/SQLite с read/write-memory tools. Confidence: medium ([builder.io](https://www.builder.io/blog/zed-ai-2026)).
- **SSH/remote agent-dispatch.** Cursor 3 «Glass» гоняет до 8 агентов local/worktree/cloud/SSH; Zed покрывает local+worktree, но не remote как first-class target. Переиспользует remote-dev SSH. Caveat: cloud-runner требует хостинга; SSH-dispatch реалистичнее.
- (Пропущено как shipped/не-новое: Parallel Agents, ACP, Skills, Profiles, Zeta2, in-thread search — см. верхний раздел.)

### Testing
- **Test Runner.** Самый высокореактивный open-issue репо ([#5242, 802👍](https://github.com/zed-industries/zed/issues/5242)); нативного «run this test» нет вообще. Мейнтейнеры зовут задачу «giant» дизайнерски, но она форкабельна (gutter + tree + status-reporting). Для Go особенно ценно (`go test` per-func/per-package).
- **Coverage-in-gutter + test-writing агент.** Overlay покрытия в gutter (переиспользует git-status decoration-слой) + skill, гоняющий coverage-tool и дописывающий пробелы. Долгоживущие запросы [#19384](https://github.com/zed-industries/zed/issues/19384), #24002, #45973.

### Debugging
- **Hover-значения.** DAP-дебаггер уже есть; [#32932 (115👍)](https://github.com/zed-industries/zed/issues/32932) — точечная UX-надстройка (hover-provider + DAP value-request). Высокая реализуемость, изолированно.

### VCS / Git
- **Staged/Unstaged + line-by-line staging.** [#26560 (520👍)](https://github.com/zed-industries/zed/issues/26560) + [#45295](https://github.com/zed-industries/zed/issues/45295). Высокая ценность, но **low feasibility** — трогает центральную git-модель и diff-pipeline, не изолированную панель.
- **Difftastic.** [#9721 (271👍)](https://github.com/zed-industries/zed/issues/9721) — альтернативный diff-backend за setting, не требует переархитектуры. High feasibility.
- **GitHub PR review.** [disc. #34759 (72 comments)](https://github.com/zed-industries/zed/discussions/34759) — новая панель + GitHub API + auth. Замыкает review↔edit-петлю.
- **jj (Jujutsu) SCM.** [#21538 (454👍)](https://github.com/zed-industries/zed/issues/21538) — второй VCS-backend. Med feasibility, большой скоуп; ниша (не факт, что нужна Go-разработчику на git).

### HTTP / API
- **REST-клиент панель** (.http/.rest-файлы, inline-ответы, env-переменные). Панель-образная фича → **core-only** по той же причине, что DB-панель (нет dock-регистрации в extension-API). Для backend-разработчика — прямая ежедневная польза; архитектурно копирует вашу DB-панель (запрос → результат-грид).

### Editor UX
- **Easymotion/flash-прыжки** [#4930 (444👍)](https://github.com/zed-industries/zed/issues/4930), **flash-search** [#14801](https://github.com/zed-industries/zed/issues/14801) — аддитивно к vim-crate.
- **Undo-tree** [#17455](https://github.com/zed-industries/zed/issues/17455) — UI поверх готовой истории.
- **`.code-workspace`** [#9459](https://github.com/zed-industries/zed/issues/9459) — миграционная фича, парсинг JSON.
- **Smooth scrolling** [#4355 (778👍)](https://github.com/zed-industries/zed/issues/4355) — 2-й по спросу, но завязан на GPUI rendering/animation internals (**med/low**, core-adjacent); польза «feel», не функциональная.

### Remote / Containers, Notebooks
- **Dev Containers** — уже shipped нативно; при желании форк расширяет `customizations.zed.*` без shell-out.
- **Jupyter/interactive computing** [#9778](https://github.com/zed-industries/zed/issues/9778) — high demand у data-science, но **low feasibility**: зависит от заблокированного webview/rich-output рендера, многоквартальная core-инициатива. Для Go-бэкендера низкий приоритет.

---

## Требуют глубокой работы в ядре / низкая реализуемость

- **Staged/Unstaged + partial staging** — центральная git-модель, не изолируемо (но ценность 4/5, стоит держать в уме).
- **Smooth scrolling** — GPUI rendering/animation internals.
- **Secondary/multi-monitor windows** ([#9662, 384👍](https://github.com/zed-industries/zed/issues/9662)) — глубокие правки window/workspace-lifecycle и сериализации.
- **Jupyter notebooks** — зависит от несуществующего webview/rich-render surface.
- **Zeta2 retraining** — нужны данные + train-инфра (но *смена/приоритизация провайдеров* — high).
- **Webview/Visual Extension API** ([RFC #53403](https://github.com/zed-industries/zed/discussions/53403)) — мейнтейнеры прямо назвали не-near-term; форку ждать нельзя, панели строить в ядре.

---

## Пробелы и достоверность

- **Реакции, не перепроверенные live этой сессией** (взяты из авто-трекера Zed #5393): #14801, #21208, #9459, #7808, #17455, #45295. Если число решает — проверьте `gh issue view`.
- **disc. #34759 (PR review)**: verified 72 comments, но upvote/reaction-count через API не вытащен — реальный спрос может быть выше/ниже.
- **Custom editor-state MCP (кандидат #1)**: источник — *inferred, без URL* (из истории вашего же форка). Confidence: medium. Это гипотеза-обобщение, а не документированный upstream-факт.
- **Semantic index, PR-агент, memory, coverage-агент, SSH-dispatch**: confidence medium, часть URL — сторонние обзоры (builder.io, cursor-alternatives, dev.to), не первоисточники Zed. Стоит перепроверить актуальность перед планированием.
- **Windsurf/Cursor сравнения** — маркетинговые/обзорные статьи 2026, не бенчмарки; трактовать как «направление», не как факт.
- **Оговорка процесса:** 2 из 6 исследовательских агентов (baseline «shipped» и «gaps vs конкурентов») деградировали в placeholder-заглушки; их зона покрыта остальными углами + синтезом, но раздел «Уже есть в Zed» и часть IDE-gap стоит перепроверить, если будете опираться на него как на исчерпывающий.

---

## Рекомендация: с чего начать

Три кандидата на брейншторм следующим шагом (не дизайн — только рационал):

1. **Обобщённые editor-state MCP-инструменты (кандидат #1).** Прямое продолжение вашего DB MCP и единственный настоящий дифференциатор форка; паттерн уже доказан, обобщение на terminal/buffers/debugger даёт много ценности при малой стоимости — и композируется с Claude Code через ACP, который вы и так используете.

2. **Test Runner для Go (кандидат #3) + coverage-in-gutter (#5).** Закрывает #1 по спросу gap всего репо и напрямую бьёт в ваш daily-workflow на Go; coverage-overlay переиспользует готовый git-gutter-слой, а test-writing-skill естественно ложится на agentic-стек.

3. **PR-authoring агент (кандидат #4) или Memory-MCP (#7).** Оба — high-feasibility, чистый fork-side skill/MCP без upstream-работ: PR-агент автоматизирует issue→PR по вашим же CLAUDE.md-правилам; Memory-MCP закрывает названный «крупнейший AI-UX gap» Zed vs конкурентов дешёвым SQLite-сервером.
