# Simplified Chinese (zh-CN) catalog for ui (the shared component library).
#
# Product names (Zed, Jupyter, ACP) and file names are deliberately left untranslated.
# Most strings in this crate are documentation-comment examples or component-gallery
# preview fixtures for developers and are intentionally not localized.

## Project empty state

choose-one-of-the-options-below-to-use-the-label = 从下方选项中选择一项以使用 { $label }
or = 或

## Alert modal defaults

# `ok` / `cancel` reused from common.ftl.

## Copy button

copied = 已复制
# `copy` reused from zed.ftl.

## Disclosure toggle

collapse = 折叠
expand = 展开

## Update button (title bar auto-update status)

checking-for-zed-updates = 正在检查 Zed 更新…
downloading-zed-update = 正在下载 Zed 更新…
installing-zed-update = 正在安装 Zed 更新…
restart-to-update = 重启以更新
failed-to-update = 更新失败
update-to-version-version = 更新到版本：{ $version }
update-to-version-version-percent-downloaded = 更新到版本：{ $version }（已下载 { $percent }%）
# `dismiss` reused from workspace.ftl.

## Announcement toast defaults

try-now = 立即试用
# `learn-more` reused from common.ftl.

## Configured API card

# `reset-key` reused from settings_ui.ftl.

## AI setting item status tooltips

server-is-stopped = 服务器已停止。
server-is-starting = 服务器正在启动。
server-is-active = 服务器正在运行。
server-has-an-error = 服务器出现错误。
authentication-required = 需要认证。
client-secret-required = 需要客户端密钥。
waiting-for-authorization = 等待授权…
label-was-installed-from-an-extension = { $label } 是通过扩展安装的。
label-was-installed-from-the-acp-registry = { $label } 是通过 ACP 注册表安装的。
label-was-configured-manually = { $label } 是手动配置的。

## Thread item status tooltip (agent panel)

thread-has-an-error = 会话出现错误
waiting-for-confirmation = 等待确认

## Reveal in file manager

# `ui::utils::reveal_in_file_manager_label` picks one of these by platform, and
# the project panel, the editor, the pane tab bar and the outline panel all show
# what it returns. Finder and File Explorer are the platforms' own product names.
reveal-in-finder = 在 Finder 中显示
reveal-in-file-explorer = 在 File Explorer 中显示
reveal-in-file-manager = 在文件管理器中显示
