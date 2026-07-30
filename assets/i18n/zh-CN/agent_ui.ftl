# Simplified Chinese (zh-CN) catalog for the agent panel and its surfaces.
#
# Product names (Zed, Git, Codex, Claude Agent, ACP, MCP, WSL2) and file names
# (AGENTS.md) are deliberately left untranslated. Keys already defined in other
# catalogs (cancel, settings, retry, learn-more, …) are reused, not redefined.

## Skills migration

rules-to-skills-migration-rerun-please-double-check-agents-md-and-skills-for-missing-or-duplicated-prompts = 已重新执行「规则迁移为技能」。请检查 AGENTS.md 与技能中是否有缺失或重复的提示词。
rules-have-been-migrated-to-skills = 规则已迁移为技能。
view-docs = 查看文档

## Agent panel

no-active-native-thread-to-copy = 没有可复制的当前原生会话
thread-copied-to-clipboard-base64-encoded = 会话已复制到剪贴板（base64 编码）
open-a-project-to-load-a-thread = 请先打开项目再加载会话
no-clipboard-content-available = 剪贴板中没有内容
clipboard-does-not-contain-text = 剪贴板中的内容不是文本
failed-to-decode-clipboard-content-expected-base64 = 无法解码剪贴板内容（应为 base64）
failed-to-parse-thread-data-from-clipboard = 无法解析剪贴板中的会话数据
thread-loaded-from-clipboard = 已从剪贴板加载会话
no-active-thread = 没有活动会话
thread-metadata-store-not-available = 会话元数据存储不可用
no-metadata-found-for-active-thread = 未找到当前会话的元数据
title-generation-failed-click-to-retry = 标题生成失败。点击重试。
edit-thread-title = 编辑会话标题
no-model-is-configured-for-summarizing-thread-titles = 尚未配置用于总结会话标题的模型。
toggle-agent-menu = 切换智能体菜单
current-thread = 当前会话
regenerate-thread-title = 重新生成会话标题
open-thread-as-markdown = 以 Markdown 打开会话
add-server-menu = 添加服务器…
install-new-servers = 安装新服务器…
context = 上下文
open-global-rules = 打开全局规则
open-project-rules = 打开项目规则
profiles = 配置档
toggle-threads-sidebar = 切换会话侧边栏
reauthenticate = 重新认证
add-more-agents = 添加更多智能体
selected-agent = 已选智能体
disable-full-screen = 退出全屏
enable-full-screen = 进入全屏
new-agent-thread = 新建 { $agent } 会话
new-thread-menu = 新建会话…
new-thread = 新建会话
start-new-thread = 新建会话
start-new-agent-thread = 新建智能体会话

## Conversation view

external-agents-are-not-yet-supported-in-shared-projects = 共享项目暂不支持外部智能体。
loading-or-resuming-sessions-is-not-supported-by-this-agent = 此智能体不支持加载或恢复会话。
upgrade-agent = 升级 { $agent }
failed-to-install-agent = 安装 { $agent } 失败
agent-exited = { $agent } 已退出
error-loading-agent = 加载 { $agent } 出错
waiting-for-tool-confirmation = 等待工具确认
waiting-for-input = 等待输入
finished-running-tools = 工具运行完毕
new-message = 新消息
name-refused-to-respond-to-this-request = { $name } 拒绝回应此请求
agent-stopped-due-to-an-error = 智能体因出错而停止
authenticating-to-agent = 正在向 { $agent } 认证…
authenticate-to-agent = 向 { $agent } 认证
choose-one-of-the-following-authentication-options = 请选择以下认证方式之一：
failed-to-install = 安装失败
server-exited-with-status-status = 服务器退出，状态码 { $status }
failed-to-launch = 启动失败
upgrade-agent-to-work-with-zed = 升级 { $agent } 以配合 Zed 使用
currently-using-path-which-does-not-report-a-valid-version = 当前使用 { $path }，它没有输出有效的 --version
currently-using-path-which-is-only-version-version-need-at-least-minimum = 当前使用 { $path }，版本仅为 { $version }（至少需要 { $minimum }）
the-model = 该模型
copy-error-message = 复制错误信息
message-the-agent-to-include-context-for-commands = 给 { $agent } 发消息，@ 引用上下文，/ 使用命令
message-agent-to-include-context-for-commands = 给 { $agent } 发消息 — @ 引用上下文，/ 使用命令
message-agent-to-include-context = 给 { $agent } 发消息 — @ 引用上下文
edit-message-to-include-context = 编辑消息 － @ 引用上下文

