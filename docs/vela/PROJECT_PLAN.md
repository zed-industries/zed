# AI Code IDE 项目规划

> 状态：Draft v0.1  
> 目标平台：macOS arm64 优先，后续支持 Linux / Windows  
> 调研基线：Zed `6297c88f42`、Pi `9b3a2059`

## 1. 项目愿景

基于 **Zed** 的高性能编辑器、Tree-sitter、LSP、调试、工作区和原生 Rust Agent 能力，并参考 **Pi** 的多模型、会话管理、工具调用等设计，构建一个：

- 日常开发足够快、稳定、低延迟；
- AI 与编辑器上下文深度结合，而不是简单嵌入聊天窗口；
- 支持函数、类型、属性、变量的定义跳转、引用查找、重命名和诊断；
- 支持主流语言工具链自动发现、明确配置和故障诊断；
- 模型与供应商可替换，项目数据默认留在本地；
- 尽量少修改 Zed 上游代码，便于持续升级。

暂定项目代号：**CodeIDE**。

### 1.1 主开发语言

项目运行时代码以 **Rust** 为主语言：桌面端、Agent Runtime、模型适配、语义工具、权限系统、配置、会话和诊断均在 Zed 进程内运行。**不嵌入 Node.js，不启动 Pi 子进程，也不依赖 Pi RPC/ACP 适配器。** Pi 仅作为设计与行为参考；适合复用的算法需按 MIT 许可证要求以 Rust 重写并记录来源。

## 2. 已调研项目结论

### 2.1 Zed 可复用能力

Zed 是 Rust + GPUI 构建的高性能编辑器，已经具备本项目最难且最基础的部分：

- Tree-sitter：语法高亮、代码结构、outline、缩进、语言注入；
- LSP：补全、诊断、Hover、定义/类型定义跳转、引用、重命名、Code Action、Workspace Symbol；
- DAP：调试器接入；
- Toolchain：部分语言的 SDK、解释器和虚拟环境发现；
- 工作区、终端、任务、Git、远程开发与扩展系统；
- Agent Panel、并行线程和 ACP（Agent Client Protocol）外部 Agent 支持；
- Zed 原生 Agent 已有基于 LSP 的 `go_to_definition`、`find_references`、`diagnostics` 等工具实现，可作为语义桥接设计参考。

关键源码位置：

- `../zed/crates/language*`：语言和 Tree-sitter 基础；
- `../zed/crates/lsp`、`../zed/crates/project`：LSP 与项目模型；
- `../zed/crates/editor`、`../zed/crates/workspace`：编辑器和工作区；
- `../zed/crates/agent_servers`、`../zed/crates/acp_thread`：ACP 客户端；
- `../zed/crates/agent/src/tools/`：原生 Agent 的语义工具。

### 2.2 Pi 可参考能力

Pi 是 TypeScript/Node.js 的轻量 Agent Harness。本项目不直接运行或嵌入 Pi，但参考以下成熟设计：

- 多模型供应商与 OAuth/API Key 支持；
- 流式事件、工具调用、中止、重试、消息队列；
- JSONL 树形会话、分支、恢复和上下文压缩；
- SDK、RPC 模式以及 TypeScript Extension；
- 内置 `read`、`write`、`edit`、`bash` 等工具；
- 项目级 `AGENTS.md`、Skills、Prompts 和扩展发现；
- MIT License，适合独立适配与分发。

关键源码位置：

- `../pi/packages/ai`：模型供应商抽象；
- `../pi/packages/agent`：Agent 状态机和工具循环；
- `../pi/packages/coding-agent`：SDK、RPC、会话与扩展；
- `../pi/packages/coding-agent/docs/sdk.md`：嵌入 API；
- `../pi/packages/coding-agent/docs/rpc.md`：JSONL RPC 协议。

### 2.3 重要约束

1. **不要重写编辑器和 LSP 客户端。** 首版直接复用 Zed。
2. **AI Runtime 必须进程内运行。** 以 Zed 的 `agent`、`language_model*`、`agent_ui` 等 Rust crate 为基础扩展，不使用 Pi 子进程、Node 嵌入或 TUI/RPC 转换。
3. **“不使用子进程”特指 AI Runtime。** LSP、DAP、终端、编译器和格式化器按其协议天然需要外部进程；完全禁用所有子进程将无法实现完整语言支持。
4. **权限必须位于工具执行入口。** Agent 即使在进程内运行，也不能绕过写文件、Shell、网络和敏感路径授权。
5. **许可证必须提前处理。** Zed 主体为 GPL-3.0-or-later（部分组件 Apache-2.0），Pi 为 MIT。分发修改后的 Zed 桌面端需满足 GPL 源码与许可证义务；参考或移植 Pi 代码时保留 MIT 声明与来源记录。
6. 当前机器为 macOS arm64；本机 Rust `1.94.1`，而当前 Zed 固定为 `1.95.0`，首次构建前需要由 rustup 安装对应工具链。

