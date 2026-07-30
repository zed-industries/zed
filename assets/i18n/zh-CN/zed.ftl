# Simplified Chinese (zh-CN) catalog for Zed's interface.
#
# Keys are derived from the English source strings by lower-kebab-casing them,
# so `t!("Zoom In")` resolves `zoom-in`. Any key that is missing here falls back
# to the English string at its call site.
#
# Product names (Zed, Git, GPUI), file names (tasks.json, debug.json) and URLs
# are deliberately left untranslated.

## Application menu

about-zed = 关于 Zed
check-for-updates = 检查更新
settings = 设置
open-settings = 打开设置
open-settings-file = 打开设置文件
open-project-settings = 打开项目设置
open-project-settings-file = 打开项目设置文件
open-default-settings = 打开默认设置
open-keymap = 打开按键映射
open-keymap-file = 打开按键映射文件
open-default-key-bindings = 打开默认快捷键
select-theme = 选择主题…
select-icon-theme = 选择图标主题…
services = 服务
extensions = 扩展
install-cli = 安装命令行工具
hide-zed = 隐藏 Zed
hide-others = 隐藏其他
show-all = 全部显示
quit-zed = 退出 Zed

## File

file = 文件
new = 新建
new-window = 新建窗口
open-file = 打开文件…
open-folder = 打开文件夹…
open = 打开…
open-recent = 打开最近使用…
open-remote = 打开远程…
add-folder-to-project = 将文件夹添加到项目…
save = 保存
save-as = 另存为…
save-all = 全部保存
close-editor = 关闭编辑器
close-project = 关闭项目
close-window = 关闭窗口

## Edit

edit = 编辑
undo = 撤销
redo = 重做
cut = 剪切
copy = 复制
copy-and-trim = 复制并去除首尾空白
paste = 粘贴
find = 查找
find-in-project = 在项目中查找
toggle-line-comment = 切换行注释

## Selection

selection = 选择
select-all = 全选
expand-selection = 扩大选区
shrink-selection = 缩小选区
select-next-sibling = 选择下一个同级节点
select-previous-sibling = 选择上一个同级节点
add-cursor-above = 在上方添加光标
add-cursor-below = 在下方添加光标
select-next-occurrence = 选择下一个匹配项
select-previous-occurrence = 选择上一个匹配项
select-all-occurrences = 选择所有匹配项
move-line-up = 上移一行
move-line-down = 下移一行
duplicate-selection = 复制选区

## View

view = 视图
zoom-in = 放大
zoom-out = 缩小
reset-zoom = 重置缩放
reset-all-zoom = 重置所有缩放
toggle-left-dock = 切换左侧边栏
toggle-right-dock = 切换右侧边栏
toggle-bottom-dock = 切换底部面板
toggle-all-docks = 切换所有面板
editor-layout = 编辑器布局
split-up = 向上拆分
split-down = 向下拆分
split-left = 向左拆分
split-right = 向右拆分
project-panel = 项目面板
outline-panel = 大纲面板
collab-panel = 协作面板
terminal-panel = 终端面板
debugger-panel = 调试器面板
agent-panel = 智能体面板
git-panel = Git 面板
diagnostics = 诊断
toggle-gpui-inspector = 切换 GPUI 检查器

## Go

go = 跳转
back = 后退
forward = 前进
command-palette = 命令面板…
go-to-file = 跳转到文件…
go-to-symbol-in-editor = 跳转到编辑器内符号…
go-to-line-column = 跳转到行/列…
go-to-definition = 跳转到定义
go-to-declaration = 跳转到声明
go-to-type-definition = 跳转到类型定义
find-all-references = 查找所有引用
next-problem = 下一个问题
previous-problem = 上一个问题

## Run

run = 运行
spawn-task = 运行任务
start-debugger = 启动调试器
edit-tasks-json = 编辑 tasks.json…
edit-debug-json = 编辑 debug.json…
continue = 继续
step-over = 单步跳过
step-into = 单步进入
step-out = 单步跳出
toggle-breakpoint = 切换断点
edit-breakpoint = 编辑断点
clear-all-breakpoints = 清除所有断点

