# Stripe Automatic Tax Implementation

This document describes the implementation of automatic tax support for Stripe subscriptions in the Zed codebase.

## Overview

We've added support for Stripe's automatic tax calculation feature to ensure tax compliance for subscriptions. When enabled, Stripe automatically calculates taxes based on:
- Customer location (billing address)
- Configured tax registrations in your Stripe account
- Product tax codes

## Implementation Details

### 1. Type Definitions (`stripe_client.rs`)

Added new types to represent automatic tax configuration:

```rust
pub struct StripeAutomaticTax {
    pub enabled: bool,
    pub liability: Option<StripeAutomaticTaxLiability>,
}

pub enum StripeAutomaticTaxLiability {
    Account,  // Platform account is responsible
    Self_,    // Connected account is responsible
}
```

### 2. API Updates

Updated subscription creation and update parameters:

```rust
pub struct StripeCreateSubscriptionParams {
    // ... existing fields ...
    pub automatic_tax: Option<StripeAutomaticTax>,
}

pub struct UpdateSubscriptionParams {
    // ... existing fields ...
    pub automatic_tax: Option<StripeAutomaticTax>,
}
```

### 3. Business Logic (`stripe_billing.rs`)

All subscription creation methods now enable automatic tax by default:

- `subscribe_to_price()` - Enables automatic tax when updating subscriptions
- `subscribe_to_zed_free()` - Enables automatic tax for free tier subscriptions
- `checkout_with_zed_pro()` and `checkout_with_zed_pro_trial()` - Already collect billing addresses and tax IDs

### 4. Client Implementations

#### Real Stripe Client (`real_stripe_client.rs`)
- Prepared for automatic tax support
- Currently includes TODOs as async-stripe may need updates to fully support the automatic_tax field
- The implementation is ready to be activated once the underlying library supports it

#### Fake Client (`fake_stripe_client.rs`)
- Accepts automatic_tax parameters for testing
- Doesn't process them but maintains API compatibility

## Usage

When creating or updating subscriptions, automatic tax is now enabled by default:

```rust
let params = StripeCreateSubscriptionParams {
    customer: customer_id,
    items: vec![/* ... */],
    automatic_tax: Some(StripeAutomaticTax {
        enabled: true,
        liability: None,  // Let Stripe determine liability
    }),
};
```

## Prerequisites

For automatic tax to work correctly:

1. **Tax Registrations**: Configure tax registrations in your Stripe Dashboard
2. **Customer Address**: Ensure customers have valid billing addresses
3. **Product Tax Codes**: Assign appropriate tax codes to your products

## Customer Address Collection

- Checkout sessions already collect billing addresses (`billing_address_collection: Required`)
- Direct subscription creation (e.g., `subscribe_to_zed_free`) requires customers to have addresses beforehand

## Error Handling

If automatic tax calculation fails:
- The subscription creation should still proceed
- Tax calculation errors should be logged but not block the transaction
- Monitor the `automatic_tax.status` field in responses

## Testing

Added comprehensive tests in `stripe_automatic_tax_test.rs`:
- Test subscription creation with automatic tax
- Test subscription updates with automatic tax
- Test integration with StripeBilling

## Future Considerations

1. **Webhook Handling**: Consider adding handlers for tax-related webhook events
2. **Tax Display**: Update UI to show tax amounts on invoices
3. **Price Display**: Consider showing "plus applicable taxes" on pricing pages
4. **Monitoring**: Track the `automatic_tax.liability` field to understand tax responsibility

## Migration Notes

Existing subscriptions will need to be updated to enable automatic tax. This can be done:
1. Automatically during the next subscription update
2. Via a batch migration script
3. Using Stripe's Dashboard migration tools

## Related Documentation

- [Stripe Tax Documentation](https://docs.stripe.com/tax)
- [Collect taxes for recurring payments](https://docs.stripe.com/tax/subscriptions)
- [Migrate to Stripe Tax](https://docs.stripe.com/billing/taxes/migration)