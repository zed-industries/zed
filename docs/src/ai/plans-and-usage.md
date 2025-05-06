# Plans and Usage

To view your current usage, you can visit your account at [zed.dev/account](https://zed.dev/account).
You’ll also find usage meters in-product when you’re nearing the limit for your plan or trial.

## Available Plans {#plans}

For costs and more information on pricing, visit [Zed’s pricing page](zed.dev/pricing).

Please note that if you’re interested in just using Zed as the world’s fastest editor, with no AI or subscription features, you can always do so for free, without [authentication](../accounts.md).

## Usage {#usage}

- A `prompt` in Zed is an input from the user, initiated on pressing enter, composed of one or many `requests`. A `prompt` can be initiated from the Agent Panel, or via Inline Assist.
- A `request` in Zed is a response to a `prompt`, plus any tool calls that are initiated as part of that response. There may be one `request` per `prompt`, or many.

Most models offered by Zed are metered per-prompt.
Some models that use large context windows and unlimited tool calls ([“Max Mode”](./models.md#max-mode)) count each individual request within a prompt against your prompt limit, since the agentic work spawned by the prompt is expensive to support.

See [the Models page](./models.md) for a list of which subset of models are metered by request.

Plans come with a set amount of prompts included, with the number varying depending on the plan you’ve selected.

## Usage-Based Pricing {#ubp}

You may opt in to usage-based pricing for prompts that exceed what is included in your paid plan from [your account page](https://zed.dev/account).

Usage-based pricing is only available with a paid plan, and is exclusively opt-in.
From the dashboard, you can toggle usage-based pricing for usage exceeding your paid plan.
You can also configure a spend limit in USD.
Once the spend limit is hit, we’ll stop any further usage until your prompt limit resets.

We will bill for additional prompts when you’ve made prompts totaling $20, or when your billing date occurs, whichever comes first.

Cost per request for each model can be found on [the models page](./models.md).

## Business Usage {#business-usage}

Email [sales@zed.dev](mailto:sales@zed.dev) with any questions on business plans, metering, and usage-based pricing.
