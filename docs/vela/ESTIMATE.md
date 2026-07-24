# CodeIDE 工期预估

> 基于 `docs/vela/PROJECT_PLAN.md` Draft v0.1  
> 估算基线：Zed `6297c88f42`、Pi `9b3a2059`  
> 估算精度：早期 ROM（量级估算），误差约 `-25% / +40%`

## 1. 结论

在**直接 fork Zed、AI Runtime 全部使用进程内 Rust、macOS arm64 首发**的前提下：

| 交付范围 | 1 名熟悉 Rust 的全职开发者 | 3 人小组 | 说明 |
|---|---:|---:|---|
| 技术验证版 | 4–6 周 | 3–4 周 | 能编译、聊天、选择代码、产生基础 Diff |
| 可日用 Alpha | 4–6 个月 | 2.5–3.5 个月 | macOS、P0 语言、核心 UI 流程可用 |
| 稳定 Beta | 10–15 个月 | 5–7 个月 | 权限、恢复、TOML 配置、测试和打包较完整 |
| 跨平台正式版 | 16–24 个月 | 8–12 个月 | macOS/Linux/Windows、升级和长期稳定性 |

上述 3 人小组建议配置：

- 1 人：Zed/GPUI、编辑器、Diff、文件树；
- 1 人：Rust Agent、模型供应商、上下文和权限；
- 1 人：配置、Git、语言环境、自动化测试与发布。

如果开发者此前没有维护过 Zed/GPUI，大约再增加 **20%–35%**。兼职开发则不能按人数线性折算。

## 2. 为什么不是从零开发

以下能力 Zed 已经具备，可直接复用：

- 编辑器、完整文件树、工作区和 Git 基础；
- Tree-sitter、LSP、定义跳转、引用、重命名和诊断；
- Agent Panel、原生 Rust Agent、模型抽象和工具框架；
- Diff 编辑器、终端、任务、DAP 和语言扩展；
- GPUI、设置系统和跨平台应用框架。

因此项目不是“开发一个新 IDE”，而是“维护一个有较深定制的 Zed fork”。主要成本来自把现有能力连接成要求的产品流程，以及长期跟进 Zed 上游。

## 3. 功能拆分估算

单位为**人周**，包含实现、代码审查、自测和基础文档，不包含大规模返工。

| 模块 | 乐观 | 常规 | 悲观 | 说明 |
|---|---:|---:|---:|---|
| Zed 构建、fork、品牌和开发脚本 | 1 | 2 | 3 | 首次熟悉 workspace 成本较高 |
| Zed Agent 架构梳理与进程内内核调整 | 2 | 4 | 7 | 不使用 Pi 子进程会增加 Rust 实现量 |
| 结构化聊天 Block 与流式 UI | 2 | 4 | 6 | 复用 Agent Panel，但需结果卡片和虚拟化 |
| 代码选择、多轮 Context Chip、anchor/stale | 2 | 4 | 6 | 多 selection 和会话恢复是难点 |
| ChangeSet、Turn Diff、按 hunk 接受/拒绝 | 3 | 6 | 9 | 并发编辑、三方比较和撤销最容易低估 |
| 文件树增强和聊天联动 | 1 | 2 | 4 | 文件树主体直接复用 Zed |
| Branch Diff Base Picker | 2 | 3 | 5 | branch/tag/SHA、merge-base、失效 ref |
| Commit 生成、快捷提交和 hooks UI | 2 | 3 | 5 | 默认只使用 staged diff |
| Action、快捷键 UI、冲突检测 | 1 | 2 | 4 | 基于 Zed action/keymap 扩展 |
| TOML 配置 facade | 4 | 7 | 11 | 全局/项目分层、注释保留、热加载、迁移 |
| 自定义 Base URL + Key | 2 | 3 | 5 | Keychain、协议差异、连接测试和脱敏 |
| 每模型上下文窗口与 Token 预算 | 1 | 2 | 4 | tokenizer 估算和压缩边界需要测试 |
| 权限、敏感路径和审计 | 3 | 5 | 8 | Shell/文件/网络权限不能后补 |
| P0 语言环境与 fixture | 2 | 4 | 6 | TS/Python/Go/Rust，主体复用 Zed |
| 会话恢复、迁移和异常隔离 | 2 | 4 | 7 | 包括未处理 ChangeSet 恢复 |
| 自动化测试、性能与 soak test | 4 | 7 | 11 | 稳定 Beta 的主要成本之一 |
| macOS 签名、打包、升级与许可证 | 2 | 4 | 7 | 正式分发前必须完成 |

