# Billing

We use Stripe as our payments provider, and Orb for invoicing and metering. All Pro plans require payment via credit card or other supported payment method.
For invoice-based billing, a Business plan is required. Contact [sales@zed.dev](mailto:sales@zed.dev) for more information.

## Billing Information {#settings}

You can access billing information and settings at [zed.dev/account](https://zed.dev/account).
Most of the page embeds information from our invoicing/metering partner, Orb (we're planning on a more native experience soon!).

## Billing Cycles {#billing-cycles}

Zed is billed on a monthly basis based on the date you initially subscribe. You'll receive _at least_ one invoice from Zed each month you're subscribed to Zed Pro, and more than one if you use more than $10 in incremental token spend within the month.

## Threshold Billing {#threshold-billing}

Zed utilizes threshold billing to ensure timely collection of owed monies and prevent abuse. Every time your usage of Zed's hosted models crosses a $10 spend threshold, a new invoice is generated, and the threshold resets to $0.

For example,

- You subscribe on February 1. Your first invoice is $10.
- You use $12 of incremental tokens in the month of February, with the first $10 spent on February 15. You'll receive an invoice for $10 on February 15
- On March 1, you receive an invoice for $12: $10 (March Pro subscription) and $2 in leftover token spend, since your usage didn't cross the $10 threshold.

## Payment Failures {#payment-failures}

If payment of an invoice fails, Zed will block usage of our hosted models until the payment is complete. Email [billing-support@zed.dev](mailto:billing-support@zed.dev) for assistance.

## Invoice History {#invoice-history}

You can access your invoice history by navigating to [zed.dev/account](https://zed.dev/account) and clicking `Invoice history` within the embedded Orb portal.

If you require historical Stripe invoices, email [billing-support@zed.dev](mailto:billing-support@zed.dev)

## Updating Billing Information {#updating-billing-info}

Email [billing-support@zed.dev](mailto:billing-support@zed.dev) for help updating payment methods, names, addresses, and tax information.

> We'll be updating our account page shortly to allow for self-service updates. Stay tuned!

Please note that changes to billing information will **only** affect future invoices — **we cannot modify historical invoices**.

## Sales Tax {#sales-tax}

Zed partners with [Sphere](https://www.getsphere.com/) to calculate indirect tax rate for invoices, based on customer location and the product being sold. Tax is listed as a separate line item on invoices, based preferentially on your billing address, followed by the card issue country known to Stripe.

If you have a VAT/GST ID, you can add it at during checkout. Check the box that denotes you as a business.

Please note that changes to VAT/GST IDs and address will **only** affect future invoices — **we cannot modify historical invoices**.
Questions or issues can be directed to [billing-support@zed.dev](mailto:billing-support@zed.dev).
