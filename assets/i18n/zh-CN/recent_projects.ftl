# Simplified Chinese (zh-CN) catalog for recent projects and remote connections.
#
# Product and technology names (Zed, SSH, WSL, Docker), host names, ports, shell
# command snippets and file names are deliberately left untranslated.

## Project picker

search-projects = 搜索项目…
current-folders = 当前文件夹
this-window = 当前窗口
no-matches = 无匹配项
recently-opened-projects-will-show-up-here = 最近打开的项目会显示在这里
actions = 操作
activate = 激活
failed-to-open-project = 打开项目失败

## Opening projects and folders

open-local-folders = 打开本地文件夹
open-remote-folder = 打开远程文件夹
# The File menu's `open-folder` is "Open Folder…", whose ellipsis promises a
# dialog; this one names the folder it opens in the row beside it.
open-folder-action = 打开文件夹
open-in-new-window = 在新窗口中打开
open-project-in-this-window = 在当前窗口中打开项目
open-project-in-new-window = 在新窗口中打开项目
add-folder-to-this-project = 将文件夹添加到此项目
add-folders-to-this-project = 将文件夹添加到此项目
as-a-multi-root-folder = 作为多根文件夹

## Removing entries

remove-folder = 移除文件夹
remove-folder-from-project = 从项目中移除文件夹
remove-from-window = 从窗口中移除
remove-project-from-window = 从窗口中移除项目
remove-from-recent-projects = 从最近的项目中移除

## Remote server picker

search-remote-projects = 搜索远程项目…
no-matching-remote-projects = 没有匹配的远程项目。
connect-ssh-server = 连接 SSH 服务器
connect-dev-container = 连接开发容器
add-wsl-distro = 添加 WSL 发行版
view-server-options = 查看服务器选项
delete-remote-project = 删除远程项目
select = 选择
# The navigation history arrow owns `go-back`; this one steps back through the
# connection wizard.
go-back-a-step = 返回
exit = 退出

## Adding an SSH server

enter-the-command-you-use-to-ssh-into-this-server = 输入你用于 SSH 连接此服务器的命令。
could-not-parse-error = 无法解析：{ $error }

## Server options

edit-nickname = 编辑昵称
add-nickname-to-server = 为服务器添加昵称
add-a-nickname-for-this-server = 为此服务器添加一个昵称
copy-server-address = 复制服务器地址
copied-server-address-address-to-clipboard = 已将服务器地址（{ $address }）复制到剪贴板
remove-server = 移除服务器
remove-server-name = 要移除服务器 `{ $name }` 吗？
remove-distro = 移除发行版
remove-wsl-distro-name = 要移除 WSL 发行版 `{ $name }` 吗？
enter-wsl-distro-name = 输入 WSL 发行版名称

## Dev containers

dev-containers = 开发容器
select-dev-container-configuration = 选择开发容器配置
start-dev-container = 启动开发容器
open-devcontainer-json = 打开 devcontainer.json
creating-dev-container = 正在创建开发容器
error-creating-dev-container = 创建开发容器时出错：
failed-to-start-dev-container-see-logs-for-details = 启动开发容器失败。详情请查看日志
cannot-open-dev-container-from-remote-project = 无法从远程项目打开开发容器
open-zed-log = 打开 Zed 日志
name-contains-a-dev-container-configuration-file-would-you-like-to-re-open-it-in-a-container = { $name } 中包含开发容器配置文件，是否在容器中重新打开它？
yes-open-in-container = 是，在容器中打开
don-t-show-again = 不再显示

## Connection failures

failed-to-connect = 连接失败
failed-to-connect-over-ssh = 通过 SSH 连接失败
failed-to-connect-to-wsl = 连接 WSL 失败
failed-to-connect-to-dev-container = 连接开发容器失败
failed-to-connect-to-mock-server = 连接模拟服务器失败

## Disconnected overlay

reconnect = 重新连接
failed-to-reconnect = 重新连接失败
your-connection-to-the-remote-project-has-been-lost = 与远程项目的连接已断开。
your-connection-to-host-has-been-lost-due-to-the-server-process-exiting-unexpectedly = 与 { $host } 的连接已断开，因为服务器进程意外退出。
your-connection-to-host-has-been-lost-due-to-the-server-not-responding = 与 { $host } 的连接已断开，因为服务器没有响应。
unsaved-changes-are-stored-locally = 未保存的更改已保存在本地。

## Opening folders inside WSL

invalid-path = 路径无效
invalid-path-specified-when-trying-to-open-a-folder-inside-wsl = 尝试在 WSL 中打开文件夹时指定的路径无效。
please-note-that-zed-currently-does-not-support-opening-network-share-folders-inside-wsl = 请注意，Zed 目前不支持在 WSL 中打开网络共享文件夹。

## Prompt buttons

# `ok`, `cancel` and `retry` are defined in common.ftl.

yes-remove-it = 是，移除
no-keep-it = 否，保留