## 3. 产品范围

### 3.1 MVP 必须具备

- 文件树、搜索、编辑、多光标、终端、Git 基础体验；
- TypeScript/JavaScript、Python、Go、Rust 四组语言的一等支持；
- 定义跳转、类型定义、查找引用、符号重命名、Workspace Symbol；
- 补全、Hover、诊断、格式化、Code Action、Inlay Hints；
- 进程内 Rust Agent 对话、流式输出、停止、继续、恢复会话；
- 聊天框内可视化 Markdown、代码、工具调用、诊断、测试结果和文件改动；
- Agent 可读取/编辑当前工作区，每轮改动形成可追踪 ChangeSet；
- 点击聊天中的代码或改动卡片可打开对应文件 Diff；
- Shell 和写文件操作有清晰的权限确认；
- Agent 修改后可读取最新 LSP 诊断；
- 模型选择、认证状态、Token/费用信息和错误提示；
- 完整文件树，支持打开、创建、移动、重命名、删除、搜索和 Git/诊断/AI 改动状态；
- 支持选择一段或多段代码加入对话，并围绕选择内容进行多轮沟通；
- 设置、语言、模型、权限、快捷键等配置均可在 UI 中完成；
- 模型供应商支持自定义 Base URL、API Key、协议类型和模型列表；
- 支持通过快捷键打开 Agent、加入选择、查看 Diff、切换对比分支、生成 Commit 信息和提交；
- UI 修改自动生成带注释的 TOML 配置文件，不把 JSON 作为用户配置格式；
- 崩溃后工作区、未保存文件、Agent 会话和未处理 ChangeSet 可恢复。

### 3.2 MVP 不做

- 自研文本渲染器、LSP 或语言服务器；
- 云端账号、团队协作和计费系统；
- 自动执行任意高危命令；
- 同时大幅重构 Zed UI 与 Agent Runtime；
- 首版覆盖所有 Zed 支持的语言；
- 首版实现完整远程开发、容器编排和多人协同。

### 3.3 核心交互模型

#### 聊天与结果可视化

Agent Panel 是主要入口，消息不是单一字符串，而是由结构化 Block 组成：

- `TextBlock`：Markdown、列表、表格和链接；
- `CodeBlock`：语言、代码、来源路径和可选行范围；
- `ToolCallBlock`：工具名称、状态、耗时、权限和可折叠输出；
- `DiagnosticBlock`：错误、警告、位置和修复状态；
- `CommandBlock`：命令、输出、退出码和取消状态；
- `FileChangeBlock`：文件、增删行数、ChangeSet id 和 Diff 入口；
- `ProgressBlock`：当前步骤、排队状态、重试和取消。

流式响应只更新对应 Block，不重建整个消息列表。长输出默认折叠并虚拟化，避免聊天历史增长后拖慢编辑器。

#### 代码改动与 Diff

每次 Agent 写入都经过 `ChangeTransaction`，记录：

- 修改前 buffer revision 与内容快照；
- 修改后的 revision；
- 文件路径、hunk、发起消息和工具调用；
- 本轮 `ChangeSetId`；
- 后续用户编辑造成的冲突或失效状态。

聊天中的代码引用、文件名和 `FileChangeBlock` 都可点击：

1. 若该内容产生过修改，打开对应 ChangeSet 的 Diff；
2. 若只是代码引用，跳到源文件与范围；
3. Diff 支持按文件或 hunk 接受、拒绝、撤销；
4. 文件已被用户继续修改时进行三方比较，不用旧快照静默覆盖；
5. 一个 Agent turn 可以修改多个文件，但必须归属于同一个可审计 ChangeSet。

#### 完整文件树

直接复用并增强 Zed Project Panel：

- 工作区多根目录、展开/折叠、拖动、复制路径和快速搜索；
- 新建、移动、重命名、删除文件与目录；
- Git 状态、诊断数量、Agent 未接受改动状态；
- 右键“加入对话”“让 Agent 解释/修改/测试”；
- 文件、目录和多选条目都能成为结构化聊天上下文；
- 点击聊天中的路径时，在文件树定位并在编辑器打开。

#### 选择代码进行多轮沟通

编辑器支持把当前 selection 通过快捷键或右键加入当前会话。上下文项保存：

- 工作区与相对路径；
- buffer revision；
- start/end anchor 和选中文本快照；
- 语言、所属 symbol，以及前后少量上下文；
- `pinned`（跨轮保留）或 `once`（仅下一轮）生命周期。

聊天输入区以 Context Chip 显示已选择的文件、目录、symbol 和代码范围。多轮对话规则：

- pinned selection 默认持续参与后续轮次，用户可随时移除；
- buffer 改动后优先通过 Zed anchor 跟踪新范围；
- 无法可靠重定位时标记 `stale`，展示旧快照与当前位置差异，不静默替换；
- Agent 回答引用范围时可点击回到编辑器；
- 支持同时加入多个不连续 selection，并明确 Token 预算；
- 会话恢复后保留 Context Chip、快照和失效状态。

