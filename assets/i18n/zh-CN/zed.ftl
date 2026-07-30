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
