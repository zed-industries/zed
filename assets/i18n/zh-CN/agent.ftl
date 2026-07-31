# Simplified Chinese (zh-CN) catalog for the native agent's tool calls and
# permission prompts.
#
# Only the text the agent panel renders is localized here. Tool *results* stay in
# English on purpose: they are fed back to the language model, and translating
# them would degrade the model's ability to act on them.
#
# Product names (Zed, MCP, WSL), file names, paths, commands and regexes are
# deliberately left untranslated.

## 工具卡片标题：读写文件

read-file = 读取文件
read-file-path = 读取文件 { $path }
read-file-path-from-line-start = 读取文件 { $path }（从第 { $start } 行开始）
read-file-path-lines-start-end = 读取文件 { $path }（第 { $start }-{ $end } 行）
editing-file = 正在编辑文件
writing-file = 正在写入文件

## 工具卡片标题：路径操作

agent-tool-copy-path = 复制路径
copy-src-to-dest = 将 { $src } 复制到 { $dest }
agent-tool-move-path = 移动路径
move-src-to-dest = 将 { $src } 移动到 { $dest }
rename-src-to-filename = 将 { $src } 重命名为 { $filename }
agent-tool-delete-path = 删除路径
delete-target = 删除 { $path }
agent-tool-create-directory = 创建目录
create-directory-path = 创建目录 { $path }
list-directory = 列出目录
list-the-path-directory-s-contents = 列出 { $path } 目录的内容

## 工具卡片标题：搜索

find-paths = 查找路径
find-paths-matching-glob = 查找匹配 { $glob } 的路径
search-with-regex = 用正则搜索
search-files-for-regex-regex = 在文件中搜索正则 { $regex }
search-files-for-regex-regex-case-sensitive = 在文件中搜索正则 { $regex }（区分大小写）
get-page-page-of-search-results-for-regex-regex = 获取正则 { $regex } 搜索结果的第 { $page } 页
get-page-page-of-search-results-for-regex-regex-case-sensitive = 获取正则 { $regex } 搜索结果的第 { $page } 页（区分大小写）

## 工具卡片标题：语言智能

find-references = 查找引用
find-references-to-symbol = 查找 { $symbol } 的引用
agent-tool-go-to-definition = 跳转到定义
go-to-definition-of-symbol = 跳转到 { $symbol } 的定义
agent-tool-rename-symbol = 重命名符号
rename-symbol-to-new-name = 将 { $symbol } 重命名为 { $new_name }
get-code-actions = 获取代码操作
get-code-actions-for-symbol = 获取 { $symbol } 的代码操作
apply-code-action = 应用代码操作
apply-code-action-index = 应用代码操作 #{ $index }
apply-code-action-title = 应用代码操作：{ $title }
check-project-diagnostics = 检查项目诊断
check-diagnostics-for-path = 检查 { $path } 的诊断

## 工具卡片标题：网络

fetch-url = 抓取 URL
fetch-target = 抓取 { $url }
searching-the-web = 正在搜索网页
search-the-web-for-query = 搜索网页：{ $query }
searched-the-web-1-result = 已搜索网页：1 条结果
searched-the-web-count-results = 已搜索网页：{ $count } 条结果
web-search-failed = 网页搜索失败

## 工具卡片标题：会话与智能体

create-thread = 创建会话
create-thread-title = 创建会话：{ $title }
agent-tool-spawning-agent = 正在启动智能体
list-agents-and-models = 列出智能体与模型
named-skill = { $name } 技能

## 权限提示

edit-path = 编辑 { $path }
title-local-settings = { $title }（本地设置）
title-settings = { $title }（设置）
title-agent-skills = { $title }（智能体技能）
path-points-outside-the-project-symlink-to-target = { $path } 指向项目之外（符号链接到 { $target }）
targets-symlinks-outside-project = { $targets }（符号链接指向项目之外）

## 权限选项按钮

always-for-tool = 始终允许 { $tool }
always-for-tool-mcp-tool = 始终允许 MCP 工具 { $tool }
always-for-pattern = 始终允许 { $pattern }
always-for-pattern-commands = 始终允许 { $pattern } 命令
allow-once = 仅允许一次
allow-always = 始终允许
allow-for-this-thread = 在本会话中允许
allow-for-this-subagent = 在本子智能体中允许
abort = 中止

## 沙箱回退提示

retry-attempt-retries = 重试（第 { $retries } 次尝试）
run-without-sandbox-once = 本次不使用沙箱运行
run-without-sandbox-for-this-thread = 在本会话中不使用沙箱运行
run-without-sandbox-for-this-subagent = 在本子智能体中不使用沙箱运行
always-run-without-sandbox = 始终不使用沙箱运行

## 未保存修改的提示

this-file-has-unsaved-changes-do-you-want-to-save-or-discard-them-before-the-agent-continues-editing = 该文件有未保存的修改。在智能体继续编辑之前，要保存还是放弃这些修改？
this-file-has-unsaved-changes-and-the-agent-wants-to-overwrite-it = 该文件有未保存的修改，而智能体想覆盖它。
discard = 放弃
