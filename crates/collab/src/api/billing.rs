use anyhow::{anyhow, bail, Context};
use axum::{
    extract::{self, Query},
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, SecondsFormat, Utc};
use collections::HashSet;
use reqwest::StatusCode;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc, time::Duration};
use stripe::{
    BillingPortalSession, CreateBillingPortalSession, CreateBillingPortalSessionFlowData,
    CreateBillingPortalSessionFlowDataAfterCompletion,
    CreateBillingPortalSessionFlowDataAfterCompletionRedirect,
    CreateBillingPortalSessionFlowDataType, CreateCustomer, Customer, CustomerId, EventObject,
    EventType, Expandable, ListEvents, Subscription, SubscriptionId, SubscriptionStatus,
};
use util::ResultExt;

use crate::llm::{DEFAULT_MAX_MONTHLY_SPEND, FREE_TIER_MONTHLY_SPENDING_LIMIT};
use crate::rpc::{ResultExt as _, Server};
use crate::{
    db::{
        billing_customer, BillingSubscriptionId, CreateBillingCustomerParams,
        CreateBillingSubscriptionParams, CreateProcessedStripeEventParams,
        UpdateBillingCustomerParams, UpdateBillingPreferencesParams,
        UpdateBillingSubscriptionParams,
    },
    stripe_billing::StripeBilling,
};
use crate::{
    db::{billing_subscription::StripeSubscriptionStatus, UserId},
    llm::db::LlmDatabase,
};
use crate::{AppState, Cents, Error, Result};

pub fn router() -> Router {
    Router::new()
        .route(
            "/billing/preferences",
            get(get_billing_preferences).put(update_billing_preferences),
        )
        .route(
            "/billing/subscriptions",
            get(list_billing_subscriptions).post(create_billing_subscription),
        )
        .route(
            "/billing/subscriptions/manage",
            post(manage_billing_subscription),
        )
        .route("/billing/monthly_spend", get(get_monthly_spend))
}

#[derive(Debug, Deserialize)]
struct GetBillingPreferencesParams {
    github_user_id: i32,
}

#[derive(Debug, Serialize)]
struct BillingPreferencesResponse {
    max_monthly_llm_usage_spending_in_cents: i32,
}

async fn get_billing_preferences(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<GetBillingPreferencesParams>,
) -> Result<Json<BillingPreferencesResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(params.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let preferences = app.db.get_billing_preferences(user.id).await?;

    Ok(Json(BillingPreferencesResponse {
        max_monthly_llm_usage_spending_in_cents: preferences
            .map_or(DEFAULT_MAX_MONTHLY_SPEND.0 as i32, |preferences| {
                preferences.max_monthly_llm_usage_spending_in_cents
            }),
    }))
}

#[derive(Debug, Deserialize)]
struct UpdateBillingPreferencesBody {
    github_user_id: i32,
    max_monthly_llm_usage_spending_in_cents: i32,
}

async fn update_billing_preferences(
    Extension(app): Extension<Arc<AppState>>,
    Extension(rpc_server): Extension<Arc<crate::rpc::Server>>,
    extract::Json(body): extract::Json<UpdateBillingPreferencesBody>,
) -> Result<Json<BillingPreferencesResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(body.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let billing_preferences =
        if let Some(_billing_preferences) = app.db.get_billing_preferences(user.id).await? {
            app.db
                .update_billing_preferences(
                    user.id,
                    &UpdateBillingPreferencesParams {
                        max_monthly_llm_usage_spending_in_cents: ActiveValue::set(
                            body.max_monthly_llm_usage_spending_in_cents,
                        ),
                    },
                )
                .await?
        } else {
            app.db
                .create_billing_preferences(
                    user.id,
                    &crate::db::CreateBillingPreferencesParams {
                        max_monthly_llm_usage_spending_in_cents: body
                            .max_monthly_llm_usage_spending_in_cents,
                    },
                )
                .await?
        };

    rpc_server.refresh_llm_tokens_for_user(user.id).await;

    Ok(Json(BillingPreferencesResponse {
        max_monthly_llm_usage_spending_in_cents: billing_preferences
            .max_monthly_llm_usage_spending_in_cents,
    }))
}

