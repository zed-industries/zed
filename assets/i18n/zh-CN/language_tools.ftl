# Simplified Chinese (zh-CN) catalog for language_tools (LSP 日志、语法树与高亮查看器、键盘上下文调试视图).
#
# LSP 方法名、服务器名、日志级别标识符等技术标识符保持不译。

## 键盘上下文视图

keyboard-context = 键盘上下文
this-view-lets-you-determine-the-current-context-stack-for-creating-custom-key-bindings-in-zed-when-a-keyboard-shortcut-is-triggered-it-also-shows-all-the-possible-contexts-it-could-have-triggered-in-and-which-one-matched = 此视图可用于查看当前的上下文栈，以便为 Zed 创建自定义快捷键。触发快捷键时，它还会显示所有可能匹配的上下文，以及实际匹配到的那一个。
open-documentation = 打开文档
view-default-keymap = 查看默认按键映射
edit-keymap-file = 编辑按键映射文件
current-context-stack = 当前上下文栈
last-keystroke = 上一次按键
waiting-for-more-input-keystrokes = 正在等待更多输入：{ $keystrokes }
typed-keystrokes = 已输入：{ $keystrokes }
match = 匹配
low-precedence = 优先级较低
no-match = 不匹配
key-equivalents = 按键等效项
shortcuts-defined-using-some-characters-have-been-remapped-so-that-shortcuts-can-be-typed-without-holding-option = 使用特定字符定义的快捷键已被重新映射，这样无需按住 Option 键即可输入。

## 语法树视图

syntax-tree = 语法树
current-editor-has-no-associated-language = 当前编辑器未关联任何语言
try-assigning-a-language-or-switching-to-a-different-buffer = 请尝试指定一种语言，或切换到其他缓冲区
not-attached-to-an-editor = 未关联到任何编辑器
focus-an-editor-to-show-a-new-tree-view = 聚焦编辑器以显示新的语法树视图
update-view-to-active-tab-name = 更新视图到「{ $active_tab_name }」

## 高亮查看器

all-highlights-are-filtered-out = 所有高亮均已被筛选掉
enable-text-syntax-or-semantic-highlights-in-the-toolbar = 请在工具栏中启用文本、语法或语义高亮
no-highlights-found = 未找到高亮
the-editor-has-no-text-syntax-or-semantic-token-highlights = 该编辑器没有文本、语法或语义 token 高亮
focus-an-editor-to-show-highlights = 聚焦编辑器以显示高亮
highlights = 高亮
total-highlights = { $total } 处高亮
filtered-total-highlights = { $filtered } / { $total } 处高亮
highlights-settings = 高亮设置
text-highlights = 文本高亮
syntax-tokens = 语法 Token

## 语言服务器状态按钮与菜单

project-is-in-restricted-mode = 项目处于受限模式
language-servers-can-t-run-until-you-trust-this-project = 在你信任此项目之前，语言服务器无法运行。
restart-all-servers = 重启所有服务器
stop-all-servers = 停止所有服务器
starting = 正在启动…
stopped = 已停止
error = 错误
running = 运行中
warning = 警告
view-message = 查看消息
language-server-server-name = 语言服务器 { $server_name }：
view-logs = 查看日志
restart-server = 重启服务器
stop-server = 停止服务器
server-with-errors = 存在错误的服务器
server-with-warnings = 存在警告的服务器
server-with-notifications = 有通知的服务器
all-servers-operational = 所有服务器均正常运行

## LSP 日志查看器

lsp-logs = LSP 日志
no-server-selected = 未选择服务器
rpc-messages = RPC 消息
server-trace = 服务器跟踪
server-logs = 服务器日志
server-info = 服务器信息
trace-level = 跟踪级别
off = 关闭
messages = 消息
verbose = 详细
log-level = 日志级别
log = 日志
info = 信息
