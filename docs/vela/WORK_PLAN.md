# CodeIDE 工作计划

> 依据：`docs/vela/PROJECT_PLAN.md`、`docs/vela/ESTIMATE.md`  
> 目标：先交付 macOS arm64 可日用 Alpha，再完成稳定 Beta  
> 技术路线：Zed fork + 进程内 Rust Agent + UI-first + TOML 配置

## 1. 计划基准

本计划按 **3 人全职团队、约 26–30 周交付稳定 Beta** 编排：

- **A：Editor/UI**——Zed、GPUI、文件树、聊天、Diff；
- **B：Agent/Model**——Rust Agent、模型供应商、上下文、语义工具；
- **C：Platform/Quality**——TOML、Git、权限、语言环境、测试与发布。

单人执行时按依赖顺序串行，预计 10–15 个月。所有周期都以验证结果为准，不以牺牲权限、数据安全和恢复能力赶进度。

## 2. 交付阶段

| 阶段 | 周期（三人） | 主要交付物 | 里程碑 |
|---|---:|---|---|
| P0 项目基线 | 第 1–2 周 | 可重复构建的 Zed fork | Build Green |
| P1 技术验证 | 第 3–5 周 | 进程内 Agent、选择代码、基础 Diff | Architecture Proven |
| P2 编辑器与配置基线 | 第 6–9 周 | 文件树、Action、TOML facade | Editor Baseline |
| P3 聊天与多轮上下文 | 第 10–13 周 | 结构化聊天、Context Chip | Conversation Ready |
| P4 ChangeSet 与 Diff | 第 14–18 周 | Turn Diff、hunk 审批、恢复 | Safe Editing |
| P5 模型与语义能力 | 第 19–22 周 | Base URL/Key、Token 预算、LSP 工具 | Alpha |
| P6 Git 工作流 | 第 23–25 周 | Branch Diff、Commit 生成、快捷提交 | Git Complete |
| P7 稳定化与发布 | 第 26–30 周 | 权限、性能、安装包、迁移 | Beta |

## 3. 详细任务

### P0：项目基线（第 1–2 周）

#### 工作项

- [ ] `BASE-001` 建立 CodeIDE 的 Zed fork 与上游 remote；
- [x] `BASE-002` 创建 `upstream.lock`，固定 Zed/Pi 调研 commit 和工具链；
- [x] `BASE-003` 安装 Rust 1.95.0、Xcode、cmake 等 macOS 构建依赖；
- [x] `BASE-004` 完成 debug/release 构建脚本；
- [ ] `BASE-005` 建立格式化、Clippy、单元测试和 smoke test CI；
- [ ] `BASE-006` 明确 GPL/MIT 来源记录和第三方许可证流程；
- [ ] `BASE-007` 建立 issue、ADR、变更日志与上游补丁模板。

#### 退出条件

- 新机器按文档可以完成构建；
- Debug 应用能打开一个 Rust/TS/Python/Go 工程；
- CI 对主分支持续通过；
- 没有未记录的上游源码复制。

---

### P1：技术验证（第 3–5 周）

#### Agent 与模型

- [x] `SPIKE-001` 梳理 `agent`、`agent_ui`、`language_model*`、`project` 依赖图；
- [x] `SPIKE-002` 使用 Zed 原生 Rust Agent 跑通一次流式模型请求；
- [x] `SPIKE-003` 验证取消、超时、错误和 UI 恢复；
- [x] `SPIKE-004` 验证 Agent 直接调用 `Entity<Project>`；
- [x] `SPIKE-005` 形成 ADR-001：进程内 Agent crate 边界。

#### 编辑体验

- [ ] `SPIKE-006` 将当前 selection 加入 Agent 消息；
- [ ] `SPIKE-007` Agent 修改一个文件并打开修改前后 Diff；
- [ ] `SPIKE-008` 聊天中的文件路径可跳转到编辑器；
- [x] `SPIKE-009` 验证 Zed 文件树、Git Panel、Diff Editor 的可复用范围。