#[derive(Debug, Deserialize)]
struct ListBillingSubscriptionsParams {
    github_user_id: i32,
}

#[derive(Debug, Serialize)]
struct BillingSubscriptionJson {
    id: BillingSubscriptionId,
    name: String,
    status: StripeSubscriptionStatus,
    cancel_at: Option<String>,
    /// Whether this subscription can be canceled.
    is_cancelable: bool,
}

#[derive(Debug, Serialize)]
struct ListBillingSubscriptionsResponse {
    subscriptions: Vec<BillingSubscriptionJson>,
}

async fn list_billing_subscriptions(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<ListBillingSubscriptionsParams>,
) -> Result<Json<ListBillingSubscriptionsResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(params.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let subscriptions = app.db.get_billing_subscriptions(user.id).await?;

    Ok(Json(ListBillingSubscriptionsResponse {
        subscriptions: subscriptions
            .into_iter()
            .map(|subscription| BillingSubscriptionJson {
                id: subscription.id,
                name: "Zed LLM Usage".to_string(),
                status: subscription.stripe_subscription_status,
                cancel_at: subscription.stripe_cancel_at.map(|cancel_at| {
                    cancel_at
                        .and_utc()
                        .to_rfc3339_opts(SecondsFormat::Millis, true)
                }),
                is_cancelable: subscription.stripe_subscription_status.is_cancelable()
                    && subscription.stripe_cancel_at.is_none(),
            })
            .collect(),
    }))
}

#[derive(Debug, Deserialize)]
struct CreateBillingSubscriptionBody {
    github_user_id: i32,
}

#[derive(Debug, Serialize)]
struct CreateBillingSubscriptionResponse {
    checkout_session_url: String,
}