#### MVP 端到端验收场景

1. 用户从完整文件树打开 `src/service.rs`；
2. 选中一个函数并执行“加入对话”；
3. 连续提问“解释它”→“重构错误处理”→“补充测试”，无需重复粘贴代码；
4. 聊天框实时展示分析、工具状态、测试结果和两个文件的改动卡片；
5. 点击 `src/service.rs +12 -5`，打开该轮精确 Diff；
6. 用户接受一个 hunk、拒绝另一个 hunk，文件树状态同步更新；
7. 用户手动修改同一区域后再次提问，系统提示 selection/ChangeSet 已变化并安全重定位；
8. 用户接受改动后，通过快捷操作生成基于 staged diff 的 Commit 信息，确认后提交；
9. 用户在设置 UI 中修改 Rust 格式化和 Agent 权限，磁盘上的项目 TOML 配置原子更新；
10. 重启 IDE 后，会话、选择上下文、未处理 Diff 和配置均可恢复。

### 3.4 快捷操作与 Git 工作流

所有快捷操作先注册为稳定的 Rust Action，再由快捷键映射调用，避免把业务逻辑写进按键处理器。首批 Action：

| Action | 功能 |
|---|---|
| `codeide::ToggleAgentPanel` | 打开或聚焦聊天面板 |
| `codeide::AddSelectionToChat` | 将当前代码选择加入会话 |
| `codeide::SendChatMessage` | 发送消息或按配置排队 |
| `codeide::OpenTurnDiff` | 打开当前 Agent turn 的 ChangeSet Diff |
| `codeide::SelectDiffBase` | 选择 Branch Diff 的对比分支或 commit |
| `codeide::AcceptCurrentHunk` | 接受当前 Diff hunk |
| `codeide::RejectCurrentHunk` | 拒绝当前 Diff hunk |
| `codeide::GenerateCommitMessage` | 根据 staged diff 生成 Commit 信息 |
| `codeide::CommitStaged` | 提交已暂存改动 |
| `codeide::ReviewAndCommit` | 打开审查界面，生成信息并确认提交 |

默认快捷键需要先审计 Zed 在 macOS/Linux/Windows 上的现有绑定，避免覆盖编辑器常用操作。用户可在快捷键 UI 中搜索 Action、录制组合键、查看冲突并修改；最终写入 `keybindings.toml`，不要求手写。

#### 可配置的 Diff 对比基准

不能把所有 Diff 固定为相对 `main`、`master` 或仓库默认分支。UI 必须清晰区分三种视图：

1. **Turn Diff**：本轮 Agent 修改前快照 ↔ 本轮结果，不依赖 Git 分支；
2. **Working Tree Diff**：`HEAD` 或 index ↔ 当前工作区，用于暂存与提交；
3. **Branch Diff**：用户选择的 branch/tag/commit ↔ 当前分支或工作区，用于查看整个功能分支的累计变化。

Branch Diff 顶部提供可搜索的 Base Picker，支持：

- 本地分支，例如 `develop`、`release/1.x`；
- 远程跟踪分支，例如 `origin/develop`；
- tag、任意 commit SHA 和当前分支 upstream；
- 自动模式：优先当前分支 upstream，其次仓库配置的 base，再次才使用默认分支；
- `merge-base` 模式：从共同祖先开始查看当前分支引入的变化，作为默认比较语义；
- `direct` 模式：直接以所选 ref 的树作为基准。

选择结果可以按 workspace 保存，也可以只覆盖当前会话或当前 Diff Tab。切换基准只改变展示和分析范围，不修改分支、不执行 checkout/reset/rebase。远程 ref 的 fetch 必须由用户明确触发。

当 ref 不存在、已被删除或无法解析时，保留原值并显示错误，不能静默退回默认分支。Diff 标题始终展示完整基准，例如：

```text
Branch Diff: merge-base(origin/develop, feature/agent-ui) → Working Tree
```

Agent 引用“本次修改”时默认链接 Turn Diff；用户要求“总结当前分支改动”时使用当前 Branch Diff 基准。Commit 信息生成仍默认只使用 staged diff，不能因切换 Branch Diff 而把历史提交或未暂存内容混入本次 Commit。

#### Commit 信息生成流程

1. 默认只读取 **staged diff**，不把未暂存内容混入 Commit；
2. 可参考仓库最近 Commit 的语言和格式，以及项目配置中的 Conventional Commits 规则；
3. 在 Git 面板展示可编辑的标题和正文，不直接提交；
4. 用户确认后执行 Commit，展示 hooks、签名和失败信息；
5. Commit 成功后关联对应 ChangeSet 和会话消息，便于回溯；
6. `commit` 与 `push` 必须是独立 Action，默认快捷操作不会自动 push；
7. 仓库存在冲突、无 staged change 或 ChangeSet 尚未审批时，UI 明确阻止或警告；
8. 支持“不调用模型”的模板模式，在离线状态下按文件类型和变更摘要生成基础信息。

