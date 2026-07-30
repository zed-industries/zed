# Simplified Chinese (zh-CN) catalog for repl (Jupyter/REPL integration).
#
# Product names (Zed, Jupyter, ipykernel), kernel/environment type names (Conda,
# Pyenv, venv, WSL, SSH…), language identifiers, Jupyter protocol field names, and
# the `repl::Run` action reference are deliberately left untranslated.

## ipykernel install toasts

installing-ipykernel-in-env = 正在为 { $env } 安装 ipykernel…
ipykernel-installed-in-env = 已在 { $env } 中安装 ipykernel
failed-to-install-ipykernel-in-env-error = 在 { $env } 中安装 ipykernel 失败：{ $error }

## REPL sessions page

repl-sessions = REPL 会话
to-start-interactively-running-code-in-your-editor-you-need-to-install-and-configure-jupyter-kernels = 要在编辑器中交互式运行代码，需要先安装并配置 Jupyter 内核。
no-jupyter-kernels-available = 没有可用的 Jupyter 内核
install-kernels = 安装内核
to-run-code-in-a-jupyter-kernel-select-some-code-and-use-the-repl-run-command = 要在 Jupyter 内核中运行代码，请选中一段代码并使用 'repl::Run' 命令。
no-jupyter-kernel-sessions = 没有 Jupyter 内核会话
jupyter-kernel-sessions = Jupyter 内核会话

## Kernel picker

# `recommended` reused from agent_ui.ftl.
python-environments = Python 环境
jupyter-kernels = Jupyter 内核
wsl-kernels = WSL 内核
remote-servers = 远程服务器
select-a-kernel = 选择内核…
ipykernel-not-installed = 未安装 ipykernel
kernel-docs = 内核文档

## Kernel/session status

interrupt = 中断
# Its own key because the English is "Starting", while the language server's
# `starting` spells it "Starting…" — the ellipsis makes them separate strings that
# would otherwise derive the same key. The badge drops the ellipsis but keeps the
# 正在… wording that 正在关闭 and 正在重启 use in this same status group.
kernel-status-starting = 正在启动
shutting-down = 正在关闭
shutdown = 已关闭
restarting = 正在重启
# `error` reused from language_tools.ftl; `error-error` (Error: { $error }) reused from workspace.ftl.
close-output-area = 关闭输出区域
idle = 空闲
busy = 忙碌

## Notebook toolbar

execute-all-cells = 运行所有单元格
clear-all-outputs = 清除所有输出
move-cell-up = 上移单元格
move-cell-down = 下移单元格
add-markdown-block = 添加 Markdown 块
add-code-block = 添加代码块
delete-cell = 删除单元格
more-options = 更多选项
select-kernel = 选择内核
kernel-name-status-click-to-change-kernel = { $kernel_name }（{ $status }）。点击以更改内核。
kernel-kernel-name-status-click-to-change = 内核：{ $kernel_name }（{ $status }）。点击以更改。
restart-kernel = 重启内核
interrupt-kernel = 中断内核

## Notebook empty state

this-notebook-is-empty = 此笔记本为空。
add-code-cell = 添加代码单元格
add-markdown-cell = 添加 Markdown 单元格

## Output controls and messages

copy-output = 复制输出
open-in-buffer = 在缓冲区中打开
copy-full-error = 复制完整错误
open-full-error-in-buffer = 在缓冲区中打开完整错误
failed-to-parse-json = 解析 JSON 失败
failed-to-load-image-error = 加载图像失败：{ $error }
unsupported-media-type = 不支持的媒体类型
type-here-and-press-enter = 在此输入并按下 Enter
# `input` (Input:) reused from agent_ui.ftl.

## Execution status

connecting-to-kernel = 正在连接到内核…
executing = 正在执行…
unknown-status = 未知状态
kernel-shutting-down = 内核正在关闭…
kernel-shutdown = 内核已关闭
kernel-restarting = 内核正在重启…
queued = 排队中…
kernel-error-error = 内核错误：{ $error }
