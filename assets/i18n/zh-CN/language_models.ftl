# Simplified Chinese (zh-CN) catalog for the LLM provider configuration UI.
#
# Provider, product and vendor names (Zed, Anthropic, OpenAI, ChatGPT, Google,
# Ollama, LM Studio, llama.cpp, LlamaCpp, OpenCode, Amazon Bedrock, AWS, IAM,
# GitHub Copilot, Llama, Mistral, Gemma, Phi, Qwen, GGUF, WebUI…), plan and
# subscription tier names (Pro, Student, Business, VIP, Zen, Go, Free, Priority
# Tier), model ids, environment variable names, API field values, file names
# (settings.json) and URLs are deliberately left untranslated.

## API key / credential fields

api-key-input-label = API 密钥
context-window = 上下文窗口
context-window-tokens = 上下文窗口：{ $tokens }
access-key-id = 访问密钥 ID
secret-access-key = 私有访问密钥
session-token-optional = 会话 Token（可选）
bedrock-api-key = Bedrock API 密钥
static-credentials = 静态凭据
using-the-api-key = 使用 API 密钥

## Credential status

loading-credentials = 正在加载凭据…
api-key-configured-status = API 密钥已配置
api-key-configured-for-api-url = 已为 { $api_url } 配置 API 密钥
api-key-set-in-env-var-name-environment-variable = API 密钥已通过环境变量 { $env_var_name } 设置
you-can-also-set-the-env-var-name-environment-variable-and-restart-zed = 你也可以设置环境变量 { $env_var_name }，然后重启 Zed。
to-reset-your-api-key-unset-the-env-var-name-environment-variable = 要重置 API 密钥，请取消设置环境变量 { $env_var_name }。
reset-api-key = 重置 API 密钥
reset-api-url = 重置 API URL
remove-provider = 移除提供方
connected = 已连接
refresh-models = 刷新模型
not-authenticated = 未认证
signed-in = 已登录
signed-in-as-email = 已登录为 { $email }

## OpenAI-/Anthropic-compatible providers

to-use-zed-s-agent-with-an-provider-name-compatible-provider-you-need-to-add-an-api-key = 要在 Zed 智能体中使用兼容 { $provider_name } 的提供方，你需要添加 API 密钥。

## Ollama

run-local-models-on-your-machine-with-ollama = 使用 Ollama 在本机运行本地模型。
run-llms-locally-on-your-machine-with-ollama-or-connect-to-an-ollama-server-can-provide-access-to-llama-mistral-gemma-and-hundreds-of-other-models = 使用 Ollama 在本机运行 LLM，或连接到 Ollama 服务器。可访问 Llama、Mistral、Gemma 等数百个模型。
to-use-local-ollama = 要使用本地 Ollama：
download-and-install-ollama-from = 下载并安装 Ollama：
start-ollama-and-download-a-model = 启动 Ollama 并下载一个模型：
click-connect-below-to-start-using-ollama-in-zed = 点击下方的「连接」，开始在 Zed 中使用 Ollama
alternatively-you-can-connect-to-an-ollama-server-by-specifying-its-url-and-api-key-may-not-be-required = 你也可以通过指定 URL 与 API 密钥（可能不需要）连接到 Ollama 服务器：
default-model-specific = 默认：由模型决定
download-ollama = 下载 Ollama
view-all-models = 查看全部模型

## LM Studio

run-local-llms-like-llama-phi-and-qwen-with-lm-studio = 使用 LM Studio 运行 Llama、Phi、Qwen 等本地 LLM。
run-local-llms-like-llama-phi-and-qwen = 运行 Llama、Phi、Qwen 等本地 LLM。
lm-studio-needs-to-be-running-with-at-least-one-model-downloaded = LM Studio 需要处于运行状态，并且至少已下载一个模型。
to-get-your-first-model-try-running = 要获取第一个模型，可以试试运行
alternatively-you-can-connect-to-an-lm-studio-server-by-specifying-its-url-and-api-key-may-not-be-required = 你也可以通过指定 URL 与 API 密钥（可能不需要）连接到 LM Studio 服务器：
download-lm-studio = 下载 LM Studio
model-catalog = 模型目录

## llama.cpp

run-local-models-on-your-machine-with-llamacpp = 使用 LlamaCpp 在本机运行本地模型。
run-open-models-locally-with-llama-cpp-s-built-in-server-or-connect-to-a-remote-llama-cpp-server = 使用 llama.cpp 内置服务器在本地运行开放模型，或连接到远程 llama.cpp 服务器。
to-use-a-local-llama-cpp-server = 要使用本地 llama.cpp 服务器：
install-llama-cpp-from = 安装 llama.cpp：
start-the-server-in-router-mode = 以路由模式启动服务器：
click-connect-below-to-start-using-llama-cpp-in-zed = 点击下方的「连接」，开始在 Zed 中使用 llama.cpp
alternatively-you-can-connect-to-a-remote-llama-cpp-server-by-specifying-its-url-and-api-key-set-with-api-key-may-not-be-required = 你也可以通过指定 URL 与 API 密钥（用 --api-key 设置，可能不需要）连接到远程 llama.cpp 服务器：
default-discovered-from-the-server = 默认：从服务器自动获取
open-webui = 打开 WebUI
get-llama-cpp = 获取 llama.cpp
browse-gguf-models = 浏览 GGUF 模型

