# Simplified Chinese (zh-CN) catalog for the editor.
#
# Product names (Zed, Git, Unicode, SVG, Markdown), action identifiers
# (`editor::AcceptEditPrediction`) and language server terminology are
# deliberately left untranslated.
#
# Keys already defined by another catalog are reused rather than redefined:
# go-to-definition / go-to-declaration / go-to-type-definition /
# find-all-references / cut / copy / copy-and-trim / paste (zed.ftl),
# open-in-terminal / copy-path / copy-relative-path / reveal-in-project-panel /
# close / open-file-button (workspace.ftl), stage / unstage / restore /
# stage-hunk / unstage-hunk / restore-hunk / view-file-history / see-docs
# (git_ui.ftl), open-markdown-preview (project_panel.ftl),
# accept (collab_ui.ftl), cancel (common.ftl), confirm (settings_ui.ftl).
#
# The buffer header's "Open File" button names `open-file-button` explicitly:
# the File menu's "Open File..." owns the derived `open-file` key, and the two
# spellings must not share one translation.

## Editor context menu

run-to-cursor = 运行到光标处
evaluate-selection = 求值选中内容
go-to-implementation = 跳转到实现
rename-symbol = 重命名符号
format-buffer = 格式化缓冲区
format-selections = 格式化选区
show-code-actions = 显示代码操作
toggle-code-actions = 切换代码操作
add-to-agent-thread = 添加到智能体会话
open-svg-preview = 打开 SVG 预览
copy-permalink = 复制永久链接

## Rename

rename-old-name-new-name = 重命名：{ $old_name } → { $new_name }

## Gutter context menu and hover buttons

set-bookmark = 设置书签
add-bookmark = 添加书签
remove-bookmark = 移除书签
edit-bookmark = 编辑书签
bookmarks = 书签
bookmark = 书签
set-breakpoint = 设置断点
unset-breakpoint = 取消断点
breakpoint = 断点
edit-log-breakpoint = 编辑日志断点
set-log-breakpoint = 设置日志断点
edit-condition-breakpoint = 编辑条件断点
set-condition-breakpoint = 设置条件断点
edit-hit-condition-breakpoint = 编辑命中次数断点
set-hit-condition-breakpoint = 设置命中次数断点
disable = 禁用
enable = 启用
open-git-blame = 打开 Git Blame
close-git-blame = 关闭 Git Blame
clear-run-status = 清除运行状态
right-click-for-more-options = 右键点击查看更多选项
modifier-click-to-add-a-target = { $modifier }-点击添加{ $target }
modifier-click-to-disable = { $modifier }-点击以禁用
no-executable-code-is-associated-with-this-line = 此行没有关联可执行代码。

## Breakpoint and bookmark prompts

message-to-log-when-a-breakpoint-is-hit-expressions-within-are-interpolated = 命中断点时记录的消息。{ "{}" } 中的表达式会被求值。
condition-when-a-breakpoint-is-hit-expressions-within-are-interpolated = 命中断点的条件。{ "{}" } 中的表达式会被求值。
how-many-breakpoint-hits-to-ignore = 要忽略的断点命中次数
enter-bookmark-label-optional = 输入书签标签（可选）

## Multi-buffer excerpt header

expand-excerpt = 扩展摘录
fold-excerpt = 折叠摘录
unfold-excerpt = 展开摘录
keystroke-to-toggle-all = { $keystroke } 可切换全部
show-symbol-outline = 显示符号大纲
right-click-to-copy-path = 右键点击复制路径

## Diff view style

unified = 统一视图
split = 分栏视图
split-when-wider-than-columns-columns = 宽度超过 { $columns } 列时分栏
click-to-change-min-width = 点击可修改最小宽度

## Diff hunks

next-hunk = 下一个变更块
previous-hunk = 上一个变更块

## Diff review comments

add-a-review-comment = 添加审阅评论…
add-review-drag-to-select-multiple-lines = 添加审阅（拖动可选择多行）
add-comment = 添加评论
count-comment = { $count } 条评论
count-comments = { $count } 条评论
line-line = 第 { $line } 行
lines-start-end = 第 { $start }-{ $end } 行

## Permalinks

failed-to-copy-permalink-error = 复制永久链接失败：{ $error }
failed-to-open-permalink-error = 打开永久链接失败：{ $error }

## Hover popover

unicode-character-u-code = Unicode 字符 U+{ $code }
copy-diagnostic = 复制诊断信息

## Signature help

previous-signature = 上一个签名
next-signature = 下一个签名

## Edit prediction

hold = 按住
preview = 预览
jump = 跳转
scroll = 滚动
jump-to-edit = 跳转到修改处
jump-to-file-name = 跳转到 { $file_name }
conflict-with-accept-keybinding = 与接受快捷键冲突
your-keymap-currently-overrides-the-default-accept-keybinding-to-continue-assign-one-keybinding-for-the-editor-accepteditprediction-action = 你的按键映射覆盖了默认的接受快捷键。请为 `editor::AcceptEditPrediction` 操作指定一个快捷键以继续。
assign-keybinding = 指定快捷键

## Code navigation results

references = 引用
references-to-target = { $target } 的引用
implementations = 实现
definitions = 定义
declarations = 声明
types = 类型
kind-for-target = { $target } 的{ $kind }

## Code lens

t-0-references = 0 个引用