#### 快捷提交安全规则

- 首次使用 `CommitStaged` 默认进入确认界面，而非静默提交；
- 用户可在设置中启用“确认后记住”，但受保护分支始终确认；
- 不自动绕过 Git hooks，不默认使用 `--no-verify`；
- 不把 Diff 中可能存在的密钥原文发送给模型，生成前运行敏感信息过滤；
- Amend、force push、reset、rebase 等高风险操作使用独立权限等级。

### 3.5 UI 配置与磁盘格式

#### UI-first 原则

所有 CodeIDE 自身配置都必须提供可发现的 UI，不要求用户手写配置文件：

- 通用编辑器设置、外观和行为；
- 语言、LSP、Formatter、Debugger 和 Toolchain；
- 模型供应商、默认模型、思考等级和网络设置；
- 自定义模型端点的 Base URL、API 协议、认证方式、附加请求头和模型列表；
- Agent 工具权限、敏感路径和审批策略；
- 快捷键、任务、终端和 Git 行为；
- 工作区级覆盖与配置来源说明。

每个设置项显示当前值、默认值、最终生效值及来源层级。高级用户仍可在 IDE 内打开“高级 TOML 编辑器”，获得 schema 补全、注释、校验和错误定位，但普通操作不依赖它。

#### TOML 配置格式

用户可见配置统一使用 **TOML**，原因是与 Rust 生态一致、支持注释、可读性好，并有成熟的 `serde` 与 `toml_edit` 支持。建议路径：

```text
~/.config/codeide/config.toml          # 全局设置
~/.config/codeide/keybindings.toml     # 全局快捷键
~/.config/codeide/permissions.toml     # 全局权限
<workspace>/.codeide/config.toml       # 项目覆盖，可提交版本库
<workspace>/.codeide/tasks.toml        # 项目任务
<workspace>/.codeide/debug.toml        # 调试配置
```

示例：

```toml
schema_version = 1

[editor]
font_size = 14
format_on_save = true

[agent]
default_model = "company-gateway/code-model"
# 写文件前始终展示 Diff
write_policy = "review"

[model_providers.company-gateway]
protocol = "openai-responses"
base_url = "https://ai.example.com/v1"
credential = "keychain:codeide/company-gateway"

[[model_providers.company-gateway.models]]
id = "code-model"
context_window = 131072
max_output_tokens = 8192
# 达到可用输入预算的 85% 时自动压缩
compact_threshold = 0.85

[languages.rust]
language_server = "rust-analyzer"
formatter = "rustfmt"
inlay_hints = true

[git.diff]
base = "origin/develop"
mode = "merge-base" # 也可使用 "direct"
```

快捷键文件同样使用 TOML，并由 UI 生成：

```toml
schema_version = 1

[[bindings]]
keys = "cmd-shift-enter"
action = "codeide::ReviewAndCommit"
when = "GitPanel && HasStagedChanges"

[[bindings]]
keys = "cmd-shift-d"
action = "codeide::OpenTurnDiff"
when = "AgentPanel"
```

以上按键仅为格式示例，正式默认值需通过跨平台冲突审计。

实现要求：

- 使用 `toml_edit` 修改配置，保留用户注释、字段顺序和未知字段；
- 临时文件写入、`fsync` 后原子 rename，避免断电产生半文件；
- UI 写入前进行强类型校验，失败时不覆盖旧配置；
- 文件监听支持外部变更热加载，UI 显示来源与冲突，不采用最后写入者静默覆盖；
- 配置带 `schema_version`，迁移前自动备份并提供迁移报告；
- 只有偏离默认值的配置才写盘，UI 可按单项/分组恢复默认；
- API Key、OAuth Token 等秘密只存系统 Keychain，TOML 仅保存 credential id；
- Agent 会话和高频运行状态不塞进 TOML，应使用现有数据库或专用持久化层；
- 外部生态强制要求的格式（例如 `Cargo.toml`、`tsconfig.json`）保持其标准格式；“不使用 JSON”限定为 CodeIDE 自身生成和维护的用户配置。

#### 自定义模型 Base URL 与 Key

模型设置 UI 提供“添加自定义供应商”，至少支持：

- 自定义供应商 id、显示名称和 Base URL；
- API 协议：OpenAI Responses、OpenAI Chat Completions、Anthropic Messages；
- API Key、Bearer Token 或自定义认证 Header；
- 手动填写模型 id，或从兼容的 models endpoint 拉取；
- 每个模型独立配置上下文窗口、最大输出、压缩阈值、图像输入、工具调用和 thinking 能力；
- 自定义非敏感 Header、请求超时和代理设置；
- “测试连接”功能，展示 DNS/TLS/HTTP/协议解析阶段的具体错误。

#### 上下文窗口与 Token 预算