## Thread view: feedback and retries

what-went-wrong-share-your-feedback-so-we-can-improve = 哪里出了问题？欢迎反馈，帮助我们改进。
copy-code = 复制代码
thanks-for-your-feedback = 感谢你的反馈！
helpful-response = 有帮助
rating-the-thread-sends-all-of-your-current-conversation-to-the-zed-team = 评价会话会将你当前的全部对话发送给 Zed 团队。
we-appreciate-your-feedback-and-will-use-it-to-improve-in-the-future = 感谢你的反馈，我们会用它持续改进。
not-helpful-response = 没帮助
retrying-next-attempt-in-1-second = 正在重试。1 秒后进行下一次尝试。
retrying-next-attempt-in-seconds-seconds = 正在重试。{ $seconds } 秒后进行下一次尝试。
retrying-next-attempt-in-1-second-attempt-attempt-of-total = 正在重试。1 秒后进行下一次尝试（第 { $attempt } 次，共 { $total } 次）。
retrying-next-attempt-in-seconds-seconds-attempt-attempt-of-total = 正在重试。{ $seconds } 秒后进行下一次尝试（第 { $attempt } 次，共 { $total } 次）。
go-to-file-button = 跳转到文件

## Thread view: subagents

subagent = 子智能体
subagents-awaiting-permission = 等待授权的子智能体：
scroll-to-subagent = 滚动到子智能体
stop-subagent = 停止子智能体
minimize-subagent = 最小化子智能体
make-subagent-full-screen = 子智能体全屏显示
subagent-output = 子智能体输出
everything-below-this-line-was-sent-as-output-from-this-subagent-to-the-main-agent = 此分隔线以下的内容都是该子智能体回传给主智能体的输出。
subagent-canceled = 子智能体已取消
subagent-cancelled = 子智能体已取消
subagent-failed = 子智能体失败
spawning-agent = 正在启动智能体…
click-to-collapse = 点击折叠
click-to-preview = 点击预览

## Thread view: message queue

awaiting-confirmation-count = 等待确认（{ $count }）
awaiting-confirmation = 等待确认
t-1-queued-message = 1 条排队消息
count-queued-messages = { $count } 条排队消息
clear-all = 全部清除
# A toggle on a queued message: on, it interrupts the agent at its next step
# instead of waiting its turn. 插队 reads as skipping the queue, which is the
# behaviour; 打断插入 stacks two verbs and invites the wrong parse.
steer = 插队
interrupt-the-agent-at-its-next-step-to-send-this-message-when-off-queued-messages-wait-for-the-agent-to-finish = 在智能体的下一步打断它以发送此消息。关闭时，排队消息会等智能体完成后再发送。
next-in-queue = 队列中的下一条
in-queue = 排队中
edit-queued-message = 编辑排队消息
type-anything-to-edit = 输入任意内容即可编辑
send-now = 立即发送
remove-message-from-queue = 从队列中移除消息

## Thread view: plan and context

current = 当前：
all-done = 全部完成
count-tasks = { $count } 项任务
plan = 计划
clear-plan = 清除计划
completed-plan = 已完成的计划
compacting-context = 正在压缩上下文…
context-compacted = 上下文已压缩
compaction-canceled = 压缩已取消
context-too-large = 上下文过大
thread-reaching-the-token-limit-soon = 会话即将达到 token 上限
thread-reached-the-token-limit = 会话已达到 token 上限
to-continue-run-compact-or-start-a-new-thread-and-mention-this-one = 若要继续，请运行 /compact，或新建会话并用 @ 引用当前会话
loading-added-context = 正在加载已添加的上下文…
add-context = 添加上下文
image = 图片
files-directories = 文件与目录
symbols = 符号
threads = 会话
branch-diff = 分支差异
# The editor selection offered as context, not the Selection menu that zed.ftl
# owns — hence a key of its own.
selection-context = 选区
rules = 规则
t-1-global-rule = 1 条全局规则
cost = 费用
input = 输入：
output = 输出：
thinking = 思考

