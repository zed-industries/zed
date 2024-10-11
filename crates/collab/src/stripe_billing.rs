use std::sync::Arc;

use crate::{llm, Cents, Result};
use anyhow::Context;
use chrono::Utc;
use collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub struct StripeBilling {
    meters_by_event_name: RwLock<HashMap<String, StripeMeter>>,
    price_ids_by_meter_id: RwLock<HashMap<String, stripe::PriceId>>,
    client: Arc<stripe::Client>,
}

pub struct StripeModel {
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
            meters_by_event_name: RwLock::new(HashMap::default()),
            price_ids_by_meter_id: RwLock::new(HashMap::default()),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        log::info!("initializing StripeBilling");

        {
            let meters = StripeMeter::list(&self.client).await?.data;
            let mut meters_by_event_name = self.meters_by_event_name.write().await;
            for meter in meters {
                meters_by_event_name.insert(meter.event_name.clone(), meter);
            }
        }

        {
            let prices = stripe::Price::list(&self.client, &stripe::ListPrices::default())
                .await?
                .data;
            let mut price_ids_by_meter_id = self.price_ids_by_meter_id.write().await;
            for price in prices {
                if let Some(recurring) = price.recurring {
                    if let Some(meter) = recurring.meter {
                        price_ids_by_meter_id.insert(meter, price.id);
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn register_model(&self, model: &llm::db::model::Model) -> Result<StripeModel> {
        let input_tokens_price = self
            .get_or_insert_price(
                &format!("model_{}/input_tokens", model.id),
                &format!("{} (Input Tokens)", model.name),
                Cents::new(model.price_per_million_input_tokens as u32),
            )
            .await?;
        let input_cache_creation_tokens_price = self
            .get_or_insert_price(
                &format!("model_{}/input_cache_creation_tokens", model.id),
                &format!("{} (Input Cache Creation Tokens)", model.name),
                Cents::new(model.price_per_million_cache_creation_input_tokens as u32),
            )
            .await?;
        let input_cache_read_tokens_price = self
            .get_or_insert_price(
                &format!("model_{}/input_cache_read_tokens", model.id),
                &format!("{} (Input Cache Read Tokens)", model.name),
                Cents::new(model.price_per_million_cache_read_input_tokens as u32),
            )
            .await?;
        let output_tokens_price = self
            .get_or_insert_price(
                &format!("model_{}/output_tokens", model.id),
                &format!("{} (Output Tokens)", model.name),
                Cents::new(model.price_per_million_output_tokens as u32),
            )
            .await?;
        Ok(StripeModel {
            input_tokens_price,
            input_cache_creation_tokens_price,
            input_cache_read_tokens_price,
            output_tokens_price,
        })
    }

    async fn get_or_insert_price(
        &self,
        meter_event_name: &str,
        price_description: &str,
        price_per_million_tokens: Cents,
    ) -> Result<StripeBillingPrice> {
        let meter =
            if let Some(meter) = self.meters_by_event_name.read().await.get(meter_event_name) {
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
                self.meters_by_event_name
                    .write()
                    .await
                    .insert(meter_event_name.to_string(), meter.clone());
                meter
            };

        let price_id =
            if let Some(price_id) = self.price_ids_by_meter_id.read().await.get(&meter.id) {
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
                self.price_ids_by_meter_id
                    .write()
                    .await
                    .insert(meter.id, price.id.clone());
                price.id
            };

        Ok(StripeBillingPrice {
            id: price_id,
            meter_event_name: meter_event_name.to_string(),
        })
    }

    pub async fn subscribe_to_model(
        &self,
        subscription_id: &stripe::SubscriptionId,
        model: &StripeModel,
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

    pub async fn bill_model_usage(
        &self,
        customer_id: &stripe::CustomerId,
        model: &StripeModel,
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

    pub async fn checkout(
        &self,
        customer_id: stripe::CustomerId,
        github_login: &str,
        model: &StripeModel,
        success_url: &str,
    ) -> Result<String> {
        let mut params = stripe::CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.customer = Some(customer_id);
        params.client_reference_id = Some(github_login);
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
        struct Params {}

        client.get_query("/billing/meters", Params {})
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
