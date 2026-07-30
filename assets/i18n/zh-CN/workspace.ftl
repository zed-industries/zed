# Simplified Chinese (zh-CN) catalog for the workspace shell: docks, panes,
# tabs, the status bar, notifications and the welcome page.
#
# Product names (Zed, Git, WSL, MCP, ACP), file names (.zed/settings.json) and
# telemetry event names are deliberately left untranslated. Keys already defined
# in zed.ftl, title_bar.ftl or recent_projects.ftl (save, save-all, open-file,
# open-settings, split-left/right/up/down, zoom-in, zoom-out, new, new-window,
# recent-projects, go-back, ok, cancel) are reused from there and intentionally
# absent here.

## Window regions (exposed to assistive technology)

editor = 编辑器
left-dock = 左侧边栏
right-dock = 右侧边栏
bottom-dock = 底部面板

## Docks and panels

close-left-dock = 关闭左侧边栏
close-right-dock = 关闭右侧边栏
close-bottom-dock = 关闭底部面板
dock-left = 停靠到左侧
dock-right = 停靠到右侧
dock-bottom = 停靠到底部
flex-width = 自适应宽度
fixed-width = 固定宽度
hide-button = 隐藏按钮
left = 左侧
right = 右侧
open-threads-sidebar = 打开会话侧边栏

## Tab context menu

close = 关闭
close-others = 关闭其他
close-multibuffers = 关闭多缓冲区
close-left = 关闭左侧
close-right = 关闭右侧
close-clean = 关闭未修改
close-all = 全部关闭
close-tab = 关闭标签页
pin-tab = 固定标签页
unpin-tab = 取消固定标签页
make-tab-read-only = 将标签页设为只读
make-tab-editable = 将标签页设为可编辑
copy-path = 复制路径
copy-relative-path = 复制相对路径
reveal-in-project-panel = 在项目面板中显示
open-in-terminal = 在终端中打开

## Read-only tabs

unlock-tab = 解锁标签页
this-will-make-this-tab-editable = 这会使此标签页变为可编辑
locked-tab = 标签页已锁定
this-tab-is-read-only = 此标签页为只读
read-only-tab = 只读标签页

## Tab bar

go-back = 后退
go-forward = 前进
new-file = 新建文件
# Explicit keys (t!(key = …)): the derived keys `new` / `open-file` are owned by
# the application menu in zed.ftl, whose English keeps a different spelling.
new-menu = 新建…
open-file-button = 打开文件…
search-project = 搜索项目
search-symbols = 搜索符号
new-terminal = 新建终端
new-center-terminal = 在中央区域新建终端
split-pane = 拆分窗格

## Saving and closing

do-you-want-to-save-changes-to-the-following-files = 是否保存以下文件的更改？
do-you-want-to-save-all-changes-in-the-following-files = 是否保存以下文件中的所有更改？
this-file-has-changed-on-disk-since-you-started-editing-it-do-you-want-to-overwrite-it = 自你开始编辑以来，此文件在磁盘上已被修改。是否覆盖它？
this-file-has-been-deleted-on-disk-since-you-started-editing-it-do-you-want-to-recreate-it = 自你开始编辑以来，此文件已在磁盘上被删除。是否重新创建它？
path-contains-unsaved-edits-do-you-want-to-save-it = { $path } 有未保存的编辑。是否保存？
this-buffer-contains-unsaved-edits-do-you-want-to-save-it = 此缓冲区有未保存的编辑。是否保存？
unable-to-save-file-error = 无法保存文件：{ $error }
failed-to-save = 保存失败
discard-all = 全部放弃
discard-edits = 放弃编辑
don-t-save = 不保存
close-without-saving = 不保存并关闭
overwrite = 覆盖
untitled = 未命名

## Notifications

dismiss = 忽略
suppress = 屏蔽
click-to-close = 点击以关闭
suppress-with-shift-click = 按住 Shift 点击可屏蔽
shift-click-to-suppress = 按住 Shift 点击可屏蔽
copy-description = 复制描述
copy-message = 复制消息
error-error = 错误：{ $error }
failed-to-load-the-database-file = 无法加载数据库文件。
file-an-issue = 提交问题
task-spawn-failed-error = 任务启动失败：{ $error }
open-in-wsl = 在 WSL 中打开
path-is-inside-a-wsl-filesystem-some-features-may-not-work-unless-you-open-it-with-wsl-remote = { $path } 位于 WSL 文件系统中，除非以 WSL 远程方式打开，否则部分功能可能无法正常工作
path-does-not-exist = 「{ $path }」不存在

