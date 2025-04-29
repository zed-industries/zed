use std::sync::Arc;

use crate::{Cents, Result, llm};
use anyhow::{Context as _, anyhow};
use chrono::{Datelike, Utc};
use collections::HashMap;
use serde::{Deserialize, Serialize};
use stripe::PriceId;
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

pub struct StripeModelTokenPrices {
    input_tokens_price: StripeBillingPrice,
    input_cache_creation_tokens_price: StripeBillingPrice,
    input_cache_read_tokens_price: StripeBillingPrice,
    output_tokens_price: StripeBillingPrice,
}

struct StripeBillingPrice {
    id: stripe::PriceId,
    meter_event_name: String,
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

    pub async fn find_price_by_lookup_key(&self, lookup_key: &str) -> Result<stripe::Price> {
        self.state
            .read()
            .await
            .prices_by_lookup_key
            .get(lookup_key)
            .cloned()
            .ok_or_else(|| crate::Error::Internal(anyhow!("no price ID found for {lookup_key:?}")))
    }

    pub async fn register_model_for_token_based_usage(
        &self,
        model: &llm::db::model::Model,
    ) -> Result<StripeModelTokenPrices> {
        let input_tokens_price = self
            .get_or_insert_token_price(
                &format!("model_{}/input_tokens", model.id),
                &format!("{} (Input Tokens)", model.name),
                Cents::new(model.price_per_million_input_tokens as u32),
            )
            .await?;
        let input_cache_creation_tokens_price = self
            .get_or_insert_token_price(
                &format!("model_{}/input_cache_creation_tokens", model.id),
                &format!("{} (Input Cache Creation Tokens)", model.name),
                Cents::new(model.price_per_million_cache_creation_input_tokens as u32),
            )
            .await?;
        let input_cache_read_tokens_price = self
            .get_or_insert_token_price(
                &format!("model_{}/input_cache_read_tokens", model.id),
                &format!("{} (Input Cache Read Tokens)", model.name),
                Cents::new(model.price_per_million_cache_read_input_tokens as u32),
            )
            .await?;
        let output_tokens_price = self
            .get_or_insert_token_price(
                &format!("model_{}/output_tokens", model.id),
                &format!("{} (Output Tokens)", model.name),
                Cents::new(model.price_per_million_output_tokens as u32),
            )
            .await?;
        Ok(StripeModelTokenPrices {
            input_tokens_price,
            input_cache_creation_tokens_price,
            input_cache_read_tokens_price,
            output_tokens_price,
        })
    }

    async fn get_or_insert_token_price(
        &self,
        meter_event_name: &str,
        price_description: &str,
        price_per_million_tokens: Cents,
    ) -> Result<StripeBillingPrice> {
        // Fast code path when the meter and the price already exist.
        {
            let state = self.state.read().await;
            if let Some(meter) = state.meters_by_event_name.get(meter_event_name) {
                if let Some(price_id) = state.price_ids_by_meter_id.get(&meter.id) {
                    return Ok(StripeBillingPrice {
                        id: price_id.clone(),
                        meter_event_name: meter_event_name.to_string(),
                    });
                }
            }
        }

        let mut state = self.state.write().await;
        let meter = if let Some(meter) = state.meters_by_event_name.get(meter_event_name) {
            meter.clone()
        } else {
            let meter = StripeMeter::create(
                &self.client,
                StripeCreateMeterParams {
                    default_aggregation: DefaultAggregation { formula: "sum" },
                    display_name: price_description.to_string(),
                    event_name: meter_event_name,
                },
            )
            .await?;
            state
                .meters_by_event_name
                .insert(meter_event_name.to_string(), meter.clone());
            meter
        };

        let price_id = if let Some(price_id) = state.price_ids_by_meter_id.get(&meter.id) {
            price_id.clone()
        } else {
            let price = stripe::Price::create(
                &self.client,
                stripe::CreatePrice {
                    active: Some(true),
                    billing_scheme: Some(stripe::PriceBillingScheme::PerUnit),
                    currency: stripe::Currency::USD,
                    currency_options: None,
                    custom_unit_amount: None,
                    expand: &[],
                    lookup_key: None,
                    metadata: None,
                    nickname: None,
                    product: None,
                    product_data: Some(stripe::CreatePriceProductData {
                        id: None,
                        active: Some(true),
                        metadata: None,
                        name: price_description.to_string(),
                        statement_descriptor: None,
                        tax_code: None,
                        unit_label: None,
                    }),
                    recurring: Some(stripe::CreatePriceRecurring {
                        aggregate_usage: None,
                        interval: stripe::CreatePriceRecurringInterval::Month,
                        interval_count: None,
                        trial_period_days: None,
                        usage_type: Some(stripe::CreatePriceRecurringUsageType::Metered),
                        meter: Some(meter.id.clone()),
                    }),
                    tax_behavior: None,
                    tiers: None,
                    tiers_mode: None,
                    transfer_lookup_key: None,
                    transform_quantity: None,
                    unit_amount: None,
                    unit_amount_decimal: Some(&format!(
                        "{:.12}",
                        price_per_million_tokens.0 as f64 / 1_000_000f64
                    )),
                },
            )
            .await?;
            state
                .price_ids_by_meter_id
                .insert(meter.id, price.id.clone());
            price.id
        };

        Ok(StripeBillingPrice {
            id: price_id,
            meter_event_name: meter_event_name.to_string(),
        })
    }

