---
title: Use a Gateway - Zed
description: Configure OpenRouter, Vercel AI Gateway, Amazon Bedrock, and other gateway or cloud model platforms in Zed.
---

# Use a Gateway

Use a gateway when you route model requests through a platform such as OpenRouter, Vercel AI Gateway, Amazon Bedrock, or another OpenAI-compatible service.

| Gateway                   | Zed AI features | [External Agents](./external-agents.md) | Terminal Threads | Notes                                        |
| ------------------------- | --------------- | --------------------------------------- | ---------------- | -------------------------------------------- |
| OpenRouter                | Yes             | Separate config                         | Separate config  | Uses OpenRouter API access                   |
| Vercel AI Gateway         | Yes             | Separate config                         | Separate config  | Uses Vercel AI Gateway API access            |
| Amazon Bedrock            | Yes             | Separate config                         | Separate config  | Uses AWS credentials or Bedrock bearer token |
| OpenAI-compatible gateway | Yes             | Separate config                         | Separate config  | Configure base URL, model, and key           |

## OpenRouter {#openrouter}

Use OpenRouter when you want to route Zed AI features through OpenRouter.

1. Visit [OpenRouter](https://openrouter.ai) and create an account.
2. Generate an API key from your [OpenRouter keys page](https://openrouter.ai/keys).
3. Open **Settings → AI → LLM Providers** with {#action agent::OpenSettings} and find the OpenRouter row.
4. Enter your OpenRouter API key.

Zed also reads `OPENROUTER_API_KEY` from the local Zed process environment.

When using OpenRouter as your assistant provider, explicitly select a model in your settings:

```json [settings]
{
  "agent": {
    "default_model": {
      "provider": "openrouter",
      "model": "openrouter/auto"
    }
  }
}
```

The `openrouter/auto` model routes requests to an available model selected by OpenRouter. You can also specify any model available through OpenRouter's API.

### OpenRouter Custom Models {#openrouter-custom-models}

You can add custom models to the OpenRouter provider in settings:

```json [settings]
{
  "language_models": {
    "open_router": {
      "api_url": "https://openrouter.ai/api/v1",
      "available_models": [
        {
          "name": "google/gemini-2.0-flash-thinking-exp",
          "display_name": "Gemini 2.0 Flash (Thinking)",
          "max_tokens": 200000,
          "max_output_tokens": 8192,
          "supports_tools": true,
          "supports_images": true,
          "mode": {
            "type": "thinking",
            "budget_tokens": 8000
          }
        }
      ]
    }
  }
}
```

Custom model entries support fields such as `name`, `display_name`, `max_tokens`, `max_output_tokens`, `max_completion_tokens`, `supports_tools`, `supports_images`, and `mode`.

### OpenRouter Provider Routing {#openrouter-provider-routing}

You can control how OpenRouter routes a custom model request among upstream providers with the `provider` object on each model entry.

Supported fields include `order`, `allow_fallbacks`, `require_parameters`, `data_collection`, `only`, `ignore`, `quantizations`, and `sort`.

```json [settings]
{
  "language_models": {
    "open_router": {
      "available_models": [
        {
          "name": "openrouter/auto",
          "display_name": "Auto Router",
          "max_tokens": 2000000,
          "supports_tools": true,
          "provider": {
            "order": ["anthropic", "openai"],
            "allow_fallbacks": true,
            "require_parameters": true,
            "data_collection": "allow"
          }
        }
      ]
    }
  }
}
```

## Vercel AI Gateway {#vercel-ai-gateway}

Use Vercel AI Gateway when you want to route Zed AI features through Vercel.

1. Create an API key from your Vercel AI Gateway keys page.
2. Open **Settings → AI → LLM Providers** with {#action agent::OpenSettings} and find the Vercel AI Gateway row.
3. Enter your Vercel AI Gateway API key.

Zed also reads `VERCEL_AI_GATEWAY_API_KEY` from the local Zed process environment.

You can set a custom endpoint for Vercel AI Gateway in settings:

```json [settings]
{
  "language_models": {
    "vercel_ai_gateway": {
      "api_url": "https://ai-gateway.vercel.sh/v1"
    }
  }
}
```

## Amazon Bedrock {#amazon-bedrock}

Use Amazon Bedrock when you want model access through AWS.

Open **Settings → AI → LLM Providers** with {#action agent::OpenSettings} and find the Amazon Bedrock row to configure authentication and see the source of your model list.

### Bedrock models {#bedrock-models}

After you authenticate, Zed calls Bedrock's `ListInferenceProfiles` and `ListFoundationModels` in your Region. The picker includes models with active [inference profiles](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles.html), including application inference profiles in your account. Listing a model does not prove that your account has permission to invoke it.

Zed keeps models whose foundation-model listing reports text input, text-only output, and streaming support. It excludes IDs starting with `twelvelabs.pegasus`, which Zed does not serve through Converse. Inactive profiles and profiles whose foundation model is missing from that filtered list are skipped.

Models without an inference profile are not listed. Add them as [custom Bedrock models](#bedrock-custom-models).

The listing APIs used by Zed do not report token limits, tool use, or reasoning controls. Zed reads these from the public [models.dev](https://models.dev) catalog after authentication. It fetches the catalog without your credentials, stores the Bedrock section in a local cache, and refreshes it hourly and when you manually refresh models. A fresh cache can supply the first copy without a network fetch.

A listed model that the catalog does not know gets a 128,000-token context window, no explicit output limit, no temperature field, and no reasoning controls. Zed enables tools for it, except for IDs starting with `meta.llama`, `mistral.pixtral-large`, `writer.palmyra`, or `deepseek.r1`, for which Zed disables streaming tool use. Bedrock's listing supplies image support. You can override capabilities with a [custom Bedrock model](#bedrock-custom-models).

While the first AWS listing is in progress, Zed offers only models from your settings. If listing fails without an earlier account list, including when your credentials lack listing permissions, Zed offers the catalog's profiles for your Region instead. This public list excludes deprecated entries and entries without text output. It contains no application profiles and can include a model or route that your account or Region cannot invoke. Such a request fails with Bedrock's error.

Zed uses the same public list when you set a nonempty [custom endpoint](#bedrock-custom-endpoints): it lists from AWS, not from your gateway, and skips AWS listing in this case. If a refresh fails, Zed keeps the earlier account list for the same configuration. If only application-profile listing fails, Zed shows the successful system-profile list without application profiles. If no usable public copy is loaded or it has no profiles for your Region, only settings models remain. A failed public-catalog refresh also keeps the last good copy. The Amazon Bedrock settings view states which case applies.

Settings changes apply without restarting Zed. Changing the authentication method, profile, resolved Region, or endpoint starts discovery again (or skips it for a gateway). Changing `allow_global` chooses a route from the current list without listing again. Custom models, headers, and guardrails also apply without another listing.

For static credentials, Zed reads these prefixed environment variables from its local process environment:

- `ZED_ACCESS_KEY_ID`
- `ZED_SECRET_ACCESS_KEY`
- `ZED_SESSION_TOKEN`
- `ZED_AWS_REGION`
- `ZED_BEDROCK_BEARER_TOKEN`

Set the profile with `profile` and the gateway with `endpoint_url` in settings. `ZED_AWS_REGION` takes precedence over `region`. Without either, Zed uses `us-east-1`. Restart Zed after changing these environment variables.

### Bedrock authentication {#bedrock-authentication}

You can authenticate with a named profile, static credentials, or a Bedrock API key.

For a named profile, configure Bedrock in settings:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "authentication_method": "named_profile",
      "region": "your-aws-region",
      "profile": "your-profile-name"
    }
  }
}
```

For static credentials, open [Agent Settings](./agent-settings.md) with {#action agent::OpenSettings}, go to the Amazon Bedrock section, and enter the access key ID and secret access key. Add a session token for temporary credentials. Set the Region through `region` in settings or `ZED_AWS_REGION`.

For a Bedrock API key, choose API key authentication:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "authentication_method": "api_key",
      "region": "your-aws-region"
    }
  }
}
```

The API key itself is stored in the system keychain, not in `settings.json`.

### Bedrock cross-Region inference {#bedrock-cross-region-inference}

Zed sends requests through [Cross-Region inference](https://docs.aws.amazon.com/bedrock/latest/userguide/cross-region-inference.html) profiles, using the profiles that Bedrock lists for your Region.

By default, Zed uses a geographic inference profile. When your Region offers more than one for a model, Zed uses the one with the fewest destination Regions. To prefer global profiles, add `allow_global`:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "authentication_method": "named_profile",
      "region": "your-aws-region",
      "profile": "your-profile-name",
      "allow_global": true
    }
  }
}
```

`allow_global` is a preference, not a prohibition. If a listed model has only a global profile, Zed uses it even with `allow_global: false`. If it has only a geographic profile, Zed uses that even with `allow_global: true`. In public-catalog fallback, Zed also uses a global profile when the catalog has no geographic profile matching your Region, even with `allow_global: false`. An application profile keeps its ARN, and a custom entry keeps its exact `name`. Neither follows this preference.

### Bedrock guardrails {#bedrock-guardrails}

Add `guardrail_identifier` to apply a guardrail to Converse inference requests. This does not apply to listing calls or custom Mantle models:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "guardrail_identifier": "arn:aws:bedrock:us-east-1:123456789012:guardrail/abc123",
      "guardrail_version": "DRAFT"
    }
  }
}
```

### Custom Bedrock models {#bedrock-custom-models}

Add models that Zed does not list, or use your own values for a listed model, with `available_models`. Use the Bedrock model ID as `name`, including a region prefix when you want a specific inference profile:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "available_models": [
        {
          "name": "arn:aws:bedrock:us-east-1:123456789012:application-inference-profile/abcdef123456",
          "display_name": "My application profile",
          "max_tokens": 200000,
          "max_output_tokens": 8192,
          "supports_tools": true,
          "supports_images": false
        },
        {
          "name": "us.anthropic.claude-opus-4-5-20251101-v1:0",
          "display_name": "Opus 4.5 (budget thinking)",
          "max_tokens": 200000,
          "max_output_tokens": 8192,
          "supports_tools": true,
          "supports_images": true,
          "thinking": {
            "adaptive": false,
            "budget_tokens": 5000
          }
        }
      ]
    }
  }
}
```

`name` is sent to Bedrock exactly as written. Use a full model ID, profile ID, or application-profile ARN that your account can invoke. The first example uses a placeholder ARN. Replace it and its capabilities with your own profile's values. The second requests Anthropic budget thinking with 5,000 thinking tokens and an 8,192-token output cap when thinking is enabled.

Zed first looks for `name` in the AWS list, then in the public catalog. For catalog capabilities, it prefers the exact requested profile, then the foundation model, then the lexicographically first catalog profile for that foundation model. A known entry inherits the display name, output limit, tools, images, caching, and thinking controls you omit. You must supply `max_tokens`. `default_temperature` is not inherited. A known model's temperature support still controls whether temperature is sent. A model that always reasons keeps that behavior regardless of `thinking`.

An unknown custom entry defaults to a 4,096-token output limit, no tools, images, caching, or thinking, and no temperature unless you supply `default_temperature`. Set capabilities to match the model. Set `thinking: true` for Anthropic budget thinking with the default 4,096-token budget, or use `{"adaptive": false, "budget_tokens": 5000}` for a different budget. `{"adaptive": true}` selects Anthropic adaptive thinking. `has_xhigh` adds that effort, while `budget_tokens` is ignored in adaptive mode. These settings are not generic reasoning controls for other vendors.

Custom entries are pinned routes. Pinned entries and application profiles receive no automatic refusal fallback to another model. Automatic routing can offer a refusal fallback only when Zed also offers the fallback model.

When an output cap is no greater than a thinking budget, Zed reduces that budget to half the cap. If the resulting budget is below 1,024 tokens, Zed omits budget thinking. Zed also omits temperature while sending Anthropic thinking controls.

You cannot continue a thread containing tool calls or images on a Converse model that lacks the respective capability. The request fails in Zed with a message asking you to choose another model or start a new thread. This includes images in tool results.

### Bedrock custom endpoints {#bedrock-custom-endpoints}

To send Bedrock Converse requests to a proxy or gateway instead of AWS, set `endpoint_url`:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "endpoint_url": "https://gateway.example.com/bedrock",
      "region": "us-east-1",
      "authentication_method": "api_key"
    }
  }
}
```

