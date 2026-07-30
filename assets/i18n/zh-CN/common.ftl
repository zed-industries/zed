# Simplified Chinese (zh-CN) catalog for vocabulary shared across crates.
#
# Keys are global to a locale, so a word that many crates mark resolves against
# one definition. Those live here rather than in whichever crate's catalog
# happened to be written first.
#
# A key that has a natural home keeps it: the application menus own theirs in
# zed.ftl, the title bar owns its call-quality terms in title_bar.ftl. This file
# is for the plain interface vocabulary that belongs to no one crate.

## Confirmation and dismissal

ok = 确定
cancel = 取消
retry = 重试

## Actions with no single owner

# `open` is the File menu's "Open…", whose ellipsis promises a file dialog. A
# button that opens something already chosen names this key instead.
open-action = 打开
learn-more = 了解更多