    pub async fn subscribe_to_price(
        &self,
        subscription_id: &stripe::SubscriptionId,
        price_id: &stripe::PriceId,
    ) -> Result<()> {
        let subscription =
            stripe::Subscription::retrieve(&self.client, &subscription_id, &[]).await?;

        if subscription_contains_price(&subscription, price_id) {
            return Ok(());
        }

        stripe::Subscription::update(
            &self.client,
            subscription_id,
            stripe::UpdateSubscription {
                items: Some(vec![stripe::UpdateSubscriptionItems {
                    price: Some(price_id.to_string()),
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

    pub async fn subscribe_to_model(
        &self,
        subscription_id: &stripe::SubscriptionId,
        model: &StripeModelTokenPrices,
    ) -> Result<()> {
        let subscription =
            stripe::Subscription::retrieve(&self.client, &subscription_id, &[]).await?;

        let mut items = Vec::new();

        if !subscription_contains_price(&subscription, &model.input_tokens_price.id) {
            items.push(stripe::UpdateSubscriptionItems {
                price: Some(model.input_tokens_price.id.to_string()),
                ..Default::default()
            });
        }

        if !subscription_contains_price(&subscription, &model.input_cache_creation_tokens_price.id)
        {
            items.push(stripe::UpdateSubscriptionItems {
                price: Some(model.input_cache_creation_tokens_price.id.to_string()),
                ..Default::default()
            });
        }

        if !subscription_contains_price(&subscription, &model.input_cache_read_tokens_price.id) {
            items.push(stripe::UpdateSubscriptionItems {
                price: Some(model.input_cache_read_tokens_price.id.to_string()),
                ..Default::default()
            });
        }

        if !subscription_contains_price(&subscription, &model.output_tokens_price.id) {
            items.push(stripe::UpdateSubscriptionItems {
                price: Some(model.output_tokens_price.id.to_string()),
                ..Default::default()
            });
        }

        if !items.is_empty() {
            items.extend(subscription.items.data.iter().map(|item| {
                stripe::UpdateSubscriptionItems {
                    id: Some(item.id.to_string()),
                    ..Default::default()
                }
            }));

            stripe::Subscription::update(
                &self.client,
                subscription_id,
                stripe::UpdateSubscription {
                    items: Some(items),
                    ..Default::default()
                },
            )
            .await?;
        }

        Ok(())
    }

    pub async fn bill_model_token_usage(
        &self,
        customer_id: &stripe::CustomerId,
        model: &StripeModelTokenPrices,
        event: &llm::db::billing_event::Model,
    ) -> Result<()> {
        let timestamp = Utc::now().timestamp();

        if event.input_tokens > 0 {
            StripeMeterEvent::create(
                &self.client,
                StripeCreateMeterEventParams {
                    identifier: &format!("input_tokens/{}", event.idempotency_key),
                    event_name: &model.input_tokens_price.meter_event_name,
                    payload: StripeCreateMeterEventPayload {
                        value: event.input_tokens as u64,
                        stripe_customer_id: customer_id,
                    },
                    timestamp: Some(timestamp),
                },
            )
            .await?;
        }

        if event.input_cache_creation_tokens > 0 {
            StripeMeterEvent::create(
                &self.client,
                StripeCreateMeterEventParams {
                    identifier: &format!("input_cache_creation_tokens/{}", event.idempotency_key),
                    event_name: &model.input_cache_creation_tokens_price.meter_event_name,
                    payload: StripeCreateMeterEventPayload {
                        value: event.input_cache_creation_tokens as u64,
                        stripe_customer_id: customer_id,
                    },
                    timestamp: Some(timestamp),
                },
            )
            .await?;
        }

        if event.input_cache_read_tokens > 0 {
            StripeMeterEvent::create(
                &self.client,
                StripeCreateMeterEventParams {
                    identifier: &format!("input_cache_read_tokens/{}", event.idempotency_key),
                    event_name: &model.input_cache_read_tokens_price.meter_event_name,
                    payload: StripeCreateMeterEventPayload {
                        value: event.input_cache_read_tokens as u64,
                        stripe_customer_id: customer_id,
                    },
                    timestamp: Some(timestamp),
                },
            )
            .await?;
        }

        if event.output_tokens > 0 {
            StripeMeterEvent::create(
                &self.client,
                StripeCreateMeterEventParams {
                    identifier: &format!("output_tokens/{}", event.idempotency_key),
                    event_name: &model.output_tokens_price.meter_event_name,
                    payload: StripeCreateMeterEventPayload {
                        value: event.output_tokens as u64,
                        stripe_customer_id: customer_id,
                    },
                    timestamp: Some(timestamp),
                },
            )
            .await?;
        }

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

    pub async fn checkout(
        &self,
        customer_id: stripe::CustomerId,
        github_login: &str,
        model: &StripeModelTokenPrices,
        success_url: &str,
    ) -> Result<String> {
        let first_of_next_month = Utc::now()
            .checked_add_months(chrono::Months::new(1))
            .unwrap()
            .with_day(1)
            .unwrap();

        let mut params = stripe::CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.customer = Some(customer_id);
        params.client_reference_id = Some(github_login);
        params.subscription_data = Some(stripe::CreateCheckoutSessionSubscriptionData {
            billing_cycle_anchor: Some(first_of_next_month.timestamp()),
            ..Default::default()
        });
        params.line_items = Some(
            [
                &model.input_tokens_price.id,
                &model.input_cache_creation_tokens_price.id,
                &model.input_cache_read_tokens_price.id,
                &model.output_tokens_price.id,
            ]
            .into_iter()
            .map(|price_id| stripe::CreateCheckoutSessionLineItems {
                price: Some(price_id.to_string()),
                ..Default::default()
            })
            .collect(),
        );
        params.success_url = Some(success_url);

        let session = stripe::CheckoutSession::create(&self.client, params).await?;
        Ok(session.url.context("no checkout session URL")?)
    }

    pub async fn checkout_with_price(
        &self,
        price_id: PriceId,
        customer_id: stripe::CustomerId,
        github_login: &str,
        success_url: &str,
    ) -> Result<String> {
        let mut params = stripe::CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.customer = Some(customer_id);
        params.client_reference_id = Some(github_login);
        params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            price: Some(price_id.to_string()),
            quantity: Some(1),
            ..Default::default()
        }]);
        params.success_url = Some(success_url);

        let session = stripe::CheckoutSession::create(&self.client, params).await?;
        Ok(session.url.context("no checkout session URL")?)
    }

    pub async fn checkout_with_zed_pro_trial(
        &self,
        zed_pro_price_id: PriceId,
        customer_id: stripe::CustomerId,
        github_login: &str,
        success_url: &str,
    ) -> Result<String> {
        let mut params = stripe::CreateCheckoutSession::new();
        params.subscription_data = Some(stripe::CreateCheckoutSessionSubscriptionData {
            trial_period_days: Some(14),
            trial_settings: Some(stripe::CreateCheckoutSessionSubscriptionDataTrialSettings {
                end_behavior: stripe::CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior {
                    missing_payment_method: stripe::CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Pause,
                }
            }),
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
}

#[derive(Serialize)]
struct DefaultAggregation {
    formula: &'static str,
}

#[derive(Serialize)]
struct StripeCreateMeterParams<'a> {
    default_aggregation: DefaultAggregation,
    display_name: String,
    event_name: &'a str,
}

#[derive(Clone, Deserialize)]
struct StripeMeter {
    id: String,
    event_name: String,
}

impl StripeMeter {
    pub fn create(
        client: &stripe::Client,
        params: StripeCreateMeterParams,
    ) -> stripe::Response<Self> {
        client.post_form("/billing/meters", params)
    }

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
