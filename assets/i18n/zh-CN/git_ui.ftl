# Simplified Chinese (zh-CN) catalog for the Git UI: panel, commit editor,
# branch and stash pickers, diff views, blame, git graph and conflict resolution.
#
# Product names (Zed, Git, GitHub, GitLab), git subcommands and flags
# (`git commit`, `--force-with-lease`), remote names, branch names, commit SHAs
# and file names are deliberately left untranslated.

## Shared verbs and short labels

apply = 应用
changes = 更改
commit = 提交
create = 创建
deleting = 正在删除…
drop = 丢弃
history = 历史
list = 列表
loading = 加载中…
name = 名称
none = 无
path = 路径
pop = 弹出
restore = 恢复
switch = 切换
tree = 树形
unknown = 未知
view-log = 查看日志
view-options = 视图选项
view-diff = 查看差异
view-file = 查看文件

## Git panel sections and columns

conflicts = 冲突
tracked = 已跟踪
untracked = 未跟踪
staged = 已暂存
unstaged = 未暂存
staged-unstaged = 已暂存与未暂存
tracked-untracked = 已跟踪与未跟踪
columns = 列
graph = 图谱
description = 描述
date = 日期
author = 作者
toggle-tab-tab = 切换「{ $tab }」标签页

## Staging

stage = 暂存
unstage = 取消暂存
stage-all = 全部暂存
unstage-all = 全部取消暂存
stage-file = 暂存文件
unstage-file = 取消暂存文件
stage-hunk = 暂存变更块
unstage-hunk = 取消暂存变更块
stage-selected-hunks = 暂存所选变更块
unstage-selected-hunks = 取消暂存所选变更块
stage-all-changes = 暂存全部更改
unstage-all-changes = 取消暂存全部更改
stage-and-go-to-next-hunk = 暂存并跳转到下一个变更块
unstage-and-go-to-next-hunk = 取消暂存并跳转到下一个变更块
toggle-staged = 切换暂存状态
action-folder = { $action }文件夹
all-conflicts-marked-as-resolved = 所有冲突均已标记为已解决
conflict-marked-as-resolved = 此冲突已标记为已解决
conflicts-marked-as-resolved = 这些冲突已标记为已解决
failed-to-stage-file = 暂存文件失败
failed-to-unstage-file = 取消暂存文件失败

## Restoring, discarding and trashing

restore-all = 全部恢复
restore-all-changes = 恢复全部更改
restore-file = 恢复文件
restore-hunk = 恢复变更块
restore-selected-hunk = 恢复所选变更块
restore-selected-hunks = 恢复所选变更块
discard-changes = 放弃更改
discard-tracked-changes = 放弃已跟踪文件的更改
discard-changes-to-these-files = 放弃对这些文件的更改？
trash-file = 移到回收站
trash-untracked-files = 将未跟踪的文件移到回收站
trash-these-files = 将这些文件移到回收站？
trash-name = 将 { $name } 移到回收站？
and-count-more = 还有 { $count } 个…
are-you-sure = 确定要继续吗？
are-you-sure-you-want-to-restore-path = 确定要恢复 { $path } 吗？
are-you-sure-you-want-to-discard-changes-to-path = 确定要放弃对 { $path } 的更改吗？
failed-to-trash-file = 移到回收站失败
failed-to-trash-files = 移到回收站失败

## Committing

