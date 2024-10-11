use crate::{llm, Cents, Result};
use chrono::Utc;
use collections::HashMap;
use serde::{Deserialize, Serialize};

pub struct StripeBilling {
    meters_by_event_name: HashMap<String, StripeMeter>,
    price_ids_by_meter_id: HashMap<String, stripe::PriceId>,
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
    pub async fn new(client: &stripe::Client) -> Result<Self> {
        let mut meters_by_event_name = HashMap::default();
        for meter in StripeMeter::list(client).await?.data {
            meters_by_event_name.insert(meter.event_name.clone(), meter);
        }

        let mut price_ids_by_meter_id = HashMap::default();
        for price in stripe::Price::list(client, &stripe::ListPrices::default())
            .await?
            .data
        {
            if let Some(recurring) = price.recurring {
                if let Some(meter) = recurring.meter {
                    price_ids_by_meter_id.insert(meter, price.id);
                }
            }
        }

        Ok(Self {
            meters_by_event_name,
            price_ids_by_meter_id,
        })
    }

    pub async fn register_model(
        &mut self,
        model_id: llm::db::ModelId,
        model_name: &str,
        price_per_million_input_tokens: Cents,
        price_per_million_cache_creation_input_tokens: Cents,
        price_per_million_cache_read_input_tokens: Cents,
        price_per_million_output_tokens: Cents,
        client: &stripe::Client,
    ) -> Result<StripeModel> {
        let input_tokens_price = self
            .get_or_insert_price(
                &format!("model_{model_id}/input_tokens"),
                &format!("{model_name} (Input Tokens)"),
                price_per_million_input_tokens,
                client,
            )
            .await?;
        let input_cache_creation_tokens_price = self
            .get_or_insert_price(
                &format!("model_{model_id}/input_cache_creation_tokens"),
                &format!("{model_name} (Input Cache Creation Tokens)"),
                price_per_million_cache_creation_input_tokens,
                client,
            )
            .await?;
        let input_cache_read_tokens_price = self
            .get_or_insert_price(
                &format!("model_{model_id}/input_cache_read_tokens"),
                &format!("{model_name} (Input Cache Read Tokens)"),
                price_per_million_cache_read_input_tokens,
                client,
            )
            .await?;
        let output_tokens_price = self
            .get_or_insert_price(
                &format!("model_{model_id}/output_tokens"),
                &format!("{model_name} (Output Tokens)"),
                price_per_million_output_tokens,
                client,
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
        &mut self,
        meter_event_name: &str,
        price_description: &str,
        price_per_million_tokens: Cents,
        client: &stripe::Client,
    ) -> Result<StripeBillingPrice> {
        let meter = if let Some(meter) = self.meters_by_event_name.get(meter_event_name) {
            meter.clone()
        } else {
            let meter = StripeMeter::create(
                client,
                StripeCreateMeterParams {
                    default_aggregation: DefaultAggregation { formula: "sum" },
                    display_name: price_description.to_string(),
                    event_name: meter_event_name,
                },
            )
            .await?;
            self.meters_by_event_name
                .insert(meter_event_name.to_string(), meter.clone());
            meter
        };

        let price_id = if let Some(price_id) = self.price_ids_by_meter_id.get(&meter.id) {
            price_id.clone()
        } else {
            let price = stripe::Price::create(
                client,
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
                        aggregate_usage: Some(stripe::CreatePriceRecurringAggregateUsage::Sum),
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
                    transform_quantity: Some(stripe::CreatePriceTransformQuantity {
                        divide_by: 1000000,
                        round: stripe::CreatePriceTransformQuantityRound::Up,
                    }),
                    unit_amount: Some(price_per_million_tokens.0 as i64),
                    unit_amount_decimal: None,
                },
            )
            .await?;
            self.price_ids_by_meter_id
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
        client: &stripe::Client,
    ) -> Result<()> {
        let subscription = stripe::Subscription::retrieve(client, &subscription_id, &[]).await?;

        if !subscription_contains_price(&subscription, &model.input_tokens_price.id)
            && !subscription_contains_price(
                &subscription,
                &model.input_cache_creation_tokens_price.id,
            )
            && !subscription_contains_price(&subscription, &model.input_cache_read_tokens_price.id)
            && !subscription_contains_price(&subscription, &model.output_tokens_price.id)
        {
            stripe::Subscription::update(
                client,
                subscription_id,
                stripe::UpdateSubscription {
                    items: Some(vec![
                        stripe::UpdateSubscriptionItems {
                            price: Some(model.input_tokens_price.id.to_string()),
                            ..Default::default()
                        },
                        stripe::UpdateSubscriptionItems {
                            price: Some(model.input_cache_creation_tokens_price.id.to_string()),
                            ..Default::default()
                        },
                        stripe::UpdateSubscriptionItems {
                            price: Some(model.input_cache_read_tokens_price.id.to_string()),
                            ..Default::default()
                        },
                        stripe::UpdateSubscriptionItems {
                            price: Some(model.output_tokens_price.id.to_string()),
                            ..Default::default()
                        },
                    ]),
                    ..Default::default()
                },
            )
            .await?;
        }

        Ok(())
    }

    pub async fn bill_model_usage(
        &self,
        model: &StripeModel,
        customer_id: &stripe::CustomerId,
        input_tokens: u64,
        cache_creation_input_tokens: u64,
        cache_read_input_tokens: u64,
        output_tokens: u64,
        client: &stripe::Client,
    ) -> Result<()> {
        let timestamp = Utc::now().timestamp();

        if input_tokens > 0 {
            StripeMeterEvent::create(
                client,
                StripeCreateMeterEventParams {
                    event_name: &model.input_tokens_price.meter_event_name,
                    payload: StripeCreateMeterEventPayload {
                        value: input_tokens,
                        stripe_customer_id: customer_id,
                    },
                    timestamp: Some(timestamp),
                },
            )
            .await?;
        }

        if cache_creation_input_tokens > 0 {
            StripeMeterEvent::create(
                client,
                StripeCreateMeterEventParams {
                    event_name: &model.input_cache_creation_tokens_price.meter_event_name,
                    payload: StripeCreateMeterEventPayload {
                        value: cache_creation_input_tokens,
                        stripe_customer_id: customer_id,
                    },
                    timestamp: Some(timestamp),
                },
            )
            .await?;
        }

        if cache_read_input_tokens > 0 {
            StripeMeterEvent::create(
                client,
                StripeCreateMeterEventParams {
                    event_name: &model.input_cache_read_tokens_price.meter_event_name,
                    payload: StripeCreateMeterEventPayload {
                        value: cache_read_input_tokens,
                        stripe_customer_id: customer_id,
                    },
                    timestamp: Some(timestamp),
                },
            )
            .await?;
        }

        if output_tokens > 0 {
            StripeMeterEvent::create(
                client,
                StripeCreateMeterEventParams {
                    event_name: &model.output_tokens_price.meter_event_name,
                    payload: StripeCreateMeterEventPayload {
                        value: output_tokens,
                        stripe_customer_id: customer_id,
                    },
                    timestamp: Some(timestamp),
                },
            )
            .await?;
        }

        Ok(())
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
        client.post_form("/v1/billing/meters", params)
    }

    pub fn list(client: &stripe::Client) -> stripe::Response<stripe::List<Self>> {
        client.get_query("/billing/meters", ())
    }
}

#[derive(Deserialize)]
struct StripeMeterEvent {
    identifier: String,
}

impl StripeMeterEvent {
    pub fn create(
        client: &stripe::Client,
        params: StripeCreateMeterEventParams,
    ) -> stripe::Response<Self> {
        client.post_form("/v1/billing/meter_events", params)
    }
}

#[derive(Serialize)]
struct StripeCreateMeterEventParams<'a> {
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