## Thread view: edits and review

editing-1-file = 正在编辑 1 个文件…
editing-count-files = 正在编辑 { $count } 个文件…
edits = 改动
t-1-file = 1 个文件
count-files = { $count } 个文件
review-changes = 审阅改动
wait-until-file-edits-are-complete = 请等待文件编辑完成。
reject-all = 全部拒绝
keep-all = 全部保留
reject = 拒绝
keep = 保留
review = 审阅
review-all-files = 审阅全部文件
review-title = 审阅：{ $title }
agent-diff = 智能体差异
no-changes-to-review = 没有需要审阅的改动
continue-iterating = 继续迭代
generating-changes = 正在生成改动…
agent-changes-rejected = 已拒绝智能体的改动
restore-checkpoint = 恢复检查点
restores-all-files-in-the-project-to-the-content-they-had-at-this-point-in-the-conversation = 将项目中的所有文件恢复到对话此处时的内容。
editing-will-restart-the-thread-from-this-point = 编辑会让会话从此处重新开始。
unavailable-editing = 无法编辑
editing-previous-messages-is-not-available-for-agent-yet = { $agent } 暂不支持编辑历史消息。
interrupted-edit = 中断的编辑
discard-interrupted-edit = 丢弃中断的编辑
you-can-discard-this-interrupted-partial-edit-and-restore-the-original-file-content = 你可以丢弃这次中断的部分编辑，并恢复文件的原始内容。

## Thread view: modes and effort

disable-fast-mode = 关闭快速模式
enable-fast-mode = 开启快速模式
enable-now = 立即启用
enable-and-don-t-show-again = 启用并不再提示
disable-thinking-mode = 关闭思考模式
enable-thinking-mode = 开启思考模式
select-effort = 选择思考强度
change-mode = 切换模式
cycle-through-modes = 循环切换模式
change-model = 切换模型
cycle-favorite-models = 循环切换收藏模型
change-thinking-effort = 切换思考强度
cycle-thinking-effort = 循环切换思考强度
change-profile = 切换配置档
cycle-through-profiles = 循环切换配置档
all-options = 全部选项
select-an-option = 选择一个选项…
unfavorite = 取消收藏
favorite = 收藏
recommended = 推荐
latest = 最新
all = 全部
select-model = 选择模型
select-a-model-title = 选择模型
unfavorite-model = 取消收藏模型
favorite-model = 收藏模型
cost-multiplier-cost = 费用倍率：{ $cost }
cost-per-million-tokens-cost = 每百万 token 费用：{ $cost }
cost-cost = 费用：{ $cost }

## Thread view: sending and following

stop-generation = 停止生成
type-to-send = 输入内容以发送
queue-and-send = 排队并发送
send-immediately = 立即发送
send-message = 发送消息
stop-following-the-agent = 停止跟随 { $agent }
stop-following-agent = 停止跟随 { $agent }
follow-the-agent = 跟随 { $agent }
follow-agent = 跟随 { $agent }
track-the-agent-s-location-as-it-reads-and-edits-files = 在智能体读取和编辑文件时跟踪它的位置。
scroll-to-user-message = 滚动到用户消息
scroll-to-top = 滚动到顶部
scroll-to-bottom = 滚动到底部
copy-this-agent-response = 复制此条智能体回复
copy-selection = 复制选区

## Thread view: terminal tool and sandboxing