commit-tracked = 提交已跟踪文件
amend = 修补提交
amend-tracked = 修补已跟踪文件
signoff = 添加签署
skip-hooks = 跳过钩子
uncommit = 撤销提交
commit-in-progress = 正在提交
no-commit-message = 无提交信息
no-changes-to-commit = 没有可提交的更改
enter-commit-message = 输入提交信息
generate-commit-message = 生成提交信息
generating-commit = 正在生成提交…
generating-commit-message = 正在生成提交信息…
cancel-commit-message-generation = 取消生成提交信息
configure-an-llm-provider-to-generate-commit-messages = 需要配置 LLM 提供方才能生成提交信息。
configure-provider = 配置提供方
see-docs = 查看文档
commit-message-title-exceeds-limit-character-limit = 提交信息标题超过 { $limit } 个字符的限制。
failed-to-generate-commit-message-error = 生成提交信息失败：{ $error }
this-will-update-your-most-recent-commit = 这将更新你最近的一次提交。
there-are-still-conflicts-you-must-stage-these-before-committing = 仍存在冲突，必须先暂存这些文件才能提交
you-must-resolve-conflicts-before-committing = 提交前必须先解决冲突
you-do-not-have-write-access-to-this-project = 你对此项目没有写入权限
add-co-authored-by = 添加 Co-authored-by
remove-co-authored-by = 移除 Co-authored-by
expand-commit-editor = 展开提交编辑器
collapse-commit-editor = 折叠提交编辑器
open-commit-modal = 打开提交对话框
this-commit-was-already-pushed-to-remotes = 此提交已推送到 { $remotes }。

## Commit view and history

commit-sha = 提交 SHA
commit-short-sha = 提交 { $short_sha }
ref-name = 引用 { $name }
copy-sha = 复制 SHA
copy-commit-sha = 复制提交 SHA
commit-sha-copied = 提交 SHA 已复制！
copy-email = 复制邮箱
email-copied = 邮箱已复制！
copy-ref-name = 复制引用名
copy-tag = 复制标签
copy-tag-tag = 复制标签：{ $tag }
custom-commands = 自定义命令
open-permalink = 打开永久链接
open-file-in-project = 在项目中打开文件
expand-commit-description = 展开提交描述
fold-commit-description = 折叠提交描述
view-commit = 查看提交
view-commit-diff = 查看提交差异
view-commit-failed = 查看提交失败
view-changes = 查看更改
view-file-history = 查看文件历史
view-branch-diff = 查看分支差异
view-on-provider = 在 { $provider } 上查看
buffer-search = 缓冲区搜索
show-in-git-graph = 在 Git 图谱中显示
open-git-graph = 打开 Git 图谱
git-graph = Git 图谱
path-history = 路径历史
search-commits = 搜索提交…
select-next-match = 选择下一个匹配项
select-previous-match = 选择上一个匹配项
no-commits-found = 未找到提交
no-commits-yet = 尚无提交
loading-commit-history = 正在加载提交历史…
failed-to-load-commit-history = 加载提交历史失败
failed-to-load-commits = 加载提交失败
no-repository-found = 未找到仓库
contains-unpushed-changes-sha = 包含未推送的更改 — { $sha }
error-loading-error = 加载出错：{ $error }
t-1-changed-file = 1 个已更改文件
count-changed-files = { $count } 个已更改文件
show-flat-view = 显示平铺视图
show-tree-view = 显示树形视图
toggle-folder = 切换文件夹
error-parsing-date = 日期解析失败
binary-file-not-shown = （未显示二进制文件）
no-name = <无名称>

## Stash

stashes = 贮藏
stash-all = 全部贮藏
stash-pop = 弹出贮藏
pop-stash = 弹出贮藏
drop-stash = 丢弃贮藏
view-stash = 查看贮藏
select-a-stash = 选择一个贮藏…
no-stashes-found = 未找到贮藏
failed-to-apply-stash = 应用贮藏失败
failed-to-pop-stash = 弹出贮藏失败
failed-to-drop-stash = 丢弃贮藏失败
stash-has-changed-not-applying = 贮藏已变更，未执行应用
stash-has-changed-pop-aborted = 贮藏已变更，已中止弹出
stash-has-changed-drop-aborted = 贮藏已变更，已中止丢弃
action-stash-ref = { $action } { $stash_ref }？
toggle-stash-picker = 切换贮藏选择器
toggle-branch-picker = 切换分支选择器

## Branches