不能假设自定义服务一定返回上下文大小，也不能在缺失时静默套用固定默认值。每个模型都允许在 UI 中配置：

- `context_window`：模型总上下文窗口；
- `max_output_tokens`：单次最大输出；
- `compact_threshold`：触发自动压缩的比例；
- `safety_margin_tokens`：为工具结果和估算误差预留的空间；
- 可选的模型 tokenizer/估算策略。

上下文大小的解析优先级：

1. 用户针对该模型设置的显式值；
2. 服务能力接口返回且通过校验的值；
3. CodeIDE 内置模型目录的已知值；
4. 均不存在时标记为 `unknown`，要求用户设置；仅允许用户主动选择保守兼容值，不能静默使用默认大小。

UI 必须显示当前数值及来源：`manual`、`provider`、`catalog` 或 `unknown`。自定义服务第一次选择未知模型时，在发送消息前弹出配置面板；也可通过“探测能力”尝试获取，但探测结果未经确认不永久写盘。

Token 预算按以下原则计算：

```text
usable_input = context_window - max_output_tokens - safety_margin_tokens
compact_at  = usable_input × compact_threshold
```

- 系统提示、消息、选择代码、工具定义、工具结果都计入输入预算；
- 发送前再次检查预算，禁止产生已知会超过窗口的请求；
- `max_output_tokens` 必须小于 `context_window`，UI 对非法组合即时校验；
- 服务返回 context overflow 时展示服务错误和当前配置，不自动篡改持久配置；
- 用户可在模型设置 UI 中修正并重试；
- 会话切换模型后立即按新模型窗口重新计算预算和压缩状态；
- 聊天输入区显示已用 Token、可用 Token、预留输出和压缩阈值。

安全要求：

- API Key 通过 UI 输入后立即写入系统 Keychain，绝不明文写入 TOML、日志、聊天记录或错误报告；
- TOML 只保存类似 `keychain:codeide/company-gateway` 的 credential 引用；
- Base URL 必须解析并规范化，默认要求 HTTPS；仅 localhost/明确授权的开发端点允许 HTTP；
- 重定向不得把认证 Header 转发到不同 origin；
- 日志统一脱敏 `Authorization`、`api-key`、Cookie 和用户配置的敏感 Header；
- 测试连接不把工作区代码发送出去，只请求能力或模型列表；
- 删除供应商时询问是否同步删除 Keychain 凭证；
- 项目级配置可以引用 credential id，但不能把全局秘密复制进仓库。

#### 与 Zed 设置系统兼容

Zed 当前设置体系包含 JSON/JSONC 资源。CodeIDE 首期增加 TOML facade：TOML 是用户配置的唯一事实来源，解析后映射到 Zed 强类型 Settings Store，不生成用户可见的 `settings.json`。上游内置资源可暂时保留原格式，之后再评估迁移，避免第一阶段重写整个设置框架。

## 4. 总体架构

```text
┌──────────────────── CodeIDE 单一 Rust 进程 ───────────────────┐
│ Zed / GPUI                                                    │
│ Editor · Workspace · Git · Diff · Settings · Agent UI        │
│        │                              │                        │
│        │ Rust API                     │ Rust API               │
│        ▼                              ▼                        │
│ Project / Language             CodeIDE Agent Runtime          │
│ Tree-sitter · LSP state        Loop · Context · Sessions      │
│        ▲                              │                        │
│        └──── Semantic Tools ──────────┤                        │
│                                       ▼                        │
│                              Language Model Providers          │
│                         HTTP/SSE/WebSocket · Auth · Usage       │
└────────────────────────────────────────────────────────────────┘

外部的 LSP、DAP、终端命令和编译器仍由 Zed 按标准协议管理，它们不属于 AI Runtime。
```

### 4.1 集成策略

采用“**Zed 原生 Rust Agent 为内核，吸收 Pi 的优秀设计**”策略：

1. **复用 Zed Agent。** 基于 `agent`、`agent_ui`、`language_model*`、`project` crate，不另建跨进程 Agent。
2. **补齐会话能力。** 参考 Pi 的树形会话、分支、压缩、重试和消息队列，在 Rust 中实现缺失部分。
3. **直接调用语义能力。** Agent 工具通过 `Entity<Project>` 调用定义、引用、诊断等能力，不经过本地 socket、ACP 或 JSONL：
   - `ide_go_to_definition`
   - `ide_find_references`
   - `ide_hover`
   - `ide_workspace_symbols`
   - `ide_diagnostics`
   - `ide_code_actions`
   - 后续再开放 `ide_rename_symbol`
4. **统一原生体验。** 上下文选择、Diff 审批、权限、诊断回路和 Agent 状态均使用 GPUI 组件。
5. **保留可选 ACP。** Zed 原有外部 Agent 能力可以保留兼容，但不是 CodeIDE 默认 Agent，也不参与核心实现。

### 4.2 Rust 模块边界