run-command = 运行命令
copy-command = 复制命令
stop-this-command = 停止此命令
also-possible-by-placing-your-cursor-inside-the-terminal-and-using-regular-terminal-bindings = 也可以把光标放进终端，使用终端本身的快捷键。
exited-with-code-code = 退出码 { $code }
detail-click-to-learn-more-about-sandboxing = { $detail } 点击了解沙箱的更多信息。
output-exceeded-terminal-max-lines-and-was-truncated-the-model-received-the-first-size = 输出超过终端最大行数已被截断，模型只收到了前 { $size }。
output-is-total-long-and-to-avoid-unexpected-token-usage-only-sent-was-sent-back-to-the-agent = 输出共 { $total }，为避免意外消耗 token，只有 { $sent } 回传给了智能体。
output-was-truncated = 输出已被截断
sandboxing = 沙箱
you-have-sandboxing-disabled-in-settings = 你已在设置中禁用沙箱。
sandboxing-is-disabled-for-this-thread = 本次会话已禁用沙箱
couldn-t-create-a-sandbox = 无法创建沙箱
allowed-for-this-thread-after-the-sandbox-failed-reason = 沙箱创建失败后已为本次会话放行：{ $reason }
unsandboxed-execution-is-allowed-for-the-rest-of-this-thread = 本次会话的后续命令允许在沙箱外执行。
ran-without-sandbox = 未使用沙箱运行
runs-without-the-os-sandbox = 不使用操作系统沙箱运行
view-sandboxing-docs = 查看沙箱文档
open-docs = 打开文档
reason = 原因
defined-in-your-settings = 设置中定义：
allowed-for-this-thread = 本次会话额外允许：
write-access = 写入权限
network-access = 网络访问权限
all-paths-except-protected-git-metadata = 除受保护的 Git 元数据外的所有路径
all-domains-unrestricted = 所有域名（不受限制）
raw-input = 原始输入：
view-raw-input = 查看原始输入

## Thread view: permissions

configure-unicode-confusables-warning = 配置易混淆 Unicode 字符警告
i-understand-and-wish-to-proceed = 我已了解并希望继续
this-command-can-write-to-a-file-on-a-windows-drive = 此命令可以写入 Windows 盘上的文件
configure-windows-drive-warning = 配置 Windows 盘警告
don-t-show-this-warning-again = 不再显示此警告
source = 源
target = 目标
write-path = 写入路径
always-for-selected-commands = 对所选命令始终允许
only-this-time = 仅这一次
select-options = 选择选项…

## Thread view: errors

rate-limit-reached = 已达到速率限制
provider-s-rate-limit-was-reached-zed-will-retry-automatically-you-can-also-wait-a-moment-and-try-again = 已达到 { $provider } 的速率限制。Zed 会自动重试，你也可以稍等片刻后再试。
provider-unavailable = 提供方不可用
provider-s-servers-are-temporarily-unavailable-zed-will-retry-automatically-if-the-problem-persists-check-the-provider-s-status-page = { $provider } 的服务器暂时不可用。Zed 会自动重试。如果问题持续存在，请查看该提供方的状态页。
no-credentials-are-configured-for-provider = 尚未为 { $provider } 配置凭据。
credentials-missing = 缺少凭据
connection-interrupted = 连接已中断
the-connection-to-provider-s-api-was-interrupted-zed-will-retry-automatically-if-the-problem-persists-check-your-network-connection = 与 { $provider } API 的连接已中断。Zed 会自动重试。如果问题持续存在，请检查你的网络连接。
could-not-authenticate-with-provider = 无法通过 { $provider } 认证。
authentication-failed = 认证失败
provider-rejected-the-request-due-to-insufficient-permissions = { $provider } 因权限不足拒绝了该请求。
permission-denied = 权限不足
request-failed = 请求失败
the-request-could-not-be-completed-after-multiple-attempts-try-again-in-a-moment = 多次尝试后仍无法完成请求。请稍后再试。
output-limit-reached = 已达到输出上限
the-model-stopped-because-it-reached-its-maximum-output-length-you-can-ask-it-to-continue-where-it-left-off = 模型因达到最大输出长度而停止。你可以让它从中断处继续。
no-model-selected = 未选择模型
select-a-model-from-the-model-picker-below-to-get-started = 请在下方的模型选择器中选择一个模型以开始。
api-error = API 错误
provider-s-api-returned-an-unexpected-error-if-the-problem-persists-try-switching-models-or-restarting-zed = { $provider } 的 API 返回了意外错误。如果问题持续存在，请尝试切换模型或重启 Zed。
name-refused-to-respond-to-this-prompt-this-can-happen-when-a-model-believes-the-prompt-violates-its-content-policy-or-safety-guidelines-so-rephrasing-it-can-sometimes-address-the-issue = { $name } 拒绝回应此提示词。当模型认为提示词违反其内容政策或安全准则时可能出现这种情况，改写提示词有时可以解决问题。
request-refused = 请求被拒绝
authentication-required-title = 需要认证
you-reached-your-free-usage-limit-upgrade-to-zed-pro-for-more-prompts = 你已用完免费额度。升级到 Zed Pro 可获得更多提示次数。
free-usage-exceeded = 免费额度已用尽
failed-to-authenticate-with-provider-provider = 无法通过 { $provider } 提供方认证
open-the-settings-to-configure-the-selected-provider = 打开设置以配置所选的提供方
model-model-was-not-found = 未找到模型 { $model }
you-may-need-to-reconfigure-authentication-for-this-provider = 你可能需要重新配置此提供方的认证
provider-provider-was-not-found = 未找到提供方 { $provider }
open-the-settings-to-configure-providers = 打开设置以配置提供方
choose-a-different-model-or-configure-other-providers-to-get-started = 请选择其他模型或配置其他提供方以开始
configure-a-provider-to-get-started = 请配置一个提供方以开始
upgrade = 升级
an-error-happened = 发生错误
retry-generation = 重新生成
note-model-cannot-be-offered-with-zero-data-retention = 注意：{ $model } 无法在零数据保留模式下提供。
anthropic-will-retain-inference-logs = Anthropic 会保留推理日志。
switch-to-model = 切换到 { $model }