branches = 分支
all-branches = 全部分支
local-branches = 本地分支
remote-branches = 远程分支
current-branch = 当前分支
current-branches = 当前分支
selected-branch = 已选分支
no-branch = 无分支
filter-branches = 筛选分支
switch-branch = 切换分支
switch-or-type-to-create-a-branch = 切换分支，或输入名称以创建分支…
create-branch-name = 创建分支「{ $name }」…
create-remote-name = 创建远程「{ $name }」
create-remote-repository = 创建远程仓库
enter-a-name-for-this-remote = 为此远程输入名称…
remote-name-can-t-be-empty = 远程名称不能为空
based-off-source = 基于 { $source }
based-off-the-current-branch = 基于当前分支
create-new-from-branch = 从 { $branch } 新建
delete-branch = 删除分支
force-delete-branch = 强制删除分支
force-delete = 强制删除
hold-alt-to-force-delete = 按住 alt 可强制删除
branch-branch-is-not-fully-merged-force-delete-it = 分支「{ $branch }」尚未完全合并，是否强制删除？
failed-to-change-branch = 切换分支失败
failed-to-create-branch = 创建分支失败
failed-to-create-remote = 创建远程失败
failed-to-rename-branch = 重命名分支失败
rename-branch-branch = 重命名分支（{ $branch }）
some-branches-could-not-be-loaded-error = 部分分支无法加载：{ $error }
select-base-branch = 选择基准分支
base-branch = 基准：{ $branch }
changes-since-branch = 自 { $branch } 以来的更改

## Remote operations

fetch = 获取
fetch-from = 从指定远程获取
fetch-updates-from-remote = 从远程获取更新
fetch-in-progress = 正在获取…
fetch-already-up-to-date = 获取：已是最新
pull = 拉取
pull-rebase = 拉取（变基）
pull-in-progress = 正在拉取…
pull-already-up-to-date = 拉取：已是最新
push = 推送
push-to = 推送到指定远程
force-push = 强制推送
push-in-progress = 正在推送…
push-committed-changes-to-remote = 将已提交的更改推送到远程
push-everything-is-up-to-date = 推送：一切已是最新
publish = 发布
publish-branch-to-remote = 将分支发布到远程
republish = 重新发布
re-publish-branch-to-remote = 将分支重新发布到远程
pushed-branch-to-remote = 已将 { $branch } 推送到 { $remote }
synchronized-with-remote = 已与 { $remote } 同步
synchronized-with-remotes = 已与各远程同步
received-1-file-change-from-remote = 已从 { $remote } 接收 1 个文件更改
received-count-file-changes-from-remote = 已从 { $remote } 接收 { $count } 个文件更改
fast-forwarded-from-remote = 已从 { $remote } 快进
merged-1-file-change-from-remote = 已从 { $remote } 合并 1 个文件更改
merged-count-file-changes-from-remote = 已从 { $remote } 合并 { $count } 个文件更改
merged-from-remote = 已从 { $remote } 合并
successfully-rebased-from-remote = 已成功基于 { $remote } 变基
successfully-pulled-from-remote = 已成功从 { $remote } 拉取
no-remote-available-to-push-to-add-a-remote-to-be-able-to-publish-changes = 没有可推送的远程。请先添加一个远程，然后才能发布更改。
pick-which-remote-to-fetch = 选择要获取的远程
pick-which-remote-to-push-to = 选择要推送到的远程
create-pull-request = 创建拉取请求
create-merge-request = 创建合并请求
view-merge-request = 查看合并请求
git-action-failed = git { $action } 失败
output-from-git-operation = git { $operation } 的输出
git-fetch-failed-for-branch = { $branch } 的 git fetch 失败
use-local-branch = 使用本地的 { $branch }
show-error-logs = 显示错误日志

## Diff views