#### 语言能力

- [x] `SPIKE-010` 建立 TS、Python、Go、Rust fixture；
- [ ] `SPIKE-011` 验证定义、引用、重命名、诊断和格式化；
- [ ] `SPIKE-012` Agent 调用一次定义和诊断工具。

#### 退出条件

完成“选择函数 → 发起对话 → Agent 修改 → 打开 Diff → 用户确认”的纵向链路。若该链路无法稳定实现，暂停 UI 扩展并重新评审架构。

---

### P2：编辑器与配置基线（第 6–9 周）

#### 文件树

- [ ] `TREE-001` 保留完整多根工作区文件树；
- [ ] `TREE-002` 支持文件/目录创建、移动、重命名和删除；
- [ ] `TREE-003` 显示 Git、诊断和 AI ChangeSet 状态；
- [ ] `TREE-004` 增加“加入对话”“解释”“修改”“测试”菜单；
- [ ] `TREE-005` 聊天、编辑器和文件树双向定位。

#### Action 与快捷键

- [ ] `ACTION-001` 注册稳定的 `codeide::*` Rust Action；
- [ ] `ACTION-002` 建立跨平台快捷键冲突检测；
- [ ] `ACTION-003` 提供 Action 搜索、按键录制和恢复默认 UI；
- [ ] `ACTION-004` 将结果持久化到 `keybindings.toml`。

#### TOML 配置

- [x] `CFG-001` 形成 ADR-002：TOML schema 和 Zed Settings 映射；
- [ ] `CFG-002` 实现全局、项目和会话层级合并；
- [ ] `CFG-003` 使用 `toml_edit` 保留注释、顺序和未知字段；
- [ ] `CFG-004` 实现校验、临时文件、fsync 和原子 rename；
- [ ] `CFG-005` 实现文件监听、热加载和冲突 UI；
- [ ] `CFG-006` 设置 UI 显示默认值、生效值和来源；
- [ ] `CFG-007` 增加 schema version、备份和迁移报告；
- [ ] `CFG-008` 第一阶段覆盖 CodeIDE 新配置及高频 Zed 设置。

#### 退出条件

用户无需手写文件即可完成设置和快捷键修改；磁盘只生成 CodeIDE TOML 用户配置，重启后配置一致。

---

### P3：聊天与多轮上下文（第 10–13 周）

#### 结构化聊天

- [ ] `CHAT-001` 定义 Text/Code/Tool/Diagnostic/Command/FileChange/Progress Block；
- [ ] `CHAT-002` 流式更新单个 Block，避免重建消息列表；
- [ ] `CHAT-003` 长输出折叠、截断和虚拟化；
- [ ] `CHAT-004` 工具状态、耗时、权限和取消可视化；
- [ ] `CHAT-005` 聊天内容中的路径、代码和诊断可点击；
- [ ] `CHAT-006` 会话创建、命名、恢复和分支 UI。

#### 代码选择上下文

- [ ] `CTX-001` selection 保存路径、revision、anchor、快照和 symbol；
- [ ] `CTX-002` 输入区显示可删除的 Context Chip；
- [ ] `CTX-003` 支持 `once` 和 `pinned` 生命周期；
- [ ] `CTX-004` 支持多个不连续 selection；
- [ ] `CTX-005` buffer 修改后通过 anchor 重定位；
- [ ] `CTX-006` 无法重定位时标记 stale 并显示差异；
- [ ] `CTX-007` 会话恢复时恢复 Context Chip 和状态。

#### 退出条件

同一段选中代码可以连续完成“解释 → 重构 → 补测试”，无需重复粘贴，且代码变化后不会静默引用错误位置。

---

### P4：ChangeSet 与 Diff（第 14–18 周）

#### ChangeSet 数据模型

- [ ] `CHG-001` 定义 ChangeTransaction、ChangeSetId、FileChange 和 Hunk；
- [ ] `CHG-002` 所有 Agent 写操作必须经过 ChangeTransaction；
- [ ] `CHG-003` 记录 buffer revision、修改前快照、turn 和 tool call；
- [ ] `CHG-004` 一个 turn 的多文件改动归属同一 ChangeSet；
- [ ] `CHG-005` ChangeSet 与会话一起持久化和恢复。