模块直接相加约为 **38–108 人周**，常规值约 **68 人周**。技术验证后可确认哪些 Zed 能力能直接复用，从而删除部分工作；多人并行可以缩短日历时间，但不会等比例减少总人周。稳定 Beta 建议按 **55–75 有效人周（已含主要集成缓冲）**规划。

## 4. 推荐排期

### Phase 0：技术验证，4–6 周（单人）

- 构建 Zed fork；
- 跑通原生 Rust Agent 和一个模型；
- 选择代码加入聊天；
- Agent 修改文件并打开基础 Turn Diff；
- 验证 TS、Python、Go、Rust 的定义和引用；
- 形成 Agent、Settings、Diff 三份 ADR。

这一阶段完成后再重新估算，误差可收敛到约 `±20%`。

### Phase 1：可日用 Alpha，再投入 12–18 周（单人）

- 结构化聊天结果；
- Context Chip 和多轮选择；
- ChangeSet 与 Diff 审批；
- Branch Diff Base Picker；
- 自定义 Base URL、Key 和上下文大小；
- 基础权限；
- 文件树/Git/快捷操作联动；
- macOS 开发构建。

可压缩项：TOML 先覆盖 CodeIDE 新配置，不一次性映射 Zed 全部设置。

### Phase 2：稳定 Beta，再投入 16–24 周（单人）

- 完整 TOML UI facade；
- 三方冲突与恢复；
- 权限、审计和敏感信息保护；
- Commit/hook/signing 完整流程；
- 长会话、异常注入和大仓库性能；
- 签名、打包、更新、许可证和迁移。

### Phase 3：跨平台正式版，再投入 12–20 周（单人）

- Linux/Windows 差异处理；
- Keychain、快捷键、路径和 Shell 兼容；
- 安装器、签名、升级回滚；
- 上游同步和跨平台 E2E。

## 5. 最大工期风险

### 5.1 TOML 替代用户 JSON 配置

这是当前需求中最容易被低估的模块。Zed 已有成熟的 JSON/JSONC Settings 体系。如果要求所有现有 Zed 设置都通过新 UI 和 TOML 双向映射，需要处理 schema、默认值、来源、语言覆盖、热加载、注释保留和迁移。

建议 Alpha 只让 CodeIDE 新能力使用 TOML；Beta 再逐步覆盖高频 Zed 设置。这样可以节省首版约 **3–6 周**。

### 5.2 可靠的 ChangeSet

“点击聊天改动打开对应 Diff”本身不难，难的是用户和 Agent 同时继续修改后，仍能准确接受、拒绝或撤销单个 hunk。需要 buffer revision、anchor、快照和三方比较，常规至少 **4–6 周**。

### 5.3 进程内 Agent

不使用子进程能获得更自然的 Zed/LSP 集成，但 Agent 卡死、panic、超大工具结果和取消失败会直接影响 IDE。必须投入任务隔离、超时、内存边界和异常测试。

### 5.4 Zed 上游同步

深度修改 `agent`、`settings`、`git`、`project` 和 `agent_ui` 会增加同步成本。建议预留长期开发时间的 **15%–20%** 用于跟进上游和回归测试。

## 6. 缩短首版的方法

若目标是尽快得到可用版本，建议 Alpha 暂时这样裁剪：

1. 只支持 macOS arm64；
2. 直接复用 Zed 文件树和大部分 Git UI；
3. TOML 先管理 CodeIDE 新设置，不替换全部 Zed 设置；
4. 模型协议先支持 OpenAI Responses、OpenAI Chat、Anthropic 三种；
5. ChangeSet 首版支持按文件接受/拒绝，第二阶段再做复杂三方 hunk；
6. Commit 只基于 staged diff，不在首版加入自动 push/rebase；
7. P0 保留 TS/Python/Go/Rust，其他语言沿用 Zed 原有能力但不承诺专项验收。

按这个范围，经验丰富的单人开发者可争取在 **14–20 周**交付可日用 Alpha；3 人小组约 **8–12 周**。稳定 Beta 建议保留约 **5–7 个月（三人）**。

## 7. 估算前提

本估算假设：

- Zed 当前版本可在目标机器正常构建；
- 不重写编辑器、LSP、Git 和 Diff 引擎；
- 不开发云端账号、同步和计费后端；
- 不包含多人实时协作定制；
- 模型调用由用户自己的 API Key 或 Base URL 提供；
- 产品允许遵守 Zed GPL 的分发要求；
- AI Runtime 不使用子进程，但 LSP/DAP/终端等标准工具仍可使用外部进程。
