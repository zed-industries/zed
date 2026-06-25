Assessment: how default agent + model are selected today

There are **two independent decisions** when the agent panel opens:

1. **Which agent backend is "selected"** (`AgentPanel::selected_agent`) — Zed's native agent vs. an external/ACP agent like Codex, Claude ACP, Copilot CLI, Cursor.
2. **Which model the native Zed agent uses** (`LanguageModelRegistry::default_model()`) — only relevant when the selected agent is the native Zed agent. External agents pick their own models internally and ignore this entirely.

### What onboarding actually writes

In [`crates/onboarding/src/basics_page.rs`](worktree://8076267e-ea69-4f86-ad5b-7dc61c9cf305/crates/onboarding/src/basics_page.rs), the "Agent Setup" section has two kinds of buttons:

- **Zed Agent button** (`render_zed_agent_button`): only triggers sign-in / start-trial. It does **not** write `settings.agent` or any `selected_agent`.
- **Registry agent buttons** (`render_registry_agent_button` for Codex / Claude / Copilot / Cursor): writes a `CustomAgentServerSettings::Registry` entry into `settings.agent_servers`. It does **not** touch `selected_agent` or `default_model`.

So onboarding installs/configures providers, but **never records a preference** for which agent or model should be the panel default.

### How the panel picks defaults on open

- **`selected_agent`** is restored from the serialized panel state / global last-used agent, and falls back to [`Agent::default()` = `NativeAgent`](worktree://8076267e-ea69-4f86-ad5b-7dc61c9cf305/crates/agent_ui/src/agent_ui.rs) for a brand-new user. There is **no logic that checks which agents were installed during onboarding.**
- **Native model** comes from `LanguageModelRegistry::default_model()`, which is `settings.agent.default_model` if set, otherwise `available_fallback_model` (the cloud provider's default if signed in, otherwise the first authenticated provider). Onboarding never sets `default_model`.

Comprehensive table — fresh user, no prior panel state

| Onboarding action | What gets written | `selected_agent` on panel open | Model shown (native only) | Reflects their choice? |
|---|---|---|---|---|
| Zed Agent only (signed in) | trial/sign-in state | **Zed Agent** (`Agent::default`) | Cloud provider default | ✅ Yes |
| Zed Agent + API key (e.g. Anthropic) | sign-in; API key in settings | **Zed Agent** | Cloud default (not necessarily the API-key model) | ⚠️ Agent right, model maybe not |
| **Claude (`claude-acp`)** only | `agent_servers["claude-acp"]` | **Zed Agent** | fallback / none if no LLM auth | ❌ Lands on Zed agent, not Claude |
| **Codex (`codex-acp`)** only | `agent_servers["codex-acp"]` | **Zed Agent** | fallback / none | ❌ Lands on Zed agent, not Codex |
| **GitHub Copilot CLI (`github-copilot-cli`)** only | `agent_servers["github-copilot-cli"]` | **Zed Agent** | fallback / none | ❌ Lands on Zed agent, not Copilot CLI |
| **Cursor (`cursor`)** only | `agent_servers["cursor"]` | **Zed Agent** | fallback / none | ❌ Lands on Zed agent, not Cursor |
| Any combination of the above registry agents | multiple `agent_servers` entries | **Zed Agent** | fallback / none | ❌ Never picks an installed external agent |
| Nothing configured | nothing | **Zed Agent** | none / fallback | ➖ Expected empty state |

### Key takeaways for the personalization work

- The panel **always defaults to the native Zed Agent**, regardless of which agents the user actually set up in onboarding. A user who only installed Codex still lands on the Zed agent.
- For the native agent, the **default model never reflects an onboarding choice** — there's no model-selection step in onboarding at all, and `default_model` is left unset, so the registry's generic fallback applies.
- There's a clean insertion point for personalization: onboarding could write either `selected_agent` (global last-used agent) and/or `settings.agent.default_model` based on what the user configured, since both feed directly into the panel-open logic described above.

Also, a comment from our lead eng here: "We will likely have a data race between registry download + provider auth checks.
But this initialization phase has been kind of racy for awhile, and I think we should do a refactor while we work on this to basically have a way to know if we have all of the data we need in order to make a decision like this"

Design assessment

### 1. The decision rule you proposed, stated precisely

Here's the rule I'd encode, with the model-selection logic spelled out:

**Inputs needed to decide:**
- `zed_configured` = cloud provider authenticated **OR** a non-cloud LLM provider authenticated (i.e. `LanguageModelRegistry::has_authenticated_provider`).
- `installed_registry_agents` = which of the onboarding registry agents have an entry in `settings.agent_servers`.

**Decision:**

| Condition | `selected_agent` | Model |
|---|---|---|
| Zed configured (regardless of registry agents) | `NativeAgent` | the Zed default model (cloud `default_model()`, or the authenticated provider's `default_model(cx)`) |
| Not Zed configured, exactly 1 registry agent installed | that agent (`Agent::Custom`) | **don't pick a model** — let the agent choose (see below) |
| Not Zed configured, >1 registry agent installed | **prompt the user** (see §3) | n/a until they pick |
| Nothing configured | `NativeAgent` (today's behavior) | fallback / empty state |

**Why "let the registry agent choose its model" rather than us picking one:** registry agents (Codex, Copilot CLI, Cursor, Claude ACP) are external ACP processes. Unlike native providers, they expose **no cheap "default model" or "is authenticated" API** — `CustomAgentServer` only has `connect()`, and model selection happens *inside* the ACP session after launch (`list_available_agents` returns `models: Vec::new()` for them precisely because we can't enumerate ahead of time). So "default to that provider's default model" can't mean *we* write `settings.agent.default_model` — that setting only governs the native agent. It means: **select that agent as the panel's active agent and let its own session report its default model.** Trying to second-guess it from Zed would be wrong and fragile.

So your phrase "default to that provider's default model" maps in practice to: *set `selected_agent` to that agent; the model falls out of the agent's own session.*

### 2. The data-race concern — the lead eng is right, and here's the shape of it

There are **three independent async streams** that all feed the decision, none of which is currently synchronized:

1. **Registry download** — `AgentRegistryStore::refresh()` is a network fetch with cache fallback; `is_fetching` flips async. Needed to resolve agent *metadata/icons*, though installed-state itself comes from settings.
2. **Provider auth restoration** — each LLM provider restores credentials independently and emits `ProviderStateChanged`/`ProvidersChanged` whenever its auth resolves. The cloud provider additionally waits on client sign-in status (`maintain_client_status`). So `has_authenticated_provider(cx)` can be `false` at panel-load and flip to `true` a moment later.
3. **Panel load** — `AgentPanel::load` reads serialized state + global last-used agent synchronously at construction and picks `selected_agent` *once*.

Today these are loosely coupled by `cx.subscribe(LanguageModelRegistry)` → `update_active_language_model_from_settings`, which just re-syncs the default model when providers change. But for the *new* onboarding decision, reading `has_authenticated_provider` at panel-construction time is exactly the race: you can decide "Zed not configured → default to Codex" and then the cloud auth resolves 50ms later, contradicting the choice.

**The refactor the lead eng is asking for** — "a way to know if we have all the data we need to make a decision" — is the right call. I'd implement it as an explicit **readiness gate**:

- A small piece of state (e.g. on `LanguageModelRegistry` or a new coordinator) that answers: *"Has initial credential restoration for all registered providers completed?"* Today providers restore independently with no aggregate "all settled" signal — that's the missing primitive.
- Concretely: track a count of providers whose initial `authenticate`/credential-restore task has completed, and emit an `InitialAuthenticationComplete` (or expose `is_initialized() -> bool`) once all have settled. The registry already owns the provider set and already emits per-provider `ProviderStateChanged`, so it's the natural home.
- Similarly expose `AgentRegistryStore` readiness (it already has `is_fetching` + a cache; "ready" = cache loaded or fetch settled).

Then the onboarding-default decision becomes: **don't make the decision until both gates are ready.** Either (a) defer it — render a neutral loading state in the panel until `registry.is_initialized()` and `agent_registry.is_ready()`, then decide once; or (b) make the decision provisionally and *re-run it* on the readiness event, but only while the user hasn't interacted (which is more error-prone — I'd prefer deferring).

### 3. Choosing among >1 registry agents

Don't auto-pick. Two viable approaches, in order of preference:

- **Preferred — a lightweight first-open prompt in the panel.** When the readiness gate settles and we find multiple installed registry agents and no Zed config, render a small "Which agent do you want to start with?" picker (the panel already has an agent-switcher UI for `selected_agent`, so this reuses existing affordances). Record the choice as the global last-used agent so it sticks.
- **Alternative — decide it during onboarding instead of at panel-open.** Add explicit selection state to the onboarding step: the last registry agent the user clicked (or a "make default" affordance) writes the global last-used-agent value that `AgentPanel::load` already reads. This sidesteps the >1 ambiguity entirely because the user expressed intent at install time, and it sidesteps the *auth* race for the registry-agent case (registry agents have no auth precondition we check). The Zed-vs-registry race still needs the gate from §2.

I lean toward **doing the bulk of the decision at onboarding time** (write the global last-used agent based on what the user configured), with the readiness gate as the safety net for the "is Zed actually configured?" branch — because onboarding is where the user's intent is freshest, and it avoids a cold-start prompt in the panel for most users.

### Suggested sequencing

1. **Refactor first (unblocks everything):** add the provider-auth readiness signal to `LanguageModelRegistry` and an analogous readiness check on `AgentRegistryStore`. This is independently valuable and de-races the existing init.
2. **Record intent at onboarding:** have the Zed-agent and registry-agent buttons write a "preferred default agent" (the global last-used agent the panel already consumes). For registry agents, last-clicked wins; surface a clear default indicator if >1.
3. **Panel-open decision behind the gate:** in `AgentPanel::load`, when there's no serialized panel state, wait for both readiness gates, then apply the §1 rule (Zed config beats registry preference; otherwise honor the recorded registry preference; otherwise prompt if still ambiguous).
4. **Model:** only ever write `settings.agent.default_model` for the native path; for registry agents, let the session report its own model.

Want me to dig into the `LanguageModelRegistry` provider-init code to scope exactly what the readiness primitive in step 1 would touch?