优先扩展或封装 Zed 现有 crate，避免平行实现同一套状态：

- `codeide_agent`：Agent loop、上下文、队列、压缩与会话协调；
- `codeide_agent_tools`：文件、Shell、Git、LSP 语义工具；
- `codeide_context`：selection anchor、Context Chip、快照和 Token 预算；
- `codeide_changeset`：ChangeTransaction、hunk、三方比较和撤销；
- `codeide_permissions`：命令、路径、网络权限与审计；
- `codeide_config`：TOML schema、分层合并、注释保留、热加载和迁移；
- `codeide_sessions`：树形会话、迁移、恢复和持久化；
- `codeide_model`：内置/自定义供应商、Base URL、Keychain 认证、模型能力、上下文预算、流式传输、重试与用量；
- `codeide_agent_ui`：结构化消息 Block、Agent Panel、Diff 审批和状态展示；
- `codeide_git_workflow`：staged diff 摘要、Commit 生成、hooks 状态和 ChangeSet 关联。

若 Zed 已有等价能力，应向现有 crate 增量贡献，而不是创建 `codeide_*` 重复层。Rust 代码避免无边界的 `serde_json::Value` 传播；网络和持久化不得阻塞 GPUI 主线程；长任务必须支持取消。

### 4.3 语义工具设计原则

- 工具参数使用工作区相对路径和精确文本位置，并明确 UTF-16/LSP 坐标转换；
- 返回结构化 Location、Range、Diagnostic，不返回难以解析的纯文本；
- 所有路径 canonicalize 后必须位于已授权工作区；
- 读工具支持 timeout、cancel 和 LSP readiness 状态；
- LSP 未就绪、文件未索引、语言服务器崩溃时返回可诊断错误；
- 写操作与读操作分权，rename/code action 必须进入 Diff/审批流程；
- 可参考 Zed 原生工具的 `SymbolLocator`，但正式接口优先使用精确字符位置，避免同一行重名歧义。

## 5. 语言环境规划

语义跳转和引用主要由 **LSP** 提供，Tree-sitter 只负责语法与结构能力，二者不能混为一谈。

| 优先级 | 语言 | Tree-sitter | 默认 LSP | 格式化/检查 | 调试 |
|---|---|---|---|---|---|
| P0 | TypeScript/JavaScript | Zed 内置 | vtsls | Prettier / ESLint | vscode-js-debug |
| P0 | Python | Zed 内置 | basedpyright + Ruff | Ruff | debugpy |
| P0 | Go | Zed 内置 | gopls | gofmt/gopls | Delve |
| P0 | Rust | Zed 内置 | rust-analyzer | rustfmt / cargo check | CodeLLDB |
| P1 | Java | 扩展/内置配置 | JDTLS | Spotless/项目配置 | Java DAP |
| P1 | C/C++ | Zed 内置 | clangd | clang-format | CodeLLDB/GDB |
| P1 | Vue/Svelte | Zed 支持 | 对应 LSP + TS | Prettier/ESLint | JS Debug |

### 5.1 工具链管理

- **系统版本优先，托管版本兜底**，避免重复下载和 PATH 混乱；
- Python 优先识别 `.venv`、`uv`、Poetry；
- Node 识别项目 `packageManager`、`.nvmrc`、`.node-version`；
- Go 读取 `go.mod` / `go.work`；Rust 读取 `rust-toolchain.toml`；
- 下载的 LSP/DAP 必须有版本、来源、校验和及更新通道；
- 设置页显示每种语言的 Grammar、LSP、Formatter、Debugger 状态；
- 提供“一键诊断”，但不静默修改项目依赖或全局环境；
- 大型项目提供内存、索引范围和 check-on-save 降载配置。

### 5.2 语言能力验收用例

每种 P0 语言维护一个小型多文件 fixture，统一验证：

1. 函数、类型、属性、局部变量跳到定义；
2. 查找跨文件引用；
3. 重命名后所有引用正确更新；
4. 修改代码后诊断在目标时间内刷新；
5. Workspace Symbol 可搜索类和函数；
6. 格式化、Code Action 与测试/调试入口可用；
7. Agent 能通过语义工具找到定义和引用，而不只依赖 grep。

## 6. 安全与稳定性

### 6.1 权限模型

默认分为四级：

- **自动允许**：读取工作区文件、只读语义查询；
- **预览后允许**：创建/修改/删除工作区文件、批量重命名；
- **每次确认**：Shell、网络访问、工作区外读取；
- **默认拒绝**：工作区外写入、读取密钥目录、提权命令、破坏性 Git 操作。

补充措施：

- `.env`、SSH、云凭证、Keychain 等敏感路径保护；
- 命令、文件改动、授权决定写入本地审计日志；
- 支持“一次允许 / 本会话允许 / 拒绝”；
- 模型请求只获得明确选择的上下文，不自动上传整个工作区；
- Shell 工具后续可支持容器或轻量 VM 沙箱，但不作为 MVP 唯一执行方式。