## Window

window = 窗口
minimize = 最小化
zoom = 缩放

## Help

help = 帮助
view-release-notes-locally = 在本地查看发行说明
view-telemetry = 查看遥测日志
view-dependency-licenses = 查看依赖许可证
show-welcome = 显示欢迎页
file-bug-report = 提交缺陷报告…
request-feature = 请求新功能…
email-us = 给我们发邮件…
documentation = 文档
zed-repository = Zed 代码仓库
zed-twitter = Zed 的 Twitter
join-the-team = 加入团队

## Move to Applications

move-zed-to-applications = 将 Zed 移动到「应用程序」？
zed-is-running-from-a-temporary-location-move-it-to-applications-to-finish-installing-it = Zed 正在从临时位置运行。将它移动到「应用程序」以完成安装。
don-t-ask-me-again = 不再询问
failed-to-move-zed-to-applications = 将 Zed 移动到「应用程序」失败
installing-zed = 正在安装 Zed…
moving-zed-to-applications = 正在将 Zed 移动到「应用程序」
zed-will-reopen-when-installation-is-complete = 安装完成后 Zed 将重新打开。

## Quick action bar

no-code-actions-available = 没有可用的代码操作
selection-controls = 选区控制
go-to-symbol = 跳转到符号
go-to-line-column-inline = 跳转到行/列
editor-controls = 编辑器控制
inline-values = 内联值
semantic-highlights = 语义高亮
you-can-t-toggle-edit-predictions-for-this-file-as-it-is-within-the-excluded-files-list = 此文件在排除列表中，无法为其切换编辑预测。
inline-diagnostics-are-not-available-until-regular-diagnostics-are-enabled = 需先启用常规诊断，才能使用内联诊断。
line-numbers = 行号
selection-menu = 选区菜单
column-git-blame = 列 Git Blame

## REPL

kernel-name-language = 内核：{ $name }（{ $language }）
run-selection = 运行选中内容
run-line = 运行当前行
# `interrupt` reused from repl.ftl.
clear-outputs = 清除输出
shut-down-kernel = 关闭内核
# `restart-kernel` reused from repl.ftl.
view-sessions = 查看会话
repl-menu = REPL 菜单
start-repl-for-kernel = 为 { $kernel } 启动 REPL
# `select-kernel` reused from repl.ftl.
setup-zed-repl-for-language = 为 { $language } 设置 Zed REPL
nothing-running = 未在运行
kernel-is-starting = { $kernel } 正在启动
restarting-kernel = 正在重启 { $kernel }
kernel-is-shutting-down = { $kernel } 正在关闭
auto-restarting-kernel = 正在自动重启 { $kernel }
kernel-state-unknown = { $kernel } 状态未知
kernel-state-state = { $kernel } 状态：{ $state }
run-code-on-kernel-language = 在 { $kernel }（{ $language }）上运行代码
interrupt-kernel-language = 中断 { $kernel }（{ $language }）
error-with-kernel-kernel-error = 内核 { $kernel } 出错：{ $error }
preview-markdown = 预览 Markdown
preview-svg = 预览 SVG
preview-csv = 预览 CSV
keystroke-to-open-in-a-split = 按住 { $keystroke } 可在拆分窗格中打开

## Telemetry log

failed-to-read-telemetry-log-error = 读取遥测日志失败：{ $error }
t-1-telemetry-log-entry-failed-to-parse = 有 1 条遥测日志条目解析失败
count-telemetry-log-entries-failed-to-parse = 有 { $count } 条遥测日志条目解析失败
telemetry-log = 遥测日志
signed-in-chip = 已登录
no-telemetry-events-recorded-yet = 尚未记录任何遥测事件
no-events-match-the-current-filter = 没有事件匹配当前筛选条件
filter-events = 筛选事件…
clear-events = 清除事件
open-raw-log-file = 打开原始日志文件