/// Initiates a Stripe Checkout session for creating a billing subscription.
async fn create_billing_subscription(
    Extension(app): Extension<Arc<AppState>>,
    extract::Json(body): extract::Json<CreateBillingSubscriptionBody>,
) -> Result<Json<CreateBillingSubscriptionResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(body.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let Some(stripe_client) = app.stripe_client.clone() else {
        log::error!("failed to retrieve Stripe client");
        Err(Error::http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };
    let Some(stripe_billing) = app.stripe_billing.clone() else {
        log::error!("failed to retrieve Stripe billing object");
        Err(Error::http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };
    let Some(llm_db) = app.llm_db.clone() else {
        log::error!("failed to retrieve LLM database");
        Err(Error::http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };

    if app.db.has_active_billing_subscription(user.id).await? {
        return Err(Error::http(
            StatusCode::CONFLICT,
            "user already has an active subscription".into(),
        ));
    }

    let customer_id =
        if let Some(existing_customer) = app.db.get_billing_customer_by_user_id(user.id).await? {
            CustomerId::from_str(&existing_customer.stripe_customer_id)
                .context("failed to parse customer ID")?
        } else {
            let customer = Customer::create(
                &stripe_client,
                CreateCustomer {
                    email: user.email_address.as_deref(),
                    ..Default::default()
                },
            )
            .await?;

            customer.id
        };

    let default_model = llm_db.model(rpc::LanguageModelProvider::Anthropic, "claude-3-5-sonnet")?;
    let stripe_model = stripe_billing.register_model(default_model).await?;
    let success_url = format!("{}/account", app.config.zed_dot_dev_url());
    let checkout_session_url = stripe_billing
        .checkout(customer_id, &user.github_login, &stripe_model, &success_url)
        .await?;
    Ok(Json(CreateBillingSubscriptionResponse {
        checkout_session_url,
    }))
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ManageSubscriptionIntent {
    /// The user intends to cancel their subscription.
    Cancel,
    /// The user intends to stop the cancellation of their subscription.
    StopCancellation,
}

#[derive(Debug, Deserialize)]
struct ManageBillingSubscriptionBody {
    github_user_id: i32,
    intent: ManageSubscriptionIntent,
    /// The ID of the subscription to manage.
    subscription_id: BillingSubscriptionId,
}

#[derive(Debug, Serialize)]
struct ManageBillingSubscriptionResponse {
    billing_portal_session_url: Option<String>,
}

/// Initiates a Stripe customer portal session for managing a billing subscription.
async fn manage_billing_subscription(
    Extension(app): Extension<Arc<AppState>>,
    extract::Json(body): extract::Json<ManageBillingSubscriptionBody>,
) -> Result<Json<ManageBillingSubscriptionResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(body.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let Some(stripe_client) = app.stripe_client.clone() else {
        log::error!("failed to retrieve Stripe client");
        Err(Error::http(
            StatusCode::NOT_IMPLEMENTED,
            "not supported".into(),
        ))?
    };

    let customer = app
        .db
        .get_billing_customer_by_user_id(user.id)
        .await?
        .ok_or_else(|| anyhow!("billing customer not found"))?;
    let customer_id = CustomerId::from_str(&customer.stripe_customer_id)
        .context("failed to parse customer ID")?;

    let subscription = app
        .db
        .get_billing_subscription_by_id(body.subscription_id)
        .await?
        .ok_or_else(|| anyhow!("subscription not found"))?;

    if body.intent == ManageSubscriptionIntent::StopCancellation {
        let subscription_id = SubscriptionId::from_str(&subscription.stripe_subscription_id)
            .context("failed to parse subscription ID")?;

        let updated_stripe_subscription = Subscription::update(
            &stripe_client,
            &subscription_id,
            stripe::UpdateSubscription {
                cancel_at_period_end: Some(false),
                ..Default::default()
            },
        )
        .await?;

        app.db
            .update_billing_subscription(
                subscription.id,
                &UpdateBillingSubscriptionParams {
                    stripe_cancel_at: ActiveValue::set(
                        updated_stripe_subscription
                            .cancel_at
                            .and_then(|cancel_at| DateTime::from_timestamp(cancel_at, 0))
                            .map(|time| time.naive_utc()),
                    ),
                    ..Default::default()
                },
            )
            .await?;

        return Ok(Json(ManageBillingSubscriptionResponse {
            billing_portal_session_url: None,
        }));
    }

    let flow = match body.intent {
        ManageSubscriptionIntent::Cancel => CreateBillingPortalSessionFlowData {
            type_: CreateBillingPortalSessionFlowDataType::SubscriptionCancel,
            after_completion: Some(CreateBillingPortalSessionFlowDataAfterCompletion {
                type_: stripe::CreateBillingPortalSessionFlowDataAfterCompletionType::Redirect,
                redirect: Some(CreateBillingPortalSessionFlowDataAfterCompletionRedirect {
                    return_url: format!("{}/account", app.config.zed_dot_dev_url()),
                }),
                ..Default::default()
            }),
            subscription_cancel: Some(
                stripe::CreateBillingPortalSessionFlowDataSubscriptionCancel {
                    subscription: subscription.stripe_subscription_id,
                    retention: None,
                },
            ),
            ..Default::default()
        },
        ManageSubscriptionIntent::StopCancellation => unreachable!(),
    };

    let mut params = CreateBillingPortalSession::new(customer_id);
    params.flow_data = Some(flow);
    let return_url = format!("{}/account", app.config.zed_dot_dev_url());
    params.return_url = Some(&return_url);

    let session = BillingPortalSession::create(&stripe_client, params).await?;

    Ok(Json(ManageBillingSubscriptionResponse {
        billing_portal_session_url: Some(session.url),
    }))
}

/// The amount of time we wait in between each poll of Stripe events.
///
/// This value should strike a balance between:
///   1. Being short enough that we update quickly when something in Stripe changes
///   2. Being long enough that we don't eat into our rate limits.
///
/// As a point of reference, the Sequin folks say they have this at **500ms**:
///
/// > We poll the Stripe /events endpoint every 500ms per account
/// >
/// > — https://blog.sequinstream.com/events-not-webhooks/
const POLL_EVENTS_INTERVAL: Duration = Duration::from_secs(5);

/// The maximum number of events to return per page.
///
/// We set this to 100 (the max) so we have to make fewer requests to Stripe.
///
/// > Limit can range between 1 and 100, and the default is 10.
const EVENTS_LIMIT_PER_PAGE: u64 = 100;

/// The number of pages consisting entirely of already-processed events that we
/// will see before we stop retrieving events.
///
/// This is used to prevent over-fetching the Stripe events API for events we've
/// already seen and processed.
const NUMBER_OF_ALREADY_PROCESSED_PAGES_BEFORE_WE_STOP: usize = 4;

/// Polls the Stripe events API periodically to reconcile the records in our
/// database with the data in Stripe.
pub fn poll_stripe_events_periodically(app: Arc<AppState>, rpc_server: Arc<Server>) {
    let Some(stripe_client) = app.stripe_client.clone() else {
        log::warn!("failed to retrieve Stripe client");
        return;
    };

    let executor = app.executor.clone();
    executor.spawn_detached({
        let executor = executor.clone();
        async move {
            loop {
                poll_stripe_events(&app, &rpc_server, &stripe_client)
                    .await
                    .log_err();

                executor.sleep(POLL_EVENTS_INTERVAL).await;
            }
        }
    });
}

async fn poll_stripe_events(
    app: &Arc<AppState>,
    rpc_server: &Arc<Server>,
    stripe_client: &stripe::Client,
) -> anyhow::Result<()> {
    fn event_type_to_string(event_type: EventType) -> String {
        // Calling `to_string` on `stripe::EventType` members gives us a quoted string,
        // so we need to unquote it.
        event_type.to_string().trim_matches('"').to_string()
    }

    let event_types = [
        EventType::CustomerCreated,
        EventType::CustomerUpdated,
        EventType::CustomerSubscriptionCreated,
        EventType::CustomerSubscriptionUpdated,
        EventType::CustomerSubscriptionPaused,
        EventType::CustomerSubscriptionResumed,
        EventType::CustomerSubscriptionDeleted,
    ]
    .into_iter()
    .map(event_type_to_string)
    .collect::<Vec<_>>();

    let mut pages_of_already_processed_events = 0;
    let mut unprocessed_events = Vec::new();

    log::info!(
        "Stripe events: starting retrieval for {}",
        event_types.join(", ")
    );
    let mut params = ListEvents::new();
    params.types = Some(event_types.clone());
    params.limit = Some(EVENTS_LIMIT_PER_PAGE);

    let mut event_pages = stripe::Event::list(&stripe_client, &params)
        .await?
        .paginate(params);

    loop {
        let processed_event_ids = {
            let event_ids = event_pages
                .page
                .data
                .iter()
                .map(|event| event.id.as_str())
                .collect::<Vec<_>>();
            app.db
                .get_processed_stripe_events_by_event_ids(&event_ids)
                .await?
                .into_iter()
                .map(|event| event.stripe_event_id)
                .collect::<Vec<_>>()
        };

        let mut processed_events_in_page = 0;
        let events_in_page = event_pages.page.data.len();
        for event in &event_pages.page.data {
            if processed_event_ids.contains(&event.id.to_string()) {
                processed_events_in_page += 1;
                log::debug!("Stripe events: already processed '{}', skipping", event.id);
            } else {
                unprocessed_events.push(event.clone());
            }
        }

        if processed_events_in_page == events_in_page {
            pages_of_already_processed_events += 1;
        }

        if event_pages.page.has_more {
            if pages_of_already_processed_events >= NUMBER_OF_ALREADY_PROCESSED_PAGES_BEFORE_WE_STOP
            {
                log::info!("Stripe events: stopping, saw {pages_of_already_processed_events} pages of already-processed events");
                break;
            } else {
                log::info!("Stripe events: retrieving next page");
                event_pages = event_pages.next(&stripe_client).await?;
            }
        } else {
            break;
        }
    }

    log::info!("Stripe events: unprocessed {}", unprocessed_events.len());

    // Sort all of the unprocessed events in ascending order, so we can handle them in the order they occurred.
    unprocessed_events.sort_by(|a, b| a.created.cmp(&b.created).then_with(|| a.id.cmp(&b.id)));

    for event in unprocessed_events {
        let event_id = event.id.clone();
        let processed_event_params = CreateProcessedStripeEventParams {
            stripe_event_id: event.id.to_string(),
            stripe_event_type: event_type_to_string(event.type_),
            stripe_event_created_timestamp: event.created,
        };

        // If the event has happened too far in the past, we don't want to
        // process it and risk overwriting other more-recent updates.
        //
        // 1 day was chosen arbitrarily. This could be made longer or shorter.
        let one_day = Duration::from_secs(24 * 60 * 60);
        let a_day_ago = Utc::now() - one_day;
        if a_day_ago.timestamp() > event.created {
            log::info!(
                "Stripe events: event '{}' is more than {one_day:?} old, marking as processed",
                event_id
            );
            app.db
                .create_processed_stripe_event(&processed_event_params)
                .await?;

            return Ok(());
        }

        let process_result = match event.type_ {
            EventType::CustomerCreated | EventType::CustomerUpdated => {
                handle_customer_event(app, stripe_client, event).await
            }
            EventType::CustomerSubscriptionCreated
            | EventType::CustomerSubscriptionUpdated
            | EventType::CustomerSubscriptionPaused
            | EventType::CustomerSubscriptionResumed
            | EventType::CustomerSubscriptionDeleted => {
                handle_customer_subscription_event(app, rpc_server, stripe_client, event).await
            }
            _ => Ok(()),
        };

        if let Some(()) = process_result
            .with_context(|| format!("failed to process event {event_id} successfully"))
            .log_err()
        {
            app.db
                .create_processed_stripe_event(&processed_event_params)
                .await?;
        }
    }

    Ok(())
}

async fn handle_customer_event(
    app: &Arc<AppState>,
    _stripe_client: &stripe::Client,
    event: stripe::Event,
) -> anyhow::Result<()> {
    let EventObject::Customer(customer) = event.data.object else {
        bail!("unexpected event payload for {}", event.id);
    };

    log::info!("handling Stripe {} event: {}", event.type_, event.id);

    let Some(email) = customer.email else {
        log::info!("Stripe customer has no email: skipping");
        return Ok(());
    };

    let Some(user) = app.db.get_user_by_email(&email).await? else {
        log::info!("no user found for email: skipping");
        return Ok(());
    };

    if let Some(existing_customer) = app
        .db
        .get_billing_customer_by_stripe_customer_id(&customer.id)
        .await?
    {
        app.db
            .update_billing_customer(
                existing_customer.id,
                &UpdateBillingCustomerParams {
                    // For now we just leave the information as-is, as it is not
                    // likely to change.
                    ..Default::default()
                },
            )
            .await?;
    } else {
        app.db
            .create_billing_customer(&CreateBillingCustomerParams {
                user_id: user.id,
                stripe_customer_id: customer.id.to_string(),
            })
            .await?;
    }

    Ok(())
}

async fn handle_customer_subscription_event(
    app: &Arc<AppState>,
    rpc_server: &Arc<Server>,
    stripe_client: &stripe::Client,
    event: stripe::Event,
) -> anyhow::Result<()> {
    let EventObject::Subscription(subscription) = event.data.object else {
        bail!("unexpected event payload for {}", event.id);
    };

    log::info!("handling Stripe {} event: {}", event.type_, event.id);

    let billing_customer =
        find_or_create_billing_customer(app, stripe_client, subscription.customer)
            .await?
            .ok_or_else(|| anyhow!("billing customer not found"))?;

    if let Some(existing_subscription) = app
        .db
        .get_billing_subscription_by_stripe_subscription_id(&subscription.id)
        .await?
    {
        app.db
            .update_billing_subscription(
                existing_subscription.id,
                &UpdateBillingSubscriptionParams {
                    billing_customer_id: ActiveValue::set(billing_customer.id),
                    stripe_subscription_id: ActiveValue::set(subscription.id.to_string()),
                    stripe_subscription_status: ActiveValue::set(subscription.status.into()),
                    stripe_cancel_at: ActiveValue::set(
                        subscription
                            .cancel_at
                            .and_then(|cancel_at| DateTime::from_timestamp(cancel_at, 0))
                            .map(|time| time.naive_utc()),
                    ),
                },
            )
            .await?;
    } else {
        // If the user already has an active billing subscription, ignore the
        // event and return an `Ok` to signal that it was processed
        // successfully.
        //
        // There is the possibility that this could cause us to not create a
        // subscription in the following scenario:
        //
        //   1. User has an active subscription A
        //   2. User cancels subscription A
        //   3. User creates a new subscription B
        //   4. We process the new subscription B before the cancellation of subscription A
        //   5. User ends up with no subscriptions
        //
        // In theory this situation shouldn't arise as we try to process the events in the order they occur.
        if app
            .db
            .has_active_billing_subscription(billing_customer.user_id)
            .await?
        {
            log::info!(
                "user {user_id} already has an active subscription, skipping creation of subscription {subscription_id}",
                user_id = billing_customer.user_id,
                subscription_id = subscription.id
            );
            return Ok(());
        }

        app.db
            .create_billing_subscription(&CreateBillingSubscriptionParams {
                billing_customer_id: billing_customer.id,
                stripe_subscription_id: subscription.id.to_string(),
                stripe_subscription_status: subscription.status.into(),
            })
            .await?;
    }

    // When the user's subscription changes, we want to refresh their LLM tokens
    // to either grant/revoke access.
    rpc_server
        .refresh_llm_tokens_for_user(billing_customer.user_id)
        .await;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct GetMonthlySpendParams {
    github_user_id: i32,
}

#[derive(Debug, Serialize)]
struct GetMonthlySpendResponse {
    monthly_free_tier_spend_in_cents: u32,
    monthly_free_tier_allowance_in_cents: u32,
    monthly_spend_in_cents: u32,
}

async fn get_monthly_spend(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<GetMonthlySpendParams>,
) -> Result<Json<GetMonthlySpendResponse>> {
    let user = app
        .db
        .get_user_by_github_user_id(params.github_user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;

    let Some(llm_db) = app.llm_db.clone() else {
        return Err(Error::http(
            StatusCode::NOT_IMPLEMENTED,
            "LLM database not available".into(),
        ));
    };

    let free_tier = user
        .custom_llm_monthly_allowance_in_cents
        .map(|allowance| Cents(allowance as u32))
        .unwrap_or(FREE_TIER_MONTHLY_SPENDING_LIMIT);

    let spending_for_month = llm_db
        .get_user_spending_for_month(user.id, Utc::now())
        .await?;

    let free_tier_spend = Cents::min(spending_for_month, free_tier);
    let monthly_spend = spending_for_month.saturating_sub(free_tier);

    Ok(Json(GetMonthlySpendResponse {
        monthly_free_tier_spend_in_cents: free_tier_spend.0,
        monthly_free_tier_allowance_in_cents: free_tier.0,
        monthly_spend_in_cents: monthly_spend.0,
    }))
}

impl From<SubscriptionStatus> for StripeSubscriptionStatus {
    fn from(value: SubscriptionStatus) -> Self {
        match value {
            SubscriptionStatus::Incomplete => Self::Incomplete,
            SubscriptionStatus::IncompleteExpired => Self::IncompleteExpired,
            SubscriptionStatus::Trialing => Self::Trialing,
            SubscriptionStatus::Active => Self::Active,
            SubscriptionStatus::PastDue => Self::PastDue,
            SubscriptionStatus::Canceled => Self::Canceled,
            SubscriptionStatus::Unpaid => Self::Unpaid,
            SubscriptionStatus::Paused => Self::Paused,
        }
    }
}

/// Finds or creates a billing customer using the provided customer.
async fn find_or_create_billing_customer(
    app: &Arc<AppState>,
    stripe_client: &stripe::Client,
    customer_or_id: Expandable<Customer>,
) -> anyhow::Result<Option<billing_customer::Model>> {
    let customer_id = match &customer_or_id {
        Expandable::Id(id) => id,
        Expandable::Object(customer) => customer.id.as_ref(),
    };

    // If we already have a billing customer record associated with the Stripe customer,
    // there's nothing more we need to do.
    if let Some(billing_customer) = app
        .db
        .get_billing_customer_by_stripe_customer_id(customer_id)
        .await?
    {
        return Ok(Some(billing_customer));
    }

    // If all we have is a customer ID, resolve it to a full customer record by
    // hitting the Stripe API.
    let customer = match customer_or_id {
        Expandable::Id(id) => Customer::retrieve(stripe_client, &id, &[]).await?,
        Expandable::Object(customer) => *customer,
    };

    let Some(email) = customer.email else {
        return Ok(None);
    };

    let Some(user) = app.db.get_user_by_email(&email).await? else {
        return Ok(None);
    };

    let billing_customer = app
        .db
        .create_billing_customer(&CreateBillingCustomerParams {
            user_id: user.id,
            stripe_customer_id: customer.id.to_string(),
        })
        .await?;

    Ok(Some(billing_customer))
}

const SYNC_LLM_USAGE_WITH_STRIPE_INTERVAL: Duration = Duration::from_secs(60);

pub fn sync_llm_usage_with_stripe_periodically(app: Arc<AppState>) {
    let Some(stripe_billing) = app.stripe_billing.clone() else {
        log::warn!("failed to retrieve Stripe billing object");
        return;
    };
    let Some(llm_db) = app.llm_db.clone() else {
        log::warn!("failed to retrieve LLM database");
        return;
    };

    let executor = app.executor.clone();
    executor.spawn_detached({
        let executor = executor.clone();
        async move {
            loop {
                sync_with_stripe(&app, &llm_db, &stripe_billing)
                    .await
                    .context("failed to sync LLM usage to Stripe")
                    .trace_err();
                executor.sleep(SYNC_LLM_USAGE_WITH_STRIPE_INTERVAL).await;
            }
        }
    });
}

async fn sync_with_stripe(
    app: &Arc<AppState>,
    llm_db: &Arc<LlmDatabase>,
    stripe_billing: &Arc<StripeBilling>,
) -> anyhow::Result<()> {
    let events = llm_db.get_billing_events().await?;
    let user_ids = events
        .iter()
        .map(|(event, _)| event.user_id)
        .collect::<HashSet<UserId>>();
    let stripe_subscriptions = app.db.get_active_billing_subscriptions(user_ids).await?;

    for (event, model) in events {
        let Some((stripe_db_customer, stripe_db_subscription)) =
            stripe_subscriptions.get(&event.user_id)
        else {
            tracing::warn!(
                user_id = event.user_id.0,
                "Registered billing event for user who is not a Stripe customer. Billing events should only be created for users who are Stripe customers, so this is a mistake on our side."
            );
            continue;
        };
        let stripe_subscription_id: stripe::SubscriptionId = stripe_db_subscription
            .stripe_subscription_id
            .parse()
            .context("failed to parse stripe subscription id from db")?;
        let stripe_customer_id: stripe::CustomerId = stripe_db_customer
            .stripe_customer_id
            .parse()
            .context("failed to parse stripe customer id from db")?;

        let stripe_model = stripe_billing.register_model(&model).await?;
        stripe_billing
            .subscribe_to_model(&stripe_subscription_id, &stripe_model)
            .await?;
        stripe_billing
            .bill_model_usage(&stripe_customer_id, &stripe_model, &event)
            .await?;
        llm_db.consume_billing_event(event.id).await?;
    }

    Ok(())
}
