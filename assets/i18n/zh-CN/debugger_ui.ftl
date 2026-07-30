# Simplified Chinese (zh-CN) catalog for the debugger panel and its sub-views
# (breakpoints, variables, stack frames, memory view, console, new-process modal).
#
# Product names (Zed), file names (debug.json, settings.json) and DAP adapter /
# process identifiers are deliberately left untranslated. Several keys used by
# this crate (minimize, step-over, step-out, open-documentation, variables,
# terminal, run, expand, debugger, program, working-directory) are already
# defined in other catalogs and reused here.

## Buttons that open debug.json

# The application menu's entry is 'Edit debug.json…', whose ellipsis promises a
# dialog; these three are plain buttons and a tooltip, so they need their own key
# rather than folding into zed.ftl's edit-debug-json.
edit-debug-json-button = 编辑 debug.json

## Stack frames

restart-stack-frame = 重启栈帧
show-count-more = 显示另外 { $count } 个
show-stack-frames-from-your-project = 显示项目内的栈帧
show-all-stack-frames = 显示所有栈帧

## Variables and watches

copy-name = 复制名称
copy-value = 复制值
edit-value = 编辑值
go-to-memory = 跳转到内存
watch-variable = 监视变量
remove-watch = 移除监视
read = 读
write = 写
read-write = 读/写
toggle-access-data-breakpoint = 切换 { $access } 数据断点
toggle-data-breakpoint = 切换数据断点

## Breakpoints

remove-breakpoint-from-a-breakpoint-list = 从断点列表中移除断点
exception-breakpoints-cannot-be-removed-from-the-breakpoint-list = 异常断点无法从断点列表中移除
remove-data-breakpoint-from-a-breakpoint-list = 从断点列表中移除数据断点
disable-breakpoint = 禁用断点
disable-a-breakpoint-without-removing-it-from-the-list = 禁用断点但不将其从列表中移除
enable-breakpoint = 启用断点
re-enable-a-breakpoint = 重新启用断点
remove-breakpoint = 移除断点
worktree-parent-path-path = 工作树父路径：{ $path }
disable-data-breakpoint = 禁用数据断点
enable-data-breakpoint = 启用数据断点
disable-exception-breakpoint = 禁用异常断点
enable-exception-breakpoint = 启用异常断点
set-log-message = 设置日志消息
set-log-message-to-display-instead-of-stopping-when-a-breakpoint-is-hit = 设置命中断点时显示的日志消息（而非暂停程序）。
set-condition = 设置条件
set-condition-to-evaluate-when-a-breakpoint-is-hit-program-execution-will-stop-only-when-the-condition-is-met = 设置命中断点时要判断的条件，仅当条件成立时程序才会暂停。
set-hit-condition = 设置命中条件
set-expression-that-controls-how-many-hits-of-the-breakpoint-are-ignored = 设置控制忽略多少次断点命中的表达式。
breakpoints = 断点
no-breakpoints-set = 未设置断点

## Memory view

debug-adapter-adapter-name-does-not-support-writing-to-memory = 调试适配器 `{ $adapter_name }` 不支持写入内存
edit-memory-at-a-selected-address = 在选定地址处编辑内存
change-address-of-currently-viewed-memory = 更改当前查看的内存地址

## Console

watch-expression = 监视表达式
evaluate = 求值

## Session picker and thread dropdown

child = 子项
unknown-session = 未知会话
select-a-debug-session = 选择调试会话
tid-id = Tid：{ $id }

## Attach modal

select-the-process-you-want-to-attach-the-debugger-to = 选择你想要附加调试器的进程

## Debugger panel toolbar

start-debug-session = 启动调试会话
open-debug-adapter-logs = 打开调试适配器日志
close-panel = 关闭面板
pause-program = 暂停程序
continue-program = 继续程序
step-in = 单步进入
rerun-session = 重新运行会话
terminate-thread = 终止线程
terminate-all-threads = 终止所有线程
detach = 分离
step-back-in-session-history = 回退会话历史记录
current-state = 当前状态

## Debugger panel empty state

new-session = 新建会话
debugger-docs = 调试器文档
debugger-extensions = 调试器扩展

## Pane items (session layout)

console = 控制台
frames = 调用栈
modules = 模块
sources = 源文件
memory-view = 内存视图
displays-program-output-and-allows-manual-input-of-debugger-commands = 显示程序输出，并允许手动输入调试器命令
shows-current-values-of-local-and-global-variables-in-the-current-stack-frame = 显示当前栈帧中局部变量和全局变量的当前值
lists-all-active-breakpoints-set-in-the-code = 列出代码中设置的所有生效断点
displays-the-call-stack-letting-you-navigate-between-function-calls = 显示调用栈，可在函数调用之间跳转
shows-all-modules-or-libraries-loaded-by-the-program = 显示程序加载的所有模块或库
lists-all-source-files-currently-loaded-and-used-by-the-debugger = 列出调试器当前加载并使用的所有源文件
provides-an-interactive-terminal-session-within-the-debugging-environment = 在调试环境中提供交互式终端会话
allows-inspection-of-memory-contents = 允许查看内存内容

## New process modal

debug = 调试
attach = 附加
launch = 启动
run-predefined-task = 运行预定义任务
start-a-predefined-debug-scenario = 启动预定义调试场景
attach-the-debugger-to-a-running-process = 将调试器附加到正在运行的进程
launch-a-new-process-with-a-debugger = 使用调试器启动新进程
edit-in-debug-json = 在 debug.json 中编辑
start = 启动
debugger-adapter-label = 调试器：
stop-on-entry = 入口处暂停
lsp-language-name = LSP：{ $language_name }
language-name = 语言：{ $name }
in-path = 位于 { $path }
launch-custom = 启动自定义配置
rerun = 重新运行
spawn = 运行