Zed lists models from AWS, not from your gateway. With `endpoint_url` set, it offers the models that the [models.dev](https://models.dev) catalog names for your Region. Add any other model your gateway serves as a [custom Bedrock model](#bedrock-custom-models).

[Mantle models](#bedrock-mantle-models), which are the models in `mantle_available_models`, are called through a different service with a different request shape, so they ignore `endpoint_url` and go to AWS.

If your gateway serves one of those models over the Converse API, add it as a [custom Bedrock model](#bedrock-custom-models) so Zed calls it through `endpoint_url`. Use the model ID your gateway expects as `name`:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "endpoint_url": "https://gateway.example.com/bedrock",
      "available_models": [
        {
          "name": "us.openai.gpt-5.6-luna",
          "display_name": "GPT-5.6 Luna (Gateway)",
          "max_tokens": 1000000,
          "max_output_tokens": 128000,
          "supports_tools": true,
          "supports_images": true
        }
      ]
    }
  }
}
```

Don't use an ID that a model in `mantle_available_models` already uses as `name`. A Mantle model with the same ID replaces the custom model, so requests go to AWS instead of your gateway.

### Bedrock Mantle models {#bedrock-mantle-models}

`bedrock-mantle` is AWS's OpenAI-compatible inference endpoint. Zed does not list its models. To use a model that is only reachable through `bedrock-mantle`, add it with `mantle_available_models`.

Zed has no built-in Mantle models. Only your `mantle_available_models` entries use this transport.

Custom Mantle models need authorization for the `bedrock-mantle` endpoint, separate from Converse permissions. Zed restricts these requests to its configured supported Region list. If your Region is outside that list, Zed returns an error naming it and the configured supported Regions before sending a request. See the [Bedrock Mantle documentation](https://docs.aws.amazon.com/bedrock/latest/userguide/bedrock-mantle.html#regions) for service availability.

#### Custom Bedrock Mantle models {#bedrock-mantle-custom-models}

Add models served through `bedrock-mantle` with `mantle_available_models`:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "mantle_available_models": [
        {
          "name": "openai.gpt-oss-120b",
          "display_name": "GPT-OSS 120B",
          "max_tokens": 128000,
          "protocol": "chat_completions",
          "supports_tools": true,
          "supports_images": false,
          "supports_thinking": true
        }
      ]
    }
  }
}
```

