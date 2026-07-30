# Simplified Chinese (zh-CN) catalog for edit_prediction_ui (edit prediction status
# bar entry, onboarding, and provider settings).
#
# Product/provider names (Zed, GitHub, Copilot, Codestral, Ollama, Mercury, Zeta) are
# deliberately left untranslated. `actions`, `configure-providers`, `edit-predictions`,
# `learn-more`, `privacy`, `sign-out`, `view-docs` are reused from other catalogs.

## Status bar tooltips

edit-prediction = 编辑预测
disabled-for-this-file = 此文件已禁用
enable-to-use = 启用以使用
sign-in-or-configure-a-provider = 登录或配置提供方
choose-a-plan = 选择套餐
configure-a-provider = 配置提供方
powered-by-codestral = 由 Codestral 提供支持
missing-api-key-for-codestral = 缺少 Codestral 的 API 密钥
powered-by-ollama-model = 由 Ollama（{ $model }）提供支持
ollama-model-not-configured-configure-a-model-before-use = 尚未配置 Ollama 模型 — 使用前请先配置模型
powered-by-mercury = 由 Mercury 提供支持
missing-api-key-for-mercury = 缺少 Mercury 的 API 密钥
mercury-free-tier-limit-reached = 已达到 Mercury 免费层级限制
powered-by-zeta = 由 Zeta 提供支持
github-copilot-edit-predictions = GitHub Copilot 编辑预测
copilot-edit-predictions-can-t-be-started-error = Copilot 编辑预测无法启动：{ $error }

## Provider switching & language settings menu

providers = 提供方
edit-predictions-are-disabled-for-this-organization = 此组织已禁用编辑预测。
show-edit-predictions-for = 显示编辑预测的范围
this-buffer = 此缓冲区
edit-predictions-are-disabled-for-language = { $language } 已禁用编辑预测
all-files = 所有文件
display-modes = 显示模式
eager = 积极
display-predictions-inline-when-there-are-no-language-server-completions-available = 当没有可用的语言服务器补全时内联显示预测。
subtle = 低调
display-predictions-inline-only-when-holding-a-modifier-key-alt-by-default = 仅在按住修饰键（默认为 Alt）时内联显示预测。

## Privacy & data collection

training-data-collection = 训练数据采集
project-identified-as-open-source-and-you-re-sharing-data = 项目已被识别为开源项目，你正在分享数据。
project-identified-as-open-source-but-you-re-not-sharing-data = 项目已被识别为开源项目，但你未分享数据。
project-not-identified-as-open-source-no-data-captured = 项目未被识别为开源项目。未采集任何数据。
project-not-identified-as-open-source-and-setting-turned-off = 项目未被识别为开源项目，且该设置已关闭。
help-us-improve-our-open-dataset-model-by-sharing-data-from-open-source-repositories-zed-must-detect-a-license-file-in-your-repo-for-this-setting-to-take-effect-files-with-sensitive-data-and-secrets-are-excluded-by-default = 通过分享开源仓库中的数据，帮助我们改进开放数据集模型。此设置生效前，Zed 必须在你的仓库中检测到许可证文件。默认排除包含敏感数据和密钥的文件。
no-data-captured = 未采集任何数据。
configure-excluded-files = 配置排除文件
open-your-settings-to-add-sensitive-paths-for-which-zed-will-never-predict-edits = 打开设置以添加 Zed 永远不会预测编辑的敏感路径。
view-docs-menu-entry = 查看文档
this-file-is-excluded = 此文件已被排除。

## Actions & Copilot switching

predict-edit-at-cursor = 在光标处预测编辑
rate-predictions = 评价预测
copilot-next-edit-suggestions = Copilot：下一步编辑建议
go-to-copilot-settings = 前往 Copilot 设置
sign-in-to-copilot-edit-predictions = 登录 Copilot 编辑预测
disable-copilot-edit-predictions = 禁用 Copilot 编辑预测
reinstall-copilot-edit-predictions = 重新安装 Copilot 编辑预测

## Sign-in upsell & usage

sign-in-start-using = 登录并开始使用
you-get-2-000-accepted-suggestions-at-every-keystroke-for-free-powered-by-zeta-our-open-source-open-data-model = 你每次按键都可免费获得 2,000 次被采纳的建议，由我们的开源开放数据模型 Zeta 提供支持
free-tier-limit-reached = 已达到免费层级限制
upgrade-to-a-paid-plan-to-continue-using-the-service = 升级到付费套餐以继续使用该服务
usage = 用量
subscribe-to-increase-your-limit = 订阅以提高你的限额
your-github-account-is-less-than-30-days-old = 你的 GitHub 账户注册不满 30 天。
upgrade-to-zed-pro-or-contact-us = 升级到 Zed Pro 或联系我们。
you-have-an-outstanding-invoice = 你有一笔未结清的账单
check-your-payment-status-or-contact-us-at-billing-support-zed-dev-to-continue-using-this-feature = 请检查你的付款状态，或通过 billing-support@zed.dev 联系我们以继续使用此功能。
