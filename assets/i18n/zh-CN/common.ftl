# Simplified Chinese (zh-CN) catalog for vocabulary shared across crates.
#
# Keys are global to a locale, so a word that many crates mark resolves against
# one definition. Those live here rather than in whichever crate's catalog
# happened to be written first.
#
# A key that has a natural home keeps it: the application menus own theirs in
# zed.ftl, the title bar owns its call-quality terms in title_bar.ftl. This file
# is for the plain interface vocabulary that belongs to no one crate.
#
# A key earns a place here when several crates mark it and none of them owns it.
# That leaves the application-menu items (Copy, Paste, Cut, Save, Settings, Zoom
# In, Zoom Out) in zed.ftl, the named workspace actions (Copy Path, Copy
# Relative Path, Open in Terminal) in workspace.ftl, and message templates like
# `error-error` wherever they are raised.

## Confirmation and dismissal

ok = 确定
cancel = 取消
retry = 重试
yes = 是
no = 否
close = 关闭
dismiss = 忽略

## Actions with no single owner

# `open` is the File menu's "Open…", whose ellipsis promises a file dialog. A
# button that opens something already chosen names this key instead.
open-action = 打开
learn-more = 了解更多
configure = 配置
delete = 删除
remove = 移除
copy-link = 复制链接

## Account

sign-in = 登录
sign-out = 退出登录

## Zed concepts named across crates

restricted-mode = 受限模式

## Tab titles

# The tab title every preview pane builds from the file it previews: markdown,
# SVG and CSV all name this key.
preview-name = 预览 { $name }

## Tools an agent can be granted

# The tool that retrieves a URL, named by both the agent's @fetch command and the
# tool permission settings. Distinct from `fetch` in git_ui.ftl, which is git
# fetch: Chinese keeps 抓取 for pulling down a web page and 获取 for the git verb.
fetch-tool = 抓取