## Thread view: notices

this-agent-does-not-support-viewing-previous-messages-however-your-session-will-still-continue-from-where-you-last-left-off = 此智能体不支持查看历史消息。不过，你的会话仍会从上次中断处继续。
resumed-session = 已恢复会话
codex-on-windows = Windows 上的 Codex
for-best-performance-run-codex-in-windows-subsystem-for-linux-wsl2 = 为获得最佳性能，请在 Windows Subsystem for Linux（WSL2）中运行 Codex
dismiss-warning = 忽略警告
skill-failed-to-load = 技能加载失败
skill-omitted-from-model-catalog = 技能未纳入模型目录
open-skill = 打开技能
t-1-skill-loaded-with-a-long-description = 1 个技能的描述过长
count-skills-loaded-with-long-descriptions = { $count } 个技能的描述过长
ensure-skill-descriptions-are-at-most-limit-bytes-longer-ones-may-consume-more-model-context-tokens = 请将技能描述控制在 { $limit } 字节以内；过长的描述会占用更多模型上下文 token。
review-before-sending = 发送前请确认
this-prompt-was-pre-filled-by-an-external-link-read-it-carefully-before-you-submit-it-to-the-model = 此提示词由外部链接预填。提交给模型前请仔细阅读。
one-folder = 一个文件夹
this-agent-doesn-t-currently-support-multi-root-workspaces = 此智能体目前不支持多根工作区
it-currently-only-operates-by-default-on-folder = 它目前默认只在「{ $folder }」上工作。
new-version-available = 有新版本可用
agent-update-available = 智能体有更新可用
update-to-v-version = 更新到 v{ $version }

## Elicitation

# $field and $title come from an external MCP tool's schema, so they are usually
# an English word ("Age", "Email"). They get a space after them, like every other
# interpolated value in this catalog — running them straight into the Chinese
# makes the boundary hard to see.
field-is-required = { $field } 为必填项
field-needs-more-selections = { $field } 需要选择更多个选项
field-has-too-many-selections = { $field } 选择的项过多
title-must-be-a-number = { $title } 必须是数字
title-must-be-a-finite-number = { $title } 必须是有限数字
title-must-be-at-least-minimum = { $title } 不能小于 { $minimum }
title-must-be-at-most-maximum = { $title } 不能大于 { $maximum }
title-must-be-an-integer = { $title } 必须是整数
title-is-too-short = { $title } 过短
title-is-too-long = { $title } 过长
title-must-be-one-of-the-provided-options = { $title } 必须是所提供的选项之一
title-has-an-invalid-validation-pattern = { $title } 的校验正则无效
title-is-too-long-to-validate-safely = { $title } 过长，无法安全校验
title-has-an-invalid-validation-format = { $title } 的校验格式无效
title-has-a-validation-pattern-that-is-too-complex = { $title } 的校验正则过于复杂
title-does-not-match-the-requested-constraints = { $title } 不满足所要求的约束
title-does-not-match-the-requested-pattern = { $title } 不匹配所要求的正则
# $format resolves to Chinese (电子邮件地址, 日期…), so no space before it.
title-must-be-format = { $title } 必须是{ $format }
an-email-address = 电子邮件地址
a-uri = URI
a-date = 日期
a-date-and-time = 日期和时间
waiting-for-completion = 等待完成
submitted = 已提交
declined = 已拒绝
canceled = 已取消
completed = 已完成
input-requested-by-requester = { $requester } 请求输入
destination = 目标地址
this-internationalized-address-displays-as-host-verify-it-carefully = 该国际化地址显示为 { $host }。请仔细核对。
open-again = 再次打开
submit = 提交

