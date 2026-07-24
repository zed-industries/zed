# ADR-003: Native multi-provider authentication

- Status: Accepted
- Date: 2026-07-24

## Context

Vela needs account and API-key authentication for multiple language-model providers without depending on Zed login or running Pi/Node as a child process. Pi demonstrates a useful separation between provider metadata, login interaction, credential persistence, refresh, and request-time auth resolution.

## Decision

Vela will implement provider authentication in-process in Rust and extend the existing `LanguageModelProvider`, provider registry, Settings UI, `CredentialsProvider`, and macOS Keychain integration.

Authentication modes are provider capabilities rather than global assumptions:

- API key, including custom Base URL providers;
- OAuth authorization-code + PKCE;
- OAuth device-code flow;
- ambient cloud credentials such as AWS profiles and Google ADC;
- keyless local providers.

Secrets and OAuth tokens are stored in the system Keychain. TOML stores only non-secret provider metadata and Keychain references. OAuth refresh must be serialized per provider/account to prevent concurrent refresh-token rotation.

The Settings UI provides one provider section with any valid choices, including `Sign In` and `API Key` when both are supported. Login and logout must update provider availability without restarting Vela.

Provider OAuth integrations must use an official/public flow or a Vela-owned registered client. Existing third-party client IDs must not be copied merely because they are visible in another open-source client.

## Initial delivery

1. Reuse existing ChatGPT Codex OAuth and GitHub Copilot account flows.
2. Add OpenRouter browser PKCE sign-in, which mints a revocable API key and persists it through Vela's Keychain-backed `ApiKeyState`.
3. Retain Keychain-backed API-key setup for OpenAI, Anthropic, Gemini, OpenRouter, DeepSeek, Mistral, xAI, and compatible providers.
4. Add a provider-neutral auth state and refresh coordinator before adding expiring OAuth integrations.

## Pi reference and attribution

The provider/auth capability split, login interaction concepts, and OpenRouter PKCE protocol behavior were informed by the MIT-licensed Pi sources pinned in `upstream.lock`, especially:

```text
packages/ai/src/auth/types.ts
packages/ai/src/models.ts
packages/ai/src/auth/oauth/openrouter.ts
packages/coding-agent/docs/providers.md
```

Vela's implementation is independently written in Rust and uses its existing Keychain and GPUI infrastructure. Pi is not linked, embedded, spawned, or required at runtime.

## Consequences

- Provider settings can expose multiple authentication methods cleanly.
- Vela credentials remain isolated from plaintext TOML and Pi auth files.
- Each OAuth provider still requires protocol, terms-of-service, client registration, refresh, cancellation, and test review.
- Subscription authentication does not imply or bypass provider quotas.