#### Diff 审批

- [ ] `DIFF-001` 聊天 FileChangeBlock 打开准确 Turn Diff；
- [ ] `DIFF-002` 支持按文件和 hunk 接受、拒绝、撤销；
- [ ] `DIFF-003` 文件树同步显示待审批状态；
- [ ] `DIFF-004` 用户继续编辑时执行三方比较；
- [ ] `DIFF-005` 冲突时显示明确 UI，不静默覆盖；
- [ ] `DIFF-006` Diff 与源文件、聊天 turn 双向跳转。

#### 退出条件

通过并发编辑测试：Agent 修改后，用户手动修改同一区域，再接受/拒绝 hunk，不丢失用户代码；崩溃恢复后未审批 ChangeSet 仍可处理。

---

### P5：模型与语义能力（第 19–22 周）

#### 自定义模型供应商

- [ ] `MODEL-001` UI 添加自定义供应商；
- [ ] `MODEL-002` 支持 OpenAI Responses；
- [ ] `MODEL-003` 支持 OpenAI Chat Completions；
- [ ] `MODEL-004` 支持 Anthropic Messages；
- [ ] `MODEL-005` 配置 Base URL、模型 id、非敏感 Header 和代理；
- [ ] `MODEL-006` API Key/Token 写入系统 Keychain；
- [ ] `MODEL-007` TOML 只保存 credential id；
- [ ] `MODEL-008` 实现测试连接及 DNS/TLS/HTTP/协议错误展示；
- [ ] `MODEL-009` 限制跨 origin 重定向并完成日志脱敏。

#### 上下文窗口

- [ ] `TOKEN-001` 每个模型配置 context window、max output、margin 和 compact threshold；
- [ ] `TOKEN-002` 实现 manual/provider/catalog/unknown 来源优先级；
- [ ] `TOKEN-003` 未知上下文模型发送前要求用户配置；
- [ ] `TOKEN-004` 统计系统提示、消息、选择、工具和结果预算；
- [ ] `TOKEN-005` 输入区显示使用量、剩余量和压缩阈值；
- [ ] `TOKEN-006` 切换模型后重新计算预算；
- [ ] `TOKEN-007` context overflow 可修正配置后重试。

#### LSP 语义工具

- [ ] `SEM-001` go to definition；
- [ ] `SEM-002` find references；
- [ ] `SEM-003` hover 和 workspace symbols；
- [ ] `SEM-004` diagnostics 和 code actions；
- [ ] `SEM-005` timeout、cancel、path containment 和 readiness；
- [ ] `SEM-006` rename 进入 Diff 审批，不直接静默写入。

#### Alpha 退出条件

- macOS arm64 可连续日用；
- 四种 P0 语言验收通过；
- 自定义 Base URL + Key + context window 可从 UI 完成；
- Agent 能完成“读取 → 语义定位 → 修改 → 测试 → 诊断 → 修复”；
- 所有改动可审查、拒绝和恢复。

---

### P6：Git 工作流（第 23–25 周）

#### Branch Diff

- [ ] `GIT-001` 明确 Turn/Working Tree/Branch 三类 Diff；
- [ ] `GIT-002` Base Picker 支持 local/remote branch、tag 和 SHA；
- [ ] `GIT-003` 支持 upstream 自动模式；
- [ ] `GIT-004` 支持 merge-base 和 direct 模式；
- [ ] `GIT-005` workspace、会话和 Diff Tab 级覆盖；
- [ ] `GIT-006` ref 失效时明确报错，不回退默认分支；
- [ ] `GIT-007` 切换 base 不执行 checkout/reset/rebase。

#### Commit