### 6.2 稳定性目标

- Agent panic 必须被任务边界隔离，错误不应导致整个编辑器退出；
- Agent 中止应在 1 秒内反馈到 UI，流式请求和工具任务必须可取消；
- LSP 崩溃自动退避重启，避免无限重启循环；
- 模型网络请求有超时、取消和最大响应限制；
- 文件写入使用原子操作，Diff 应基于版本号检测并发修改；
- 会话和设置使用 schema version，升级前备份；
- 离线状态下编辑、LSP、终端等非 AI 能力正常可用。

## 7. 仓库与上游维护策略

当前仓库是 Vela 产品源码及文档的唯一来源；官方 Zed 通过只读 `upstream` remote 同步，`../pi` 仅用于设计调研。Agent 与 Project/LSP 通过进程内 Rust API 集成：

```text
vela/
  docs/vela/                    # Vela 架构、路线图、ADR、来源记录
  crates/codeide_agent/         # Agent loop、上下文和协调
  crates/codeide_agent_tools/   # 文件、Shell、Git、语义工具
  crates/codeide_context/       # 代码选择与多轮上下文
  crates/codeide_changeset/     # 改动账本、Diff 与撤销
  crates/codeide_permissions/   # 权限策略与审计
  crates/codeide_config/        # TOML 配置、合并、迁移和监听
  crates/codeide_sessions/      # 会话、分支、压缩和迁移
  crates/codeide_model/         # 模型供应商与认证
  crates/codeide_agent_ui/      # GPUI Agent UI 与 Diff 审批
  crates/codeide_git_workflow/  # Commit 生成与快捷提交流程
  config/                       # 默认设置和语言配置
  fixtures/languages/           # P0 语言验收工程
  scripts/                      # bootstrap、build、test、package
  upstream.lock                 # Zed commit 与参考 Pi commit
```

版本控制原则：

- CodeIDE 基于 Zed fork 维护，Pi 不作为运行时依赖；
- `upstream.lock` 固定已验证的 Zed commit，不跟随 latest 自动构建；
- 优先复用 Zed 已有 crate 和抽象，避免复制 Agent 状态；
- 每项 Zed 核心改动记录原因、关联测试和可上游化方案；
- 移植 Pi 的具体实现时记录原文件、commit 和 MIT 许可证；
- 建立定期上游同步流程，而不是长期积累一次性大合并。

## 8. 里程碑

### M0：可行性验证（1 周）

- 在当前 macOS arm64 环境构建并运行 Zed；
- 梳理 Zed 原生 Agent、模型、会话、工具与 UI crate 的调用边界；
- 用 Zed 原生 Agent 完成一次模型流式对话和受控文件修改；
- 打开 P0 fixture，验证人工定义跳转和引用查找；
- 输出 ADR-001：进程内 Rust Agent 的 crate 边界与最小 fork 改动。

**退出条件：** Zed 原生 Rust Agent 可完成一次受控文件修改；P0 至少两种语言的 LSP 跳转可用。

### M1：稳定编辑器基线（1–2 周）

- 固定上游 commit 和构建脚本；
- 品牌、默认设置和开发/发布构建分离；
- 完整 Project Panel 文件树及状态标记；
- 注册首批 Action，完成快捷键 UI、冲突检测和 TOML 持久化；
- UI-first 设置页和 TOML 配置 facade；
- 全局/项目配置分层、注释保留、原子写入与热加载；
- P0 语言环境状态页与 fixture；
- 崩溃日志、基础性能指标和 smoke test。

**退出条件：** 四种 P0 语言的导航、引用、诊断和格式化自动化验收通过。

### M2：Rust Agent 深度集成（2–3 周）

- 加固进程内 Agent loop、流式事件和取消边界；
- 实现结构化聊天 Block 与长列表虚拟化；
- 模型登录/选择、自定义 Base URL + Key、每模型上下文窗口与预算、连接测试、会话恢复、中止与错误 UI；
- 实现 ChangeTransaction、聊天改动卡片和按 hunk Diff 审批；
- 实现 Turn/Working Tree/Branch 三类 Diff 和可配置 Base Picker；
- 实现基于 staged diff 的 Commit 信息生成、ReviewAndCommit 和 ChangeSet 关联；
- 实现 selection Context Chip、pinned/once 与 stale 处理；
- 命令和路径权限策略；
- Agent 任务异常隔离与会话恢复。

**退出条件：** Agent 可连续完成“读取 → 修改 → 测试 → 诊断 → 修复”，所有写操作可审计和撤销。

### M3：IDE 语义工具（2–3 周）

- 实现定义、引用、Hover、Workspace Symbol、诊断工具；
- 在 Rust Agent Tool Registry 注册结构化语义工具；
- 加入 timeout/cancel/path containment；
- 语义工具与 grep 的效果对比评测。