## OpenCode

to-use-opencode-models-in-zed-you-need-an-api-key = 要在 Zed 中使用 OpenCode 模型，你需要一个 API 密钥。
to-use-opencode-models-in-zed-you-need-an-api-key-colon = 要在 Zed 中使用 OpenCode 模型，你需要一个 API 密钥：
sign-in-and-get-your-key-at = 登录并获取密钥：
paste-your-api-key-below-and-hit-enter-to-start-using-opencode = 在下方粘贴 API 密钥并按回车，即可开始使用 OpenCode
subscriptions = 订阅
show-zen-models = 显示 Zen 模型
show-go-models = 显示 Go 模型
show-free-models = 显示 Free 模型
no-subscriptions-enabled-enable-at-least-one-subscription-to-use-opencode = 未启用任何订阅。至少启用一项订阅才能使用 OpenCode。

## Amazon Bedrock

to-use-zed-s-agent-with-bedrock-set-a-custom-authentication-strategy-in-your-settings-or-use-static-credentials-mantle-only-models-e-g-gpt-5-5-gpt-5-4-grok-4-3-additionally-require-iam-permissions-for-the-bedrock-mantle-endpoint = 要在 Zed 智能体中使用 Bedrock，请在设置中指定自定义认证策略，或使用静态凭据。仅 Mantle 提供的模型（如 GPT-5.5、GPT-5.4、Grok 4.3）还需要 `bedrock-mantle` 端点的 IAM 权限。
to-use-zed-s-agent-with-bedrock-you-can-set-a-custom-authentication-strategy-through-your-settings-file-or-use-static-credentials = 要在 Zed 智能体中使用 Bedrock，你可以通过设置文件指定自定义认证策略，或使用静态凭据。
but-first-to-access-models-on-aws-you-need-to = 但首先，要访问 AWS 上的模型，你需要：
grant-permissions-to-the-strategy-you-ll-use-according-to-the = 按以下文档为你要使用的策略授予权限：
select-the-models-you-would-like-access-to = 选择你想要访问的模型：
for-access-keys-create-an-iam-user-in-the-aws-console-with-programmatic-access = 使用访问密钥：在 AWS 控制台中创建一个具有编程访问权限的 IAM 用户：
for-bedrock-api-keys-generate-an-api-key-from-the = 使用 Bedrock API 密钥：从以下位置生成 API 密钥：
attach-the-necessary-bedrock-permissions-to = 将必要的 Bedrock 权限附加给
this-user = 该用户
enter-either-access-keys-or-a-bedrock-api-key-below-not-both = 在下方填写访问密钥或 Bedrock API 密钥（二者只填其一）
this-method-uses-your-aws-access-key-id-and-secret-access-key-or-a-bedrock-api-key = 此方式使用你的 AWS 访问密钥 ID 与私有访问密钥，或 Bedrock API 密钥。
you-can-also-set-the-access-key-id-var-secret-access-key-var-and-region-var-environment-variables-or-bearer-token-var-for-bedrock-api-key-authentication-and-restart-zed = 你也可以设置环境变量 { $access_key_id_var }、{ $secret_access_key_var } 和 { $region_var }（若使用 Bedrock API 密钥认证则设置 { $bearer_token_var }），然后重启 Zed。
optionally-if-your-environment-uses-aws-cli-profiles-you-can-set-profile-var-if-it-requires-a-custom-endpoint-you-can-set-endpoint-var-and-if-it-requires-a-session-token-you-can-set-session-token-var = 可选：如果你的环境使用 AWS CLI 配置文件，可以设置 { $profile_var }；如果需要自定义端点，可以设置 { $endpoint_var }；如果需要会话 Token，可以设置 { $session_token_var }。
region-is-configured-via-region-var-environment-variable-or-settings-json-defaults-to-us-east-1 = 区域通过环境变量 { $region_var } 或 settings.json 配置（默认为 us-east-1）。
using-automatic-credentials-aws-default-chain = 正在使用自动凭据（AWS 默认凭据链）
using-aws-profile-profile-name = 正在使用 AWS 配置文件：{ $profile_name }
using-aws-sso-profile-profile-name = 正在使用 AWS SSO 配置文件：{ $profile_name }
using-iam-credentials = 正在使用 IAM 凭据
using-iam-credentials-from-access-key-id-var-and-secret-access-key-var-environment-variables = 正在使用来自环境变量 { $access_key_id_var } 与 { $secret_access_key_var } 的 IAM 凭据
using-bedrock-api-key = 正在使用 Bedrock API 密钥
using-bedrock-api-key-from-bearer-token-var-environment-variable = 正在使用来自环境变量 { $bearer_token_var } 的 Bedrock API 密钥
to-reset-your-credentials-unset-the-access-key-id-var-secret-access-key-var-and-session-token-var-or-bearer-token-var-environment-variables = 要重置凭据，请取消设置环境变量 { $access_key_id_var }、{ $secret_access_key_var } 和 { $session_token_var }，或 { $bearer_token_var }。
authentication-method-is-configured-in-settings-edit-settings-json-to-change = 认证方式已在设置中配置。要更改请编辑 settings.json。