## Settings & keymap migration

keymap-file-kind = 按键映射
settings-file-kind = 设置
your-file-type-file-uses-deprecated-settings-which-can-be-automatically-updated-a-backup-will-be-saved-to-backup-file-name = 你的{ $file_type }文件使用了已弃用的设置，可以自动更新。备份将保存为 `{ $backup_file_name }`
backup-and-update = 备份并更新
invalid-user-settings-file = 用户设置文件无效
failed-to-migrate-settings = 迁移设置失败
failed-to-load-path = 加载 { $path } 失败
json-parse-error-in-keymap-file-bindings-not-reloaded = 按键映射文件 JSON 解析错误，快捷键未重新加载。
unable-to-access-open-log-file-at-path-path-error = 无法访问/打开位于 { $path } 的日志文件：{ $error }
last-count-lines-in-path = { $path } 中的最后 { $count } 行

## Open URL modal

paste-a-url-to-open = 粘贴要打开的 URL。

## Startup & window errors

inotify-init-returned-error = inotify_init 返回 { $error }
this-may-be-due-to-system-wide-limits-on-inotify-instances-for-troubleshooting-see = 这可能是由于系统对 inotify 实例数的限制。故障排除请参见：
could-not-start-inotify = 无法启动 inotify
troubleshoot-and-quit = 排查问题并退出
readdirectorychangesw-initialization-failed-error = ReadDirectoryChangesW 初始化失败：{ $error }
this-may-occur-on-network-filesystems-and-wsl-paths-for-troubleshooting-see = 这可能发生在网络文件系统和 WSL 路径中。故障排除请参见：
could-not-start-readdirectorychangesw = 无法启动 ReadDirectoryChangesW
zed-uses-api-for-rendering-and-requires-a-compatible-gpu = Zed 使用 { $api } 进行渲染，需要兼容的 GPU。
currently-you-are-using-a-software-emulated-gpu-device-which-will-result-in-awful-performance = 你当前使用的是软件模拟 GPU（{ $device }），这会导致性能极差。
for-troubleshooting-see = 故障排除请参见：
set-zed-allow-emulated-gpu-1-env-var-to-permanently-override = 设置环境变量 ZED_ALLOW_EMULATED_GPU=1 可永久覆盖此限制。
unsupported-gpu = 不受支持的 GPU
skip = 跳过
zed-failed-to-launch = Zed 启动失败
kind-when-creating-directory-path = 创建目录 { $path } 时出现 { $kind }
kind-when-creating-directories-paths = 创建目录 { $paths } 时出现 { $kind }
consider-using-chown-and-chmod-tools-for-altering-the-directories-permissions-if-your-user-has-corresponding-rights = 如果你的用户拥有相应权限，可以考虑使用 chown 和 chmod 工具修改目录权限。
for-example-chown-cmd-and-chmod-cmd = 例如：`{ $chown_cmd }` 和 `{ $chmod_cmd }`

## Open in browser / zed:// scheme

opening-this-url-in-a-browser-failed-because-the-url-is-invalid-url = 在浏览器中打开此 URL 失败，因为 URL 无效：{ $url }
error-was-error = 错误：{ $error }
zed-links-will-now-open-in-channel = zed:// 链接现在将在 { $channel } 中打开。
error-registering-zed-scheme = 注册 zed:// 协议时出错

## About window

version = 版本
commit-commit = 提交：{ $commit }
version-version = 版本：{ $version }

## Notifications

this-project-has-no-folders-open = 此项目未打开任何文件夹。
are-you-sure-you-want-to-quit = 确定要退出吗？
quit = 退出
failed-to-restore-1-workspace-check-logs-for-details = 恢复 1 个工作区失败，详情请查看日志。
failed-to-restore-count-workspaces-check-logs-for-details = 恢复 { $count } 个工作区失败，详情请查看日志。
