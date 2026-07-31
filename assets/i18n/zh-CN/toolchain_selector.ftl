# Simplified Chinese (zh-CN) catalog for the toolchain selector.
#
# `{ $term }` is the term a language uses for its toolchains, supplied as an
# English identifier by that language's toolchain lister in `crates/languages`.
# The terms Zed itself ships have their own whole-sentence entries above, so only
# an unrecognized term (a language name, say) reaches `{ $term }` untranslated.
# Toolchain names and paths are likewise untranslated.

# Zed 自带的词条各有整句条目，其余（例如语言名）沿用 `{ $term }` 这条通用句式。
select-toolchain = 选择工具链
select-virtual-environment = 选择虚拟环境
select-term = 选择{ $term }
select-a-toolchain = 选择工具链…
select-a-toolchain-for-path = 为 { $path } 选择工具链…
select-a-virtual-environment-for-path = 为 { $path } 选择虚拟环境…
select-a-term-for-path = 为 { $path } 选择{ $term }…
add-toolchain = 添加工具链
add-virtual-environment = 添加虚拟环境
add-term = 添加{ $term }
worktree-root = 工作树根目录
select-toolchain-path = 选择工具链路径

## 作用域（`scope` 本身复用 settings_ui.ftl 的定义）

toolchain-scope-subproject = 子项目
toolchain-scope-project = 项目
toolchain-scope-global = 全局
available-only-in-the-subproject-you-re-currently-in = 仅在当前所处的子项目中可用。
available-in-all-locations-in-your-current-project = 在当前项目的所有位置可用。
available-in-all-of-your-projects-on-this-machine = 在本机上的所有项目中可用。
