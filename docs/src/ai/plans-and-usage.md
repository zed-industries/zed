---
title: Plans & Pricing
description: Compare Zed's Free, Pro, and Business plans, and understand token-based usage metering, spend limits, and trial details.
---

# Plans & Pricing

For costs and more information on pricing, visit [Zed's pricing page](https://zed.dev/pricing).

Zed works without AI features or a subscription. No [authentication](../authentication.md) is required for the editor itself.

## Plans {#plans}

|                                           | Free    | Pro       | Student   | Business  |
| ----------------------------------------- | ------- | --------- | --------- | --------- |
| Zed-hosted AI models                      | —       | ✓         | ✓         | ✓         |
| [AI via own API keys](./llm-providers.md) | ✓       | ✓         | ✓         | ✓         |
| [External agents](./external-agents.md)   | ✓       | ✓         | ✓         | ✓         |
| Edit Predictions                          | Limited | Unlimited | Unlimited | Unlimited |
| Org-wide admin controls                   | —       | —         | —         | ✓         |
| Roles & permissions                       | —       | —         | —         | ✓         |
| Consolidated billing                      | —       | —         | —         | ✓         |

### Zed Free {#free}

Zed is free to use. You can configure AI agents with your own API keys via [Providers](./llm-providers.md). [Edit Predictions](./edit-prediction.md) are available on a limited basis. Zed's hosted models require a Pro subscription.

### Zed Pro {#pro}

Zed Pro includes access to all hosted AI models and Edit Predictions. The plan includes $5 of monthly token credit; usage beyond that is billed at the rates listed on [the Models page](./models.md). A trial of Zed Pro includes $20 of credit, usable for 14 days.

For details on billing and payment, see [Individual Billing](./billing.md).

### Zed Business {#business}

Zed Business gives every member access to all of Zed's hosted AI models, unlimited edit predictions, plus org-wide controls for administrators: which AI features are available, what data leaves your organization, and how AI spend is tracked. All seats and AI usage are consolidated into a single invoice.

For a full feature overview, see [Zed Business](../business/overview.md). For billing details, see [Billing](./billing.md#organization).

### Student Plan {#student}

The [Zed Student plan](https://zed.dev/education) includes all Zed Pro features: unlimited [Edit Predictions](./edit-prediction.md), all [hosted AI models](./models.md) except Claude Opus, and $10/month in token credits. Available free for one year to verified university students.

## Usage {#usage}

Usage of Zed's hosted models is measured on a token basis, converted to dollars at the rates listed on [the Models page](./models.md) (list price from the provider, +10%).

Monthly included credit resets on your monthly billing date. To view your current usage, navigate to the Billing page at [dashboard.zed.dev](https://dashboard.zed.dev). Usage data from our metering provider, Orb, is embedded on that page.

## Spend Limits {#usage-spend-limits}

On your Billing page you'll find an input for `Monthly Spend Limit`. The dollar amount here specifies your _monthly_ limit for spend on tokens, _not counting_ the $5/month included with your Pro subscription.

The default value for all Pro users is $10, for a total monthly spend with Zed of $20 ($10 for your Pro subscription, $10 in incremental token spend). This can be set to $0 to limit your spend with Zed to exactly $10/month. If you adjust this limit _higher_ than $10 and consume more than $10 of incremental token spend, you'll be billed via [threshold billing](./billing.md#threshold-billing).

Once the spend limit is hit, we'll stop any further usage until your token spend limit resets.

On Zed Business, administrators set an org-wide spend limit from the Data & Privacy page in the organization dashboard. See [Organization Billing](./billing.md#ai-usage) for details.

> **Note:** Spend limits are a Zed Pro and Business feature. Student plan users cannot configure spend limits; usage is capped at the $10/month included credit.

### Trials {#trials}

Trials automatically convert to Zed Free when they end. Trials do not include access to Anthropic's Opus models. No cancellation is needed to prevent conversion to Zed Pro.