- [ ] `COMMIT-001` 只基于 staged diff 生成 Commit 信息；
- [ ] `COMMIT-002` 支持 Conventional Commits 和仓库历史风格；
- [ ] `COMMIT-003` Git UI 中编辑标题与正文；
- [ ] `COMMIT-004` 执行 hooks 和 signing，展示完整错误；
- [ ] `COMMIT-005` Commit 与 ChangeSet/会话关联；
- [ ] `COMMIT-006` ReviewAndCommit 快捷流程；
- [ ] `COMMIT-007` Commit 与 Push 保持独立；
- [ ] `COMMIT-008` 提供离线模板模式。

#### 退出条件

可选择 `origin/develop`、tag 或 SHA 作为对比基准；生成 Commit 信息不会混入未暂存内容或 Branch Diff 中的历史提交。

---

### P7：稳定化与 Beta（第 26–30 周）

#### 权限与安全

- [ ] `SEC-001` 文件、Shell、网络和敏感路径权限分级；
- [ ] `SEC-002` 一次允许、会话允许、拒绝；
- [ ] `SEC-003` `.env`、SSH、云凭证和 Keychain 路径保护；
- [ ] `SEC-004` 操作审计日志和敏感信息脱敏；
- [ ] `SEC-005` 受保护分支和高危 Git 操作确认；
- [ ] `SEC-006` 自定义 Base URL、重定向和认证安全测试。

#### 稳定性

- [ ] `STAB-001` Agent panic、取消、超时和错误边界；
- [ ] `STAB-002` LSP 崩溃退避重启；
- [ ] `STAB-003` 8 小时长会话 soak test；
- [ ] `STAB-004` 大仓库内存、首 token、Diff 和跳转性能；
- [ ] `STAB-005` 配置、会话和 ChangeSet 故障恢复；
- [ ] `STAB-006` 离线编辑和非 AI 能力回归。

#### 发布

- [ ] `REL-001` macOS 签名和安装包；
- [ ] `REL-002` 更新检查、升级和回滚；
- [ ] `REL-003` License、NOTICE 和对应源码提供流程；
- [ ] `REL-004` 首次启动、模型配置和语言环境引导；
- [ ] `REL-005` 日志导出和隐私过滤；
- [ ] `REL-006` Beta 发布检查表。

#### Beta 退出条件

- P0 fixture、核心 E2E 和安全测试全部通过；
- 无 P0/P1 数据丢失或凭证泄漏问题；
- 会话和 ChangeSet 恢复成功率达到目标；
- 已完成签名、许可证和升级回滚演练。

## 4. 每周工作节奏

- 周一：确认本周目标、依赖和风险；
- 每日：小 PR 合并，避免长期大分支；
- 周三：运行集成测试并同步一次 Zed 上游风险；
- 周五：演示真实用户流程，更新指标和计划；
- 每阶段结束：完成 ADR、退出条件和回归测试后再进入下一阶段。

建议每两周同步一次 Zed 上游；若发生大规模 Agent/Settings 重构，暂停功能开发，先完成兼容评估。

## 5. Definition of Done

一个工作项只有同时满足以下条件才算完成：

- Rust 代码通过 fmt、Clippy 和相关测试；
- UI 有 loading、empty、error、cancelled 状态；
- 可取消的任务不会阻塞 GPUI 主线程；
- 配置通过 UI 操作并正确 round-trip 到 TOML；
- 文件/命令/网络操作经过权限层；
- 敏感字段不会进入日志、TOML 或会话；
- 新 Action 可以在快捷键 UI 中发现和重绑定；
- 用户可见行为有 E2E 或明确的手工验收步骤；
- 对 Zed 核心的修改记录了上游同步影响。

## 6. 首周立即执行

1. 建立 Zed fork 和 `upstream.lock`；
2. 安装 Rust 1.95.0 与 macOS 构建依赖；
3. 完成一次 Zed debug build；
4. 运行 Zed 原生 Agent，记录模型、工具和会话调用链；
5. 创建四种 P0 语言 fixture 目录；
6. 创建 ADR-001 草稿和功能验收 checklist；
7. 建立 CI 的 fmt、Clippy、test 和构建缓存。