## Thread search bar

search-this-thread = 搜索此会话…
previous-match = 上一个匹配项
next-match = 下一个匹配项
close-search = 关闭搜索

## Message editor

paste-as-plain-text = 粘贴为纯文本
select-images = 选择图片

## Completion provider

positive-feedback = 好评
negative-feedback = 差评
rate-this-response-as-helpful-sends-the-current-conversation-to-the-zed-team = 将此回复评为有帮助。会把当前对话发送给 Zed 团队。
rate-this-response-as-not-helpful-sends-the-current-conversation-to-the-zed-team = 将此回复评为没帮助。会把当前对话发送给 Zed 团队。
commands = 命令
mcp-server-commands = MCP 服务器命令
branch-diff-vs-base = 分支差异（对比 { $base }）
errors-and-warnings = { $errors }和{ $warnings }
diagnostics-body = 诊断：{ $body }
errors-warnings = { $errors }和{ $warnings }
t-1-error = 1 个错误
count-errors = { $count } 个错误
t-1-warning = 1 个警告
count-warnings = { $count } 个警告

## Profile selector

tools-unsupported = 不支持工具
this-model-does-not-support-tools = 此模型不支持工具。
get-help-to-write-anything = 帮你写任何东西。
chat-about-your-codebase = 就你的代码库进行对话。
chat-about-anything-with-no-tools = 不使用工具，随意对话。
custom-profiles = 自定义配置档
search-profiles = 搜索配置档…
no-profiles = 没有配置档。
no-profiles-match-your-search = 没有匹配搜索的配置档。
disabled-in-restricted-mode = 受限模式下已禁用
some-tools-are-disabled-click-to-review-trust-settings = 部分工具已被禁用。点击查看信任设置。

## Agent registry

search-agents = 搜索智能体…
loading-registry = 正在加载注册表…
failed-to-load-the-agent-registry-please-check-your-connection-and-try-again = 加载智能体注册表失败。请检查网络连接后重试。
no-agents-match-your-search = 没有匹配搜索的智能体。
no-agents-available = 没有可用的智能体。
no-installed-agents-match-your-search = 没有匹配搜索的已安装智能体。
no-installed-agents = 没有已安装的智能体。
no-uninstalled-agents-match-your-search = 没有匹配搜索的未安装智能体。
no-uninstalled-agents = 没有未安装的智能体。
missing-registry-entry = 缺少注册表条目。
visit-agent-repository = 访问智能体仓库
visit-agent-website = 访问智能体网站
not-supported-on-this-platform = 此平台不支持
unavailable = 不可用
install = 安装
installed = 已安装
not-installed = 未安装
acp-registry = ACP 注册表

## Thread import

fetching-sessions = 正在获取会话…
importing-threads-from-this-agent-is-not-possible-as-it-doesn-t-support-acp-s-session-list-capability = 无法从此智能体导入会话，因为它不支持 ACP 的 session/list 能力。
failed-to-fetch-sessions-error = 获取会话失败：{ $error }
could-not-find-workspace-to-import-from = 找不到可供导入的工作区。
did-not-find-any-workspaces-to-import-from = 未找到任何可供导入的工作区。
no-threads-found-to-import = 没有可导入的会话。
imported-1-thread = 已导入 1 个会话。
imported-count-threads = 已导入 { $count } 个会话。
no-threads = 没有会话
count-threads = { $count } 个会话
import-external-agent-threads = 导入外部智能体会话
import-threads-from-agents-like-claude-agent-codex-and-more-whether-started-in-zed-or-another-client-choose-which-agents-to-include-and-their-threads-will-appear-in-your-thread-history = 从 Claude Agent、Codex 等智能体导入会话，无论它们是在 Zed 还是其他客户端中开始的。选择要包含的智能体，它们的会话就会出现在你的会话历史中。
no-external-agents-available = 没有可用的外部智能体。
fetching-agent-threads = 正在获取智能体会话…
import-threads = 导入会话
failed-to-list-sessions = 无法列出会话。
no-new-threads-found-to-import = 没有可导入的新会话。
imported-1-thread-from-other-channels = 已从其他渠道导入 1 个会话。
imported-count-threads-from-other-channels = 已从其他渠道导入 { $count } 个会话。

