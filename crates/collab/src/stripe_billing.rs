use std::sync::Arc;

use crate::Result;
use crate::db::billing_subscription::SubscriptionKind;
use crate::llm::AGENT_EXTENDED_TRIAL_FEATURE_FLAG;
use anyhow::{Context as _, anyhow};
use chrono::Utc;
use collections::HashMap;
use serde::{Deserialize, Serialize};
use stripe::{CreateCustomer, Customer, CustomerId, PriceId, SubscriptionStatus};
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct StripeBilling {
    state: RwLock<StripeBillingState>,
    client: Arc<stripe::Client>,
}

#[derive(Default)]
struct StripeBillingState {
    meters_by_event_name: HashMap<String, StripeMeter>,
    price_ids_by_meter_id: HashMap<String, stripe::PriceId>,
    prices_by_lookup_key: HashMap<String, stripe::Price>,
}

impl StripeBilling {
    pub fn new(client: Arc<stripe::Client>) -> Self {
        Self {
            client,
            state: RwLock::default(),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        log::info!("StripeBilling: initializing");

        let mut state = self.state.write().await;

        let (meters, prices) = futures::try_join!(
            StripeMeter::list(&self.client),
            stripe::Price::list(
                &self.client,
                &stripe::ListPrices {
                    limit: Some(100),
                    ..Default::default()
                }
            )
        )?;

        for meter in meters.data {
            state
                .meters_by_event_name
                .insert(meter.event_name.clone(), meter);
        }

        for price in prices.data {
            if let Some(lookup_key) = price.lookup_key.clone() {
                state.prices_by_lookup_key.insert(lookup_key, price.clone());
            }

            if let Some(recurring) = price.recurring {
                if let Some(meter) = recurring.meter {
                    state.price_ids_by_meter_id.insert(meter, price.id);
                }
            }
        }

        log::info!("StripeBilling: initialized");

        Ok(())
    }

    pub async fn zed_pro_price_id(&self) -> Result<PriceId> {
        self.find_price_id_by_lookup_key("zed-pro").await
    }

    pub async fn zed_free_price_id(&self) -> Result<PriceId> {
        self.find_price_id_by_lookup_key("zed-free").await
    }

    pub async fn find_price_id_by_lookup_key(&self, lookup_key: &str) -> Result<PriceId> {
        self.state
            .read()
            .await
            .prices_by_lookup_key
            .get(lookup_key)
            .map(|price| price.id.clone())
            .ok_or_else(|| crate::Error::Internal(anyhow!("no price ID found for {lookup_key:?}")))
    }

    pub async fn find_price_by_lookup_key(&self, lookup_key: &str) -> Result<stripe::Price> {
        self.state
            .read()
            .await
            .prices_by_lookup_key
            .get(lookup_key)
            .cloned()
            .ok_or_else(|| crate::Error::Internal(anyhow!("no price found for {lookup_key:?}")))
    }

    pub async fn determine_subscription_kind(
        &self,
        subscription: &stripe::Subscription,
    ) -> Option<SubscriptionKind> {
        let zed_pro_price_id = self.zed_pro_price_id().await.ok()?;
        let zed_free_price_id = self.zed_free_price_id().await.ok()?;

        subscription.items.data.iter().find_map(|item| {
            let price = item.price.as_ref()?;

            if price.id == zed_pro_price_id {
                Some(if subscription.status == SubscriptionStatus::Trialing {
                    SubscriptionKind::ZedProTrial
                } else {
                    SubscriptionKind::ZedPro
                })
            } else if price.id == zed_free_price_id {
                Some(SubscriptionKind::ZedFree)
            } else {
                None
            }
        })
    }

    /// Returns the Stripe customer associated with the provided email address, or creates a new customer, if one does
    /// not already exist.
    ///
    /// Always returns a new Stripe customer if the email address is `None`.
    pub async fn find_or_create_customer_by_email(
        &self,
        email_address: Option<&str>,
    ) -> Result<CustomerId> {
        let existing_customer = if let Some(email) = email_address {
            let customers = Customer::list(
                &self.client,
                &stripe::ListCustomers {
                    email: Some(email),
                    ..Default::default()
                },
            )
            .await?;

            customers.data.first().cloned()
        } else {
            None
        };

        let customer_id = if let Some(existing_customer) = existing_customer {
            existing_customer.id
        } else {
            let customer = Customer::create(
                &self.client,
                CreateCustomer {
                    email: email_address,
                    ..Default::default()
                },
            )
            .await?;

            customer.id
        };

        Ok(customer_id)
    }

    pub async fn subscribe_to_price(
        &self,
        subscription_id: &stripe::SubscriptionId,
        price: &stripe::Price,
    ) -> Result<()> {
        let subscription =
            stripe::Subscription::retrieve(&self.client, &subscription_id, &[]).await?;

        if subscription_contains_price(&subscription, &price.id) {
            return Ok(());
        }

        const BILLING_THRESHOLD_IN_CENTS: i64 = 20 * 100;

        let price_per_unit = price.unit_amount.unwrap_or_default();
        let _units_for_billing_threshold = BILLING_THRESHOLD_IN_CENTS / price_per_unit;

        stripe::Subscription::update(
            &self.client,
            subscription_id,
            stripe::UpdateSubscription {
                items: Some(vec![stripe::UpdateSubscriptionItems {
                    price: Some(price.id.to_string()),
                    ..Default::default()
                }]),
                trial_settings: Some(stripe::UpdateSubscriptionTrialSettings {
                    end_behavior: stripe::UpdateSubscriptionTrialSettingsEndBehavior {
                        missing_payment_method: stripe::UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
                    },
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    pub async fn bill_model_request_usage(
        &self,
        customer_id: &stripe::CustomerId,
        event_name: &str,
        requests: i32,
    ) -> Result<()> {
        let timestamp = Utc::now().timestamp();
        let idempotency_key = Uuid::new_v4();

        StripeMeterEvent::create(
            &self.client,
            StripeCreateMeterEventParams {
                identifier: &format!("model_requests/{}", idempotency_key),
                event_name,
                payload: StripeCreateMeterEventPayload {
                    value: requests as u64,
                    stripe_customer_id: customer_id,
                },
                timestamp: Some(timestamp),
            },
        )
        .await?;

        Ok(())
    }

    pub async fn checkout_with_zed_pro(
        &self,
        customer_id: stripe::CustomerId,
        github_login: &str,
        success_url: &str,
    ) -> Result<String> {
        let zed_pro_price_id = self.zed_pro_price_id().await?;

        let mut params = stripe::CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.customer = Some(customer_id);
        params.client_reference_id = Some(github_login);
        params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            price: Some(zed_pro_price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);
        params.success_url = Some(success_url);

        let session = stripe::CheckoutSession::create(&self.client, params).await?;
        Ok(session.url.context("no checkout session URL")?)
    }

    pub async fn checkout_with_zed_pro_trial(
        &self,
        customer_id: stripe::CustomerId,
        github_login: &str,
        feature_flags: Vec<String>,
        success_url: &str,
    ) -> Result<String> {
        let zed_pro_price_id = self.zed_pro_price_id().await?;

        let eligible_for_extended_trial = feature_flags
            .iter()
            .any(|flag| flag == AGENT_EXTENDED_TRIAL_FEATURE_FLAG);

        let trial_period_days = if eligible_for_extended_trial { 60 } else { 14 };

        let mut subscription_metadata = std::collections::HashMap::new();
        if eligible_for_extended_trial {
            subscription_metadata.insert(
                "promo_feature_flag".to_string(),
                AGENT_EXTENDED_TRIAL_FEATURE_FLAG.to_string(),
            );
        }

        let mut params = stripe::CreateCheckoutSession::new();
        params.subscription_data = Some(stripe::CreateCheckoutSessionSubscriptionData {
            trial_period_days: Some(trial_period_days),
            trial_settings: Some(stripe::CreateCheckoutSessionSubscriptionDataTrialSettings {
                end_behavior: stripe::CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior {
                    missing_payment_method: stripe::CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
                }
            }),
            metadata: if !subscription_metadata.is_empty() {
                Some(subscription_metadata)
            } else {
                None
            },
            ..Default::default()
        });
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.payment_method_collection =
            Some(stripe::CheckoutSessionPaymentMethodCollection::IfRequired);
        params.customer = Some(customer_id);
        params.client_reference_id = Some(github_login);
        params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            price: Some(zed_pro_price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);
        params.success_url = Some(success_url);

        let session = stripe::CheckoutSession::create(&self.client, params).await?;
        Ok(session.url.context("no checkout session URL")?)
    }

    pub async fn subscribe_to_zed_free(
        &self,
        customer_id: stripe::CustomerId,
    ) -> Result<stripe::Subscription> {
        let zed_free_price_id = self.zed_free_price_id().await?;

        let existing_subscriptions = stripe::Subscription::list(
            &self.client,
            &stripe::ListSubscriptions {
                customer: Some(customer_id.clone()),
                status: None,
                ..Default::default()
            },
        )
        .await?;

        let existing_active_subscription =
            existing_subscriptions
                .data
                .into_iter()
                .find(|subscription| {
                    subscription.status == SubscriptionStatus::Active
                        || subscription.status == SubscriptionStatus::Trialing
                });
        if let Some(subscription) = existing_active_subscription {
            return Ok(subscription);
        }

        let mut params = stripe::CreateSubscription::new(customer_id);
        params.items = Some(vec![stripe::CreateSubscriptionItems {
            price: Some(zed_free_price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);

        let subscription = stripe::Subscription::create(&self.client, params).await?;

        Ok(subscription)
    }

    pub async fn checkout_with_zed_free(
        &self,
        customer_id: stripe::CustomerId,
        github_login: &str,
        success_url: &str,
    ) -> Result<String> {
        let zed_free_price_id = self.zed_free_price_id().await?;

        let mut params = stripe::CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.payment_method_collection =
            Some(stripe::CheckoutSessionPaymentMethodCollection::IfRequired);
        params.customer = Some(customer_id);
        params.client_reference_id = Some(github_login);
        params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            price: Some(zed_free_price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);
        params.success_url = Some(success_url);

        let session = stripe::CheckoutSession::create(&self.client, params).await?;
        Ok(session.url.context("no checkout session URL")?)
    }
}

#[derive(Clone, Deserialize)]
struct StripeMeter {
    id: String,
    event_name: String,
}

impl StripeMeter {
    pub fn list(client: &stripe::Client) -> stripe::Response<stripe::List<Self>> {
        #[derive(Serialize)]
        struct Params {
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u64>,
        }

        client.get_query("/billing/meters", Params { limit: Some(100) })
    }
}

#[derive(Deserialize)]
struct StripeMeterEvent {
    identifier: String,
}

impl StripeMeterEvent {
    pub async fn create(
        client: &stripe::Client,
        params: StripeCreateMeterEventParams<'_>,
    ) -> Result<Self, stripe::StripeError> {
        let identifier = params.identifier;
        match client.post_form("/billing/meter_events", params).await {
            Ok(event) => Ok(event),
            Err(stripe::StripeError::Stripe(error)) => {
                if error.http_status == 400
                    && error
                        .message
                        .as_ref()
                        .map_or(false, |message| message.contains(identifier))
                {
                    Ok(Self {
                        identifier: identifier.to_string(),
                    })
                } else {
                    Err(stripe::StripeError::Stripe(error))
                }
            }
            Err(error) => Err(error),
        }
    }
}

#[derive(Serialize)]
struct StripeCreateMeterEventParams<'a> {
    identifier: &'a str,
    event_name: &'a str,
    payload: StripeCreateMeterEventPayload<'a>,
    timestamp: Option<i64>,
}

#[derive(Serialize)]
struct StripeCreateMeterEventPayload<'a> {
    value: u64,
    stripe_customer_id: &'a stripe::CustomerId,
}

fn subscription_contains_price(
    subscription: &stripe::Subscription,
    price_id: &stripe::PriceId,
) -> bool {
    subscription.items.data.iter().any(|item| {
        item.price
            .as_ref()
            .map_or(false, |price| price.id == *price_id)
    })
}
