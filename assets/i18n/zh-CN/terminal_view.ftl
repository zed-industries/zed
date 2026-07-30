# Simplified Chinese (zh-CN) catalog for the terminal tab bar and panel.
#
# Product names (Zed) and file names (settings.json) are deliberately left
# untranslated. Many keys used by this crate (new-terminal, copy, paste,
# select-all, clear, split-*, zoom-in/out, open-settings, spawn-task, new-menu,
# add-to-agent-thread) are already defined in other catalogs and reused here.
# The tab bar's '+' tooltip shares workspace.ftl's new-menu ('New…'), not
# zed.ftl's new ('New') — the ellipsis is part of the string.

## Terminal context menu

paste-text = 粘贴为文本
inline-assist = 内联助手
close-terminal-tab = 关闭终端标签页
rerun-task = 重新运行任务
process-id-pid-pid = 进程 ID（PID）：{ $pid }

## Terminal panel

edit-settings-json = 编辑 settings.json
failed-to-spawn-terminal = 终端启动失败
edit-settings = 编辑设置
