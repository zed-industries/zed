---
title: Billing
description: Manage billing for your Zed subscription, including payment methods, invoices, and sales tax information for individual and organization accounts.
---

# Billing

Zed uses Stripe for payment processing. All plans that require payment do so via credit card or other supported payment methods. Individual Pro subscriptions also use Orb for invoicing and metering.

For details on what's included in each plan and how token usage works, see [Plans & Pricing](./plans-and-usage.md).

## Individual billing {#individual}

### Billing information {#settings}

Access billing information and settings from your [Zed dashboard](https://dashboard.zed.dev).
This page embeds data from Orb, our invoicing and metering partner.

### Billing cycles {#billing-cycles}

Zed is billed on a monthly basis based on the date you initially subscribe. You'll receive _at least_ one invoice from Zed each month you're subscribed to Zed Pro, and may receive more than one invoice if you use hosted models beyond your included monthly token credit.

### Zed Pro threshold billing {#threshold-billing}

For individual Zed Pro subscriptions, Zed uses threshold billing to ensure timely payment collection. Threshold billing controls when already-allowed token usage is invoiced during the month; your [monthly spend limit](./plans-and-usage.md#usage-spend-limits) still controls when hosted model usage stops.

Threshold invoices start at $10 of pre-tax incremental token spend. For higher token usage, Zed may automatically raise your pre-tax invoicing threshold in $10 increments, up to $100, so you receive fewer mid-cycle invoices. Once raised, the invoicing threshold is not automatically lowered during the same subscription.

For Zed Business billing, see [Organization billing](#organization).

For example:

- You subscribe on February 1. Your first invoice is $10.
- You use $12 of incremental tokens in the month of February, with the first $10 spent on February 15. You'll receive an invoice for $10 on February 15.
- On March 1, you receive your next monthly subscription invoice, plus any remaining token spend that was not already invoiced during February.

### Payment failures {#payment-failures}

If payment of an invoice fails, Zed will block usage of our hosted models until the payment is complete. Email [billing-support@zed.dev](mailto:billing-support@zed.dev) for assistance.

### Invoice history {#invoice-history}

You can access your invoice history from the Billing page at [dashboard.zed.dev](https://dashboard.zed.dev) by clicking `Invoice history` within the embedded Orb portal.

If you require historical Stripe invoices, email [billing-support@zed.dev](mailto:billing-support@zed.dev).

## Organization billing {#organization}

Zed Business consolidates your team's costs. Seat licenses and AI usage for all members appear on one bill, with no separate invoices per member. For a full feature overview, see [Zed Business](../business/overview.md).

### Billing dashboard {#dashboard}

Owners and admins can access billing information at [dashboard.zed.dev](https://dashboard.zed.dev). The dashboard shows the plan you're currently on and offers jumping off points to update billing details, such as the billing name and address, as well as payment information. You can also access your invoice history, accessible through the Orb billing portal.

### AI usage {#ai-usage}

AI usage across the organization is metered on a token basis at the same rates as individual Pro subscriptions. See [Plans & Pricing](./plans-and-usage.md#usage) for rate details.

Administrators can set an org-wide AI spend limit from the Data & Privacy page in the organization dashboard. The limit starts at $0, so it must be increased before members can use any hosted models. Once the limit is reached, members will see an error when attempting to use hosted models.

### Invoice history {#org-invoice-history}

Owners and Admins can access an organization's invoice history from the Billing page at [dashboard.zed.dev](https://dashboard.zed.dev) by clicking `Invoice history` within the embedded Orb portal.

If you require historical Stripe invoices, email [billing-support@zed.dev](mailto:billing-support@zed.dev).

## Updating billing information {#updating-billing-info}

From the _Billing_ page, owners can update their billing name, address, and payment method. Tax IDs are collected during checkout and cannot be changed self-serve; email [billing-support@zed.dev](mailto:billing-support@zed.dev) to update your tax ID.

Changes to billing information will **only** affect future invoices. We cannot modify historical invoices. Email [billing-support@zed.dev](mailto:billing-support@zed.dev) with any questions.

## Sales tax {#sales-tax}

Zed partners with [Sphere](https://www.getsphere.com/) to calculate indirect tax rates for invoices, based on customer location and the product being sold. Tax is listed as a separate line item on invoices, based preferentially on your billing address, followed by the card issue country known to Stripe.

If you have a VAT/GST ID, you can add it during checkout. Check the box that denotes you as a business.

Changes to VAT/GST IDs and address will **only** affect future invoices. We cannot modify historical invoices.

Email [billing-support@zed.dev](mailto:billing-support@zed.dev) with any tax questions.