## Threads archive

today = 今天
yesterday = 昨天
this-week = 本周
past-week = 上周
older = 更早
search-all-threads = 搜索所有会话…
cancel-restore = 取消恢复
delete-thread = 删除会话
archive-thread = 归档会话
clear-search = 清除搜索
t-1-thread = 1 个会话
show-all-threads = 显示所有会话
show-only-archived-threads = 仅显示已归档会话
no-threads-match-your-search = 没有匹配搜索的会话。
no-threads-yet-empty-state = 还没有会话。
choose-from-local-folders = 从本地文件夹选择

## Inline assistant

inline-assistant-error-error = 内联助手出错：{ $error }
terminal-inline-assistant-error-error = 终端内联助手出错：{ $error }
or-type-to-include-context = 或输入 @ 引用上下文
add-a-prompt = 添加提示词…
generate = 生成
transform = 转换
keybinding-to-chat = { $keybinding } 开始对话
action-keybinding-for-history-to-include-context = { $action }…（{ $keybinding } ― ↓↑ 查看历史 — @ 引用上下文）
can-t-rate-still-generating = 仍在生成，暂时无法评价…
already-rated-this-completion = 已对此补全评价过
no-configured-model = 没有已配置的模型
click-to-copy-rating-id = 点击复制评价 ID
changes-won-t-be-discarded = 改动不会被丢弃
changes-will-be-discarded = 改动将被丢弃
good-result = 结果很好
you-already-rated-this-result = 你已对此结果评价过
bad-result = 结果不好
execute-generated-command = 执行生成的命令
close-assistant = 关闭助手
previous-alternative = 上一个备选
next-alternative = 下一个备选
interrupt-generation = 中断生成
interrupt-transform = 中断转换
restart-generation = 重新生成
restart-transform = 重新转换
accept-generation = 接受生成结果
accept-transform = 接受转换结果

## MCP server configuration

enter-client-secret-leave-empty-for-public-clients = 输入客户端密钥（公共客户端可留空）
enter-your-oauth-client-secret-or-leave-empty-for-public-clients = 输入你的 OAuth 客户端密钥，公共客户端可留空
server-configured-successfully = { $server } 配置成功。
configure-server-named = 配置 { $server }
configure-server = 配置服务器
check-the-server-docs-for-required-arguments-and-environment-variables = 请查阅服务器文档了解所需的参数和环境变量。
open-repository = 打开仓库
connecting-server = 正在连接服务器…

## Agent profiles

profile-name = 配置档名称
customize = 自定义
agent-profiles = 智能体配置档
add-new-profile = 新增配置档
fork-profile-named = 复刻 { $profile }
fork-profile = 复刻配置档
new-profile = 新建配置档
configure-default-model = 配置默认模型
configure-built-in-tools = 配置内置工具
configure-mcp-tools = 配置 MCP 工具
delete-profile = 删除配置档
profile-configure-built-in-tools = { $profile } — 配置内置工具
profile-configure-default-model = { $profile } — 配置默认模型
profile-configure-mcp-tools = { $profile } — 配置 MCP 工具
search-built-in-tools = 搜索内置工具…
search-mcp-tools = 搜索 MCP 工具…

## Notifications and upsells

view-agent-notification = 查看
upgrade-to-zed-pro = 升级到 Zed Pro
current-plan = （当前套餐）
your-zed-pro-trial-has-expired = 你的 Zed Pro 试用已到期
you-ve-been-automatically-reset-to-the-free-plan = 你已被自动重置为免费套餐。