`protocol` selects which OpenAI-compatible API the model is called through, and must be either `chat_completions` or `responses`. Set `supports_thinking` to `true` for custom Mantle models that accept OpenAI reasoning effort parameters. Zed then exposes `low`, `medium`, `high`, and `xhigh` in the thinking effort picker, while disabling thinking sends `none`.

### Upgrade from earlier versions {#bedrock-upgrade}

- **Authentication:** A model in settings can appear before sign-in. Requests from an unauthenticated provider now fail, including in an existing thread. To use the AWS default credential chain, set `authentication_method` to `"default"`.
- **Saved short IDs:** A saved selection such as `{"provider": "amazon-bedrock", "model": "claude-sonnet-4-5"}` continues to select `anthropic.claude-sonnet-4-5-20250929-v1:0` when that model is offered. Before and after the upgrade, the saved short ID stays `claude-sonnet-4-5`. Requests now use a discovered inference profile.
- **Former built-in Mantle models:** Before the upgrade, a saved `gpt-5.5` selection used Mantle. After the upgrade, the same selection resolves to `openai.gpt-5.5` over Converse when it is listed. This path needs Converse invocation permissions, not only Mantle permissions. To keep using Mantle, add a `mantle_available_models` entry and select its full `name`, such as `openai.gpt-5.5`.
- **Models without inference profiles:** Before the upgrade, a built-in selection such as `qwen3-32b` could refer to an on-demand model. If `qwen.qwen3-32b-v1:0` has no inference profile in your listing, it is no longer offered automatically. After the upgrade, add that full ID to `available_models` and select it:

```json [settings]
{
  "language_models": {
    "bedrock": {
      "available_models": [
        {
          "name": "qwen.qwen3-32b-v1:0",
          "max_tokens": 128000,
          "max_output_tokens": 8192,
          "supports_tools": true
        }
      ]
    }
  },
  "agent": {
    "default_model": {
      "provider": "amazon-bedrock",
      "model": "qwen.qwen3-32b-v1:0"
    }
  }
}
```

Choose limits and capabilities appropriate for the model you invoke. This example does not verify your account's access.

### Bedrock permissions {#bedrock-permissions}

Converse requests need `bedrock:InvokeModelWithResponseStream` for streaming and `bedrock:InvokeModel` for invocation authorization. If your credentials cannot invoke the selected model or profile, the request fails with Bedrock's error. A model appearing in the picker does not grant these permissions.

Discovery needs `bedrock:ListInferenceProfiles` to list system and application profiles, and `bedrock:ListFoundationModels` to obtain model names and modalities. If either required listing call fails without an earlier account list, Zed uses the public-catalog fallback described above. If only the application-profile call fails, Zed keeps the system-profile list without application profiles. Requests do not require listing permissions when you configure a custom model or use the public fallback.

## OpenAI-Compatible Gateways {#openai-compatible}

If your gateway exposes an OpenAI-compatible API, configure it with [Use API Access](./use-api-access.md#openai-compatible).
