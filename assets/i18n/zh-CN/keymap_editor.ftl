# Simplified Chinese (zh-CN) catalog for keymap_editor (按键映射编辑器).
#
# 快捷键字面量（如 cmd-shift-p）与 action 标识符均保持不译。
# 与 zed / git_ui / agent_ui / settings_ui / common 共用的词条直接复用其定义，不在此重复。

## 顶部工具栏与过滤

filter-action-names = 筛选操作名称…
keymap-editor = 按键映射编辑器
toggle-exact-match-mode = 切换精确匹配模式
search-by-keystrokes = 按按键搜索
edit-in-json = 在 JSON 中编辑
create-keybinding = 创建快捷键
filters = 筛选器
no-action = 无操作
categories = 分类
default = 默认

## 表格列与行

action = 操作
keystrokes = 按键
null = 空
no-arguments = 无参数
this-action-is-unbound = 此操作未绑定
show-matching-keybinds = 显示匹配的快捷键
edit-this-binding = 编辑此绑定
use-alt-click-to-edit-this-binding = 按住 Alt 点击可编辑此绑定
this-binding-is-overridden-by-other-bindings = 此绑定已被其他绑定覆盖。
this-binding-is-overridden = 此绑定已被覆盖。
your-keymap = 你的按键映射
the-vim-keymap = Vim 按键映射
your-base-keymap = 你的基础按键映射
view-conflicts = 查看冲突
use-alt-click-to-show-all-conflicts = 按住 Alt 点击可显示所有冲突
this-keybinding-is-overridden-by-the-binding-binding-from-source = 此快捷键已被来自{ $source }的「{ $binding }」绑定覆盖。
no-conflicting-keybinds-found-that-match-the-provided-query = 未找到与查询匹配的冲突快捷键
no-conflicting-keybinds-found = 未找到冲突的快捷键
no-keybinds-found-matching-the-entered-keystrokes = 未找到与所输入按键匹配的快捷键
no-matches-found-for-the-provided-query = 未找到与查询匹配的结果

## 右键菜单

copy-action = 复制操作
copy-context = 复制上下文
show-matching-keybindings = 显示匹配的快捷键

## 创建/编辑快捷键弹窗

edit-keybinding = 编辑快捷键
edit-keystroke = 编辑按键
edit-arguments = 编辑参数
edit-context = 编辑上下文
keybinding-context = 快捷键上下文
type-an-action-name = 输入操作名称
action-arguments = 操作参数
there-are-count-bindings-with-the-same-keystrokes = 有 { $count } 个绑定使用相同的按键
keymap-view-matching-keybindings = 查看
your-keybind-would-conflict-with-the-name-action-and-count-other-bindings = 你的快捷键将与「{ $name }」操作及另外 { $count } 个绑定冲突
your-keybind-would-conflict-with-the-name-action = 你的快捷键将与「{ $name }」操作冲突
your-keybind-would-conflict-with-other-actions = 你的快捷键将与其他操作冲突
saved-edits-to-the-action-action = 已保存对 { $action } 操作的修改。

## 按键录制组件

search-keystroke-label = 搜索
stop-searching = 停止搜索
stop-recording = 停止录制
start-searching = 开始搜索
start-recording = 开始录制
clear-keystrokes = 清除按键
hit-it-three-times-to-execute = 连按三次即可执行
