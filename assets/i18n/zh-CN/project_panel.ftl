# Simplified Chinese (zh-CN) catalog for the project panel.
#
# Platform file manager names (Finder, File Explorer), file names
# (.gitignore, .git/info/exclude), settings keys (file_scan_exclusions) and
# Git path attributes (private) are deliberately left untranslated.

## Context menu

open-markdown-preview = 打开 Markdown 预览
search-inside = 在其中搜索
new-folder = 新建文件夹
reveal-in-finder = 在 Finder 中显示
reveal-in-file-explorer = 在 File Explorer 中显示
reveal-in-file-manager = 在文件管理器中显示
find-in-folder = 在文件夹中查找…
unfold-directory = 展开目录
fold-directory = 折叠目录
compare-marked-files = 比较已标记的文件
duplicate = 创建副本
download = 下载…
# Confirm button of the native folder picker opened by "Download…".
download-prompt = 下载
view-history = 查看历史
rename = 重命名
trash = 移到废纸篓
add-folders-to-project = 将文件夹添加到项目…
remove-from-project = 从项目中移除
expand-all = 全部展开
collapse-all = 全部折叠

## New file and rename validation

file-or-directory-name-cannot-be-empty = 文件或目录名不能为空。
file-or-directory-name-contains-leading-or-trailing-whitespace = 文件或目录名的开头或结尾包含空白字符。
file-or-directory-name-already-exists-at-location-please-choose-a-different-name = 该位置已存在名为「{ $name }」的文件或目录，请换一个名称。

## Excluded directories

created-an-excluded-directory-at-path = 已在 { $path } 创建一个被排除的目录。
alter-file-scan-exclusions-in-the-settings-to-show-it-in-the-panel = 请修改设置中的 `file_scan_exclusions` 以在面板中显示它

## Git operations

discard-changes-to-path = 要放弃对 { $path } 的更改吗？
failed-to-restore-name-error = 恢复 { $name } 失败：{ $error }
failed-to-add-to-gitignore-error = 添加到 .gitignore 失败：{ $error }
failed-to-add-to-git-info-exclude-error = 添加到 .git/info/exclude 失败：{ $error }

## Trash and delete confirmation

do-you-want-to-trash-path = 要将 { $path } 移到废纸篓吗？
are-you-sure-you-want-to-permanently-delete-path = 确定要永久删除 { $path } 吗？
do-you-want-to-trash-the-following-count-files = 要将以下 { $count } 个文件移到废纸篓吗？
are-you-sure-you-want-to-permanently-delete-the-following-count-files = 确定要永久删除以下 { $count } 个文件吗？
it-has-unsaved-changes-which-will-be-lost = 它有未保存的更改，这些更改将会丢失。
t-1-of-these-has-unsaved-changes-which-will-be-lost = 其中 1 个有未保存的更改，这些更改将会丢失。
count-of-these-have-unsaved-changes-which-will-be-lost = 其中 { $count } 个有未保存的更改，这些更改将会丢失。
t-1-file-not-shown = .. 还有 1 个文件未显示
count-files-not-shown = .. 还有 { $count } 个文件未显示
this-cannot-be-undone = 此操作无法撤销。
failed-to-trash-failed-count-of-total-count-file = 未能将 { $total_count } 个文件中的 { $failed_count } 个移到废纸篓。
failed-to-trash-failed-count-of-total-count-files = 未能将 { $total_count } 个文件中的 { $failed_count } 个移到废纸篓。
failed-to-delete-failed-count-of-total-count-file = 未能删除 { $total_count } 个文件中的 { $failed_count } 个。
failed-to-delete-failed-count-of-total-count-files = 未能删除 { $total_count } 个文件中的 { $failed_count } 个。

## Opening entries

failed-to-open-file = 打开文件失败
disconnected-from-ssh-host = 已断开与 SSH 主机的连接
disconnected-from-remote-project = 已断开与远程项目的连接
path-is-not-shared-by-the-host-this-could-be-because-it-has-been-marked-as-private = { $path } 未被主机共享，可能是因为它已被标记为 `private`

## Downloading from a remote project

downloading-index-total-files = 正在下载 { $index }/{ $total } 个文件…
downloaded-total-files = 已下载 { $total } 个文件

## Dragging files into the panel

a-file-or-folder-with-name-name-already-exists-in-the-destination-folder-do-you-want-to-replace-it = 目标文件夹中已存在名为 { $name } 的文件或文件夹。要替换它吗？
replace = 替换

## Entry details

symbolic-link = 符号链接
count-entries = { $count } 个条目

## Undo and redo

undo-failed = 撤销失败
redo-failed = 重做失败