## Zed hosted models

sign-in-to-have-access-to-zed-s-complete-agentic-experience-with-hosted-models = 登录即可通过托管模型体验 Zed 完整的智能体功能。
sign-in-to-use-zed-ai = 登录以使用 Zed AI
you-have-access-to-zed-s-hosted-models-through-your-pro-subscription = 你可以通过 Pro 订阅使用 Zed 的托管模型。
you-have-access-to-zed-s-hosted-models-through-your-pro-trial = 你可以通过 Pro 试用使用 Zed 的托管模型。
you-have-access-to-zed-s-hosted-models-through-your-student-subscription = 你可以通过 Student 订阅使用 Zed 的托管模型。
you-have-access-to-zed-s-hosted-models-through-your-vip-subscription = 你可以通过 VIP 订阅使用 Zed 的托管模型。
you-have-access-to-zed-s-hosted-models-through-your-organization = 你可以通过所属组织使用 Zed 的托管模型。
zed-s-hosted-models-are-disabled-by-your-organization-s-configuration = 你所属组织的配置已停用 Zed 的托管模型。
subscribe-for-access-to-zed-s-hosted-models = 订阅即可使用 Zed 的托管模型。
subscribe-for-access-to-zed-s-hosted-models-start-with-a-14-day-free-trial = 订阅即可使用 Zed 的托管模型，可先享受 14 天免费试用。
subscribed-to-pro = 已订阅 Pro
subscribed-to-pro-trial = 已订阅 Pro 试用
subscribed-to-student = 已订阅 Student
subscribed-to-business = 已订阅 Business
subscribed-to-vip = 已订阅 VIP
manage-subscription = 管理订阅
start-14-day-free-pro-trial = 开始 14 天 Pro 免费试用
upgrade-to-pro = 升级到 Pro
failed-to-sign-in-with-your-zed-account-401 = 使用 Zed 账户登录失败（401）。
you-are-not-signed-in-to-your-zed-account-sign-in-to-continue = 你尚未登录 Zed 账户。请登录后继续。

## ChatGPT subscription

configure-chatgpt = 配置 ChatGPT
sign-in-with-your-chatgpt-plus-or-pro-subscription-to-use-openai-models-in-zed-s-agent = 使用你的 ChatGPT Plus 或 Pro 订阅登录，即可在 Zed 智能体中使用 OpenAI 模型。
your-chatgpt-subscription-session-is-invalid-or-has-expired-sign-in-again-via-settings-ai-llm-providers-to-continue = 你的 ChatGPT 订阅会话无效或已过期。请通过「设置 > AI > LLM 提供方」重新登录后继续。
you-are-not-signed-in-to-your-chatgpt-account-sign-in-via-settings-ai-llm-providers-to-continue = 你尚未登录 ChatGPT 账户。请通过「设置 > AI > LLM 提供方」登录后继续。

## GitHub Copilot

configure-copilot-chat = 配置 Copilot Chat
requires-an-active-github-copilot-subscription = 需要有效的 GitHub Copilot 订阅。

## Fast mode confirmation

enable-fast-mode-for-anthropic = 要为 Anthropic 启用快速模式吗？
fast-mode-lets-requests-use-your-anthropic-priority-tier-capacity-which-anthropic-prioritizes-over-standard-requests-during-peak-load-requires-a-priority-tier-commitment-with-anthropic-without-one-requests-behave-the-same-as-the-standard-tier = 快速模式让请求使用你的 Anthropic Priority Tier 容量，Anthropic 会在高峰负载时优先处理这些请求。这需要与 Anthropic 签订 Priority Tier 承诺；若没有，请求的表现与标准层级相同。
enable-fast-mode-for-openai = 要为 OpenAI 启用快速模式吗？
fast-mode-sends-requests-using-openai-s-priority-processing-tier-which-targets-significantly-lower-latency-than-the-standard-tier-and-is-billed-at-a-premium-per-token-rate = 快速模式使用 OpenAI 的 Priority 处理层级发送请求，其延迟显著低于标准层级，但按更高的单位 token 价格计费。
enable-fast-mode-for-zed = 要为 Zed 启用快速模式吗？
fast-mode-routes-requests-through-the-upstream-provider-s-fast-mode-or-priority-tier-the-upstream-provider-s-premium-per-token-pricing-applies-and-is-passed-through-to-your-zed-billing = 快速模式会将请求路由到上游提供方的快速模式或优先层级。上游提供方的高价 token 计费会生效，并计入你的 Zed 账单。