uncommitted-changes = 未提交的更改
staged-changes = 已暂存的更改
unstaged-changes = 未暂存的更改
no-uncommitted-changes = 没有未提交的更改
no-staged-changes = 没有已暂存的更改
no-staged-changes-yet = 尚无已暂存的更改
no-unstaged-changes = 没有未暂存的更改
no-changes = 没有更改
remote-up-to-date = 远程已是最新
go-to-next-hunk = 跳转到下一个变更块
go-to-previous-hunk = 跳转到上一个变更块
show-changes-only = 仅显示更改
show-full-file = 显示完整文件
diff-1-file = 差异（1 个文件）
diff-count-files = 差异（{ $count } 个文件）
clipboard-title = 剪贴板 ↔ { $title }
review-diff = 评审差异
send-this-diff-for-your-last-agent-to-review = 将此差异发送给上一个智能体进行评审。
send-review-to-agent-count = 向智能体发送评审（{ $count }）
send-all-review-comments-to-the-agent-panel = 将所有评审意见发送到智能体面板
open-diff = 打开差异
open-file-diff = 打开文件差异

## Conflict resolution

use-branch = 采用 { $branch }
use-both = 两者都采用
resolve-with-agent = 用智能体解决
click-to-resolve-with-agent = 点击以用智能体解决
resolve-merge-conflict-with-agent = 用智能体解决合并冲突
resolve-merge-conflicts-with-agent = 用智能体解决合并冲突
found-1-conflict-across-the-codebase = 在代码库中发现 1 处冲突
found-count-conflicts-across-the-codebase = 在代码库中发现 { $count } 处冲突

## Repository state, initialization and cloning

no-active-repository = 没有活动的仓库
no-git-repositories = 没有 Git 仓库
initialize-repository = 初始化仓库
unable-to-initialize-a-git-repository = 无法初始化 Git 仓库
open-a-directory-first = 请先打开一个目录
where-would-you-like-to-initialize-this-git-repository = 你想在哪里初始化这个 Git 仓库？
select-a-repository = 选择一个仓库…
switch-active-repository = 切换活动仓库
trust-directory = 信任此目录
detected-dubious-ownership-in-repository-at-path-this-happens-when-the-git-directory-is-not-owned-by-the-current-user-if-you-want-to-learn-more-about-safe-directories-visit-git-s-documentation = 检测到仓库 { $path } 的归属存疑。这通常是因为 .git/ 目录不属于当前用户。若想进一步了解安全目录，请查阅 git 文档。
add-to-gitignore = 添加到 .gitignore
add-to-git-info-exclude = 添加到 .git/info/exclude
select-as-repository-destination = 选择为仓库存放位置
git-clone-name = Git 克隆：{ $name }
add-repo-to-project = 将仓库添加到项目
open-repo-in-new-project = 在新项目中打开仓库
enter-repository-url = 输入仓库 URL…
clone-a-repository-from-github-or-other-sources = 从 GitHub 或其他来源克隆仓库。
enter-git-ref = 输入 git 引用…
you-may-need-to-configure-git-for-github = 你可能需要为 Github 配置 git。

## Worktrees

automate-setup = 自动化设置
automate-worktree-setup = 自动化工作树设置
delete-worktree = 删除工作树
force-delete-worktree = 强制删除工作树
remove-worktree-from-window = 从窗口中移除工作树
select-or-type-to-create-a-worktree = 选择工作树，或输入名称以创建…
create-new-worktree-based-on-branch = 基于 { $branch } 创建新工作树
a-worktree-with-this-name-already-exists = 已存在同名的工作树
cannot-create-a-named-worktree-in-a-project-with-multiple-repositories = 无法在包含多个仓库的项目中创建命名工作树
worktree-creation-is-not-supported-in-collaborative-projects = 协作项目不支持创建工作树
requires-a-git-repository-in-the-project = 需要项目中存在 Git 仓库
worktree-name-contains-modified-or-untracked-files-force-delete-it = 工作树「{ $name }」包含已修改或未跟踪的文件，是否强制删除？
some-project-folders-are-not-git-repositories-they-were-included-as-is-without-creating-a-worktree = 部分项目文件夹不是 Git 仓库，它们已按原样加入，未为其创建工作树。