**退出条件：** 四种 P0 语言中，Agent 对跨文件符号任务优先使用 LSP，并达到预设成功率。

### M4：Beta 发布（2 周）

- 签名、打包、自动更新通道；
- License/NOTICE/源码提供流程；
- 冷启动、内存、大仓库和长会话测试；
- 安装失败、LSP 失败、模型失败的用户可理解诊断。

## 9. 测试与质量门槛

### 9.1 自动化层次

- 单元测试：Agent 状态转换、selection anchor、ChangeSet、merge-base/direct ref 解析、Commit prompt 输入边界、快捷键冲突、路径权限、Diff、TOML round-trip/合并/迁移、会话迁移；
- Contract Test：内置/自定义 Base URL 的模型流事件、上下文能力缺失/覆盖/切换、认证脱敏、工具 schema 与持久化格式；
- Integration Test：真实 LSP + fixture；
- E2E：文件树操作、UI 修改配置并验证 TOML、选择代码多轮提问、Agent 修改、点击聊天改动打开 Turn Diff、切换 Branch Diff 基准、按 hunk 接受/拒绝、生成并编辑 Commit 信息、快捷提交、诊断刷新；
- Soak Test：8 小时会话、反复中止、LSP 崩溃与 Agent 任务 panic 注入；
- 性能测试：冷启动、首个补全、定义跳转、首次 Agent token、内存峰值。

### 9.2 初始 SLO

| 指标 | MVP 目标 |
|---|---|
| 编辑输入可见延迟 | p95 < 16 ms |
| 已预热定义跳转 | p95 < 500 ms |
| Agent 停止反馈 | p95 < 1 s |
| Agent 任务异常后 UI 恢复可用 | < 2 s |
| 会话恢复成功率 | > 99%（正常关闭场景） |
| P0 fixture 语义测试 | 100% 通过 |

性能目标需要在 M0/M1 获取基线后调整，不能为了达标隐藏失败。

## 10. 关键风险与应对

| 风险 | 影响 | 应对 |
|---|---|---|
| Zed 上游变化快、fork 冲突 | 升级成本高 | 复用现有 crate、少改核心、固定版本、定期小步同步 |
| 进程内 Agent 故障影响 IDE | 卡顿或崩溃 | 任务隔离、取消、超时、panic 边界、压力测试 |
| Pi 功能需用 Rust 补齐 | 初期开发量增加 | 只移植经过验证且 Zed 缺失的能力，分阶段实现 |
| TOML facade 与 Zed Settings 不一致 | 配置显示或生效错误 | 强类型映射、来源追踪、round-trip 与 E2E 测试 |
| 自定义 Base URL 泄漏 Key | 凭证安全风险 | Keychain、同源重定向限制、Header 脱敏和安全测试 |
| 服务未声明上下文窗口 | 请求溢出或过早压缩 | 每模型显式配置、来源展示、未知时发送前阻止 |
| Agent 工具权限不足 | 误操作和凭证泄漏 | 工具入口权限、路径限制、审计、Diff 审批、可选沙箱 |
| 多语言环境差异大 | “能高亮但不能跳转” | P0 fixture、状态页、版本固定、一键诊断 |
| GPL 分发义务 | 发布风险 | 发布前完成许可证审查、源码提供和 NOTICE 流程 |
| 大仓库 LSP/AI 资源过高 | 卡顿、OOM | 索引范围、资源上限、退避、可关闭 check-on-save |

## 11. 待确认决策

1. 产品仅供个人内部使用，还是计划公开分发？这会影响品牌、签名、更新和 GPL 合规工作。
2. 首发只做 macOS，还是同时要求 Linux/Windows？
3. 是否保留 Zed 原有 ACP 外部 Agent 入口，还是产品层隐藏该入口？
4. 模型认证是直接沿用 Zed Provider 设置，还是建立 CodeIDE 统一设置页？
5. 首批语言除 TypeScript/Python/Go/Rust 外，是否必须包含 Java、C++ 或前端框架？
6. 高危工具执行是采用本机确认模式，还是 MVP 就要求容器/VM 沙箱？

## 12. 下一步执行清单

按以下顺序开始，不先做 UI 大改：

1. 创建 `upstream.lock`，记录当前 Zed commit、参考 Pi commit、工具链和平台；
2. 安装 Rust `1.95.0` 和 Zed 的 macOS 构建依赖，完成首次构建；
3. 阅读并画出 Zed `agent`、`agent_ui`、`language_model*`、`project` 的进程内依赖图；
4. 运行 Zed 原生 Agent，验证模型流、取消、工具和会话持久化；
5. 创建四种 P0 语言 fixture 和统一的语义验收表；
6. 编写 ADR-001，确定应扩展的 Zed crate 与 CodeIDE 新 crate；
7. 编写 ADR-002，确定 TOML schema、配置层级和 Zed Settings 映射；
8. 在任何 Agent 写入能力上线前，先实现权限策略和 Diff 审批原型。