## Opening files and projects

could-not-open-file = 无法打开文件
open-in-default-app = 用默认应用打开
cannot-drop-files-on-a-remote-project = 无法将文件拖放到远程项目中
you-cannot-add-folders-to-someone-else-s-project = 你不能向他人的项目中添加文件夹
empty-project = 空项目
are-you-sure-you-want-to-restart = 确定要重启吗？
restart = 重启

## Collaboration

do-you-want-to-leave-the-current-call = 是否离开当前通话？
close-window-and-hang-up = 关闭窗口并挂断
do-you-want-to-switch-channels = 是否切换频道？
leaving-this-call-will-unshare-your-current-project = 离开此通话将停止共享你当前的项目。
yes-join-channel = 是，加入频道
failed-to-join-channel = 加入频道失败
failed-to-join-project = 加入项目失败
please-sign-in-to-continue = 请登录后继续。
your-are-running-an-unsupported-version-of-zed-please-update-to-continue = 你正在运行不受支持的 Zed 版本。请更新后继续。
no-matching-channel-was-found-please-check-the-link-and-try-again = 未找到匹配的频道。请检查链接后重试。
this-channel-is-private-and-you-do-not-have-access-please-ask-someone-to-add-you-and-try-again = 此频道为私有频道，你没有访问权限。请让他人将你加入后重试。
please-check-your-internet-connection-and-try-again = 请检查你的网络连接后重试。
please-try-again = 请重试。
user-s-screen = { $user } 的屏幕
user-is-in-an-unshared-pane = { $user } 正在一个未共享的窗格中
follow-user-to-their-active-project = 跟随 { $user } 前往其当前项目
user-is-viewing-an-unshared-zed-project = { $user } 正在查看一个未共享的 Zed 项目
user-is-viewing-a-window-outside-of-zed = { $user } 正在查看 Zed 之外的窗口

## Project trust

unrecognized-project = 无法识别的项目
unrecognized-projects-count = 无法识别的项目（{ $count } 个）
untrusted-projects-are-opened-in-restricted-mode-to-protect-your-system = 未受信任的项目会以受限模式打开，以保护你的系统。
review-zed-settings-json-for-any-extensions-or-commands-configured-by-this-project = 请检查 .zed/settings.json 中此项目配置的扩展或命令。
restricted-mode-prevents = 受限模式会阻止：
project-settings-from-being-applied = 应用项目设置
language-servers-from-running = 运行语言服务器
mcp-server-integrations-from-installing = 安装 MCP 服务器集成
stay-in-restricted-mode = 保持受限模式
trust-and-continue = 信任并继续
folder-to-trust = 要信任的文件夹
trust-all-projects-in = 信任以下目录中的所有项目
trust-all-single-files = 信任所有单个文件
trust-all-projects-in-the-folder-folder = 信任 { $folder } 文件夹中的所有项目
trust-all-projects-in-the-parent-folders = 信任各上级文件夹中的所有项目
enter-a-folder-to-trust = 请输入要信任的文件夹
enter-an-absolute-folder-path = 请输入绝对文件夹路径
must-be-a-parent-folder-of-the-project = 必须是该项目的上级文件夹

## Welcome page

welcome = 欢迎
welcome-to-zed = 欢迎使用 Zed
welcome-back-to-zed = 欢迎回到 Zed
the-editor-for-what-s-next = 面向未来的编辑器
get-started = 开始使用
open-project = 打开项目
clone-repository = 克隆仓库
open-command-palette = 打开命令面板
configure = 配置
customize-keymaps = 自定义按键映射
explore-extensions = 探索扩展
collaborate-with-agents = 与智能体协作
run-multiple-threads-at-once-mix-and-match-any-acp-compatible-agent-and-keep-work-conflict-free-with-worktrees = 同时运行多个会话，自由搭配任何兼容 ACP 的智能体，并借助工作树让各项工作互不冲突。
open-agent-panel = 打开智能体面板
return-to-onboarding = 返回新手引导
